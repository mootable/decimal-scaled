// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Const-`N` fast-arm integer square root wrapper.
//!
//! [`isqrt_mag_fixed`] mirrors
//! [`crate::int::algos::div::div_fixed::div_rem_mag_fixed`]: native
//! single-/double-limb fast arms in front of the width-agnostic Newton
//! kernel [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`] for the
//! fixed-width `Int<N>` types.

use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;

/// Const-`N` fast-arm integer square root over little-endian u64
/// magnitude limbs. Writes `floor(sqrt(n))` into `out`.
///
/// Mirrors `div_rem_mag_fixed`: `N == 1` uses the native `u64::isqrt`,
/// `N == 2` uses `u128::isqrt`, and `N >= 3` falls through to the shared
/// [`isqrt_newton`] (Newton with a hardware-`f64::sqrt` seed). All arms
/// return the identical floor square root.
#[inline]
pub(crate) fn isqrt_mag_fixed<const N: usize>(n: &[u64; N], out: &mut [u64; N]) {
    if N == 1 {
        out[0] = n[0].isqrt();
    } else if N == 2 {
        let v = (n[0] as u128) | ((n[1] as u128) << 64);
        let r = v.isqrt();
        out[0] = r as u64;
        out[1] = (r >> 64) as u64;
    } else {
        isqrt_newton(n, out);
    }
}
