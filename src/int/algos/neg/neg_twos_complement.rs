// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Two's-complement integer negation over little-endian `u64` limb arrays.
//!
//! [`neg_twos_complement`] is the width-agnostic negation algorithm selected
//! by the negation policy [`crate::int::policy::neg::dispatch`]. Pure kernel
//! — bitwise-NOT then carry-propagating increment, wrapping modulo `2^BITS`;
//! no algorithm choice.

use crate::int::algos::support::limbs::add_assign_fixed;
use crate::int::types::Int;

/// Two's-complement negation for `Int<N>`: bitwise-NOT then increment
/// by one, wrapping modulo `2^BITS`. `MIN` maps to itself, matching
/// the primitive signed integer `wrapping_neg` contract.
///
/// Uses [`add_assign_fixed`] for the increment step so the carry
/// propagation is the same const-safe kernel the add policy uses.
#[inline]
pub(crate) const fn neg_twos_complement<const N: usize>(a: Int<N>) -> Int<N> {
    let mut out = [0u64; N];
    let mut i = 0;
    while i < N {
        out[i] = !a.as_limbs()[i];
        i += 1;
    }
    let mut one = [0u64; N];
    if N > 0 {
        one[0] = 1;
    }
    add_assign_fixed(&mut out, &one);
    Int::<N>::from_limbs(out)
}
