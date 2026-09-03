// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_pythagoras` -- integer hypotenuse `round(sqrt(a^2 + b^2))`.
//!
//! The integer-tier core of the hypotenuse: given two `Int<N>` values it
//! forms the radicand `a^2 + b^2` (on the magnitudes -- the sign drops out
//! of squaring) in a limb scratch buffer spanning up to `2N` limbs, takes
//! the floor root via the slice kernel's exact-scratch door
//! [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton_into`], then applies
//! a single round step (every [`RoundingMode`]). Returns [`None`] when the
//! rounded root does not fit the signed range of `Int<N>` (true overflow);
//! the caller maps that to its own out-of-range handling.
//!
//! # R10 -- enabling a width feature must not cost RUNTIME
//!
//! The root goes through `isqrt_newton_into` rather than the width-agnostic
//! `isqrt_newton` door **because this kernel already holds a concrete `N`**.
//! That door exists for callers with no `N` and must therefore size its five
//! working buffers from the build-max `MAX_WORK_N`, which is selected by the
//! build's WIDTH FEATURES -- so a consumer who enables `xx-wide` for one
//! D1232 value would make every narrower hypot zero the widest enabled tier's
//! width for work it never does. Sourcing the scratch here, from
//! [`ComputeLimbs`] on `Limbs<N>`, makes the cost track the operands instead
//! of the feature set.
//!
//! The decimal tier dispatches DOWN to this kernel (both decimal operands
//! carry the same `10^SCALE`, which cancels out of the root, so decimal
//! hypot is exactly int hypot on the raw storages).
//!
//! # Generic over the storage width only
//!
//! The work-width arithmetic is done in limbs -- no `W = Int<2N>` work
//! type. The kernel bounds only on `Limbs<N>: ComputeLimbs` for its scratch.
//!
//! Semantics: `hypot(0, 0) = 0`; `hypot(0, x) = |x|`.

use crate::int::algos::isqrt::isqrt_newton::isqrt_newton_into;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::sum_sq::sum_sq_schoolbook::{sig_len, sum_sq_radicand};
use crate::int::algos::support::limbs::{cmp_cross, is_zero, sub_assign};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `round(sqrt(a^2 + b^2))` via the int slice `isqrt`. `N` is the storage
/// limb count of the `Int<N>` operands. Returns [`None`] on true overflow
/// (the rounded root does not fit the signed range of `Int<N>`).
#[inline]
#[must_use]
pub(crate) fn hypot_pythagoras<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Limbs<N>: ComputeLimbs,
{
    // -- radicand = a^2 + b^2 (magnitudes; sign drops out of squaring) ---
    // The radicand former is shared with `sum_sq`; hypot roots the radicand
    // rather than fit-checking it, so it keeps every representable hypot.
    let a_magnitude = a.unsigned_abs();
    let b_magnitude = b.unsigned_abs();
    let mut radicand_buf = Limbs::<N>::double_buffered_u64();
    let radicand = radicand_buf.as_mut();
    let radicand_len =
        sum_sq_radicand::<N>(a_magnitude.as_limbs(), b_magnitude.as_limbs(), radicand);
    if radicand_len == 1 && radicand[0] == 0 {
        return Some(Int::<N>::ZERO);
    }

    // -- root = floor(sqrt(radicand)) ------------------------------------
    // EXACT scratch, sourced here where `N` is concrete: the slice root kernel
    // has no `N` of its own, so its build-max door sizes three Newton buffers
    // and two Knuth normalisation buffers from `MAX_WORK_N` -- a width chosen
    // by the build's WIDTH FEATURES, not by these operands. That is the R10
    // defect (enabling `xx-wide` for one D1232 value slowing every narrower
    // hypot); the `_into` door takes these instead, so the cost tracks `N`.
    // Same shape `crate::algos::sqrt::sqrt_newton` already uses.
    let mut root_buf = Limbs::<N>::double_buffered_u64();
    let root = root_buf.as_mut();
    let mut newton_x = Limbs::<N>::double_buffered_u64();
    let mut newton_q = Limbs::<N>::double_buffered_u64();
    let mut newton_y = Limbs::<N>::double_buffered_u64();
    // Knuth's normalised dividend needs `radicand_len + 2` limbs. The radicand
    // is `a^2 + b^2` on two `Int<N>` magnitudes, each `< 2^(64N-1)`, so it is
    // `<= 2^(128N-1)` and spans at most `2N` limbs -- meaning this ONE buffer
    // comes from the 4N family rather than the 2N one: `2N + ceil(N/2)` meets
    // `2N + 2` only from `N = 3` up, and sizing it `4N` keeps the kernel
    // correct at EVERY `N` instead of leaning on the hypot policy routing
    // `N <= 2` to the scalar fast arm elsewhere. Still exact per-`N`, so it
    // carries no feature coupling.
    let mut knuth_u = Limbs::<N>::quad_u64();
    let mut knuth_v = Limbs::<N>::double_buffered_u64();
    // Packed base-2^128 divide scratch. The build-max door passes empty slices
    // here, so its guard always falls closed to base-2^64 Knuth; sized per-`N`
    // these are free, and `div_rem_into`'s length guard keeps the choice
    // bit-identical either way (it can only change WHICH engine runs).
    let mut knuth_u128_u = Limbs::<N>::double_buffered_u128();
    let mut knuth_u128_v = Limbs::<N>::double_buffered_u128();
    debug_assert!(radicand_len + 2 <= knuth_u.as_ref().len(), "hypot Knuth scratch overflow");
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

    // -- diff = radicand - root^2  (reuse `radicand` in place as the
    //    remainder) -----------------------------------------------------
    let mut root_sq_buf = Limbs::<N>::double_buffered_u64();
    let root_sq = root_sq_buf.as_mut();
    let root_sq_cap = root_sq.len();
    mul_schoolbook(&root[..root_len], &root[..root_len],
        &mut root_sq[..(2 * root_len).min(root_sq_cap)]);
    sub_assign(&mut radicand[..radicand_len], &root_sq[..radicand_len]);
    let halfway_round_up = cmp_cross(&radicand[..radicand_len], &root[..root_len]) > 0;
    let diff_nonzero = !is_zero(&radicand[..radicand_len]);
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        // A hypotenuse is non-negative, so up IS away from zero.
        RoundingMode::Ceiling | RoundingMode::AwayFromZero => diff_nonzero,
        // The last decimal digit spans the whole `root_len`-limb root.
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

    // -- fit check: positive magnitude must be < 2^(64N-1) (signed range) -
    let root_fit_len = sig_len(&root[..(N + 2).min(root_sq_cap)]);
    if root_fit_len > N || (root_fit_len == N && (root[N - 1] >> 63) != 0) {
        return None;
    }
    let mut out = [0u64; N];
    out.copy_from_slice(&root[..N]);
    Some(Int::<N>::from_limbs(out))
}

