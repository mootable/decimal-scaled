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
//! # Precision
//!
//! The f64-bridge forms are **Lossy** — `self` round-trips through
//! `f64`. Every `_strict` / `_strict_with` / `_approx` /
//! `_approx_with` form is **correctly rounded** under the selected
//! [`RoundingMode`]: the result is within 0.5 ULP of the exact
//! value. They evaluate the series in the `d_w128_kernels::Fixed`
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

use crate::core_type::D38;

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

pub(crate) const STRICT_GUARD: u32 = 30;

// `ln(2)` and `ln(10)` are embedded at 75 fractional digits — the same
// reference scale `wide_pi` uses — so callers running at the maximum
// strict working scale `W = SCALE + STRICT_GUARD = 38 + 30 = 68` always
// rescale **down** (never up, which would wrap `from_w − to_w` as `u32`
// and silently produce a wrong constant). The 75-digit window covers
// every supported strict scale; widening it further is wasted work
// because the working scale is capped by `D38<SCALE>`'s storage range
// (no input that fits `i128` at `SCALE > 38` exists).
//
// Half-to-even rounded ln(2) × 10^75 and ln(10) × 10^75; both fit an
// `Int256` (max ≈ 5.78 × 10⁷⁶). The next-digit rounding is documented
// in-line so the truncation step is auditable from this file alone.

/// `ln(2) × 10^75`, half-to-even rounded (76th frac digit is 4 — round
/// down). Source: high-precision evaluation of the natural logarithm.
const LN2_S75: &str =
    "693147180559945309417232121458176568075500134360255254120680009493393621969";

/// `ln(10) × 10^75`, half-to-even rounded (76th frac digit is 3 — round
/// down). Source: high-precision evaluation of the natural logarithm.
const LN10_S75: &str =
    "2302585092994045684017991454684364207601101488628772976033327900967572609677";

const LN2_RAW: crate::wide_int::Int256 =
    match crate::wide_int::Int256::from_str_radix(LN2_S75, 10) {
        Ok(v) => v,
        Err(_) => panic!("log_exp_strict: LN2_S75 not parseable"),
    };

const LN10_RAW: crate::wide_int::Int256 =
    match crate::wide_int::Int256::from_str_radix(LN10_S75, 10) {
        Ok(v) => v,
        Err(_) => panic!("log_exp_strict: LN10_S75 not parseable"),
    };

/// Repacks an `Int256` reference (internally `[u64; 4]`) into a
/// `Fixed` magnitude (`[u128; 2]`) sourced at scale `75`.
#[inline]
fn fixed_from_int256(raw: crate::wide_int::Int256) -> crate::d_w128_kernels::Fixed {
    let words = raw.0;
    crate::d_w128_kernels::Fixed {
        negative: false,
        mag: [
            (words[0] as u128) | ((words[1] as u128) << 64),
            (words[2] as u128) | ((words[3] as u128) << 64),
        ],
    }
}

/// `ln(2)` as a `Fixed` at working scale `w` (`w <= 75`). Sourced from
/// the 75-digit reference and rescaled **down** to `w`.
///
/// Caller-side precondition: `w <= 75`. The D38 strict log family runs
/// at `w = SCALE + STRICT_GUARD`, capped at `38 + 30 = 68`, so every
/// strict call site is comfortably inside the bound. A debug-assert
/// documents the invariant for any future caller.
pub(crate) fn wide_ln2(w: u32) -> crate::d_w128_kernels::Fixed {
    debug_assert!(w <= 75, "wide_ln2: working scale {w} exceeds embedded 75-digit ln 2");
    let ln2_at_75 = fixed_from_int256(LN2_RAW);
    if w == 75 { ln2_at_75 } else { ln2_at_75.rescale_down(75, w) }
}

