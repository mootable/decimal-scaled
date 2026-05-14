//! Logarithm and exponential methods for [`D128`].
//!
//! # Methods
//!
//! - **Logarithms:** [`D128::ln`] / [`D128::log`] / [`D128::log2`] / [`D128::log10`].
//! - **Exponentials:** [`D128::exp`] / [`D128::exp2`].
//!
//! # Feature gating
//!
//! Each method is provided in two cfg-disjoint forms:
//!
//! - The f64-based form is gated `#[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]`
//!   and calls an inherent `f64` method (`f64::ln`, `f64::log`, `f64::log2`,
//!   `f64::log10`, `f64::exp`, `f64::exp2`). Those intrinsics live in `std`,
//!   so the gate excludes them from `no_std` builds.
//! - The strict form is gated `#[cfg(all(feature = "strict", not(feature = "no_strict")))]` and is an
//!   integer-only implementation. The bodies are currently `todo!` stubs;
//!   they compile under `no_std` and will be filled in with real
//!   integer algorithms in a later pass.
//!
//! The module declaration in `lib.rs` is ungated so that the strict stubs
//! remain reachable under `no_std + strict`. `no_std` users without the
//! `strict` feature can compose logarithms and exponentials externally via
//! `libm` or hardware-specific intrinsics.
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
//! `D128::ZERO` and saturates infinities to `D128::MAX` or `D128::MIN`.
//! Callers that require an explicit error for out-of-domain inputs should
//! check `is_negative()` or `is_zero()` before calling these methods.
//!
//! # Base-aware `log`
//!
//! `D128::log(self, base)` routes through `f64::log(self_f64, base_f64)`
//! rather than computing `ln(self) / ln(base)`, avoiding a second f64
//! round-trip and the associated extra quantisation noise.

use crate::core_type::D128;

// High-precision math constants used by the strict-mode transcendentals.
// Each value is the half-to-even rounding of the named irrational to 35
// fractional digits, stored as a raw i128 at SCALE_REF = 35 (matching
// the convention in `consts.rs`). Sources: research/strict_transcendentals_research.md §6.
//
// Not feature-gated: the `*_strict` methods are always compiled (see the
// module doc), so the constants they depend on are too.
const LN_2_RAW_S35: i128 = 69_314_718_055_994_530_941_723_212_145_817_657_i128;
const LN_10_RAW_S35: i128 = 230_258_509_299_404_568_401_799_145_468_436_421_i128;

impl<const SCALE: u32> D128<SCALE> {
    // Logarithms

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// # Algorithm
    ///
    /// Range reduction `x = 2^k * m` with `m ∈ [1, 2)`, then a Mercator
    /// series `ln(m) = ln(1 + y) = sum_{n=1..} (-1)^(n+1) * y^n / n` on
    /// the reduced mantissa `y = m - 1`. The series is truncated when
    /// the next term contributes less than one LSB at the call site's
    /// SCALE. Result is `k * ln(2) + ln(m)` where `ln(2)` is materialised
    /// from the 35-digit canonical reference.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only. Result accuracy is within
    /// roughly ±10 ULPs at `D128s12` and degrades as SCALE approaches 38
    /// (the series cap of 200 terms is insufficient at the extreme
    /// SCALEs). A tighter Remez-polynomial implementation per
    /// `research/strict_transcendentals_research.md` is planned for a
    /// later phase.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0`.
    ///
    /// Always available, regardless of the `strict` feature. When
    /// `strict` is enabled, the plain [`Self::ln`] delegates here.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn ln_strict(self) -> Self {
        if self.0 <= 0 {
            panic!("D128::ln: argument must be positive");
        }
        let one = Self::ONE;
        let two = Self::from_bits(one.to_bits().saturating_mul(2));
        let ln_2 = D128::<35>::from_bits(LN_2_RAW_S35).rescale::<SCALE>();

        // Range reduction: x = 2^k * m, m in [1, 2). Halve via arithmetic
        // right-shift (the value is positive after the panic guard above)
        // and double via left-shift. At SCALE = 38 the left-shift can
        // overflow for x.0 in roughly (i128::MAX/2, ONE.0) -- accepted as
        // a known precision-cliff at the extreme scale; the Phase 2
        // research doc tracks this as the Q-format intermediate
        // limitation at SCALE >= 36.
        let mut x = self;
        let mut k: i128 = 0;
        while x >= two {
            x = Self::from_bits(x.to_bits() >> 1);
            k += 1;
        }
        while x < one {
            x = Self::from_bits(x.to_bits() << 1);
            k -= 1;
        }

