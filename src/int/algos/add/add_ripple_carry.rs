// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Ripple-carry integer addition over little-endian `u64` limb arrays.
//!
//! [`add_ripple_carry`] is the width-agnostic addition algorithm selected by
//! the addition policy [`crate::int::policy::add::dispatch`]. Pure kernel —
//! it adds the two operands' limbs with carry propagation and wraps modulo
//! `2^BITS`; no algorithm choice.

use crate::int::algos::support::limbs::add_assign_fixed;
use crate::int::types::Int;

/// Ripple-carry integer addition for `Int<N>`. Wraps
/// [`add_assign_fixed`], discarding the carry-out so the result wraps
/// modulo `2^BITS` (two's-complement wrapping semantics).
#[inline]
pub(crate) const fn add_ripple_carry<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let mut limbs = *a.as_limbs();
    add_assign_fixed(&mut limbs, b.as_limbs());
    Int::<N>::from_limbs(limbs)
}
