//! Lossy (f64-bridge) trig methods for D38.
//!
//! Companion to `types/trig.rs`. The plain methods here are the
//! f64-bridge variants, gated on std + (no strict feature or
//! fast set). When strict is on, the dispatcher in the
//! _strict file shadows these.


impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    // ── Forward trig (radians input) ──────────────────────────────────

    /// Sine of `self`, where `self` is in radians.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // sin(0) == 0 (bit-exact: f64::sin(0.0) == 0.0).
    /// assert_eq!(D38s12::ZERO.sin(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn sin_fast(self) -> Self {
        Self::from_f64(self.to_f64().sin())
    }

    /// Cosine of `self`, where `self` is in radians.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // cos(0) == 1 (bit-exact: f64::cos(0.0) == 1.0).
    /// assert_eq!(D38s12::ZERO.cos(), D38s12::ONE);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn cos_fast(self) -> Self {
        Self::from_f64(self.to_f64().cos())
    }

    /// Tangent of `self`, where `self` is in radians.
    ///
    /// `f64::tan` returns very large magnitudes near odd multiples of
    /// `pi/2` and infinity at the limit. Inputs that drive the f64
    /// result outside `[D38::MIN, D38::MAX]` saturate per
    /// [`Self::from_f64`].
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // tan(0) == 0 (bit-exact: f64::tan(0.0) == 0.0).
    /// assert_eq!(D38s12::ZERO.tan(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn tan_fast(self) -> Self {
        Self::from_f64(self.to_f64().tan())
    }

    // ── Inverse trig (returns radians) ────────────────────────────────

    /// Arcsine of `self`. Returns radians in `[-pi/2, pi/2]`.
    ///
    /// `f64::asin` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64`] maps to `D38::ZERO`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // asin(0) == 0.
    /// assert_eq!(D38s12::ZERO.asin(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn asin_fast(self) -> Self {
        Self::from_f64(self.to_f64().asin())
    }

    /// Arccosine of `self`. Returns radians in `[0, pi]`.
    ///
    /// `f64::acos` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64`] maps to `D38::ZERO`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::{D38s12, DecimalConstants};
    /// // acos(1) == 0.
    /// assert_eq!(D38s12::ONE.acos(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn acos_fast(self) -> Self {
        Self::from_f64(self.to_f64().acos())
    }

    /// Arctangent of `self`. Returns radians in `(-pi/2, pi/2)`.
    ///
    /// Defined for the entire real line.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // atan(0) == 0.
    /// assert_eq!(D38s12::ZERO.atan(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn atan_fast(self) -> Self {
        Self::from_f64(self.to_f64().atan())
    }

    /// Four-quadrant arctangent of `self` (`y`) over `other` (`x`).
    /// Returns radians in `(-pi, pi]`.
    ///
    /// Signature matches `f64::atan2(self, other)`: the receiver is
    /// `y` and the argument is `x`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::{D38s12, DecimalConstants};
    /// // atan2(1, 1) ~= pi/4 (45 degrees, first quadrant).
    /// let one = D38s12::ONE;
    /// let result = one.atan2(one); // approximately D38s12::quarter_pi()
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn atan2_fast(self, other: Self) -> Self {
        Self::from_f64(self.to_f64().atan2(other.to_f64()))
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    /// Hyperbolic sine of `self`.
    ///
    /// Defined for the entire real line. Saturates at large magnitudes
    /// per [`Self::from_f64`].
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // sinh(0) == 0.
    /// assert_eq!(D38s12::ZERO.sinh(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn sinh_fast(self) -> Self {
        Self::from_f64(self.to_f64().sinh())
    }

    /// Hyperbolic cosine of `self`.
    ///
    /// Defined for the entire real line; result is always >= 1.
    /// Saturates at large magnitudes per [`Self::from_f64`].
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // cosh(0) == 1.
    /// assert_eq!(D38s12::ZERO.cosh(), D38s12::ONE);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn cosh_fast(self) -> Self {
        Self::from_f64(self.to_f64().cosh())
    }

    /// Hyperbolic tangent of `self`.
    ///
    /// Defined for the entire real line; range is `(-1, 1)`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // tanh(0) == 0.
    /// assert_eq!(D38s12::ZERO.tanh(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn tanh_fast(self) -> Self {
        Self::from_f64(self.to_f64().tanh())
    }

    /// Inverse hyperbolic sine of `self`.
    ///
    /// Defined for the entire real line.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // asinh(0) == 0.
    /// assert_eq!(D38s12::ZERO.asinh(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn asinh_fast(self) -> Self {
        Self::from_f64(self.to_f64().asinh())
    }

    /// Inverse hyperbolic cosine of `self`.
    ///
    /// `f64::acosh` returns NaN for inputs less than 1, which
    /// [`Self::from_f64`] maps to `D38::ZERO`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // acosh(1) == 0.
    /// assert_eq!(D38s12::ONE.acosh(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn acosh_fast(self) -> Self {
        Self::from_f64(self.to_f64().acosh())
    }

    /// Inverse hyperbolic tangent of `self`.
    ///
    /// `f64::atanh` returns NaN for inputs outside `(-1, 1)`, which
    /// [`Self::from_f64`] maps to `D38::ZERO`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // atanh(0) == 0.
    /// assert_eq!(D38s12::ZERO.atanh(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn atanh_fast(self) -> Self {
        Self::from_f64(self.to_f64().atanh())
    }

    // ── Angle conversions ─────────────────────────────────────────────

    /// Convert radians to degrees: `self * (180 / pi)`.
    ///
    /// Routed through `f64::to_degrees` so results match the de facto
    /// reference produced by the rest of the Rust ecosystem. Multiplying
    /// by a precomputed `D38` factor derived from `D38::pi()` would
    /// diverge from f64 by a 1-LSB rescale rounding without any
    /// practical determinism gain, since the f64 bridge is already the
    /// precision floor.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // to_degrees(0) == 0.
    /// assert_eq!(D38s12::ZERO.to_degrees(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn to_degrees_fast(self) -> Self {
        Self::from_f64(self.to_f64().to_degrees())
    }

    /// Convert degrees to radians: `self * (pi / 180)`.
    ///
    /// Routed through `f64::to_radians`. See [`Self::to_degrees`] for
    /// the rationale.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # #[cfg(feature = "std")]
    /// # {
    /// use decimal_scaled::D38s12;
    /// // to_radians(0) == 0.
    /// assert_eq!(D38s12::ZERO.to_radians(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[inline]
    #[must_use]
    pub fn to_radians_fast(self) -> Self {
        Self::from_f64(self.to_f64().to_radians())
    }
}

