//! Trigonometric, hyperbolic, and angle-conversion methods for [`D128`].
//!
//! # Methods
//!
//! Fifteen methods:
//!
//! - **Forward trig (radians input):** [`D128::sin`] / [`D128::cos`] /
//!   [`D128::tan`].
//! - **Inverse trig (returns radians):** [`D128::asin`] / [`D128::acos`]
//!   / [`D128::atan`] / [`D128::atan2`].
//! - **Hyperbolic:** [`D128::sinh`] / [`D128::cosh`] / [`D128::tanh`] /
//!   [`D128::asinh`] / [`D128::acosh`] / [`D128::atanh`].
//! - **Angle conversions:** [`D128::to_degrees`] / [`D128::to_radians`].
//!
//! # The `*_strict` dual API
//!
//! Each method has two implementations:
//!
//! - An integer-only `<method>_strict` form — always compiled (unless
//!   the `no_strict` feature is set), `no_std`-compatible, and
//!   platform-deterministic. `sin`/`cos`/`tan` range-reduce and
//!   evaluate a Taylor series; `atan`/`asin`/`acos`/`atan2` derive from
//!   a reciprocal-reduced Taylor `atan`; the hyperbolic family composes
//!   the strict `exp` / `ln` / `sqrt`.
//! - An f64-bridge form — converts to `f64`, calls the platform
//!   intrinsic, converts back. Gated on `std`.
//!
//! The plain `<method>` is a dispatcher: with the `strict` feature it
//! calls `<method>_strict`; otherwise it is the f64 bridge. See
//! `docs/strict-mode.md` for the full dual-API and feature-gating
//! rules and the 0.5 ULP accuracy contract.
//!
//! # Precision
//!
//! The f64-bridge forms are **Lossy** — the `D128` value round-trips
//! through `f64`, which introduces up to one LSB of quantisation per
//! conversion. The `*_strict` forms are held to the IEEE-754
//! correctly-rounded standard (within 0.5 ULP of the exact result);
//! the trig family is mid-rework toward that bound (see
//! `research/strict_transcendentals_research.md`).
//!
//! # `atan2` signature
//!
//! `f64::atan2(self, other)` treats `self` as `y` and `other` as `x`.
//! This module matches that signature exactly so generic numeric code
//! calling `y.atan2(x)` works with `T = D128`.

use crate::core_type::D128;