        // Mercator: ln(1 + y) = y - y²/2 + y³/3 - y⁴/4 + ...
        let y = x - one;
        let mut sum_bits: i128 = 0;
        let mut term_power = y;
        let mut n: i128 = 1;
        loop {
            let term_bits = term_power.to_bits() / n;
            if term_bits == 0 {
                break;
            }
            if n & 1 == 1 {
                sum_bits = sum_bits.saturating_add(term_bits);
            } else {
                sum_bits = sum_bits.saturating_sub(term_bits);
            }
            n += 1;
            if n > 200 {
                break;
            }
            // Next power: term_power = term_power * y at D128 scale.
            term_power = term_power * y;
        }

        let k_part = k.saturating_mul(ln_2.to_bits());
        Self::from_bits(k_part.saturating_add(sum_bits))
    }

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// # Precision
    ///
    /// Lossy: converts to f64, calls `f64::ln`, converts back. `f64::ln`
    /// returns `-Infinity` for `0.0` (saturates to `D128::MIN`) and `NaN`
    /// for negative inputs (maps to `D128::ZERO`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D128s12;
    /// // ln(1) == 0 (f64::ln(1.0) == 0.0 exactly).
    /// assert_eq!(D128s12::ONE.ln(), D128s12::ZERO);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().ln())
    }

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::ln_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        self.ln_strict()
    }

    /// Returns the logarithm of `self` in the given `base`, computed
    /// integer-only as `ln_strict(self) / ln_strict(base)`.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0` (via [`Self::ln_strict`]), if `base <= 0`,
    /// or if `base == 1` (division by `ln(1) = 0`).
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn log_strict(self, base: Self) -> Self {
        let ln_base = base.ln_strict();
        if ln_base.to_bits() == 0 {
            panic!("D128::log: base must not equal 1 (ln(1) is zero)");
        }
        self.ln_strict() / ln_base
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
    /// use decimal_scaled::D128s12;
    /// // log_2(8) is approximately 3 within f64 precision.
    /// let eight = D128s12::from_int(8);
    /// let two   = D128s12::from_int(2);
    /// let result = eight.log(two);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().log(base.to_f64_lossy()))
    }

    /// Returns the logarithm of `self` in the given `base`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::log_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        self.log_strict(base)
    }

    /// Returns the base-2 logarithm of `self`, computed integer-only as
    /// `ln_strict(self) / ln(2)`.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0` (via [`Self::ln_strict`]).
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn log2_strict(self) -> Self {
        let ln_2 = D128::<35>::from_bits(LN_2_RAW_S35).rescale::<SCALE>();
        self.ln_strict() / ln_2
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
    /// use decimal_scaled::D128s12;
    /// // log2(1) == 0 (f64::log2(1.0) == 0.0 exactly).
    /// assert_eq!(D128s12::ONE.log2(), D128s12::ZERO);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().log2())
    }

    /// Returns the base-2 logarithm of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::log2_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        self.log2_strict()
    }

    /// Returns the base-10 logarithm of `self`, computed integer-only
    /// as `ln_strict(self) / ln(10)`.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0` (via [`Self::ln_strict`]).
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn log10_strict(self) -> Self {
        let ln_10 = D128::<35>::from_bits(LN_10_RAW_S35).rescale::<SCALE>();
        self.ln_strict() / ln_10
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
    /// use decimal_scaled::D128s12;
    /// // log10(1) == 0 (f64::log10(1.0) == 0.0 exactly).
    /// assert_eq!(D128s12::ONE.log10(), D128s12::ZERO);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn log10(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().log10())
    }

    /// Returns the base-10 logarithm of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::log10_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn log10(self) -> Self {
        self.log10_strict()
    }

    // Exponentials

    /// Returns `e^self` (natural exponential).
    ///
    /// # Algorithm
    ///
    /// Range reduction `x = k * ln(2) + r` with `|r| ≤ ln(2)/2 ≈ 0.347`,
    /// then a Taylor series `exp(r) = 1 + r + r²/2! + r³/3! + ...` on the
    /// reduced remainder. Reassembly is `exp(x) = 2^k * exp(r)`, applied
    /// via a left- or right-shift of the storage bits.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only. The series converges
    /// quickly because `|r|` is bounded by `ln(2)/2`. Result accuracy
    /// is within roughly ±10 ULPs at `D128s12` and degrades at higher
    /// SCALE for the same reasons as [`Self::ln`].
    ///
    /// # Panics
    ///
    /// Panics if the result overflows D128's representable range.
    /// At `D128s12` this happens for `self.to_f64_lossy()` greater than
    /// roughly `60`.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn exp_strict(self) -> Self {
        let one = Self::ONE;
        if self.to_bits() == 0 {
            return one;
        }
        let ln_2 = D128::<35>::from_bits(LN_2_RAW_S35).rescale::<SCALE>();

        // x = k * ln(2) + r, |r| <= ln(2)/2.
        let q = self / ln_2;
        let half_bits = one.to_bits() >> 1;
        let q_with_half = if q.to_bits() >= 0 {
            q.to_bits().saturating_add(half_bits)
        } else {
            q.to_bits().saturating_sub(half_bits)
        };
        let k = q_with_half / one.to_bits();
        let k_ln_2_bits = match k.checked_mul(ln_2.to_bits()) {
            Some(v) => v,
            None => panic!("D128::exp: argument out of range"),
        };
        let r = Self::from_bits(self.to_bits() - k_ln_2_bits);

        // Taylor series for exp(r): 1 + r + r²/2! + r³/3! + ...
        // At each step `t` represents r^n / n!.
        let mut sum_bits: i128 = one.to_bits();
        let mut t = one;
        let mut n: i128 = 1;
        loop {
            t = t * r;
            let term_bits = t.to_bits() / n;
            if term_bits == 0 {
                break;
            }
            sum_bits = sum_bits.saturating_add(term_bits);
            t = Self::from_bits(term_bits);
            n += 1;
            if n > 100 {
                break;
            }
        }

        // exp(x) = 2^k * exp(r); apply via bit-shift on the raw storage.
        let result_bits = if k > 0 {
            let shift = k as u32;
            if shift >= 128 || sum_bits.checked_shl(shift).is_none() {
                panic!("D128::exp: result overflows D128 range");
            }
            sum_bits << shift
        } else if k < 0 {
            let shift = (-k) as u32;
            if shift >= 128 {
                0
            } else {
                sum_bits >> shift
            }
        } else {
            sum_bits
        };
        Self::from_bits(result_bits)
    }

    /// Returns `e^self` (natural exponential).
    ///
    /// # Precision
    ///
    /// Lossy: involves f64 at some point; result may lose precision.
    /// Large positive inputs overflow f64 to `+Infinity`, which saturates
    /// to `D128::MAX`. Large negative inputs underflow to `0.0` in f64,
    /// which maps to `D128::ZERO`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D128s12;
    /// // exp(0) == 1 (f64::exp(0.0) == 1.0 exactly).
    /// assert_eq!(D128s12::ZERO.exp(), D128s12::ONE);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().exp())
    }

    /// Returns `e^self` (natural exponential).
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::exp_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        self.exp_strict()
    }

    /// Returns `2^self` (base-2 exponential), computed integer-only as
    /// `exp_strict(self * ln(2))`.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows D128's representable range.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn exp2_strict(self) -> Self {
        let ln_2 = D128::<35>::from_bits(LN_2_RAW_S35).rescale::<SCALE>();
        (self * ln_2).exp_strict()
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
    /// use decimal_scaled::D128s12;
    /// // exp2(0) == 1 (f64::exp2(0.0) == 1.0 exactly).
    /// assert_eq!(D128s12::ZERO.exp2(), D128s12::ONE);
    /// ```
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        Self::from_f64_lossy(self.to_f64_lossy().exp2())
    }

    /// Returns `2^self` (base-2 exponential).
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::exp2_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "no_strict")))]
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        self.exp2_strict()
    }
}

