//! Macro-generated strict-mode transcendentals for narrow decimal
//! widths by delegation to the D128 path.
//!
//! For each narrow width we widen the input to `D128<SCALE>`, call
//! the D128 strict transcendental, then narrow the result back. This
//! gives D32 / D64 the full strict surface (ln, log, log2, log10,
//! exp, exp2, sqrt, cbrt, powf) without duplicating the algorithmic
//! work. The narrowing step panics if the result exceeds the target
//! storage's range.

/// Emits the strict-mode transcendental methods for `$Type<SCALE>`
/// by delegating to D128.
macro_rules! decl_strict_transcendentals_via_d128 {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln())
                    .expect(concat!(stringify!($Type), "::ln: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                let wide_self: $crate::core_type::D128<SCALE> = self.into();
                let wide_base: $crate::core_type::D128<SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log(wide_base))
                    .expect(concat!(stringify!($Type), "::log: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2())
                    .expect(concat!(stringify!($Type), "::log2: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10())
                    .expect(concat!(stringify!($Type), "::log10: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp())
                    .expect(concat!(stringify!($Type), "::exp: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2())
                    .expect(concat!(stringify!($Type), "::exp2: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sqrt())
                    .expect(concat!(stringify!($Type), "::sqrt: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cbrt())
                    .expect(concat!(stringify!($Type), "::cbrt: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                let wide_self: $crate::core_type::D128<SCALE> = self.into();
                let wide_exp: $crate::core_type::D128<SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf(wide_exp))
                    .expect(concat!(stringify!($Type), "::powf: result out of range"))
            }

            // ── Trigonometric / hyperbolic / angle-conversion ─────────
            // Same widen-compute-narrow delegation as above.

            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin())
                    .expect(concat!(stringify!($Type), "::sin: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos())
                    .expect(concat!(stringify!($Type), "::cos: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan())
                    .expect(concat!(stringify!($Type), "::tan: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin())
                    .expect(concat!(stringify!($Type), "::asin: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos())
                    .expect(concat!(stringify!($Type), "::acos: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan())
                    .expect(concat!(stringify!($Type), "::atan: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                let wide_self: $crate::core_type::D128<SCALE> = self.into();
                let wide_other: $crate::core_type::D128<SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.atan2(wide_other))
                    .expect(concat!(stringify!($Type), "::atan2: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh())
                    .expect(concat!(stringify!($Type), "::sinh: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh())
                    .expect(concat!(stringify!($Type), "::cosh: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh())
                    .expect(concat!(stringify!($Type), "::tanh: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh())
                    .expect(concat!(stringify!($Type), "::asinh: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh())
                    .expect(concat!(stringify!($Type), "::acosh: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh())
                    .expect(concat!(stringify!($Type), "::atanh: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees())
                    .expect(concat!(stringify!($Type), "::to_degrees: result out of range"))
            }
            #[cfg(feature = "strict")]
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                let wide: $crate::core_type::D128<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians())
                    .expect(concat!(stringify!($Type), "::to_radians: result out of range"))
            }
        }
    };
}

pub(crate) use decl_strict_transcendentals_via_d128;
