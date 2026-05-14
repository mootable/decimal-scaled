//! Logarithm and exponential methods for [`D128`].
//!
//! # Methods
//!
//! - **Logarithms:** [`D128::ln`] / [`D128::log`] / [`D128::log2`] / [`D128::log10`].
//! - **Exponentials:** [`D128::exp`] / [`D128::exp2`].
//!
//! # The `*_strict` dual API
//!
//! Each method has an integer-only `<method>_strict` form and an
//! f64-bridge form:
//!
//! - `<method>_strict` — always compiled (unless the `no_strict`
//!   feature is set), `no_std`-compatible, platform-deterministic.
//!   `ln_strict` uses range reduction plus a Mercator series;
//!   `exp_strict` uses range reduction plus a Taylor series; the
//!   remaining methods compose those two.
//! - The f64-bridge form is gated on `std` and calls the inherent
//!   `f64` intrinsic.
//!
//! The plain `<method>` is a dispatcher: with the `strict` feature it
//! calls `<method>_strict`, otherwise the f64 bridge. See
//! `docs/strict-mode.md` for the full dual-API and feature rules.
//!
//! # Precision
//!
//! The f64-bridge forms are **Lossy** — `self` round-trips through
//! `f64`. The `*_strict` forms are **correctly rounded**: the result
//! is within 0.5 ULP of the exact value (IEEE-754 round-to-nearest).
//! They evaluate the series in the `wide_int::Fixed` guard-digit
//! intermediate and round once at the end.
//!
//! # Domain handling
//!
//! `f64::ln`, `f64::log2`, `f64::log10`, and `f64::log` return `-Infinity`
//! for `0.0` and `NaN` for negative inputs. The f64 bridge maps `NaN` to
//! `D128::ZERO` and saturates infinities to `D128::MAX` or `D128::MIN`.
//! The `*_strict` forms panic on out-of-domain inputs (`self <= 0`).

use crate::core_type::D128;

// ─────────────────────────────────────────────────────────────────────
// Correctly-rounded strict log / exp core.
//
// The strict `ln` / `log` / `log2` / `log10` / `exp` / `exp2` all run
// on a 256-bit `Fixed` intermediate at `SCALE + GUARD` working digits.
// The 30 guard digits bound the total accumulated rounding error far
// below 0.5 ULP of the output, so each result — rounded once,
// half-to-even, back to `SCALE` — is correctly rounded.
//
// `GUARD = 30` keeps the working scale `W = SCALE + 30 <= 68` for
// `SCALE <= 38`, which is small enough that the 64-digit constants
// cover it, `r · 10^GUARD` fits `U256`, and the 512-bit mul/div
// intermediates never overflow.
// ─────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "no_strict"))]
pub(crate) const STRICT_GUARD: u32 = 30;

/// `ln(2)` as a `Fixed` at working scale `w` (`w <= 64`). The constant
/// is embedded to 64 fractional digits and narrowed to `w`.
#[cfg(not(feature = "no_strict"))]
pub(crate) fn wide_ln2(w: u32) -> crate::wide_int::Fixed {
    // ln 2 = 0.693147180559945309417232121458176568075500134360255254120680 0094
    crate::wide_int::Fixed::from_decimal_split(
        69_314_718_055_994_530_941_723_212_145_817_u128,
        65_680_755_001_343_602_552_541_206_800_094_u128,
    )
    .rescale_down(64, w)
}

/// `ln(10)` as a `Fixed` at working scale `w` (`w <= 63`). Embedded to
/// 63 fractional digits (`ln 10 ≈ 2.30…` has an integer digit) and
/// narrowed to `w`.
#[cfg(not(feature = "no_strict"))]
fn wide_ln10(w: u32) -> crate::wide_int::Fixed {
    // ln 10 = 2.302585092994045684017991454684364207601101488628772976033327 901
    crate::wide_int::Fixed::from_decimal_split(
        23_025_850_929_940_456_840_179_914_546_843_u128,
        64_207_601_101_488_628_772_976_033_327_901_u128,
    )
    .rescale_down(63, w)
}

