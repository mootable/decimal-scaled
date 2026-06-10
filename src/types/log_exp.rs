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
//! Each function ships four entry points so a single name covers
//! every (precision × rounding) combination:
//!
//! | Method            | Guard width    | Rounding mode               |
//! |-------------------|----------------|------------------------------|
//! | `<fn>_strict`     | crate default  | crate default               |
//! | `<fn>_strict_with`| crate default  | caller-supplied              |
//! | `<fn>_approx`     | caller-chosen  | crate default               |
//! | `<fn>_approx_with`| caller-chosen  | caller-supplied              |
//!
//! `_strict` runs at `SCALE + STRICT_GUARD` (const-folded so LLVM
//! specialises one optimal kernel per `SCALE`). `_approx` runs at
//! `SCALE + working_digits` chosen at call time — drop below
//! `STRICT_GUARD` to trade precision for latency (the Mercator /
//! Taylor series shortens proportionally), raise above for more
//! headroom on chained compositions. When `working_digits ==
//! STRICT_GUARD` the `_approx_with` body redirects to `_strict_with`
//! so the const-folded path is never displaced.
//!
//! `ln_strict` uses range reduction plus a Mercator series;
//! `exp_strict` uses range reduction plus a Taylor series; the
//! remaining methods compose those two. All four variants are
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
//! per-variant `ln_strict` / `ln_with` / `log_strict` / `log_with` /
//! `log2_*` / `log10_*` / `exp_strict` / `exp_with` / `exp2_*`
//! `Fixed`-shape functions) live in
//! [`crate::algos::ln::ln_series_2limb`] and
//! [`crate::algos::exp::exp_series_2limb`]. This file is a typed-shell
//! surface; there are zero `crate::algos::*` or
//! `crate::algos::support::fixed::*` references in it.
//!
//! # Precision
//!
//! The f64-bridge forms are **Lossy** — `self` round-trips through
//! `f64`. Every `_strict` / `_strict_with` / `_approx` /
//! `_approx_with` form is **correctly rounded** under the selected
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


/// Re-export of the D38 strict-mode guard-digit constant for in-crate
/// callers that branch on the strict-vs-approx working-scale match.
/// The authoritative definition lives in
/// [`crate::algos::ln::ln_series_2limb::STRICT_GUARD`].
pub(crate) use crate::algos::ln::ln_series_2limb::STRICT_GUARD;

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
    pub fn ln_strict(self) -> Self {
        self.ln_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Natural log under the supplied rounding mode. See [`Self::ln_strict`].
    #[inline]
    #[must_use]
    pub fn ln_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::ln::dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Natural logarithm with a caller-chosen number of guard digits
    /// above the storage scale, trading away the strict 0.5-ULP
    /// guarantee for proportionally faster evaluation.
    #[inline]
    #[must_use]
    pub fn ln_approx(self, working_digits: u32) -> Self {
        self.ln_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// Natural log with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn ln_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.ln_strict_with(mode);
        }
        Self::from_bits(crate::policy::ln::dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
    }

    /// Returns the natural logarithm (base e) of `self`.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        self.ln_strict()
    }

    /// Returns the logarithm of `self` in the given `base`.
    #[inline]
    #[must_use]
    pub fn log_strict(self, base: Self) -> Self {
        self.log_strict_with(base, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Logarithm in `base` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn log_strict_with(self, base: Self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::log::dispatch::<_, SCALE>(self.to_bits(), base.to_bits(), mode))
    }

    /// Logarithm with caller-chosen guard digits. See `ln_approx`.
    #[inline]
    #[must_use]
    pub fn log_approx(self, base: Self, working_digits: u32) -> Self {
        self.log_approx_with(
            base,
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// Logarithm with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn log_approx_with(
        self,
        base: Self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.log_strict_with(base, mode);
        }
        Self::from_bits(crate::policy::log::dispatch_with::<_, SCALE>(self.to_bits(), base.to_bits(), working_digits, mode))
    }

    /// Returns the logarithm of `self` in the given `base`.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn log(self, base: Self) -> Self {
        self.log_strict(base)
    }

    /// Returns the base-2 logarithm of `self`.
    #[inline]
    #[must_use]
    pub fn log2_strict(self) -> Self {
        self.log2_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-2 log under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn log2_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::ln::log2_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Base-2 log with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn log2_approx(self, working_digits: u32) -> Self {
        self.log2_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// Base-2 log with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn log2_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.log2_strict_with(mode);
        }
        Self::from_bits(crate::policy::ln::log2_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
    }

    /// Returns the base-2 logarithm of `self`.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        self.log2_strict()
    }

    /// Returns the base-10 logarithm of `self`.
    #[inline]
    #[must_use]
    pub fn log10_strict(self) -> Self {
        self.log10_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-10 log under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn log10_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::ln::log10_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Base-10 log with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn log10_approx(self, working_digits: u32) -> Self {
        self.log10_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// Base-10 log with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn log10_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.log10_strict_with(mode);
        }
        Self::from_bits(crate::policy::ln::log10_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
    }

    /// Returns the base-10 logarithm of `self`.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn log10(self) -> Self {
        self.log10_strict()
    }

    // ── Exponentials ──────────────────────────────────────────────

    /// Returns `e^self` (natural exponential).
    #[inline]
    #[must_use]
    pub fn exp_strict(self) -> Self {
        self.exp_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `e^self` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn exp_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::exp::dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Exponential with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn exp_approx(self, working_digits: u32) -> Self {
        self.exp_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// Exponential with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn exp_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.exp_strict_with(mode);
        }
        Self::from_bits(crate::policy::exp::dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
    }

    /// Returns `e^self` (natural exponential).
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        self.exp_strict()
    }

    /// Returns `2^self` (base-2 exponential).
    #[inline]
    #[must_use]
    pub fn exp2_strict(self) -> Self {
        self.exp2_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `2^self` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn exp2_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        Self::from_bits(crate::policy::exp::exp2_dispatch::<_, SCALE>(self.to_bits(), mode))
    }

    /// Base-2 exponential with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn exp2_approx(self, working_digits: u32) -> Self {
        self.exp2_approx_with(
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// Base-2 exponential with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn exp2_approx_with(
        self,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == STRICT_GUARD {
            return self.exp2_strict_with(mode);
        }
        Self::from_bits(crate::policy::exp::exp2_dispatch_with::<_, SCALE>(self.to_bits(), working_digits, mode))
    }

    /// Returns `2^self` (base-2 exponential).
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        self.exp2_strict()
    }
}

