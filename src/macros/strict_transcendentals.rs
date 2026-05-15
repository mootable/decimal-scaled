//! Macro-generated strict-mode transcendentals for the narrow decimal
//! widths (D32 / D64), by delegation to the D128 strict path.
//!
//! For each method the input is widened to `D128<SCALE>`, the D128
//! `*_strict` implementation is called, and the result is narrowed
//! back. This gives D32 / D64 the full integer-only transcendental
//! surface (ln, log, log2, log10, exp, exp2, sqrt, cbrt, powf, and the
//! trig / hyperbolic / angle family) without duplicating the
//! algorithmic work. The narrowing step panics if the result exceeds
//! the target storage's range.
//!
//! Two surfaces are emitted per method, mirroring the D128 layout:
//!
//! - `<method>_strict` — always present unless the `no_strict` feature
//! is set. Integer-only; `no_std`-compatible.
//! - `<method>` — a dispatcher present only under
//! `#[cfg(all(feature = "strict", not(feature = "no_strict")))]`,
//! forwarding to `<method>_strict`. (D32 / D64 have no f64-bridge
//! transcendentals of their own, so there is no non-strict `<method>`
//! for these widths.)

/// Emits the strict-mode transcendental surface for `$Type<SCALE>` by
/// delegating to the D128 `*_strict` implementations.
macro_rules! decl_strict_transcendentals_via_d128 {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn ln_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln_strict())
                    .expect(concat!(stringify!($Type), "::ln_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn log2_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_strict())
                    .expect(concat!(stringify!($Type), "::log2_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn log10_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_strict())
                    .expect(concat!(stringify!($Type), "::log10_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn exp_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp_strict())
                    .expect(concat!(stringify!($Type), "::exp_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn exp2_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_strict())
                    .expect(concat!(stringify!($Type), "::exp2_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn sqrt_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sqrt_strict())
                    .expect(concat!(stringify!($Type), "::sqrt_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn cbrt_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cbrt_strict())
                    .expect(concat!(stringify!($Type), "::cbrt_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn sin_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin_strict())
                    .expect(concat!(stringify!($Type), "::sin_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn cos_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos_strict())
                    .expect(concat!(stringify!($Type), "::cos_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn tan_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan_strict())
                    .expect(concat!(stringify!($Type), "::tan_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn asin_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin_strict())
                    .expect(concat!(stringify!($Type), "::asin_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn acos_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos_strict())
                    .expect(concat!(stringify!($Type), "::acos_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn atan_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan_strict())
                    .expect(concat!(stringify!($Type), "::atan_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn sinh_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict())
                    .expect(concat!(stringify!($Type), "::sinh_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn cosh_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict())
                    .expect(concat!(stringify!($Type), "::cosh_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn tanh_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict())
                    .expect(concat!(stringify!($Type), "::tanh_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn asinh_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict())
                    .expect(concat!(stringify!($Type), "::asinh_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn acosh_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict())
                    .expect(concat!(stringify!($Type), "::acosh_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn atanh_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict())
                    .expect(concat!(stringify!($Type), "::atanh_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn to_degrees_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict())
                    .expect(concat!(stringify!($Type), "::to_degrees_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn to_radians_strict(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict())
                    .expect(concat!(stringify!($Type), "::to_radians_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn log_strict(self, base: Self) -> Self {
                let wide_self: $crate::core_type::D128<SCALE> = self.into();
                let wide_base: $crate::core_type::D128<SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_strict(wide_base))
                    .expect(concat!(stringify!($Type), "::log_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn atan2_strict(self, other: Self) -> Self {
                let wide_self: $crate::core_type::D128<SCALE> = self.into();
                let wide_other: $crate::core_type::D128<SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.atan2_strict(wide_other))
                    .expect(concat!(stringify!($Type), "::atan2_strict: result out of range"))
            }
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn powf_strict(self, exp: Self) -> Self {
                let wide_self: $crate::core_type::D128<SCALE> = self.into();
                let wide_exp: $crate::core_type::D128<SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf_strict(wide_exp))
                    .expect(concat!(stringify!($Type), "::powf_strict: result out of range"))
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                self.ln_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                self.log2_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                self.log10_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                self.exp_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                self.exp2_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                self.sqrt_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                self.cbrt_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                self.sin_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                self.cos_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                self.tan_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                self.asin_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                self.acos_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                self.atan_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                self.sinh_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                self.cosh_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                self.tanh_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                self.asinh_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                self.acosh_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                self.atanh_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                self.to_degrees_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                self.to_radians_strict()
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                self.log_strict(base)
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                self.atan2_strict(other)
            }
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                self.powf_strict(exp)
            }
        }
    };
}

pub(crate) use decl_strict_transcendentals_via_d128;
