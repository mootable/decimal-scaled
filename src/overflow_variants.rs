//! Overflow-aware arithmetic variants for [`D128`].
//!
//! Provides the standard Rust explicit-overflow method families as
//! inherent methods on [`D128<SCALE>`], covering six operations:
//!
//! - `add`, `sub`, `mul`, `div`, `rem` (binary)
//! - `neg` (unary)
//!
//! Each operation is available in four forms:
//!
//! - **`checked_*`** — returns `Option<Self>`; `None` on overflow or
//!   div-by-zero; never panics.
//! - **`wrapping_*`** — returns `Self` with two's-complement wrap on
//!   overflow; never panics. `div` and `rem` variants still panic on
//!   `rhs == ZERO` to match `i128::wrapping_div` / `i128::wrapping_rem`
//!   semantics.
//! - **`saturating_*`** — clamps to `MAX` / `MIN` on overflow; never
//!   panics. No `saturating_rem` is provided because remainder is always
//!   bounded by `|rhs|` and the standard library does not define one for
//!   primitive integers. `div` variants still panic on `rhs == ZERO`.
//! - **`overflowing_*`** — returns `(Self, bool)` where the `bool`
//!   indicates overflow and `Self` is the wrapping result. `div` and
//!   `rem` variants panic on `rhs == ZERO`.
//!
//! # Algorithm notes
//!
//! - `add`, `sub`, `neg`, `rem` variants delegate directly to the
//!   corresponding `i128` intrinsics on the raw storage field.
//! - `mul` variants route through the same widening multiply-then-divide
//!   helper (`crate::mg_divide::mul_div_pow10`) as the default `Mul`
//!   operator. The intermediate product uses 256-bit arithmetic and
//!   cannot observably overflow; the only failure mode is a final `i128`
//!   quotient that does not fit.
//! - `div` variants route through the same widening long-divide helper
//!   (`crate::mg_divide::div_pow10_div`) as the default `Div` operator.

use crate::core_type::D128;

impl<const SCALE: u32> D128<SCALE> {
    // Add