impl<const SCALE: u32> D128<SCALE> {
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
    /// use decimal_scaled::D128s12;
    /// // sin(0) == 0 (bit-exact: f64::sin(0.0) == 0.0).
    /// assert_eq!(D128s12::ZERO.sin(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // cos(0) == 1 (bit-exact: f64::cos(0.0) == 1.0).
    /// assert_eq!(D128s12::ZERO.cos(), D128s12::ONE);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().cos())
    }

    /// Tangent of `self`, where `self` is in radians.
    ///
    /// `f64::tan` returns very large magnitudes near odd multiples of
    /// `pi/2` and infinity at the limit. Inputs that drive the f64
    /// result outside `[D128::MIN, D128::MAX]` saturate per
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
    /// use decimal_scaled::D128s12;
    /// // tan(0) == 0 (bit-exact: f64::tan(0.0) == 0.0).
    /// assert_eq!(D128s12::ZERO.tan(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().tan())
    }

    // ── Inverse trig (returns radians) ────────────────────────────────

    /// Arcsine of `self`. Returns radians in `[-pi/2, pi/2]`.
    ///
    /// `f64::asin` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64_lossy`] maps to `D128::ZERO`.
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
    /// use decimal_scaled::D128s12;
    /// // asin(0) == 0.
    /// assert_eq!(D128s12::ZERO.asin(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().asin())
    }

    /// Arccosine of `self`. Returns radians in `[0, pi]`.
    ///
    /// `f64::acos` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64_lossy`] maps to `D128::ZERO`.
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
    /// use decimal_scaled::{D128s12, DecimalConsts};
    /// // acos(1) == 0.
    /// assert_eq!(D128s12::ONE.acos(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // atan(0) == 0.
    /// assert_eq!(D128s12::ZERO.atan(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::{D128s12, DecimalConsts};
    /// // atan2(1, 1) ~= pi/4 (45 degrees, first quadrant).
    /// let one = D128s12::ONE;
    /// let result = one.atan2(one); // approximately D128s12::quarter_pi()
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // sinh(0) == 0.
    /// assert_eq!(D128s12::ZERO.sinh(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // cosh(0) == 1.
    /// assert_eq!(D128s12::ZERO.cosh(), D128s12::ONE);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // tanh(0) == 0.
    /// assert_eq!(D128s12::ZERO.tanh(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // asinh(0) == 0.
    /// assert_eq!(D128s12::ZERO.asinh(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().asinh())
    }

    /// Inverse hyperbolic cosine of `self`.
    ///
    /// `f64::acosh` returns NaN for inputs less than 1, which
    /// [`Self::from_f64_lossy`] maps to `D128::ZERO`.
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
    /// use decimal_scaled::D128s12;
    /// // acosh(1) == 0.
    /// assert_eq!(D128s12::ONE.acosh(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().acosh())
    }

    /// Inverse hyperbolic tangent of `self`.
    ///
    /// `f64::atanh` returns NaN for inputs outside `(-1, 1)`, which
    /// [`Self::from_f64_lossy`] maps to `D128::ZERO`.
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
    /// use decimal_scaled::D128s12;
    /// // atanh(0) == 0.
    /// assert_eq!(D128s12::ZERO.atanh(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// by a precomputed `D128` factor derived from `D128::pi()` would
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
    /// use decimal_scaled::D128s12;
    /// // to_degrees(0) == 0.
    /// assert_eq!(D128s12::ZERO.to_degrees(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
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
    /// use decimal_scaled::D128s12;
    /// // to_radians(0) == 0.
    /// assert_eq!(D128s12::ZERO.to_radians(), D128s12::ZERO);
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().to_radians())
    }
}

// ─────────────────────────────────────────────────────────────────────
// Strict-mode (integer-only) trigonometric, hyperbolic, and angle-
// conversion methods.
//
// These mirror the f64-bridge surface above but are compiled under
// `#[cfg(feature = "strict")]`. They are integer-only and `no_std`-
// compatible. Accuracy matches the rest of the strict module: within
// roughly ±10 ULP at moderate SCALE, degrading toward the extreme
// SCALEs (a tighter Remez-polynomial implementation is tracked in
// `research/strict_transcendentals_research.md`).
//
// Composition strategy:
//
// - Hyperbolic functions are composed from the strict `exp` / `ln` /
//   `sqrt` already implemented in `log_exp.rs` / `powers.rs`.
// - `cos` is `sin` phase-shifted by π/2; `tan` is `sin / cos`.
// - `sin` uses range reduction modulo τ into one π/2 octant followed by
//   a Taylor series.
// - `atan` uses reciprocal reduction for |x| > 1 plus argument halving,
//   then a Taylor series; `asin` / `acos` / `atan2` are derived from it.
// ─────────────────────────────────────────────────────────────────────

