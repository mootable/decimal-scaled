// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trigonometric, hyperbolic, and angle-conversion methods for [`D38`].
//!
//! # Surface
//!
//! Fifteen mathematical functions:
//!
//! - **Forward trig (radians input):** [`D38::sin`] / [`D38::cos`] /
//! [`D38::tan`].
//! - **Inverse trig (returns radians):** [`D38::asin`] / [`D38::acos`]
//! / [`D38::atan`] / [`D38::atan2`].
//! - **Hyperbolic:** [`D38::sinh`] / [`D38::cosh`] / [`D38::tanh`] /
//! [`D38::asinh`] / [`D38::acosh`] / [`D38::atanh`].
//! - **Angle conversions:** [`D38::to_degrees`] / [`D38::to_radians`].
//!
//! # The four-variant matrix
//!
//! Each function ships with two entry points so a single name covers
//! either rounding-mode choice the surface needs:
//!
//! | Method            | Guard width    | Rounding mode               |
//! |-------------------|----------------|------------------------------|
//! | `<fn>_strict`     | crate default  | crate default ([`RoundingMode::HalfToEven`] unless a `rounding-*` feature is set) |
//! | `<fn>_strict_with`| crate default  | caller-supplied              |
//!
//! Both variants are integer-only, `no_std`-compatible, and
//! correctly rounded under the selected mode. Without the `strict`
//! feature, the plain `<fn>` is an f64-bridge instead.
//!
//! # Layering
//!
//! Every public method on this file is a one-line delegate into
//! `policy::trig`. The correctly-rounded kernels
//! (`sin_fixed`, `atan_fixed`, `atan2_kernel`, `to_fixed`, `wide_pi`,
//! `wide_half_pi`, `small_x_linear_threshold`, and every per-method
//! `*_strict` / `*_with` `Fixed`-shape function for sin / cos / tan /
//! atan / asin / acos / atan2 / sinh / cosh / tanh / asinh / acosh /
//! atanh / to_degrees / to_radians) live in
//! [`crate::algos::trig::trig_series_2limb`]. This file is a typed-shell
//! surface; there are zero `crate::algos::*` or
//! `crate::algos::support::fixed::*` references in it.
//!
//! [`RoundingMode::HalfToEven`]: crate::RoundingMode::HalfToEven
//!
//! # `atan2` signature
//!
//! `f64::atan2(self, other)` treats `self` as `y` and `other` as `x`.
//! This module matches that signature exactly so generic numeric code
//! calling `y.atan2(x)` works with `T = D38`.

impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    // ── Plain dispatchers (strict path) ───────────────────────────

    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        self.sin_strict()
    }

    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        self.cos_strict()
    }

    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        self.tan_strict()
    }

    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        self.asin_strict()
    }

    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        self.acos_strict()
    }

    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        self.atan_strict()
    }

    #[inline]
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        self.atan2_strict(other)
    }

    #[inline]
    #[must_use]
    pub fn sinh(self) -> Self {
        self.sinh_strict()
    }

    #[inline]
    #[must_use]
    pub fn cosh(self) -> Self {
        self.cosh_strict()
    }

    #[inline]
    #[must_use]
    pub fn tanh(self) -> Self {
        self.tanh_strict()
    }

    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        self.asinh_strict()
    }

    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        self.acosh_strict()
    }

    #[inline]
    #[must_use]
    pub fn atanh(self) -> Self {
        self.atanh_strict()
    }

    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> Self {
        self.to_degrees_strict()
    }

    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        self.to_radians_strict()
    }

    // ── Forward trig (one-line policy delegates) ──────────────────

    /// Sine of `self` (radians). Correctly rounded.
    #[inline]
    #[must_use]
    pub fn sin_strict(self) -> Self {
        self.sin_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn sin_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::sin_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Cosine of `self` (radians). `cos(x) = sin(x + π/2)`.
    #[inline]
    #[must_use]
    pub fn cos_strict(self) -> Self {
        self.cos_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn cos_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::cos_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Tangent. Panics if `cos(self)` is zero.
    #[inline]
    #[must_use]
    pub fn tan_strict(self) -> Self {
        self.tan_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn tan_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::tan_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Arctangent.
    #[inline]
    #[must_use]
    pub fn atan_strict(self) -> Self {
        self.atan_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atan_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::atan_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Arcsine. Panics if `|self| > 1`.
    #[inline]
    #[must_use]
    pub fn asin_strict(self) -> Self {
        self.asin_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn asin_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::asin_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Arccosine. Panics if `|self| > 1`.
    #[inline]
    #[must_use]
    pub fn acos_strict(self) -> Self {
        self.acos_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn acos_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::acos_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Four-quadrant arctangent of `self` (`y`) and `other` (`x`).
    #[inline]
    #[must_use]
    pub fn atan2_strict(self, other: Self) -> Self {
        self.atan2_strict_with(other, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atan2_strict_with(
        self,
        other: Self,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        Self::from_bits(crate::policy::trig::atan2_dispatch::<_, SCALE>(self.to_bits(), other.to_bits(), mode))
    }

    // ── Hyperbolic family (one-line policy delegates) ─────────────

    /// Hyperbolic sine. Correctly rounded.
    #[inline]
    #[must_use]
    pub fn sinh_strict(self) -> Self {
        self.sinh_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn sinh_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::sinh_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Hyperbolic cosine.
    #[inline]
    #[must_use]
    pub fn cosh_strict(self) -> Self {
        self.cosh_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn cosh_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::cosh_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Hyperbolic tangent.
    #[inline]
    #[must_use]
    pub fn tanh_strict(self) -> Self {
        self.tanh_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn tanh_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::tanh_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Inverse hyperbolic sine. `asinh(x) = sign · ln(|x| + √(x²+1))`.
    #[inline]
    #[must_use]
    pub fn asinh_strict(self) -> Self {
        self.asinh_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn asinh_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::asinh_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Inverse hyperbolic cosine. Panics if `self < 1`.
    #[inline]
    #[must_use]
    pub fn acosh_strict(self) -> Self {
        self.acosh_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn acosh_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::acosh_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Inverse hyperbolic tangent. Panics if `|self| >= 1`.
    #[inline]
    #[must_use]
    pub fn atanh_strict(self) -> Self {
        self.atanh_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atanh_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::atanh_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    // ── Angle conversions (one-line policy delegates) ─────────────

    /// Convert radians to degrees: `self · (180 / π)`.
    #[inline]
    #[must_use]
    pub fn to_degrees_strict(self) -> Self {
        self.to_degrees_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn to_degrees_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::to_degrees_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Convert degrees to radians: `self · (π / 180)`.
    #[inline]
    #[must_use]
    pub fn to_radians_strict(self) -> Self {
        self.to_radians_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn to_radians_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::trig::to_radians_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

}