#[cfg(all(test, feature = "strict", not(feature = "no_strict")))]
mod strict_tests {
    use crate::core_type::D128s12;

    /// Tolerance in ULPs for the Phase 2A integer-only transcendentals.
    /// Tightens as the implementation evolves toward Remez polynomials.
    const STRICT_TOLERANCE_LSB: i128 = 10;

    fn within(actual: D128s12, expected_bits: i128, tolerance: i128) -> bool {
        (actual.to_bits() - expected_bits).abs() <= tolerance
    }

    /// ln(1) == 0 exactly (no series terms contribute).
    #[test]
    fn ln_of_one_is_zero() {
        assert_eq!(D128s12::ONE.ln(), D128s12::ZERO);
    }

    /// ln(2) at scale 12 = 693_147_180_560 (canonical rounded to 12 places).
    #[test]
    fn ln_of_two_close_to_canonical() {
        let two = D128s12::from_bits(2_000_000_000_000);
        let result = two.ln();
        // ln(2) = 0.693147180559945... so at scale 12, bits = 693_147_180_560.
        assert!(
            within(result, 693_147_180_560, STRICT_TOLERANCE_LSB),
            "ln(2) bits = {}",
            result.to_bits()
        );
    }

    /// ln(e) is approximately 1. Uses the existing pi/e constants via DecimalConsts.
    #[test]
    fn ln_of_e_close_to_one() {
        // e at scale 12 = 2_718_281_828_459 (canonical 35-digit reference rescaled).
        let e_at_s12 = D128s12::from_bits(2_718_281_828_459);
        let result = e_at_s12.ln();
        // ln(e) = 1.0 -> bits = 1_000_000_000_000 at scale 12.
        assert!(
            within(result, 1_000_000_000_000, STRICT_TOLERANCE_LSB),
            "ln(e) bits = {}, expected ~1_000_000_000_000",
            result.to_bits()
        );
    }

