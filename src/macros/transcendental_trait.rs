// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Emits the [`DecimalTranscendental`] impl for a decimal width.
//!
//! Every method is a one-line delegator to the inherent method of
//! the same name. `#[inline]` lets LLVM erase the trait dispatch,
//! so generic-over-`DecimalTranscendental` code pays no runtime
//! cost compared to calling the inherent method directly.

/// Emits `impl<const SCALE: u32> DecimalTranscendental for $Type<SCALE>`
/// for a decimal type. Requires the type to have every inherent
/// method named in the trait (see `decl_strict_transcendentals!` and
/// `decl_wide_transcendental!` for the per-width method emissions).
macro_rules! decl_decimal_transcendental_impl {
    ($Type:ident) => {
        impl<const SCALE: u32> $crate::types::traits::transcendental::DecimalTranscendental
            for $Type<SCALE>
        {
            // ── Logarithms ───────────────────────────────
            #[inline]
            fn ln(self) -> Self {
                <$Type<SCALE>>::ln(self)
            }
            #[inline]
            fn ln_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::ln_with(self, mode)
            }

            #[inline]
            fn log1p(self) -> Self {
                <$Type<SCALE>>::log1p(self)
            }
            #[inline]
            fn log1p_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::log1p_with(self, mode)
            }

            #[inline]
            fn expm1(self) -> Self {
                <$Type<SCALE>>::expm1(self)
            }
            #[inline]
            fn expm1_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::expm1_with(self, mode)
            }

            #[inline]
            fn log(self, base: Self) -> Self {
                <$Type<SCALE>>::log(self, base)
            }
            #[inline]
            fn log_with(
                self,
                base: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::log_with(self, base, mode)
            }

            #[inline]
            fn log2(self) -> Self {
                <$Type<SCALE>>::log2(self)
            }
            #[inline]
            fn log2_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::log2_with(self, mode)
            }

            #[inline]
            fn log10(self) -> Self {
                <$Type<SCALE>>::log10(self)
            }
            #[inline]
            fn log10_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::log10_with(self, mode)
            }

            // ── Exponentials ─────────────────────────────
            #[inline]
            fn exp(self) -> Self {
                <$Type<SCALE>>::exp(self)
            }
            #[inline]
            fn exp_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::exp_with(self, mode)
            }

            #[inline]
            fn exp2(self) -> Self {
                <$Type<SCALE>>::exp2(self)
            }
            #[inline]
            fn exp2_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::exp2_with(self, mode)
            }

            // ── Power ────────────────────────────────────
            #[inline]
            fn powf(self, exp: Self) -> Self {
                <$Type<SCALE>>::powf(self, exp)
            }
            #[inline]
            fn powf_with(
                self,
                exp: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::powf_with(self, exp, mode)
            }

            // ── Roots ────────────────────────────────────
            #[inline]
            fn sqrt(self) -> Self {
                <$Type<SCALE>>::sqrt(self)
            }
            #[inline]
            fn sqrt_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::sqrt_with(self, mode)
            }
            #[inline]
            fn cbrt(self) -> Self {
                <$Type<SCALE>>::cbrt(self)
            }
            #[inline]
            fn cbrt_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::cbrt_with(self, mode)
            }
            #[inline]
            fn hypot(self, other: Self) -> Self {
                <$Type<SCALE>>::hypot(self, other)
            }
            #[inline]
            fn hypot_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::hypot_with(self, other, mode)
            }

            // ── Trig (forward) ───────────────────────────
            #[inline]
            fn sin(self) -> Self {
                <$Type<SCALE>>::sin(self)
            }
            #[inline]
            fn sin_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::sin_with(self, mode)
            }

            #[inline]
            fn cos(self) -> Self {
                <$Type<SCALE>>::cos(self)
            }
            #[inline]
            fn cos_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::cos_with(self, mode)
            }

            #[inline]
            fn tan(self) -> Self {
                <$Type<SCALE>>::tan(self)
            }
            #[inline]
            fn tan_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::tan_with(self, mode)
            }

            // ── Trig (inverse) ───────────────────────────
            #[inline]
            fn atan(self) -> Self {
                <$Type<SCALE>>::atan(self)
            }
            #[inline]
            fn atan_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::atan_with(self, mode)
            }

            #[inline]
            fn asin(self) -> Self {
                <$Type<SCALE>>::asin(self)
            }
            #[inline]
            fn asin_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::asin_with(self, mode)
            }

            #[inline]
            fn acos(self) -> Self {
                <$Type<SCALE>>::acos(self)
            }
            #[inline]
            fn acos_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::acos_with(self, mode)
            }

            #[inline]
            fn atan2(self, other: Self) -> Self {
                <$Type<SCALE>>::atan2(self, other)
            }
            #[inline]
            fn atan2_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::atan2_with(self, other, mode)
            }

            // ── Hyperbolic ───────────────────────────────
            #[inline]
            fn sinh(self) -> Self {
                <$Type<SCALE>>::sinh(self)
            }
            #[inline]
            fn sinh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::sinh_with(self, mode)
            }

            #[inline]
            fn cosh(self) -> Self {
                <$Type<SCALE>>::cosh(self)
            }
            #[inline]
            fn cosh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::cosh_with(self, mode)
            }

            #[inline]
            fn tanh(self) -> Self {
                <$Type<SCALE>>::tanh(self)
            }
            #[inline]
            fn tanh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::tanh_with(self, mode)
            }

            #[inline]
            fn asinh(self) -> Self {
                <$Type<SCALE>>::asinh(self)
            }
            #[inline]
            fn asinh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::asinh_with(self, mode)
            }

            #[inline]
            fn acosh(self) -> Self {
                <$Type<SCALE>>::acosh(self)
            }
            #[inline]
            fn acosh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::acosh_with(self, mode)
            }

            #[inline]
            fn atanh(self) -> Self {
                <$Type<SCALE>>::atanh(self)
            }
            #[inline]
            fn atanh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::atanh_with(self, mode)
            }

            // ── Angle conversion ─────────────────────────
            #[inline]
            fn to_degrees(self) -> Self {
                <$Type<SCALE>>::to_degrees(self)
            }
            #[inline]
            fn to_degrees_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::to_degrees_with(self, mode)
            }

            #[inline]
            fn to_radians(self) -> Self {
                <$Type<SCALE>>::to_radians(self)
            }
            #[inline]
            fn to_radians_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::to_radians_with(self, mode)
            }
        }
    };
}

pub(crate) use decl_decimal_transcendental_impl;
