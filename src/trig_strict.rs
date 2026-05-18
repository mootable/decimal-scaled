//! Trigonometric, hyperbolic, and angle-conversion methods for [`D38`].
//!
//! # Methods
//!
//! Fifteen methods:
//!
//! - **Forward trig (radians input):** [`D38::sin`] / [`D38::cos`] /
//! [`D38::tan`].
//! - **Inverse trig (returns radians):** [`D38::asin`] / [`D38::acos`]
//! / [`D38::atan`] / [`D38::atan2`].
//! - **Hyperbolic:** [`D38::sinh`] / [`D38::cosh`] / [`D38::tanh`] /
//! [`D38::asinh`] / [`D38::acosh`] / [`D38::atanh`].
//! - **Angle conversions:** [`D38::to_degrees`] / [`D38::to_radians`].
//!
//! # The `*_strict` dual API
//!
//! Each method has two implementations:
//!
//! - An integer-only `<method>_strict` form — always compiled (unless
//! the `fast` feature is set), `no_std`-compatible, and
//! platform-deterministic. `sin`/`cos`/`tan` range-reduce and
//! evaluate a Taylor series; `atan`/`asin`/`acos`/`atan2` derive from
//! a reciprocal-reduced Taylor `atan`; the hyperbolic family composes
//! the strict `exp` / `ln` / `sqrt`.
//! - An f64-bridge form — converts to `f64`, calls the platform
//! intrinsic, converts back. Gated on `std`.
//!
//! The plain `<method>` is a dispatcher: with the `strict` feature it
//! calls `<method>_strict`; otherwise it is the f64 bridge. See
//! `docs/strict-mode.md` for the full dual-API and feature-gating
//! rules and the 0.5 ULP accuracy contract.
//!
//! # Precision
//!
//! The f64-bridge forms are **Lossy** — the `D38` value round-trips
//! through `f64`, which introduces up to one LSB of quantisation per
//! conversion. The `*_strict` forms are **correctly rounded**: within
//! 0.5 ULP of the exact result (IEEE-754 round-to-nearest). They
//! evaluate every reduction and series step in the `d_w128_kernels::Fixed`
//! guard-digit intermediate and round once at the end.
//!
//! # `atan2` signature
//!
//! `f64::atan2(self, other)` treats `self` as `y` and `other` as `x`.
//! This module matches that signature exactly so generic numeric code
//! calling `y.atan2(x)` works with `T = D38`.

use crate::core_type::D38;

