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

/// Rounds `n / m` (truncating-toward-zero quotient) according to
/// `$mode` (a [`RoundingMode`]) for *primitive* signed integer types
/// (`i32` / `i64` / `i128`).
///
/// Mode-specific behaviour is delegated to
/// [`crate::rounding::should_bump`], which receives the three
/// pre-computed inputs every mode needs: the `|r|` vs `|m|−|r|`
/// ordering (the round-up test without the `2·|r|` overflow risk),
/// the parity of the truncated quotient, and the result sign. The
/// caller bumps the quotient by ±1 in the result direction.
///
/// Passing `crate::rounding::DEFAULT_ROUNDING_MODE` yields the
/// crate-wide default (IEEE-754 round-half-to-even unless a
/// `rounding-*` feature overrides it).
///
/// [`RoundingMode`]: crate::rounding::RoundingMode
macro_rules! round_with_mode_native {
    ($n:expr, $m:expr, $mode:expr) => {{
        let n = $n;
        let m = $m;
        let mode = $mode;
        let q = n / m;
        let r = n % m;
        if r == 0 {
            q
        } else {
            let abs_r = if r < 0 { -r } else { r };
            let abs_m = if m < 0 { -m } else { m };
            let comp = abs_m - abs_r;
            let cmp_r = abs_r.cmp(&comp);
            let q_is_odd = (q & 1) != 0;
            let result_positive = (n < 0) == (m < 0);
            if $crate::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
                if result_positive {
                    q + 1
                } else {
                    q - 1
                }
            } else {
                q
            }
        }
    }};
}
pub(crate) use round_with_mode_native;

/// Wide-storage counterpart of [`round_with_mode_native!`] — the same
/// strategy-pattern dispatch over [`crate::rounding::should_bump`],
/// adapted to a hand-rolled wide integer `$W`. Uses
/// `<$W>::from_i128(0/1)` for the small constants and the type's
/// operators throughout.
macro_rules! round_with_mode_wide {
    ($n:expr, $m:expr, $W:ty, $mode:expr) => {{
        let n = $n;
        let m = $m;
        let mode = $mode;
        let q = n / m;
        let r = n % m;
        let zero = <$W>::from_i128(0);
        if r == zero {
            q
        } else {
            let one = <$W>::from_i128(1);
            let abs_r = if r < zero { -r } else { r };
            let abs_m = if m < zero { -m } else { m };
            let comp = abs_m - abs_r;
            let cmp_r = abs_r.cmp(&comp);
            let q_is_odd = {
                let two = <$W>::from_i128(2);
                (q % two) != zero
            };
            let result_positive = (n < zero) == (m < zero);
            if $crate::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
                if result_positive {
                    q + one
                } else {
                    q - one
                }
            } else {
                q
            }
        }
    }};
}
pub(crate) use round_with_mode_wide;

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
            /// to hold `a · b` exactly, divides by `10^SCALE` using the
            /// crate-default [`RoundingMode`] (IEEE-754 round-to-nearest;
            /// within 0.5 ULP), and narrows back to `$Storage`. See
            /// [`Self::mul_with`] to choose a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::rounding::RoundingMode
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.mul_with(rhs, $crate::rounding::DEFAULT_ROUNDING_MODE)
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
            /// Divide two values of the same scale using the crate-default
            /// [`RoundingMode`] (within 0.5 ULP). Numerator is widened to
            /// `$Wider`, multiplied by `10^SCALE`, then divided by `b`
            /// preserving the `value · 10^SCALE` form. See
            /// [`Self::div_with`] for a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::rounding::RoundingMode
            #[inline]
            fn div(self, rhs: Self) -> Self {
                self.div_with(rhs, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> $Type<SCALE> {
            /// Multiply two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Result is
            /// within 0.5 ULP for the half-* family and bounded by the
            /// directed-rounding rule otherwise.
            #[inline]
            pub fn mul_with(self, rhs: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = <$Wider>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
                let n = a * b;
                let scaled =
                    $crate::macros::arithmetic::round_with_mode_wide!(n, m, $Wider, mode);
                Self(scaled.resize::<$Storage>())
            }

            /// Divide two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            #[inline]
            pub fn div_with(self, rhs: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = <$Wider>::from_str_radix("10", 10)
                    .expect("wide decimal: invalid base-10 literal")
                    .pow(SCALE);
                let n = a * m;
                let result =
                    $crate::macros::arithmetic::round_with_mode_wide!(n, b, $Wider, mode);
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
            /// to hold `a · b` exactly, divides by `10^SCALE` using the
            /// crate-default [`RoundingMode`] (IEEE-754 round-to-nearest;
            /// within 0.5 ULP), and narrows back to `$Storage`. See
            /// [`Self::mul_with`] to choose a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::rounding::RoundingMode
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.mul_with(rhs, $crate::rounding::DEFAULT_ROUNDING_MODE)
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
            /// Divide two values of the same scale using the crate-default
            /// [`RoundingMode`] (within 0.5 ULP). Numerator is widened to
            /// `$Wider`, multiplied by `10^SCALE`, then divided by `b`
            /// preserving the `value · 10^SCALE` form. See
            /// [`Self::div_with`] for a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::rounding::RoundingMode
            #[inline]
            fn div(self, rhs: Self) -> Self {
                self.div_with(rhs, $crate::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> $Type<SCALE> {
            /// Multiply two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            #[inline]
            pub fn mul_with(self, rhs: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let n = a * b;
                let scaled =
                    $crate::macros::arithmetic::round_with_mode_native!(n, m, mode);
                Self(scaled as $Storage)
            }

            /// Divide two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            #[inline]
            pub fn div_with(self, rhs: Self, mode: $crate::rounding::RoundingMode) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let n = a * m;
                let result =
                    $crate::macros::arithmetic::round_with_mode_native!(n, b, mode);
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
