//! The [`DecimalTranscendental`] trait — the width-generic
//! transcendental + root surface shared by every decimal type.
//!
//! Implemented by every width (`D9`, `D18`, `D38`, `D57`, `D76`,
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
//! width is exposed here in its **four-variant** form (where
//! applicable):
//!
//! - `<fn>_strict` — crate-default guard width + crate-default mode
//! - `<fn>_strict_with(mode)` — crate-default guard, caller mode
//! - `<fn>_approx(working_digits)` — caller guard, crate-default mode
//! - `<fn>_approx_with(working_digits, mode)` — both caller-chosen
//!
//! Roots (`sqrt` / `cbrt` / `hypot`) only ship the `_strict` /
//! `_strict_with` pair — the exact-integer-root path is precision-
//! independent, so there's no `working_digits` parameter to expose.
//!
//! # What's NOT on the trait
//!
//! - Joint kernels like `sin_cos` / `sinh_cosh` exist only on the
//!   wide-tier macros and aren't available on D9 / D18 / D38. Access
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
    fn ln_strict(self) -> Self;
    fn ln_strict_with(self, mode: RoundingMode) -> Self;
    fn ln_approx(self, working_digits: u32) -> Self;
    fn ln_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    /// Log to caller-chosen base.
    fn log_strict(self, base: Self) -> Self;
    fn log_strict_with(self, base: Self, mode: RoundingMode) -> Self;
    fn log_approx(self, base: Self, working_digits: u32) -> Self;
    fn log_approx_with(self, base: Self, working_digits: u32, mode: RoundingMode) -> Self;

    /// Log base 2.
    fn log2_strict(self) -> Self;
    fn log2_strict_with(self, mode: RoundingMode) -> Self;
    fn log2_approx(self, working_digits: u32) -> Self;
    fn log2_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    /// Log base 10.
    fn log10_strict(self) -> Self;
    fn log10_strict_with(self, mode: RoundingMode) -> Self;
    fn log10_approx(self, working_digits: u32) -> Self;
    fn log10_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Exponentials ───────────────────────────────────────────

    fn exp_strict(self) -> Self;
    fn exp_strict_with(self, mode: RoundingMode) -> Self;
    fn exp_approx(self, working_digits: u32) -> Self;
    fn exp_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn exp2_strict(self) -> Self;
    fn exp2_strict_with(self, mode: RoundingMode) -> Self;
    fn exp2_approx(self, working_digits: u32) -> Self;
    fn exp2_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Power ──────────────────────────────────────────────────

    fn powf_strict(self, exp: Self) -> Self;
    fn powf_strict_with(self, exp: Self, mode: RoundingMode) -> Self;
    fn powf_approx(self, exp: Self, working_digits: u32) -> Self;
    fn powf_approx_with(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Roots ──────────────────────────────────────────────────
    //
    // No `_approx` siblings — the exact-integer-root path has no
    // guard-width parameter to expose.

    fn sqrt_strict(self) -> Self;
    fn sqrt_strict_with(self, mode: RoundingMode) -> Self;

    fn cbrt_strict(self) -> Self;
    fn cbrt_strict_with(self, mode: RoundingMode) -> Self;

    fn hypot_strict(self, other: Self) -> Self;
    fn hypot_strict_with(self, other: Self, mode: RoundingMode) -> Self;

    // ── Trig (forward) ─────────────────────────────────────────

    fn sin_strict(self) -> Self;
    fn sin_strict_with(self, mode: RoundingMode) -> Self;
    fn sin_approx(self, working_digits: u32) -> Self;
    fn sin_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn cos_strict(self) -> Self;
    fn cos_strict_with(self, mode: RoundingMode) -> Self;
    fn cos_approx(self, working_digits: u32) -> Self;
    fn cos_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn tan_strict(self) -> Self;
    fn tan_strict_with(self, mode: RoundingMode) -> Self;
    fn tan_approx(self, working_digits: u32) -> Self;
    fn tan_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Trig (inverse) ─────────────────────────────────────────

    fn atan_strict(self) -> Self;
    fn atan_strict_with(self, mode: RoundingMode) -> Self;
    fn atan_approx(self, working_digits: u32) -> Self;
    fn atan_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn asin_strict(self) -> Self;
    fn asin_strict_with(self, mode: RoundingMode) -> Self;
    fn asin_approx(self, working_digits: u32) -> Self;
    fn asin_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn acos_strict(self) -> Self;
    fn acos_strict_with(self, mode: RoundingMode) -> Self;
    fn acos_approx(self, working_digits: u32) -> Self;
    fn acos_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    /// `atan2(self, other)` — matches the f64 convention where
    /// `self` is `y` and `other` is `x`.
    fn atan2_strict(self, other: Self) -> Self;
    fn atan2_strict_with(self, other: Self, mode: RoundingMode) -> Self;
    fn atan2_approx(self, other: Self, working_digits: u32) -> Self;
    fn atan2_approx_with(self, other: Self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Hyperbolic ─────────────────────────────────────────────

    fn sinh_strict(self) -> Self;
    fn sinh_strict_with(self, mode: RoundingMode) -> Self;
    fn sinh_approx(self, working_digits: u32) -> Self;
    fn sinh_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn cosh_strict(self) -> Self;
    fn cosh_strict_with(self, mode: RoundingMode) -> Self;
    fn cosh_approx(self, working_digits: u32) -> Self;
    fn cosh_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn tanh_strict(self) -> Self;
    fn tanh_strict_with(self, mode: RoundingMode) -> Self;
    fn tanh_approx(self, working_digits: u32) -> Self;
    fn tanh_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn asinh_strict(self) -> Self;
    fn asinh_strict_with(self, mode: RoundingMode) -> Self;
    fn asinh_approx(self, working_digits: u32) -> Self;
    fn asinh_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn acosh_strict(self) -> Self;
    fn acosh_strict_with(self, mode: RoundingMode) -> Self;
    fn acosh_approx(self, working_digits: u32) -> Self;
    fn acosh_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn atanh_strict(self) -> Self;
    fn atanh_strict_with(self, mode: RoundingMode) -> Self;
    fn atanh_approx(self, working_digits: u32) -> Self;
    fn atanh_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Angle conversion ───────────────────────────────────────

    fn to_degrees_strict(self) -> Self;
    fn to_degrees_strict_with(self, mode: RoundingMode) -> Self;
    fn to_degrees_approx(self, working_digits: u32) -> Self;
    fn to_degrees_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;

    fn to_radians_strict(self) -> Self;
    fn to_radians_strict_with(self, mode: RoundingMode) -> Self;
    fn to_radians_approx(self, working_digits: u32) -> Self;
    fn to_radians_approx_with(self, working_digits: u32, mode: RoundingMode) -> Self;
}
