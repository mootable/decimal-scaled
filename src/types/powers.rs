// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
//! - `powf_strict` runs `exp(y·ln(x))` entirely in the `algos::support::fixed`
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


impl<const SCALE: u32> crate::D<crate::int::types::Int<2>, SCALE> {
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
    pub fn powf_strict(self, exp: crate::D<crate::int::types::Int<2>, SCALE>) -> Self {
        self.powf_strict_with(exp, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// `self^exp` under the supplied rounding mode.
    #[inline]
    #[must_use]
    pub fn powf_strict_with(
        self,
        exp: crate::D<crate::int::types::Int<2>, SCALE>,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        Self::from_bits(crate::policy::pow::dispatch::<_, SCALE>(self.to_bits(), exp.to_bits(), mode))
    }

    /// `self^exp` with caller-chosen guard digits.
    #[inline]
    #[must_use]
    pub fn powf_approx(self, exp: crate::D<crate::int::types::Int<2>, SCALE>, working_digits: u32) -> Self {
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
        exp: crate::D<crate::int::types::Int<2>, SCALE>,
        working_digits: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        if working_digits == crate::types::log_exp::STRICT_GUARD {
            return self.powf_strict_with(exp, mode);
        }
        Self::from_bits(crate::policy::pow::dispatch_with::<_, SCALE>(self.to_bits(), exp.to_bits(), working_digits, mode))
    }

    /// Raises `self` to the power `exp`.
    ///
    /// With the `strict` feature enabled this is the integer-only
    /// [`Self::powf_strict`]; without it, the f64-bridge form.
    #[cfg(all(feature = "strict", not(feature = "fast")))]
    #[inline]
    #[must_use]
    pub fn powf(self, exp: crate::D<crate::int::types::Int<2>, SCALE>) -> Self {
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
        Self(crate::policy::sqrt::dispatch::<_, SCALE>(self.0, mode))
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
        Self(crate::policy::cbrt::dispatch::<_, SCALE>(self.0, mode))
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
    ///
    /// Body delegates to `policy::hypot::HypotPolicy::hypot_impl`.
    #[inline]
    #[must_use]
    pub fn hypot_strict_with(
        self,
        other: Self,
        mode: crate::support::rounding::RoundingMode,
    ) -> Self {
        Self(crate::policy::hypot::dispatch::<_, SCALE>(self.0, other.0, mode))
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
            let x = crate::D::<crate::int::types::Int<2>, S>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
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
            let x = crate::D::<crate::int::types::Int<2>, S>::from_bits(crate::int::types::Int::<2>::from_i128(raw));
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
}
