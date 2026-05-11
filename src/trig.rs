//! Trigonometric, hyperbolic, and angle-conversion methods for [`I128`].
//!
//! # Methods
//!
//! Fifteen methods, all routed through the f64 bridge:
//!
//! - **Forward trig (radians input):** [`I128::sin`] / [`I128::cos`] /
//!   [`I128::tan`].
//! - **Inverse trig (returns radians):** [`I128::asin`] / [`I128::acos`]
//!   / [`I128::atan`] / [`I128::atan2`].
//! - **Hyperbolic:** [`I128::sinh`] / [`I128::cosh`] / [`I128::tanh`] /
//!   [`I128::asinh`] / [`I128::acosh`] / [`I128::atanh`].
//! - **Angle conversions:** [`I128::to_degrees`] / [`I128::to_radians`].
//!
//! # Feature gating
//!
//! Every method here calls an inherent `f64` method (`f64::sin`,
//! `f64::cos`, `f64::tan`, `f64::asin`, `f64::acos`, `f64::atan`,
//! `f64::atan2`, `f64::sinh`, `f64::cosh`, `f64::tanh`, `f64::asinh`,
//! `f64::acosh`, `f64::atanh`, `f64::to_degrees`, `f64::to_radians`),
//! which are only available in `std` — they delegate to platform or
//! hardware intrinsics that are not in `core`. The whole module is
//! gated `#[cfg(feature = "std")]` at the `mod trig;` declaration in
//! `lib.rs` rather than repeating the gate on each method.
//!
//! `no_std` users that need trigonometric or hyperbolic functions can
//! compose them externally via `libm` or hardware-specific intrinsics.
//!
//! # Precision
//!
//! All methods in this module are **Lossy**: the `I128` value is
//! converted to `f64` via `to_f64_lossy`, the corresponding `f64`
//! intrinsic is applied, and the result is converted back via
//! `from_f64_lossy`. The f64 round-trip introduces up to one LSB of
//! quantisation error per conversion step.
//!
//! IEEE 754 mandates correct rounding for `f64::sqrt` but not for
//! transcendental functions. In practice mainstream libm implementations
//! (glibc, msvcrt, macOS libm, musl) produce bit-identical results for
//! identical inputs, so results are bit-deterministic on supported
//! platforms in practice.
//!
//! # `atan2` signature
//!
//! `f64::atan2(self, other)` treats `self` as `y` and `other` as `x`.
//! This module matches that signature exactly so generic numeric code
//! calling `y.atan2(x)` works with `T = I128`.

use crate::core_type::I128;

