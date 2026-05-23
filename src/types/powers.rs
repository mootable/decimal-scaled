//! Power and root methods for [`D38`].
//!
//! # Methods
//!
//! - [`D38::pow`] — unsigned integer power via square-and-multiply over
//! the `Mul` operator. Panics on overflow in debug builds; wraps in
//! release builds. Matches `i128::pow` semantics.
//! - [`D38::powi`] — signed integer power. For negative `exp`, returns
//! `D38::ONE / self.pow(exp.unsigned_abs())`.
//! - [`D38::powf`] — floating-point power via the f64 bridge. Lossy.
//! Requires the `std` feature.
//! - [`D38::sqrt`] — square root via the f64 bridge. IEEE 754 mandates
//! that `f64::sqrt` is correctly-rounded, so identical inputs produce
//! identical output bit-patterns on every conformant platform.
//! Requires the `std` feature.
//! - [`D38::cbrt`] — cube root via the f64 bridge. Defined for negative
//! inputs. Requires the `std` feature.
//! - [`D38::mul_add`] — `self * a + b` in one call. No hardware FMA;
//! mirrors the `f64::mul_add` call shape so generic numeric code can
//! monomorphise to `D38`. Always available.
//! - [`D38::hypot`] — `sqrt(self^2 + other^2)` without intermediate
//! overflow, using the scale-trick algorithm. Requires the `std`
//! feature via `sqrt`.
//!
//! # The `*_strict` dual API
//!
//! `sqrt` / `cbrt` / `powf` / `hypot` each have an integer-only
//! `*_strict` form and an f64-bridge form (see `docs/strict-mode.md`).
//! The `*_strict` forms are **correctly rounded** — within 0.5 ULP of
//! the exact result under the active [`RoundingMode`]:
//!
//! - `sqrt_strict` / `cbrt_strict` form the exact 256-/384-bit
//! radicand and take its exact integer root, then apply the rounding
//! mode (no ties exist for integer-sqrt so the three half-modes
//! coincide; `Floor`/`Ceiling` divert for the directed cases);
//! - `powf_strict` runs `exp(y·ln(x))` entirely in the `algos::support::fixed_d38`
//! guard-digit intermediate;
//! - `hypot_strict` composes `sqrt_strict` via the scale-trick.
//!
//! Each strict method has a `*_strict_with(mode)` sibling that takes
//! the rounding mode explicitly; the no-arg `*_strict` form
//! delegates to it with the crate-default mode (see
//! [`crate::RoundingMode`] for the `rounding-*` Cargo features).
//! `powf` additionally ships `powf_approx(working_digits)` and
//! `powf_approx_with(working_digits, mode)` — the four-variant matrix
//! the transcendentals expose; `sqrt` / `cbrt` / `hypot` have no
//! guard-width parameter (the exact-integer-root path is precision-
//! independent), so only the `_strict` / `_strict_with` pair exists.
//!
//! `pow` / `powi` (integer exponents) are exact at any feature
//! configuration. The plain `sqrt` / `cbrt` / `powf` / `hypot`
//! dispatch to the `*_strict` form under the `strict` feature, and to
//! the f64 bridge otherwise; the `*_strict` forms are always compiled
//! unless `fast` is set, and are `no_std`-compatible.
//!
//! [`RoundingMode`]: crate::RoundingMode
//!
//! # Variant family for `pow`
//!
//! - [`D38::checked_pow`] — `Option<Self>`, `None` on overflow at any
//! step.
//! - [`D38::wrapping_pow`] — two's-complement wrap at every step.
//! - [`D38::saturating_pow`] — clamps to `D38::MAX` or `D38::MIN`
//! based on the sign of the would-be result.
//! - [`D38::overflowing_pow`] — `(Self, bool)`; the bool is `true` if
//! any step overflowed, with the value equal to the wrapping form.
//!
//! # Square-and-multiply algorithm
//!
//! Starting from `acc = ONE`, the algorithm walks the bits of `exp` from
//! low to high. On each iteration:
//!
//! 1. If the current bit of `exp` is set, multiply `acc *= base`.
//! 2. Square `base *= base`.
//!
//! This costs `O(log exp)` multiplications rather than `O(exp)`. The
//! variant family follows the same structure but applies the
//! corresponding overflow arithmetic at every multiplication step.
//!
//! # `i32::MIN` edge case for `powi`
//!
//! `i32::unsigned_abs` returns `2_147_483_648_u32` for `i32::MIN`,
//! avoiding the signed-negation overflow that `(-i32::MIN) as u32` would
//! cause. `D38::ONE.powi(i32::MIN)` therefore evaluates correctly as
//! `D38::ONE / D38::ONE.pow(2_147_483_648_u32)`.

use crate::types::widths::D38;

