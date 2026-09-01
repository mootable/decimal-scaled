// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Const-`N` fast-arm divmod wrappers for the fixed-width `Int<N>` types.
//!
//! [`div_rem_mag_fixed`] / [`div_rem_mag_slice`] front the divisor-shape
//! policy [`crate::int::policy::div_rem`], adding the native single-/
//! double-limb fast arms the fixed-width `Int<N>` types lower to.

use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

/// Const-`N` fast-arm divmod over little-endian u64 magnitude limbs.
///
/// `dividend`, `divisor`, `quotient`, `remainder` are all `N`-limb
/// magnitudes (sign handling is the caller's; this is an unsigned division
/// of the magnitudes). The quotient and remainder are written into
/// `quotient` / `remainder`.
///
/// Because `N` is a compile-time constant, the `if N == …` ladder
/// const-folds per monomorphisation:
///
/// * `N == 1` lowers to a single native `u64` `/` + `%` (the hardware
///   `idiv`).
/// * `N == 2` widens to native `u128` `/` + `%`.
/// * `N >= 3` falls through to the shared [`div_rem_dispatch`] (Knuth-D /
///   Burnikel–Ziegler).
///
/// All three arms are behaviour-identical: truncating (Euclidean on
/// non-negative magnitudes) division. The divisor must be non-zero (the
/// caller guards this before delegating).
#[inline]
pub(crate) fn div_rem_mag_fixed<const N: usize>(
    dividend: &[u64; N],
    divisor: &[u64; N],
    quotient: &mut [u64; N],
    remainder: &mut [u64; N],
) {
    if N == 1 {
        let dividend_limb = dividend[0];
        let divisor_limb = divisor[0];
        quotient[0] = dividend_limb / divisor_limb;
        remainder[0] = dividend_limb % divisor_limb;
    } else if N == 2 {
        let dividend_u128 = (dividend[0] as u128) | ((dividend[1] as u128) << 64);
        let divisor_u128 = (divisor[0] as u128) | ((divisor[1] as u128) << 64);
        let quotient_u128 = dividend_u128 / divisor_u128;
        let remainder_u128 = dividend_u128 % divisor_u128;
        quotient[0] = quotient_u128 as u64;
        quotient[1] = (quotient_u128 >> 64) as u64;
        remainder[0] = remainder_u128 as u64;
        remainder[1] = (remainder_u128 >> 64) as u64;
    } else {
        div_rem_dispatch(dividend, divisor, quotient, remainder);
    }
}

/// Variable-length divmod over little-endian `u64` magnitude slices,
/// routed through the divisor-shape policy so the optimal engine
/// (hardware single-limb / Knuth / Burnikel–Ziegler) is selected at run
/// time. The int-algos-layer entry for callers whose operands have a
/// **runtime live length** that no const-`N` `Int<N>` width can express
/// (the reciprocal-table buffers in
/// [`crate::algos::support::newton_reciprocal`] are the one such caller):
/// it lets them reach the dispatching divmod without importing the
/// `int::policy` layer directly. Fixed-width `Int<N>` callers take
/// [`div_rem_mag_fixed`] instead. The divisor must be non-zero.
#[inline]
pub(crate) fn div_rem_mag_slice(dividend: &[u64], divisor: &[u64], quotient: &mut [u64],
    remainder: &mut [u64]) {
    div_rem_dispatch(dividend, divisor, quotient, remainder);
}