impl<const SCALE: u32> D38<SCALE> {
#[cfg(all(feature = "strict", not(feature = "fast")))]
    /// With `strict` this dispatches to [`Self::sin_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        self.sin_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::cos_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        self.cos_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::tan_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        self.tan_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::asin_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        self.asin_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::acos_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        self.acos_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::atan_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        self.atan_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// Four-quadrant arctangent of `self` (`y`) and `other` (`x`).
    /// With `strict` this dispatches to [`Self::atan2_strict`];
    /// without it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        self.atan2_strict(other)
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::sinh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn sinh(self) -> Self {
        self.sinh_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::cosh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn cosh(self) -> Self {
        self.cosh_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::tanh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn tanh(self) -> Self {
        self.tanh_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::asinh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        self.asinh_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::acosh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        self.acosh_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::atanh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn atanh(self) -> Self {
        self.atanh_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::to_degrees_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> Self {
        self.to_degrees_strict()
    }
#[cfg(all(feature = "strict", not(feature = "fast")))]

    /// With `strict` this dispatches to [`Self::to_radians_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        self.to_radians_strict()
    }

    // Sine of `self` (radians). Strict: integer-only and correctly
    // rounded — the result is within 0.5 ULP of the exact sine.
    #[inline]
    #[must_use]
    pub fn sin_strict(self) -> Self {
        self.sin_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Sine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn sin_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let raw = sin_fixed(to_fixed(self.0), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::sin: result out of range");
        Self::from_bits(raw)
    }

    /// Sine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn sin_approx(self, working_digits: u32) -> Self {
        self.sin_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Sine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn sin_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.sin_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + working_digits;
        let raw = sin_fixed(to_fixed(self.0), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::sin: result out of range");
        Self::from_bits(raw)
    }

    /// Cosine of `self` (radians). Strict: `cos(x) = sin(x + π/2)`,
    /// correctly rounded.
    #[inline]
    #[must_use]
    pub fn cos_strict(self) -> Self {
        self.cos_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Cosine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn cos_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::from_bits(10_i128.pow(SCALE));
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let arg = to_fixed(self.0).add(wide_half_pi(w));
        let raw = sin_fixed(arg, w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::cos: result out of range");
        Self::from_bits(raw)
    }

    /// Cosine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn cos_approx(self, working_digits: u32) -> Self {
        self.cos_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Cosine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn cos_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.cos_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::from_bits(10_i128.pow(SCALE));
        }
        let w = SCALE + working_digits;
        let arg = to_fixed(self.0).add(wide_half_pi(w));
        let raw = sin_fixed(arg, w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::cos: result out of range");
        Self::from_bits(raw)
    }

    /// Tangent of `self` (radians). Strict: `tan(x) = sin(x) / cos(x)`,
    /// with the division carried in the wide intermediate so the result
    /// is correctly rounded.
    ///
    /// # Panics
    ///
    /// Panics if `cos(self)` is zero (an odd multiple of π/2).
    #[inline]
    #[must_use]
    pub fn tan_strict(self) -> Self {
        self.tan_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Tangent under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn tan_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let v = to_fixed(self.0);
        let sin_w = sin_fixed(v, w);
        let cos_w = sin_fixed(v.add(wide_half_pi(w)), w);
        assert!(!cos_w.is_zero(), "D38::tan: cosine is zero (argument is an odd multiple of pi/2)");
        let raw = sin_w
            .div(cos_w, w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::tan: result out of range");
        Self::from_bits(raw)
    }

    /// Tangent with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn tan_approx(self, working_digits: u32) -> Self {
        self.tan_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Tangent with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn tan_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.tan_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + working_digits;
        let v = to_fixed(self.0);
        let sin_w = sin_fixed(v, w);
        let cos_w = sin_fixed(v.add(wide_half_pi(w)), w);
        assert!(!cos_w.is_zero(), "D38::tan: cosine is zero (argument is an odd multiple of pi/2)");
        let raw = sin_w
            .div(cos_w, w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::tan: result out of range");
        Self::from_bits(raw)
    }

    /// Arctangent of `self`, in radians, in `(−π/2, π/2)`. Strict:
    /// integer-only and correctly rounded.
    #[inline]
    #[must_use]
    pub fn atan_strict(self) -> Self {
        self.atan_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Arctangent under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn atan_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        use crate::consts::DecimalConsts;
        if self.0 == 0 {
            return Self::ZERO;
        }
        let one_bits: i128 = 10_i128.pow(SCALE);
        if self.0 == one_bits {
            return Self::quarter_pi();
        }
        if self.0 == -one_bits {
            return -Self::quarter_pi();
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let raw = atan_fixed(to_fixed(self.0), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::atan: result out of range");
        Self::from_bits(raw)
    }

    /// Arctangent with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn atan_approx(self, working_digits: u32) -> Self {
        self.atan_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Arctangent with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn atan_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.atan_strict_with(mode);
        }
        use crate::consts::DecimalConsts;
        if self.0 == 0 {
            return Self::ZERO;
        }
        let one_bits: i128 = 10_i128.pow(SCALE);
        if self.0 == one_bits {
            return Self::quarter_pi();
        }
        if self.0 == -one_bits {
            return -Self::quarter_pi();
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + working_digits;
        let raw = atan_fixed(to_fixed(self.0), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::atan: result out of range");
        Self::from_bits(raw)
    }

    /// Arcsine of `self`, in radians, in `[−π/2, π/2]`. Strict.
    ///
    /// `asin(x) = atan(x / √(1 − x²))`; the endpoints `±1` map directly
    /// to `±π/2`.
    ///
    /// # Panics
    ///
    /// Panics if `|self| > 1`.
    #[inline]
    #[must_use]
    pub fn asin_strict(self) -> Self {
        self.asin_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Arcsine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn asin_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed(self.0);
        let abs_v = Fixed { negative: false, mag: v.mag };
        assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "D38::asin: argument out of domain [-1, 1]");
        if abs_v == one_w {
            let hp = wide_half_pi(w);
            let hp = if v.negative { hp.neg() } else { hp };
            let raw = hp
                .round_to_i128_with(w, SCALE, mode)
                .expect("D38::asin: result out of range");
            return Self::from_bits(raw);
        }
        let half_w = one_w.halve();
        let asin_w = if !abs_v.ge_mag(half_w) {
            let denom = one_w.sub(v.mul(v, w)).sqrt(w);
            atan_fixed(v.div(denom, w), w)
        } else {
            let inner = one_w.sub(abs_v).halve();
            let inner_sqrt = inner.sqrt(w);
            let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
            let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
            let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
            if v.negative { result_abs.neg() } else { result_abs }
        };
        let raw = asin_w
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::asin: result out of range");
        Self::from_bits(raw)
    }

    /// Arcsine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn asin_approx(self, working_digits: u32) -> Self {
        self.asin_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Arcsine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn asin_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.asin_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + working_digits;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed_w(self.0, working_digits);
        let abs_v = Fixed { negative: false, mag: v.mag };
        assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "D38::asin: argument out of domain [-1, 1]");
        if abs_v == one_w {
            let hp = wide_half_pi(w);
            let hp = if v.negative { hp.neg() } else { hp };
            let raw = hp
                .round_to_i128_with(w, SCALE, mode)
                .expect("D38::asin: result out of range");
            return Self::from_bits(raw);
        }
        let half_w = one_w.halve();
        let asin_w = if !abs_v.ge_mag(half_w) {
            let denom = one_w.sub(v.mul(v, w)).sqrt(w);
            atan_fixed(v.div(denom, w), w)
        } else {
            let inner = one_w.sub(abs_v).halve();
            let inner_sqrt = inner.sqrt(w);
            let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
            let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
            let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
            if v.negative { result_abs.neg() } else { result_abs }
        };
        let raw = asin_w
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::asin: result out of range");
        Self::from_bits(raw)
    }

    /// Arccosine of `self`, in radians, in `[0, π]`. Strict:
    /// `acos(x) = π/2 − asin(x)`, correctly rounded.
    ///
    /// # Panics
    ///
    /// Panics if `|self| > 1`.
    #[inline]
    #[must_use]
    pub fn acos_strict(self) -> Self {
        self.acos_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Arccosine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn acos_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        use crate::consts::DecimalConsts;
        if self.0 == 0 {
            return Self::half_pi();
        }
        let one_bits: i128 = 10_i128.pow(SCALE);
        if self.0 == one_bits {
            return Self::ZERO;
        }
        if self.0 == -one_bits {
            return Self::pi();
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed(self.0);
        let abs_v = Fixed { negative: false, mag: v.mag };
        assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "D38::acos: argument out of domain [-1, 1]");
        let half_w = one_w.halve();
        let asin_w = if abs_v == one_w {
            let hp = wide_half_pi(w);
            if v.negative { hp.neg() } else { hp }
        } else if !abs_v.ge_mag(half_w) {
            let denom = one_w.sub(v.mul(v, w)).sqrt(w);
            atan_fixed(v.div(denom, w), w)
        } else {
            let inner = one_w.sub(abs_v).halve();
            let inner_sqrt = inner.sqrt(w);
            let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
            let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
            let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
            if v.negative { result_abs.neg() } else { result_abs }
        };
        let raw = wide_half_pi(w)
            .sub(asin_w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::acos: result out of range");
        Self::from_bits(raw)
    }

    /// Arccosine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn acos_approx(self, working_digits: u32) -> Self {
        self.acos_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Arccosine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn acos_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.acos_strict_with(mode);
        }
        use crate::consts::DecimalConsts;
        if self.0 == 0 {
            return Self::half_pi();
        }
        let one_bits: i128 = 10_i128.pow(SCALE);
        if self.0 == one_bits {
            return Self::ZERO;
        }
        if self.0 == -one_bits {
            return Self::pi();
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + working_digits;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed_w(self.0, working_digits);
        let abs_v = Fixed { negative: false, mag: v.mag };
        assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "D38::acos: argument out of domain [-1, 1]");
        let half_w = one_w.halve();
        let asin_w = if abs_v == one_w {
            let hp = wide_half_pi(w);
            if v.negative { hp.neg() } else { hp }
        } else if !abs_v.ge_mag(half_w) {
            let denom = one_w.sub(v.mul(v, w)).sqrt(w);
            atan_fixed(v.div(denom, w), w)
        } else {
            let inner = one_w.sub(abs_v).halve();
            let inner_sqrt = inner.sqrt(w);
            let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
            let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
            let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
            if v.negative { result_abs.neg() } else { result_abs }
        };
        let raw = wide_half_pi(w)
            .sub(asin_w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::acos: result out of range");
        Self::from_bits(raw)
    }

    /// Four-quadrant arctangent of `self` (`y`) and `other` (`x`), in
    /// radians, in `(−π, π]`. Strict: integer-only and correctly
    /// rounded.
    #[inline]
    #[must_use]
    pub fn atan2_strict(self, other: Self) -> Self {
        self.atan2_strict_with(other, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Four-quadrant arctangent under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn atan2_strict_with(self, other: Self, mode: crate::rounding::RoundingMode) -> Self {
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let raw = atan2_kernel(to_fixed(self.0), to_fixed(other.0), self.0, w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::atan2: result out of range");
        Self::from_bits(raw)
    }

    /// Four-quadrant arctangent with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
        self.atan2_approx_with(other, working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Four-quadrant arctangent with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn atan2_approx_with(self, other: Self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.atan2_strict_with(other, mode);
        }
        let w = SCALE + working_digits;
        let raw = atan2_kernel(
            to_fixed_w(self.0, working_digits),
            to_fixed_w(other.0, working_digits),
            self.0,
            w,
        )
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::atan2: result out of range");
        Self::from_bits(raw)
    }

    /// Hyperbolic sine of `self`. Strict: `sinh(x) = (eˣ − e⁻ˣ)/2`,
    /// composed in the wide intermediate from the correctly-rounded
    /// `exp`, so the result is itself correctly rounded.
    #[inline]
    #[must_use]
    pub fn sinh_strict(self) -> Self {
        self.sinh_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hyperbolic sine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn sinh_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let v = to_fixed(self.0);
        let ex = crate::log_exp_strict::exp_fixed(v, w);
        let enx = crate::log_exp_strict::exp_fixed(v.neg(), w);
        let raw = ex
            .sub(enx)
            .halve()
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::sinh: result out of range");
        Self::from_bits(raw)
    }

    /// Hyperbolic sine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn sinh_approx(self, working_digits: u32) -> Self {
        self.sinh_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hyperbolic sine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn sinh_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.sinh_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + working_digits;
        let v = to_fixed_w(self.0, working_digits);
        let ex = crate::log_exp_strict::exp_fixed(v, w);
        let enx = crate::log_exp_strict::exp_fixed(v.neg(), w);
        let raw = ex
            .sub(enx)
            .halve()
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::sinh: result out of range");
        Self::from_bits(raw)
    }

    /// Hyperbolic cosine of `self`. Strict: `cosh(x) = (eˣ + e⁻ˣ)/2`,
    /// correctly rounded.
    #[inline]
    #[must_use]
    pub fn cosh_strict(self) -> Self {
        self.cosh_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hyperbolic cosine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn cosh_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::from_bits(10_i128.pow(SCALE));
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let v = to_fixed(self.0);
        let ex = crate::log_exp_strict::exp_fixed(v, w);
        let enx = crate::log_exp_strict::exp_fixed(v.neg(), w);
        let raw = ex
            .add(enx)
            .halve()
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::cosh: result out of range");
        Self::from_bits(raw)
    }

    /// Hyperbolic cosine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn cosh_approx(self, working_digits: u32) -> Self {
        self.cosh_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hyperbolic cosine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn cosh_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.cosh_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::from_bits(10_i128.pow(SCALE));
        }
        let w = SCALE + working_digits;
        let v = to_fixed_w(self.0, working_digits);
        let ex = crate::log_exp_strict::exp_fixed(v, w);
        let enx = crate::log_exp_strict::exp_fixed(v.neg(), w);
        let raw = ex
            .add(enx)
            .halve()
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::cosh: result out of range");
        Self::from_bits(raw)
    }

    /// Hyperbolic tangent of `self`. Strict: `tanh(x) = sinh(x)/cosh(x)`
    /// with the division in the wide intermediate. `cosh ≥ 1`, so the
    /// division never traps.
    #[inline]
    #[must_use]
    pub fn tanh_strict(self) -> Self {
        self.tanh_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hyperbolic tangent under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn tanh_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let v = to_fixed(self.0);
        let ex = crate::log_exp_strict::exp_fixed(v, w);
        let enx = crate::log_exp_strict::exp_fixed(v.neg(), w);
        let raw = ex
            .sub(enx)
            .div(ex.add(enx), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::tanh: result out of range");
        Self::from_bits(raw)
    }

    /// Hyperbolic tangent with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn tanh_approx(self, working_digits: u32) -> Self {
        self.tanh_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hyperbolic tangent with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn tanh_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.tanh_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + working_digits;
        let v = to_fixed_w(self.0, working_digits);
        let ex = crate::log_exp_strict::exp_fixed(v, w);
        let enx = crate::log_exp_strict::exp_fixed(v.neg(), w);
        let raw = ex
            .sub(enx)
            .div(ex.add(enx), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::tanh: result out of range");
        Self::from_bits(raw)
    }

    /// Inverse hyperbolic sine of `self`. Strict:
    /// `asinh(x) = sign · ln(|x| + √(x² + 1))`, correctly rounded.
    /// For `|x| ≥ 1` the radicand is factored as
    /// `|x|·(1 + √(1 + 1/x²))` to keep `x²` from overflowing the wide
    /// intermediate.
    #[inline]
    #[must_use]
    pub fn asinh_strict(self) -> Self {
        self.asinh_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Inverse hyperbolic sine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn asinh_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        use crate::d_w128_kernels::Fixed;
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed(self.0);
        let ax = Fixed { negative: false, mag: v.mag };
        let inner = if ax.ge_mag(one_w) {
            let inv = one_w.div(ax, w);
            let root = one_w.add(inv.mul(inv, w)).sqrt(w);
            crate::log_exp_strict::ln_fixed(ax, w).add(crate::log_exp_strict::ln_fixed(one_w.add(root), w))
        } else {
            let root = ax.mul(ax, w).add(one_w).sqrt(w);
            crate::log_exp_strict::ln_fixed(ax.add(root), w)
        };
        let signed = if self.0 < 0 { inner.neg() } else { inner };
        let raw = signed
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::asinh: result out of range");
        Self::from_bits(raw)
    }

    /// Inverse hyperbolic sine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn asinh_approx(self, working_digits: u32) -> Self {
        self.asinh_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Inverse hyperbolic sine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn asinh_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.asinh_strict_with(mode);
        }
        use crate::d_w128_kernels::Fixed;
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        let w = SCALE + working_digits;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed_w(self.0, working_digits);
        let ax = Fixed { negative: false, mag: v.mag };
        let inner = if ax.ge_mag(one_w) {
            let inv = one_w.div(ax, w);
            let root = one_w.add(inv.mul(inv, w)).sqrt(w);
            crate::log_exp_strict::ln_fixed(ax, w).add(crate::log_exp_strict::ln_fixed(one_w.add(root), w))
        } else {
            let root = ax.mul(ax, w).add(one_w).sqrt(w);
            crate::log_exp_strict::ln_fixed(ax.add(root), w)
        };
        let signed = if self.0 < 0 { inner.neg() } else { inner };
        let raw = signed
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::asinh: result out of range");
        Self::from_bits(raw)
    }

    /// Inverse hyperbolic cosine of `self`. Strict:
    /// `acosh(x) = ln(x + √(x² − 1))`, defined for `x ≥ 1`, correctly
    /// rounded. For `x ≥ 2` the radicand is factored as
    /// `x·(1 + √(1 − 1/x²))` to keep `x²` in range.
    ///
    /// # Panics
    ///
    /// Panics if `self < 1`.
    #[inline]
    #[must_use]
    pub fn acosh_strict(self) -> Self {
        self.acosh_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Inverse hyperbolic cosine under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn acosh_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        let one_bits: i128 = 10_i128.pow(SCALE);
        if self.0 == one_bits {
            return Self::ZERO;
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed(self.0);
        assert!(!v.negative && v.ge_mag(one_w), "D38::acosh: argument must be >= 1");
        let two_w = one_w.double();
        let inner = if v.ge_mag(two_w) {
            let inv = one_w.div(v, w);
            let root = one_w.sub(inv.mul(inv, w)).sqrt(w);
            crate::log_exp_strict::ln_fixed(v, w).add(crate::log_exp_strict::ln_fixed(one_w.add(root), w))
        } else {
            let root = v.mul(v, w).sub(one_w).sqrt(w);
            crate::log_exp_strict::ln_fixed(v.add(root), w)
        };
        let raw = inner
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::acosh: result out of range");
        Self::from_bits(raw)
    }

    /// Inverse hyperbolic cosine with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn acosh_approx(self, working_digits: u32) -> Self {
        self.acosh_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Inverse hyperbolic cosine with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn acosh_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.acosh_strict_with(mode);
        }
        let one_bits: i128 = 10_i128.pow(SCALE);
        if self.0 == one_bits {
            return Self::ZERO;
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + working_digits;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed_w(self.0, working_digits);
        assert!(!v.negative && v.ge_mag(one_w), "D38::acosh: argument must be >= 1");
        let two_w = one_w.double();
        let inner = if v.ge_mag(two_w) {
            let inv = one_w.div(v, w);
            let root = one_w.sub(inv.mul(inv, w)).sqrt(w);
            crate::log_exp_strict::ln_fixed(v, w).add(crate::log_exp_strict::ln_fixed(one_w.add(root), w))
        } else {
            let root = v.mul(v, w).sub(one_w).sqrt(w);
            crate::log_exp_strict::ln_fixed(v.add(root), w)
        };
        let raw = inner
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::acosh: result out of range");
        Self::from_bits(raw)
    }

    /// Inverse hyperbolic tangent of `self`. Strict:
    /// `atanh(x) = ln((1 + x) / (1 − x)) / 2`, defined for `|x| < 1`,
    /// correctly rounded.
    ///
    /// # Panics
    ///
    /// Panics if `|self| >= 1`.
    #[inline]
    #[must_use]
    pub fn atanh_strict(self) -> Self {
        self.atanh_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Inverse hyperbolic tangent under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn atanh_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed(self.0);
        let ax = Fixed { negative: false, mag: v.mag };
        assert!(!ax.ge_mag(one_w), "D38::atanh: argument out of domain (-1, 1)");
        let ratio = one_w.add(v).div(one_w.sub(v), w);
        let raw = crate::log_exp_strict::ln_fixed(ratio, w)
            .halve()
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::atanh: result out of range");
        Self::from_bits(raw)
    }

    /// Inverse hyperbolic tangent with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn atanh_approx(self, working_digits: u32) -> Self {
        self.atanh_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Inverse hyperbolic tangent with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn atanh_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.atanh_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        if self.0.abs() <= small_x_linear_threshold::<SCALE>() {
            return self;
        }
        use crate::d_w128_kernels::Fixed;
        let w = SCALE + working_digits;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        let v = to_fixed_w(self.0, working_digits);
        let ax = Fixed { negative: false, mag: v.mag };
        assert!(!ax.ge_mag(one_w), "D38::atanh: argument out of domain (-1, 1)");
        let ratio = one_w.add(v).div(one_w.sub(v), w);
        let raw = crate::log_exp_strict::ln_fixed(ratio, w)
            .halve()
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::atanh: result out of range");
        Self::from_bits(raw)
    }

    /// Convert radians to degrees: `self · (180 / π)`. Strict: the
    /// multiply and divide run in the wide intermediate, so the result
    /// is correctly rounded.
    #[inline]
    #[must_use]
    pub fn to_degrees_strict(self) -> Self {
        self.to_degrees_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Radians-to-degrees conversion under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn to_degrees_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let raw = to_fixed(self.0)
            .mul_u128(180)
            .div(wide_pi(w), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::to_degrees: result out of range");
        Self::from_bits(raw)
    }

    /// Radians-to-degrees conversion with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn to_degrees_approx(self, working_digits: u32) -> Self {
        self.to_degrees_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Radians-to-degrees with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn to_degrees_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.to_degrees_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        let w = SCALE + working_digits;
        let raw = to_fixed_w(self.0, working_digits)
            .mul_u128(180)
            .div(wide_pi(w), w)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::to_degrees: result out of range");
        Self::from_bits(raw)
    }

    /// Convert degrees to radians: `self · (π / 180)`. Strict:
    /// correctly rounded.
    #[inline]
    #[must_use]
    pub fn to_radians_strict(self) -> Self {
        self.to_radians_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Degrees-to-radians conversion under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn to_radians_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        let w = SCALE + crate::log_exp_strict::STRICT_GUARD;
        let raw = to_fixed(self.0)
            .mul(wide_pi(w), w)
            .div_small(180)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::to_radians: result out of range");
        Self::from_bits(raw)
    }

    /// Degrees-to-radians conversion with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn to_radians_approx(self, working_digits: u32) -> Self {
        self.to_radians_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Degrees-to-radians with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn to_radians_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == crate::log_exp_strict::STRICT_GUARD {
            return self.to_radians_strict_with(mode);
        }
        if self.0 == 0 {
            return Self::ZERO;
        }
        let w = SCALE + working_digits;
        let raw = to_fixed_w(self.0, working_digits)
            .mul(wide_pi(w), w)
            .div_small(180)
            .round_to_i128_with(w, SCALE, mode)
            .expect("D38::to_radians: result out of range");
        Self::from_bits(raw)
    }
}


// ─────────────────────────────────────────────────────────────────────
// Strict-mode (integer-only) trigonometric, hyperbolic, and angle-
// conversion methods.
//
// These mirror the f64-bridge surface above but are integer-only,
// `no_std`-compatible, and **correctly rounded** — within 0.5 ULP of
// the exact result. Every reduction and series step runs in the
// `d_w128_kernels::Fixed` guard-digit intermediate (the same machinery the
// log/exp family uses) and the value is rounded once at the end.
//
// Composition strategy:
//
// - Hyperbolic functions are composed from the strict `exp` / `ln` /
// `sqrt` already implemented in `log_exp_strict.rs` / `powers_strict.rs`.
// - `cos` is `sin` phase-shifted by π/2; `tan` is `sin / cos`.
// - `sin` uses range reduction modulo τ into one π/2 octant followed by
// a Taylor series.
// - `atan` uses reciprocal reduction for |x| > 1 plus argument halving,
// then a Taylor series; `asin` / `acos` / `atan2` are derived from it.
// ─────────────────────────────────────────────────────────────────────

// Strict-feature dispatchers. When `strict` is enabled (and
// `fast` is not), the plain trig methods route to the
// integer-only `*_strict` implementations below.

// ─────────────────────────────────────────────────────────────────────
// Correctly-rounded strict trigonometric core.
//
// Every strict trig method runs on the shared `d_w128_kernels::Fixed`
// guard-digit intermediate at `SCALE + STRICT_GUARD` working digits,
// the same machinery the log/exp family uses, and rounds once at the
// end — so each result is within 0.5 ULP of the exact value.
// ─────────────────────────────────────────────────────────────────────

/// π at working scale `w`, sourced from the crate-wide 75-digit
/// `consts::PI_RAW` (Int256 holding `π × 10^75`).
///
/// Caller-side precondition: `w <= 75`. The D38 strict trig kernel
/// runs at `w = SCALE + STRICT_GUARD`, capped at `38 + 30 = 68`, so
/// the bound is satisfied by every call site in this module. The
/// debug-assert documents the invariant for any future caller; in
/// release, `rescale_down(75, w > 75)` would silently produce a
/// wrong π via the wrapping `from_w − to_w` subtraction.
/// Threshold below which the linear small-x fast paths fire for the
/// odd trig functions (`atan`, `sin`, `tan`, `sinh`, `tanh`, `asin`,
/// `asinh`, `atanh`).
///
/// All these functions have a Taylor series `f(x) = x + c·x³ + …`
/// where `|c| ≤ 1/3`. For `|x| < (1.5·10⁻ˢᶜᴬᴸᴱ)^(1/3) ≈ 10^(−⌈SCALE/3⌉)`
/// the cubic correction is bounded by `0.5·ULP` and `f(x) == x`
/// exactly at the storage scale. The threshold returned here is the
/// conservative integer `10^(SCALE − ⌈(SCALE+2)/3⌉)` in storage
/// units (one decimal digit safety margin from the exact bound).
///
/// The atan-shaped functions (`c = 1/3`) get the tightest correction;
/// the sin-shaped functions (`c = 1/6`) and asin-shaped (`c = 1/6`)
/// are slightly less restrictive but using the atan threshold for
/// uniformity is safe for all and avoids per-function tuning.
#[inline]
const fn small_x_linear_threshold<const SCALE: u32>() -> i128 {
    let thresh_exp = SCALE.saturating_sub((SCALE + 2) / 3);
    10_i128.pow(thresh_exp)
}

fn wide_pi(w: u32) -> crate::d_w128_kernels::Fixed {
    debug_assert!(w <= 75, "wide_pi: working scale {w} exceeds embedded 75-digit π");
    // PI_RAW is an Int256, internally [u64; 4]. The D38 Fixed kernel
    // expects [u128; 2]; repack pairs of u64 limbs into u128.
    let words = crate::consts::PI_RAW.0;
    let pi_at_75 = crate::d_w128_kernels::Fixed {
        negative: false,
        mag: [
            (words[0] as u128) | ((words[1] as u128) << 64),
            (words[2] as u128) | ((words[3] as u128) << 64),
        ],
    };
    if w == 75 {
        pi_at_75
    } else {
        pi_at_75.rescale_down(75, w)
    }
}

/// τ = 2π at working scale `w`.
fn wide_tau(w: u32) -> crate::d_w128_kernels::Fixed {
    wide_pi(w).double()
}

/// π/2 at working scale `w`.
fn wide_half_pi(w: u32) -> crate::d_w128_kernels::Fixed {
    wide_pi(w).halve()
}

/// Builds a working-scale `Fixed` from a signed `D38` raw value `r`:
/// `r · 10^STRICT_GUARD`, carrying the sign.
fn to_fixed(raw: i128) -> crate::d_w128_kernels::Fixed {
    to_fixed_w(raw, crate::log_exp_strict::STRICT_GUARD)
}

/// Builds a working-scale `Fixed` from a signed `D38` raw value `r`:
/// `r · 10^working_digits`, carrying the sign. Used by the `_approx`
/// variants where the guard width is chosen at runtime.
fn to_fixed_w(raw: i128, working_digits: u32) -> crate::d_w128_kernels::Fixed {
    use crate::d_w128_kernels::Fixed;
    let m = Fixed::from_u128_mag(raw.unsigned_abs(), false)
        .mul_u128(10u128.pow(working_digits));
    if raw < 0 {
        m.neg()
    } else {
        m
    }
}

/// Shared `atan2` body factored out so the `_strict` and `_approx`
/// dispatchers can compose it at their chosen working scale `w`.
/// `y_raw` keeps the original sign of the y-argument for the x-zero
/// branch where the wide y value would have been signed-zero.
fn atan2_kernel(
    y: crate::d_w128_kernels::Fixed,
    x: crate::d_w128_kernels::Fixed,
    y_raw: i128,
    w: u32,
) -> crate::d_w128_kernels::Fixed {
    use crate::d_w128_kernels::Fixed;
    if x.is_zero() {
        return if y_raw > 0 {
            wide_half_pi(w)
        } else if y_raw < 0 {
            wide_half_pi(w).neg()
        } else {
            Fixed::ZERO
        };
    }
    // Max-branch: feed atan_fixed the |smaller|/|larger| ratio so the
    // argument-halving cascade doesn't blow up when |y| ≫ |x|.
    let abs_y_ge_abs_x = y.ge_mag(x);
    let base = if !abs_y_ge_abs_x {
        atan_fixed(y.div(x, w), w)
    } else {
        let inv = atan_fixed(x.div(y, w), w);
        let hp = wide_half_pi(w);
        let same_sign = y.negative == x.negative;
        if same_sign { hp.sub(inv) } else { hp.neg().sub(inv) }
    };
    if !x.negative {
        base
    } else if !y.negative {
        base.add(wide_pi(w))
    } else {
        base.sub(wide_pi(w))
    }
}

/// Taylor series for `sin` on a reduced non-negative argument
/// `r ∈ [0, π/2]`, evaluated at working scale `w`.
fn sin_taylor(r: crate::d_w128_kernels::Fixed, w: u32) -> crate::d_w128_kernels::Fixed {
    let r2 = r.mul(r, w);
    let mut sum = r;
    let mut term = r; // term = r^(2k-1)
    let mut k: u128 = 1;
    loop {
        // term_k = term_{k-1} · r² / ((2k)(2k+1)); sign alternates.
        term = term.mul(r2, w).div_small((2 * k) * (2 * k + 1));
        if term.is_zero() {
            break;
        }
        if k % 2 == 1 {
            sum = sum.sub(term);
        } else {
            sum = sum.add(term);
        }
        k += 1;
        if k > 200 {
            break;
        }
    }
    sum
}

/// Sine of a working-scale value `v_w`, at working scale `w`.
///
/// Reduces `v` modulo τ via `q = round(v/τ)`, folds the remainder into
/// `[0, π/2]` tracking sign and the `π − x` reflection, then evaluates
/// the Taylor series.
fn sin_fixed(v_w: crate::d_w128_kernels::Fixed, w: u32) -> crate::d_w128_kernels::Fixed {
    use crate::d_w128_kernels::Fixed;
    let tau = wide_tau(w);
    let pi = wide_pi(w);
    let half_pi = wide_half_pi(w);

    // r = v - round(v/τ)·τ ∈ [-π, π].
    let q = v_w.div(tau, w).round_to_nearest_int(w);
    let q_tau = if q >= 0 {
        tau.mul_u128(q as u128)
    } else {
        tau.mul_u128((-q) as u128).neg()
    };
    let r = v_w.sub(q_tau);

    // Fold |r| ∈ [0, π] into [0, π/2] via sin(π − x) = sin(x).
    let sign = r.negative;
    let abs_r = Fixed { negative: false, mag: r.mag };
    let reduced = if abs_r.ge_mag(half_pi) {
        pi.sub(abs_r)
    } else {
        abs_r
    };
    let s = sin_taylor(reduced, w);
    if sign {
        s.neg()
    } else {
        s
    }
}

/// Taylor series for `atan` on a reduced non-negative argument
/// `x ∈ [0, ~1/8]`, evaluated at working scale `w`.
fn atan_taylor(x: crate::d_w128_kernels::Fixed, w: u32) -> crate::d_w128_kernels::Fixed {
    let x2 = x.mul(x, w);
    let mut sum = x;
    let mut term = x; // term = x^(2k-1)
    let mut k: u128 = 1;
    loop {
        term = term.mul(x2, w);
        let contrib = term.div_small(2 * k + 1);
        if contrib.is_zero() {
            break;
        }
        if k % 2 == 1 {
            sum = sum.sub(contrib);
        } else {
            sum = sum.add(contrib);
        }
        k += 1;
        if k > 300 {
            break;
        }
    }
    sum
}

/// Arctangent of a working-scale value `v_w`, at working scale `w`,
/// result in `(−π/2, π/2)`.
///
/// Odd-function fold to `x ≥ 0`; reciprocal reduction
/// `atan(x) = π/2 − atan(1/x)` for `x > 1`; three rounds of argument
/// halving `atan(x) = 2·atan(x / (1 + √(1+x²)))`; then the series.
fn atan_fixed(v_w: crate::d_w128_kernels::Fixed, w: u32) -> crate::d_w128_kernels::Fixed {
    use crate::d_w128_kernels::Fixed;

    #[cfg(feature = "perf-trace")]
    let _atan_span = ::tracing::info_span!("atan_fixed").entered();

    #[cfg(feature = "perf-trace")]
    let _setup_span = ::tracing::info_span!("setup").entered();
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let sign = v_w.negative;
    let mut x = Fixed { negative: false, mag: v_w.mag };
    let mut add_half_pi = false;
    if x.ge_mag(one_w) && x != one_w {
        x = one_w.div(x, w); // atan(x) = π/2 − atan(1/x)
        add_half_pi = true;
    }
    #[cfg(feature = "perf-trace")]
    drop(_setup_span);

    // Adaptive argument halvings: atan(x) = 2·atan(x / (1 + √(1+x²))).
    // Halve only while |x| > ~0.2 (the Taylor convergence target);
    // matches g_math's adaptive-halvings approach. Skips halvings
    // entirely when the input is already small (e.g. `atan(0.1)`
    // would previously waste 3 halvings to reach 0.0125 — now 0).
    // Hard cap at 8 halvings as a safety net against pathological
    // edge cases; the post-reciprocal-reduce `x` always falls below
    // the threshold long before that.
    #[cfg(feature = "perf-trace")]
    let _halvings_span = ::tracing::info_span!("halvings").entered();
    let halving_threshold = one_w.div_small(5); // 0.2 at scale w
    let mut halvings: u32 = 0;
    while x.ge_mag(halving_threshold) && halvings < 8 {
        let x2 = x.mul(x, w);
        let denom = one_w.add(one_w.add(x2).sqrt(w));
        x = x.div(denom, w);
        halvings += 1;
    }
    #[cfg(feature = "perf-trace")]
    drop(_halvings_span);

    #[cfg(feature = "perf-trace")]
    let _taylor_span = ::tracing::info_span!("taylor").entered();
    let mut result = atan_taylor(x, w);
    #[cfg(feature = "perf-trace")]
    drop(_taylor_span);

    #[cfg(feature = "perf-trace")]
    let _reasm_span = ::tracing::info_span!("reassemble").entered();
    result = result.shl(halvings);
    if add_half_pi {
        result = wide_half_pi(w).sub(result);
    }
    if sign {
        result.neg()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::DecimalConsts;
    use crate::core_type::D38s12;

    // Tolerance for single-operation results. The f64-bridge build is
    // one f64 round-trip (≤ 2 LSB); the integer-only `strict` build is
    // correctly rounded (≤ 0.5 ULP per call) and is held to the same
    // 2-LSB bound — a couple of LSB for the test's own expected-value
    // rounding.
    const TWO_LSB: i128 = 2;

    // Tolerance for results that chain multiple trig calls (e.g.
    // `sin² + cos²`, `cosh² − sinh²`): each input is within 0.5 ULP, so
    // the composed quantity stays within a few LSB in both builds.
    const FOUR_LSB: i128 = 4;

    // Angle-conversion results compared against exact integer targets
    // (180, 90, 45 degrees). The `pi()` / `quarter_pi()` *input*
    // constants are themselves rounded to the type's scale, and
    // `to_degrees` amplifies that input quantization by ~57.3 — so even
    // a perfectly-rounded conversion lands ~30 LSB off the exact
    // integer at SCALE = 12. (This bounds the *input*, not the
    // conversion: `to_degrees` itself is correctly rounded in `strict`.)
    const ANGLE_TOLERANCE_LSB: i128 = 32;

    fn within_lsb(actual: D38s12, expected: D38s12, lsb: i128) -> bool {
        let diff = (actual.to_bits() - expected.to_bits()).abs();
        diff <= lsb
    }

    // ── Forward trig ──────────────────────────────────────────────────

    /// The strict trig / hyperbolic family is correctly rounded:
    /// cross-check every method against the f64 bridge at D38<9>,
    /// where f64 (≈ 15–16 significant digits) is comfortably more
    /// precise than the type's ULP, so a correctly-rounded integer
    /// result must agree to within 1 ULP (allow 1 more for the f64
    /// reference's own rounding).
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[test]
    fn strict_trig_family_matches_f64() {
        use crate::core_type::D38;
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
        // Forward trig — arguments across a few periods, incl. negative.
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
        // asin / acos — domain [-1, 1].
        for &raw in &[
            -1_000_000_000_i128, -700_000_000, -100_000_000, 0,
            250_000_000, 500_000_000, 999_999_999,
        ] {
            let x = D38::<9>::from_bits(raw);
            check!("asin", raw, x.asin_strict().to_bits(), f64::asin);
            check!("acos", raw, x.acos_strict().to_bits(), f64::acos);
        }
        // atanh — domain (-1, 1).
        for &raw in &[-900_000_000_i128, -300_000_000, 1, 300_000_000, 900_000_000] {
            let x = D38::<9>::from_bits(raw);
            check!("atanh", raw, x.atanh_strict().to_bits(), f64::atanh);
        }
        // acosh — domain [1, ∞).
        for &raw in &[1_000_000_000_i128, 1_500_000_000, 3_000_000_000, 50_000_000_000] {
            let x = D38::<9>::from_bits(raw);
            check!("acosh", raw, x.acosh_strict().to_bits(), f64::acosh);
        }
        // tan — avoid the poles.
        for &raw in &[-1_000_000_000_i128, 1, 500_000_000, 1_000_000_000, 1_400_000_000] {
            let x = D38::<9>::from_bits(raw);
            check!("tan", raw, x.tan_strict().to_bits(), f64::tan);
        }
    }

    /// `sin(0) == 0` -- bit-exact via `f64::sin(0.0) == 0.0`.
    #[test]
    fn sin_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sin(), D38s12::ZERO);
    }

