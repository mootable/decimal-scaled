// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_pythagoras` -- integer hypotenuse `round(sqrt(a^2 + b^2))`.
//!
//! The integer-tier core of the hypotenuse: given two `Int<N>` values it
//! forms the radicand `a^2 + b^2` (on the magnitudes -- the sign drops out
//! of squaring) in a limb scratch buffer spanning up to `2N` limbs, takes
//! the floor root via the width-agnostic slice kernel
//! [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`], then applies a
//! single round step (all six [`RoundingMode`]s). Returns [`None`] when the
//! rounded root does not fit the signed range of `Int<N>` (true overflow);
//! the caller maps that to its own out-of-range handling.
//!
//! This is the same arithmetic the decimal hypot used to inline; the
//! decimal tier now dispatches DOWN to this kernel (both decimal operands
//! carry the same `10^SCALE`, which cancels out of the root, so decimal
//! hypot is exactly int hypot on the raw storages).
//!
//! # Generic over the storage width only
//!
//! The work-width arithmetic is done in limbs -- no `W = Int<2N>` work
//! type. The kernel bounds only on `Int<N>: WorkScratch` for its scratch.
//!
//! Semantics: `hypot(0, 0) = 0`; `hypot(0, x) = |x|`.

use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{add_assign, cmp_cross, is_zero, sub_assign};
use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// `round(sqrt(a^2 + b^2))` via the int slice `isqrt`. `N` is the storage
/// limb count of the `Int<N>` operands. Returns [`None`] on true overflow
/// (the rounded root does not fit the signed range of `Int<N>`).
#[inline]
#[must_use]
pub(crate) fn hypot_pythagoras<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Int<N>: WorkScratch,
{
    // -- n = a^2 + b^2 (magnitudes; sign drops out of squaring) ----------
    let ma = a.unsigned_abs();
    let mb = b.unsigned_abs();
    let la = sig_len(ma.as_limbs());
    let lb = sig_len(mb.as_limbs());
    let mut n_buf = Int::<N>::work2();
    let n = n_buf.as_mut();
    mul_schoolbook(&ma.as_limbs()[..la], &ma.as_limbs()[..la], &mut n[..2 * la]);
    let mut bsq_buf = Int::<N>::work2();
    let bsq = bsq_buf.as_mut();
    mul_schoolbook(&mb.as_limbs()[..lb], &mb.as_limbs()[..lb], &mut bsq[..2 * lb]);
    let span = (2 * la).max(2 * lb) + 1;
    add_assign(&mut n[..span], &bsq[..2 * lb]);
    let nl = sig_len(&n[..span]);
    if nl == 1 && n[0] == 0 {
        return Some(Int::<N>::ZERO);
    }

    // -- q = floor(sqrt(n)) ----------------------------------------------
    let mut q_buf = Int::<N>::work2();
    let q = q_buf.as_mut();
    isqrt_newton(&n[..nl], &mut q[..nl]);
    let ql = sig_len(&q[..nl]);

    // -- diff = n - q^2  (reuse `n` in place as the remainder) -----------
    let mut qsq_buf = Int::<N>::work2();
    let qsq = qsq_buf.as_mut();
    let qsq_cap = qsq.len();
    mul_schoolbook(&q[..ql], &q[..ql], &mut qsq[..(2 * ql).min(qsq_cap)]);
    sub_assign(&mut n[..nl], &qsq[..nl]);
    let halfway_round_up = cmp_cross(&n[..nl], &q[..ql]) > 0;
    let diff_nonzero = !is_zero(&n[..nl]);
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    if bump {
        let mut i = 0;
        loop {
            let (v, c) = q[i].overflowing_add(1);
            q[i] = v;
            if !c {
                break;
            }
            i += 1;
        }
    }

    // -- fit check: positive magnitude must be < 2^(64N-1) (signed range) -
    let qfl = sig_len(&q[..(N + 2).min(qsq_cap)]);
    if qfl > N || (qfl == N && (q[N - 1] >> 63) != 0) {
        return None;
    }
    let mut out = [0u64; N];
    out.copy_from_slice(&q[..N]);
    Some(Int::<N>::from_limbs(out))
}

#[cfg(test)]
mod tests {
    use super::hypot_pythagoras;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
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
