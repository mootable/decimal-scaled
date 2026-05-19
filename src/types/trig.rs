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
//! `policy::trig::TrigPolicy`. The correctly-rounded kernels
//! (`sin_fixed`, `atan_fixed`, `atan2_kernel`, `to_fixed`, `wide_pi`,
//! `wide_half_pi`, `small_x_linear_threshold`, and every per-method
//! `*_strict` / `*_with` `Fixed`-shape function for sin / cos / tan /
//! atan / asin / acos / atan2 / sinh / cosh / tanh / asinh / acosh /
//! atanh / to_degrees / to_radians) live in
//! [`crate::algos::trig::fixed_d38`]. This file is a typed-shell
//! surface; there are zero `crate::algos::*` or
//! `crate::algos::fixed_d38::*` references in it.
//!
//! [`RoundingMode::HalfToEven`]: crate::RoundingMode::HalfToEven
//!
//! # `atan2` signature
//!
//! `f64::atan2(self, other)` treats `self` as `y` and `other` as `x`.
//! This module matches that signature exactly so generic numeric code
//! calling `y.atan2(x)` works with `T = D38`.

use crate::types::widths::D38;
use crate::types::log_exp::STRICT_GUARD;

impl<const SCALE: u32> D38<SCALE> {
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
        <Self as crate::policy::trig::TrigPolicy>::sin_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn sin_approx(self, working_digits: u32) -> Self {
        self.sin_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn sin_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.sin_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::sin_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::cos_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn cos_approx(self, working_digits: u32) -> Self {
        self.cos_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn cos_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.cos_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::cos_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::tan_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn tan_approx(self, working_digits: u32) -> Self {
        self.tan_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn tan_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.tan_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::tan_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::atan_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn atan_approx(self, working_digits: u32) -> Self {
        self.atan_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atan_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.atan_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::atan_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::asin_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn asin_approx(self, working_digits: u32) -> Self {
        self.asin_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn asin_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.asin_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::asin_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::acos_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn acos_approx(self, working_digits: u32) -> Self {
        self.acos_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn acos_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.acos_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::acos_with_impl(self, working_digits, mode)
    }

    /// Four-quadrant arctangent of `self` (`y`) and `other` (`x`).
    #[inline]
    #[must_use]
    pub fn atan2_strict(self, other: Self) -> Self {
        self.atan2_strict_with(other, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atan2_strict_with(self, other: Self, mode: crate::support::rounding::RoundingMode) -> Self {
        <Self as crate::policy::trig::TrigPolicy>::atan2_impl(self, other, mode)
    }

    #[inline]
    #[must_use]
    pub fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
        self.atan2_approx_with(other, working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atan2_approx_with(self, other: Self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.atan2_strict_with(other, mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::atan2_with_impl(self, other, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::sinh_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn sinh_approx(self, working_digits: u32) -> Self {
        self.sinh_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn sinh_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.sinh_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::sinh_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::cosh_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn cosh_approx(self, working_digits: u32) -> Self {
        self.cosh_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn cosh_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.cosh_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::cosh_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::tanh_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn tanh_approx(self, working_digits: u32) -> Self {
        self.tanh_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn tanh_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.tanh_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::tanh_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::asinh_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn asinh_approx(self, working_digits: u32) -> Self {
        self.asinh_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn asinh_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.asinh_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::asinh_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::acosh_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn acosh_approx(self, working_digits: u32) -> Self {
        self.acosh_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn acosh_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.acosh_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::acosh_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::atanh_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn atanh_approx(self, working_digits: u32) -> Self {
        self.atanh_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn atanh_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.atanh_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::atanh_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::to_degrees_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn to_degrees_approx(self, working_digits: u32) -> Self {
        self.to_degrees_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn to_degrees_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.to_degrees_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::to_degrees_with_impl(self, working_digits, mode)
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
        <Self as crate::policy::trig::TrigPolicy>::to_radians_impl(self, mode)
    }

    #[inline]
    #[must_use]
    pub fn to_radians_approx(self, working_digits: u32) -> Self {
        self.to_radians_approx_with(working_digits, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    #[inline]
    #[must_use]
    pub fn to_radians_approx_with(self, working_digits: u32, mode: crate::support::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.to_radians_strict_with(mode);
        }
        <Self as crate::policy::trig::TrigPolicy>::to_radians_with_impl(self, working_digits, mode)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::consts::DecimalConstants;
    use crate::types::widths::D38s12;

    // Tolerance for single-operation results. The f64-bridge build is
    // one f64 round-trip (≤ 2 LSB); the integer-only `strict` build is
    // correctly rounded (≤ 0.5 ULP per call) and is held to the same
    // 2-LSB bound — a couple of LSB for the test's own expected-value
    // rounding.
    const TWO_LSB: i128 = 2;

    // Tolerance for results that chain multiple trig calls.
    const FOUR_LSB: i128 = 4;

    // Angle conversions amplify the f64 reference's pi quantization;
    // 32 LSB at SCALE = 12.
    const ANGLE_TOLERANCE_LSB: i128 = 32;

    fn within_lsb(actual: D38s12, expected: D38s12, lsb: i128) -> bool {
        let diff = (actual.to_bits() - expected.to_bits()).abs();
        diff <= lsb
    }

    // ── Forward trig ──────────────────────────────────────────────────

    /// The strict trig / hyperbolic family is correctly rounded:
    /// cross-check every method against the f64 bridge at D38<9>.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[test]
    fn strict_trig_family_matches_f64() {
        use crate::types::widths::D38;
        macro_rules! check {
            ($name:literal, $raw:expr, $strict:expr, $f64expr:expr) => {{
                let strict: i128 = $strict;
                let v = $raw as f64 / 1e9;
                let reference = ($f64expr(v) * 1e9).round() as i128;
                assert!(
                    (strict - reference).abs() <= 2,
                    concat!($name, "({}) = {}, f64 reference {}"),
                    $raw,
                    strict,
                    reference
                );
            }};
        }
        for &raw in &[
            -7_000_000_000_i128, -1_000_000_000, -100_000_000, 1,
            500_000_000, 1_000_000_000, 1_570_796_327, 3_000_000_000,
            6_283_185_307, 12_000_000_000,
        ] {
            let x = D38::<9>::from_bits(raw);
            check!("sin", raw, x.sin_strict().to_bits(), f64::sin);
            check!("cos", raw, x.cos_strict().to_bits(), f64::cos);
            check!("atan", raw, x.atan_strict().to_bits(), f64::atan);
            check!("sinh", raw, x.sinh_strict().to_bits(), f64::sinh);
            check!("cosh", raw, x.cosh_strict().to_bits(), f64::cosh);
            check!("tanh", raw, x.tanh_strict().to_bits(), f64::tanh);
            check!("asinh", raw, x.asinh_strict().to_bits(), f64::asinh);
        }
        for &raw in &[
            -1_000_000_000_i128, -700_000_000, -100_000_000, 0,
            250_000_000, 500_000_000, 999_999_999,
        ] {
            let x = D38::<9>::from_bits(raw);
            check!("asin", raw, x.asin_strict().to_bits(), f64::asin);
            check!("acos", raw, x.acos_strict().to_bits(), f64::acos);
        }
        for &raw in &[-900_000_000_i128, -300_000_000, 1, 300_000_000, 900_000_000] {
            let x = D38::<9>::from_bits(raw);
            check!("atanh", raw, x.atanh_strict().to_bits(), f64::atanh);
        }
        for &raw in &[1_000_000_000_i128, 1_500_000_000, 3_000_000_000, 50_000_000_000] {
            let x = D38::<9>::from_bits(raw);
            check!("acosh", raw, x.acosh_strict().to_bits(), f64::acosh);
        }
        for &raw in &[-1_000_000_000_i128, 1, 500_000_000, 1_000_000_000, 1_400_000_000] {
            let x = D38::<9>::from_bits(raw);
            check!("tan", raw, x.tan_strict().to_bits(), f64::tan);
        }
    }

    /// `sin(0) == 0`.
    #[test]
    fn sin_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sin(), D38s12::ZERO);
    }

    /// Regression: D38 strict trig at high SCALE drives the working
    /// scale `w = SCALE + STRICT_GUARD` past the old hard-coded
    /// 63-digit π constant.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[test]
    fn sin_one_correct_past_63_digit_pi_window() {
        use crate::types::widths::D38;
        let expected_35: i128 = 84_147_098_480_789_650_665_250_232_163_029_900;
        let expected_37: i128 =
            8_414_709_848_078_965_066_525_023_216_302_989_996;

        let got_35 = D38::<35>::ONE.sin_strict().to_bits();
        assert!(
            (got_35 - expected_35).abs() <= 1,
            "sin(1) @ D38<35>: got {got_35}, expected {expected_35}"
        );

        let got_37 = D38::<37>::ONE.sin_strict().to_bits();
        assert!(
            (got_37 - expected_37).abs() <= 1,
            "sin(1) @ D38<37>: got {got_37}, expected {expected_37}"
        );
    }

    /// `cos(0) == 1`.
    #[test]
    fn cos_zero_is_one() {
        assert_eq!(D38s12::ZERO.cos(), D38s12::ONE);
    }

    /// `tan(0) == 0`.
    #[test]
    fn tan_zero_is_zero() {
        assert_eq!(D38s12::ZERO.tan(), D38s12::ZERO);
    }

    /// Pythagorean identity.
    #[test]
    fn sin_squared_plus_cos_squared_is_one() {
        for raw in [
            1_234_567_890_123_i128,
            -2_345_678_901_234_i128,
            500_000_000_000_i128,
            -500_000_000_000_i128,
            4_567_891_234_567_i128,
        ] {
            let x = D38s12::from_bits(raw);
            let s = x.sin();
            let c = x.cos();
            let sum = (s * s) + (c * c);
            assert!(
                within_lsb(sum, D38s12::ONE, FOUR_LSB),
                "sin^2 + cos^2 != 1 for raw={raw}: got bits {} (delta {})",
                sum.to_bits(),
                (sum.to_bits() - D38s12::ONE.to_bits()).abs(),
            );
        }
    }

    // ── Inverse trig ──────────────────────────────────────────────────

    #[test]
    fn asin_zero_is_zero() {
        assert_eq!(D38s12::ZERO.asin(), D38s12::ZERO);
    }

    #[test]
    fn acos_one_is_zero() {
        assert_eq!(D38s12::ONE.acos(), D38s12::ZERO);
    }

    #[test]
    fn acos_zero_is_half_pi() {
        let result = D38s12::ZERO.acos();
        assert!(
            within_lsb(result, D38s12::half_pi(), FOUR_LSB),
            "acos(0) bits {}, half_pi bits {}",
            result.to_bits(),
            D38s12::half_pi().to_bits(),
        );
    }

    #[test]
    fn atan_zero_is_zero() {
        assert_eq!(D38s12::ZERO.atan(), D38s12::ZERO);
    }

    #[test]
    fn asin_of_sin_round_trip() {
        for raw in [
            123_456_789_012_i128,
            -123_456_789_012_i128,
            456_789_012_345_i128,
            -456_789_012_345_i128,
            1_234_567_890_123_i128,
            -1_234_567_890_123_i128,
        ] {
            let x = D38s12::from_bits(raw);
            let recovered = x.sin().asin();
            assert!(
                within_lsb(recovered, x, FOUR_LSB),
                "asin(sin(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits(),
                (recovered.to_bits() - x.to_bits()).abs(),
            );
        }
    }

    // ── atan2 ─────────────────────────────────────────────────────────

    #[test]
    fn atan2_first_quadrant_diagonal() {
        let one = D38s12::ONE;
        let result = one.atan2(one);
        assert!(
            within_lsb(result, D38s12::quarter_pi(), TWO_LSB),
            "atan2(1, 1) bits {}, quarter_pi bits {}",
            result.to_bits(),
            D38s12::quarter_pi().to_bits(),
        );
    }

    #[test]
    fn atan2_third_quadrant_diagonal() {
        let neg_one = -D38s12::ONE;
        let result = neg_one.atan2(neg_one);
        let three = D38s12::from_int(3);
        let expected = -(D38s12::quarter_pi() * three);
        assert!(
            within_lsb(result, expected, TWO_LSB),
            "atan2(-1, -1) bits {}, expected -3pi/4 bits {}",
            result.to_bits(),
            expected.to_bits(),
        );
    }

    #[test]
    fn atan2_second_quadrant_diagonal() {
        let one = D38s12::ONE;
        let neg_one = -D38s12::ONE;
        let result = one.atan2(neg_one);
        let three = D38s12::from_int(3);
        let expected = D38s12::quarter_pi() * three;
        assert!(
            within_lsb(result, expected, TWO_LSB),
            "atan2(1, -1) bits {}, expected 3pi/4 bits {}",
            result.to_bits(),
            expected.to_bits(),
        );
    }

    #[test]
    fn atan2_fourth_quadrant_diagonal() {
        let one = D38s12::ONE;
        let neg_one = -D38s12::ONE;
        let result = neg_one.atan2(one);
        let expected = -D38s12::quarter_pi();
        assert!(
            within_lsb(result, expected, TWO_LSB),
            "atan2(-1, 1) bits {}, expected -pi/4 bits {}",
            result.to_bits(),
            expected.to_bits(),
        );
    }

    #[test]
    fn atan2_positive_x_axis_is_zero() {
        let zero = D38s12::ZERO;
        let one = D38s12::ONE;
        assert_eq!(zero.atan2(one), D38s12::ZERO);
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    #[test]
    fn sinh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sinh(), D38s12::ZERO);
    }

    #[test]
    fn cosh_zero_is_one() {
        assert_eq!(D38s12::ZERO.cosh(), D38s12::ONE);
    }

    #[test]
    fn tanh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.tanh(), D38s12::ZERO);
    }

    #[test]
    fn asinh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.asinh(), D38s12::ZERO);
    }

    #[test]
    fn acosh_one_is_zero() {
        assert_eq!(D38s12::ONE.acosh(), D38s12::ZERO);
    }

    #[test]
    fn atanh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.atanh(), D38s12::ZERO);
    }

    #[test]
    fn cosh_squared_minus_sinh_squared_is_one() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_234_567_890_123_i128,
            -1_234_567_890_123_i128,
            2_500_000_000_000_i128,
        ] {
            let x = D38s12::from_bits(raw);
            let ch = x.cosh();
            let sh = x.sinh();
            let diff = (ch * ch) - (sh * sh);
            assert!(
                within_lsb(diff, D38s12::ONE, FOUR_LSB),
                "cosh^2 - sinh^2 != 1 for raw={raw}: got bits {} (delta {})",
                diff.to_bits(),
                (diff.to_bits() - D38s12::ONE.to_bits()).abs(),
            );
        }
    }

    // ── Angle conversions ─────────────────────────────────────────────

    #[test]
    fn to_degrees_pi_is_180() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        let pi = D38s12::pi();
        let result = pi.to_degrees();
        let expected = D38s12::from_int(180);
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_degrees(pi) bits {}, expected 180 bits {} (delta {})",
            result.to_bits(),
            expected.to_bits(),
            (result.to_bits() - expected.to_bits()).abs(),
        );
    }

    #[test]
    fn to_radians_180_is_pi() {
        let one_eighty = D38s12::from_int(180);
        let result = one_eighty.to_radians();
        let expected = D38s12::pi();
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_radians(180) bits {}, expected pi bits {} (delta {})",
            result.to_bits(),
            expected.to_bits(),
            (result.to_bits() - expected.to_bits()).abs(),
        );
    }

    #[test]
    fn to_degrees_zero_is_zero() {
        assert_eq!(D38s12::ZERO.to_degrees(), D38s12::ZERO);
    }

    #[test]
    fn to_radians_zero_is_zero() {
        assert_eq!(D38s12::ZERO.to_radians(), D38s12::ZERO);
    }

    #[test]
    fn to_radians_to_degrees_round_trip() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_234_567_890_123_i128,
            -2_345_678_901_234_i128,
        ] {
            let x = D38s12::from_bits(raw);
            let recovered = x.to_degrees().to_radians();
            assert!(
                within_lsb(recovered, x, FOUR_LSB),
                "to_radians(to_degrees(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits(),
                (recovered.to_bits() - x.to_bits()).abs(),
            );
        }
    }

    #[test]
    fn to_degrees_half_pi_is_90() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        let result = D38s12::half_pi().to_degrees();
        let expected = D38s12::from_int(90);
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_degrees(half_pi) bits {}, expected 90 bits {} (delta {})",
            result.to_bits(),
            expected.to_bits(),
            (result.to_bits() - expected.to_bits()).abs(),
        );
    }

    #[test]
    fn to_degrees_quarter_pi_is_45() {
        let result = D38s12::quarter_pi().to_degrees();
        let expected = D38s12::from_int(45);
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_degrees(quarter_pi) bits {}, expected 45 bits {} (delta {})",
            result.to_bits(),
            expected.to_bits(),
            (result.to_bits() - expected.to_bits()).abs(),
        );
    }

    // ── Cross-method consistency ──────────────────────────────────────

    #[test]
    fn tan_matches_sin_over_cos() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_000_000_000_000_i128,
            -1_000_000_000_000_i128,
            123_456_789_012_i128,
        ] {
            let x = D38s12::from_bits(raw);
            let t = x.tan();
            let sc = x.sin() / x.cos();
            assert!(
                within_lsb(t, sc, FOUR_LSB),
                "tan(x) != sin/cos for raw={raw}: tan bits {}, sin/cos bits {}",
                t.to_bits(),
                sc.to_bits(),
            );
        }
    }

    #[test]
    fn tanh_matches_sinh_over_cosh() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_234_567_890_123_i128,
            -2_345_678_901_234_i128,
        ] {
            let x = D38s12::from_bits(raw);
            let t = x.tanh();
            let sc = x.sinh() / x.cosh();
            assert!(
                within_lsb(t, sc, FOUR_LSB),
                "tanh(x) != sinh/cosh for raw={raw}: tanh bits {}, sinh/cosh bits {}",
                t.to_bits(),
                sc.to_bits(),
            );
        }
    }
}