    /// Regression: D38 strict trig at high SCALE drives the working
    /// scale `w = SCALE + STRICT_GUARD` past the old hard-coded
    /// 63-digit π constant. With the previous wide_pi the
    /// `rescale_down(63, w > 63)` call wrapped `from_w - to_w` as u32
    /// and produced a silently-wrong π. Verify two representative
    /// SCALEs above the old window (working scale 65 and 67) land
    /// within 1 LSB of the high-precision canonical sin(1) value.
    ///
    /// Reference: sin(1) = 0.8414709848078965066525023216302989996226...
    /// (OEIS A049469, 40 digits).
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[test]
    fn sin_one_correct_past_63_digit_pi_window() {
        use crate::core_type::D38;
        // sin(1) × 10^35, rounded half-to-even:
        // 0.84147098480789650665250232163029899|96226... → ...029900
        let expected_35: i128 = 84_147_098_480_789_650_665_250_232_163_029_900;
        // sin(1) × 10^37, rounded:
        // 0.8414709848078965066525023216302989996|226... → ...02989996
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

    /// `cos(0) == 1` -- bit-exact via `f64::cos(0.0) == 1.0`.
    #[test]
    fn cos_zero_is_one() {
        assert_eq!(D38s12::ZERO.cos(), D38s12::ONE);
    }

    /// `tan(0) == 0` -- bit-exact via `f64::tan(0.0) == 0.0`.
    #[test]
    fn tan_zero_is_zero() {
        assert_eq!(D38s12::ZERO.tan(), D38s12::ZERO);
    }

    /// Pythagorean identity: `sin^2(x) + cos^2(x) ~= 1` within 4 LSB
    /// for representative values of `x`. Values are chosen to be well
    /// away from any well-known mathematical constant.
    #[test]
    fn sin_squared_plus_cos_squared_is_one() {
        for raw in [
            1_234_567_890_123_i128,  // ~1.234567...
            -2_345_678_901_234_i128, // ~-2.345678...
            500_000_000_000_i128,    // 0.5
            -500_000_000_000_i128,   // -0.5
            4_567_891_234_567_i128,  // ~4.567891...
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

    /// `asin(0) == 0` -- bit-exact.
    #[test]
    fn asin_zero_is_zero() {
        assert_eq!(D38s12::ZERO.asin(), D38s12::ZERO);
    }

    /// `acos(1) == 0` -- bit-exact via `f64::acos(1.0) == 0.0`.
    #[test]
    fn acos_one_is_zero() {
        assert_eq!(D38s12::ONE.acos(), D38s12::ZERO);
    }

    /// `acos(0) ~= pi/2` within 4 LSB.
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

    /// `atan(0) == 0` -- bit-exact via `f64::atan(0.0) == 0.0`.
    #[test]
    fn atan_zero_is_zero() {
        assert_eq!(D38s12::ZERO.atan(), D38s12::ZERO);
    }

    /// Round-trip identity: `asin(sin(x)) ~= x` for `x` in
    /// `[-pi/2, pi/2]`. Values stay within the principal branch.
    #[test]
    fn asin_of_sin_round_trip() {
        for raw in [
            123_456_789_012_i128,    // ~0.123456...
            -123_456_789_012_i128,   // ~-0.123456...
            456_789_012_345_i128,    // ~0.456789...
            -456_789_012_345_i128,   // ~-0.456789...
            1_234_567_890_123_i128,  // ~1.234567... (well inside pi/2)
            -1_234_567_890_123_i128, // ~-1.234567...
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

    /// `atan2(1, 1) ~= pi/4` (first-quadrant 45 degrees).
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

    /// `atan2(-1, -1) ~= -3*pi/4` (third-quadrant correctness).
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

    /// `atan2(1, -1) ~= 3*pi/4` (second-quadrant correctness).
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

    /// `atan2(-1, 1) ~= -pi/4` (fourth-quadrant correctness).
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

    /// `atan2(0, 1) == 0` (positive x-axis is bit-exact).
    #[test]
    fn atan2_positive_x_axis_is_zero() {
        let zero = D38s12::ZERO;
        let one = D38s12::ONE;
        assert_eq!(zero.atan2(one), D38s12::ZERO);
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    /// `sinh(0) == 0` -- bit-exact via `f64::sinh(0.0) == 0.0`.
    #[test]
    fn sinh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sinh(), D38s12::ZERO);
    }

    /// `cosh(0) == 1` -- bit-exact via `f64::cosh(0.0) == 1.0`.
    #[test]
    fn cosh_zero_is_one() {
        assert_eq!(D38s12::ZERO.cosh(), D38s12::ONE);
    }

    /// `tanh(0) == 0` -- bit-exact via `f64::tanh(0.0) == 0.0`.
    #[test]
    fn tanh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.tanh(), D38s12::ZERO);
    }

    /// `asinh(0) == 0` -- bit-exact.
    #[test]
    fn asinh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.asinh(), D38s12::ZERO);
    }

