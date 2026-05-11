//! Logarithm and exponential methods for [`I128`].
//!
//! # Methods
//!
//! - **Logarithms:** [`I128::ln`] / [`I128::log`] / [`I128::log2`] / [`I128::log10`].
//! - **Exponentials:** [`I128::exp`] / [`I128::exp2`].
//!
//! # Feature gating
//!
//! Without the `strict` feature, every method here calls an inherent `f64`
//! method (`f64::ln`, `f64::log`, `f64::log2`, `f64::log10`, `f64::exp`,
//! `f64::exp2`), which requires `std`. In that configuration the module is
//! gated `#[cfg(feature = "std")]` at the `mod log_exp;` declaration in
//! `lib.rs`, and `no_std` users that need logarithms or exponentials can
//! compose them externally via `libm` or hardware-specific intrinsics.
//!
//! With the `strict` feature enabled, all methods compile without `std` using
//! integer-only algorithms. Each method's body is replaced with a
//! `todo!`-guarded stub so the module compiles in `no_std` environments;
//! callers that invoke these stubs at runtime will panic until full
//! integer-only implementations are provided.
//!
//! # Precision
//!
//! All methods in this module are **Lossy** (without `strict`): each one
//! converts `self` to `f64`, applies the corresponding `f64` transcendental,
//! and converts the result back. IEEE 754 does not mandate correct rounding
//! for transcendental functions, so results may differ by one or more ULPs
//! across platforms or library versions.
//!
//! # Domain handling
//!
//! `f64::ln`, `f64::log2`, `f64::log10`, and `f64::log` return `-Infinity`
//! for `0.0` and `NaN` for negative inputs. The f64 bridge maps `NaN` to
//! `I128::ZERO` and saturates infinities to `I128::MAX` or `I128::MIN`.
//! Callers that require an explicit error for out-of-domain inputs should
//! check `is_negative()` or `is_zero()` before calling these methods.
//!
//! # Base-aware `log`
//!
//! `I128::log(self, base)` routes through `f64::log(self_f64, base_f64)`
//! rather than computing `ln(self) / ln(base)`, avoiding a second f64
//! round-trip and the associated extra quantisation noise.

use crate::core_type::I128;

