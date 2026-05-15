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
                let mut acc = Self::ONE;
                let mut base = self;
                let mut e = exp;
                while e > 0 {
                    if e & 1 == 1 {
                        acc = acc * base;
                    }
                    e >>= 1;
                    if e > 0 {
                        base = base * base;
                    }
                }
                acc
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
                ::core::option::Option::Some(acc)
            }

            /// Two's-complement wrap at every multiplication step.
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
                let neg = self < Self::ZERO && (exp & 1) == 1;
                match self.checked_pow(exp) {
                    ::core::option::Option::Some(v) => v,
                    ::core::option::Option::None => {
                        if neg { Self::MIN } else { Self::MAX }
                    }
                }
            }

            /// `(self^exp, overflowed)`. `overflowed` is `true` if any
            /// multiplication step overflowed; the value is the
            /// wrapping form.
            #[inline]
            #[must_use]
            pub fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                let mut acc = Self::ONE;
                let mut base = self;
                let mut e = exp;
                let mut overflowed = false;
                while e > 0 {
                    if e & 1 == 1 {
                        let (v, ov) = acc.overflowing_mul(base);
                        acc = v;
                        overflowed |= ov;
                    }
                    e >>= 1;
                    if e > 0 {
                        let (v, ov) = base.overflowing_mul(base);
                        base = v;
                        overflowed |= ov;
                    }
                }
                (acc, overflowed)
            }
        }
    };
}

pub(crate) use decl_decimal_pow;
