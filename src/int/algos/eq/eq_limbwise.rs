// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Limb-by-limb integer equality over little-endian `u64` limb arrays.
//!
//! [`eq_limbwise`] is the width-agnostic equality algorithm selected by the
//! equality policy [`crate::int::policy::eq::dispatch`]. Pure kernel — a
//! limb-by-limb two's-complement equality test; no algorithm choice.

use crate::int::algos::support::limbs::cmp_fixed;
use crate::int::types::Int;

/// Limb-by-limb two's-complement equality for `Int<N>`. Delegates to
/// [`cmp_fixed`] (the equal-length comparison primitive): two values are
/// equal iff their `cmp_fixed` result is `0`. Reuses the comparison
/// kernel so the limb loop is not duplicated here.
#[inline]
pub(crate) const fn eq_limbwise<const N: usize>(a: Int<N>, b: Int<N>) -> bool {
    cmp_fixed(a.as_limbs(), b.as_limbs()) == 0
}

#[cfg(test)]
mod tests {
    use super::eq_limbwise;
    use crate::int::types::Int;

    /// Same value is equal.
    #[test]
    fn eq_same_value() {
        let a = Int::<1>::from_i64(42);
        let b = Int::<1>::from_i64(42);
        assert!(eq_limbwise(a, b));
    }

    /// Different values are not equal.
    #[test]
    fn eq_different_values() {
        let a = Int::<1>::from_i64(1);
        let b = Int::<1>::from_i64(2);
        assert!(!eq_limbwise(a, b));
    }

    /// Zero equals zero.
    #[test]
    fn eq_zeros() {
        let z1 = Int::<2>::from_i64(0);
        let z2 = Int::<2>::from_i64(0);
        assert!(eq_limbwise(z1, z2));
    }

    /// Positive vs its negation: not equal (unless zero).
    #[test]
    fn eq_pos_vs_neg_not_equal() {
        let a = Int::<1>::from_i64(7);
        let b = Int::<1>::from_i64(-7);
        assert!(!eq_limbwise(a, b));
    }

    /// Multi-limb equality: value spanning two limbs equals itself.
    #[test]
    fn eq_multi_limb_large_value() {
        let v = Int::<2>::from_u128(u128::MAX);
        assert!(eq_limbwise(v, v));
    }

    /// Multi-limb inequality: differ only in the high limb.
    #[test]
    fn eq_multi_limb_differ_high_limb() {
        let a = Int::<2>::from_u128(1_u128 << 64); // high limb = 1
        let b = Int::<2>::from_u128(2_u128 << 64); // high limb = 2
        assert!(!eq_limbwise(a, b));
    }

    /// Wide (3-limb) equality.
    #[test]
    fn eq_three_limb_equal() {
        let a = Int::<3>::from_i128(i128::MIN);
        let b = Int::<3>::from_i128(i128::MIN);
        assert!(eq_limbwise(a, b));
    }
}
