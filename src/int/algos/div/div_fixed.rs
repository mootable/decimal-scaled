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
/// `num`, `den`, `quot`, `rem` are all `N`-limb magnitudes (sign handling
/// is the caller's; this is an unsigned division of the magnitudes). The
/// quotient and remainder are written into `quot` / `rem`.
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
    num: &[u64; N],
    den: &[u64; N],
    quot: &mut [u64; N],
    rem: &mut [u64; N],
) {
    if N == 1 {
        let n0 = num[0];
        let d0 = den[0];
        quot[0] = n0 / d0;
        rem[0] = n0 % d0;
    } else if N == 2 {
        let n = (num[0] as u128) | ((num[1] as u128) << 64);
        let d = (den[0] as u128) | ((den[1] as u128) << 64);
        let q = n / d;
        let r = n % d;
        quot[0] = q as u64;
        quot[1] = (q >> 64) as u64;
        rem[0] = r as u64;
        rem[1] = (r >> 64) as u64;
    } else {
        div_rem_dispatch(num, den, quot, rem);
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
pub(crate) fn div_rem_mag_slice(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    div_rem_dispatch(num, den, quot, rem);
}