    /// Returns `self + rhs`, or `None` if the result overflows `i128`.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(1_000_000_000_000); // 1.0
    /// let b = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.checked_add(b), Some(D128e12::from_bits(3_000_000_000_000)));
    /// assert_eq!(D128e12::MAX.checked_add(D128e12::ONE), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Returns `self + rhs` with two's-complement wrap on overflow.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(1_000_000_000_000); // 1.0
    /// let b = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.wrapping_add(b), D128e12::from_bits(3_000_000_000_000));
    /// // Overflow wraps to MIN.
    /// assert_eq!(D128e12::MAX.wrapping_add(D128e12::from_bits(1)), D128e12::MIN);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    /// Returns `self + rhs`, clamped to `D128::MAX` or `D128::MIN` on overflow.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(1_000_000_000_000); // 1.0
    /// let b = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.saturating_add(b), D128e12::from_bits(3_000_000_000_000));
    /// assert_eq!(D128e12::MAX.saturating_add(D128e12::ONE), D128e12::MAX);
    /// assert_eq!(D128e12::MIN.saturating_add(-D128e12::ONE), D128e12::MIN);
    /// ```
    #[inline]
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Returns `(self + rhs, did_overflow)` where the value is the
    /// two's-complement wrapping result.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(1_000_000_000_000); // 1.0
    /// let b = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.overflowing_add(b), (D128e12::from_bits(3_000_000_000_000), false));
    /// assert_eq!(
    ///     D128e12::MAX.overflowing_add(D128e12::from_bits(1)),
    ///     (D128e12::MIN, true),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (v, ovf) = self.0.overflowing_add(rhs.0);
        (Self(v), ovf)
    }

    // Sub

    /// Returns `self - rhs`, or `None` if the result overflows `i128`.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let three = D128e12::from_bits(3_000_000_000_000);
    /// let two   = D128e12::from_bits(2_000_000_000_000);
    /// assert_eq!(three.checked_sub(two), Some(D128e12::ONE));
    /// assert_eq!(D128e12::MIN.checked_sub(D128e12::ONE), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Returns `self - rhs` with two's-complement wrap on overflow.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let three = D128e12::from_bits(3_000_000_000_000);
    /// let two   = D128e12::from_bits(2_000_000_000_000);
    /// assert_eq!(three.wrapping_sub(two), D128e12::ONE);
    /// // Underflow wraps to MAX.
    /// assert_eq!(D128e12::MIN.wrapping_sub(D128e12::from_bits(1)), D128e12::MAX);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// Returns `self - rhs`, clamped to `D128::MAX` or `D128::MIN` on overflow.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let three = D128e12::from_bits(3_000_000_000_000);
    /// let two   = D128e12::from_bits(2_000_000_000_000);
    /// assert_eq!(three.saturating_sub(two), D128e12::ONE);
    /// assert_eq!(D128e12::MIN.saturating_sub(D128e12::ONE), D128e12::MIN);
    /// ```
    #[inline]
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Returns `(self - rhs, did_overflow)` where the value is the
    /// two's-complement wrapping result.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let three = D128e12::from_bits(3_000_000_000_000);
    /// let two   = D128e12::from_bits(2_000_000_000_000);
    /// assert_eq!(three.overflowing_sub(two), (D128e12::ONE, false));
    /// assert_eq!(
    ///     D128e12::MIN.overflowing_sub(D128e12::from_bits(1)),
    ///     (D128e12::MAX, true),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (v, ovf) = self.0.overflowing_sub(rhs.0);
        (Self(v), ovf)
    }

    // Neg

    /// Returns `-self`, or `None` for `D128::MIN` (whose two's-complement
    /// negation does not fit in `i128`).
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// assert_eq!(D128e12::ONE.checked_neg(), Some(-D128e12::ONE));
    /// assert_eq!(D128e12::MIN.checked_neg(), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_neg(self) -> Option<Self> {
        match self.0.checked_neg() {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Returns `-self` with two's-complement wrap. For `D128::MIN`
    /// this returns `D128::MIN` (matches `i128::wrapping_neg`).
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// assert_eq!(D128e12::ONE.wrapping_neg(), -D128e12::ONE);
    /// assert_eq!(D128e12::MIN.wrapping_neg(), D128e12::MIN);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wrapping_neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }

    /// Returns `-self`, clamped to `D128::MAX` for `D128::MIN`
    /// (matches `i128::saturating_neg`).
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// assert_eq!(D128e12::ONE.saturating_neg(), -D128e12::ONE);
    /// assert_eq!(D128e12::MIN.saturating_neg(), D128e12::MAX);
    /// ```
    #[inline]
    #[must_use]
    pub const fn saturating_neg(self) -> Self {
        Self(self.0.saturating_neg())
    }

    /// Returns `(-self, did_overflow)`. Overflow occurs only for
    /// `D128::MIN`, in which case the wrapping result is `D128::MIN`.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// assert_eq!(D128e12::ONE.overflowing_neg(), (-D128e12::ONE, false));
    /// assert_eq!(D128e12::MIN.overflowing_neg(), (D128e12::MIN, true));
    /// ```
    #[inline]
    #[must_use]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (v, ovf) = self.0.overflowing_neg();
        (Self(v), ovf)
    }

    // Mul (rescale-aware via widening multiply-then-divide)
    //
    // All mul variants share the same widening boundary as the default
    // `Mul` operator. The helper `mul_div_pow10::<SCALE>` returns
    // `Some(q)` when the final i128 quotient fits, or `None` on overflow.
    // The four variants differ only in how they handle that `None`.

    /// Returns `self * rhs`, or `None` if the rescaled product does not
    /// fit in `i128`.
    ///
    /// The intermediate product is computed with 256-bit arithmetic and
    /// cannot itself overflow. The only failure mode is a final `i128`
    /// quotient that exceeds the storage range.
    ///
    /// # Precision
    ///
    /// Strict: the result is truncated (not rounded) toward zero during
    /// the scale-restoring divide, identical to the default `*` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let half = D128e12::from_bits(500_000_000_000); // 0.5
    /// assert_eq!(half.checked_mul(half), Some(D128e12::from_bits(250_000_000_000)));
    /// assert_eq!(D128e12::MAX.checked_mul(D128e12::from_bits(2_000_000_000_000)), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        crate::mg_divide::mul_div_pow10::<SCALE>(self.0, rhs.0).map(Self)
    }

    /// Returns `self * rhs` with two's-complement wrap when the rescaled
    /// product does not fit in `i128`.
    ///
    /// On overflow, falls back to `(a.wrapping_mul(b)).wrapping_div(multiplier())`.
    /// The exact bit pattern of the wrapping result at extreme magnitudes
    /// is an implementation detail; only the no-panic contract is guaranteed.
    ///
    /// # Precision
    ///
    /// Strict: truncates toward zero during the scale-restoring divide.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let half = D128e12::from_bits(500_000_000_000); // 0.5
    /// assert_eq!(half.wrapping_mul(half), D128e12::from_bits(250_000_000_000));
    /// // Overflow does not panic.
    /// let _ = D128e12::MAX.wrapping_mul(D128e12::from_bits(2_000_000_000_000));
    /// ```
    #[inline]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        match crate::mg_divide::mul_div_pow10::<SCALE>(self.0, rhs.0) {
            Some(q) => Self(q),
            None => Self(
                self.0
                    .wrapping_mul(rhs.0)
                    .wrapping_div(Self::multiplier()),
            ),
        }
    }

    /// Returns `self * rhs`, clamped to `D128::MAX` or `D128::MIN` on overflow.
    ///
    /// The clamp direction is determined by the XOR of operand signs:
    /// same-sign operands saturate to `MAX`; mixed-sign operands saturate
    /// to `MIN`.
    ///
    /// # Precision
    ///
    /// Strict: truncates toward zero during the scale-restoring divide.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(D128e12::MAX.saturating_mul(two), D128e12::MAX);
    /// assert_eq!(D128e12::MAX.saturating_mul(-two), D128e12::MIN);
    /// ```
    #[inline]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        match crate::mg_divide::mul_div_pow10::<SCALE>(self.0, rhs.0) {
            Some(q) => Self(q),
            None => {
                // Clamp direction: negative result iff exactly one operand
                // is negative (a zero operand cannot produce overflow).
                let neg = (self.0 < 0) ^ (rhs.0 < 0);
                if neg { Self::MIN } else { Self::MAX }
            }
        }
    }

    /// Returns `(self * rhs, did_overflow)` where the value is the
    /// wrapping result when overflow occurs.
    ///
    /// # Precision
    ///
    /// Strict: truncates toward zero during the scale-restoring divide.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let half = D128e12::from_bits(500_000_000_000); // 0.5
    /// assert_eq!(half.overflowing_mul(half), (D128e12::from_bits(250_000_000_000), false));
    /// let (_, ovf) = D128e12::MAX.overflowing_mul(D128e12::from_bits(2_000_000_000_000));
    /// assert!(ovf);
    /// ```
    #[inline]
    #[must_use]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        match crate::mg_divide::mul_div_pow10::<SCALE>(self.0, rhs.0) {
            Some(q) => (Self(q), false),
            None => (self.wrapping_mul(rhs), true),
        }
    }

    // Div (rescale-aware via widening long-divide)
    //
    // All div variants use `div_pow10_div` as the default `Div` operator.
    // Div-by-zero policy:
    //   - `checked_div(_, ZERO)` returns `None`.
    //   - `wrapping_div`, `saturating_div`, `overflowing_div` panic on
    //     `rhs == ZERO`, matching their `i128` counterparts.

    /// Returns `self / rhs`, or `None` on division by zero or if the
    /// rescaled quotient does not fit in `i128`.
    ///
    /// The only finite-operand overflow case is
    /// `D128::MIN / NEG_ONE` (storage negation of `i128::MIN` overflows).
    ///
    /// # Precision
    ///
    /// Strict: the widening divide truncates toward zero, identical to
    /// the default `/` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let six = D128e12::from_bits(6_000_000_000_000); // 6.0
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(six.checked_div(two), Some(D128e12::from_bits(3_000_000_000_000)));
    /// assert_eq!(D128e12::ONE.checked_div(D128e12::ZERO), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        crate::mg_divide::div_pow10_div::<SCALE>(self.0, rhs.0).map(Self)
    }

    /// Returns `self / rhs` with two's-complement wrap when the rescaled
    /// quotient does not fit in `i128`.
    ///
    /// On overflow, falls back to
    /// `(a.wrapping_mul(multiplier())).wrapping_div(b)`.
    ///
    /// # Precision
    ///
    /// Strict: truncates toward zero.
    ///
    /// # Panics
    ///
    /// Panics on `rhs == ZERO` (matches `i128::wrapping_div`).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let six = D128e12::from_bits(6_000_000_000_000); // 6.0
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(six.wrapping_div(two), D128e12::from_bits(3_000_000_000_000));
    /// ```
    #[inline]
    #[must_use]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            panic!("attempt to divide by zero");
        }
        match crate::mg_divide::div_pow10_div::<SCALE>(self.0, rhs.0) {
            Some(q) => Self(q),
            None => Self(
                self.0
                    .wrapping_mul(Self::multiplier())
                    .wrapping_div(rhs.0),
            ),
        }
    }

    /// Returns `self / rhs`, clamped to `D128::MAX` or `D128::MIN` on
    /// overflow.
    ///
    /// The clamp direction is determined by the XOR of operand signs,
    /// because the scale multiplier is always positive.
    ///
    /// # Precision
    ///
    /// Strict: truncates toward zero.
    ///
    /// # Panics
    ///
    /// Panics on `rhs == ZERO` (matches `i128::saturating_div`).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let six = D128e12::from_bits(6_000_000_000_000); // 6.0
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(six.saturating_div(two), D128e12::from_bits(3_000_000_000_000));
    /// // MAX / 0.5 overflows; both positive so clamp to MAX.
    /// assert_eq!(D128e12::MAX.saturating_div(D128e12::from_bits(500_000_000_000)), D128e12::MAX);
    /// ```
    #[inline]
    #[must_use]
    pub fn saturating_div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            panic!("attempt to divide by zero");
        }
        match crate::mg_divide::div_pow10_div::<SCALE>(self.0, rhs.0) {
            Some(q) => Self(q),
            None => {
                // Clamp direction: negative iff exactly one operand is negative.
                let neg = (self.0 < 0) ^ (rhs.0 < 0);
                if neg { Self::MIN } else { Self::MAX }
            }
        }
    }

    /// Returns `(self / rhs, did_overflow)` where the value is the
    /// wrapping result when overflow occurs.
    ///
    /// # Precision
    ///
    /// Strict: truncates toward zero.
    ///
    /// # Panics
    ///
    /// Panics on `rhs == ZERO` (matches `i128::overflowing_div`).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let six = D128e12::from_bits(6_000_000_000_000); // 6.0
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(six.overflowing_div(two), (D128e12::from_bits(3_000_000_000_000), false));
    /// let half = D128e12::from_bits(500_000_000_000); // 0.5
    /// let (_, ovf) = D128e12::MAX.overflowing_div(half);
    /// assert!(ovf);
    /// ```
    #[inline]
    #[must_use]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        if rhs.0 == 0 {
            panic!("attempt to divide by zero");
        }
        match crate::mg_divide::div_pow10_div::<SCALE>(self.0, rhs.0) {
            Some(q) => (Self(q), false),
            None => (self.wrapping_div(rhs), true),
        }
    }

    // Rem (no rescale; both operands share SCALE)
    //
    // Because both operands share `SCALE`, `(a * 10^S) % (b * 10^S)`
    // already lives in `value * 10^S` form with no further rescaling
    // needed. Delegates directly to `i128` intrinsics.
    //
    // No `saturating_rem` is provided: the result is always bounded by
    // `|rhs|`, so it cannot exceed the storage range when both operands
    // fit. The standard library does not define one for primitive integers.

    /// Returns `self % rhs`, or `None` on `rhs == ZERO` or for the
    /// storage-level overflow case `D128::MIN % from_bits(-1)`
    /// (matches `i128::checked_rem`).
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(5_500_000_000_000); // 5.5
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.checked_rem(two), Some(D128e12::from_bits(1_500_000_000_000)));
    /// assert_eq!(D128e12::ONE.checked_rem(D128e12::ZERO), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        match self.0.checked_rem(rhs.0) {
            Some(v) => Some(Self(v)),
            None => None,
        }
    }

    /// Returns `self % rhs`. For the storage-level overflow case
    /// `D128::MIN.wrapping_rem(from_bits(-1))`, returns `ZERO`
    /// (matches `i128::wrapping_rem`).
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Panics
    ///
    /// Panics on `rhs == ZERO` (matches `i128::wrapping_rem`).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(5_500_000_000_000); // 5.5
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.wrapping_rem(two), D128e12::from_bits(1_500_000_000_000));
    /// ```
    #[inline]
    #[must_use]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        Self(self.0.wrapping_rem(rhs.0))
    }

    /// Returns `(self % rhs, did_overflow)`. Overflow (`did_overflow == true`)
    /// occurs only for the storage-level case `D128::MIN % from_bits(-1)`,
    /// in which case the wrapping result is `ZERO`.
    ///
    /// # Precision
    ///
    /// Strict: operates on integer raw storage with no rounding.
    ///
    /// # Panics
    ///
    /// Panics on `rhs == ZERO` (matches `i128::overflowing_rem`).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D128e12;
    ///
    /// let a = D128e12::from_bits(5_500_000_000_000); // 5.5
    /// let two = D128e12::from_bits(2_000_000_000_000); // 2.0
    /// assert_eq!(a.overflowing_rem(two), (D128e12::from_bits(1_500_000_000_000), false));
    /// let neg_one_lsb = D128e12::from_bits(-1);
    /// assert_eq!(D128e12::MIN.overflowing_rem(neg_one_lsb), (D128e12::ZERO, true));
    /// ```
    #[inline]
    #[must_use]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        let (v, ovf) = self.0.overflowing_rem(rhs.0);
        (Self(v), ovf)
    }
}