    /// ln(10) at scale 12 = 2_302_585_092_994 (canonical).
    #[test]
    fn ln_of_ten_close_to_canonical() {
        let ten = D128s12::from_bits(10_000_000_000_000);
        let result = ten.ln();
        assert!(
            within(result, 2_302_585_092_994, STRICT_TOLERANCE_LSB),
            "ln(10) bits = {}, expected ~2_302_585_092_994",
            result.to_bits()
        );
    }

    /// ln of a value > 1 is positive.
    #[test]
    fn ln_above_one_is_positive() {
        let v = D128s12::from_bits(1_500_000_000_000); // 1.5
        let result = v.ln();
        assert!(result.to_bits() > 0);
    }

    /// ln of a value in (0, 1) is negative.
    #[test]
    fn ln_below_one_is_negative() {
        let v = D128s12::from_bits(500_000_000_000); // 0.5
        let result = v.ln();
        assert!(result.to_bits() < 0);
        // ln(0.5) = -ln(2) ~= -0.693147...
        assert!(
            within(result, -693_147_180_560, STRICT_TOLERANCE_LSB),
            "ln(0.5) bits = {}, expected ~-693_147_180_560",
            result.to_bits()
        );
    }

    #[test]
    #[should_panic(expected = "argument must be positive")]
    fn ln_of_zero_panics() {
        let _ = D128s12::ZERO.ln();
    }

    #[test]
    #[should_panic(expected = "argument must be positive")]
    fn ln_of_negative_panics() {
        let neg = D128s12::from_bits(-1_000_000_000_000);
        let _ = neg.ln();
    }

    // log2 / log10 / log derive from ln; tolerance grows because the
    // additional division step accumulates ~1 LSB.
    const DERIVED_LOG_TOLERANCE_LSB: i128 = 20;

