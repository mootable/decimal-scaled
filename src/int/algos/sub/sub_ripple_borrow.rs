// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Ripple-borrow integer subtraction over little-endian `u64` limb arrays.
//!
//! [`sub_ripple_borrow`] is the width-agnostic subtraction algorithm selected
//! by the subtraction policy [`crate::int::policy::sub::dispatch`]. Pure
//! kernel — it subtracts the two operands' limbs with borrow propagation and
//! wraps modulo `2^BITS`; no algorithm choice.

use crate::int::algos::support::limbs::sub_assign_fixed;
use crate::int::types::Int;

/// Ripple-borrow integer subtraction for `Int<N>`. Wraps
/// [`sub_assign_fixed`], discarding the borrow-out so the result wraps
/// modulo `2^BITS` (two's-complement wrapping semantics).
#[inline]
pub(crate) const fn sub_ripple_borrow<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let mut limbs = *a.as_limbs();
    sub_assign_fixed(&mut limbs, b.as_limbs());
    Int::<N>::from_limbs(limbs)
}

#[cfg(test)]
mod tests {
    use super::sub_ripple_borrow;
    use crate::int::types::Int;

    /// 5 - 3 = 2 in a single-limb Int<1>.
    #[test]
    fn sub_simple_single_limb() {
        let a = Int::<1>::from_i64(5);
        let b = Int::<1>::from_i64(3);
        let got = sub_ripple_borrow(a, b);
        assert_eq!(got.as_i128(), 2);
    }

    /// Borrow propagates across the limb boundary: 2^64 - 1 in Int<2>
    /// must produce u64::MAX exactly (limb[0] = u64::MAX, limb[1] = 0).
    #[test]
    fn sub_borrow_across_limb_boundary() {
        let a = Int::<2>::from_u128(1_u128 << 64);
        let b = Int::<2>::from_i64(1);
        let got = sub_ripple_borrow(a, b);
        assert_eq!(got.as_i128(), u64::MAX as i128);
    }

    /// Wrapping: MIN<1> - 1 wraps to MAX<1>.
    #[test]
    fn sub_wraps_at_min() {
        let a = Int::<1>::from_i64(i64::MIN);
        let b = Int::<1>::from_i64(1);
        let got = sub_ripple_borrow(a, b);
        assert_eq!(got.as_i128(), i64::MAX as i128);
    }

    /// Subtracting self yields zero.
    #[test]
    fn sub_self_is_zero() {
        let v = Int::<2>::from_i128(-987_654_321_i128);
        let got = sub_ripple_borrow(v, v);
        assert_eq!(got.as_i128(), 0);
    }

    /// Subtracting zero is identity.
    #[test]
    fn sub_zero_identity() {
        let v = Int::<3>::from_i128(1_000_000_000_000_i128);
        let zero = Int::<3>::from_i64(0);
        assert_eq!(sub_ripple_borrow(v, zero).as_i128(), 1_000_000_000_000);
    }
}