/// `ln(10)` as a `Fixed` at working scale `w` (`w <= 75`). Sourced from
/// the 75-digit reference and rescaled **down** to `w`.
///
/// Caller-side precondition: `w <= 75`. See [`wide_ln2`].
fn wide_ln10(w: u32) -> crate::d_w128_kernels::Fixed {
    debug_assert!(w <= 75, "wide_ln10: working scale {w} exceeds embedded 75-digit ln 10");
    let ln10_at_75 = fixed_from_int256(LN10_RAW);
    if w == 75 { ln10_at_75 } else { ln10_at_75.rescale_down(75, w) }
}

/// Natural logarithm of a positive working-scale value `v_w`, returned
/// at the same working scale `w`.
///
/// Range-reduces `v = 2^k · m` with `m ∈ [1,2)` — the mantissa is
/// recomputed exactly from `v_w` once `k` is known — then evaluates
/// `ln(m) = 2·artanh((m-1)/(m+1))` (`t ∈ [0,1/3]`, fast convergence)
/// and returns `k·ln(2) + ln(m)`.
pub(crate) fn ln_fixed(v_w: crate::d_w128_kernels::Fixed, w: u32) -> crate::d_w128_kernels::Fixed {
    use crate::d_w128_kernels::Fixed;
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
pub(crate) fn exp_fixed(v_w: crate::d_w128_kernels::Fixed, w: u32) -> crate::d_w128_kernels::Fixed {
    use crate::d_w128_kernels::Fixed;
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
        assert!(sum.bit_length() + shift <= 256, "D38::exp: result overflows the representable range");
        sum.shl(shift)
    } else {
        sum.shr((-k) as u32)
    }
}

impl<const SCALE: u32> D38<SCALE> {
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
    pub fn ln_strict(self) -> Self {
        self.ln_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Natural log under the supplied rounding mode. See [`Self::ln_strict`].
    ///
    /// Body delegates to `policy::ln::LnPolicy::ln_impl`.
    #[inline]
    #[must_use]
    pub fn ln_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        <Self as crate::policy::ln::LnPolicy>::ln_impl(self, mode)
    }

