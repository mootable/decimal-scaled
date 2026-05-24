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