    /// `acosh(1) == 0` -- bit-exact via `f64::acosh(1.0) == 0.0`.
    #[test]
    fn acosh_one_is_zero() {
        assert_eq!(D38s12::ONE.acosh(), D38s12::ZERO);
    }

    /// `atanh(0) == 0` -- bit-exact.
    #[test]
    fn atanh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.atanh(), D38s12::ZERO);
    }

    /// Identity: `cosh^2(x) - sinh^2(x) == 1` within 4 LSB for
    /// representative values of `x`.
    #[test]
    fn cosh_squared_minus_sinh_squared_is_one() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        for raw in [
            500_000_000_000_i128,    // 0.5
            -500_000_000_000_i128,   // -0.5
            1_234_567_890_123_i128,  // ~1.234567
            -1_234_567_890_123_i128, // ~-1.234567
            2_500_000_000_000_i128,  // 2.5
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

    /// `to_degrees(pi) ~= 180` within `ANGLE_TOLERANCE_LSB`. The
    /// tolerance is dominated by f64's limited precision on `pi`,
    /// amplified by ~57.3 during the degrees conversion.
    #[test]
    fn to_degrees_pi_is_180() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
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

    /// `to_radians(180) ~= pi` within `ANGLE_TOLERANCE_LSB`.
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

    /// `to_degrees(0) == 0` -- bit-exact (0 * anything == 0).
    #[test]
    fn to_degrees_zero_is_zero() {
        assert_eq!(D38s12::ZERO.to_degrees(), D38s12::ZERO);
    }

    /// `to_radians(0) == 0` -- bit-exact.
    #[test]
    fn to_radians_zero_is_zero() {
        assert_eq!(D38s12::ZERO.to_radians(), D38s12::ZERO);
    }

    /// Round-trip: `to_radians(to_degrees(x)) ~= x` within 4 LSB
    /// (two f64 round-trips).
    #[test]
    fn to_radians_to_degrees_round_trip() {
        for raw in [
            500_000_000_000_i128,    // 0.5
            -500_000_000_000_i128,   // -0.5
            1_234_567_890_123_i128,  // ~1.234567
            -2_345_678_901_234_i128, // ~-2.345678
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

    /// `to_degrees(half_pi) ~= 90` within `ANGLE_TOLERANCE_LSB`.
    #[test]
    fn to_degrees_half_pi_is_90() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
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

    /// `to_degrees(quarter_pi) ~= 45` within `ANGLE_TOLERANCE_LSB`.
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

    /// `tan(x) ~= sin(x) / cos(x)` within 4 LSB for `x` away from
    /// odd multiples of `pi/2`.
    #[test]
    fn tan_matches_sin_over_cos() {
        for raw in [
            500_000_000_000_i128,    // 0.5
            -500_000_000_000_i128,   // -0.5
            1_000_000_000_000_i128,  // 1.0 (cos(1.0) ~= 0.54, safe)
            -1_000_000_000_000_i128, // -1.0
            123_456_789_012_i128,    // ~0.123456
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

    /// `tanh(x) ~= sinh(x) / cosh(x)` within 4 LSB. `cosh` is always
    /// positive so there is no divide-by-zero risk.
    #[test]
    fn tanh_matches_sinh_over_cosh() {
        for raw in [
            500_000_000_000_i128,    // 0.5
            -500_000_000_000_i128,   // -0.5
            1_234_567_890_123_i128,  // ~1.234567
            -2_345_678_901_234_i128, // ~-2.345678
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

