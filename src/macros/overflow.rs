//! Macro-generated overflow-aware arithmetic variants for the decimal
//! widths that use a *uniform* mul/div pattern (D18, and the wide
//! tier D76 / D153 / D307).
//!
//! Emits the four standard families (`checked_*`, `wrapping_*`,
//! `saturating_*`, `overflowing_*`) for `add`, `sub`, `neg`, `mul`,
//! `div`, `rem`. Behaviour mirrors the corresponding `i128::*` method
//! per the Rust standard library convention:
//!
//! - `checked_*` returns `Some(result)` on success or `None` on
//!   overflow / division-by-zero.
//! - `wrapping_*` returns the result modulo the storage's
//!   `MAX − MIN` range.
//! - `saturating_*` clamps to `Self::MIN` / `Self::MAX` on
//!   overflow.
//! - `overflowing_*` returns `(wrapping_result, overflowed_flag)`.
//!
//! Add / sub / neg / rem delegate to the storage type's `checked_*` /
//! `wrapping_*` / `saturating_*` / `overflowing_*` intrinsics, which
//! the wide integers expose with the same names and `const`-ness as the
//! primitive integers — so those families live in a shared `@common`
//! arm. Mul / div widen to `$Wider` for the intermediate; only the
//! widening *spelling* differs (native `as`-casts vs the `BigInt` cast),
//! so they are written inline per front-end arm.
//!
//! D38 is the exception: its overflow mul/div go through the
//! hand-rolled `mg_divide` path and are not generated here.