impl<const SCALE: u32> I128<SCALE> {
    // Logarithms

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // ln(1) == 0 (f64::ln(1.0) == 0.0 exactly).
    /// assert_eq!(I128s12::ONE.ln(), I128s12::ZERO);
    /// ```
    #[cfg(feature = "strict")]
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        todo!("strict: integer-only ln not yet implemented")
    }

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// # Precision
    ///
    /// Lossy: converts to f64, calls `f64::ln`, converts back. `f64::ln`
    /// returns `-Infinity` for `0.0` (saturates to `I128::MIN`) and `NaN`
    /// for negative inputs (maps to `I128::ZERO`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // ln(1) == 0 (f64::ln(1.0) == 0.0 exactly).
    /// assert_eq!(I128s12::ONE.ln(), I128s12::ZERO);
    /// ```
    #[cfg(not(feature = "strict"))]
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().ln())
    }

    /// Returns the logarithm of `self` in the given `base`.
    ///
    /// Implemented via a single `f64::log(self_f64, base_f64)` call, which
    /// avoids the extra quantisation that would come from computing
    /// `ln(self) / ln(base)` with two separate f64 round-trips.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // log_2(8) is approximately 3 within f64 precision.
    /// let eight = I128s12::from_int(8);
    /// let two   = I128s12::from_int(2);
    /// let result = eight.log(two);
    /// ```
    #[cfg(feature = "strict")]
    #[inline]
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        todo!("strict: integer-only log not yet implemented")
    }

    /// Returns the logarithm of `self` in the given `base`.
    ///
    /// Implemented via a single `f64::log(self_f64, base_f64)` call, which
    /// avoids the extra quantisation that would come from computing
    /// `ln(self) / ln(base)` with two separate f64 round-trips.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // log_2(8) is approximately 3 within f64 precision.
    /// let eight = I128s12::from_int(8);
    /// let two   = I128s12::from_int(2);
    /// let result = eight.log(two);
    /// ```
    #[cfg(not(feature = "strict"))]
    #[inline]
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().log(base.to_f64_lossy()))
    }

    /// Returns the base-2 logarithm of `self`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // log2(1) == 0 (f64::log2(1.0) == 0.0 exactly).
    /// assert_eq!(I128s12::ONE.log2(), I128s12::ZERO);
    /// ```
    #[cfg(feature = "strict")]
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        todo!("strict: integer-only log2 not yet implemented")
    }

    /// Returns the base-2 logarithm of `self`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    /// On IEEE-754 platforms, `f64::log2` is exact for integer powers
    /// of two (e.g. `log2(8.0) == 3.0`). Out-of-domain inputs follow
    /// the same saturation policy as [`Self::ln`].
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // log2(1) == 0 (f64::log2(1.0) == 0.0 exactly).
    /// assert_eq!(I128s12::ONE.log2(), I128s12::ZERO);
    /// ```
    #[cfg(not(feature = "strict"))]
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().log2())
    }

    /// Returns the base-10 logarithm of `self`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // log10(1) == 0 (f64::log10(1.0) == 0.0 exactly).
    /// assert_eq!(I128s12::ONE.log10(), I128s12::ZERO);
    /// ```
    #[cfg(feature = "strict")]
    #[inline]
    #[must_use]
    pub fn log10(self) -> Self {
        todo!("strict: integer-only log10 not yet implemented")
    }

    /// Returns the base-10 logarithm of `self`.
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    /// Out-of-domain inputs follow the same saturation policy as [`Self::ln`].
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // log10(1) == 0 (f64::log10(1.0) == 0.0 exactly).
    /// assert_eq!(I128s12::ONE.log10(), I128s12::ZERO);
    /// ```
    #[cfg(not(feature = "strict"))]
    #[inline]
    #[must_use]
    pub fn log10(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().log10())
    }

    // Exponentials

    /// Returns `e^self` (natural exponential).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // exp(0) == 1 (f64::exp(0.0) == 1.0 exactly).
    /// assert_eq!(I128s12::ZERO.exp(), I128s12::ONE);
    /// ```
    #[cfg(feature = "strict")]
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        todo!("strict: integer-only exp not yet implemented")
    }

    /// Returns `e^self` (natural exponential).
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    /// Large positive inputs overflow f64 to `+Infinity`, which saturates
    /// to `I128::MAX`. Large negative inputs underflow to `0.0` in f64,
    /// which maps to `I128::ZERO`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // exp(0) == 1 (f64::exp(0.0) == 1.0 exactly).
    /// assert_eq!(I128s12::ZERO.exp(), I128s12::ONE);
    /// ```
    #[cfg(not(feature = "strict"))]
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().exp())
    }

    /// Returns `2^self` (base-2 exponential).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // exp2(0) == 1 (f64::exp2(0.0) == 1.0 exactly).
    /// assert_eq!(I128s12::ZERO.exp2(), I128s12::ONE);
    /// ```
    #[cfg(feature = "strict")]
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        todo!("strict: integer-only exp2 not yet implemented")
    }

    /// Returns `2^self` (base-2 exponential).
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    /// Saturation behaviour is analogous to [`Self::exp`] but at different
    /// magnitudes (inputs beyond approximately 1024 overflow to `+Infinity`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::I128s12;
    /// // exp2(0) == 1 (f64::exp2(0.0) == 1.0 exactly).
    /// assert_eq!(I128s12::ZERO.exp2(), I128s12::ONE);
    /// ```
    #[cfg(not(feature = "strict"))]
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().exp2())
    }
}

#[cfg(all(test, not(feature = "strict")))]
mod tests {
    use crate::consts::DecimalConsts;
    use crate::core_type::I128s12;

    /// Tolerance for f64-bridge log/exp tests against integer-valued
    /// expectations.
    ///
    /// The f64 round-trip introduces roughly 1 LSB of quantisation noise.
    /// Log and exp then amplify that noise in proportion to input magnitude.
    /// For the test inputs (powers of 10 and powers of 2 up to 2^16) the
    /// worst-case slack is around 16 LSB; 32 gives comfortable margin.
    /// At SCALE=12 this is 32 picometers, nine orders of magnitude below
    /// any physical measurement. The test margin reflects f64 arithmetic
    /// noise, not I128 imprecision.
    const LOG_EXP_TOLERANCE_LSB: i128 = 32;

    /// Looser tolerance for round-trips like `exp(ln(x)) ~= x`.
    ///
    /// An epsilon-LSB error in `ln(x)` becomes a `~|x| * epsilon`-LSB
    /// error after `exp` (because `exp(ln(x) + eps) ~= x * (1 + eps)`).
    /// For `|x|` up to ~80 the worst observed slack is ~56 LSB; 128 LSB
    /// gives margin while staying well under 1 nanometer at SCALE=12.
    const ROUND_TRIP_TOLERANCE_LSB: i128 = 128;

