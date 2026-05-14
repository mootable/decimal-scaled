//! Macro-generated arithmetic operator overloads for narrow decimal
//! widths (D32, D64).
//!
//! For D32 and D64 the storage type is `i32` and `i64` respectively,
//! and a native wider integer (`i64` and `i128`) is available for the
//! mul/div widening step. This is simpler than D128's
//! `mg_divide`-based 256-bit widening; the macro below captures the
//! pattern.
//!
//! Overflow semantics mirror Rust's default integer arithmetic:
//! debug-mode panic on overflow, release-mode wrap. Explicit-overflow
//! variants (`checked_*`, `saturating_*`, `wrapping_*`) live in a
//! companion module.

/// Generates the standard arithmetic operator overloads for a decimal
/// width `$Type<SCALE>` with storage `$Storage` and widening type
/// `$Wider`. `$Wider` must satisfy
/// `<Storage as Into<Wider>>::into` and `(Wider).pow(SCALE)` must fit
/// in `Wider`.
macro_rules! decl_decimal_arithmetic {
    ($Type:ident, $Storage:ty, $Wider:ty) => {
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
                self.0 += rhs.0;
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
                self.0 -= rhs.0;
            }
        }

        impl<const SCALE: u32> ::core::ops::Neg for $Type<SCALE> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale.
            ///
            /// Uses native widening to `$Wider` to compute `a * b`
            /// without intermediate overflow, then divides by `10^SCALE`
            /// and narrows back to `$Storage`. The narrowing cast wraps
            /// if the result is out of range (release) or panics (debug),
            /// matching Rust's standard `as`-cast behaviour for primitive
            /// integers.
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
            /// Divide two values of the same scale.
            ///
            /// Numerator is widened to `$Wider` and multiplied by
            /// `10^SCALE` before division by `b`, preserving the
            /// `value * 10^SCALE` form.
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
                self.0 %= rhs.0;
            }
        }
    };
}

pub(crate) use decl_decimal_arithmetic;
