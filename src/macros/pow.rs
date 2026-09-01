// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer-exponent power family — `pow` / `powi` and the
//! `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*`
//! overflow variants.
//!
//! All six methods are square-and-multiply built on the type's `Mul`
//! operator (or the matching overflow-variant of multiplication), so
//! they apply unchanged to every storage kind. D38 keeps its
//! hand-written versions, which sit closer to the `mg_divide` path;
//! every other width takes them from this macro.

/// Emits `pow` / `powi` and the four overflow variants for
/// `$Type<SCALE>`.
macro_rules! decl_decimal_pow {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Raises `self` to the power `exp` via square-and-multiply.
            /// `exp = 0` always returns `ONE`. Overflow at any
            /// multiplication step follows the `Mul` operator's
            /// semantics (debug-panic, release-wrap).
            #[inline]
            #[must_use]
            pub fn pow(self, exp: u32) -> Self {
                let mut accumulator = Self::ONE;
                let mut base = self;
                let mut remaining_exponent = exp;
                while remaining_exponent > 0 {
                    if remaining_exponent & 1 == 1 {
                        accumulator *= base;
                    }
                    remaining_exponent >>= 1;
                    if remaining_exponent > 0 {
                        base = base * base;
                    }
                }
                accumulator
            }

            /// Signed integer exponent. For non-negative `exp` this is
            /// `self.pow(exp as u32)`; for negative `exp` it is
            /// `Self::ONE / self.pow(exp.unsigned_abs())`.
            ///
            /// `i32::unsigned_abs` handles `i32::MIN` without the
            /// signed-negation overflow that `(-i32::MIN) as u32`
            /// would cause.
            #[inline]
            #[must_use]
            pub fn powi(self, exp: i32) -> Self {
                if exp >= 0 {
                    self.pow(exp as u32)
                } else {
                    Self::ONE / self.pow(exp.unsigned_abs())
                }
            }

            /// `Some(self^exp)`, or `None` if any multiplication step
            /// overflows.
            #[inline]
            #[must_use]
            pub fn checked_pow(self, exp: u32) -> ::core::option::Option<Self> {
                let mut accumulator = Self::ONE;
                let mut base = self;
                let mut remaining_exponent = exp;
                while remaining_exponent > 0 {
                    if remaining_exponent & 1 == 1 {
                        accumulator = accumulator.checked_mul(base)?;
                    }
                    remaining_exponent >>= 1;
                    if remaining_exponent > 0 {
                        base = base.checked_mul(base)?;
                    }
                }
                ::core::option::Option::Some(accumulator)
            }

            /// Two's-complement wrap at every multiplication step.
            #[inline]
            #[must_use]
            pub fn wrapping_pow(self, exp: u32) -> Self {
                let mut accumulator = Self::ONE;
                let mut base = self;
                let mut remaining_exponent = exp;
                while remaining_exponent > 0 {
                    if remaining_exponent & 1 == 1 {
                        accumulator = accumulator.wrapping_mul(base);
                    }
                    remaining_exponent >>= 1;
                    if remaining_exponent > 0 {
                        base = base.wrapping_mul(base);
                    }
                }
                accumulator
            }

            /// Saturates to `Self::MAX` or `Self::MIN` on overflow,
            /// based on the sign the mathematical result would have.
            #[inline]
            #[must_use]
            pub fn saturating_pow(self, exp: u32) -> Self {
                if exp == 0 {
                    return Self::ONE;
                }
                // The result is negative iff the base is negative and
                // the exponent is odd.
                let is_negative = self < Self::ZERO && (exp & 1) == 1;
                match self.checked_pow(exp) {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        if is_negative {
                            Self::MIN
                        } else {
                            Self::MAX
                        }
                    }
                }
            }

            /// `(self^exp, overflowed)`. `overflowed` is `true` if any
            /// multiplication step overflowed; the value is the
            /// wrapping form.
            #[inline]
            #[must_use]
            pub fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                let mut accumulator = Self::ONE;
                let mut base = self;
                let mut remaining_exponent = exp;
                let mut overflowed = false;
                while remaining_exponent > 0 {
                    if remaining_exponent & 1 == 1 {
                        let (product, step_overflowed) = accumulator.overflowing_mul(base);
                        accumulator = product;
                        overflowed |= step_overflowed;
                    }
                    remaining_exponent >>= 1;
                    if remaining_exponent > 0 {
                        let (squared, step_overflowed) = base.overflowing_mul(base);
                        base = squared;
                        overflowed |= step_overflowed;
                    }
                }
                (accumulator, overflowed)
            }
        }
    };
}

pub(crate) use decl_decimal_pow;
