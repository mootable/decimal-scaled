// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Naive repeated-multiply integer power algorithm.
//!
//! `pow_schoolbook` is the naive reference exponentiation algorithm for
//! [`crate::int::policy::pow`]. It computes `base^exp` modulo `2^BITS` by
//! performing `exp - 1` sequential multiplications of the accumulator by
//! `base`, starting from `base` itself (or 1 for `exp == 0`).
//!
//! This is the naive reference: O(exp) multiplications. The optimised
//! production algorithm
//! [`crate::int::algos::pow::pow_square_and_multiply::pow_square_and_multiply`]
//! uses binary exponentiation by squaring (O(log exp) multiplications) --
//! but the two are numerically identical modulo `2^BITS`.
//!
//! Per the layering rule (see `docs/ARCHITECTURE.md` -> "Layering
//! direction"), the algorithm computes via the const KERNEL
//! [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] -- it never calls
//! `pow`/`wrapping_pow`/`wrapping_mul`/`wrapping_sqr` methods back on
//! its own type.

use crate::int::algos::mul::mul_schoolbook::mul_low_fixed;
use crate::int::types::Uint;

/// Naive repeated-multiply integer power for `Uint<N>`: `base^exp` modulo
/// `2^BITS`; `base^0 == 1`.
///
/// Performs `exp - 1` sequential multiplications of an accumulator by
/// `base` via the const [`mul_low_fixed`] kernel: `acc = base`, then
/// `acc = acc * base` repeated `exp - 1` times. For `exp == 0` returns 1
/// directly (no multiplications). `const fn`, matching the const-ness of
/// the production algorithm.
#[inline]
#[allow(dead_code)]
pub(crate) const fn pow_schoolbook<const N: usize>(base: Uint<N>, exp: u32) -> Uint<N> {
    if exp == 0 {
        // base^0 = 1 for all bases.
        let mut one = [0u64; N];
        if N > 0 {
            one[0] = 1;
        }
        return Uint::<N>::from_limbs(one);
    }

    // Start accumulator at `base`; multiply by `base` for each
    // remaining factor (exp - 1 total multiplications).
    let mut acc = *base.as_limbs();
    let b = *base.as_limbs();
    let mut remaining = exp - 1;
    while remaining > 0 {
        let mut prod = [0u64; N];
        mul_low_fixed(&acc, &b, &mut prod);
        acc = prod;
        remaining -= 1;
    }
    Uint::<N>::from_limbs(acc)
}
