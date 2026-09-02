// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `div_rem_into` — the divide's **exact-scratch door**: the matcher's verdict
//! routed into the chosen engine's `_into` variant with the CALLER's
//! normalisation buffers.
//!
//! The slice [`dispatch`](crate::int::policy::div_rem::dispatch) is the
//! build-max door: every engine behind it allocates and zeroes its own
//! `MAX_SINGLE_LIMBS` scratch on entry (`div_knuth` two of them,
//! `div_knuth_u128_limb` four), which is `4·MAX_WORK_N + 2` limbs — a size
//! chosen by the build's WIDTH FEATURES, not by the operands. A caller that
//! divides in a loop therefore pays that memset once per iteration, at a width
//! it may never touch: enabling `xx-wide` for one D1232 value makes every D57
//! Newton iteration zero 258 limbs instead of 66, for identical work.
//!
//! This door takes the scratch instead, so its cost tracks the operands. It is
//! the same shape [`crate::algos::rem::rem_int_layer`] and
//! `exp_generic::div_rem_exact` already use — read the verdict, run the chosen
//! engine's `_into` — factored out so the two slice roots
//! ([`isqrt_newton`](crate::int::algos::isqrt::isqrt_newton::isqrt_newton_into),
//! [`icbrt_newton`](crate::int::algos::icbrt::icbrt_newton::icbrt_newton_into))
//! share one copy rather than each growing its own match.

use crate::int::algos::div::div_knuth::div_knuth_into;
use crate::int::algos::div::div_knuth_u128_limb::div_knuth_u128_limb_into;
use crate::int::algos::div::div_rem::div_rem;
use crate::int::policy::div_rem::{select_for_limbs, Algorithm};

