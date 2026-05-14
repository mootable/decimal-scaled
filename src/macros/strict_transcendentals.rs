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
        }
    };
}

pub(crate) use decl_strict_transcendentals_via_d128;
