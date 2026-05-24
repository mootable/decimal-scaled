//! `div_native` -- decimal division via the hardware `i128` path, for narrow
//! storage widths (`N <= 2`, i.e. D18 / D38).
//!
//! When the storage fits a single `i128` (`N == 1` is `i64`-backed but
//! widens losslessly; `N == 2` *is* `i128`), the whole scale-then-divide can
//! be done in hardware integers instead of forming a `2N`-limb scaled
//! numerator in a scratch buffer and routing it through the slice divide.
//!
//! The kernel converts both operands to `i128` (`Int<N>::as_i128`, lossless
//! for `N <= 2`), computes `(a * 10^SCALE) / b` rounded under `mode` via the
//! shared hardware kernel
//! [`crate::algos::support::mg_divide::div_pow10_div_with`] (an `i128` fast
//! path with a `256`-bit fallback; returns `None` on a zero divisor or on
//! `i128` overflow of the quotient), and rebuilds `Int<N>`.
//!
//! # Overflow / divide-by-zero contract
//!
//! A zero divisor panics up front (matching `i128 /` and the `WidenScale`
//! kernel). `div_pow10_div_with` returns `None` for an out-of-range quotient
//! (and for the zero divisor it never sees, guarded here): debug-panic /
//! release-wrap per the standard integer contract. The release wrap is
//! re-derived as `(a * 10^SCALE) / b` with wrapping `Int<N>` arithmetic,
//! matching the naive form and the `WidenScale` wrap in range.
//!
//! # Layering
//!
//! All arithmetic is `i128` / `Int<N>` integer work dispatched DOWN to the
//! int layer and the shared `mg_divide` leaf; this fn never calls a decimal
//! method on its own value. Valid only for `N <= 2` (where `as_i128` is
//! lossless) -- [`crate::policy::div`] routes only `N == 1 | 2` here.

use crate::algos::support::mg_divide::div_pow10_div_with;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Hardware-`i128` decimal divide kernel for narrow storage (`N <= 2`).
///
/// Computes `(a * 10^SCALE) / b` rounded under `mode`. Panics on a zero
/// divisor. On `i128` overflow of the quotient, debug-panics / release-wraps
/// per the standard integer contract (the wrap re-derives via `Int<N>`
/// wrapping ops). `mult` is the pre-computed `10^SCALE` in `Int<N>`, used
/// only on the release-wrap path (the hardware kernel forms its own).
#[inline]
#[must_use]
pub(crate) fn div_native<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    if b == Int::<N>::ZERO {
        panic!("attempt to divide by zero");
    }
    let ai = a.as_i128();
    let bi = b.as_i128();
    match div_pow10_div_with::<SCALE>(ai, bi, mode) {
        Some(q) => Int::<N>::from_i128(q),
        None => {
            if cfg!(debug_assertions) {
                panic!("attempt to divide with overflow");
            }
            // Release wrap: (a * 10^SCALE) / b in wrapping Int<N> arithmetic.
            a.wrapping_mul(mult).wrapping_div(b)
        }
    }
}