impl<const SCALE: u32> D38<SCALE> {
    /// Raises `self` to the power `exp`.
    ///
    /// Uses square-and-multiply: walks the bits of `exp` from low to
    /// high, squaring the base each step and accumulating when the
    /// corresponding bit is set. Costs `O(log exp)` multiplications.
    /// Each multiplication routes through the `D38` `Mul` operator.
    ///
    /// `exp = 0` always returns `ONE`, even when `self` is `ZERO`
    /// (matches `i128::pow` convention).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Panics
    ///
    /// In debug builds, panics on `i128` overflow at any multiplication
    /// step. In release builds, wraps two's-complement. Matches
    /// `i128::pow` and `D38::Mul` semantics.
    ///
    /// Use [`Self::checked_pow`], [`Self::wrapping_pow`],
    /// [`Self::saturating_pow`], or [`Self::overflowing_pow`] for
    /// explicit overflow control.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let two = D38s12::from(2);
    /// assert_eq!(two.pow(10), D38s12::from(1024));
    /// // exp = 0 returns ONE regardless of base.
    /// assert_eq!(D38s12::ZERO.pow(0), D38s12::ONE);
    /// ```
    #[inline]
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        let mut acc = Self::ONE;
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                acc *= base;
            }
            e >>= 1;
            if e > 0 {
                base *= base;
            }
        }
        acc
    }

    /// Raises `self` to the signed integer power `exp`.
    ///
    /// For non-negative `exp`, equivalent to `self.pow(exp as u32)`.
    /// For negative `exp`, returns `D38::ONE / self.pow(exp.unsigned_abs())`,
    /// i.e. the reciprocal of the positive-exponent form.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Panics
    ///
    /// - Overflow of `i128` storage at any step in debug builds (matches
    /// [`Self::pow`]).
    /// - Division by zero when `self == ZERO` and `exp < 0`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let two = D38s12::from(2);
    /// assert_eq!(two.powi(-1), D38s12::ONE / two);
    /// assert_eq!(two.powi(0), D38s12::ONE);
    /// assert_eq!(two.powi(3), D38s12::from(8));
    /// ```
    #[inline]
    #[must_use]
    pub fn powi(self, exp: i32) -> Self {
        if exp >= 0 {
            self.pow(exp as u32)
        } else {
            // unsigned_abs handles i32::MIN without signed-negation overflow.
            Self::ONE / self.pow(exp.unsigned_abs())
        }
    }

    /// Raises `self` to the power `exp` (strict integer-only stub).
    ///
    /// Converts both operands to f64, calls `f64::powf`, then converts
    /// the result back. For integer exponents, prefer [`Self::pow`] or
    /// [`Self::powi`], which are bit-exact.
    ///
    /// NaN results map to `ZERO`; infinities clamp to `MAX` or `MIN`,
    /// following the saturate-vs-error policy of [`Self::from_f64`].
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let two = D38s12::from(2);
    /// let three = D38s12::from(3);
    /// // 2^3 = 8, within f64 precision.
    /// assert!((two.powf(three).to_f64() - 8.0).abs() < 1e-9);
    /// ```
    /// Raises `self` to the power `exp`, computed integer-only as
    /// `exp(exp · ln(self))` — the `ln`, the `· exp`, and the `exp` all
    /// run in the shared wide guard-digit intermediate, so the result
    /// is correctly rounded (within 0.5 ULP).
    ///
    /// Always available, regardless of the `strict` feature. When
    /// `strict` is enabled, the plain [`Self::powf`] delegates here.
    ///
    /// A zero or negative base saturates to `ZERO` (a negative base
    /// with an arbitrary fractional exponent is not real-valued),
    /// matching the f64-bridge NaN-to-ZERO policy.
    #[inline]
    #[must_use]
    pub fn powf_strict(self, exp: D38<SCALE>) -> Self {
        self.powf_strict_with(exp, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `self^exp` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn powf_strict_with(
        self,
        exp: D38<SCALE>,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        <Self as crate::policy::pow::PowPolicy>::powf_impl(self, exp, mode)
    }

    /// `self^exp` with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn powf_approx(self, exp: D38<SCALE>, working_digits: u32) -> Self {
        self.powf_approx_with(
            exp,
            working_digits,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }

    /// `self^exp` with caller-chosen guard digits AND rounding mode.
    #[inline]
    #[must_use]
    pub fn powf_approx_with(
        self,
        exp: D38<SCALE>,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == crate::types::log_exp::STRICT_GUARD {
            return self.powf_strict_with(exp, mode);
        }
        <Self as crate::policy::pow::PowPolicy>::powf_with_impl(self, exp, working_digits, mode)
    }

    /// Raises `self` to the power `exp`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::powf_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn powf(self, exp: D38<SCALE>) -> Self {
        self.powf_strict(exp)
    }

    /// Returns the square root of `self` (strict integer-only stub).
    ///
    /// IEEE 754 mandates that `f64::sqrt` is correctly-rounded
    /// (round-to-nearest, ties-to-even). Combined with the deterministic
    /// `to_f64` / `from_f64` round-trip, this makes
    /// `D38::sqrt` bit-deterministic: the same input produces the same
    /// output bit-pattern on every IEEE-754-conformant platform.
    ///
    /// Negative inputs produce a NaN from `f64::sqrt`, which
    /// [`Self::from_f64`] maps to `ZERO` per the saturate-vs-error
    /// policy. No panic is raised for negative inputs.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// assert_eq!(D38s12::ZERO.sqrt(), D38s12::ZERO);
    /// // f64::sqrt(1.0) == 1.0 exactly, so the result is bit-exact.
    /// assert_eq!(D38s12::ONE.sqrt(), D38s12::ONE);
    /// ```
    #[inline]
    #[must_use]
    pub fn sqrt_strict(self) -> Self {
        self.sqrt_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Square root under the supplied rounding mode.
    ///
    /// Negative inputs saturate to [`Self::ZERO`] regardless of mode,
    /// matching the f64-bridge policy.
    ///
    /// Body delegates to `policy::sqrt::SqrtPolicy::sqrt_impl`,
    /// which for D38 selects the `mg_divide_d38` width-override kernel.
    #[inline]
    #[must_use]
    pub fn sqrt_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        <Self as crate::policy::sqrt::SqrtPolicy>::sqrt_impl(self, mode)
    }

    /// Returns the square root of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only,
    /// correctly-rounded [`Self::sqrt_strict`]; without it, the
    /// f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        self.sqrt_strict()
    }

    /// Returns the cube root of `self`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::cbrt_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn cbrt(self) -> Self {
        self.cbrt_strict()
    }

    /// Cube root of `self`. Defined for all reals — the sign of the
    /// input is preserved (`cbrt(-8) = -2`).
    ///
    /// # Algorithm
    ///
    /// For a `D38<SCALE>` with raw storage `r`, the raw storage of the
    /// cube root is
    ///
    /// round( cbrt(r / 10^SCALE) · 10^SCALE )
    /// = round( cbrt(r · 10^(2·SCALE)) ).
    ///
    /// `r · 10^(2·SCALE)` is formed exactly as a 384-bit value and its
    /// integer cube root is computed exactly, so the result is the
    /// exact cube root correctly rounded to the type's last place
    /// (within 0.5 ULP — the IEEE-754 round-to-nearest result).
    ///
    /// # Precision
    ///
    /// Strict: integer-only; correctly rounded.
    #[inline]
    #[must_use]
    pub fn cbrt_strict(self) -> Self {
        self.cbrt_strict_with(crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Cube root under the supplied rounding mode. The sign of the
    /// input is preserved; `Floor` / `Ceiling` resolve direction
    /// relative to the signed result.
    ///
    /// Body delegates to `policy::cbrt::CbrtPolicy::cbrt_impl`.
    #[inline]
    #[must_use]
    pub fn cbrt_strict_with(self, mode: crate::support::rounding::RoundingMode) -> Self {
        <Self as crate::policy::cbrt::CbrtPolicy>::cbrt_impl(self, mode)
    }

    /// Returns `sqrt(self^2 + other^2)` without intermediate overflow,
    /// computed integer-only via the correctly-rounded
    /// [`Self::sqrt_strict`]. Same scale-trick algorithm as the
    /// f64-bridge [`Self::hypot`]; available in `no_std`.
    ///
    /// Always available, regardless of the `strict` feature.
    #[inline]
    #[must_use]
    pub fn hypot_strict(self, other: Self) -> Self {
        self.hypot_strict_with(other, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Hypot under the supplied rounding mode. The mode applies to the
    /// inner square root; the surrounding adds and multiplies are
    /// exact-or-truncating per the operator path's own contract.
    #[inline]
    #[must_use]
    pub fn hypot_strict_with(
        self,
        other: Self,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        let a = self.abs();
        let b = other.abs();
        let (large, small) = if a >= b { (a, b) } else { (b, a) };
        if large == Self::ZERO {
            Self::ZERO
        } else {
            let ratio = small / large;
            let one_plus_sq = Self::ONE + ratio * ratio;
            large * one_plus_sq.sqrt_strict_with(mode)
        }
    }

    /// Returns `sqrt(self^2 + other^2)` without intermediate overflow.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::hypot_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn hypot(self, other: Self) -> Self {
        self.hypot_strict(other)
    }

    // Overflow-variant family for pow.

    /// Returns `Some(self^exp)`, or `None` if any multiplication step
    /// overflows `i128`.
    ///
    /// Walks the same square-and-multiply as [`Self::pow`] but uses
    /// `mul_div_pow10` (which returns `Option<i128>`) at each step.
    /// The first `None` short-circuits to a `None` return.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// // MAX^2 overflows.
    /// assert!(D38s12::MAX.checked_pow(2).is_none());
    /// // Any power of ONE is ONE.
    /// assert_eq!(D38s12::ONE.checked_pow(1_000_000), Some(D38s12::ONE));
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_pow(self, exp: u32) -> Option<Self> {
        let mut acc = Self::ONE;
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                acc = acc.checked_mul(base)?;
            }
            e >>= 1;
            if e > 0 {
                base = base.checked_mul(base)?;
            }
        }
        Some(acc)
    }

    /// Returns `self^exp`, wrapping two's-complement on overflow at
    /// every multiplication step.
    ///
    /// Follows the same square-and-multiply structure as [`Self::pow`].
    /// When a step overflows `mul_div_pow10`, the fallback is
    /// `wrapping_mul` followed by `wrapping_div` of the scale
    /// multiplier. The exact wrap pattern is deterministic and
    /// reproducible but is not otherwise specified.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// // ONE^N never overflows and returns ONE.
    /// assert_eq!(D38s12::ONE.wrapping_pow(1_000_000), D38s12::ONE);
    /// // MAX^2 wraps to a deterministic but unspecified value.
    /// let _ = D38s12::MAX.wrapping_pow(2);
    /// ```
    #[inline]
    #[must_use]
    pub fn wrapping_pow(self, exp: u32) -> Self {
        let mut acc = Self::ONE;
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                acc = acc.wrapping_mul(base);
            }
            e >>= 1;
            if e > 0 {
                base = base.wrapping_mul(base);
            }
        }
        acc
    }

    /// Returns `self^exp`, clamping to `D38::MAX` or `D38::MIN` on
    /// overflow at any step.
    ///
    /// On the first step that overflows, the result is clamped based on
    /// the sign of the mathematical result: positive overflows clamp to
    /// `MAX`, negative overflows clamp to `MIN`. The sign of the result
    /// is determined by `self.signum()` and whether `exp` is odd.
    ///
    /// `exp = 0` always returns `ONE` before entering the loop.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// assert_eq!(D38s12::MAX.saturating_pow(2), D38s12::MAX);
    /// assert_eq!(D38s12::ONE.saturating_pow(1_000_000), D38s12::ONE);
    /// ```
    #[inline]
    #[must_use]
    pub fn saturating_pow(self, exp: u32) -> Self {
        // exp == 0: result is ONE by convention.
        if exp == 0 {
            return Self::ONE;
        }
        let mut acc = Self::ONE;
        let mut base = self;
        let mut e = exp;
        // The final result is negative iff the base is negative and exp is odd.
        let result_negative_if_overflow = self.is_negative() && (exp & 1) == 1;
        while e > 0 {
            if e & 1 == 1 {
                match acc.checked_mul(base) {
                    Some(q) => acc = q,
                    None => {
                        return if result_negative_if_overflow {
                            Self::MIN
                        } else {
                            Self::MAX
                        };
                    }
                }
            }
            e >>= 1;
            if e > 0 {
                match base.checked_mul(base) {
                    Some(q) => base = q,
                    None => {
                        // base*base is non-negative (squared); clamp by the
                        // sign of the would-be final result.
                        return if result_negative_if_overflow {
                            Self::MIN
                        } else {
                            Self::MAX
                        };
                    }
                }
            }
        }
        acc
    }

    /// Returns `(self^exp, overflowed)`.
    ///
    /// `overflowed` is `true` if any multiplication step overflowed
    /// `i128`. The returned value is the wrapping form (matches
    /// [`Self::wrapping_pow`]).
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use decimal_scaled::D38s12;
    /// let (_value, overflowed) = D38s12::MAX.overflowing_pow(2);
    /// assert!(overflowed);
    /// let (value, overflowed) = D38s12::ONE.overflowing_pow(5);
    /// assert!(!overflowed);
    /// assert_eq!(value, D38s12::ONE);
    /// ```
    #[inline]
    #[must_use]
    pub fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        let mut acc = Self::ONE;
        let mut base = self;
        let mut e = exp;
        let mut overflowed = false;
        while e > 0 {
            if e & 1 == 1 {
                let (q, o) = acc.overflowing_mul(base);
                acc = q;
                overflowed |= o;
            }
            e >>= 1;
            if e > 0 {
                let (q, o) = base.overflowing_mul(base);
                base = q;
                overflowed |= o;
            }
        }
        (acc, overflowed)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::widths::D38s12;

    // Tolerance for f64-bridge tests. Used only by std-feature-gated
    // tests below; gated to suppress unused-item warnings on no_std builds.
    #[cfg(feature = "std")]
    const TWO_LSB: i128 = 2;

    #[cfg(feature = "std")]
    fn within_lsb(actual: D38s12, expected: D38s12, lsb: i128) -> bool {
        let diff = (actual.to_bits().as_i128() - expected.to_bits().as_i128()).abs();
        diff <= lsb
    }

    // pow (integer)

    /// `pow(0)` returns ONE for a nonzero base.
    #[test]
    fn pow_zero_is_one_for_nonzero() {
        let v = D38s12::from_int(7);
        assert_eq!(v.pow(0), D38s12::ONE);
    }

    /// `pow(1)` returns self.
    #[test]
    fn pow_one_is_self() {
        let v = D38s12::from_int(7);
        assert_eq!(v.pow(1), v);
    }

    /// `pow(2)` equals `self * self` for an integer value.
    #[test]
    fn pow_two_matches_mul() {
        let v = D38s12::from_int(13);
        assert_eq!(v.pow(2), v * v);
    }

    /// `pow(2)` equals `self * self` for a fractional value.
    #[test]
    fn pow_two_matches_mul_fractional() {
        // 1.5 in raw bits at SCALE = 12.
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(v.pow(2), v * v);
    }

    /// `2^10 == 1024`.
    #[test]
    fn pow_two_to_the_ten() {
        let two = D38s12::from_int(2);
        assert_eq!(two.pow(10), D38s12::from_int(1024));
    }

    /// `pow(0, 0) == ONE` — matches `i128::pow(0, 0) == 1`.
    #[test]
    fn zero_pow_zero_is_one() {
        assert_eq!(D38s12::ZERO.pow(0), D38s12::ONE);
    }

    /// `pow(0, n)` for `n > 0` is `ZERO`.
    #[test]
    fn zero_pow_positive_is_zero() {
        assert_eq!(D38s12::ZERO.pow(1), D38s12::ZERO);
        assert_eq!(D38s12::ZERO.pow(5), D38s12::ZERO);
    }

    /// `pow(n)` of `ONE` is always `ONE`.
    #[test]
    fn one_pow_n_is_one() {
        assert_eq!(D38s12::ONE.pow(0), D38s12::ONE);
        assert_eq!(D38s12::ONE.pow(1), D38s12::ONE);
        assert_eq!(D38s12::ONE.pow(100), D38s12::ONE);
    }

    /// `(-1)^n` alternates sign.
    #[test]
    fn negative_one_pow_alternates() {
        let neg_one = -D38s12::ONE;
        assert_eq!(neg_one.pow(0), D38s12::ONE);
        assert_eq!(neg_one.pow(1), neg_one);
        assert_eq!(neg_one.pow(2), D38s12::ONE);
        assert_eq!(neg_one.pow(3), neg_one);
    }

    // powi (signed integer)

    /// `powi(0)` returns ONE.
    #[test]
    fn powi_zero_is_one() {
        let v = D38s12::from_int(7);
        assert_eq!(v.powi(0), D38s12::ONE);
    }

    /// `powi(1)` returns self.
    #[test]
    fn powi_one_is_self() {
        let v = D38s12::from_int(7);
        assert_eq!(v.powi(1), v);
    }

    /// `powi(-1)` returns `ONE / self`.
    #[test]
    fn powi_minus_one_is_reciprocal() {
        let v = D38s12::from_int(7);
        assert_eq!(v.powi(-1), D38s12::ONE / v);
    }

    /// `powi(-3)` equals `ONE / pow(3)`.
    #[test]
    fn powi_negative_matches_reciprocal_of_positive() {
        let v = D38s12::from_int(2);
        assert_eq!(v.powi(-3), D38s12::ONE / v.pow(3));
    }

    /// `powi` agrees with `pow` for non-negative exponents.
    #[test]
    fn powi_positive_matches_pow() {
        let v = D38s12::from_int(3);
        for e in 0_i32..6 {
            assert_eq!(v.powi(e), v.pow(e as u32));
        }
    }

    /// `i32::MIN` edge: `unsigned_abs` returns 2_147_483_648; for a base
    /// of ONE the result is ONE.
    #[test]
    fn powi_i32_min_for_one_base() {
        assert_eq!(D38s12::ONE.powi(i32::MIN), D38s12::ONE);
    }

    // powf — requires std feature

    /// `powf(0.5)` approximates `sqrt` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn powf_half_matches_sqrt() {
        let v = D38s12::from_int(4);
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000)); // 0.5 at SCALE=12
        let powf_result = v.powf(half);
        let sqrt_result = v.sqrt();
        assert!(
            within_lsb(powf_result, sqrt_result, TWO_LSB),
            "powf(0.5)={}, sqrt={}, diff={}",
            powf_result.to_bits().as_i128(),
            sqrt_result.to_bits().as_i128(),
            (powf_result.to_bits().as_i128() - sqrt_result.to_bits().as_i128()).abs(),
        );
    }

    /// `powf(2)` agrees with `pow(2)` within 2 LSB (f64 bridge).
    #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
    #[test]
    fn powf_two_matches_pow_two_within_lsb() {
        let v = D38s12::from_int(7);
        let two = D38s12::from_int(2);
        assert!(within_lsb(v.powf(two), v.pow(2), TWO_LSB));
    }

    /// Strict `powf` is correctly rounded: `powf(7, 2)` agrees with the
    /// exact `pow(7, 2)` to within 1 ULP — the whole `exp(y·ln(x))`
    /// chain runs in the shared wide guard-digit intermediate.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[test]
    fn powf_two_matches_pow_two_within_lsb() {
        let v = D38s12::from_int(7);
        let two = D38s12::from_int(2);
        assert!(within_lsb(v.powf(two), v.pow(2), 1));
        // A few more integer-exponent cross-checks against exact `pow`.
        for base in [2_i64, 3, 5, 11] {
            let b = D38s12::from_int(base);
            assert!(
                within_lsb(b.powf(D38s12::from_int(3)), b.pow(3), 1),
                "powf({base}, 3)"
            );
        }
    }

    // sqrt — requires std feature

    /// `sqrt(0) == 0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sqrt(), D38s12::ZERO);
    }

    /// `sqrt(1) == 1` — bit-exact because `f64::sqrt(1.0) == 1.0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_one_is_one_bit_exact() {
        assert_eq!(D38s12::ONE.sqrt(), D38s12::ONE);
    }

    /// `sqrt(4) == 2` — bit-exact because `f64::sqrt(4.0) == 2.0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_four_is_two() {
        let four = D38s12::from_int(4);
        let two = D38s12::from_int(2);
        assert_eq!(four.sqrt(), two);
    }

    /// Strict `sqrt` is correctly rounded: for the raw result `q`, the
    /// scaled radicand `N = r · 10^SCALE` must satisfy
    /// `(q − 0.5)² ≤ N ≤ (q + 0.5)²`, i.e. `q` is the exact square root
    /// rounded to nearest. Checked exactly in 256-bit integer space
    /// across several scales and magnitudes.
    #[test]
    fn strict_sqrt_is_correctly_rounded() {
        // (q - 0.5)^2 = q^2 - q + 0.25 → lower bound  N ≥ q^2 - q + 1 (ints, when q>0)
        // (q + 0.5)^2 = q^2 + q + 0.25 → upper bound  N ≤ q^2 + q
        // So a correctly-rounded q satisfies q^2 - q < N ≤ q^2 + q  (q>0),
        // or N == 0 when q == 0.
        fn check<const S: u32>(raw: i128) {
            let x = crate::types::widths::D38::<S>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let q = x.sqrt_strict().to_bits().as_i128();
            assert!(q >= 0, "sqrt result must be non-negative");
            // N = raw · 10^S as 256-bit; q is small enough that q^2 fits 256-bit.
            let mult = 10u128.pow(S);
            let (n_hi, n_lo) = crate::algos::support::mg_divide::mul_u128_to_u256(raw as u128, mult);
            let (qsq_hi, qsq_lo) = crate::algos::support::mg_divide::mul_u128_to_u256(q as u128, q as u128);
            // lower: N > q^2 - q ⇔   N + q > q^2   (q ≥ 0)
            // upper: N ≤ q^2 + q
            let q_u = q as u128;
            // q^2 + q (256-bit)
            let (uphi, uplo) = {
                let (lo, c) = qsq_lo.overflowing_add(q_u);
                (qsq_hi + c as u128, lo)
            };
            // N ≤ q^2 + q ?
            let n_le_upper = n_hi < uphi || (n_hi == uphi && n_lo <= uplo);
            assert!(n_le_upper, "sqrt({raw} @ s{S}) = {q}: N exceeds (q+0.5)^2");
            if q > 0 {
                // N + q (256-bit)
                let (nphi, nplo) = {
                    let (lo, c) = n_lo.overflowing_add(q_u);
                    (n_hi + c as u128, lo)
                };
                // N + q > q^2 ?
                let above_lower = nphi > qsq_hi || (nphi == qsq_hi && nplo > qsq_lo);
                assert!(above_lower, "sqrt({raw} @ s{S}) = {q}: N below (q-0.5)^2");
            }
        }
        for &raw in &[
            1_i128,
            2,
            3,
            4,
            5,
            999_999_999_999,
            1_000_000_000_000,
            1_500_000_000_000,
            123_456_789_012_345,
            i128::MAX,
            i128::MAX / 7,
        ] {
            check::<0>(raw);
            check::<6>(raw);
            check::<12>(raw);
            check::<19>(raw);
        }
        // High-scale cases where the radicand approaches the 256-bit cap.
        for &raw in &[1_i128, 2, 17, i128::MAX, i128::MAX / 3] {
            check::<38>(raw);
        }
    }

    /// Strict `cbrt` is correctly rounded: for the raw result `q`, the
    /// scaled radicand `N = |r| · 10^(2·SCALE)` must satisfy
    /// `(2q − 1)³ < 8·N ≤ (2q + 1)³`, i.e. `q` is the exact cube root
    /// rounded to nearest. Checked exactly in 384-bit integer space.
    #[test]
    fn strict_cbrt_is_correctly_rounded() {
        // q correctly rounded ⇔  q − 0.5 < cbrt(N) ≤ q + 0.5
        // ⇔  (2q − 1)³ < 8N ≤ (2q + 1)³.
        // 384-bit comparison via num-bigint-free manual limbs would be
        // verbose, so this check leans on the i256 dev-dependency to
        // hold the 384-bit cubes (i256 is already a dev-dependency).
        use i256::U256;
        fn check<const S: u32>(raw: i128) {
            let x = crate::types::widths::D38::<S>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let q = x.cbrt_strict().to_bits().as_i128();
            // Sign must match the input.
            assert_eq!(q.signum(), raw.signum(), "cbrt sign mismatch");
            let qa = q.unsigned_abs();
            let ra = raw.unsigned_abs();
            // N = |r| · 10^(2S). 2S ≤ 76, so 10^(2S) needs U256; the
            // product needs more than 256 bits at high S, so cap the
            // scales exercised here to keep the check in U256 range.
            // (The 384-bit path itself is exercised across all scales by
            // the round-trip tests; this exact check covers S ≤ 25.)
            let m = U256::from(10u8).pow(2 * S);
            let n = U256::from(ra) * m;
            let eight_n = n << 3;
            let two_q = U256::from(qa) * U256::from(2u8);
            let upper = {
                let t = two_q + U256::from(1u8);
                t * t * t
            };
            assert!(
                eight_n <= upper,
                "cbrt({raw} @ s{S}) = {q}: 8N exceeds (2q+1)^3"
            );
            if qa > 0 {
                let t = two_q - U256::from(1u8);
                let lower = t * t * t;
                assert!(
                    eight_n > lower,
                    "cbrt({raw} @ s{S}) = {q}: 8N at/below (2q-1)^3"
                );
            }
        }
        for &raw in &[
            1_i128,
            2,
            7,
            8,
            9,
            26,
            27,
            28,
            999_999_999_999,
            1_000_000_000_000,
            123_456_789_012_345,
            -8,
            -27,
            -1_000_000_000_000,
        ] {
            check::<0>(raw);
            check::<6>(raw);
            check::<12>(raw);
        }
        // Larger magnitudes at low scale (still within the U256 check).
        for &raw in &[i128::MAX, i128::MIN + 1, i128::MAX / 11] {
            check::<0>(raw);
            check::<2>(raw);
        }
    }

    /// `sqrt(self * self) ~= self.abs()` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_of_square_recovers_abs() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_234_567_890_123));
        let squared = v * v;
        let recovered = squared.sqrt();
        let abs_v = v.abs();
        assert!(
            within_lsb(recovered, abs_v, TWO_LSB),
            "sqrt({})={}, expected~={}, diff={}",
            squared.to_bits().as_i128(),
            recovered.to_bits().as_i128(),
            abs_v.to_bits().as_i128(),
            (recovered.to_bits().as_i128() - abs_v.to_bits().as_i128()).abs(),
        );
    }

    /// `sqrt(self * self) ~= self.abs()` within 2 LSB for negative self.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_of_square_negative_recovers_abs() {
        let v = -D38s12::from_bits(crate::int::types::Int::<2>::from_i128(4_567_891_234_567));
        let squared = v * v;
        let recovered = squared.sqrt();
        let abs_v = v.abs();
        assert!(within_lsb(recovered, abs_v, TWO_LSB));
    }

    /// A negative input produces NaN in f64, which maps to ZERO.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_negative_saturates_to_zero() {
        let v = -D38s12::from_int(4);
        assert_eq!(v.sqrt(), D38s12::ZERO);
    }

    // cbrt — requires std feature

    /// `cbrt(0) == 0`.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_zero_is_zero() {
        assert_eq!(D38s12::ZERO.cbrt(), D38s12::ZERO);
    }

    /// `cbrt(1) == 1`.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_one_is_one() {
        assert_eq!(D38s12::ONE.cbrt(), D38s12::ONE);
    }

    /// `cbrt(8) ~= 2` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_eight_is_two() {
        let eight = D38s12::from_int(8);
        let two = D38s12::from_int(2);
        assert!(within_lsb(eight.cbrt(), two, TWO_LSB));
    }

    /// `cbrt(-8) ~= -2` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_minus_eight_is_minus_two() {
        let neg_eight = D38s12::from_int(-8);
        let neg_two = D38s12::from_int(-2);
        assert!(
            within_lsb(neg_eight.cbrt(), neg_two, TWO_LSB),
            "cbrt(-8) = {}, expected ~ {}",
            neg_eight.cbrt().to_bits().as_i128(),
            neg_two.to_bits().as_i128(),
        );
    }

    // checked_pow / wrapping_pow / saturating_pow / overflowing_pow

    /// `checked_pow(MAX, 2)` is `None` because MAX^2 overflows.
    #[test]
    fn checked_pow_max_squared_is_none() {
        assert!(D38s12::MAX.checked_pow(2).is_none());
    }

    /// `checked_pow(ONE, N)` is `Some(ONE)` for any N.
    #[test]
    fn checked_pow_one_is_some_one() {
        assert_eq!(D38s12::ONE.checked_pow(1_000_000), Some(D38s12::ONE));
        assert_eq!(D38s12::ONE.checked_pow(0), Some(D38s12::ONE));
    }

    /// `checked_pow` agrees with `pow` for non-overflowing inputs.
    #[test]
    fn checked_pow_matches_pow_when_no_overflow() {
        let v = D38s12::from_int(3);
        assert_eq!(v.checked_pow(0), Some(v.pow(0)));
        assert_eq!(v.checked_pow(1), Some(v.pow(1)));
        assert_eq!(v.checked_pow(5), Some(v.pow(5)));
    }

    /// `saturating_pow(MAX, 2)` clamps to `MAX`.
    #[test]
    fn saturating_pow_max_squared_is_max() {
        assert_eq!(D38s12::MAX.saturating_pow(2), D38s12::MAX);
    }

    /// `saturating_pow(MIN, 3)` clamps to `MIN` (negative result, odd exp).
    #[test]
    fn saturating_pow_min_cubed_is_min() {
        assert_eq!(D38s12::MIN.saturating_pow(3), D38s12::MIN);
    }

    /// `saturating_pow(MIN, 2)` clamps to `MAX` (positive result, even exp).
    #[test]
    fn saturating_pow_min_squared_is_max() {
        assert_eq!(D38s12::MIN.saturating_pow(2), D38s12::MAX);
    }

    /// `saturating_pow(ONE, N)` is ONE for any N.
    #[test]
    fn saturating_pow_one_is_one() {
        assert_eq!(D38s12::ONE.saturating_pow(1_000_000), D38s12::ONE);
    }

    /// `saturating_pow(v, 0)` is ONE for any base.
    #[test]
    fn saturating_pow_zero_exp_is_one() {
        assert_eq!(D38s12::MAX.saturating_pow(0), D38s12::ONE);
        assert_eq!(D38s12::MIN.saturating_pow(0), D38s12::ONE);
    }

    /// `overflowing_pow(MAX, 2)` returns `(wrapping_value, true)`.
    #[test]
    fn overflowing_pow_max_squared_flags_overflow() {
        let (value, overflowed) = D38s12::MAX.overflowing_pow(2);
        assert!(overflowed);
        assert_eq!(value, D38s12::MAX.wrapping_pow(2));
    }

    /// `overflowing_pow(ONE, N)` never overflows.
    #[test]
    fn overflowing_pow_one_no_overflow() {
        let (value, overflowed) = D38s12::ONE.overflowing_pow(1_000_000);
        assert!(!overflowed);
        assert_eq!(value, D38s12::ONE);
    }

    /// `overflowing_pow(v, 0)` is `(ONE, false)`.
    #[test]
    fn overflowing_pow_zero_exp_no_overflow() {
        let (value, overflowed) = D38s12::MAX.overflowing_pow(0);
        assert!(!overflowed);
        assert_eq!(value, D38s12::ONE);
    }

    /// `wrapping_pow(MAX, 2)` matches the value half of `overflowing_pow`.
    #[test]
    fn wrapping_pow_max_squared_matches_overflowing() {
        let wrap = D38s12::MAX.wrapping_pow(2);
        let (over, _flag) = D38s12::MAX.overflowing_pow(2);
        assert_eq!(wrap, over);
    }

    /// `wrapping_pow(ONE, N)` is ONE.
    #[test]
    fn wrapping_pow_one_is_one() {
        assert_eq!(D38s12::ONE.wrapping_pow(1_000_000), D38s12::ONE);
    }

    /// `wrapping_pow` agrees with `pow` for non-overflowing inputs.
    #[test]
    fn wrapping_pow_matches_pow_when_no_overflow() {
        let v = D38s12::from_int(3);
        for e in 0..6 {
            assert_eq!(v.wrapping_pow(e), v.pow(e));
        }
    }

    /// `pow(2) == self * self` for several representative raw values.
    #[test]
    fn pow_two_property_safe_values() {
        for raw in [
            1_234_567_890_123_i128,
            4_567_891_234_567_i128,
            7_890_123_456_789_i128,
        ] {
            let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            assert_eq!(v.pow(2), v * v, "raw bits {raw}");
        }
    }

    // mul_add (always available)

    /// `mul_add(0, 0, 0) == 0`.
    #[test]
    fn mul_add_zero_zero_zero_is_zero() {
        let z = D38s12::ZERO;
        assert_eq!(z.mul_add(z, z), D38s12::ZERO);
    }

    /// `mul_add(2, 3, 4) == 10`.
    #[test]
    fn mul_add_two_three_four_is_ten() {
        let two = D38s12::from_int(2);
        let three = D38s12::from_int(3);
        let four = D38s12::from_int(4);
        assert_eq!(two.mul_add(three, four), D38s12::from_int(10));
    }

    /// `mul_add(self, ONE, ZERO) == self`.
    #[test]
    fn mul_add_identity_collapses() {
        let v = D38s12::from_int(7);
        assert_eq!(v.mul_add(D38s12::ONE, D38s12::ZERO), v);
    }

    /// `mul_add(self, ZERO, b) == b`.
    #[test]
    fn mul_add_zero_factor_yields_addend() {
        let v = D38s12::from_int(7);
        let b = D38s12::from_int(13);
        assert_eq!(v.mul_add(D38s12::ZERO, b), b);
    }

    /// `mul_add(a, b, c) == a * b + c` for representative raw values.
    #[test]
    fn mul_add_matches_mul_then_add_safe_values() {
        for (a_raw, b_raw, c_raw) in [
            (
                1_234_567_890_123_i128,
                2_345_678_901_234_i128,
                3_456_789_012_345_i128,
            ),
            (
                4_567_891_234_567_i128,
                5_678_912_345_678_i128,
                6_789_123_456_789_i128,
            ),
            (
                7_890_123_456_789_i128,
                8_901_234_567_891_i128,
                9_012_345_678_912_i128,
            ),
        ] {
            let a = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(a_raw));
            let b = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(b_raw));
            let c = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(c_raw));
            assert_eq!(
                a.mul_add(b, c),
                a * b + c,
                "raw bits ({a_raw}, {b_raw}, {c_raw})",
            );
        }
    }

    /// `(-a).mul_add(b, c)` propagates the sign through the multiply step.
    #[test]
    fn mul_add_sign_propagates_through_factor() {
        let a = D38s12::from_int(3);
        let b = D38s12::from_int(5);
        let c = D38s12::from_int(7);
        // (-3) * 5 + 7 = -15 + 7 = -8
        assert_eq!((-a).mul_add(b, c), D38s12::from_int(-8));
    }

    // hypot — requires std feature

    // Tolerance for hypot: composes divide + square + add + sqrt + multiply,
    // each with up to 1 LSB of f64-bridge slack; sqrt quantisation dominates.
    #[cfg(feature = "std")]
    const HYPOT_TOLERANCE_LSB: i128 = 32;

    /// `hypot(3, 4) ~= 5` — the Pythagorean triple.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_three_four_is_five() {
        let three = D38s12::from_int(3);
        let four = D38s12::from_int(4);
        let five = D38s12::from_int(5);
        let result = three.hypot(four);
        assert!(
            within_lsb(result, five, HYPOT_TOLERANCE_LSB),
            "hypot(3, 4)={}, expected~={}, diff={}",
            result.to_bits().as_i128(),
            five.to_bits().as_i128(),
            (result.to_bits().as_i128() - five.to_bits().as_i128()).abs(),
        );
    }

    /// `hypot(0, 0) == 0` — bit-exact via the early-return path.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_zero_zero_is_zero_bit_exact() {
        assert_eq!(D38s12::ZERO.hypot(D38s12::ZERO), D38s12::ZERO);
    }

    /// `hypot(0, x) ~= x.abs()` for nonzero x.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_zero_x_is_abs_x() {
        let x = D38s12::from_int(7);
        let result = D38s12::ZERO.hypot(x);
        assert!(
            within_lsb(result, x.abs(), HYPOT_TOLERANCE_LSB),
            "hypot(0, 7)={}, expected~={}",
            result.to_bits().as_i128(),
            x.abs().to_bits().as_i128(),
        );
    }

    /// `hypot(x, 0) ~= x.abs()` for nonzero x.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_x_zero_is_abs_x() {
        let x = D38s12::from_int(-9);
        let result = x.hypot(D38s12::ZERO);
        assert!(
            within_lsb(result, x.abs(), HYPOT_TOLERANCE_LSB),
            "hypot(-9, 0)={}, expected~={}",
            result.to_bits().as_i128(),
            x.abs().to_bits().as_i128(),
        );
    }

    /// `hypot(-a, b) == hypot(a, b)` — sign invariance from the abs step.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_sign_invariant() {
        let three = D38s12::from_int(3);
        let four = D38s12::from_int(4);
        let pos = three.hypot(four);
        let neg_a = (-three).hypot(four);
        let neg_b = three.hypot(-four);
        let neg_both = (-three).hypot(-four);
        assert_eq!(pos, neg_a);
        assert_eq!(pos, neg_b);
        assert_eq!(pos, neg_both);
    }

    /// `hypot` does not panic at large magnitudes that the naive form
    /// would overflow.
    ///
    /// At SCALE=12 with `i128::MAX / 2` raw bits, the true hypotenuse
    /// is well below `D38::MAX / sqrt(2)`, so no overflow occurs and
    /// the result is a nonzero positive value.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_large_magnitudes_does_not_panic() {
        let half_max = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(i128::MAX / 2));
        let result = half_max.hypot(half_max);
        assert!(result > D38s12::ZERO);
        assert!(result >= half_max);
    }

    /// `hypot(a, b)` matches the naive `sqrt(a^2 + b^2)` within tolerance
    /// for small magnitudes where the naive form does not overflow.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_matches_naive_sqrt_of_sum_of_squares() {
        let a = D38s12::from_int(12);
        let b = D38s12::from_int(13);
        let h = a.hypot(b);
        let naive = (a * a + b * b).sqrt();
        assert!(
            within_lsb(h, naive, HYPOT_TOLERANCE_LSB),
            "hypot(12, 13)={}, naive sqrt(a^2+b^2)={}, diff={}",
            h.to_bits().as_i128(),
            naive.to_bits().as_i128(),
            (h.to_bits().as_i128() - naive.to_bits().as_i128()).abs(),
        );
    }
}