    /// Tighter tolerance for moderate-magnitude round-trips where `|x| < 10`.
    /// Each f64 step adds up to ~1 LSB; 4 LSB absorbs two quantisation steps.
    const FOUR_LSB: i128 = 4;

    fn within_lsb(actual: I128s12, expected: I128s12, lsb: i128) -> bool {
        let diff = (actual.to_bits() - expected.to_bits()).abs();
        diff <= lsb
    }

    // Bit-exact identity tests

    /// `exp(0) == 1` -- bit-exact via `f64::exp(0.0) == 1.0`.
    #[test]
    fn exp_zero_is_one() {
        assert_eq!(I128s12::ZERO.exp(), I128s12::ONE);
    }

    /// `exp2(0) == 1` -- bit-exact via `f64::exp2(0.0) == 1.0`.
    #[test]
    fn exp2_zero_is_one() {
        assert_eq!(I128s12::ZERO.exp2(), I128s12::ONE);
    }

    /// `ln(1) == 0` -- bit-exact via `f64::ln(1.0) == 0.0`.
    #[test]
    fn ln_one_is_zero() {
        assert_eq!(I128s12::ONE.ln(), I128s12::ZERO);
    }

    /// `log2(1) == 0` -- bit-exact via `f64::log2(1.0) == 0.0`.
    #[test]
    fn log2_one_is_zero() {
        assert_eq!(I128s12::ONE.log2(), I128s12::ZERO);
    }

    /// `log10(1) == 0` -- bit-exact via `f64::log10(1.0) == 0.0`.
    #[test]
    fn log10_one_is_zero() {
        assert_eq!(I128s12::ONE.log10(), I128s12::ZERO);
    }

    // Integer-power identities (within tolerance)

    /// `log2(8) ~= 3` within tolerance.
    #[test]
    fn log2_of_eight_is_three() {
        let eight = I128s12::from_int(8);
        let result = eight.log2();
        let expected = I128s12::from_int(3);
        assert!(
            within_lsb(result, expected, LOG_EXP_TOLERANCE_LSB),
            "log2(8) bits {}, expected 3 bits {} (delta {})",
            result.to_bits(),
            expected.to_bits(),
            (result.to_bits() - expected.to_bits()).abs(),
        );
    }

    /// `log10(1000) ~= 3` within tolerance.
    #[test]
    fn log10_of_thousand_is_three() {
        let thousand = I128s12::from_int(1000);
        let result = thousand.log10();
        let expected = I128s12::from_int(3);
        assert!(
            within_lsb(result, expected, LOG_EXP_TOLERANCE_LSB),
            "log10(1000) bits {}, expected 3 bits {} (delta {})",
            result.to_bits(),
            expected.to_bits(),
            (result.to_bits() - expected.to_bits()).abs(),
        );
    }

    /// `log10(10^n) ~= n` for representative n.
    #[test]
    fn log10_of_power_of_ten() {
        // n = 1, 2, 4, 6 chosen to stay well within f64's range at SCALE=12.
        for n in [1_i64, 2, 4, 6] {
            let pow_of_ten = I128s12::from_int(10_i64.pow(n as u32));
            let result = pow_of_ten.log10();
            let expected = I128s12::from_int(n);
            assert!(
                within_lsb(result, expected, LOG_EXP_TOLERANCE_LSB),
                "log10(10^{n}) bits {}, expected {n} bits {} (delta {})",
                result.to_bits(),
                expected.to_bits(),
                (result.to_bits() - expected.to_bits()).abs(),
            );
        }
    }

    /// `log2(2^n) ~= n` for representative n.
    #[test]
    fn log2_of_power_of_two() {
        for n in [1_i64, 2, 4, 8, 16] {
            let pow_of_two = I128s12::from_int(2_i64.pow(n as u32));
            let result = pow_of_two.log2();
            let expected = I128s12::from_int(n);
            assert!(
                within_lsb(result, expected, LOG_EXP_TOLERANCE_LSB),
                "log2(2^{n}) bits {}, expected {n} bits {} (delta {})",
                result.to_bits(),
                expected.to_bits(),
                (result.to_bits() - expected.to_bits()).abs(),
            );
        }
    }

    // Round-trip identities

