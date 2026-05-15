//! Lossy (f64-bridge) trig methods for D38.
//!
//! Companion to trig_strict.rs. The plain methods here are the
//! f64-bridge variants, gated on std + (no strict feature or
//! fast set). When strict is on, the dispatcher in the
//! _strict file shadows these.

use crate::core_type::D38;

impl<const SCALE: u32> D38<SCALE> {
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().sin())
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().cos())
    }

    /// Tangent of `self`, where `self` is in radians.
    ///
    /// `f64::tan` returns very large magnitudes near odd multiples of
    /// `pi/2` and infinity at the limit. Inputs that drive the f64
    /// result outside `[D38::MIN, D38::MAX]` saturate per
    /// [`Self::from_f64_lossy`].
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().tan())
    }

    // ── Inverse trig (returns radians) ────────────────────────────────

    /// Arcsine of `self`. Returns radians in `[-pi/2, pi/2]`.
    ///
    /// `f64::asin` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64_lossy`] maps to `D38::ZERO`.
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().asin())
    }

    /// Arccosine of `self`. Returns radians in `[0, pi]`.
    ///
    /// `f64::acos` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64_lossy`] maps to `D38::ZERO`.
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
    /// use decimal_scaled::{D38s12, DecimalConsts};
    /// // acos(1) == 0.
    /// assert_eq!(D38s12::ONE.acos(), D38s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().acos())
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().atan())
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
    /// use decimal_scaled::{D38s12, DecimalConsts};
    /// // atan2(1, 1) ~= pi/4 (45 degrees, first quadrant).
    /// let one = D38s12::ONE;
    /// let result = one.atan2(one); // approximately D38s12::quarter_pi()
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().atan2(other.to_f64_lossy()))
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    /// Hyperbolic sine of `self`.
    ///
    /// Defined for the entire real line. Saturates at large magnitudes
    /// per [`Self::from_f64_lossy`].
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn sinh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().sinh())
    }

    /// Hyperbolic cosine of `self`.
    ///
    /// Defined for the entire real line; result is always >= 1.
    /// Saturates at large magnitudes per [`Self::from_f64_lossy`].
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn cosh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().cosh())
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn tanh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().tanh())
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().asinh())
    }

    /// Inverse hyperbolic cosine of `self`.
    ///
    /// `f64::acosh` returns NaN for inputs less than 1, which
    /// [`Self::from_f64_lossy`] maps to `D38::ZERO`.
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().acosh())
    }

    /// Inverse hyperbolic tangent of `self`.
    ///
    /// `f64::atanh` returns NaN for inputs outside `(-1, 1)`, which
    /// [`Self::from_f64_lossy`] maps to `D38::ZERO`.
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn atanh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().atanh())
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().to_degrees())
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
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().to_radians())
    }
}