/// Emits overflow variants for a decimal type.
///
/// - `decl_decimal_overflow_variants!(i32, i64)` — *native*
/// storage; `$Wider` is a primitive integer.
/// - `decl_decimal_overflow_variants!(wide D76, I256, I512)` — *wide*
/// storage; `$Wider` is the next size up.
macro_rules! decl_decimal_overflow_variants {
    // Wide storage.
    (wide $Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::overflow::decl_decimal_overflow_variants!(@common $Type, $Storage);

        impl<const SCALE: u32> $Type<SCALE> {
            // ----- mul (uses widening) ------------------------------

            /// Checked multiplication. Computes `self * rhs` rounded
            /// toward zero, returning `None` if the result doesn't fit
            /// in `Self`. The intermediate product is computed in
            /// `$Wider` so widening overflow is detected before the
            /// final narrowing.
            #[inline]
            #[must_use]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let prod = a.checked_mul(b)?;
                let scaled = prod / m;
                let storage_max: $Wider = <$Storage>::MAX.resize::<$Wider>();
                let storage_min: $Wider = <$Storage>::MIN.resize::<$Wider>();
                if scaled > storage_max || scaled < storage_min {
                    None
                } else {
                    Some(Self(scaled.resize::<$Storage>()))
                }
            }

            /// Wrapping multiplication. Computes `self * rhs` modulo
            /// the storage type's `MAX − MIN` range. The intermediate
            /// product still widens to `$Wider`; only the *narrowing*
            /// step wraps.
            #[inline]
            #[must_use]
            pub fn wrapping_mul(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let prod = a.wrapping_mul(b);
                let scaled = prod / m;
                Self(scaled.resize::<$Storage>())
            }

            /// Saturating multiplication. Computes `self * rhs`,
            /// clamping to [`Self::MIN`] / [`Self::MAX`] on overflow.
            /// Sign of the saturated bound matches the sign of the
            /// exact mathematical product.
            #[inline]
            #[must_use]
            pub fn saturating_mul(self, rhs: Self) -> Self {
                match self.checked_mul(rhs) {
                    Some(v) => v,
                    None => {
                        let neg_result =
                            self.0.is_negative() ^ rhs.0.is_negative();
                        if neg_result { Self::MIN } else { Self::MAX }
                    }
                }
            }

            /// Overflowing multiplication. Returns the wrapped result
            /// together with a boolean flag — `true` if the
            /// mathematical product was out of range.
            #[inline]
            #[must_use]
            pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                match self.checked_mul(rhs) {
                    Some(v) => (v, false),
                    None => (self.wrapping_mul(rhs), true),
                }
            }

            // ----- div ----------------------------------------------

            /// Checked division. Returns `None` if `rhs` is zero or
            /// the result would overflow [`Self`]. Rounds to nearest
            /// using the crate-default [`RoundingMode`], identical to
            /// the `/` operator. The numerator is pre-multiplied by
            /// `10^SCALE` in `$Wider` so the intermediate carries the
            /// scale-up step exactly before rounding.
            #[inline]
            #[must_use]
            pub fn checked_div(self, rhs: Self) -> Option<Self> {
                if rhs == Self::ZERO {
                    return None;
                }
                let b: $Wider = rhs.0.resize::<$Wider>();
                let n: $Wider = self.0.widen_mul::<$Wider>($Type::<SCALE>::multiplier());
                let result = $crate::macros::arithmetic::round_with_mode_wide!(
                    n, b, $Wider, $crate::support::rounding::DEFAULT_ROUNDING_MODE);
                let storage_max: $Wider = <$Storage>::MAX.resize::<$Wider>();
                let storage_min: $Wider = <$Storage>::MIN.resize::<$Wider>();
                if result > storage_max || result < storage_min {
                    None
                } else {
                    Some(Self(result.resize::<$Storage>()))
                }
            }

            /// Wrapping division. Computes `self / rhs` rounded to
            /// nearest using the crate-default [`RoundingMode`] (like
            /// the `/` operator), with the scale-up step done modulo
            /// `$Wider`'s range and the final narrowing wrapping.
            /// **Panics** on divide-by-zero (matches `i128::wrapping_div`).
            #[inline]
            #[must_use]
            pub fn wrapping_div(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let scaled_numer = a.wrapping_mul(m);
                let result = $crate::macros::arithmetic::round_with_mode_wide!(
                    scaled_numer, b, $Wider, $crate::support::rounding::DEFAULT_ROUNDING_MODE);
                Self(result.resize::<$Storage>())
            }

            /// Saturating division. Computes `self / rhs`, clamping to
            /// [`Self::MIN`] / [`Self::MAX`] on overflow.
            /// Divide-by-zero saturates to the appropriate-sign bound.
            #[inline]
            #[must_use]
            pub fn saturating_div(self, rhs: Self) -> Self {
                if rhs == Self::ZERO {
                    panic!("attempt to divide by zero");
                }
                match self.checked_div(rhs) {
                    Some(v) => v,
                    None => {
                        let neg_result =
                            self.0.is_negative() ^ rhs.0.is_negative();
                        if neg_result { Self::MIN } else { Self::MAX }
                    }
                }
            }

            /// Overflowing division. Returns the wrapped result
            /// together with a boolean flag — `true` if the exact
            /// quotient was out of range *or* `rhs` was zero.
            #[inline]
            #[must_use]
            pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                match self.checked_div(rhs) {
                    Some(v) => (v, false),
                    None => (self.wrapping_div(rhs), true),
                }
            }
        }
    };


    // Shared: add / sub / neg / rem and their overflow families.
    // the wide integers expose these intrinsics with the same names and
    // `const`-ness as the primitive integers.
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            // ----- add ----------------------------------------------

            /// Checked addition. `Some(self + rhs)`, or `None` if the
            /// sum would overflow [`Self`].
            #[inline]
            #[must_use]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                match self.0.checked_add(rhs.0) {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            /// Wrapping addition. `self + rhs` modulo the storage
            /// type's `MAX − MIN` range.
            #[inline]
            #[must_use]
            pub const fn wrapping_add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }

            /// Saturating addition. Clamps to [`Self::MIN`] /
            /// [`Self::MAX`] on overflow.
            #[inline]
            #[must_use]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }

            /// Overflowing addition. Returns `(self.wrapping_add(rhs),
            /// overflowed)`.
            #[inline]
            #[must_use]
            pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_add(rhs.0);
                (Self(v), of)
            }

            // ----- sub ----------------------------------------------

            /// Checked subtraction. `Some(self - rhs)`, or `None` if
            /// the difference would overflow [`Self`].
            #[inline]
            #[must_use]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                match self.0.checked_sub(rhs.0) {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            /// Wrapping subtraction.
            #[inline]
            #[must_use]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }

            /// Saturating subtraction. Clamps to [`Self::MIN`] /
            /// [`Self::MAX`] on overflow.
            #[inline]
            #[must_use]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }

            /// Overflowing subtraction. Returns
            /// `(self.wrapping_sub(rhs), overflowed)`.
            #[inline]
            #[must_use]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_sub(rhs.0);
                (Self(v), of)
            }

            // ----- neg ----------------------------------------------

            /// Checked negation. `Some(-self)`, or `None` when
            /// `self == Self::MIN` (whose negation is unrepresentable
            /// in two's-complement).
            #[inline]
            #[must_use]
            pub const fn checked_neg(self) -> Option<Self> {
                match self.0.checked_neg() {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            /// Wrapping negation. `Self::MIN.wrapping_neg() == Self::MIN`
            /// (same as `i128::wrapping_neg`).
            #[inline]
            #[must_use]
            pub const fn wrapping_neg(self) -> Self {
                Self(self.0.wrapping_neg())
            }

            /// Saturating negation. `Self::MIN.saturating_neg() ==
            /// Self::MAX`.
            #[inline]
            #[must_use]
            pub const fn saturating_neg(self) -> Self {
                Self(self.0.saturating_neg())
            }

            /// Overflowing negation. Returns
            /// `(self.wrapping_neg(), overflowed)`; `overflowed` is
            /// `true` only when `self == Self::MIN`.
            #[inline]
            #[must_use]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_neg();
                (Self(v), of)
            }

            // ----- rem ----------------------------------------------

            /// Checked remainder. `Some(self % rhs)`, or `None` if
            /// `rhs == 0` *or* the operation would overflow (the
            /// pathological case `Self::MIN % -ONE`).
            #[inline]
            #[must_use]
            pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                match self.0.checked_rem(rhs.0) {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            /// Wrapping remainder. **Panics** on divide-by-zero
            /// (matches `i128::wrapping_rem`).
            #[inline]
            #[must_use]
            pub const fn wrapping_rem(self, rhs: Self) -> Self {
                Self(self.0.wrapping_rem(rhs.0))
            }

            /// Overflowing remainder. Returns
            /// `(self.wrapping_rem(rhs), overflowed)`; `overflowed`
            /// is `true` only at the `Self::MIN % -ONE` boundary.
            #[inline]
            #[must_use]
            pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_rem(rhs.0);
                (Self(v), of)
            }
        }
    };
}

pub(crate) use decl_decimal_overflow_variants;