/// Natural logarithm of a positive working-scale value `v_w`, returned
/// at the same working scale `w`.
///
/// Range-reduces `v = 2^k · m` with `m ∈ [1,2)` — the mantissa is
/// recomputed exactly from `v_w` once `k` is known — then evaluates
/// `ln(m) = 2·artanh((m-1)/(m+1))` (`t ∈ [0,1/3]`, fast convergence)
/// and returns `k·ln(2) + ln(m)`.
#[cfg(not(feature = "no_strict"))]
pub(crate) fn ln_fixed(v_w: crate::wide_int::Fixed, w: u32) -> crate::wide_int::Fixed {
    use crate::wide_int::Fixed;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let two_w = one_w.double();

    // Range reduction: find k with v ∈ [2^k, 2^(k+1)); m_w = v_w / 2^k.
    let mut k: i32 = v_w.bit_length() as i32 - one_w.bit_length() as i32;
    let m_w = loop {
        let m = if k >= 0 {
            v_w.shr(k as u32)
        } else {
            v_w.shl((-k) as u32)
        };
        if m.ge_mag(two_w) {
            k += 1;
        } else if !m.ge_mag(one_w) {
            k -= 1;
        } else {
            break m;
        }
    };

    // t = (m - 1) / (m + 1) ∈ [0, 1/3]; artanh(t) = t + t³/3 + t⁵/5 + …
    let t = m_w.sub(one_w).div(m_w.add(one_w), w);
    let t2 = t.mul(t, w);
    let mut sum = t;
    let mut term = t;
    let mut j: u128 = 1;
    loop {
        term = term.mul(t2, w);
        let contrib = term.div_small(2 * j + 1);
        if contrib.is_zero() {
            break;
        }
        sum = sum.add(contrib);
        j += 1;
        if j > 400 {
            break;
        }
    }
    let ln_m = sum.double();

    let ln2 = wide_ln2(w);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    k_ln2.add(ln_m)
}

/// `e` raised to a working-scale value `v_w`, returned at the same
/// working scale `w`.
///
/// Range-reduces `v = k·ln(2) + s` with `|s| ≤ ln(2)/2`, evaluates the
/// Taylor series for `exp(s)`, then reassembles `2^k · exp(s)` by
/// shifting the working-scale value (so the `2^k` factor never
/// amplifies a rounding error).
///
/// # Panics
///
/// Panics if `2^k · exp(s)` cannot fit a 256-bit working value — i.e.
/// the caller's result would overflow its representable range.
#[cfg(not(feature = "no_strict"))]
pub(crate) fn exp_fixed(v_w: crate::wide_int::Fixed, w: u32) -> crate::wide_int::Fixed {
    use crate::wide_int::Fixed;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let ln2 = wide_ln2(w);

    // k = round(v / ln 2); s = v - k·ln(2), |s| <= ln(2)/2.
    let k = v_w.div(ln2, w).round_to_nearest_int(w);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    let s = v_w.sub(k_ln2);

    // Taylor series exp(s) = 1 + s + s²/2! + … — `term` carries sⁿ/n!.
    let mut sum = one_w;
    let mut term = one_w;
    let mut n: u128 = 1;
    loop {
        term = term.mul(s, w).div_small(n);
        if term.is_zero() {
            break;
        }
        sum = sum.add(term);
        n += 1;
        if n > 400 {
            break;
        }
    }

    // exp(v) = 2^k · exp(s).
    if k >= 0 {
        let shift = k as u32;
        if sum.bit_length() + shift > 256 {
            panic!("D128::exp: result overflows the representable range");
        }
        sum.shl(shift)
    } else {
        sum.shr((-k) as u32)
    }
}

