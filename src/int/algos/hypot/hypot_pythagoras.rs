// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_pythagoras` -- integer hypotenuse `round(sqrt(a^2 + b^2))`.
//!
//! The integer-tier core of the hypotenuse: given two `Int<N>` values it
//! forms the radicand `a^2 + b^2` (on the magnitudes -- the sign drops out
//! of squaring) in a limb scratch buffer spanning up to `2N` limbs, takes
//! the floor root via the width-agnostic slice kernel
//! [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`], then applies a
//! single round step (every [`RoundingMode`]). Returns [`None`] when the
//! rounded root does not fit the signed range of `Int<N>` (true overflow);
//! the caller maps that to its own out-of-range handling.
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

use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
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
    let mut root_buf = Limbs::<N>::double_buffered_u64();
    let root = root_buf.as_mut();
    isqrt_newton(&radicand[..radicand_len], &mut root[..radicand_len]);
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

    #[test]
    fn hypot_pythagoras_overflow_returns_none() {
        // a = b = MAX magnitude. a^2 + b^2 ~= 2*MAX^2, root ~= MAX*sqrt(2)
        // which exceeds the signed range -> None.
        let m = Int::<2>::MAX;
        assert_eq!(hypot_pythagoras::<2>(m, m, RoundingMode::HalfToEven), None);
    }
}
