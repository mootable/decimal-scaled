// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `sqrt_newton` — Newton integer square root of `raw · 10^SCALE`, with a
//! single round step, computed directly over `u64` limbs.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the logical value
//! is `r / 10^SCALE`, so the square-root raw storage is
//! `round(sqrt(r · 10^SCALE))`. The radicand `|r| · 10^SCALE` is formed in
//! a local limb scratch buffer (it spans up to `2N` limbs) by ONE multiply
//! against the baked `10^SCALE` const-table entry, the exact integer square
//! root is taken via the int layer's width-agnostic slice kernel
//! ([`isqrt_newton_into`](crate::int::algos::isqrt::isqrt_newton::isqrt_newton_into)),
//! and a single round-to-nearest step lands the result on the type's last
//! representable place. Within 0.5 ULP under any rounding mode.
//!
//! # Exact scratch — every buffer sized from `N`, none from the build
//!
//! `N` is concrete here, so every working buffer comes from `ComputeLimbs` on
//! the `Limbs<N>` carrier — including the ones the ROOT needs. The slice
//! kernel has no `N` of its own, so its width-agnostic door would size four
//! Newton buffers and (per divide) two Knuth normalisation buffers from
//! `MAX_WORK_N`, which the build's WIDTH FEATURES select. That is the R10
//! defect — enabling `xx-wide` for a single D1232 value made every D57 sqrt
//! zero ~4× the buffer for identical work — so this kernel threads its own
//! scratch through the `_into` door and the cost tracks `N` instead.
//!
//! # Generic over the storage width only
//!
//! The kernel is generic over the storage limb count `N` and does the
//! work-width arithmetic in limbs — there is **no** `W = Int<2N>` work
//! *type* (which stable Rust cannot name from `N`), and therefore no
//! per-tier work-width binding in the policy. The integer work dispatches
//! *down* to the int layer's slice kernels: `isqrt_newton` for the root and
//! the multiply matcher's slice door
//! [`crate::int::policy::mul::dispatch_slice`] for the products (so the
//! schoolbook-vs-Karatsuba choice is the matcher's, not hardcoded), with the
//! `limbs` primitives for the rest.
//!
//! Returns `Int::<N>::ZERO` for `raw <= 0` (saturate-not-panic).