// Strict-feature dispatchers. When `strict` is enabled (and
// `no_strict` is not), the plain trig methods route to the
// integer-only `*_strict` implementations below.
#[cfg(all(feature = "strict", not(feature = "no_strict")))]
impl<const SCALE: u32> D128<SCALE> {
    /// With `strict` this dispatches to [`Self::sin_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        self.sin_strict()
    }

    /// With `strict` this dispatches to [`Self::cos_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        self.cos_strict()
    }

    /// With `strict` this dispatches to [`Self::tan_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        self.tan_strict()
    }

    /// With `strict` this dispatches to [`Self::asin_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        self.asin_strict()
    }

    /// With `strict` this dispatches to [`Self::acos_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        self.acos_strict()
    }

    /// With `strict` this dispatches to [`Self::atan_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        self.atan_strict()
    }

    /// Four-quadrant arctangent of `self` (`y`) and `other` (`x`).
    /// With `strict` this dispatches to [`Self::atan2_strict`];
    /// without it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn atan2(self, other: Self) -> Self {
        self.atan2_strict(other)
    }

    /// With `strict` this dispatches to [`Self::sinh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn sinh(self) -> Self {
        self.sinh_strict()
    }

    /// With `strict` this dispatches to [`Self::cosh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn cosh(self) -> Self {
        self.cosh_strict()
    }

    /// With `strict` this dispatches to [`Self::tanh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn tanh(self) -> Self {
        self.tanh_strict()
    }

    /// With `strict` this dispatches to [`Self::asinh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        self.asinh_strict()
    }

    /// With `strict` this dispatches to [`Self::acosh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        self.acosh_strict()
    }

    /// With `strict` this dispatches to [`Self::atanh_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn atanh(self) -> Self {
        self.atanh_strict()
    }

    /// With `strict` this dispatches to [`Self::to_degrees_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn to_degrees(self) -> Self {
        self.to_degrees_strict()
    }

    /// With `strict` this dispatches to [`Self::to_radians_strict`]; without
    /// it, the f64-bridge form is used instead.
    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        self.to_radians_strict()
    }
}

#[cfg(not(feature = "no_strict"))]
impl<const SCALE: u32> D128<SCALE> {
    /// Taylor series for `sin` on a reduced argument `r ∈ [0, π/2]`:
    /// `r − r³/3! + r⁵/5! − …`. Integer-only; terms are accumulated at
    /// the storage scale and the loop stops once a term underflows one
    /// LSB.
    fn taylor_sin_reduced(r: Self) -> Self {
        let mut sum_bits: i128 = r.to_bits();
        // term = r ; iteratively term *= -r*r / ((2k)(2k+1))
        let mut term = r;
        let mut k: i128 = 1;
        loop {
            // term_{k} = term_{k-1} * r^2 / ((2k)(2k+1)), alternating sign
            term = term * r;
            term = term * r;
            let denom = (2 * k) * (2 * k + 1);
            term = Self::from_bits(term.to_bits() / denom);
            let tb = term.to_bits();
            if tb == 0 {
                break;
            }
            if k & 1 == 1 {
                sum_bits = sum_bits.saturating_sub(tb);
            } else {
                sum_bits = sum_bits.saturating_add(tb);
            }
            k += 1;
            if k > 40 {
                break;
            }
        }
        Self::from_bits(sum_bits)
    }

    /// Taylor series for `atan` on a reduced argument `x ∈ [0, ~0.42]`:
    /// `x − x³/3 + x⁵/5 − …`. Integer-only.
    fn taylor_atan_reduced(x: Self) -> Self {
        let mut sum_bits: i128 = x.to_bits();
        let mut power = x; // x^(2k+1)
        let mut k: i128 = 1;
        loop {
            power = power * x;
            power = power * x;
            let denom = 2 * k + 1;
            let tb = power.to_bits() / denom;
            if tb == 0 {
                break;
            }
            if k & 1 == 1 {
                sum_bits = sum_bits.saturating_sub(tb);
            } else {
                sum_bits = sum_bits.saturating_add(tb);
            }
            k += 1;
            if k > 60 {
                break;
            }
        }
        Self::from_bits(sum_bits)
    }

