// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Sqr-then-multiply integer cubing algorithm.
//!
//! `cube_schoolbook` is the cubing algorithm dispatched by
//! [`crate::int::policy::cube`]. The optimal form of `x³` is `x²·x` — two
//! limb operations rather than three sequential multiplies; no cheaper
//! form exists below two multiplications. The squaring step uses the const
//! half-product kernel [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`]; the
//! final multiply uses the const truncated-product kernel
//! [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`].
//!
//! Per the layering rule (see `docs/ARCHITECTURE.md` → "Layering
//! direction"), the algorithm computes via the const KERNELS — it never
//! calls `cube`/`sqr`/`mul` methods back on its own type. The type methods
//! `Uint<N>::cube` / `Int<N>::cube` (and the `wrapping_cube` siblings) are
//! thin delegators *down* to this algorithm via the policy dispatcher.

use crate::int::algos::mul::mul_schoolbook::mul_low_fixed;
use crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed;
use crate::int::types::Uint;

/// Sqr-then-multiply integer cube for `Uint<N>`: `x³` modulo `2^BITS`.
///
/// Computes `x²` with the const [`sqr_low_fixed`] kernel, then multiplies
/// by `x` with the const truncated [`mul_low_fixed`] kernel — `x²·x`.
/// Total limb-multiply count `N(N+1)/2 + N²`. `const fn`, so the
/// [`crate::int::policy::cube`] dispatcher and the type methods that
/// delegate to it can be `const fn` too.
#[inline]
pub(crate) const fn cube_schoolbook<const N: usize>(x: Uint<N>) -> Uint<N> {
    let limbs = x.as_limbs();
    let mut sq = [0u64; N];
    sqr_low_fixed(limbs, &mut sq);
    let mut out = [0u64; N];
    mul_low_fixed(&sq, limbs, &mut out);
    Uint::<N>::from_limbs(out)
}
