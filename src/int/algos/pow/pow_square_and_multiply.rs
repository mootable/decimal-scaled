// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Binary exponentiation-by-squaring integer power algorithm.
//!
//! `pow_square_and_multiply` is the exponentiation algorithm dispatched by
//! [`crate::int::policy::pow`]. It computes `base^exp` modulo `2^BITS` for
//! an `N`-limb base via the classic binary square-and-multiply loop: at
//! each set bit of the exponent multiply the accumulator by the running
//! square, then square the running base. Optimal for the small fixed
//! exponents `pow` is used with in this crate (root iterations: `k-1`, `k`
//! ≤ ~10).
//!
//! Per the layering rule (see `docs/ARCHITECTURE.md` → "Layering
//! direction"), the loop computes via the const KERNELS
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] (square step) and
//! [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] (multiply step) — it never
//! calls `pow`/`wrapping_pow`/`wrapping_mul`/`wrapping_sqr` methods back on
//! its own type. The type methods `Uint<N>::pow` / `Int<N>::pow` (and the
//! `wrapping_pow` siblings) are thin delegators *down* to this algorithm
//! via the policy dispatcher.

use crate::int::algos::mul::mul_schoolbook::mul_low_fixed;
use crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed;
use crate::int::types::Uint;

/// Binary exponentiation by squaring for `Uint<N>`: `base^exp` modulo
/// `2^BITS`; `base^0 == 1`.
///
/// Operates directly on the raw limbs extracted via the const
/// [`Uint::as_limbs`] accessor, using the const [`sqr_low_fixed`] kernel
/// for the square step and the const truncated [`mul_low_fixed`] kernel
/// for the multiply step, then rebuilds via [`Uint::from_limbs`]. `const
/// fn`, so the [`crate::int::policy::pow`] dispatcher and the type methods
/// that delegate to it can be `const fn` too. The loop body is the port of
/// the historic `Uint::wrapping_pow`, with the kernel calls replacing the
/// method calls.
#[inline]
pub(crate) const fn pow_square_and_multiply<const N: usize>(base: Uint<N>, mut exp: u32) -> Uint<N> {
    // acc = 1 (multiplicative identity in raw limbs).
    let mut acc = [0u64; N];
    if N > 0 {
        acc[0] = 1;
    }
    let mut b = *base.as_limbs();
    while exp > 0 {
        if exp & 1 == 1 {
            let mut prod = [0u64; N];
            mul_low_fixed(&acc, &b, &mut prod);
            acc = prod;
        }
        exp >>= 1;
        if exp > 0 {
            let mut sq = [0u64; N];
            sqr_low_fixed(&b, &mut sq);
            b = sq;
        }
    }
    Uint::<N>::from_limbs(acc)
}
