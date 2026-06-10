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
//! Each function ships with four entry points so a single name covers
//! every (precision × rounding) combination the surface needs:
//!
//! | Method            | Guard width    | Rounding mode               |
//! |-------------------|----------------|------------------------------|
//! | `<fn>_strict`     | crate default  | crate default ([`RoundingMode::HalfToEven`] unless a `rounding-*` feature is set) |
//! | `<fn>_strict_with`| crate default  | caller-supplied              |
//! | `<fn>_approx`     | caller-chosen  | crate default               |
//! | `<fn>_approx_with`| caller-chosen  | caller-supplied              |
//!
//! All four variants are integer-only, `no_std`-compatible, and
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

use crate::types::log_exp::STRICT_GUARD;

impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    // ── Plain dispatchers (strict-feature) ────────────────────────

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        self.sin_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        self.cos_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        self.tan_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        self.asin_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        self.acos_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        self.atan_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        self.atan2_strict(other)
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn sinh(self) -> Self {
        self.sinh_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn cosh(self) -> Self {
        self.cosh_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn tanh(self) -> Self {
        self.tanh_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        self.asinh_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        self.acosh_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn atanh(self) -> Self {
        self.atanh_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> Self {
        self.to_degrees_strict()
    }

    #[cfg(all(feature = "strict", not(feature = "fast")))]
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

    #[inline]
    #[must_use]
    pub fn sin_approx(self, working_digits: u32) -> Self {
        self.sin_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn sin_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.sin_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::sin_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn cos_approx(self, working_digits: u32) -> Self {
        self.cos_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn cos_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.cos_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::cos_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn tan_approx(self, working_digits: u32) -> Self {
        self.tan_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn tan_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.tan_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::tan_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn atan_approx(self, working_digits: u32) -> Self {
        self.atan_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn atan_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.atan_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::atan_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn asin_approx(self, working_digits: u32) -> Self {
        self.asin_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn asin_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.asin_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::asin_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn acos_approx(self, working_digits: u32) -> Self {
        self.acos_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn acos_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.acos_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::acos_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
        self.atan2_approx_with(
            other,
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn atan2_approx_with(
        self,
        other: Self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.atan2_strict_with(other, mode);
        }
        Self::from_bits(crate::policy::trig::atan2_dispatch_with::<_, SCALE>(self.to_bits(), other.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn sinh_approx(self, working_digits: u32) -> Self {
        self.sinh_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn sinh_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.sinh_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::sinh_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn cosh_approx(self, working_digits: u32) -> Self {
        self.cosh_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn cosh_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.cosh_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::cosh_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn tanh_approx(self, working_digits: u32) -> Self {
        self.tanh_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn tanh_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.tanh_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::tanh_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn asinh_approx(self, working_digits: u32) -> Self {
        self.asinh_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn asinh_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.asinh_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::asinh_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn acosh_approx(self, working_digits: u32) -> Self {
        self.acosh_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn acosh_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.acosh_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::acosh_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn atanh_approx(self, working_digits: u32) -> Self {
        self.atanh_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn atanh_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.atanh_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::atanh_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn to_degrees_approx(self, working_digits: u32) -> Self {
        self.to_degrees_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_degrees_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.to_degrees_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::to_degrees_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
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

    #[inline]
    #[must_use]
    pub fn to_radians_approx(self, working_digits: u32) -> Self {
        self.to_radians_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    #[inline]
    #[must_use]
    pub fn to_radians_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.to_radians_strict_with(mode);
        }
        Self::from_bits(crate::policy::trig::to_radians_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
    }
}
