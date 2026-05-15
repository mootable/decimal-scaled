//! Macro-generated arithmetic operator overloads for the decimal
//! widths that use a *uniform* mul/div pattern (D32, D64, and the wide
//! tier D256 / D512 / D1024).
//!
//! For D32 / D64 the storage type is a primitive (`i32` / `i64`) and a
//! native wider integer (`i64` / `i128`) carries the mul/div widening
//! step. For D256 / D512 / D1024 the storage type is a `bnum`
//! fixed-width integer and the widening type is the next `bnum` size
//! up; the only thing that changes is *how* the `10^SCALE` literal and
//! the width casts are spelled.
//!
//! D128 is the exception: its mul/div go through the hand-rolled
//! 256-bit `mg_divide` path and are not generated here.
//!
//! Add / Sub / Neg / Rem (and their `*Assign` forms) are identical for
//! both storage kinds and live in the shared `@common` arm. Only Mul
//! and Div differ, so they are written inline in each front-end arm —
//! that keeps `self` / `rhs` in the same macro hygiene context as the
//! method signature.
//!
//! Overflow semantics mirror Rust's default integer arithmetic:
//! debug-mode panic on overflow, release-mode wrap. Explicit-overflow
//! variants (`checked_*`, `saturating_*`, `wrapping_*`) live in a
//! companion module.

/// Generates the standard arithmetic operator overloads for a decimal
/// width `$Type<SCALE>`.
///
/// - `decl_decimal_arithmetic!(D32, i32, i64)` — *native* storage; the
///   widening type is a primitive integer, `as`-casts and the
///   `(10 as $Wider)` literal carry the mul/div step.
/// - `decl_decimal_arithmetic!(wide D256, I256, I512)` — *wide*
///   storage; the widening type is a `bnum` integer, `bnum::cast::As`
///   carries the width casts and `from_str_radix` builds the
///   `10^SCALE` factor.
macro_rules! decl_decimal_arithmetic {
    // Wide (bnum-backed) storage.
    (wide $Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@common $Type, $Storage);

        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale. Widens to `$Wider`
            /// to hold `a * b` without intermediate overflow, divides
            /// by `10^SCALE`, then narrows back to `$Storage`.
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = <$Wider>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
                let scaled = (a * b) / m;
                Self(scaled.resize::<$Storage>())
            }
        }

        impl<const SCALE: u32> ::core::ops::MulAssign for $Type<SCALE> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Div for $Type<SCALE> {
            type Output = Self;
            /// Divide two values of the same scale. Numerator is widened
            /// to `$Wider` and multiplied by `10^SCALE` before dividing
            /// by `b`, preserving the `value * 10^SCALE` form.
            #[inline]
            fn div(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = <$Wider>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
                let result = (a * m) / b;
                Self(result.resize::<$Storage>())
            }
        }

        impl<const SCALE: u32> ::core::ops::DivAssign for $Type<SCALE> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@common $Type, $Storage);

        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale. Widens to `$Wider`
            /// to hold `a * b` without intermediate overflow, divides
            /// by `10^SCALE`, then narrows back to `$Storage`. The
            /// narrowing cast wraps (release) / panics (debug) when the
            /// result is out of range, matching Rust's `as`-cast.
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let prod = a * b;
                let scaled = prod / m;
                Self(scaled as $Storage)
            }
        }

        impl<const SCALE: u32> ::core::ops::MulAssign for $Type<SCALE> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Div for $Type<SCALE> {
            type Output = Self;
            /// Divide two values of the same scale. Numerator is widened
            /// to `$Wider` and multiplied by `10^SCALE` before dividing
            /// by `b`, preserving the `value * 10^SCALE` form.
            #[inline]
            fn div(self, rhs: Self) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let result = (a * m) / b;
                Self(result as $Storage)
            }
        }

        impl<const SCALE: u32> ::core::ops::DivAssign for $Type<SCALE> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };

    // Add / Sub / Neg / Rem and their assign forms — identical for
    // native and wide storage (the `core::ops` impls on `bnum` integers
    // match the primitive integer surface).
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::ops::Add for $Type<SCALE> {
            type Output = Self;
            /// Add two values of the same scale.
            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::AddAssign for $Type<SCALE> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 = self.0 + rhs.0;
            }
        }

        impl<const SCALE: u32> ::core::ops::Sub for $Type<SCALE> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::SubAssign for $Type<SCALE> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 = self.0 - rhs.0;
            }
        }

        impl<const SCALE: u32> ::core::ops::Neg for $Type<SCALE> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::Rem for $Type<SCALE> {
            type Output = Self;
            /// Remainder of two values at the same scale. Because both
            /// operands share the scale factor, the storage-level
            /// remainder is the answer with no rescaling.
            #[inline]
            fn rem(self, rhs: Self) -> Self {
                Self(self.0 % rhs.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::RemAssign for $Type<SCALE> {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 = self.0 % rhs.0;
            }
        }
    };
}

pub(crate) use decl_decimal_arithmetic;