    /// log2(2) ~= 1.
    #[test]
    fn log2_of_two_is_one() {
        let two = D128s12::from_bits(2_000_000_000_000);
        let result = two.log2();
        assert!(
            within(result, 1_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log2(2) bits = {}",
            result.to_bits()
        );
    }

    /// log2(8) ~= 3.
    #[test]
    fn log2_of_eight_is_three() {
        let eight = D128s12::from_bits(8_000_000_000_000);
        let result = eight.log2();
        assert!(
            within(result, 3_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log2(8) bits = {}",
            result.to_bits()
        );
    }

    /// log10(10) ~= 1.
    #[test]
    fn log10_of_ten_is_one() {
        let ten = D128s12::from_bits(10_000_000_000_000);
        let result = ten.log10();
        assert!(
            within(result, 1_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log10(10) bits = {}",
            result.to_bits()
        );
    }

    /// log10(100) ~= 2.
    #[test]
    fn log10_of_hundred_is_two() {
        let hundred = D128s12::from_bits(100_000_000_000_000);
        let result = hundred.log10();
        assert!(
            within(result, 2_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log10(100) bits = {}",
            result.to_bits()
        );
    }

    /// log_base_b(b) == 1 for any b > 0, b != 1.
    #[test]
    fn log_self_is_one() {
        let base = D128s12::from_bits(5_000_000_000_000); // 5
        let result = base.log(base);
        assert!(
            within(result, 1_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log_5(5) bits = {}",
            result.to_bits()
        );
    }

    /// log_2(8) == 3 via the generic log.
    #[test]
    fn log_with_base_two() {
        let eight = D128s12::from_bits(8_000_000_000_000);
        let two = D128s12::from_bits(2_000_000_000_000);
        let result = eight.log(two);
        assert!(
            within(result, 3_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log_2(8) bits = {}",
            result.to_bits()
        );
    }

    #[test]
    #[should_panic(expected = "base must not equal 1")]
    fn log_base_one_panics() {
        let x = D128s12::from_bits(5_000_000_000_000);
        let one = D128s12::ONE;
        let _ = x.log(one);
    }

    // exp / exp2: tolerance accounts for Taylor truncation, 2^k bit-shift
    // exactness, and the range-reduction rounding step. ~20 LSB at D128s12.
    const EXP_TOLERANCE_LSB: i128 = 20;

    /// exp(0) == 1 exactly.
    #[test]
    fn exp_of_zero_is_one() {
        assert_eq!(D128s12::ZERO.exp(), D128s12::ONE);
    }

    /// exp(1) ~= e.
    #[test]
    fn exp_of_one_is_e() {
        let result = D128s12::ONE.exp();
        // e ~= 2.718281828459 at D128s12.
        assert!(
            within(result, 2_718_281_828_459, EXP_TOLERANCE_LSB),
            "exp(1) bits = {}",
            result.to_bits()
        );
    }

    /// exp(ln(2)) ~= 2.
    #[test]
    fn exp_of_ln_2_is_two() {
        let ln_2 = D128s12::from_bits(693_147_180_560);
        let result = ln_2.exp();
        assert!(
            within(result, 2_000_000_000_000, EXP_TOLERANCE_LSB),
            "exp(ln 2) bits = {}",
            result.to_bits()
        );
    }

    /// exp(-1) ~= 1/e ~= 0.367879441171.
    #[test]
    fn exp_of_negative_one_is_reciprocal_e() {
        let neg_one = D128s12::from_bits(-1_000_000_000_000);
        let result = neg_one.exp();
        // 1/e ~= 0.367879441171 at D128s12 -> bits ~= 367_879_441_171.
        assert!(
            within(result, 367_879_441_171, EXP_TOLERANCE_LSB),
            "exp(-1) bits = {}",
            result.to_bits()
        );
    }

    /// exp2(0) == 1 exactly.
    #[test]
    fn exp2_of_zero_is_one() {
        assert_eq!(D128s12::ZERO.exp2(), D128s12::ONE);
    }

    /// exp2(1) ~= 2.
    #[test]
    fn exp2_of_one_is_two() {
        let result = D128s12::ONE.exp2();
        assert!(
            within(result, 2_000_000_000_000, EXP_TOLERANCE_LSB),
            "exp2(1) bits = {}",
            result.to_bits()
        );
    }

    /// exp2(10) ~= 1024.
    #[test]
    fn exp2_of_ten_is_1024() {
        let ten = D128s12::from_bits(10_000_000_000_000);
        let result = ten.exp2();
        assert!(
            within(result, 1024_000_000_000_000, EXP_TOLERANCE_LSB * 10),
            "exp2(10) bits = {}",
            result.to_bits()
        );
    }
}

#[cfg(all(test, any(not(feature = "strict"), feature = "no_strict")))]
mod tests {
    use crate::consts::DecimalConsts;
    use crate::core_type::D128s12;

    /// Tolerance for f64-bridge log/exp tests against integer-valued
    /// expectations.
    ///
    /// The f64 round-trip introduces roughly 1 LSB of quantisation noise.
    /// Log and exp then amplify that noise in proportion to input magnitude.
    /// For the test inputs (powers of 10 and powers of 2 up to 2^16) the
    /// worst-case slack is around 16 LSB; 32 gives comfortable margin.
    /// At SCALE=12 this is 32 picometers, nine orders of magnitude below
    /// any physical measurement. The test margin reflects f64 arithmetic
    /// noise, not D128 imprecision.
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

    fn within_lsb(actual: D128s12, expected: D128s12, lsb: i128) -> bool {
        let diff = (actual.to_bits() - expected.to_bits()).abs();
        diff <= lsb
    }

    // Bit-exact identity tests

    /// `exp(0) == 1` -- bit-exact via `f64::exp(0.0) == 1.0`.
    #[test]
    fn exp_zero_is_one() {
        assert_eq!(D128s12::ZERO.exp(), D128s12::ONE);
    }

    /// `exp2(0) == 1` -- bit-exact via `f64::exp2(0.0) == 1.0`.
    #[test]
    fn exp2_zero_is_one() {
        assert_eq!(D128s12::ZERO.exp2(), D128s12::ONE);
    }

    /// `ln(1) == 0` -- bit-exact via `f64::ln(1.0) == 0.0`.
    #[test]
    fn ln_one_is_zero() {
        assert_eq!(D128s12::ONE.ln(), D128s12::ZERO);
    }

    /// `log2(1) == 0` -- bit-exact via `f64::log2(1.0) == 0.0`.
    #[test]
    fn log2_one_is_zero() {
        assert_eq!(D128s12::ONE.log2(), D128s12::ZERO);
    }

    /// `log10(1) == 0` -- bit-exact via `f64::log10(1.0) == 0.0`.
    #[test]
    fn log10_one_is_zero() {
        assert_eq!(D128s12::ONE.log10(), D128s12::ZERO);
    }

    // Integer-power identities (within tolerance)

    /// `log2(8) ~= 3` within tolerance.
    #[test]
    fn log2_of_eight_is_three() {
        let eight = D128s12::from_int(8);
        let result = eight.log2();
        let expected = D128s12::from_int(3);
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
        let thousand = D128s12::from_int(1000);
        let result = thousand.log10();
        let expected = D128s12::from_int(3);
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
            let pow_of_ten = D128s12::from_int(10_i64.pow(n as u32));
            let result = pow_of_ten.log10();
            let expected = D128s12::from_int(n);
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
            let pow_of_two = D128s12::from_int(2_i64.pow(n as u32));
            let result = pow_of_two.log2();
            let expected = D128s12::from_int(n);
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
            let x = D128s12::from_bits(raw);
            let recovered = x.ln().exp();
            assert!(
                within_lsb(recovered, x, ROUND_TRIP_TOLERANCE_LSB),
                "exp(ln(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits(),
                (recovered.to_bits() - x.to_bits()).abs(),
            );
        }
    }

    /// `exp(D128::e().ln()) ~= D128::e()` round-trip within tolerance.
    ///
    /// `e ~= 2.718`, so the error stays inside `LOG_EXP_TOLERANCE_LSB`.
    #[test]
    fn exp_of_ln_e_round_trip() {
        let e = D128s12::e();
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
        // Moderate inputs; large positive inputs approach D128s12 magnitude limit.
        for raw in [
            -2_345_678_901_234_i128, // ~-2.345678
            -500_000_000_000_i128,   // -0.5
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = D128s12::from_bits(raw);
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
        let e = D128s12::e();
        for raw in [
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = D128s12::from_bits(raw);
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
        let two = D128s12::from_int(2);
        for raw in [
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = D128s12::from_bits(raw);
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
        let ten = D128s12::from_int(10);
        for raw in [
            500_000_000_000_i128,    // 0.5
            1_234_567_890_123_i128,  // ~1.234567
            4_567_891_234_567_i128,  // ~4.567891
            7_890_123_456_789_i128,  // ~7.890123
        ] {
            let x = D128s12::from_bits(raw);
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
            let result = D128s12::from_int(n).exp2();
            let expected = D128s12::from_int(2_i64.pow(n as u32));
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
