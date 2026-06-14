// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Signed limbwise integer comparison over little-endian `u64` limb arrays.
//!
//! [`cmp_limbwise`] (same-width) and [`cmp_limbwise_cross`] (cross-width)
//! are the width-agnostic comparison algorithm selected by the comparison
//! policy [`crate::int::policy::cmp::dispatch`]. Pure kernels — sign-first
//! then unsigned-magnitude limbwise comparison; no algorithm choice.

use crate::int::algos::support::limbs::{cmp_cross, cmp_fixed};
use crate::int::types::Int;
use core::cmp::Ordering;

/// Signed limbwise comparison for `Int<N>`. Compares the sign bits first
/// (a negative value is always less than a non-negative one); when the
/// signs agree the magnitudes are compared via [`cmp_fixed`] for the
/// same-width case and flipped for two negatives (the larger magnitude is
/// the smaller value). Returns [`core::cmp::Ordering`].
///
/// Reuses the `cmp_fixed` kernel from [`crate::int::algos::support::limbs`] so
/// the comparison loop is not duplicated here.
#[inline]
pub(crate) const fn cmp_limbwise<const N: usize>(a: Int<N>, b: Int<N>) -> Ordering {
    let sn = a.is_negative();
    let so = b.is_negative();
    if sn && !so {
        return Ordering::Less;
    }
    if !sn && so {
        return Ordering::Greater;
    }
    // Same sign: compare magnitudes. `cmp_fixed` returns -1 / 0 / 1.
    let a_mag = a.unsigned_abs();
    let b_mag = b.unsigned_abs();
    let c = cmp_fixed(a_mag.as_limbs(), b_mag.as_limbs());
    // For two negatives the larger magnitude is the smaller value.
    let c = if sn { -c } else { c };
    if c < 0 {
        Ordering::Less
    } else if c > 0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Signed limbwise comparison for cross-width `Int<N>` vs `Int<M>`.
/// Uses [`cmp_cross`] for the magnitude comparison to handle different
/// limb counts. This is the algorithm fn backing the same policy for the
/// general `M != N` case.
#[inline]
pub(crate) const fn cmp_limbwise_cross<const N: usize, const M: usize>(
    a: Int<N>,
    b: Int<M>,
) -> Ordering {
    let sn = a.is_negative();
    let so = b.is_negative();
    if sn && !so {
        return Ordering::Less;
    }
    if !sn && so {
        return Ordering::Greater;
    }
    let a_mag = a.unsigned_abs();
    let b_mag = b.unsigned_abs();
    let c = cmp_cross(a_mag.as_limbs(), b_mag.as_limbs());
    let c = if sn { -c } else { c };
    if c < 0 {
        Ordering::Less
    } else if c > 0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::cmp_limbwise;
    use crate::int::types::Int;
    use core::cmp::Ordering;

    /// Positive vs negative: positive is greater.
    #[test]
    fn cmp_pos_gt_neg() {
        let a = Int::<1>::from_i64(1);
        let b = Int::<1>::from_i64(-1);
        assert_eq!(cmp_limbwise(a, b), Ordering::Greater);
        assert_eq!(cmp_limbwise(b, a), Ordering::Less);
    }

    /// Equal values compare Equal.
    #[test]
    fn cmp_equal_values() {
        let a = Int::<2>::from_i128(12345_i128);
        let b = Int::<2>::from_i128(12345_i128);
        assert_eq!(cmp_limbwise(a, b), Ordering::Equal);
    }

    /// Zero vs zero is Equal.
    #[test]
    fn cmp_zero_zero() {
        let z1 = Int::<1>::from_i64(0);
        let z2 = Int::<1>::from_i64(0);
        assert_eq!(cmp_limbwise(z1, z2), Ordering::Equal);
    }

    /// Magnitude ordering for two positives across limb boundary.
    #[test]
    fn cmp_large_positive_ordering() {
        let a = Int::<2>::from_u128(1_u128 << 64); // 2^64
        let b = Int::<2>::from_u128((1_u128 << 64) - 1); // 2^64 - 1
        assert_eq!(cmp_limbwise(a, b), Ordering::Greater);
        assert_eq!(cmp_limbwise(b, a), Ordering::Less);
    }

    /// For two negatives: the one with larger magnitude is smaller.
    #[test]
    fn cmp_two_negatives_magnitude_order() {
        let a = Int::<2>::from_i128(-100_i128);
        let b = Int::<2>::from_i128(-200_i128);
        // -100 > -200
        assert_eq!(cmp_limbwise(a, b), Ordering::Greater);
        assert_eq!(cmp_limbwise(b, a), Ordering::Less);
    }

    /// MIN < MAX at every width.
    #[test]
    fn cmp_min_lt_max() {
        let min1 = Int::<1>::from_i64(i64::MIN);
        let max1 = Int::<1>::from_i64(i64::MAX);
        assert_eq!(cmp_limbwise(min1, max1), Ordering::Less);

        let min2 = Int::<2>::from_i128(i128::MIN);
        let max2 = Int::<2>::from_i128(i128::MAX);
        assert_eq!(cmp_limbwise(min2, max2), Ordering::Less);
    }
}
