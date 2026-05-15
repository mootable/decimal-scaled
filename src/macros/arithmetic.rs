//! Macro-generated arithmetic operator overloads for the decimal
//! widths that use a *uniform* mul/div pattern (D32, D64, and the wide
//! tier D256 / D512 / D1024).
//!
//! For D32 / D64 the storage type is a primitive (`i32` / `i64`) and a
//! native wider integer (`i64` / `i128`) carries the mul/div widening
//! step. For D256 / D512 / D1024 the storage type is a hand-rolled wide integer
//! fixed-width integer and the widening type is the next size up
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

/// Rounds `n / m` (truncating-toward-zero quotient) to nearest, ties
/// to even — the IEEE-754 round-to-nearest contract — for *primitive*
/// signed integer types (`i32` / `i64` / `i128`).
///
/// The decision is made by comparing `|r|` against `|m| − |r|`
/// (equivalent to comparing `2·|r|` against `|m|` but without the
/// doubling-overflow risk).
macro_rules! round_half_to_even_native {
    ($n:expr, $m:expr) => {{
        let n = $n;
        let m = $m;
        let q = n / m;
        let r = n % m;
        let abs_r = if r < 0 { -r } else { r };
        let abs_m = if m < 0 { -m } else { m };
        let comp = abs_m - abs_r;
        if abs_r > comp || (abs_r == comp && abs_r > 0 && (q & 1) != 0) {
            if q < 0 {
                q - 1
            } else if q > 0 {
                q + 1
            } else if (n < 0) != (m < 0) {
                q - 1
            } else {
                q + 1
            }
        } else {
            q
        }
    }};
}
pub(crate) use round_half_to_even_native;

/// Wide-storage counterpart of `round_half_to_even_native!` — the same
/// half-to-even rounding algorithm on a hand-rolled wide integer
/// `$W`. Uses `<$W>::from_i128` for the small integer constants and
/// the type's operators throughout.
macro_rules! round_half_to_even_wide {
    ($n:expr, $m:expr, $W:ty) => {{
        let n = $n;
        let m = $m;
        let q = n / m;
        let r = n % m;
        let zero = <$W>::from_i128(0);
        let one = <$W>::from_i128(1);
        let two = <$W>::from_i128(2);
        let abs_r = if r < zero { -r } else { r };
        let abs_m = if m < zero { -m } else { m };
        let comp = abs_m - abs_r;
        let q_odd = (q % two) != zero;
        if abs_r > comp || (abs_r == comp && abs_r > zero && q_odd) {
            if q < zero {
                q - one
            } else if q > zero {
                q + one
            } else if (n < zero) != (m < zero) {
                q - one
            } else {
                q + one
            }
        } else {
            q
        }
    }};
}
pub(crate) use round_half_to_even_wide;

/// Generates the standard arithmetic operator overloads for a decimal
/// width `$Type<SCALE>`.
///
/// - `decl_decimal_arithmetic!(D32, i32, i64)` — *native* storage; the
/// widening type is a primitive integer, `as`-casts and the
/// `(10 as $Wider)` literal carry the mul/div step.
/// - `decl_decimal_arithmetic!(wide D256, I256, I512)` — *wide*
/// storage; the widening type is a hand-rolled wide integer, the `WideInt` cast
/// carries the width casts and `from_str_radix` builds the
/// `10^SCALE` factor.
macro_rules! decl_decimal_arithmetic {
    // Wide storage.
    (wide $Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@common $Type, $Storage);

        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale. Widens to `$Wider`
            /// to hold `a · b` exactly, divides by `10^SCALE` rounded
            /// half-to-even (the IEEE-754 round-to-nearest contract;
            /// within 0.5 ULP), and narrows back to `$Storage`.
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = <$Wider>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
                let n = a * b;
                let scaled =
                    $crate::macros::arithmetic::round_half_to_even_wide!(n, m, $Wider);
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
            /// to `$Wider`, multiplied by `10^SCALE`, then divided by
            /// `b` rounded half-to-even (within 0.5 ULP), preserving
            /// the `value · 10^SCALE` form.
            #[inline]
            fn div(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = <$Wider>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
                let n = a * m;
                let result =
                    $crate::macros::arithmetic::round_half_to_even_wide!(n, b, $Wider);
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
            /// to hold `a · b` exactly, divides by `10^SCALE` rounded
            /// half-to-even (within 0.5 ULP — the IEEE-754
            /// round-to-nearest contract), and narrows back to
            /// `$Storage`.
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let n = a * b;
                let scaled =
                    $crate::macros::arithmetic::round_half_to_even_native!(n, m);
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
            /// to `$Wider`, multiplied by `10^SCALE`, then divided by
            /// `b` rounded half-to-even (within 0.5 ULP), preserving
            /// the `value · 10^SCALE` form.
            #[inline]
            fn div(self, rhs: Self) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let n = a * m;
                let result =
                    $crate::macros::arithmetic::round_half_to_even_native!(n, b);
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
    // native and wide storage (the `core::ops` impls on the wide integers
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