use crate::int::policy::mul::dispatch_slice as mul_slice;
use crate::int::algos::isqrt::isqrt_newton::isqrt_newton_into;
use crate::int::algos::support::limbs::{cmp_cross, is_zero, sub_assign};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Significant limb length of `limbs` (index of the highest non-zero limb
/// + 1), clamped to at least 1 so zero has length 1.
#[inline]
fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 1 && limbs[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Newton integer square-root kernel, computed in limbs.
///
/// `N` is the storage limb count backing `D<Int<N>, SCALE>`. The radicand
/// `|raw| · 10^scale` is built in scratch and rooted via the int layer's
/// `isqrt_newton`; the result is rounded and returned as `Int<N>`.
#[inline]
#[must_use]
pub(crate) fn sqrt_newton<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if raw <= Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }

    // ── radicand = |raw| · 10^scale, in limb scratch ────────────────────
    // ONE multiply against the baked `10^scale` table entry, not `scale`
    // multiplies by 10. The per-digit chain re-zeroed and rewrote the whole
    // product prefix once per decimal digit — at D616 scale 160 that is 160
    // passes over 30+ limbs to build a value the const table already holds.
    // `pow10_limbs` covers exp < 513 in EVERY build (the always-present NARROW
    // band), which spans every scale this kernel is routed at; the ×10 chain
    // survives only as the fallback for a scale past the enabled bands.
    let mut radicand_buf = Limbs::<N>::double_buffered_u64();
    let radicand = radicand_buf.as_mut();
    let magnitude = raw.unsigned_abs();
    let magnitude_limbs = magnitude.as_limbs();
    let magnitude_len = sig_len(magnitude_limbs);
    let radicand_len;
    if let Some(pow10) = crate::consts::pow10_limbs(scale) {
        let pow10_len = sig_len(pow10);
        // A product of an `m`-limb by a `p`-limb magnitude spans at most
        // `m + p` limbs. `radicand` is freshly zeroed, so the accumulating
        // multiply lands the exact value.
        let product_len = magnitude_len + pow10_len;
        debug_assert!(product_len <= radicand.len(), "sqrt radicand scratch overflow");
        mul_slice(&magnitude_limbs[..magnitude_len], &pow10[..pow10_len],
            &mut radicand[..product_len]);
        radicand_len = sig_len(&radicand[..product_len]);
    } else {
        // Deep tail beyond the (feature-gated) table: rebuild by the ×10
        // chain. Same value, so the two branches are interchangeable.
        radicand[..N].copy_from_slice(magnitude_limbs);
        let mut chain_len = sig_len(&radicand[..N]);
        let mut product_buf = Limbs::<N>::double_buffered_u64();
        let product = product_buf.as_mut();
        for _ in 0..scale {
            let product_len = chain_len + 1;
            for limb in product[..product_len].iter_mut() {
                *limb = 0;
            }
            mul_slice(&radicand[..chain_len], &[10u64], &mut product[..product_len]);
            radicand[..product_len].copy_from_slice(&product[..product_len]);
            chain_len = sig_len(&radicand[..product_len]);
        }
        radicand_len = chain_len;
    }

    // ── root = floor(sqrt(radicand)) via the int slice kernel ───────────
    // EXACT scratch, sourced here where `N` is concrete: the slice kernel has
    // no `N` of its own, so its build-max door would size four Newton buffers
    // and two Knuth normalisation buffers PER DIVIDE from `MAX_WORK_N` — a
    // width chosen by the build's features, not by this value. That is the
    // R10 defect (enabling `xx-wide` for one D1232 value slowing every D57
    // sqrt); the `_into` door takes these instead, so the cost tracks `N`.
    let mut root_buf = Limbs::<N>::double_buffered_u64();
    let root = root_buf.as_mut();
    let mut newton_x = Limbs::<N>::double_buffered_u64();
    let mut newton_q = Limbs::<N>::double_buffered_u64();
    let mut newton_y = Limbs::<N>::double_buffered_u64();
    // Knuth's normalised dividend needs `radicand_len + 2` limbs, and the
    // radicand spans up to `2N` — so this ONE buffer comes from the 4N family
    // rather than the 2N one: `2N + ⌈N/2⌉` meets `2N + 2` only from `N = 3` up,
    // and sizing it `4N` keeps the kernel correct at EVERY `N` instead of
    // leaning on the sqrt policy routing `N <= 2` elsewhere. Still exact
    // per-`N`, so it carries no feature coupling.
    let mut knuth_u = Limbs::<N>::quad_u64();
    let mut knuth_v = Limbs::<N>::double_buffered_u64();
    let mut knuth_u128_u = Limbs::<N>::double_buffered_u128();
    let mut knuth_u128_v = Limbs::<N>::double_buffered_u128();
    debug_assert!(radicand_len + 2 <= knuth_u.as_ref().len(), "sqrt Knuth scratch overflow");
    isqrt_newton_into(
        &radicand[..radicand_len],
        &mut root[..radicand_len],
        newton_x.as_mut(),
        newton_q.as_mut(),
        newton_y.as_mut(),
        knuth_u.as_mut(),
        knuth_v.as_mut(),
        knuth_u128_u.as_mut(),
        knuth_u128_v.as_mut(),
    );
    let root_len = sig_len(&root[..radicand_len]);

    // ── diff = radicand - root²  (root² ≤ radicand, so diff fits in
    //    radicand_len limbs) ──────────────────────────────────────────────
    let mut root_sq_buf = Limbs::<N>::double_buffered_u64();
    let root_sq = root_sq_buf.as_mut();
    let root_sq_cap = root_sq.len();
    mul_slice(&root[..root_len], &root[..root_len], &mut root_sq[..(2 * root_len).min(root_sq_cap)]);
    let mut diff_buf = Limbs::<N>::double_buffered_u64();
    let diff = diff_buf.as_mut();
    diff[..radicand_len].copy_from_slice(&radicand[..radicand_len]);
    sub_assign(&mut diff[..radicand_len], &root_sq[..radicand_len]);

    // ── single round step (matches the BigInt-generic kernel exactly) ───
    // halfway_round_up: remainder past the lower root exceeds the root
    // (diff > root); diff_nonzero: any remainder at all.
    let halfway_round_up = cmp_cross(&diff[..radicand_len], &root[..root_len]) > 0;
    let diff_nonzero = !is_zero(&diff[..radicand_len]);
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        // The radicand is non-negative, so up IS away from zero.
        RoundingMode::Ceiling | RoundingMode::AwayFromZero => diff_nonzero,
        // The last decimal digit spans the whole `root_len`-limb root.
        RoundingMode::ZeroFiveUp => {
            diff_nonzero
                && matches!(crate::support::rounding::limbs_mod_10(&root[..root_len]), 0 | 5)
        }
    };
    if bump {
        // root += 1 (carry stays within root_len+1 limbs).
        let mut i = 0;
        loop {
            let (sum, carry) = root[i].overflowing_add(1);
            root[i] = sum;
            if !carry {
                break;
            }
            i += 1;
        }
    }

    // ── narrow the root to Int<N> (positive; fits by construction) ──────
    let mut root_limbs = [0u64; N];
    root_limbs.copy_from_slice(&root[..N]);
    Int::<N>::from_limbs(root_limbs)
}