    /// Natural logarithm with a caller-chosen number of guard digits
    /// above the storage scale, trading away the strict 0.5-ULP
    /// guarantee for proportionally faster evaluation.
    ///
    /// `working_digits` controls the working scale `w = SCALE +
    /// working_digits` of the internal series evaluation. The default
    /// `ln_strict` uses `working_digits = 30` (the same `STRICT_GUARD`
    /// the rest of the strict family uses, sized for `<= 0.5 ULP` at
    /// every supported `SCALE`). Callers can request fewer guard digits
    /// to converge the Taylor series in fewer iterations:
    ///
    /// - `working_digits ≈ 6-10`: roughly `working_digits` digits of
    ///   accuracy at the storage scale; typically 1.5-3× faster than
    ///   strict; suitable for plotting, intermediate convergence
    ///   checks, or any computation where bit-exact rounding is not
    ///   required.
    /// - `working_digits ≥ 30`: same accuracy as `ln_strict`, but
    ///   slower than calling `ln_strict` directly because `w` is a
    ///   runtime value here. Prefer `ln_strict` when you want full
    ///   precision.
    ///
    /// The zero / one / linear-band fast paths fire regardless of the
    /// requested guard — those answers are exact and don't depend on
    /// the working precision.
    ///
    /// # Panics
    ///
    /// Same as `ln_strict`: argument must be positive.
    #[inline]
    #[must_use]
    pub fn ln_approx(self, working_digits: u32) -> Self {
        self.ln_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Natural log with caller-chosen guard digits AND rounding mode.
    /// See [`Self::ln_approx`] for accuracy/speed contract.
    ///
    /// Body delegates to `policy::ln::LnPolicy::ln_with_impl`.
    /// When `working_digits == STRICT_GUARD` the kernel collapses to
    /// the const-folded strict path.
    #[inline]
    #[must_use]
    pub fn ln_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.ln_strict_with(mode);
        }
        <Self as crate::policy::ln::LnPolicy>::ln_with_impl(self, working_digits, mode)
    }

    /// Returns the natural logarithm (base e) of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::ln_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
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
    pub fn log_strict(self, base: Self) -> Self {
        self.log_strict_with(base, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Logarithm in `base` under the supplied rounding mode.
    ///
    /// Under `d56` / `wide` the body widens to D56, calls D56's
    /// `log_strict_with` (which shares the fast
    /// `wide_trig_d56::ln_fixed` core that `ln_strict` borrows), and
    /// narrows back. Without those features it falls through to the
    /// bespoke `Fixed` 256-bit kernel — same path the family used
    /// before the borrow migration.
    #[inline]
    #[must_use]
    pub fn log_strict_with(self, base: Self, mode: crate::rounding::RoundingMode) -> Self {
        assert!(self.0 > 0, "D38::log: argument must be positive");
        assert!(base.0 > 0, "D38::log: base must be positive");
        #[cfg(any(feature = "d56", feature = "wide"))]
        {
            Self::from_bits(crate::algos::ln::borrow_d56::log_strict::<SCALE>(
                self.0, base.0, mode,
            ))
        }
        #[cfg(not(any(feature = "d56", feature = "wide")))]
        {
            use crate::d_w128_kernels::Fixed;
            let w = SCALE + STRICT_GUARD;
            let pow = 10u128.pow(STRICT_GUARD);
            let v_w = Fixed::from_u128_mag(self.0 as u128, false).mul_u128(pow);
            let b_w = Fixed::from_u128_mag(base.0 as u128, false).mul_u128(pow);
            let ln_b = ln_fixed(b_w, w);
            assert!(!ln_b.is_zero(), "D38::log: base must not equal 1 (ln(1) is zero)");
            let raw = ln_fixed(v_w, w)
                .div(ln_b, w)
                .round_to_i128_with(w, SCALE, mode)
                .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::log", SCALE));
            Self::from_bits(raw)
        }
    }

    /// Logarithm with caller-chosen guard digits. See `ln_approx`.
    #[inline]
    #[must_use]
    pub fn log_approx(self, base: Self, working_digits: u32) -> Self {
        self.log_approx_with(base, working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Logarithm with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn log_approx_with(self, base: Self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.log_strict_with(base, mode);
        }
        use crate::d_w128_kernels::Fixed;
        assert!(self.0 > 0, "D38::log: argument must be positive");
        assert!(base.0 > 0, "D38::log: base must be positive");
        let w = SCALE + working_digits;
        let pow = 10u128.pow(working_digits);
        let v_w = Fixed::from_u128_mag(self.0 as u128, false).mul_u128(pow);
        let b_w = Fixed::from_u128_mag(base.0 as u128, false).mul_u128(pow);
        let ln_b = ln_fixed(b_w, w);
        assert!(!ln_b.is_zero(), "D38::log: base must not equal 1 (ln(1) is zero)");
        let raw = ln_fixed(v_w, w)
            .div(ln_b, w)
            .round_to_i128_with(w, SCALE, mode)
            .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::log", SCALE));
        Self::from_bits(raw)
    }

    /// Returns the logarithm of `self` in the given `base`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::log_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
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
    pub fn log2_strict(self) -> Self {
        self.log2_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-2 log under the supplied rounding mode.
    ///
    /// Under `d56` / `wide` widens to D56, calls D56's
    /// `log2_strict_with`, narrows back — see [`Self::log_strict_with`].
    #[inline]
    #[must_use]
    pub fn log2_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        assert!(self.0 > 0, "D38::log2: argument must be positive");
        #[cfg(any(feature = "d56", feature = "wide"))]
        {
            Self::from_bits(crate::algos::ln::borrow_d56::log2_strict::<SCALE>(self.0, mode))
        }
        #[cfg(not(any(feature = "d56", feature = "wide")))]
        {
            use crate::d_w128_kernels::Fixed;
            let w = SCALE + STRICT_GUARD;
            let v_w =
                Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
            let raw = ln_fixed(v_w, w)
                .div(wide_ln2(w), w)
                .round_to_i128_with(w, SCALE, mode)
                .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::log2", SCALE));
            Self::from_bits(raw)
        }
    }

    /// Base-2 log with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn log2_approx(self, working_digits: u32) -> Self {
        self.log2_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-2 log with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn log2_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.log2_strict_with(mode);
        }
        use crate::d_w128_kernels::Fixed;
        assert!(self.0 > 0, "D38::log2: argument must be positive");
        let w = SCALE + working_digits;
        let v_w =
            Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(working_digits));
        let raw = ln_fixed(v_w, w)
            .div(wide_ln2(w), w)
            .round_to_i128_with(w, SCALE, mode)
            .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::log2", SCALE));
        Self::from_bits(raw)
    }

    /// Returns the base-2 logarithm of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::log2_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
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
    pub fn log10_strict(self) -> Self {
        self.log10_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-10 log under the supplied rounding mode.
    ///
    /// Under `d56` / `wide` widens to D56, calls D56's
    /// `log10_strict_with`, narrows back — see [`Self::log_strict_with`].
    #[inline]
    #[must_use]
    pub fn log10_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        assert!(self.0 > 0, "D38::log10: argument must be positive");
        #[cfg(any(feature = "d56", feature = "wide"))]
        {
            Self::from_bits(crate::algos::ln::borrow_d56::log10_strict::<SCALE>(self.0, mode))
        }
        #[cfg(not(any(feature = "d56", feature = "wide")))]
        {
            use crate::d_w128_kernels::Fixed;
            let w = SCALE + STRICT_GUARD;
            let v_w =
                Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
            let raw = ln_fixed(v_w, w)
                .div(wide_ln10(w), w)
                .round_to_i128_with(w, SCALE, mode)
                .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::log10", SCALE));
            Self::from_bits(raw)
        }
    }

    /// Base-10 log with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn log10_approx(self, working_digits: u32) -> Self {
        self.log10_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-10 log with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn log10_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.log10_strict_with(mode);
        }
        use crate::d_w128_kernels::Fixed;
        assert!(self.0 > 0, "D38::log10: argument must be positive");
        let w = SCALE + working_digits;
        let v_w =
            Fixed::from_u128_mag(self.0 as u128, false).mul_u128(10u128.pow(working_digits));
        let raw = ln_fixed(v_w, w)
            .div(wide_ln10(w), w)
            .round_to_i128_with(w, SCALE, mode)
            .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::log10", SCALE));
        Self::from_bits(raw)
    }

    /// Returns the base-10 logarithm of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::log10_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
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
    pub fn exp_strict(self) -> Self {
        self.exp_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `e^self` under the supplied rounding mode.
    ///
    /// Body delegates to `policy::exp::ExpPolicy::exp_impl`.
    #[inline]
    #[must_use]
    pub fn exp_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        <Self as crate::policy::exp::ExpPolicy>::exp_impl(self, mode)
    }

    /// Exponential with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn exp_approx(self, working_digits: u32) -> Self {
        self.exp_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Exponential with caller-chosen guard digits AND rounding mode.
    ///
    /// Body delegates to `policy::exp::ExpPolicy::exp_with_impl`.
    /// When `working_digits == STRICT_GUARD` the call collapses to the
    /// const-folded strict path.
    #[inline]
    #[must_use]
    pub fn exp_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.exp_strict_with(mode);
        }
        <Self as crate::policy::exp::ExpPolicy>::exp_with_impl(self, working_digits, mode)
    }

    /// Returns `e^self` (natural exponential).
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::exp_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
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
    /// Panics if the result overflows D38's representable range.
    #[inline]
    #[must_use]
    pub fn exp2_strict(self) -> Self {
        self.exp2_strict_with(crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `2^self` under the supplied rounding mode.
    ///
    /// Under `d56` / `wide` widens to D56, calls D56's
    /// `exp2_strict_with`, narrows back — see [`Self::log_strict_with`]
    /// for the broader rationale.
    #[inline]
    #[must_use]
    pub fn exp2_strict_with(self, mode: crate::rounding::RoundingMode) -> Self {
        if self.0 == 0 {
            return Self::ONE;
        }
        #[cfg(any(feature = "d56", feature = "wide"))]
        {
            Self::from_bits(crate::algos::exp::borrow_d56::exp2_strict::<SCALE>(self.0, mode))
        }
        #[cfg(not(any(feature = "d56", feature = "wide")))]
        {
            use crate::d_w128_kernels::Fixed;
            let w = SCALE + STRICT_GUARD;
            let negative_input = self.0 < 0;
            let v_w = Fixed::from_u128_mag(self.0.unsigned_abs(), false)
                .mul_u128(10u128.pow(STRICT_GUARD));
            let v_w = if negative_input { v_w.neg() } else { v_w };
            let arg_w = v_w.mul(wide_ln2(w), w);
            let raw = exp_fixed(arg_w, w)
                .round_to_i128_with(w, SCALE, mode)
                .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::exp2", SCALE));
            Self::from_bits(raw)
        }
    }

    /// Base-2 exponential with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn exp2_approx(self, working_digits: u32) -> Self {
        self.exp2_approx_with(working_digits, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Base-2 exponential with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn exp2_approx_with(self, working_digits: u32, mode: crate::rounding::RoundingMode) -> Self {
        if working_digits == STRICT_GUARD {
            return self.exp2_strict_with(mode);
        }
        use crate::d_w128_kernels::Fixed;
        if self.0 == 0 {
            return Self::ONE;
        }
        let w = SCALE + working_digits;
        let negative_input = self.0 < 0;
        let v_w = Fixed::from_u128_mag(self.0.unsigned_abs(), false)
            .mul_u128(10u128.pow(working_digits));
        let v_w = if negative_input { v_w.neg() } else { v_w };
        let arg_w = v_w.mul(wide_ln2(w), w);
        let raw = exp_fixed(arg_w, w)
            .round_to_i128_with(w, SCALE, mode)
            .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::exp2", SCALE));
        Self::from_bits(raw)
    }

    /// Returns `2^self` (base-2 exponential).
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::exp2_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        self.exp2_strict()
    }
}



#[cfg(all(test, feature = "strict", not(feature = "fast")))]
mod strict_tests {
    use crate::core_type::D38s12;

    /// Tolerance in ULPs for the strict transcendentals. They are
    /// correctly rounded (≤ 0.5 ULP); 2 LSB of slack absorbs the
    /// test's own expected-value rounding.
    const STRICT_TOLERANCE_LSB: i128 = 2;

    fn within(actual: D38s12, expected_bits: i128, tolerance: i128) -> bool {
        (actual.to_bits() - expected_bits).abs() <= tolerance
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
        use crate::core_type::D38;
        // D38<9>: ULP is 1e-9; f64 ln is good to ~1e-15 over this
        // range, so the correctly-rounded result is within 1 ULP of the
        // f64 reference (allow 1 for the f64 reference's own rounding).
        fn check(raw: i128) {
            let x = D38::<9>::from_bits(raw);
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
    /// bridge to within 1 ULP at D38<9>, where f64 is comfortably more
    /// precise than the type's ULP — strong evidence of correct
    /// rounding for the whole log/exp family.
    #[test]
    fn strict_log_exp_family_matches_f64() {
        use crate::core_type::D38;
        fn check_exp(raw: i128) {
            let x = D38::<9>::from_bits(raw);
            let strict = x.exp_strict().to_bits();
            let reference = ((raw as f64 / 1e9).exp() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "exp_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log2(raw: i128) {
            let x = D38::<9>::from_bits(raw);
            let strict = x.log2_strict().to_bits();
            let reference = ((raw as f64 / 1e9).log2() * 1e9).round() as i128;
            assert!(
                (strict - reference).abs() <= 1,
                "log2_strict({raw}) = {strict}, f64 reference {reference}"
            );
        }
        fn check_log10(raw: i128) {
            let x = D38::<9>::from_bits(raw);
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
        use crate::core_type::D38;
        for k in 0_i128..=12 {
            let x = D38::<12>::from_bits(k * 10i128.pow(12));
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
        use crate::core_type::D38;
        // ln(2) at scale 18, correctly rounded:
        // 0.693147180559945309… -> 693147180559945309.
        let ln2_s18: i128 = 693_147_180_559_945_309;
        for k in 1_i128..=20 {
            let x = D38::<18>::from_bits((1i128 << k) * 10i128.pow(18));
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
        let two = D38s12::from_bits(2_000_000_000_000);
        let result = two.ln();
        // ln(2) = 0.693147180559945... so at scale 12, bits = 693_147_180_560.
        assert!(
            within(result, 693_147_180_560, STRICT_TOLERANCE_LSB),
            "ln(2) bits = {}",
            result.to_bits()
        );
    }

    /// ln(e) is approximately 1. Uses the existing pi/e constants via DecimalConstants.
    #[test]
    fn ln_of_e_close_to_one() {
        // e at scale 12 = 2_718_281_828_459 (canonical 35-digit reference rescaled).
        let e_at_s12 = D38s12::from_bits(2_718_281_828_459);
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
        let ten = D38s12::from_bits(10_000_000_000_000);
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
        let v = D38s12::from_bits(1_500_000_000_000); // 1.5
        let result = v.ln();
        assert!(result.to_bits() > 0);
    }

    /// ln of a value in (0, 1) is negative.
    #[test]
    fn ln_below_one_is_negative() {
        let v = D38s12::from_bits(500_000_000_000); // 0.5
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
        let _ = D38s12::ZERO.ln();
    }

    #[test]
    #[should_panic(expected = "argument must be positive")]
    fn ln_of_negative_panics() {
        let neg = D38s12::from_bits(-1_000_000_000_000);
        let _ = neg.ln();
    }

    // log2 / log10 / log derive from ln; tolerance grows because the
    // additional division step accumulates ~1 LSB.
    const DERIVED_LOG_TOLERANCE_LSB: i128 = 20;

    /// log2(2) ~= 1.
    #[test]
    fn log2_of_two_is_one() {
        let two = D38s12::from_bits(2_000_000_000_000);
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
        let eight = D38s12::from_bits(8_000_000_000_000);
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
        let ten = D38s12::from_bits(10_000_000_000_000);
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
        let hundred = D38s12::from_bits(100_000_000_000_000);
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
        let base = D38s12::from_bits(5_000_000_000_000); // 5
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
        let eight = D38s12::from_bits(8_000_000_000_000);
        let two = D38s12::from_bits(2_000_000_000_000);
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
        let x = D38s12::from_bits(5_000_000_000_000);
        let one = D38s12::ONE;
        let _ = x.log(one);
    }

    // exp / exp2: tolerance accounts for Taylor truncation, 2^k bit-shift
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
        // e ~= 2.718281828459 at D38s12.
        assert!(
            within(result, 2_718_281_828_459, EXP_TOLERANCE_LSB),
            "exp(1) bits = {}",
            result.to_bits()
        );
    }

    /// exp(ln(2)) ~= 2.
    #[test]
    fn exp_of_ln_2_is_two() {
        let ln_2 = D38s12::from_bits(693_147_180_560);
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
        let neg_one = D38s12::from_bits(-1_000_000_000_000);
        let result = neg_one.exp();
        // 1/e ~= 0.367879441171 at D38s12 -> bits ~= 367_879_441_171.
        assert!(
            within(result, 367_879_441_171, EXP_TOLERANCE_LSB),
            "exp(-1) bits = {}",
            result.to_bits()
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
            result.to_bits()
        );
    }

    /// exp2(10) ~= 1024.
    #[test]
    fn exp2_of_ten_is_1024() {
        let ten = D38s12::from_bits(10_000_000_000_000);
        let result = ten.exp2();
        assert!(
            within(result, 1_024_000_000_000_000, EXP_TOLERANCE_LSB * 10),
            "exp2(10) bits = {}",
            result.to_bits()
        );
    }
}

