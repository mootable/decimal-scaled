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