#[cfg(test)]
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use crate::core_type::{D128, D128e12};

    /// Returns `-ONE` as a convenience value.
    fn neg_one() -> D128e12 {
        -D128e12::ONE
    }

    /// Returns `2.0` in `D128e12` canonical form.
    fn two() -> D128e12 {
        D128e12::from_bits(2_000_000_000_000)
    }

    /// Returns `3.0` in `D128e12` canonical form.
    fn three() -> D128e12 {
        D128e12::from_bits(3_000_000_000_000)
    }

    // Add variants

    #[test]
    fn checked_add_normal() {
        assert_eq!(D128e12::ONE.checked_add(D128e12::ONE), Some(two()));
    }

    #[test]
    fn checked_add_overflow_returns_none() {
        // MAX + ONE overflows (MAX is i128::MAX raw; ONE is 10^SCALE raw).
        assert_eq!(D128e12::MAX.checked_add(D128e12::ONE), None);
        // Boundary: MAX + 1 LSB also overflows.
        assert_eq!(
            D128e12::MAX.checked_add(D128e12::from_bits(1)),
            None
        );
    }

    #[test]
    fn checked_add_negative_overflow_returns_none() {
        assert_eq!(D128e12::MIN.checked_add(neg_one()), None);
        // Boundary: MIN + (-1 LSB) also overflows.
        assert_eq!(
            D128e12::MIN.checked_add(D128e12::from_bits(-1)),
            None
        );
    }

    #[test]
    fn wrapping_add_normal_matches_op() {
        assert_eq!(D128e12::ONE.wrapping_add(D128e12::ONE), two());
    }

    #[test]
    fn wrapping_add_overflow_wraps_to_min() {
        // MAX + 1 LSB wraps to MIN under two's-complement.
        assert_eq!(
            D128e12::MAX.wrapping_add(D128e12::from_bits(1)),
            D128e12::MIN
        );
    }

    #[test]
    fn wrapping_add_negative_overflow_wraps_to_max() {
        // MIN + (-1 LSB) wraps to MAX.
        assert_eq!(
            D128e12::MIN.wrapping_add(D128e12::from_bits(-1)),
            D128e12::MAX
        );
    }

    #[test]
    fn saturating_add_normal_matches_op() {
        assert_eq!(D128e12::ONE.saturating_add(D128e12::ONE), two());
    }

    #[test]
    fn saturating_add_overflow_clamps_to_max() {
        assert_eq!(D128e12::MAX.saturating_add(D128e12::ONE), D128e12::MAX);
    }

    #[test]
    fn saturating_add_negative_overflow_clamps_to_min() {
        assert_eq!(D128e12::MIN.saturating_add(neg_one()), D128e12::MIN);
    }

    #[test]
    fn overflowing_add_normal_no_overflow() {
        assert_eq!(
            D128e12::ONE.overflowing_add(D128e12::ONE),
            (two(), false)
        );
    }

    #[test]
    fn overflowing_add_overflow_flagged() {
        // MAX + 1 LSB wraps exactly to MIN; overflow flag is set.
        assert_eq!(
            D128e12::MAX.overflowing_add(D128e12::from_bits(1)),
            (D128e12::MIN, true)
        );
    }

    #[test]
    fn overflowing_add_negative_overflow_flagged() {
        // MIN + (-1 LSB) wraps exactly to MAX.
        assert_eq!(
            D128e12::MIN.overflowing_add(D128e12::from_bits(-1)),
            (D128e12::MAX, true)
        );
    }

    // Sub variants

    #[test]
    fn checked_sub_normal() {
        assert_eq!(three().checked_sub(D128e12::ONE), Some(two()));
    }

    #[test]
    fn checked_sub_underflow_returns_none() {
        assert_eq!(D128e12::MIN.checked_sub(D128e12::ONE), None);
    }

    #[test]
    fn checked_sub_positive_overflow_returns_none() {
        // MAX - (-ONE) = MAX + ONE -> overflows.
        assert_eq!(D128e12::MAX.checked_sub(neg_one()), None);
    }

    #[test]
    fn wrapping_sub_normal() {
        assert_eq!(three().wrapping_sub(D128e12::ONE), two());
    }

    #[test]
    fn wrapping_sub_underflow_wraps_to_max() {
        // MIN - 1 LSB wraps exactly to MAX.
        assert_eq!(
            D128e12::MIN.wrapping_sub(D128e12::from_bits(1)),
            D128e12::MAX
        );
    }

    #[test]
    fn saturating_sub_normal() {
        assert_eq!(three().saturating_sub(D128e12::ONE), two());
    }

    #[test]
    fn saturating_sub_underflow_clamps_to_min() {
        assert_eq!(D128e12::MIN.saturating_sub(D128e12::ONE), D128e12::MIN);
    }

    #[test]
    fn saturating_sub_overflow_clamps_to_max() {
        // MAX - (-ONE) saturates to MAX.
        assert_eq!(D128e12::MAX.saturating_sub(neg_one()), D128e12::MAX);
    }

    #[test]
    fn overflowing_sub_normal() {
        assert_eq!(
            three().overflowing_sub(D128e12::ONE),
            (two(), false)
        );
    }

    #[test]
    fn overflowing_sub_underflow_flagged() {
        // MIN - 1 LSB wraps exactly to MAX.
        assert_eq!(
            D128e12::MIN.overflowing_sub(D128e12::from_bits(1)),
            (D128e12::MAX, true)
        );
    }

    // Neg variants

    #[test]
    fn checked_neg_normal() {
        assert_eq!(D128e12::ONE.checked_neg(), Some(neg_one()));
        assert_eq!(neg_one().checked_neg(), Some(D128e12::ONE));
        assert_eq!(D128e12::ZERO.checked_neg(), Some(D128e12::ZERO));
    }

    #[test]
    fn checked_neg_min_returns_none() {
        assert_eq!(D128e12::MIN.checked_neg(), None);
    }

    #[test]
    fn checked_neg_max_succeeds() {
        // MAX = i128::MAX, -MAX = i128::MIN + 1, fits.
        let neg_max = D128e12::from_bits(-i128::MAX);
        assert_eq!(D128e12::MAX.checked_neg(), Some(neg_max));
    }

    #[test]
    fn wrapping_neg_normal() {
        assert_eq!(D128e12::ONE.wrapping_neg(), neg_one());
        assert_eq!(D128e12::ZERO.wrapping_neg(), D128e12::ZERO);
    }

    #[test]
    fn wrapping_neg_min_returns_min() {
        // -i128::MIN wraps to i128::MIN under two's-complement.
        assert_eq!(D128e12::MIN.wrapping_neg(), D128e12::MIN);
    }

    #[test]
    fn saturating_neg_normal() {
        assert_eq!(D128e12::ONE.saturating_neg(), neg_one());
        assert_eq!(D128e12::ZERO.saturating_neg(), D128e12::ZERO);
    }

    #[test]
    fn saturating_neg_min_returns_max() {
        assert_eq!(D128e12::MIN.saturating_neg(), D128e12::MAX);
    }

    #[test]
    fn overflowing_neg_normal() {
        assert_eq!(
            D128e12::ONE.overflowing_neg(),
            (neg_one(), false)
        );
        assert_eq!(
            D128e12::ZERO.overflowing_neg(),
            (D128e12::ZERO, false)
        );
    }

    #[test]
    fn overflowing_neg_min_flagged() {
        assert_eq!(
            D128e12::MIN.overflowing_neg(),
            (D128e12::MIN, true)
        );
    }

    // Mul variants

    #[test]
    fn checked_mul_normal() {
        let half = D128e12::from_bits(500_000_000_000);
        let quarter = D128e12::from_bits(250_000_000_000);
        assert_eq!(half.checked_mul(half), Some(quarter));
    }

    #[test]
    fn checked_mul_zero() {
        assert_eq!(D128e12::MAX.checked_mul(D128e12::ZERO), Some(D128e12::ZERO));
        assert_eq!(D128e12::ZERO.checked_mul(D128e12::ZERO), Some(D128e12::ZERO));
    }

    #[test]
    fn checked_mul_one_identity() {
        let v = D128e12::from_bits(7_500_000_000_000); // 7.5
        assert_eq!(v.checked_mul(D128e12::ONE), Some(v));
        assert_eq!(D128e12::ONE.checked_mul(v), Some(v));
    }

    #[test]
    fn checked_mul_overflow_returns_none() {
        // MAX * 2.0 overflows the final i128 quotient.
        assert_eq!(D128e12::MAX.checked_mul(two()), None);
    }

    #[test]
    fn checked_mul_min_overflow_returns_none() {
        // MIN * 2.0 overflows.
        assert_eq!(D128e12::MIN.checked_mul(two()), None);
    }

    #[test]
    fn wrapping_mul_normal() {
        let half = D128e12::from_bits(500_000_000_000);
        let quarter = D128e12::from_bits(250_000_000_000);
        assert_eq!(half.wrapping_mul(half), quarter);
    }

    #[test]
    fn wrapping_mul_overflow_does_not_panic() {
        // Verify it does not panic; the exact bit pattern is unspecified.
        let _ = D128e12::MAX.wrapping_mul(two());
        let _ = D128e12::MIN.wrapping_mul(two());
    }

    #[test]
    fn saturating_mul_normal() {
        let half = D128e12::from_bits(500_000_000_000);
        let quarter = D128e12::from_bits(250_000_000_000);
        assert_eq!(half.saturating_mul(half), quarter);
    }

    #[test]
    fn saturating_mul_positive_overflow_clamps_to_max() {
        // MAX * 2.0 (both positive) saturates to MAX.
        assert_eq!(D128e12::MAX.saturating_mul(two()), D128e12::MAX);
    }

    #[test]
    fn saturating_mul_negative_overflow_clamps_to_min() {
        // MAX * (-2.0) (mixed sign) saturates to MIN.
        assert_eq!(
            D128e12::MAX.saturating_mul(-two()),
            D128e12::MIN
        );
    }

    #[test]
    fn saturating_mul_min_times_two_clamps_to_min() {
        // MIN * 2.0 (MIN negative, 2 positive) saturates to MIN.
        assert_eq!(D128e12::MIN.saturating_mul(two()), D128e12::MIN);
    }

    #[test]
    fn saturating_mul_min_times_neg_two_clamps_to_max() {
        // MIN * -2.0 (both negative) saturates to MAX.
        assert_eq!(D128e12::MIN.saturating_mul(-two()), D128e12::MAX);
    }

    #[test]
    fn overflowing_mul_normal_no_overflow() {
        let half = D128e12::from_bits(500_000_000_000);
        let quarter = D128e12::from_bits(250_000_000_000);
        assert_eq!(half.overflowing_mul(half), (quarter, false));
    }

    #[test]
    fn overflowing_mul_overflow_flagged() {
        let (_, ovf) = D128e12::MAX.overflowing_mul(two());
        assert!(ovf);
    }

    // Div variants

    #[test]
    fn checked_div_normal() {
        // 6.0 / 2.0 = 3.0
        let six = D128e12::from_bits(6_000_000_000_000);
        assert_eq!(six.checked_div(two()), Some(three()));
    }

    #[test]
    fn checked_div_by_zero_returns_none() {
        assert_eq!(D128e12::ONE.checked_div(D128e12::ZERO), None);
    }

    #[test]
    fn checked_div_overflow_returns_none() {
        // MAX / 0.5 = 2 * MAX -> overflows the final quotient.
        let half = D128e12::from_bits(500_000_000_000);
        assert_eq!(D128e12::MAX.checked_div(half), None);
    }

    #[test]
    fn checked_div_negative_normal() {
        let neg_six = D128e12::from_bits(-6_000_000_000_000);
        assert_eq!(neg_six.checked_div(two()), Some(-three()));
    }

    #[test]
    fn wrapping_div_normal() {
        let six = D128e12::from_bits(6_000_000_000_000);
        assert_eq!(six.wrapping_div(two()), three());
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn wrapping_div_by_zero_panics() {
        let _ = D128e12::ONE.wrapping_div(D128e12::ZERO);
    }

    #[test]
    fn wrapping_div_overflow_does_not_panic() {
        // Verify it does not panic; the exact result is unspecified.
        let half = D128e12::from_bits(500_000_000_000);
        let _ = D128e12::MAX.wrapping_div(half);
    }

    #[test]
    fn saturating_div_normal() {
        let six = D128e12::from_bits(6_000_000_000_000);
        assert_eq!(six.saturating_div(two()), three());
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn saturating_div_by_zero_panics() {
        let _ = D128e12::ONE.saturating_div(D128e12::ZERO);
    }

    #[test]
    fn saturating_div_overflow_clamps_to_max() {
        // MAX / 0.5 (both positive) saturates to MAX.
        let half = D128e12::from_bits(500_000_000_000);
        assert_eq!(D128e12::MAX.saturating_div(half), D128e12::MAX);
    }

    #[test]
    fn saturating_div_negative_overflow_clamps_to_min() {
        // MAX / -0.5 (mixed sign) saturates to MIN.
        let neg_half = D128e12::from_bits(-500_000_000_000);
        assert_eq!(
            D128e12::MAX.saturating_div(neg_half),
            D128e12::MIN
        );
    }

    #[test]
    fn overflowing_div_normal() {
        let six = D128e12::from_bits(6_000_000_000_000);
        assert_eq!(six.overflowing_div(two()), (three(), false));
    }

    #[test]
    fn overflowing_div_overflow_flagged() {
        let half = D128e12::from_bits(500_000_000_000);
        let (_, ovf) = D128e12::MAX.overflowing_div(half);
        assert!(ovf);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn overflowing_div_by_zero_panics() {
        let _ = D128e12::ONE.overflowing_div(D128e12::ZERO);
    }

    // Rem variants

    #[test]
    fn checked_rem_normal() {
        // 5.5 % 2.0 = 1.5
        let a = D128e12::from_bits(5_500_000_000_000);
        let expected = D128e12::from_bits(1_500_000_000_000);
        assert_eq!(a.checked_rem(two()), Some(expected));
    }

    #[test]
    fn checked_rem_by_zero_returns_none() {
        assert_eq!(D128e12::ONE.checked_rem(D128e12::ZERO), None);
    }

    #[test]
    fn checked_rem_min_neg_one_lsb_returns_none() {
        // The raw overflow case is `i128::MIN % -1` (because i128::MIN / -1
        // overflows). The divisor's raw bits are -1, not the decimal -ONE
        // (-10^12), which does not trigger this path.
        let neg_one_lsb = D128e12::from_bits(-1);
        assert_eq!(D128e12::MIN.checked_rem(neg_one_lsb), None);
    }

    #[test]
    fn wrapping_rem_normal() {
        let a = D128e12::from_bits(5_500_000_000_000);
        let expected = D128e12::from_bits(1_500_000_000_000);
        assert_eq!(a.wrapping_rem(two()), expected);
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn wrapping_rem_by_zero_panics() {
        let _ = D128e12::ONE.wrapping_rem(D128e12::ZERO);
    }

    #[test]
    fn wrapping_rem_min_neg_one_lsb_returns_zero() {
        // i128::MIN % -1 wraps to 0 (the overflow case).
        let neg_one_lsb = D128e12::from_bits(-1);
        assert_eq!(
            D128e12::MIN.wrapping_rem(neg_one_lsb),
            D128e12::ZERO
        );
    }

    #[test]
    fn overflowing_rem_normal() {
        let a = D128e12::from_bits(5_500_000_000_000);
        let expected = D128e12::from_bits(1_500_000_000_000);
        assert_eq!(a.overflowing_rem(two()), (expected, false));
    }

    #[test]
    fn overflowing_rem_min_neg_one_lsb_flagged() {
        let neg_one_lsb = D128e12::from_bits(-1);
        assert_eq!(
            D128e12::MIN.overflowing_rem(neg_one_lsb),
            (D128e12::ZERO, true)
        );
    }

    // Cross-scale exercise

    /// Verifies that the variant family compiles and functions correctly at SCALE = 6.
    #[test]
    fn variants_at_scale_6() {
        type D6 = D128<6>;
        let one = D6::ONE;
        let two_d6 = D6::from_bits(2_000_000); // 2.0 at SCALE=6
        let one_lsb = D6::from_bits(1);

        assert_eq!(one.checked_add(one), Some(two_d6));
        // MAX + 1 LSB overflows / wraps to MIN under two's-complement.
        assert_eq!(D6::MAX.checked_add(one_lsb), None);
        assert_eq!(D6::MAX.saturating_add(one_lsb), D6::MAX);
        assert_eq!(D6::MAX.wrapping_add(one_lsb), D6::MIN);
        assert_eq!(
            D6::MAX.overflowing_add(one_lsb),
            (D6::MIN, true)
        );

        assert_eq!(D6::MIN.checked_neg(), None);
        assert_eq!(D6::MIN.wrapping_neg(), D6::MIN);
        assert_eq!(D6::MIN.saturating_neg(), D6::MAX);
    }

    /// Verifies that `checked_*` matches the base operator when no overflow occurs.
    #[test]
    fn checked_matches_op_in_range() {
        let a = D128e12::from_bits(7_500_000_000_000); // 7.5
        let b = two();
        assert_eq!(a.checked_add(b), Some(a + b));
        assert_eq!(a.checked_sub(b), Some(a - b));
        assert_eq!(a.checked_mul(b), Some(a * b));
        assert_eq!(a.checked_div(b), Some(a / b));
        assert_eq!(a.checked_rem(b), Some(a % b));
    }

    /// Verifies that the `overflowing_*` flag agrees with `checked_*` returning `None`.
    #[test]
    fn overflowing_flag_matches_checked_none() {
        // Add: MAX + ONE
        let (_, ovf) = D128e12::MAX.overflowing_add(D128e12::ONE);
        assert_eq!(ovf, D128e12::MAX.checked_add(D128e12::ONE).is_none());

        // Sub: MIN - ONE
        let (_, ovf) = D128e12::MIN.overflowing_sub(D128e12::ONE);
        assert_eq!(ovf, D128e12::MIN.checked_sub(D128e12::ONE).is_none());

        // Mul: MAX * 2
        let (_, ovf) = D128e12::MAX.overflowing_mul(two());
        assert_eq!(ovf, D128e12::MAX.checked_mul(two()).is_none());

        // Neg: MIN
        let (_, ovf) = D128e12::MIN.overflowing_neg();
        assert_eq!(ovf, D128e12::MIN.checked_neg().is_none());

        // Rem: MIN % (-1 LSB) -- the raw i128::MIN % -1 case.
        let neg_one_lsb = D128e12::from_bits(-1);
        let (_, ovf) = D128e12::MIN.overflowing_rem(neg_one_lsb);
        assert_eq!(
            ovf,
            D128e12::MIN.checked_rem(neg_one_lsb).is_none()
        );
    }

    /// Verifies that `saturating_add`, `saturating_sub`, and `saturating_mul`
    /// never panic and always return a value within `[MIN, MAX]`.
    #[test]
    fn saturating_never_escapes_bounds() {
        let extremes = [
            D128e12::MIN,
            D128e12::from_bits(-1),
            D128e12::ZERO,
            D128e12::ONE,
            D128e12::MAX,
        ];
        for &a in &extremes {
            for &b in &extremes {
                let s_add = a.saturating_add(b);
                let s_sub = a.saturating_sub(b);
                let s_mul = a.saturating_mul(b);
                assert!(s_add >= D128e12::MIN && s_add <= D128e12::MAX);
                assert!(s_sub >= D128e12::MIN && s_sub <= D128e12::MAX);
                assert!(s_mul >= D128e12::MIN && s_mul <= D128e12::MAX);
            }
        }
    }
}