/// `quot = num / den`, `rem = num % den`, in caller-provided scratch.
///
/// Engine choice is the divide matcher's
/// ([`select_for_limbs`](crate::int::policy::div_rem::select_for_limbs)) — not
/// this function's — and the match is EXHAUSTIVE (no `_`), so a new
/// `Algorithm` arm forces a decision here instead of silently collapsing onto
/// Knuth. `Rem` keeps its own arm because [`div_knuth_into`] normalises the
/// dividend into `u` *before* it discovers the divisor is single-limb, so
/// routing a single-limb divisor through it would pay a full dividend copy the
/// hardware path does not need.
///
/// Required scratch, in limbs:
///
/// | buffer | minimum |
/// |---|---|
/// | `u` | `num.len() + 2` (the divide reads one limb above the live dividend) |
/// | `v` | `den.len()` |
/// | `u128_u` | `⌈(num.len() + 2) / 2⌉ + 1` — only for the base-2¹²⁸ arm |
/// | `u128_v` | `⌈den.len() / 2⌉` — only for the base-2¹²⁸ arm |
///
/// Those are the SLICE lengths, which is what a caller can size against. The
/// engines index by EFFECTIVE length (both strip trailing zero limbs first),
/// so a slice that is long by type but short by value needs correspondingly
/// less — which is why passing a generously-padded divisor slice is sound even
/// where the buffer could not hold its full nominal width.
///
/// The u128 minima are checked at the door and the arm **falls closed** to
/// base-2⁶⁴ Knuth when the caller's packed buffers are short. Both engines are
/// bit-identical (`div_knuth_u128_limb`'s own differential), so that guard can
/// only change which engine runs, never the value — it turns an
/// under-sized-buffer panic into a slower-but-correct divide, which is the
/// fail-closed length check `docs/ARCHITECTURE.md` → "The build-max divide
/// bound" asks for. `u` / `v` carry no such guard: they are the shared
/// requirement of BOTH Knuth arms, so a shortfall there has no correct
/// fallback and must fail loudly (the release slice bounds-check).
///
/// All four buffers may be **dirty** on entry — the engines write the prefixes
/// they read (`div_knuth_into`) or re-zero their whole scratch
/// (`div_knuth_u128_limb_into`), so they are reusable across calls.
#[allow(clippy::too_many_arguments)]
#[inline]
pub(crate) fn div_rem_into(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
    u: &mut [u64],
    v: &mut [u64],
    u128_u: &mut [u128],
    u128_v: &mut [u128],
) {
    match select_for_limbs(num, den) {
        Algorithm::Rem => div_rem(num, den, quot, rem),
        Algorithm::KnuthU128Limb => {
            if u128_u.len() >= (num.len() + 2).div_ceil(2) + 1
                && u128_v.len() >= den.len().div_ceil(2)
            {
                div_knuth_u128_limb_into(num, den, quot, rem, u, v, u128_u, u128_v);
            } else {
                div_knuth_into(num, den, quot, rem, u, v);
            }
        }
        Algorithm::Knuth | Algorithm::BurnikelZieglerWithKnuth | Algorithm::Schoolbook => {
            div_knuth_into(num, den, quot, rem, u, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::div_rem_into;
    use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
    use crate::int::types::compute_limbs::MAX_SINGLE_LIMBS;

    /// The exact-scratch door must be BIT-IDENTICAL to the build-max
    /// [`dispatch`] it replaces, at every divisor shape the matcher routes
    /// differently: a single-limb divisor (`Rem`), a narrow multi-limb divisor
    /// (`Knuth`), and the wide even-divisor `num_m >= 2*den_n` shape that
    /// reaches the base-2¹²⁸ arm. Also exercises buffer REUSE — the same
    /// scratch is fed dirty to every case — and the fail-closed u128 guard, by
    /// running the wide shape a second time with deliberately short packed
    /// buffers.
    #[test]
    fn exact_scratch_door_matches_build_max_dispatch() {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        // (num_len, den_len): single-limb divisor, narrow Knuth, and the
        // wide even `num_m >= 2*den_n` u128 shape (den_n = 24, 26, 32).
        let shapes: &[(usize, usize)] = &[
            (1, 1), (4, 1), (8, 1), (4, 2), (8, 3), (16, 9), (16, 16),
            (48, 24), (52, 26), (64, 32), (64, 24),
        ];
        // One dirty, reused scratch family, generously sized for every shape.
        let mut u = [0u64; 130];
        let mut v = [0u64; 130];
        let mut u128_u = [0u128; 67];
        let mut u128_v = [0u128; 67];
        // The build-max REFERENCE cannot take a dividend past its own
        // `MAX_SINGLE_LIMBS - 2` ceiling (`docs/ARCHITECTURE.md` → "The
        // build-max divide bound"), and that ceiling is feature-selected — so
        // the wide shapes are graded only in a build whose blanket can hold
        // them. The narrow shapes still assert in every build.
        let reference_ceiling = MAX_SINGLE_LIMBS.saturating_sub(2);
        let mut graded = 0usize;
        for (case, &(num_len, den_len)) in shapes.iter().enumerate() {
            if num_len > reference_ceiling {
                continue;
            }
            graded += 1;
            for round in 0..6 {
                let mut num = [0u64; 64];
                let mut den = [0u64; 64];
                for limb in num[..num_len].iter_mut() {
                    *limb = next();
                }
                for limb in den[..den_len].iter_mut() {
                    *limb = next();
                }
                // Pin both top limbs non-zero so the effective lengths are
                // exactly (num_len, den_len), and make some rounds exact
                // (`den` a power of two) to hit the zero-remainder path.
                num[num_len - 1] |= 1 << 63;
                den[den_len - 1] |= 1 << 63;
                if round % 3 == 0 {
                    for limb in den[..den_len - 1].iter_mut() {
                        *limb = 0;
                    }
                    den[den_len - 1] = 1 << 63;
                }

                let mut expected_quot = [0u64; 64];
                let mut expected_rem = [0u64; 64];
                div_rem_dispatch(
                    &num[..num_len], &den[..den_len],
                    &mut expected_quot[..num_len], &mut expected_rem[..den_len]);

                let mut quot = [0u64; 64];
                let mut rem = [0u64; 64];
                div_rem_into(
                    &num[..num_len], &den[..den_len],
                    &mut quot[..num_len], &mut rem[..den_len],
                    &mut u, &mut v, &mut u128_u, &mut u128_v);
                assert_eq!(quot, expected_quot,
                    "case {case} round {round}: quotient ({num_len}/{den_len})");
                assert_eq!(rem, expected_rem,
                    "case {case} round {round}: remainder ({num_len}/{den_len})");

                // Fail-closed guard: with packed buffers too short for the
                // u128 arm the door must still produce the same value.
                let mut short_u128_u = [0u128; 2];
                let mut short_u128_v = [0u128; 2];
                let mut guarded_quot = [0u64; 64];
                let mut guarded_rem = [0u64; 64];
                div_rem_into(
                    &num[..num_len], &den[..den_len],
                    &mut guarded_quot[..num_len], &mut guarded_rem[..den_len],
                    &mut u, &mut v, &mut short_u128_u, &mut short_u128_v);
                assert_eq!(guarded_quot, expected_quot,
                    "case {case} round {round}: guarded quotient ({num_len}/{den_len})");
                assert_eq!(guarded_rem, expected_rem,
                    "case {case} round {round}: guarded remainder ({num_len}/{den_len})");
            }
        }
        // Prove the mechanism FIRED: a filter that silently graded nothing
        // would pass this test while checking no divide at all. Five shapes
        // (through `Rem` and narrow `Knuth`) clear even the narrow default
        // build's ceiling of 8; a build wide enough for the u128 arm must
        // grade every shape, including the `num_m >= 2*den_n` ones.
        assert!(graded >= 5, "only {graded} shapes graded (ceiling {reference_ceiling})");
        if reference_ceiling >= 64 {
            assert_eq!(graded, shapes.len(), "wide build must grade the u128-arm shapes");
        }
    }
}
