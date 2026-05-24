// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Naive schoolbook integer squaring algorithm.
//!
//! `sqr_schoolbook` is the naive reference squaring algorithm for
//! [`crate::int::policy::sqr`]. It computes `x²` modulo `2^BITS` for an
//! `N`-limb value by treating squaring as a general `x·x` multiply via
//! the const truncated kernel
//! [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`].
//!
//! This is the naive reference: every partial product `x_i·x_j` (including
//! symmetric cross terms) is formed separately. The optimised production
//! algorithm [`crate::int::algos::sqr::sqr_half_product::sqr_half_product`] halves
//! the limb-multiply count by exploiting symmetry — but the two are
//! bit-identical on the low `N` limbs.
//!
//! Per the layering rule (see `docs/ARCHITECTURE.md` → "Layering
//! direction"), the algorithm computes via the const KERNEL — it never
//! calls a squaring method back on its own type.

use crate::int::algos::mul::mul_schoolbook::mul_low_fixed;
use crate::int::types::Uint;

/// Naive schoolbook integer square for `Uint<N>`: `x²` modulo `2^BITS`.
///
/// Delegates directly to [`mul_low_fixed`] with both operands set to `x`,
/// forming the full `N×N` outer product and retaining only the low `N`
/// limbs. Bit-identical to [`crate::int::algos::sqr::sqr_half_product::sqr_half_product`];
/// slower by roughly 2× because symmetric cross terms are formed twice.
/// `const fn`, matching the const-ness of the production algorithm.
#[inline]
#[allow(dead_code)]
pub(crate) const fn sqr_schoolbook<const N: usize>(x: Uint<N>) -> Uint<N> {
    let mut out = [0u64; N];
    mul_low_fixed(x.as_limbs(), x.as_limbs(), &mut out);
    Uint::<N>::from_limbs(out)
}
