// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Logarithm and exponential methods for [`D38`].
//!
//! # Methods
//!
//! - **Logarithms:** [`D38::ln`] / [`D38::log`] / [`D38::log2`] / [`D38::log10`].
//! - **Exponentials:** [`D38::exp`] / [`D38::exp2`].
//!
//! # The four-variant matrix
//!
//! Each function ships two entry points so a single name covers
//! either rounding-mode choice:
//!
//! | Method            | Guard width    | Rounding mode               |
//! |-------------------|----------------|------------------------------|
//! | `<fn>_strict`     | crate default  | crate default               |
//! | `<fn>_strict_with`| crate default  | caller-supplied              |
//!
//! `_strict` runs at `SCALE + STRICT_GUARD` (const-folded so LLVM
//! specialises one optimal kernel per `SCALE`).
//!
//! `ln` uses range reduction plus a Mercator series;
//! `exp` uses range reduction plus a Taylor series; the
//! remaining methods compose those two. Both variants are
//! integer-only, `no_std`-compatible, and correctly rounded under
//! the selected mode.
//!
//! Without the `strict` feature, the plain `<fn>` is an f64-bridge
//! (calls the inherent `f64` intrinsic, gated on `std`). With
//! `strict` it dispatches to `<fn>_strict`. See `docs/strict-mode.md`
//! for the full dual-API and feature rules.
//!
//! # Layering
//!
//! Every public method on this file is a one-line delegate into
//! `policy::ln` or `policy::exp`. The
//! correctly-rounded kernels (`ln_fixed`, `exp_fixed`,
//! `STRICT_GUARD`, the `wide_ln2` / `wide_ln10` constants, and the
//! per-variant `ln` / `log` /
//! `log2_*` / `log10_*` / `exp` / `exp2_*`
//! `Fixed`-shape functions) live in
//! [`crate::algos::ln::ln_series_2limb`] and
//! [`crate::algos::exp::exp_series_2limb`]. This file is a typed-shell
//! surface; there are zero `crate::algos::*` or
//! `crate::algos::support::fixed::*` references in it.
//!
//! # Precision
//!
//! The f64-bridge forms are **Lossy** — `self` round-trips through
//! `f64`. Every `_strict` / `_strict_with` form is
//! **correctly rounded** under the selected
//! [`RoundingMode`]: the result is within 0.5 ULP of the exact
//! value. They evaluate the series in the `algos::support::fixed::Fixed`
//! guard-digit intermediate and round once at the end.
//!
//! [`RoundingMode`]: crate::RoundingMode
//!
//! # Domain handling
//!
//! `f64::ln`, `f64::log2`, `f64::log10`, and `f64::log` return `-Infinity`
//! for `0.0` and `NaN` for negative inputs. The f64 bridge maps `NaN` to
//! `D38::ZERO` and saturates infinities to `D38::MAX` or `D38::MIN`.
//! The `*_strict` forms panic on out-of-domain inputs (`self <= 0`).

impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
    // ── Logarithms ────────────────────────────────────────────────

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// # Algorithm
    ///
    /// Range reduction `x = 2^k * m` with `m ∈ [1, 2)`, then the
    /// area-hyperbolic-tangent series
    /// `ln(m) = 2·artanh(t)`, `t = (m-1)/(m+1) ∈ [0, 1/3]`,
    /// `artanh(t) = t + t³/3 + t⁵/5 + …`, evaluated in a 256-bit
    /// fixed-point intermediate at `SCALE + STRICT_GUARD` working
    /// digits. The guard digits bound the total accumulated rounding
    /// error far below 0.5 ULP of the output, so the result —
    /// `k·ln(2) + ln(m)`, rounded once at the end — is correctly
    /// rounded.
    ///
    /// # Precision
    ///
    /// Strict: integer-only, and **correctly rounded** — the result is
    /// within 0.5 ULP of the exact natural logarithm.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0`, or if the result overflows the type's
    /// representable range (only possible for `ln` of a near-`MAX`
    /// value at `SCALE >= 37`).
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        self.ln_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Natural log under the supplied rounding mode. See [`Self::ln`].
    #[inline]
    #[must_use]
    pub fn ln_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::ln::dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Returns `ln(1 + self)`.
    ///
    /// # Why it exists
    ///
    /// For API parity and standards conformance (C `log1p`, IEEE
    /// 754-2019 `logp1`). In this crate's fixed-point representation it
    /// is numerically **equivalent** to `(1 + self).ln()` at the
    /// same scale — `1 + self` is exactly representable, so the binary
    /// floating-point cancellation that motivates a separate `log1p`
    /// does not arise here. It is not more accurate than
    /// [`Self::ln`] of `1 + self`; both are correctly rounded.
    ///
    /// # Algorithm
    ///
    /// The Goldberg/Higham reformulation
    /// `log1p(t) = 2·artanh(t / (2 + t))`, evaluated at
    /// `SCALE + STRICT_GUARD` working digits and rounded once at the
    /// end, so the result is correctly rounded. See `policy::log1p`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only, and **correctly rounded** — within 0.5 ULP.
    ///
    /// # Panics
    ///
    /// Panics if `self <= -1` (the domain is `t > -1`, mirroring
    /// [`Self::ln`]'s positive-argument requirement on `1 + t`),
    /// or if the result overflows the type's representable range.
    #[inline]
    #[must_use]
    pub fn log1p(self) -> Self {
        self.log1p_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `ln(1 + self)` under the supplied rounding mode. See
    /// [`Self::log1p`].
    #[inline]
    #[must_use]
    pub fn log1p_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::log1p::dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Returns the logarithm of `self` in the given `base`.
    #[inline]
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        self.log_with(base, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Logarithm in `base` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn log_with(self, base: Self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::log::dispatch::<_, SCALE>(self.to_bits(), base.to_bits(), mode))
    }

    /// Returns the base-2 logarithm of `self`.
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        self.log2_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-2 log under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn log2_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::ln::log2_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Returns the base-10 logarithm of `self`.
    #[inline]
    #[must_use]
    pub fn log10(self) -> Self {
        self.log10_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-10 log under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn log10_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::ln::log10_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    // ── Exponentials ──────────────────────────────────────────────

    /// Returns `e^self` (natural exponential).
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        self.exp_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `e^self` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn exp_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::exp::dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Returns `e^self - 1`.
    ///
    /// # Why it exists
    ///
    /// For API parity and standards conformance (C `expm1`, IEEE
    /// 754-2019 `expm1`) — and, unlike its `log1p` sibling, for a
    /// concrete capability `exp` cannot provide:
    ///
    /// **Domain reach.** The `- 1` happens at the WORKING scale, ahead of
    /// the storage range check, so the representable argument range is
    /// `self <= ln(1 + MAX)` where [`Self::exp`] stops at
    /// `ln(MAX)` — exactly the arguments whose `e^self` lands in
    /// `(MAX, MAX + 1]`. The extra band is `ln(1 + 1/MAX)` wide: with
    /// `MAX ≈ 17` at this width's maximum scale that is about `0.057`,
    /// narrowing at lower scales. Small, but real — there are arguments
    /// this answers and `exp` panics on.
    ///
    /// It is NOT more accurate than `exp(self) - 1` where both are
    /// representable: in this crate's fixed-point representation `1` is
    /// exactly `10^SCALE` raw units, so subtracting it is an exact grid
    /// translation and rounding commutes with it — both are correctly
    /// rounded and agree bit-for-bit. The binary floating-point
    /// cancellation that motivates a separate `expm1` does not arise
    /// here.
    ///
    /// # Algorithm
    ///
    /// For `|self| <= 1`, the leading-term-dropped Taylor series
    /// `x + x²/2! + x³/3! + …`, which needs no range reduction and keeps
    /// every digit of a tiny argument; outside that band, `e^x - 1`
    /// formed at the working scale. Evaluated at `SCALE + STRICT_GUARD`
    /// working digits with Ziv escalation, so the result is correctly
    /// rounded. See `policy::expm1`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only, and **correctly rounded** — within 0.5 ULP.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type's representable range.
    /// There is no lower domain limit: `expm1` tends to `-1` as `self`
    /// tends to `-∞`, which is representable at every scale.
    #[inline]
    #[must_use]
    pub fn expm1(self) -> Self {
        self.expm1_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `e^self - 1` under the supplied rounding mode. See
    /// [`Self::expm1`].
    #[inline]
    #[must_use]
    pub fn expm1_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::expm1::dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Returns `2^self` (base-2 exponential).
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        self.exp2_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `2^self` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn exp2_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::exp::exp2_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

}

// Gated to exactly the configurations where the plain `ln` / `exp` /
// `log2` / `log10` dispatchers above resolve to the strict path, plus
// `std` — the expected values below are f64 references, and `f64::ln`
// and friends are std-only under the crate's float policy.
#[cfg(all(test, feature = "std"))]
mod strict_tests {
    use crate::types::widths::D38s12;

    /// Tolerance in ULPs for the strict transcendentals. They are
    /// correctly rounded (≤ 0.5 ULP); 2 LSB of slack absorbs the
    /// test's own expected-value rounding.
    const STRICT_TOLERANCE_LSB: i128 = 2;

    fn within(actual: D38s12, expected_bits: i128, tolerance: i128) -> bool {
        (actual.to_bits().as_i128() - expected_bits).abs() <= tolerance
    }

    /// ln(1) == 0 exactly (no series terms contribute).
    #[test]
    fn ln_of_one_is_zero() {
        assert_eq!(D38s12::ONE.ln(), D38s12::ZERO);
    }

    /// `ln` is correctly rounded: cross-check against the f64
    /// bridge at a scale where `f64` (≈ 15–16 significant digits) is
    /// comfortably more precise than the type's ULP, so the
    /// correctly-rounded integer result must agree to within 1 ULP.
    #[test]
    fn ln_strict_is_correctly_rounded_vs_f64() {
        fn check(raw: i128) {
            let value = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = value.ln().to_bits().as_i128();
            let reference = {
                let as_float = raw as f64 / 1e9;
                (as_float.ln() * 1e9).round() as i128
            };
            assert!(
                (strict - reference).abs() <= 1,
                "ln({raw}) = {strict}, f64 reference {reference}"
            );
        }
        for &raw in &[
            1,
            500_000_000,
            1_000_000_000,
            1_500_000_000,
            2_000_000_000,
            2_718_281_828,
            10_000_000_000,
            123_456_789_012_345,
            999_999_999_999_999_999,
            i64::MAX as i128,
        ] {
            check(raw);
        }
    }

    /// `exp` / `log2` / `log10` agree with the f64
    /// bridge to within 1 ULP at D38<9>.
    #[test]
    fn strict_log_exp_family_matches_f64() {
        fn check_exp(raw: i128) {
            let value = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = value.exp().to_bits().as_i128();
            let reference = ((raw as f64 / 1e9).exp() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "exp({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log2(raw: i128) {
            let value = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = value.log2().to_bits().as_i128();
            let reference = ((raw as f64 / 1e9).log2() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log2({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log10(raw: i128) {
            let value = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = value.log10().to_bits().as_i128();
            let reference = ((raw as f64 / 1e9).log10() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log10({raw}) = {strict}, f64 reference {reference}"
            );
        }
        for &raw in &[
            -5_000_000_000,
            -1_000_000_000,
            -500_000_000,
            1,
            500_000_000,
            1_000_000_000,
            2_000_000_000,
            5_000_000_000,
            10_000_000_000,
        ] {
            check_exp(raw);
        }
        for &raw in &[
            1,
            500_000_000,
            1_000_000_000,
            2_000_000_000,
            8_000_000_000,
            10_000_000_000,
            123_456_789_012_345,
            i64::MAX as i128,
        ] {
            check_log2(raw);
            check_log10(raw);
        }
    }

    /// `exp2` is exact at integer arguments: `2^10` is `1024`.
    #[test]
    fn strict_exp2_at_integers() {
        for k in 0_i128..=12 {
            let value = crate::D::<crate::int::types::Int<2>, 12>::from_bits(crate::int::types::Int::<2>::from_i128(k * 10i128.pow(12)));
            let got = value.exp2().to_bits().as_i128();
            let expected = (1i128 << k) * 10i128.pow(12);
            assert_eq!(got, expected, "2^{k}");
        }
    }

    /// `ln` is exact at the powers of two it can represent.
    #[test]
    fn ln_strict_of_powers_of_two() {
        let ln2_s18: i128 = 693_147_180_559_945_309;
        for k in 1_i128..=20 {
            let value = crate::D::<crate::int::types::Int<2>, 18>::from_bits(crate::int::types::Int::<2>::from_i128((1i128 << k) * 10i128.pow(18)));
            let got = value.ln().to_bits().as_i128();
            let expected = k * ln2_s18;
            let tolerance = k / 2 + 2;
            assert!(
                (got - expected).abs() <= tolerance,
                "ln(2^{k}) = {got}, expected ≈ {expected}"
            );
        }
    }

    /// ln(2) at scale 12 = 693_147_180_560 (canonical rounded to 12 places).
    #[test]
    fn ln_of_two_close_to_canonical() {
        let two = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(2_000_000_000_000));
        let result = two.ln();
        assert!(
            within(result, 693_147_180_560, STRICT_TOLERANCE_LSB),
            "ln(2) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// ln(e) is approximately 1.
    #[test]
    fn ln_of_e_close_to_one() {
        let e_at_s12 = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(2_718_281_828_459));
        let result = e_at_s12.ln();
        assert!(
            within(result, 1_000_000_000_000, STRICT_TOLERANCE_LSB),
            "ln(e) bits = {}, expected ~1_000_000_000_000",
            result.to_bits().as_i128()
        );
    }

    /// ln(10) at scale 12 = 2_302_585_092_994 (canonical).
    #[test]
    fn ln_of_ten_close_to_canonical() {
        let ten = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(10_000_000_000_000));
        let result = ten.ln();
        assert!(
            within(result, 2_302_585_092_994, STRICT_TOLERANCE_LSB),
            "ln(10) bits = {}, expected ~2_302_585_092_994",
            result.to_bits().as_i128()
        );
    }

    /// ln of a value > 1 is positive.
    #[test]
    fn ln_above_one_is_positive() {
        let value = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        let result = value.ln();
        assert!(result.to_bits().as_i128() > 0);
    }

    /// ln of a value in (0, 1) is negative.
    #[test]
    fn ln_below_one_is_negative() {
        let value = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let result = value.ln();
        assert!(result.to_bits().as_i128() < 0);
        assert!(
            within(result, -693_147_180_560, STRICT_TOLERANCE_LSB),
            "ln(0.5) bits = {}, expected ~-693_147_180_560",
            result.to_bits().as_i128()
        );
    }

    #[test]
    #[should_panic(expected = "argument must be positive")]
    fn ln_of_zero_panics() {
        let _ = D38s12::ZERO.ln();
    }

    #[test]
    #[should_panic(expected = "argument must be positive")]
    fn ln_of_negative_panics() {
        let negative_value = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1_000_000_000_000));
        let _ = negative_value.ln();
    }

    // log2 / log10 / log derive from ln; tolerance grows because the
    // additional division step accumulates ~1 LSB.
    const DERIVED_LOG_TOLERANCE_LSB: i128 = 20;

    /// log2(2) ~= 1.
    #[test]
    fn log2_of_two_is_one() {
        let two = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(2_000_000_000_000));
        let result = two.log2();
        assert!(
            within(result, 1_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log2(2) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// log2(8) ~= 3.
    #[test]
    fn log2_of_eight_is_three() {
        let eight = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(8_000_000_000_000));
        let result = eight.log2();
        assert!(
            within(result, 3_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log2(8) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// log10(10) ~= 1.
    #[test]
    fn log10_of_ten_is_one() {
        let ten = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(10_000_000_000_000));
        let result = ten.log10();
        assert!(
            within(result, 1_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log10(10) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// log10(100) ~= 2.
    #[test]
    fn log10_of_hundred_is_two() {
        let hundred = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(100_000_000_000_000));
        let result = hundred.log10();
        assert!(
            within(result, 2_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log10(100) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// log_base_b(b) == 1 for any b > 0, b != 1.
    #[test]
    fn log_self_is_one() {
        let base = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(5_000_000_000_000));
        let result = base.log(base);
        assert!(
            within(result, 1_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log_5(5) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// log_2(8) == 3 via the generic log.
    #[test]
    fn log_with_base_two() {
        let eight = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(8_000_000_000_000));
        let two = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(2_000_000_000_000));
        let result = eight.log(two);
        assert!(
            within(result, 3_000_000_000_000, DERIVED_LOG_TOLERANCE_LSB),
            "log_2(8) bits = {}",
            result.to_bits().as_i128()
        );
    }

    #[test]
    #[should_panic(expected = "base must not equal 1")]
    fn log_base_one_panics() {
        let value = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(5_000_000_000_000));
        let one = D38s12::ONE;
        let _ = value.log(one);
    }

    // exp / exp2 tolerance accounts for Taylor truncation, 2^k bit-shift
    // exactness, and the range-reduction rounding step. ~20 LSB at D38s12.
    const EXP_TOLERANCE_LSB: i128 = 20;

    /// exp(0) == 1 exactly.
    #[test]
    fn exp_of_zero_is_one() {
        assert_eq!(D38s12::ZERO.exp(), D38s12::ONE);
    }

    /// exp(1) ~= e.
    #[test]
    fn exp_of_one_is_e() {
        let result = D38s12::ONE.exp();
        assert!(
            within(result, 2_718_281_828_459, EXP_TOLERANCE_LSB),
            "exp(1) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// exp(ln(2)) ~= 2.
    #[test]
    fn exp_of_ln_2_is_two() {
        let ln_2 = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(693_147_180_560));
        let result = ln_2.exp();
        assert!(
            within(result, 2_000_000_000_000, EXP_TOLERANCE_LSB),
            "exp(ln 2) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// exp(-1) ~= 1/e ~= 0.367879441171.
    #[test]
    fn exp_of_negative_one_is_reciprocal_e() {
        let neg_one = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1_000_000_000_000));
        let result = neg_one.exp();
        assert!(
            within(result, 367_879_441_171, EXP_TOLERANCE_LSB),
            "exp(-1) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// exp2(0) == 1 exactly.
    #[test]
    fn exp2_of_zero_is_one() {
        assert_eq!(D38s12::ZERO.exp2(), D38s12::ONE);
    }

    /// exp2(1) ~= 2.
    #[test]
    fn exp2_of_one_is_two() {
        let result = D38s12::ONE.exp2();
        assert!(
            within(result, 2_000_000_000_000, EXP_TOLERANCE_LSB),
            "exp2(1) bits = {}",
            result.to_bits().as_i128()
        );
    }

    /// exp2(10) ~= 1024.
    #[test]
    fn exp2_of_ten_is_1024() {
        let ten = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(10_000_000_000_000));
        let result = ten.exp2();
        assert!(
            within(result, 1_024_000_000_000_000, EXP_TOLERANCE_LSB * 10),
            "exp2(10) bits = {}",
            result.to_bits().as_i128()
        );
    }
}