impl<const SCALE: u32> I128<SCALE> {
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
    /// use decimal_scaled::I128s12;
    /// // sin(0) == 0 (bit-exact: f64::sin(0.0) == 0.0).
    /// assert_eq!(I128s12::ZERO.sin(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // cos(0) == 1 (bit-exact: f64::cos(0.0) == 1.0).
    /// assert_eq!(I128s12::ZERO.cos(), I128s12::ONE);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().cos())
    }

    /// Tangent of `self`, where `self` is in radians.
    ///
    /// `f64::tan` returns very large magnitudes near odd multiples of
    /// `pi/2` and infinity at the limit. Inputs that drive the f64
    /// result outside `[I128::MIN, I128::MAX]` saturate per
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
    /// use decimal_scaled::I128s12;
    /// // tan(0) == 0 (bit-exact: f64::tan(0.0) == 0.0).
    /// assert_eq!(I128s12::ZERO.tan(), I128s12::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().tan())
    }

    // ── Inverse trig (returns radians) ────────────────────────────────

    /// Arcsine of `self`. Returns radians in `[-pi/2, pi/2]`.
    ///
    /// `f64::asin` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64_lossy`] maps to `I128::ZERO`.
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
    /// use decimal_scaled::I128s12;
    /// // asin(0) == 0.
    /// assert_eq!(I128s12::ZERO.asin(), I128s12::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().asin())
    }

    /// Arccosine of `self`. Returns radians in `[0, pi]`.
    ///
    /// `f64::acos` returns NaN for inputs outside `[-1, 1]`, which
    /// [`Self::from_f64_lossy`] maps to `I128::ZERO`.
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
    /// use decimal_scaled::{I128s12, DecimalConsts};
    /// // acos(1) == 0.
    /// assert_eq!(I128s12::ONE.acos(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // atan(0) == 0.
    /// assert_eq!(I128s12::ZERO.atan(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// use decimal_scaled::{I128s12, DecimalConsts};
    /// // atan2(1, 1) ~= pi/4 (45 degrees, first quadrant).
    /// let one = I128s12::ONE;
    /// let result = one.atan2(one); // approximately I128s12::quarter_pi()
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // sinh(0) == 0.
    /// assert_eq!(I128s12::ZERO.sinh(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // cosh(0) == 1.
    /// assert_eq!(I128s12::ZERO.cosh(), I128s12::ONE);
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // tanh(0) == 0.
    /// assert_eq!(I128s12::ZERO.tanh(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // asinh(0) == 0.
    /// assert_eq!(I128s12::ZERO.asinh(), I128s12::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn asinh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().asinh())
    }

    /// Inverse hyperbolic cosine of `self`.
    ///
    /// `f64::acosh` returns NaN for inputs less than 1, which
    /// [`Self::from_f64_lossy`] maps to `I128::ZERO`.
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
    /// use decimal_scaled::I128s12;
    /// // acosh(1) == 0.
    /// assert_eq!(I128s12::ONE.acosh(), I128s12::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn acosh(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().acosh())
    }

    /// Inverse hyperbolic tangent of `self`.
    ///
    /// `f64::atanh` returns NaN for inputs outside `(-1, 1)`, which
    /// [`Self::from_f64_lossy`] maps to `I128::ZERO`.
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
    /// use decimal_scaled::I128s12;
    /// // atanh(0) == 0.
    /// assert_eq!(I128s12::ZERO.atanh(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// by a precomputed `I128` factor derived from `I128::pi()` would
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
    /// use decimal_scaled::I128s12;
    /// // to_degrees(0) == 0.
    /// assert_eq!(I128s12::ZERO.to_degrees(), I128s12::ZERO);
    /// # }
    /// ```
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
    /// use decimal_scaled::I128s12;
    /// // to_radians(0) == 0.
    /// assert_eq!(I128s12::ZERO.to_radians(), I128s12::ZERO);
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn to_radians(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().to_radians())
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::DecimalConsts;
    use crate::core_type::I128s12;

    // Allow 2 LSB of tolerance for single f64 round-trip operations.
    const TWO_LSB: i128 = 2;

    // Allow 4 LSB of tolerance for operations that chain multiple trig
    // calls, each adding up to 1 LSB of quantisation slack.
    const FOUR_LSB: i128 = 4;

    // Allow 32 LSB when comparing angle-conversion results against exact
    // integer targets (180, 90, 45 degrees). The I128::pi() constant has
    // more digits than f64 can represent; the rounding error multiplies
    // by ~57.3 during the degrees conversion, landing within ~30 LSB of
    // the exact integer at SCALE = 12.
    const ANGLE_TOLERANCE_LSB: i128 = 32;

    fn within_lsb(actual: I128s12, expected: I128s12, lsb: i128) -> bool {
        let diff = (actual.to_bits() - expected.to_bits()).abs();
        diff <= lsb
    }

    // ── Forward trig ──────────────────────────────────────────────────

    /// `sin(0) == 0` -- bit-exact via `f64::sin(0.0) == 0.0`.
    #[test]
    fn sin_zero_is_zero() {
        assert_eq!(I128s12::ZERO.sin(), I128s12::ZERO);
    }

    /// `cos(0) == 1` -- bit-exact via `f64::cos(0.0) == 1.0`.
    #[test]
    fn cos_zero_is_one() {
        assert_eq!(I128s12::ZERO.cos(), I128s12::ONE);
    }

    /// `tan(0) == 0` -- bit-exact via `f64::tan(0.0) == 0.0`.
    #[test]
    fn tan_zero_is_zero() {
        assert_eq!(I128s12::ZERO.tan(), I128s12::ZERO);
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
            let x = I128s12::from_bits(raw);
            let s = x.sin();
            let c = x.cos();
            let sum = (s * s) + (c * c);
            assert!(
                within_lsb(sum, I128s12::ONE, FOUR_LSB),
                "sin^2 + cos^2 != 1 for raw={raw}: got bits {} (delta {})",
                sum.to_bits(),
                (sum.to_bits() - I128s12::ONE.to_bits()).abs(),
            );
        }
    }

    // ── Inverse trig ──────────────────────────────────────────────────

    /// `asin(0) == 0` -- bit-exact.
    #[test]
    fn asin_zero_is_zero() {
        assert_eq!(I128s12::ZERO.asin(), I128s12::ZERO);
    }

    /// `acos(1) == 0` -- bit-exact via `f64::acos(1.0) == 0.0`.
    #[test]
    fn acos_one_is_zero() {
        assert_eq!(I128s12::ONE.acos(), I128s12::ZERO);
    }

    /// `acos(0) ~= pi/2` within 4 LSB.
    #[test]
    fn acos_zero_is_half_pi() {
        let result = I128s12::ZERO.acos();
        assert!(
            within_lsb(result, I128s12::half_pi(), FOUR_LSB),
            "acos(0) bits {}, half_pi bits {}",
            result.to_bits(),
            I128s12::half_pi().to_bits(),
        );
    }

    /// `atan(0) == 0` -- bit-exact via `f64::atan(0.0) == 0.0`.
    #[test]
    fn atan_zero_is_zero() {
        assert_eq!(I128s12::ZERO.atan(), I128s12::ZERO);
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
            let x = I128s12::from_bits(raw);
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
        let one = I128s12::ONE;
        let result = one.atan2(one);
        assert!(
            within_lsb(result, I128s12::quarter_pi(), TWO_LSB),
            "atan2(1, 1) bits {}, quarter_pi bits {}",
            result.to_bits(),
            I128s12::quarter_pi().to_bits(),
        );
    }

    /// `atan2(-1, -1) ~= -3*pi/4` (third-quadrant correctness).
    #[test]
    fn atan2_third_quadrant_diagonal() {
        let neg_one = -I128s12::ONE;
        let result = neg_one.atan2(neg_one);
        let three = I128s12::from_int(3);
        let expected = -(I128s12::quarter_pi() * three);
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
        let one = I128s12::ONE;
        let neg_one = -I128s12::ONE;
        let result = one.atan2(neg_one);
        let three = I128s12::from_int(3);
        let expected = I128s12::quarter_pi() * three;
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
        let one = I128s12::ONE;
        let neg_one = -I128s12::ONE;
        let result = neg_one.atan2(one);
        let expected = -I128s12::quarter_pi();
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
        let zero = I128s12::ZERO;
        let one = I128s12::ONE;
        assert_eq!(zero.atan2(one), I128s12::ZERO);
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    /// `sinh(0) == 0` -- bit-exact via `f64::sinh(0.0) == 0.0`.
    #[test]
    fn sinh_zero_is_zero() {
        assert_eq!(I128s12::ZERO.sinh(), I128s12::ZERO);
    }

    /// `cosh(0) == 1` -- bit-exact via `f64::cosh(0.0) == 1.0`.
    #[test]
    fn cosh_zero_is_one() {
        assert_eq!(I128s12::ZERO.cosh(), I128s12::ONE);
    }

    /// `tanh(0) == 0` -- bit-exact via `f64::tanh(0.0) == 0.0`.
    #[test]
    fn tanh_zero_is_zero() {
        assert_eq!(I128s12::ZERO.tanh(), I128s12::ZERO);
    }

    /// `asinh(0) == 0` -- bit-exact.
    #[test]
    fn asinh_zero_is_zero() {
        assert_eq!(I128s12::ZERO.asinh(), I128s12::ZERO);
    }

    /// `acosh(1) == 0` -- bit-exact via `f64::acosh(1.0) == 0.0`.
    #[test]
    fn acosh_one_is_zero() {
        assert_eq!(I128s12::ONE.acosh(), I128s12::ZERO);
    }

    /// `atanh(0) == 0` -- bit-exact.
    #[test]
    fn atanh_zero_is_zero() {
        assert_eq!(I128s12::ZERO.atanh(), I128s12::ZERO);
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
            let x = I128s12::from_bits(raw);
            let ch = x.cosh();
            let sh = x.sinh();
            let diff = (ch * ch) - (sh * sh);
            assert!(
                within_lsb(diff, I128s12::ONE, FOUR_LSB),
                "cosh^2 - sinh^2 != 1 for raw={raw}: got bits {} (delta {})",
                diff.to_bits(),
                (diff.to_bits() - I128s12::ONE.to_bits()).abs(),
            );
        }
    }

    // ── Angle conversions ─────────────────────────────────────────────

    /// `to_degrees(pi) ~= 180` within `ANGLE_TOLERANCE_LSB`. The
    /// tolerance is dominated by f64's limited precision on `pi`,
    /// amplified by ~57.3 during the degrees conversion.
    #[test]
    fn to_degrees_pi_is_180() {
        let pi = I128s12::pi();
        let result = pi.to_degrees();
        let expected = I128s12::from_int(180);
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
        let one_eighty = I128s12::from_int(180);
        let result = one_eighty.to_radians();
        let expected = I128s12::pi();
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
        assert_eq!(I128s12::ZERO.to_degrees(), I128s12::ZERO);
    }

    /// `to_radians(0) == 0` -- bit-exact.
    #[test]
    fn to_radians_zero_is_zero() {
        assert_eq!(I128s12::ZERO.to_radians(), I128s12::ZERO);
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
            let x = I128s12::from_bits(raw);
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
        let result = I128s12::half_pi().to_degrees();
        let expected = I128s12::from_int(90);
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
        let result = I128s12::quarter_pi().to_degrees();
        let expected = I128s12::from_int(45);
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
            let x = I128s12::from_bits(raw);
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
            let x = I128s12::from_bits(raw);
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