    /// Sine of `self` (radians). Strict: integer-only.
    ///
    /// # Algorithm
    ///
    /// Reduce `self` modulo τ into `[0, τ)`, fold into one π/2 octant
    /// while tracking the result sign and whether the cofunction
    /// reflection (`π − x`) applies, then evaluate the Taylor series.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only. Accuracy is within a few
    /// ULP at moderate SCALE.
    #[inline]
    #[must_use]
    pub fn sin_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        let tau = Self::tau();
        let pi = Self::pi();
        let half_pi = Self::half_pi();
        // Range-reduce into [0, τ).
        let mut x = self % tau;
        if x.is_negative() {
            x = x + tau;
        }
        // Fold [0, τ) -> [0, π/2] with sign + reflection bookkeeping.
        let (r, negate) = if x <= half_pi {
            (x, false)
        } else if x <= pi {
            (pi - x, false)
        } else if x <= pi + half_pi {
            (x - pi, true)
        } else {
            (tau - x, true)
        };
        let s = Self::taylor_sin_reduced(r);
        if negate { -s } else { s }
    }

    /// Cosine of `self` (radians). Strict: `cos(x) = sin(x + π/2)`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only; accuracy as for [`Self::sin`].
    #[inline]
    #[must_use]
    pub fn cos_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        (self + Self::half_pi()).sin_strict()
    }

    /// Tangent of `self` (radians). Strict: `tan(x) = sin(x) / cos(x)`.
    ///
    /// # Panics
    ///
    /// Panics if `cos(self)` is zero (an odd multiple of π/2).
    ///
    /// # Precision
    ///
    /// Strict: integer-only; accuracy as for [`Self::sin`], with the
    /// usual blow-up near the poles.
    #[inline]
    #[must_use]
    pub fn tan_strict(self) -> Self {
        let c = self.cos_strict();
        if c == Self::ZERO {
            panic!("D128::tan: cosine is zero (argument is an odd multiple of pi/2)");
        }
        self.sin_strict() / c
    }

    /// Arctangent of `self`, in radians, in `(−π/2, π/2)`. Strict.
    ///
    /// # Algorithm
    ///
    /// Odd-function fold to `x ≥ 0`; reciprocal reduction
    /// `atan(x) = π/2 − atan(1/x)` for `x > 1`; two rounds of argument
    /// halving `atan(x) = 2·atan(x / (1 + √(1+x²)))` to shrink the
    /// series argument; then the Taylor series.
    ///
    /// # Precision
    ///
    /// Strict: integer-only; accuracy within a few ULP at moderate SCALE.
    #[inline]
    #[must_use]
    pub fn atan_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        let one = Self::ONE;
        let neg = self.is_negative();
        let mut x = if neg { -self } else { self };
        let mut add_half_pi = false;
        if x > one {
            x = one / x;
            add_half_pi = true;
        }
        // Two rounds of argument halving:
        //   atan(x) = 2 * atan( x / (1 + sqrt(1 + x*x)) )
        let mut halvings: u32 = 0;
        for _ in 0..2 {
            let denom = one + (one + x * x).sqrt_strict();
            x = x / denom;
            halvings += 1;
        }
        let mut result = Self::taylor_atan_reduced(x);
        for _ in 0..halvings {
            result = Self::from_bits(result.to_bits().saturating_mul(2));
        }
        if add_half_pi {
            result = Self::half_pi() - result;
        }
        if neg { -result } else { result }
    }

    /// Arcsine of `self`, in radians, in `[−π/2, π/2]`. Strict.
    ///
    /// `asin(x) = atan(x / √(1 − x²))`, with the endpoints `±1` mapped
    /// directly to `±π/2`.
    ///
    /// # Panics
    ///
    /// Panics if `|self| > 1`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn asin_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        let one = Self::ONE;
        let mag = if self.is_negative() { -self } else { self };
        if mag > one {
            panic!("D128::asin: argument out of domain [-1, 1]");
        }
        if mag == one {
            let hp = Self::half_pi();
            return if self.is_negative() { -hp } else { hp };
        }
        let denom = (one - self * self).sqrt_strict();
        (self / denom).atan_strict()
    }

    /// Arccosine of `self`, in radians, in `[0, π]`. Strict:
    /// `acos(x) = π/2 − asin(x)`.
    ///
    /// # Panics
    ///
    /// Panics if `|self| > 1`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn acos_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        Self::half_pi() - self.asin_strict()
    }

    /// Four-quadrant arctangent of `self` (`y`) and `other` (`x`), in
    /// radians, in `(−π, π]`. Strict.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn atan2_strict(self, other: Self) -> Self {
        use crate::consts::DecimalConsts;
        let y = self;
        let x = other;
        let zero = Self::ZERO;
        let pi = Self::pi();
        let half_pi = Self::half_pi();
        if x == zero {
            return if y.is_positive() {
                half_pi
            } else if y.is_negative() {
                -half_pi
            } else {
                zero
            };
        }
        let base = (y / x).atan_strict();
        if x.is_positive() {
            base
        } else if !y.is_negative() {
            base + pi
        } else {
            base - pi
        }
    }

    /// Hyperbolic sine of `self`. Strict:
    /// `sinh(x) = (eˣ − e⁻ˣ) / 2`, composed from the strict `exp`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn sinh_strict(self) -> Self {
        let ex = self.exp_strict();
        let enx = (-self).exp_strict();
        Self::from_bits((ex - enx).to_bits() / 2)
    }

    /// Hyperbolic cosine of `self`. Strict:
    /// `cosh(x) = (eˣ + e⁻ˣ) / 2`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn cosh_strict(self) -> Self {
        let ex = self.exp_strict();
        let enx = (-self).exp_strict();
        Self::from_bits((ex + enx).to_bits() / 2)
    }

    /// Hyperbolic tangent of `self`. Strict: `tanh(x) = sinh(x)/cosh(x)`.
    /// `cosh` is always ≥ 1, so the division never traps.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn tanh_strict(self) -> Self {
        self.sinh_strict() / self.cosh_strict()
    }

    /// Inverse hyperbolic sine of `self`. Strict:
    /// `asinh(x) = ln(x + √(x² + 1))`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn asinh_strict(self) -> Self {
        let one = Self::ONE;
        let inner = self + (self * self + one).sqrt_strict();
        inner.ln_strict()
    }

    /// Inverse hyperbolic cosine of `self`. Strict:
    /// `acosh(x) = ln(x + √(x² − 1))`, defined for `x ≥ 1`.
    ///
    /// # Panics
    ///
    /// Panics if `self < 1`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn acosh_strict(self) -> Self {
        let one = Self::ONE;
        if self < one {
            panic!("D128::acosh: argument must be >= 1");
        }
        let inner = self + (self * self - one).sqrt_strict();
        inner.ln_strict()
    }

    /// Inverse hyperbolic tangent of `self`. Strict:
    /// `atanh(x) = ln((1 + x) / (1 − x)) / 2`, defined for `|x| < 1`.
    ///
    /// # Panics
    ///
    /// Panics if `|self| >= 1`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn atanh_strict(self) -> Self {
        let one = Self::ONE;
        let mag = if self.is_negative() { -self } else { self };
        if mag >= one {
            panic!("D128::atanh: argument out of domain (-1, 1)");
        }
        let ratio = (one + self) / (one - self);
        Self::from_bits(ratio.ln_strict().to_bits() / 2)
    }

    /// Convert radians to degrees: `self · (180 / π)`. Strict.
    ///
    /// Computed as `(self · 180) / π` (multiply-first) so the only
    /// rounding step is the final divide. The `self · 180` intermediate
    /// can overflow only for absurdly large angles at very low SCALE.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn to_degrees_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        (self * Self::from_int(180)) / Self::pi()
    }

    /// Convert degrees to radians: `self · (π / 180)`. Strict.
    ///
    /// Computed as `(self · π) / 180` (multiply-first) so the only
    /// rounding step is the final divide.
    ///
    /// # Precision
    ///
    /// Strict: integer-only.
    #[inline]
    #[must_use]
    pub fn to_radians_strict(self) -> Self {
        use crate::consts::DecimalConsts;
        (self * Self::pi()) / Self::from_int(180)
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::DecimalConsts;
    use crate::core_type::D128s12;

    // Tolerance for single-operation results. In the f64-bridge build
    // each op is one f64 round-trip (≤ 2 LSB). The integer-only `strict`
    // build composes several ±ULP primitives per trig call (e.g. `atan`
    // chains a `sqrt`, two argument-halvings, and a Taylor series), so
    // the bound is widened to the strict module's documented ~10-ULP
    // envelope.
    // Tight bound when the dispatched methods are the f64 bridge
    // (default or `no_strict` builds); wider when they are the
    // integer-only strict path.
    #[cfg(any(not(feature = "strict"), feature = "no_strict"))]
    const TWO_LSB: i128 = 2;
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    const TWO_LSB: i128 = 12;

    // Tolerance for results that chain multiple trig calls. Same
    // rationale as `TWO_LSB`; the strict bound is wider still because
    // identities like `cosh² − sinh²` subtract two composed quantities.
    #[cfg(any(not(feature = "strict"), feature = "no_strict"))]
    const FOUR_LSB: i128 = 4;
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    const FOUR_LSB: i128 = 24;

    // Allow 32 LSB when comparing angle-conversion results against exact
    // integer targets (180, 90, 45 degrees). The D128::pi() constant has
    // more digits than f64 can represent; the rounding error multiplies
    // by ~57.3 during the degrees conversion, landing within ~30 LSB of
    // the exact integer at SCALE = 12.
    const ANGLE_TOLERANCE_LSB: i128 = 32;

    fn within_lsb(actual: D128s12, expected: D128s12, lsb: i128) -> bool {
        let diff = (actual.to_bits() - expected.to_bits()).abs();
        diff <= lsb
    }

    // ── Forward trig ──────────────────────────────────────────────────

    /// `sin(0) == 0` -- bit-exact via `f64::sin(0.0) == 0.0`.
    #[test]
    fn sin_zero_is_zero() {
        assert_eq!(D128s12::ZERO.sin(), D128s12::ZERO);
    }

    /// `cos(0) == 1` -- bit-exact via `f64::cos(0.0) == 1.0`.
    #[test]
    fn cos_zero_is_one() {
        assert_eq!(D128s12::ZERO.cos(), D128s12::ONE);
    }

    /// `tan(0) == 0` -- bit-exact via `f64::tan(0.0) == 0.0`.
    #[test]
    fn tan_zero_is_zero() {
        assert_eq!(D128s12::ZERO.tan(), D128s12::ZERO);
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
            let x = D128s12::from_bits(raw);
            let s = x.sin();
            let c = x.cos();
            let sum = (s * s) + (c * c);
            assert!(
                within_lsb(sum, D128s12::ONE, FOUR_LSB),
                "sin^2 + cos^2 != 1 for raw={raw}: got bits {} (delta {})",
                sum.to_bits(),
                (sum.to_bits() - D128s12::ONE.to_bits()).abs(),
            );
        }
    }

    // ── Inverse trig ──────────────────────────────────────────────────

    /// `asin(0) == 0` -- bit-exact.
    #[test]
    fn asin_zero_is_zero() {
        assert_eq!(D128s12::ZERO.asin(), D128s12::ZERO);
    }

    /// `acos(1) == 0` -- bit-exact via `f64::acos(1.0) == 0.0`.
    #[test]
    fn acos_one_is_zero() {
        assert_eq!(D128s12::ONE.acos(), D128s12::ZERO);
    }

    /// `acos(0) ~= pi/2` within 4 LSB.
    #[test]
    fn acos_zero_is_half_pi() {
        let result = D128s12::ZERO.acos();
        assert!(
            within_lsb(result, D128s12::half_pi(), FOUR_LSB),
            "acos(0) bits {}, half_pi bits {}",
            result.to_bits(),
            D128s12::half_pi().to_bits(),
        );
    }

    /// `atan(0) == 0` -- bit-exact via `f64::atan(0.0) == 0.0`.
    #[test]
    fn atan_zero_is_zero() {
        assert_eq!(D128s12::ZERO.atan(), D128s12::ZERO);
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
            let x = D128s12::from_bits(raw);
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
        let one = D128s12::ONE;
        let result = one.atan2(one);
        assert!(
            within_lsb(result, D128s12::quarter_pi(), TWO_LSB),
            "atan2(1, 1) bits {}, quarter_pi bits {}",
            result.to_bits(),
            D128s12::quarter_pi().to_bits(),
        );
    }

    /// `atan2(-1, -1) ~= -3*pi/4` (third-quadrant correctness).
    #[test]
    fn atan2_third_quadrant_diagonal() {
        let neg_one = -D128s12::ONE;
        let result = neg_one.atan2(neg_one);
        let three = D128s12::from_int(3);
        let expected = -(D128s12::quarter_pi() * three);
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
        let one = D128s12::ONE;
        let neg_one = -D128s12::ONE;
        let result = one.atan2(neg_one);
        let three = D128s12::from_int(3);
        let expected = D128s12::quarter_pi() * three;
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
        let one = D128s12::ONE;
        let neg_one = -D128s12::ONE;
        let result = neg_one.atan2(one);
        let expected = -D128s12::quarter_pi();
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
        let zero = D128s12::ZERO;
        let one = D128s12::ONE;
        assert_eq!(zero.atan2(one), D128s12::ZERO);
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    /// `sinh(0) == 0` -- bit-exact via `f64::sinh(0.0) == 0.0`.
    #[test]
    fn sinh_zero_is_zero() {
        assert_eq!(D128s12::ZERO.sinh(), D128s12::ZERO);
    }

    /// `cosh(0) == 1` -- bit-exact via `f64::cosh(0.0) == 1.0`.
    #[test]
    fn cosh_zero_is_one() {
        assert_eq!(D128s12::ZERO.cosh(), D128s12::ONE);
    }

    /// `tanh(0) == 0` -- bit-exact via `f64::tanh(0.0) == 0.0`.
    #[test]
    fn tanh_zero_is_zero() {
        assert_eq!(D128s12::ZERO.tanh(), D128s12::ZERO);
    }

    /// `asinh(0) == 0` -- bit-exact.
    #[test]
    fn asinh_zero_is_zero() {
        assert_eq!(D128s12::ZERO.asinh(), D128s12::ZERO);
    }

    /// `acosh(1) == 0` -- bit-exact via `f64::acosh(1.0) == 0.0`.
    #[test]
    fn acosh_one_is_zero() {
        assert_eq!(D128s12::ONE.acosh(), D128s12::ZERO);
    }

    /// `atanh(0) == 0` -- bit-exact.
    #[test]
    fn atanh_zero_is_zero() {
        assert_eq!(D128s12::ZERO.atanh(), D128s12::ZERO);
    }

    /// Identity: `cosh^2(x) - sinh^2(x) == 1` within 4 LSB for
    /// representative values of `x`.
    #[test]
    fn cosh_squared_minus_sinh_squared_is_one() {
        for raw in [
            500_000_000_000_i128,    // 0.5
            -500_000_000_000_i128,   // -0.5
            1_234_567_890_123_i128,  // ~1.234567
            -1_234_567_890_123_i128, // ~-1.234567
            2_500_000_000_000_i128,  // 2.5
        ] {
            let x = D128s12::from_bits(raw);
            let ch = x.cosh();
            let sh = x.sinh();
            let diff = (ch * ch) - (sh * sh);
            assert!(
                within_lsb(diff, D128s12::ONE, FOUR_LSB),
                "cosh^2 - sinh^2 != 1 for raw={raw}: got bits {} (delta {})",
                diff.to_bits(),
                (diff.to_bits() - D128s12::ONE.to_bits()).abs(),
            );
        }
    }

    // ── Angle conversions ─────────────────────────────────────────────

    /// `to_degrees(pi) ~= 180` within `ANGLE_TOLERANCE_LSB`. The
    /// tolerance is dominated by f64's limited precision on `pi`,
    /// amplified by ~57.3 during the degrees conversion.
    #[test]
    fn to_degrees_pi_is_180() {
        let pi = D128s12::pi();
        let result = pi.to_degrees();
        let expected = D128s12::from_int(180);
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
        let one_eighty = D128s12::from_int(180);
        let result = one_eighty.to_radians();
        let expected = D128s12::pi();
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
        assert_eq!(D128s12::ZERO.to_degrees(), D128s12::ZERO);
    }

    /// `to_radians(0) == 0` -- bit-exact.
    #[test]
    fn to_radians_zero_is_zero() {
        assert_eq!(D128s12::ZERO.to_radians(), D128s12::ZERO);
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
            let x = D128s12::from_bits(raw);
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
        let result = D128s12::half_pi().to_degrees();
        let expected = D128s12::from_int(90);
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
        let result = D128s12::quarter_pi().to_degrees();
        let expected = D128s12::from_int(45);
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
            let x = D128s12::from_bits(raw);
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
            let x = D128s12::from_bits(raw);
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
