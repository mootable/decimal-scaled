// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_schoolbook` -- naive integer hypotenuse reference.
//!
//! The correctness baseline (and benchmarkable seam) for the integer
//! hypotenuse: forms `a^2 + b^2` exactly in a limb scratch buffer and takes
//! the floor root with the division-free, float-free
//! [`crate::int::algos::isqrt::isqrt_schoolbook::isqrt_schoolbook`]
//! (two-bits-at-a-time bitwise digit-by-digit). Identical rounding step and
//! identical results to [`super::hypot_isqrt::hypot_isqrt`]; the only
//! difference is the root engine (pure-integer schoolbook vs Newton). Used
//! as the zero-dependency reference the optimised path is cross-checked
//! against.
//!
//! Returns [`None`] on true overflow (the rounded root does not fit the
//! signed range of `Int<N>`).

use crate::int::algos::isqrt::isqrt_schoolbook::isqrt_schoolbook;
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

/// `round(sqrt(a^2 + b^2))` via the bitwise schoolbook `isqrt`. Reference
/// baseline; numerically identical to
/// [`super::hypot_isqrt::hypot_isqrt`]. Returns [`None`] on true overflow.
#[inline]
#[must_use]
pub(crate) fn hypot_schoolbook<const N: usize>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>>
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

    // -- q = floor(sqrt(n)) via the bitwise schoolbook root --------------
    let mut q_buf = Int::<N>::work2();
    let q = q_buf.as_mut();
    isqrt_schoolbook(&n[..nl], &mut q[..nl]);
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
    use super::hypot_schoolbook;
    use crate::int::algos::hypot::hypot_isqrt::hypot_isqrt;
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
    fn hypot_schoolbook_pythagorean_8_15_17_all_modes() {
        let a = Int::<2>::from_i64(8);
        let b = Int::<2>::from_i64(15);
        let expected = Int::<2>::from_i64(17);
        for mode in ALL_MODES {
            assert_eq!(hypot_schoolbook::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    /// Cross-check: schoolbook matches the Newton-rooted kernel exactly
    /// across a grid of operands and every rounding mode.
    #[test]
    fn hypot_schoolbook_matches_isqrt_grid() {
        for a in 0i64..=40 {
            for b in 0i64..=40 {
                let ia = Int::<2>::from_i64(a);
                let ib = Int::<2>::from_i64(b);
                for mode in ALL_MODES {
                    assert_eq!(
                        hypot_schoolbook::<2>(ia, ib, mode),
                        hypot_isqrt::<2>(ia, ib, mode),
                        "a={a} b={b} mode {mode:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn hypot_schoolbook_overflow_returns_none() {
        let m = Int::<2>::MAX;
        assert_eq!(hypot_schoolbook::<2>(m, m, RoundingMode::HalfToEven), None);
    }
}
