//! Macro-generated strict-mode transcendentals for the narrow decimal
//! widths (D9 / D18), by delegation to the D38 strict path.
//!
//! For each method the input is widened to `D38<SCALE>`, the D38
//! `*_strict` implementation is called, and the result is narrowed
//! back. This gives D9 / D18 the full integer-only transcendental
//! surface (ln, log, log2, log10, exp, exp2, sqrt, cbrt, powf, and the
//! trig / hyperbolic / angle family) without duplicating the
//! algorithmic work. The narrowing step panics if the result exceeds
//! the target storage's range.
//!
//! Two surfaces are emitted per method, mirroring the D38 layout:
//!
//! - `<method>_strict` — always present unless the `fast` feature
//! is set. Integer-only; `no_std`-compatible.
//! - `<method>` — a dispatcher present only under
//! `#[cfg(not(all(feature = "fast", not(feature = "strict"))))]`,
//! forwarding to `<method>_strict`. (D9 / D18 have no f64-bridge
//! transcendentals of their own, so there is no non-strict `<method>`
//! for these widths.)

/// Emits the strict-mode transcendental surface for `$Type<SCALE>` by
/// delegating to the D38 `*_strict` implementations.
macro_rules! decl_strict_transcendentals_via_d38 {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// `ln_strict` — delegates to [`crate::core_type::D38::ln_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn ln_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln_strict())
                    .expect(concat!(stringify!($Type), "::ln_strict: result out of range"))
            }
            /// `log2_strict` — delegates to [`crate::core_type::D38::log2_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log2_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_strict())
                    .expect(concat!(stringify!($Type), "::log2_strict: result out of range"))
            }
            /// `log10_strict` — delegates to [`crate::core_type::D38::log10_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log10_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_strict())
                    .expect(concat!(stringify!($Type), "::log10_strict: result out of range"))
            }
            /// `exp_strict` — delegates to [`crate::core_type::D38::exp_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn exp_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp_strict())
                    .expect(concat!(stringify!($Type), "::exp_strict: result out of range"))
            }
            /// `exp2_strict` — delegates to [`crate::core_type::D38::exp2_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn exp2_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_strict())
                    .expect(concat!(stringify!($Type), "::exp2_strict: result out of range"))
            }
            /// `sqrt_strict` — delegates to [`crate::core_type::D38::sqrt_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn sqrt_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sqrt_strict())
                    .expect(concat!(stringify!($Type), "::sqrt_strict: result out of range"))
            }
            /// `cbrt_strict` — delegates to [`crate::core_type::D38::cbrt_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cbrt_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cbrt_strict())
                    .expect(concat!(stringify!($Type), "::cbrt_strict: result out of range"))
            }
            /// `sin_strict` — delegates to [`crate::core_type::D38::sin_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn sin_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin_strict())
                    .expect(concat!(stringify!($Type), "::sin_strict: result out of range"))
            }
            /// `cos_strict` — delegates to [`crate::core_type::D38::cos_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cos_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos_strict())
                    .expect(concat!(stringify!($Type), "::cos_strict: result out of range"))
            }
            /// `tan_strict` — delegates to [`crate::core_type::D38::tan_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn tan_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan_strict())
                    .expect(concat!(stringify!($Type), "::tan_strict: result out of range"))
            }
            /// `asin_strict` — delegates to [`crate::core_type::D38::asin_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn asin_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin_strict())
                    .expect(concat!(stringify!($Type), "::asin_strict: result out of range"))
            }
            /// `acos_strict` — delegates to [`crate::core_type::D38::acos_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn acos_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos_strict())
                    .expect(concat!(stringify!($Type), "::acos_strict: result out of range"))
            }
            /// `atan_strict` — delegates to [`crate::core_type::D38::atan_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn atan_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan_strict())
                    .expect(concat!(stringify!($Type), "::atan_strict: result out of range"))
            }
            /// `sinh_strict` — delegates to [`crate::core_type::D38::sinh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn sinh_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict())
                    .expect(concat!(stringify!($Type), "::sinh_strict: result out of range"))
            }
            /// `cosh_strict` — delegates to [`crate::core_type::D38::cosh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cosh_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict())
                    .expect(concat!(stringify!($Type), "::cosh_strict: result out of range"))
            }
            /// `tanh_strict` — delegates to [`crate::core_type::D38::tanh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn tanh_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict())
                    .expect(concat!(stringify!($Type), "::tanh_strict: result out of range"))
            }
            /// `asinh_strict` — delegates to [`crate::core_type::D38::asinh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn asinh_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict())
                    .expect(concat!(stringify!($Type), "::asinh_strict: result out of range"))
            }
            /// `acosh_strict` — delegates to [`crate::core_type::D38::acosh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn acosh_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict())
                    .expect(concat!(stringify!($Type), "::acosh_strict: result out of range"))
            }
            /// `atanh_strict` — delegates to [`crate::core_type::D38::atanh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn atanh_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict())
                    .expect(concat!(stringify!($Type), "::atanh_strict: result out of range"))
            }
            /// `to_degrees_strict` — delegates to [`crate::core_type::D38::to_degrees_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn to_degrees_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict())
                    .expect(concat!(stringify!($Type), "::to_degrees_strict: result out of range"))
            }
            /// `to_radians_strict` — delegates to [`crate::core_type::D38::to_radians_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn to_radians_strict(self) -> Self {
                let wide: $crate::core_type::D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict())
                    .expect(concat!(stringify!($Type), "::to_radians_strict: result out of range"))
            }
            /// `log_strict` — delegates to [`crate::core_type::D38::log_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log_strict(self, base: Self) -> Self {
                let wide_self: $crate::core_type::D38<SCALE> = self.into();
                let wide_base: $crate::core_type::D38<SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_strict(wide_base))
                    .expect(concat!(stringify!($Type), "::log_strict: result out of range"))
            }
            /// `atan2_strict` — delegates to [`crate::core_type::D38::atan2_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn atan2_strict(self, other: Self) -> Self {
                let wide_self: $crate::core_type::D38<SCALE> = self.into();
                let wide_other: $crate::core_type::D38<SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.atan2_strict(wide_other))
                    .expect(concat!(stringify!($Type), "::atan2_strict: result out of range"))
            }
            /// `powf_strict` — delegates to [`crate::core_type::D38::powf_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn powf_strict(self, exp: Self) -> Self {
                let wide_self: $crate::core_type::D38<SCALE> = self.into();
                let wide_exp: $crate::core_type::D38<SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf_strict(wide_exp))
                    .expect(concat!(stringify!($Type), "::powf_strict: result out of range"))
            }
            /// `ln` — feature-gated dispatcher; forwards to [`Self::ln_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                self.ln_strict()
            }
            /// `log2` — feature-gated dispatcher; forwards to [`Self::log2_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                self.log2_strict()
            }
            /// `log10` — feature-gated dispatcher; forwards to [`Self::log10_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                self.log10_strict()
            }
            /// `exp` — feature-gated dispatcher; forwards to [`Self::exp_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                self.exp_strict()
            }
            /// `exp2` — feature-gated dispatcher; forwards to [`Self::exp2_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                self.exp2_strict()
            }
            /// `sqrt` — feature-gated dispatcher; forwards to [`Self::sqrt_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                self.sqrt_strict()
            }
            /// `cbrt` — feature-gated dispatcher; forwards to [`Self::cbrt_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                self.cbrt_strict()
            }
            /// `sin` — feature-gated dispatcher; forwards to [`Self::sin_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                self.sin_strict()
            }
            /// `cos` — feature-gated dispatcher; forwards to [`Self::cos_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                self.cos_strict()
            }
            /// `tan` — feature-gated dispatcher; forwards to [`Self::tan_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                self.tan_strict()
            }
            /// `asin` — feature-gated dispatcher; forwards to [`Self::asin_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                self.asin_strict()
            }
            /// `acos` — feature-gated dispatcher; forwards to [`Self::acos_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                self.acos_strict()
            }
            /// `atan` — feature-gated dispatcher; forwards to [`Self::atan_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                self.atan_strict()
            }
            /// `sinh` — feature-gated dispatcher; forwards to [`Self::sinh_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                self.sinh_strict()
            }
            /// `cosh` — feature-gated dispatcher; forwards to [`Self::cosh_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                self.cosh_strict()
            }
            /// `tanh` — feature-gated dispatcher; forwards to [`Self::tanh_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                self.tanh_strict()
            }
            /// `asinh` — feature-gated dispatcher; forwards to [`Self::asinh_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                self.asinh_strict()
            }
            /// `acosh` — feature-gated dispatcher; forwards to [`Self::acosh_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                self.acosh_strict()
            }
            /// `atanh` — feature-gated dispatcher; forwards to [`Self::atanh_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                self.atanh_strict()
            }
            /// `to_degrees` — feature-gated dispatcher; forwards to [`Self::to_degrees_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                self.to_degrees_strict()
            }
            /// `to_radians` — feature-gated dispatcher; forwards to [`Self::to_radians_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                self.to_radians_strict()
            }
            /// `log` — feature-gated dispatcher; forwards to [`Self::log_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                self.log_strict(base)
            }
            /// `atan2` — feature-gated dispatcher; forwards to [`Self::atan2_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                self.atan2_strict(other)
            }
            /// `powf` — feature-gated dispatcher; forwards to [`Self::powf_strict`] when the `strict` feature is on.
            #[cfg(not(all(feature = "fast", not(feature = "strict"))))]
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                self.powf_strict(exp)
            }
        }
    };
}

pub(crate) use decl_strict_transcendentals_via_d38;
