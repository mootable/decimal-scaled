// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `sum_sq_schoolbook` -- the integer sum of squares `a^2 + b^2`.
//!
//! The sqrt-free magnitude primitive: distance comparisons compare
//! `a^2 + b^2` instead of `sqrt(a^2 + b^2)` (the root is monotonic, so it
//! drops out of an ordering), so this is the cheap inner core shared by the
//! hypotenuse family. [`crate::int::algos::hypot::hypot_pythagoras`] forms
//! the same radicand and then takes [`isqrt`]; this file owns the radicand
//! former [`sum_sq_radicand`] so both paths share one source of truth.
//!
//! The work-width arithmetic is done in limbs -- no `W = Int<2N>` work type.
//! `a^2 + b^2` spans up to `2N + 1` limbs, formed in a [`ComputeLimbs`] `Buf2`
//! scratch (sized `2N + ceil(N/2)`, so the carry margin is covered). The
//! signs of the operands drop out of squaring, so the radicand is formed on
//! the magnitudes.
//!
//! The method form [`sum_sq_schoolbook`] returns [`None`] when the full
//! `a^2 + b^2` does not fit the signed range of `Int<N>` (it can be up to
//! one bit short of `2N` limbs); callers that need the full-width radicand
//! for a root use [`sum_sq_radicand`] directly (as hypot does).
//!
//! [`isqrt`]: crate::int::algos::isqrt::isqrt_newton::isqrt_newton

use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::add_assign;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;

/// Significant limb length of `limbs` (index of the highest non-zero limb
/// plus one), never below 1. Shared by the sum-of-squares and hypot kernels.
#[inline]
pub(crate) fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 1 && limbs[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Form `a^2 + b^2` (on the magnitude slices `a_magnitude` / `b_magnitude`)
/// into `out`, returning its significant limb length. `N` is the storage limb
/// count of the originating `Int<N>` operands, so each magnitude is `<= N`
/// limbs and the radicand fits the `Buf2` scratch the caller supplies (`out`
/// must be a freshly zeroed `Limbs::<N>::double_buffered_u64()`, i.e.
/// `>= 2N + 1` limbs). This is the single radicand former shared by
/// [`sum_sq_schoolbook`] and the hypot kernel.
#[inline]
pub(crate) fn sum_sq_radicand<const N: usize>(a_magnitude: &[u64], b_magnitude: &[u64],
    out: &mut [u64]) -> usize
where
    Limbs<N>: ComputeLimbs,
{
    let a_len = sig_len(a_magnitude);
    let b_len = sig_len(b_magnitude);
    // a^2 into `out` (zeroed by the caller); b^2 into its own scratch.
    mul_schoolbook(&a_magnitude[..a_len], &a_magnitude[..a_len], &mut out[..2 * a_len]);
    let mut b_squared_buf = Limbs::<N>::double_buffered_u64();
    let b_squared = b_squared_buf.as_mut();
    mul_schoolbook(&b_magnitude[..b_len], &b_magnitude[..b_len], &mut b_squared[..2 * b_len]);
    // accumulate into `out`; the +1 limb covers the addition carry.
    let span = (2 * a_len).max(2 * b_len) + 1;
    add_assign(&mut out[..span], &b_squared[..2 * b_len]);
    sig_len(&out[..span])
}

/// `a^2 + b^2` as an `Int<N>`, or [`None`] on true overflow (the sum does not
/// fit the signed non-negative range of `Int<N>`). `N` is the storage limb
/// count. The sum is always non-negative, so the fit test is the
/// signed-positive bound `< 2^(64N-1)`.
#[inline]
#[must_use]
pub(crate) fn sum_sq_schoolbook<const N: usize>(a: Int<N>, b: Int<N>) -> Option<Int<N>>
where
    Limbs<N>: ComputeLimbs,
{
    let a_magnitude = a.unsigned_abs();
    let b_magnitude = b.unsigned_abs();
    let mut radicand_buf = Limbs::<N>::double_buffered_u64();
    let radicand = radicand_buf.as_mut();
    let radicand_len =
        sum_sq_radicand::<N>(a_magnitude.as_limbs(), b_magnitude.as_limbs(), radicand);
    // fit check: positive magnitude must be < 2^(64N-1) (signed range).
    if radicand_len > N || (radicand_len == N && (radicand[N - 1] >> 63) != 0) {
        return None;
    }
    let mut out = [0u64; N];
    out.copy_from_slice(&radicand[..N]);
    Some(Int::<N>::from_limbs(out))
}

#[cfg(test)]
mod tests {
    use super::sum_sq_schoolbook;
    use crate::int::types::Int;

    #[test]
    fn sum_sq_3_4_is_25() {
        let a = Int::<2>::from_i64(3);
        let b = Int::<2>::from_i64(4);
        assert_eq!(sum_sq_schoolbook::<2>(a, b).unwrap().as_i128(), 25);
    }

    #[test]
    fn sum_sq_is_sign_independent() {
        // (-3)^2 + (-4)^2 == 3^2 + 4^2 == 25; the sign drops out of squaring.
        let r = sum_sq_schoolbook::<2>(Int::<2>::from_i64(-3), Int::<2>::from_i64(-4));
        assert_eq!(r.unwrap().as_i128(), 25);
    }

    #[test]
    fn sum_sq_zero_zero_is_zero() {
        let z = Int::<2>::from_i64(0);
        assert_eq!(sum_sq_schoolbook::<2>(z, z).unwrap().as_i128(), 0);
    }

    #[test]
    fn sum_sq_zero_x_is_x_squared() {
        let z = Int::<2>::from_i64(0);
        let x = Int::<2>::from_i64(123456789);
        assert_eq!(
            sum_sq_schoolbook::<2>(z, x).unwrap().as_i128(),
            123456789i128 * 123456789
        );
    }

    #[test]
    fn sum_sq_orders_like_hypot() {
        // The sqrt-free magnitude comparison: (5,12) is a larger hypot than
        // (3,4) iff 5^2+12^2 > 3^2+4^2, which the root preserves.
        let small = sum_sq_schoolbook::<2>(Int::<2>::from_i64(3), Int::<2>::from_i64(4)).unwrap();
        let large = sum_sq_schoolbook::<2>(Int::<2>::from_i64(5), Int::<2>::from_i64(12)).unwrap();
        assert!(large.as_i128() > small.as_i128());
        assert_eq!(small.as_i128(), 25);
        assert_eq!(large.as_i128(), 169);
    }

    #[test]
    fn sum_sq_overflow_returns_none() {
        // a = b = MAX: a^2 + b^2 ~= 2*MAX^2 needs ~2N limbs -> does not fit.
        let m = Int::<2>::MAX;
        assert_eq!(sum_sq_schoolbook::<2>(m, m), None);
    }
}
