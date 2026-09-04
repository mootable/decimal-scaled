// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The [`DecimalTranscendental`] trait — the width-generic
//! transcendental + root surface shared by every decimal type.
//!
//! Implemented by every width (`D18`, `D38`, `D57`, `D76`,
//! `D115`, `D153`, `D230`, `D307`, `D462`, `D616`, `D924`, `D1232`).
//! Each impl is a one-line delegator to the inherent method of the
//! same name, so the trait costs nothing at runtime — `#[inline]` on
//! every method lets LLVM erase the trait dispatch.
//!
//! Use it for width-generic helpers:
//!
//! ```ignore
//! use decimal_scaled::{DecimalTranscendental, DecimalConstants};
//! use decimal_scaled::rounding::RoundingMode;
//!
//! fn area_of_disc<T>(radius: T) -> T
//! where
//!     T: DecimalTranscendental + DecimalConstants
//!         + Copy + std::ops::Mul<Output = T>,
//! {
//!     T::pi() * radius * radius
//! }
//! ```
//!
//! # Scope
//!
//! Every transcendental and root that has an inherent method on every
//! width is exposed here in its **two-variant** form:
//!
//! - `<fn>_strict` — crate-default guard width + crate-default mode
//! - `<fn>_strict_with(mode)` — crate-default guard, caller mode
//!
//! # What's NOT on the trait
//!
//! - Joint kernels like `sin_cos` / `sinh_cosh` exist only on the
//!   wide-tier macros and aren't available on D18 / D38. Access
//!   them via the concrete type.
//! - Mathematical constants (`pi`, `tau`, `e`, …) live on the
//!   separate [`DecimalConstants`] trait so callers can opt in to just
//!   constants without pulling in the full transcendental surface.
//!
//! [`DecimalConstants`]: crate::DecimalConstants

use crate::support::rounding::RoundingMode;

/// Width-generic transcendental + root surface shared by every
/// decimal width. See the module-level docs for the four-variant
/// matrix convention and the per-method delegation contract.
pub trait DecimalTranscendental: Sized {
    // ── Logarithms ─────────────────────────────────────────────

    /// Natural log. See the log/exp implementation module for the
    /// algorithm.
    fn ln(self) -> Self;
    fn ln_with(self, mode: RoundingMode) -> Self;

    // `log1p(t) = ln(1 + t)`, domain `t > -1`. Present for API parity
    // and standards conformance; at fixed point it is equivalent to
    // `ln(1 + t)` at the same scale, not more accurate.

    fn log1p(self) -> Self;
    fn log1p_with(self, mode: RoundingMode) -> Self;

    // `expm1(x) = e^x - 1`, total over the argument (it tends to `-1` as
    // `x -> -inf`). Like `log1p` it is not more accurate than the
    // two-step form at fixed point, but it does reach slightly further:
    // the `- 1` is applied at the working scale, ahead of the range
    // check, so it is defined on `x <= ln(1 + MAX)` where `exp` stops at
    // `ln(MAX)` — a band `ln(1 + 1/MAX)` wide, a few hundredths.

    fn expm1(self) -> Self;
    fn expm1_with(self, mode: RoundingMode) -> Self;

    /// Log to caller-chosen base.
    fn log(self, base: Self) -> Self;
    fn log_with(self, base: Self, mode: RoundingMode) -> Self;

    /// Log base 2.
    fn log2(self) -> Self;
    fn log2_with(self, mode: RoundingMode) -> Self;

    /// Log base 10.
    fn log10(self) -> Self;
    fn log10_with(self, mode: RoundingMode) -> Self;

    // ── Exponentials ───────────────────────────────────────────

    fn exp(self) -> Self;
    fn exp_with(self, mode: RoundingMode) -> Self;

    fn exp2(self) -> Self;
    fn exp2_with(self, mode: RoundingMode) -> Self;

    // ── Power ──────────────────────────────────────────────────

    fn powf(self, exp: Self) -> Self;
    fn powf_with(self, exp: Self, mode: RoundingMode) -> Self;

    // ── Roots ──────────────────────────────────────────────────

    fn sqrt(self) -> Self;
    fn sqrt_with(self, mode: RoundingMode) -> Self;

    fn cbrt(self) -> Self;
    fn cbrt_with(self, mode: RoundingMode) -> Self;

    fn hypot(self, other: Self) -> Self;
    fn hypot_with(self, other: Self, mode: RoundingMode) -> Self;

    // ── Trig (forward) ─────────────────────────────────────────

    fn sin(self) -> Self;
    fn sin_with(self, mode: RoundingMode) -> Self;

    fn cos(self) -> Self;
    fn cos_with(self, mode: RoundingMode) -> Self;

    fn tan(self) -> Self;
    fn tan_with(self, mode: RoundingMode) -> Self;

    // ── Trig (inverse) ─────────────────────────────────────────

    fn atan(self) -> Self;
    fn atan_with(self, mode: RoundingMode) -> Self;

    fn asin(self) -> Self;
    fn asin_with(self, mode: RoundingMode) -> Self;

    fn acos(self) -> Self;
    fn acos_with(self, mode: RoundingMode) -> Self;

    /// `atan2(self, other)` — matches the f64 convention where
    /// `self` is `y` and `other` is `x`.
    fn atan2(self, other: Self) -> Self;
    fn atan2_with(self, other: Self, mode: RoundingMode) -> Self;

    // ── Hyperbolic ─────────────────────────────────────────────

    fn sinh(self) -> Self;
    fn sinh_with(self, mode: RoundingMode) -> Self;

    fn cosh(self) -> Self;
    fn cosh_with(self, mode: RoundingMode) -> Self;

    fn tanh(self) -> Self;
    fn tanh_with(self, mode: RoundingMode) -> Self;

    fn asinh(self) -> Self;
    fn asinh_with(self, mode: RoundingMode) -> Self;

    fn acosh(self) -> Self;
    fn acosh_with(self, mode: RoundingMode) -> Self;

    fn atanh(self) -> Self;
    fn atanh_with(self, mode: RoundingMode) -> Self;

    // ── Angle conversion ───────────────────────────────────────

    fn to_degrees(self) -> Self;
    fn to_degrees_with(self, mode: RoundingMode) -> Self;

    fn to_radians(self) -> Self;
    fn to_radians_with(self, mode: RoundingMode) -> Self;
}
