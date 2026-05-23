// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Half-product integer squaring algorithm.
//!
//! `sqr_half_product` is the squaring algorithm dispatched by
//! [`crate::int::policy::sqr`]. It computes `x²` modulo `2^BITS` for an
//! `N`-limb value by routing through the dedicated half-product squaring
//! kernel [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`]: each symmetric cross
//! term is formed once and doubled, so the limb-multiply count is
//! `N(N+1)/2` rather than the `N²` of a general multiply.
//!
//! Per the layering rule (see `docs/ARCHITECTURE.md` → "Layering
//! direction"), the algorithm computes via the const KERNEL — it never
//! calls a squaring method back on its own type. The type methods
//! `Uint<N>::sqr` / `Int<N>::sqr` (and the `wrapping_sqr` siblings) are
//! thin delegators *down* to this algorithm via the policy dispatcher.

use crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed;
use crate::int::types::Uint;

/// Half-product integer square for `Uint<N>`: `x²` modulo `2^BITS`.
///
/// Extracts the raw limbs via the const [`Uint::as_limbs`] accessor,
/// squares them with the const [`sqr_low_fixed`] kernel (forms each cross
/// term once and doubles it), and rebuilds the result via
/// [`Uint::from_limbs`]. `const fn`, so the [`crate::int::policy::sqr`]
/// dispatcher and the type methods that delegate to it can be `const fn`
/// too.
#[inline]
pub(crate) const fn sqr_half_product<const N: usize>(x: Uint<N>) -> Uint<N> {
    let mut out = [0u64; N];
    sqr_low_fixed(x.as_limbs(), &mut out);
    Uint::<N>::from_limbs(out)
}
