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
            fn ln_strict(self) -> Self {
                <$Type<SCALE>>::ln_strict(self)
            }
            #[inline]
            fn ln_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::ln_strict_with(self, mode)
            }
            #[inline]
            fn ln_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::ln_approx(self, working_digits)
            }
            #[inline]
            fn ln_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::ln_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn log_strict(self, base: Self) -> Self {
                <$Type<SCALE>>::log_strict(self, base)
            }
            #[inline]
            fn log_strict_with(
                self,
                base: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::log_strict_with(self, base, mode)
            }
            #[inline]
            fn log_approx(self, base: Self, working_digits: u32) -> Self {
                <$Type<SCALE>>::log_approx(self, base, working_digits)
            }
            #[inline]
            fn log_approx_with(
                self,
                base: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::log_approx_with(self, base, working_digits, mode)
            }

            #[inline]
            fn log2_strict(self) -> Self {
                <$Type<SCALE>>::log2_strict(self)
            }
            #[inline]
            fn log2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::log2_strict_with(self, mode)
            }
            #[inline]
            fn log2_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::log2_approx(self, working_digits)
            }
            #[inline]
            fn log2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::log2_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn log10_strict(self) -> Self {
                <$Type<SCALE>>::log10_strict(self)
            }
            #[inline]
            fn log10_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::log10_strict_with(self, mode)
            }
            #[inline]
            fn log10_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::log10_approx(self, working_digits)
            }
            #[inline]
            fn log10_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::log10_approx_with(self, working_digits, mode)
            }

            // ── Exponentials ─────────────────────────────
            #[inline]
            fn exp_strict(self) -> Self {
                <$Type<SCALE>>::exp_strict(self)
            }
            #[inline]
            fn exp_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::exp_strict_with(self, mode)
            }
            #[inline]
            fn exp_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::exp_approx(self, working_digits)
            }
            #[inline]
            fn exp_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::exp_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn exp2_strict(self) -> Self {
                <$Type<SCALE>>::exp2_strict(self)
            }
            #[inline]
            fn exp2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::exp2_strict_with(self, mode)
            }
            #[inline]
            fn exp2_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::exp2_approx(self, working_digits)
            }
            #[inline]
            fn exp2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::exp2_approx_with(self, working_digits, mode)
            }

            // ── Power ────────────────────────────────────
            #[inline]
            fn powf_strict(self, exp: Self) -> Self {
                <$Type<SCALE>>::powf_strict(self, exp)
            }
            #[inline]
            fn powf_strict_with(
                self,
                exp: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::powf_strict_with(self, exp, mode)
            }
            #[inline]
            fn powf_approx(self, exp: Self, working_digits: u32) -> Self {
                <$Type<SCALE>>::powf_approx(self, exp, working_digits)
            }
            #[inline]
            fn powf_approx_with(
                self,
                exp: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::powf_approx_with(self, exp, working_digits, mode)
            }

            // ── Roots ────────────────────────────────────
            #[inline]
            fn sqrt_strict(self) -> Self {
                <$Type<SCALE>>::sqrt_strict(self)
            }
            #[inline]
            fn sqrt_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::sqrt_strict_with(self, mode)
            }
            #[inline]
            fn cbrt_strict(self) -> Self {
                <$Type<SCALE>>::cbrt_strict(self)
            }
            #[inline]
            fn cbrt_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::cbrt_strict_with(self, mode)
            }
            #[inline]
            fn hypot_strict(self, other: Self) -> Self {
                <$Type<SCALE>>::hypot_strict(self, other)
            }
            #[inline]
            fn hypot_strict_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::hypot_strict_with(self, other, mode)
            }

            // ── Trig (forward) ───────────────────────────
            #[inline]
            fn sin_strict(self) -> Self {
                <$Type<SCALE>>::sin_strict(self)
            }
            #[inline]
            fn sin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::sin_strict_with(self, mode)
            }
            #[inline]
            fn sin_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::sin_approx(self, working_digits)
            }
            #[inline]
            fn sin_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::sin_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn cos_strict(self) -> Self {
                <$Type<SCALE>>::cos_strict(self)
            }
            #[inline]
            fn cos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::cos_strict_with(self, mode)
            }
            #[inline]
            fn cos_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::cos_approx(self, working_digits)
            }
            #[inline]
            fn cos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::cos_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn tan_strict(self) -> Self {
                <$Type<SCALE>>::tan_strict(self)
            }
            #[inline]
            fn tan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::tan_strict_with(self, mode)
            }
            #[inline]
            fn tan_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::tan_approx(self, working_digits)
            }
            #[inline]
            fn tan_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::tan_approx_with(self, working_digits, mode)
            }

            // ── Trig (inverse) ───────────────────────────
            #[inline]
            fn atan_strict(self) -> Self {
                <$Type<SCALE>>::atan_strict(self)
            }
            #[inline]
            fn atan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::atan_strict_with(self, mode)
            }
            #[inline]
            fn atan_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::atan_approx(self, working_digits)
            }
            #[inline]
            fn atan_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::atan_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn asin_strict(self) -> Self {
                <$Type<SCALE>>::asin_strict(self)
            }
            #[inline]
            fn asin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::asin_strict_with(self, mode)
            }
            #[inline]
            fn asin_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::asin_approx(self, working_digits)
            }
            #[inline]
            fn asin_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::asin_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn acos_strict(self) -> Self {
                <$Type<SCALE>>::acos_strict(self)
            }
            #[inline]
            fn acos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::acos_strict_with(self, mode)
            }
            #[inline]
            fn acos_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::acos_approx(self, working_digits)
            }
            #[inline]
            fn acos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::acos_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn atan2_strict(self, other: Self) -> Self {
                <$Type<SCALE>>::atan2_strict(self, other)
            }
            #[inline]
            fn atan2_strict_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::atan2_strict_with(self, other, mode)
            }
            #[inline]
            fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
                <$Type<SCALE>>::atan2_approx(self, other, working_digits)
            }
            #[inline]
            fn atan2_approx_with(
                self,
                other: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::atan2_approx_with(self, other, working_digits, mode)
            }

            // ── Hyperbolic ───────────────────────────────
            #[inline]
            fn sinh_strict(self) -> Self {
                <$Type<SCALE>>::sinh_strict(self)
            }
            #[inline]
            fn sinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::sinh_strict_with(self, mode)
            }
            #[inline]
            fn sinh_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::sinh_approx(self, working_digits)
            }
            #[inline]
            fn sinh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::sinh_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn cosh_strict(self) -> Self {
                <$Type<SCALE>>::cosh_strict(self)
            }
            #[inline]
            fn cosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::cosh_strict_with(self, mode)
            }
            #[inline]
            fn cosh_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::cosh_approx(self, working_digits)
            }
            #[inline]
            fn cosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::cosh_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn tanh_strict(self) -> Self {
                <$Type<SCALE>>::tanh_strict(self)
            }
            #[inline]
            fn tanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::tanh_strict_with(self, mode)
            }
            #[inline]
            fn tanh_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::tanh_approx(self, working_digits)
            }
            #[inline]
            fn tanh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::tanh_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn asinh_strict(self) -> Self {
                <$Type<SCALE>>::asinh_strict(self)
            }
            #[inline]
            fn asinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::asinh_strict_with(self, mode)
            }
            #[inline]
            fn asinh_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::asinh_approx(self, working_digits)
            }
            #[inline]
            fn asinh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::asinh_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn acosh_strict(self) -> Self {
                <$Type<SCALE>>::acosh_strict(self)
            }
            #[inline]
            fn acosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::acosh_strict_with(self, mode)
            }
            #[inline]
            fn acosh_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::acosh_approx(self, working_digits)
            }
            #[inline]
            fn acosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::acosh_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn atanh_strict(self) -> Self {
                <$Type<SCALE>>::atanh_strict(self)
            }
            #[inline]
            fn atanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::atanh_strict_with(self, mode)
            }
            #[inline]
            fn atanh_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::atanh_approx(self, working_digits)
            }
            #[inline]
            fn atanh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::atanh_approx_with(self, working_digits, mode)
            }

            // ── Angle conversion ─────────────────────────
            #[inline]
            fn to_degrees_strict(self) -> Self {
                <$Type<SCALE>>::to_degrees_strict(self)
            }
            #[inline]
            fn to_degrees_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::to_degrees_strict_with(self, mode)
            }
            #[inline]
            fn to_degrees_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::to_degrees_approx(self, working_digits)
            }
            #[inline]
            fn to_degrees_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::to_degrees_approx_with(self, working_digits, mode)
            }

            #[inline]
            fn to_radians_strict(self) -> Self {
                <$Type<SCALE>>::to_radians_strict(self)
            }
            #[inline]
            fn to_radians_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                <$Type<SCALE>>::to_radians_strict_with(self, mode)
            }
            #[inline]
            fn to_radians_approx(self, working_digits: u32) -> Self {
                <$Type<SCALE>>::to_radians_approx(self, working_digits)
            }
            #[inline]
            fn to_radians_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                <$Type<SCALE>>::to_radians_approx_with(self, working_digits, mode)
            }
        }
    };
}

pub(crate) use decl_decimal_transcendental_impl;