#[cfg(test)]
mod tests {
    use super::hypot_pythagoras;
    use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
    use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
    use crate::int::algos::sum_sq::sum_sq_schoolbook::{sig_len, sum_sq_radicand};
    use crate::int::algos::support::limbs::{cmp_cross, is_zero, sub_assign};
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

    #[test]
    fn hypot_pythagoras_pythagorean_3_4_5_all_modes() {
        let a = Int::<2>::from_i64(3);
        let b = Int::<2>::from_i64(4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_pythagoras_pythagorean_5_12_13_all_modes() {
        let a = Int::<2>::from_i64(5);
        let b = Int::<2>::from_i64(12);
        let expected = Int::<2>::from_i64(13);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_pythagoras_non_perfect_1_1() {
        let a = Int::<2>::from_i64(1);
        let b = Int::<2>::from_i64(1);
        assert_eq!(hypot_pythagoras::<2>(a, b, RoundingMode::Trunc).unwrap().as_i128(), 1);
        assert_eq!(hypot_pythagoras::<2>(a, b, RoundingMode::Ceiling).unwrap().as_i128(), 2);
        assert_eq!(hypot_pythagoras::<2>(a, b, RoundingMode::HalfToEven).unwrap().as_i128(), 1);
    }

    #[test]
    fn hypot_pythagoras_zero_zero() {
        let z = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(z, z, mode), Some(z), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_pythagoras_zero_x_equals_abs_x() {
        let z = Int::<2>::from_i64(0);
        let x = Int::<2>::from_i64(42);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(z, x, mode), Some(x), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_pythagoras_negative_inputs() {
        let a = Int::<2>::from_i64(-3);
        let b = Int::<2>::from_i64(-4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    /// The kernel EXACTLY as it stood before the exact-scratch change: the
    /// floor root taken through the build-max [`isqrt_newton`] door. Kept
    /// verbatim as the differential reference — the new kernel must reproduce
    /// it bit-for-bit at every operand pair and rounding mode, because the
    /// change is supposed to move only WHERE the Newton/Knuth buffers come
    /// from, never a value. (It does also let the base-2¹²⁸ divide arm engage,
    /// which the empty packed slices of the build-max door always fell closed
    /// on — that engine carries its own differential, so this test is what
    /// proves the two agree end to end.)
    fn hypot_pythagoras_reference<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode)
        -> Option<Int<N>>
    where
        Limbs<N>: ComputeLimbs,
    {
        let a_magnitude = a.unsigned_abs();
        let b_magnitude = b.unsigned_abs();
        let mut radicand_buf = Limbs::<N>::double_buffered_u64();
        let radicand = radicand_buf.as_mut();
        let radicand_len =
            sum_sq_radicand::<N>(a_magnitude.as_limbs(), b_magnitude.as_limbs(), radicand);
        if radicand_len == 1 && radicand[0] == 0 {
            return Some(Int::<N>::ZERO);
        }
        let mut root_buf = Limbs::<N>::double_buffered_u64();
        let root = root_buf.as_mut();
        isqrt_newton(&radicand[..radicand_len], &mut root[..radicand_len]);
        let root_len = sig_len(&root[..radicand_len]);
        let mut root_sq_buf = Limbs::<N>::double_buffered_u64();
        let root_sq = root_sq_buf.as_mut();
        let root_sq_cap = root_sq.len();
        mul_schoolbook(&root[..root_len], &root[..root_len],
            &mut root_sq[..(2 * root_len).min(root_sq_cap)]);
        sub_assign(&mut radicand[..radicand_len], &root_sq[..radicand_len]);
        let halfway_round_up = cmp_cross(&radicand[..radicand_len], &root[..root_len]) > 0;
        let diff_nonzero = !is_zero(&radicand[..radicand_len]);
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
        let root_fit_len = sig_len(&root[..(N + 2).min(root_sq_cap)]);
        if root_fit_len > N || (root_fit_len == N && (root[N - 1] >> 63) != 0) {
            return None;
        }
        let mut out = [0u64; N];
        out.copy_from_slice(&root[..N]);
        Some(Int::<N>::from_limbs(out))
    }

    /// Every `(a, b, mode)` in the cell must agree with the reference. Returns
    /// the number of comparisons made, so the caller can prove the mechanism
    /// fired rather than trusting a silent pass.
    fn check_pairs<const N: usize>(pairs: &[(i128, i128)]) -> usize
    where
        Limbs<N>: ComputeLimbs,
    {
        let mut checked = 0usize;
        for &(a_value, b_value) in pairs {
            let a = Int::<N>::from_i128(a_value);
            let b = Int::<N>::from_i128(b_value);
            for mode in ALL_MODES {
                assert_eq!(
                    hypot_pythagoras::<N>(a, b, mode),
                    hypot_pythagoras_reference::<N>(a, b, mode),
                    "N={N} a={a_value} b={b_value} mode={mode:?}"
                );
                checked += 1;
            }
        }
        checked
    }

    /// DENSE magnitudes filling all `N` storage limbs, graded at every mode.
    /// Small `i128` operands leave the wide tiers rooting a handful of limbs
    /// however large `N` is; only full-width magnitudes make the radicand span
    /// ~`2N` limbs, which is what drives the Newton divide into the wide
    /// even-divisor `num_m >= 2*den_n` shape — the base-2¹²⁸ arm the
    /// exact-scratch door now makes reachable. The top limb is capped at bit 61
    /// so the rounded root stays inside the signed range (an overflow `None`
    /// would still agree, but would grade nothing).
    fn check_dense<const N: usize>() -> usize
    where
        Limbs<N>: ComputeLimbs,
    {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut a_limbs = [0u64; N];
        let mut b_limbs = [0u64; N];
        for limb in a_limbs.iter_mut() {
            *limb = next();
        }
        for limb in b_limbs.iter_mut() {
            *limb = next();
        }
        for limbs in [&mut a_limbs, &mut b_limbs] {
            limbs[N - 1] &= (1u64 << 61) - 1;
            limbs[N - 1] |= 1 << 61;
        }
        let a = Int::<N>::from_mag_limbs(&a_limbs, false);
        let b = Int::<N>::from_mag_limbs(&b_limbs, true);
        let mut checked = 0usize;
        for mode in ALL_MODES {
            let got = hypot_pythagoras::<N>(a, b, mode);
            assert_eq!(got, hypot_pythagoras_reference::<N>(a, b, mode),
                "dense N={N} mode={mode:?}");
            assert!(got.is_some(), "dense N={N} mode={mode:?} must root, not overflow");
            checked += 1;
        }
        checked
    }

    /// Bit-identity wall for the exact-scratch change (R10).
    ///
    /// The reference is the pre-change kernel, so a mismatch anywhere means the
    /// change moved a VALUE, not just a buffer. Widths are gated by the build's
    /// own width features because the REFERENCE is the build-max path: its
    /// `SCRATCH_LIMBS` is `2·MAX_WORK_N + ⌈MAX_WORK_N/2⌉`, so it cannot hold a
    /// radicand wider than the enabled tiers — the R10 coupling this change
    /// removes, visible here as the reference's own ceiling.
    #[cfg(feature = "exact-scratch")]
    #[test]
    fn matches_pre_change_reference_all_modes() {
        // Pythagorean triples, near-zero, primes, a zero operand, a full u64
        // limb, and a pair past the 128-bit line.
        let pairs: &[(i128, i128)] = &[
            (0, 0), (0, 42), (3, 4), (5, 12), (1, 1), (2, 3), (7, 24),
            (999_999_999_999_999_999, 1),
            (999_999_999_999_999_999, 999_999_999_999_999_998),
            (170_141_183_460_469_231_731_687_303_715_884_105_i128, 7),
        ];
        let small: &[(i128, i128)] =
            &[(0, 0), (3, 4), (1, 1), (12345, 6789), (999_999_999_999_999_999, 1)];
        let mut checked = 0usize;
        // Per-tier counters, so "the wide cells ran" is proven directly rather
        // than inferred from a total a narrow build could also reach.
        #[allow(unused_mut)]
        let mut wide_checked = 0usize;
        #[allow(unused_mut)]
        let mut x_wide_checked = 0usize;
        #[allow(unused_mut)]
        let mut xx_wide_checked = 0usize;

        checked += check_pairs::<1>(&[(0, 0), (3, 4), (1, 1), (12345, 6789)]);
        checked += check_pairs::<2>(pairs);
        checked += check_dense::<1>();
        checked += check_dense::<2>();

        #[cfg(feature = "wide")]
        {
            wide_checked += check_pairs::<3>(pairs);
            wide_checked += check_pairs::<4>(pairs);
            wide_checked += check_pairs::<6>(small);
            wide_checked += check_pairs::<8>(small);
            wide_checked += check_pairs::<12>(small);
            wide_checked += check_pairs::<16>(small);
            wide_checked += check_dense::<3>();
            wide_checked += check_dense::<4>();
            wide_checked += check_dense::<6>();
            wide_checked += check_dense::<8>();
            wide_checked += check_dense::<12>();
            wide_checked += check_dense::<16>();
        }
        #[cfg(feature = "x-wide")]
        {
            x_wide_checked += check_pairs::<24>(small);
            x_wide_checked += check_pairs::<32>(small);
            // From N = 24 the dense radicand spans 48+ limbs, so the Newton
            // divide's divisor reaches 24 EVEN limbs with a `2n` dividend —
            // the base-2¹²⁸ arm. These cells are the ones that grade it.
            x_wide_checked += check_dense::<24>();
            x_wide_checked += check_dense::<32>();
        }
        #[cfg(feature = "xx-wide")]
        {
            xx_wide_checked += check_pairs::<48>(&[(0, 0), (3, 4), (12345, 6789)]);
            xx_wide_checked += check_pairs::<64>(&[(0, 0), (3, 4), (12345, 6789)]);
            xx_wide_checked += check_dense::<48>();
            xx_wide_checked += check_dense::<64>();
        }
        checked += wide_checked + x_wide_checked + xx_wide_checked;

        // Prove the mechanism FIRED — a cfg-gated test that grades nothing
        // still exits 0. The narrow floor is the exact narrow-build total:
        // (4 + 10) pairs × 8 modes + 2 dense cells × 8 modes = 128.
        assert!(checked >= 128, "only {checked} comparisons made");
        #[cfg(feature = "wide")]
        assert!(wide_checked >= 200, "wide tiers graded only {wide_checked}");
        #[cfg(feature = "x-wide")]
        assert!(x_wide_checked >= 90, "x-wide tiers graded only {x_wide_checked}");
        #[cfg(feature = "xx-wide")]
        assert!(xx_wide_checked >= 60, "xx-wide tiers graded only {xx_wide_checked}");
    }

    #[test]
    fn hypot_pythagoras_overflow_returns_none() {
        // a = b = MAX magnitude. a^2 + b^2 ~= 2*MAX^2, root ~= MAX*sqrt(2)
        // which exceeds the signed range -> None.
        let m = Int::<2>::MAX;
        assert_eq!(hypot_pythagoras::<2>(m, m, RoundingMode::HalfToEven), None);
    }
}