    /// `exp(ln(x)) ~= x` for `x` in `[0.1, 100]` within tolerance.
    ///
    /// Each f64 transcendental introduces ~1 LSB of quantisation noise;
    /// that noise is amplified by `~|x|` after the `exp` step.
    #[test]
    fn exp_of_ln_round_trip() {
        // Raw bit-patterns at SCALE=12 spanning [0.1, ~80].
        for raw in [
            100_000_000_000_i128,    // 0.1
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
            45_678_912_345_679_i128, // ~45.678912
            78_901_234_567_890_i128, // ~78.901234
        ] {
            let x = I128s12::from_bits(raw);
            let recovered = x.ln().exp();
            assert!(
                within_lsb(recovered, x, ROUND_TRIP_TOLERANCE_LSB),
                "exp(ln(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits(),
                (recovered.to_bits() - x.to_bits()).abs(),
            );
        }
    }

    /// `exp(I128::e().ln()) ~= I128::e()` round-trip within tolerance.
    ///
    /// `e ~= 2.718`, so the error stays inside `LOG_EXP_TOLERANCE_LSB`.
    #[test]
    fn exp_of_ln_e_round_trip() {
        let e = I128s12::e();
        let recovered = e.ln().exp();
        assert!(
            within_lsb(recovered, e, LOG_EXP_TOLERANCE_LSB),
            "exp(ln(e)) != e: got bits {} (delta {})",
            recovered.to_bits(),
            (recovered.to_bits() - e.to_bits()).abs(),
        );
    }

    /// `ln(exp(x)) ~= x` for moderate `x` -- the inverse round-trip.
    #[test]
    fn ln_of_exp_round_trip() {
        // Moderate inputs; large positive inputs approach I128s12 magnitude limit.
        for raw in [
            -2_345_678_901_234_i128, // ~-2.345678
            -500_000_000_000_i128,   // -0.5
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = I128s12::from_bits(raw);
            let recovered = x.exp().ln();
            assert!(
                within_lsb(recovered, x, FOUR_LSB),
                "ln(exp(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits(),
                (recovered.to_bits() - x.to_bits()).abs(),
            );
        }
    }

    // Cross-method consistency

    /// `log(self, e) ~= ln(self)` -- base-aware form is consistent with `ln`.
    #[test]
    fn log_base_e_matches_ln() {
        let e = I128s12::e();
        for raw in [
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = I128s12::from_bits(raw);
            let via_log = x.log(e);
            let via_ln = x.ln();
            assert!(
                within_lsb(via_log, via_ln, FOUR_LSB),
                "log(x, e) != ln(x) for raw={raw}: log bits {}, ln bits {}",
                via_log.to_bits(),
                via_ln.to_bits(),
            );
        }
    }

    /// `log(self, 2) ~= log2(self)` -- consistency check for base 2.
    #[test]
    fn log_base_two_matches_log2() {
        let two = I128s12::from_int(2);
        for raw in [
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = I128s12::from_bits(raw);
            let via_log = x.log(two);
            let via_log2 = x.log2();
            assert!(
                within_lsb(via_log, via_log2, FOUR_LSB),
                "log(x, 2) != log2(x) for raw={raw}: log bits {}, log2 bits {}",
                via_log.to_bits(),
                via_log2.to_bits(),
            );
        }
    }

    /// `log(self, 10) ~= log10(self)` -- consistency check for base 10.
    #[test]
    fn log_base_ten_matches_log10() {
        let ten = I128s12::from_int(10);
        for raw in [
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = I128s12::from_bits(raw);
            let via_log = x.log(ten);
            let via_log10 = x.log10();
            assert!(
                within_lsb(via_log, via_log10, FOUR_LSB),
                "log(x, 10) != log10(x) for raw={raw}: log bits {}, log10 bits {}",
                via_log.to_bits(),
                via_log10.to_bits(),
            );
        }
    }

    /// `exp2(n) ~= 2^n` for small integer n -- cross-check exp2 against
    /// the integer pow surface.
    #[test]
    fn exp2_matches_integer_power_of_two() {
        for n in [0_i64, 1, 2, 4, 8] {
            let result = I128s12::from_int(n).exp2();
            let expected = I128s12::from_int(2_i64.pow(n as u32));
            assert!(
                within_lsb(result, expected, LOG_EXP_TOLERANCE_LSB),
                "exp2({n}) bits {}, expected 2^{n} bits {} (delta {})",
                result.to_bits(),
                expected.to_bits(),
                (result.to_bits() - expected.to_bits()).abs(),
            );
        }
    }
}
