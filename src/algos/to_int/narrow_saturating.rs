// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The `Int<N>` → `i64` saturating narrow — the leaf both `to_int`
//! algorithms finish with.
//!
//! It is a leaf, not an algorithm: there is no choice to make here, so
//! it carries no `Algorithm` variant and sits below both kernels rather
//! than beside them. Keeping it in one place is what makes the two
//! kernels differ ONLY in how they round, which is what the policy is
//! choosing between.

use crate::int::types::Int;

/// Narrows a rounded whole-number `Int<N>` to `i64`, saturating to
/// `i64::MAX` / `i64::MIN` when it does not fit.
///
/// The bounds are built with the `const fn` `from_i128`, so a
/// monomorphisation folds them to compile-time constants. `i64::MAX` and
/// `i64::MIN` fit every `Int<N>` (`N >= 1` is at least 64 bits), so
/// neither construction can overflow at any width.
#[inline]
pub(crate) fn narrow_saturating_i64<const N: usize>(value: Int<N>) -> i64 {
    let i64_max = Int::<N>::from_i128(i64::MAX as i128);
    let i64_min = Int::<N>::from_i128(i64::MIN as i128);
    if value > i64_max {
        i64::MAX
    } else if value < i64_min {
        i64::MIN
    } else {
        value.as_i128() as i64
    }
}
