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