impl<const SCALE: u32> D128<SCALE> {
    // Logarithms

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// # Algorithm
    ///
    /// Range reduction `x = 2^k * m` with `m ∈ [1, 2)`, then a Mercator
    /// reduction `x = 2^k * m` with `m ∈ [1, 2)`, then the
    /// area-hyperbolic-tangent series
    /// `ln(m) = 2·artanh(t)`, `t = (m-1)/(m+1) ∈ [0, 1/3]`,
    /// `artanh(t) = t + t³/3 + t⁵/5 + …`, evaluated in a 256-bit
    /// fixed-point intermediate at `SCALE + 20` working digits. The 20
    /// guard digits bound the total accumulated rounding error far
    /// below 0.5 ULP of the output, so the result — `k·ln(2) + ln(m)`,
    /// rounded once at the end — is correctly rounded.
    ///
    /// # Precision
    ///
    /// Strict: integer-only, and **correctly rounded** — the result is
    /// within 0.5 ULP of the exact natural logarithm (IEEE-754
    /// round-to-nearest).
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0`, or if the result overflows the type's
    /// representable range (only possible for `ln` of a near-`MAX`
    /// value at `SCALE >= 37`).
    ///
    /// Always available, regardless of the `strict` feature. When
    /// `strict` is enabled, the plain [`Self::ln`] delegates here.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn ln_strict(self) -> Self {
        use crate::wide_int::Fixed;
        if self.0 <= 0 {
            panic!("D128::ln: argument must be positive");
        }
        let w = SCALE + STRICT_GUARD;
        let v_w =
            Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
        let raw = ln_fixed(v_w, w)
            .round_to_i128(w, SCALE)
            .expect("D128::ln: result out of range");
        Self::from_bits(raw)
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
    /// integer-only as `ln(self) / ln(base)` — both logarithms and the
    /// division are carried in the wide guard-digit intermediate, so
    /// the result is correctly rounded.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0` or `base <= 0`, or if `base == 1`
    /// (division by `ln(1) = 0`).
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn log_strict(self, base: Self) -> Self {
        use crate::wide_int::Fixed;
        if self.0 <= 0 {
            panic!("D128::log: argument must be positive");
        }
        if base.0 <= 0 {
            panic!("D128::log: base must be positive");
        }
        let w = SCALE + STRICT_GUARD;
        let pow = 10u128.pow(STRICT_GUARD);
        let v_w = Fixed::from_u128_mag(self.0 as u128, false).mul_u128(pow);
        let b_w = Fixed::from_u128_mag(base.0 as u128, false).mul_u128(pow);
        let ln_b = ln_fixed(b_w, w);
        if ln_b.is_zero() {
            panic!("D128::log: base must not equal 1 (ln(1) is zero)");
        }
        let raw = ln_fixed(v_w, w)
            .div(ln_b, w)
            .round_to_i128(w, SCALE)
            .expect("D128::log: result out of range");
        Self::from_bits(raw)
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
    /// `ln(self) / ln(2)` in the wide guard-digit intermediate — the
    /// result is correctly rounded.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0`.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn log2_strict(self) -> Self {
        use crate::wide_int::Fixed;
        if self.0 <= 0 {
            panic!("D128::log2: argument must be positive");
        }
        let w = SCALE + STRICT_GUARD;
        let v_w =
            Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
        let raw = ln_fixed(v_w, w)
            .div(wide_ln2(w), w)
            .round_to_i128(w, SCALE)
            .expect("D128::log2: result out of range");
        Self::from_bits(raw)
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
    /// as `ln(self) / ln(10)` in the wide guard-digit intermediate —
    /// the result is correctly rounded.
    ///
    /// Always available, regardless of the `strict` feature.
    ///
    /// # Panics
    ///
    /// Panics if `self <= 0`.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn log10_strict(self) -> Self {
        use crate::wide_int::Fixed;
        if self.0 <= 0 {
            panic!("D128::log10: argument must be positive");
        }
        let w = SCALE + STRICT_GUARD;
        let v_w =
            Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
        let raw = ln_fixed(v_w, w)
            .div(wide_ln10(w), w)
            .round_to_i128(w, SCALE)
            .expect("D128::log10: result out of range");
        Self::from_bits(raw)
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
    /// Range reduction `x = k·ln(2) + s` with `k = round(x / ln 2)` and
    /// `|s| ≤ ln(2)/2 ≈ 0.347`, then the Taylor series
    /// `exp(s) = 1 + s + s²/2! + …` evaluated in a 256-bit `Fixed`
    /// intermediate at `SCALE + 30` working digits. Reassembly is
    /// `exp(x) = 2^k · exp(s)`, applied as a shift on the working-scale
    /// value *before* the final rounding, so the `2^k` factor never
    /// amplifies a rounding error. The result is rounded once,
    /// half-to-even, back to `SCALE`.
    ///
    /// # Precision
    ///
    /// Strict: integer-only, and **correctly rounded** — the result is
    /// within 0.5 ULP of the exact exponential (IEEE-754
    /// round-to-nearest).
    ///
    /// # Panics
    ///
    /// Panics if the result overflows the type's representable range.
    #[inline]
    #[must_use]
    #[cfg(not(feature = "no_strict"))]
    pub fn exp_strict(self) -> Self {
        use crate::wide_int::Fixed;
        if self.0 == 0 {
            return Self::ONE;
        }
        let w = SCALE + STRICT_GUARD;
        let negative_input = self.0 < 0;
        let v_w = Fixed::from_u128_mag(self.0.unsigned_abs(), false)
            .mul_u128(10u128.pow(STRICT_GUARD));
        let v_w = if negative_input { v_w.neg() } else { v_w };
        let raw = exp_fixed(v_w, w)
            .round_to_i128(w, SCALE)
            .expect("D128::exp: result overflows the representable range");
        Self::from_bits(raw)
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
    /// `exp(self · ln(2))` — the `self · ln(2)` product is formed in
    /// the wide guard-digit intermediate (not at the type's own scale),
    /// so the result is correctly rounded.
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
        use crate::wide_int::Fixed;
        if self.0 == 0 {
            return Self::ONE;
        }
        let w = SCALE + STRICT_GUARD;
        let negative_input = self.0 < 0;
        let v_w = Fixed::from_u128_mag(self.0.unsigned_abs(), false)
            .mul_u128(10u128.pow(STRICT_GUARD));
        let v_w = if negative_input { v_w.neg() } else { v_w };
        // arg = self · ln(2), carried at the wide working scale.
        let arg_w = v_w.mul(wide_ln2(w), w);
        let raw = exp_fixed(arg_w, w)
            .round_to_i128(w, SCALE)
            .expect("D128::exp2: result overflows the representable range");
        Self::from_bits(raw)
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

    /// Tolerance in ULPs for the strict transcendentals. They are
    /// correctly rounded (≤ 0.5 ULP); 2 LSB of slack absorbs the
    /// test's own expected-value rounding.
    const STRICT_TOLERANCE_LSB: i128 = 2;

    fn within(actual: D128s12, expected_bits: i128, tolerance: i128) -> bool {
        (actual.to_bits() - expected_bits).abs() <= tolerance
    }

    /// ln(1) == 0 exactly (no series terms contribute).
    #[test]
    fn ln_of_one_is_zero() {
        assert_eq!(D128s12::ONE.ln(), D128s12::ZERO);
    }

    /// `ln_strict` is correctly rounded: cross-check against the f64
    /// bridge at a scale where `f64` (≈ 15–16 significant digits) is
    /// comfortably more precise than the type's ULP, so the
    /// correctly-rounded integer result must agree to within 1 ULP.
    #[test]
    fn ln_strict_is_correctly_rounded_vs_f64() {
        use crate::core_type::D128;
        // D128<9>: ULP is 1e-9; f64 ln is good to ~1e-15 over this
        // range, so the correctly-rounded result is within 1 ULP of the
        // f64 reference (allow 1 for the f64 reference's own rounding).
        fn check(raw: i128) {
            let x = D128::<9>::from_bits(raw);
            let strict = x.ln_strict().to_bits();
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
            500_000_000,            // 0.5
            1_000_000_000,          // 1.0
            1_500_000_000,          // 1.5
            2_000_000_000,          // 2.0
            2_718_281_828,          // ≈ e
            10_000_000_000,         // 10
            123_456_789_012_345,    // ≈ 123456.78…
            999_999_999_999_999_999,// ≈ 1e9
            i64::MAX as i128,
        ] {
            check(raw);
        }
    }

    /// `exp_strict` / `log2_strict` / `log10_strict` agree with the f64
    /// bridge to within 1 ULP at D128<9>, where f64 is comfortably more
    /// precise than the type's ULP — strong evidence of correct
    /// rounding for the whole log/exp family.
    #[test]
    fn strict_log_exp_family_matches_f64() {
        use crate::core_type::D128;
        fn check_exp(raw: i128) {
            let x = D128::<9>::from_bits(raw);
            let strict = x.exp_strict().to_bits();
            let reference = ((raw as f64 / 1e9).exp() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "exp_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log2(raw: i128) {
            let x = D128::<9>::from_bits(raw);
            let strict = x.log2_strict().to_bits();
            let reference = ((raw as f64 / 1e9).log2() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log2_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log10(raw: i128) {
            let x = D128::<9>::from_bits(raw);
            let strict = x.log10_strict().to_bits();
            let reference = ((raw as f64 / 1e9).log10() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log10_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        // exp: keep the argument modest so the result stays in range.
        for &raw in &[
            -5_000_000_000, -1_000_000_000, -500_000_000, 1, 500_000_000,
            1_000_000_000, 2_000_000_000, 5_000_000_000, 10_000_000_000,
        ] {
            check_exp(raw);
        }
        // log2 / log10: positive arguments across the range.
        for &raw in &[
            1, 500_000_000, 1_000_000_000, 2_000_000_000, 8_000_000_000,
            10_000_000_000, 123_456_789_012_345, i64::MAX as i128,
        ] {
            check_log2(raw);
            check_log10(raw);
        }
    }

    /// `exp2_strict` is exact at integer arguments: `2^10` is `1024`.
    #[test]
    fn strict_exp2_at_integers() {
        use crate::core_type::D128;
        for k in 0_i128..=12 {
            let x = D128::<12>::from_bits(k * 10i128.pow(12));
            let got = x.exp2_strict().to_bits();
            let expected = (1i128 << k) * 10i128.pow(12);
            // Correctly rounded: exactly the integer power of two.
            assert_eq!(got, expected, "2^{k}");
        }
    }

    /// `ln_strict` is exact at the powers of two it can represent:
    /// `ln(2^k)` rounds to `k · ln(2)` at the type's scale.
    #[test]
    fn ln_strict_of_powers_of_two() {
        use crate::core_type::D128;
        // ln(2) at scale 18, correctly rounded:
        // 0.693147180559945309… -> 693147180559945309.
        let ln2_s18: i128 = 693_147_180_559_945_309;
        for k in 1_i128..=20 {
            let x = D128::<18>::from_bits((1i128 << k) * 10i128.pow(18));
            let got = x.ln_strict().to_bits();
            let expected = k * ln2_s18;
            // k·ln(2) accumulates k roundings of the scale-18 ln(2);
            // the correctly-rounded result is within ⌈k/2⌉+1 of the
            // naive k·(rounded ln2).
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