#[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    /// Plain dispatcher: forwards to [`Self::sin_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        self.sin_fast()
    }
    /// Plain dispatcher: forwards to [`Self::cos_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        self.cos_fast()
    }
    /// Plain dispatcher: forwards to [`Self::tan_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        self.tan_fast()
    }
    /// Plain dispatcher: forwards to [`Self::asin_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        self.asin_fast()
    }
    /// Plain dispatcher: forwards to [`Self::acos_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        self.acos_fast()
    }
    /// Plain dispatcher: forwards to [`Self::atan_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        self.atan_fast()
    }
    /// Plain dispatcher: forwards to [`Self::atan2_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        self.atan2_fast(other)
    }
    /// Plain dispatcher: forwards to [`Self::sinh_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn sinh(self) -> Self {
        self.sinh_fast()
    }
    /// Plain dispatcher: forwards to [`Self::cosh_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn cosh(self) -> Self {
        self.cosh_fast()
    }
    /// Plain dispatcher: forwards to [`Self::tanh_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn tanh(self) -> Self {
        self.tanh_fast()
    }
    /// Plain dispatcher: forwards to [`Self::asinh_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        self.asinh_fast()
    }
    /// Plain dispatcher: forwards to [`Self::acosh_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        self.acosh_fast()
    }
    /// Plain dispatcher: forwards to [`Self::atanh_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn atanh(self) -> Self {
        self.atanh_fast()
    }
    /// Plain dispatcher: forwards to [`Self::to_degrees_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> Self {
        self.to_degrees_fast()
    }
    /// Plain dispatcher: forwards to [`Self::to_radians_fast`] in this feature mode.
    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        self.to_radians_fast()
    }
}