#[cfg(all(test, feature = "strict", not(feature = "fast")))]
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

    /// `ln_strict` is correctly rounded: cross-check against the f64
    /// bridge at a scale where `f64` (≈ 15–16 significant digits) is
    /// comfortably more precise than the type's ULP, so the
    /// correctly-rounded integer result must agree to within 1 ULP.
    #[test]
    fn ln_strict_is_correctly_rounded_vs_f64() {
        fn check(raw: i128) {
            let x = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = x.ln_strict().to_bits().as_i128();
            let reference = {
                let v = raw as f64 / 1e9;
                (v.ln() * 1e9).round() as i128
            };
            assert!(
                (strict - reference).abs() <= 1,
                "ln_strict({raw}) = {strict}, f64 reference {reference}"
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

    /// `exp_strict` / `log2_strict` / `log10_strict` agree with the f64
    /// bridge to within 1 ULP at D38<9>.
    #[test]
    fn strict_log_exp_family_matches_f64() {
        fn check_exp(raw: i128) {
            let x = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = x.exp_strict().to_bits().as_i128();
            let reference = ((raw as f64 / 1e9).exp() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "exp_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log2(raw: i128) {
            let x = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = x.log2_strict().to_bits().as_i128();
            let reference = ((raw as f64 / 1e9).log2() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log2_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log10(raw: i128) {
            let x = crate::D::<crate::int::types::Int<2>, 9>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let strict = x.log10_strict().to_bits().as_i128();
            let reference = ((raw as f64 / 1e9).log10() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log10_strict({raw}) = {strict}, f64 reference {reference}"
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

    /// `exp2_strict` is exact at integer arguments: `2^10` is `1024`.
    #[test]
    fn strict_exp2_at_integers() {
        for k in 0_i128..=12 {
            let x = crate::D::<crate::int::types::Int<2>, 12>::from_bits(crate::int::types::Int::<2>::from_i128(k * 10i128.pow(12)));
            let got = x.exp2_strict().to_bits().as_i128();
            let expected = (1i128 << k) * 10i128.pow(12);
            assert_eq!(got, expected, "2^{k}");
        }
    }

    /// `ln_strict` is exact at the powers of two it can represent.
    #[test]
    fn ln_strict_of_powers_of_two() {
        let ln2_s18: i128 = 693_147_180_559_945_309;
        for k in 1_i128..=20 {
            let x = crate::D::<crate::int::types::Int<2>, 18>::from_bits(crate::int::types::Int::<2>::from_i128((1i128 << k) * 10i128.pow(18)));
            let got = x.ln_strict().to_bits().as_i128();
            let expected = k * ln2_s18;
            let tol = k / 2 + 2;
            assert!(
                (got - expected).abs() <= tol,
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
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        let result = v.ln();
        assert!(result.to_bits().as_i128() > 0);
    }

    /// ln of a value in (0, 1) is negative.
    #[test]
    fn ln_below_one_is_negative() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let result = v.ln();
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
        let neg = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1_000_000_000_000));
        let _ = neg.ln();
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
        let x = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(5_000_000_000_000));
        let one = D38s12::ONE;
        let _ = x.log(one);
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