#[cfg(test)]
mod tests {
    use super::{sig_len, sqrt_newton};
    use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
    use crate::int::algos::support::limbs::{cmp_cross, is_zero, sub_assign};
    use crate::int::policy::mul::dispatch_slice as mul_slice;
    use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// The kernel EXACTLY as it stood before the exact-scratch / table-radicand
    /// change: the radicand built by one `mul_slice` by `[10]` per decimal
    /// digit, and the root taken through the build-max [`isqrt_newton`] door.
    /// Kept verbatim as the differential reference — the new kernel must
    /// reproduce it bit-for-bit at every value, scale and rounding mode,
    /// because both changes are supposed to move only WHERE the buffers come
    /// from and HOW `10^scale` is built, never a value.
    fn sqrt_newton_reference<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode)
        -> Int<N>
    where
        Limbs<N>: ComputeLimbs,
    {
        if raw <= Int::<N>::ZERO {
            return Int::<N>::ZERO;
        }
        let mut radicand_buf = Limbs::<N>::double_buffered_u64();
        let radicand = radicand_buf.as_mut();
        radicand[..N].copy_from_slice(raw.unsigned_abs().as_limbs());
        let mut radicand_len = sig_len(&radicand[..N]);
        {
            let mut product_buf = Limbs::<N>::double_buffered_u64();
            let product = product_buf.as_mut();
            for _ in 0..scale {
                let product_len = radicand_len + 1;
                for limb in product[..product_len].iter_mut() {
                    *limb = 0;
                }
                mul_slice(&radicand[..radicand_len], &[10u64], &mut product[..product_len]);
                radicand[..product_len].copy_from_slice(&product[..product_len]);
                radicand_len = sig_len(&radicand[..product_len]);
            }
        }
        let mut root_buf = Limbs::<N>::double_buffered_u64();
        let root = root_buf.as_mut();
        isqrt_newton(&radicand[..radicand_len], &mut root[..radicand_len]);
        let root_len = sig_len(&root[..radicand_len]);
        let mut root_sq_buf = Limbs::<N>::double_buffered_u64();
        let root_sq = root_sq_buf.as_mut();
        let root_sq_cap = root_sq.len();
        mul_slice(&root[..root_len], &root[..root_len],
            &mut root_sq[..(2 * root_len).min(root_sq_cap)]);
        let mut diff_buf = Limbs::<N>::double_buffered_u64();
        let diff = diff_buf.as_mut();
        diff[..radicand_len].copy_from_slice(&radicand[..radicand_len]);
        sub_assign(&mut diff[..radicand_len], &root_sq[..radicand_len]);
        let halfway_round_up = cmp_cross(&diff[..radicand_len], &root[..root_len]) > 0;
        let diff_nonzero = !is_zero(&diff[..radicand_len]);
        let bump = match mode {
            RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => halfway_round_up,
            RoundingMode::Trunc | RoundingMode::Floor => false,
            RoundingMode::Ceiling | RoundingMode::AwayFromZero => diff_nonzero,
            RoundingMode::ZeroFiveUp => {
                diff_nonzero
                    && matches!(crate::support::rounding::limbs_mod_10(&root[..root_len]), 0 | 5)
            }
        };
        if bump {
            let mut i = 0;
            loop {
                let (sum, carry) = root[i].overflowing_add(1);
                root[i] = sum;
                if !carry {
                    break;
                }
                i += 1;
            }
        }
        let mut root_limbs = [0u64; N];
        root_limbs.copy_from_slice(&root[..N]);
        Int::<N>::from_limbs(root_limbs)
    }

    /// Every `(raw, scale, mode)` in the cell must agree with the reference.
    /// Returns the number of comparisons made, so the caller can prove the
    /// mechanism fired rather than trusting a silent pass.
    fn check_cell<const N: usize>(scales: &[u32], raws: &[i128]) -> usize
    where
        Limbs<N>: ComputeLimbs,
    {
        let mut checked = 0usize;
        for &scale in scales {
            for &raw_value in raws {
                let raw = Int::<N>::from_i128(raw_value);
                for mode in ALL_MODES {
                    assert_eq!(
                        sqrt_newton::<N>(raw, scale, mode),
                        sqrt_newton_reference::<N>(raw, scale, mode),
                        "N={N} scale={scale} raw={raw_value} mode={mode:?}"
                    );
                    checked += 1;
                }
            }
        }
        checked
    }

    /// A DENSE magnitude filling all `N` storage limbs (top bit cleared so the
    /// value stays positive), graded at every mode. Small `i128` operands leave
    /// the wide tiers rooting a handful of limbs however large `N` is; only a
    /// full-width magnitude makes the radicand span ~`2N` limbs, which is what
    /// drives the Newton divide into the wide even-divisor `num_m >= 2*den_n`
    /// shape — the base-2¹²⁸ arm the exact-scratch door now routes itself.
    fn check_dense<const N: usize>(scales: &[u32]) -> usize
    where
        Limbs<N>: ComputeLimbs,
    {
        let mut limbs = [0u64; N];
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        for limb in limbs.iter_mut() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *limb = state;
        }
        limbs[N - 1] &= !(1u64 << 63);
        limbs[N - 1] |= 1 << 62;
        let raw = Int::<N>::from_mag_limbs(&limbs, false);
        let mut checked = 0usize;
        for &scale in scales {
            for mode in ALL_MODES {
                assert_eq!(
                    sqrt_newton::<N>(raw, scale, mode),
                    sqrt_newton_reference::<N>(raw, scale, mode),
                    "dense N={N} scale={scale} mode={mode:?}"
                );
                checked += 1;
            }
        }
        checked
    }

    /// Bit-identity wall for the exact-scratch + table-radicand change.
    ///
    /// The reference is the pre-change kernel, so a mismatch anywhere means one
    /// of the two changes moved a VALUE, not just a buffer. Widths are gated by
    /// the build's own width features because the REFERENCE is the build-max
    /// path: its `SCRATCH_LIMBS` is `2·MAX_WORK_N + ⌈MAX_WORK_N/2⌉`, so it
    /// cannot hold a radicand wider than the enabled tiers — the R10 defect
    /// this change removes, visible here as the reference's own ceiling.
    #[test]
    fn matches_pre_change_reference_all_modes() {
        // Values spanning perfect squares, near-zero, primes, a full u64 limb
        // and a value past the 128-bit line.
        let raws: &[i128] = &[
            1, 2, 3, 4, 7, 9, 100, 12345,
            999_999_999_999_999_999,
            170_141_183_460_469_231_731_687_303_715_884_105_i128,
        ];
        let small: &[i128] = &[1, 2, 3, 7, 12345, 999_999_999_999_999_999];
        let mut checked = 0usize;
        // Per-tier counters, so "the wide cells ran" is proven directly rather
        // than inferred from a total a narrow build could also reach.
        #[allow(unused_mut)]
        let mut wide_checked = 0usize;
        #[allow(unused_mut)]
        let mut x_wide_checked = 0usize;
        #[allow(unused_mut)]
        let mut xx_wide_checked = 0usize;

        // Narrow: the default build's build-max reference holds only a 4-limb
        // radicand, so `N <= 2` is all that is gradable without width features.
        checked += check_cell::<1>(&[0, 1, 7], &[1, 2, 3, 7, 12345]);
        checked += check_cell::<2>(&[0, 1, 7], raws);
        checked += check_cell::<2>(&[19], small);
        checked += check_dense::<1>(&[0, 1]);
        checked += check_dense::<2>(&[0, 1, 19]);

        #[cfg(feature = "wide")]
        {
            wide_checked += check_cell::<3>(&[0, 1, 7, 19, 30], raws);
            wide_checked += check_cell::<4>(&[0, 1, 19, 38], raws);
            wide_checked += check_cell::<6>(&[0, 1, 19, 23, 60], raws);
            wide_checked += check_cell::<8>(&[0, 1, 19, 31], raws);
            wide_checked += check_cell::<12>(&[0, 1, 19, 69], small);
            wide_checked += check_cell::<16>(&[0, 1, 19, 63], small);
            wide_checked += check_dense::<3>(&[0, 1, 19, 30]);
            wide_checked += check_dense::<4>(&[0, 19, 38]);
            wide_checked += check_dense::<6>(&[0, 19, 23]);
            wide_checked += check_dense::<8>(&[0, 19, 31]);
            wide_checked += check_dense::<12>(&[0, 19, 69]);
            wide_checked += check_dense::<16>(&[0, 19, 63]);
        }
        #[cfg(feature = "x-wide")]
        {
            x_wide_checked += check_cell::<24>(&[0, 1, 19, 95], small);
            x_wide_checked += check_cell::<32>(&[0, 19, 159], small);
            // From N = 24 the dense radicand spans 48+ limbs, so the Newton
            // divide's divisor reaches 24 EVEN limbs with a `2n` dividend —
            // the base-2¹²⁸ arm. These cells are the ones that grade it.
            x_wide_checked += check_dense::<24>(&[0, 19, 95]);
            x_wide_checked += check_dense::<32>(&[0, 19, 159]);
        }
        #[cfg(feature = "xx-wide")]
        {
            xx_wide_checked += check_cell::<48>(&[0, 19, 259], &[1, 2, 7]);
            xx_wide_checked += check_cell::<64>(&[0, 19, 255], &[1, 2, 7]);
            xx_wide_checked += check_dense::<48>(&[0, 19]);
            xx_wide_checked += check_dense::<64>(&[0, 19]);
        }
        checked += wide_checked + x_wide_checked + xx_wide_checked;

        // Prove the mechanism FIRED — a cfg-gated test that grades nothing
        // still exits 0.
        assert!(checked >= 400, "only {checked} comparisons made");
        #[cfg(feature = "wide")]
        assert!(wide_checked >= 1000, "wide cells graded only {wide_checked}");
        #[cfg(feature = "x-wide")]
        assert!(x_wide_checked >= 200, "x-wide cells graded only {x_wide_checked}");
        #[cfg(feature = "xx-wide")]
        assert!(xx_wide_checked >= 100, "xx-wide cells graded only {xx_wide_checked}");
    }
}
