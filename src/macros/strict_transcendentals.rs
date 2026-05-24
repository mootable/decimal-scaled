//! Macro-generated strict-mode transcendentals for the narrow decimal
//! widths (D18), by delegation to the D38 strict path.
//!
//! For each method the input is widened to `D38<SCALE>`, the D38
//! `*_strict` implementation is called, and the result is narrowed
//! back. This gives D18 the full integer-only transcendental
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
//! forwarding to `<method>_strict`. (D18 have no f64-bridge
//! transcendentals of their own, so there is no non-strict `<method>`
//! for these widths.)

/// Emits the strict-mode transcendental surface for `$Type<SCALE>` by
/// delegating to the D38 `*_strict` implementations.
macro_rules! decl_strict_transcendentals_via_d38 {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// `ln_strict` — delegates to the policy-registered ln
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn ln_strict(self) -> Self {
                Self::from_bits($crate::policy::ln::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `log2_strict` — delegates to [`crate::types::widths::D38::log2_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log2_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::log2_strict"),
                        SCALE,
                    )
                })
            }
            /// `log10_strict` — delegates to [`crate::types::widths::D38::log10_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log10_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::log10_strict"),
                        SCALE,
                    )
                })
            }
            /// `exp_strict` — delegates to the policy-registered exp
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn exp_strict(self) -> Self {
                Self::from_bits($crate::policy::exp::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `exp2_strict` — delegates to [`crate::types::widths::D38::exp2_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn exp2_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::exp2_strict"),
                        SCALE,
                    )
                })
            }
            /// `sqrt_strict` — delegates to the policy-registered sqrt
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            ///
            /// For the narrow tier this widens to the `Int<2>` storage
            /// and resolves to `algos::sqrt::sqrt_mg_divide`; see
            /// `policy::sqrt` for the `(N, SCALE)` matcher.
            #[inline]
            #[must_use]
            pub fn sqrt_strict(self) -> Self {
                Self($crate::policy::sqrt::dispatch::<_, SCALE>(
                    self.0,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `cbrt_strict` — delegates to the policy-registered cbrt
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cbrt_strict(self) -> Self {
                Self($crate::policy::cbrt::dispatch::<_, SCALE>(
                    self.0,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `sin_strict` — delegates to the policy-registered sin
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn sin_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::sin_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `cos_strict` — delegates to the policy-registered cos
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn cos_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::cos_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `tan_strict` — delegates to the policy-registered tan
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn tan_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::tan_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `asin_strict` — delegates to the policy-registered asin
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn asin_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::asin_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `acos_strict` — delegates to the policy-registered acos
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn acos_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::acos_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `atan_strict` — delegates to the policy-registered atan
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn atan_strict(self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::atan_impl(
                    self,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `sinh_strict` — delegates to [`crate::types::widths::D38::sinh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn sinh_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::sinh_strict"),
                        SCALE,
                    )
                })
            }
            /// `cosh_strict` — delegates to [`crate::types::widths::D38::cosh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cosh_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::cosh_strict"),
                        SCALE,
                    )
                })
            }
            /// `tanh_strict` — delegates to [`crate::types::widths::D38::tanh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn tanh_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::tanh_strict"),
                        SCALE,
                    )
                })
            }
            /// `asinh_strict` — delegates to [`crate::types::widths::D38::asinh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn asinh_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::asinh_strict"),
                        SCALE,
                    )
                })
            }
            /// `acosh_strict` — delegates to [`crate::types::widths::D38::acosh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn acosh_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::acosh_strict"),
                        SCALE,
                    )
                })
            }
            /// `atanh_strict` — delegates to [`crate::types::widths::D38::atanh_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn atanh_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::atanh_strict"),
                        SCALE,
                    )
                })
            }
            /// `to_degrees_strict` — delegates to [`crate::types::widths::D38::to_degrees_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn to_degrees_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::to_degrees_strict"),
                        SCALE,
                    )
                })
            }
            /// `to_radians_strict` — delegates to [`crate::types::widths::D38::to_radians_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn to_radians_strict(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::to_radians_strict"),
                        SCALE,
                    )
                })
            }
            /// `log_strict` — delegates to [`crate::types::widths::D38::log_strict`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log_strict(self, base: Self) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_base: $crate::D<$crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_strict(wide_base)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log_strict"),
                            SCALE,
                        )
                    },
                )
            }
            /// `atan2_strict` — delegates to the policy-registered atan2
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn atan2_strict(self, other: Self) -> Self {
                <Self as $crate::policy::trig::TrigPolicy>::atan2_impl(
                    self,
                    other,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }
            /// `powf_strict` — delegates to the policy-registered powf
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn powf_strict(self, exp: Self) -> Self {
                Self::from_bits($crate::policy::pow::dispatch::<_, SCALE>(
                    self.to_bits(),
                    exp.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }

            // ── Mode-aware (`_strict_with`) and guard-aware
            // (`_approx`, `_approx_with`) siblings ─────────────────
            //
            // Each method widens to `D38<SCALE>`, calls the matching
            // D38 inherent, and narrows the result back. This is the
            // same widen-strict-narrow shape used by the `_strict`
            // methods above; the extra `mode` / `working_digits`
            // arguments are forwarded verbatim to the D38 call.
            //
            // Without these the
            // `decl_decimal_transcendental_impl!` macro would emit
            // trait method bodies that resolve back to themselves,
            // causing infinite recursion at runtime on D18.

            // ─ Logarithms ────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn ln_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln_strict_with(mode)).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::ln_strict_with"),
                        SCALE,
                    )
                })
            }
            #[inline]
            #[must_use]
            pub fn ln_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::ln_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn ln_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::ln_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn log_strict_with(
                self,
                base: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_base: $crate::D<$crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_strict_with(wide_base, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log_strict_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn log_approx(self, base: Self, working_digits: u32) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_base: $crate::D<$crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_approx(wide_base, working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn log_approx_with(
                self,
                base: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_base: $crate::D<$crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_approx_with(
                    wide_base,
                    working_digits,
                    mode,
                ))
                .unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::log_approx_with"),
                        SCALE,
                    )
                })
            }

            #[inline]
            #[must_use]
            pub fn log2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log2_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn log2_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log2_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn log2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log2_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn log10_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log10_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn log10_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_approx(working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log10_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn log10_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log10_approx_with"),
                            SCALE,
                        )
                    })
            }

            // ─ Exponentials ──────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn exp_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn exp_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn exp_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn exp2_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp2_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn exp2_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp2_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn exp2_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp2_approx_with"),
                            SCALE,
                        )
                    })
            }

            // ─ Power ─────────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn powf_strict_with(
                self,
                exp: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_exp: $crate::D<$crate::int::types::Int<2>, SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf_strict_with(wide_exp, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::powf_strict_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn powf_approx(self, exp: Self, working_digits: u32) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_exp: $crate::D<$crate::int::types::Int<2>, SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf_approx(wide_exp, working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::powf_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn powf_approx_with(
                self,
                exp: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_exp: $crate::D<$crate::int::types::Int<2>, SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf_approx_with(
                    wide_exp,
                    working_digits,
                    mode,
                ))
                .unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::powf_approx_with"),
                        SCALE,
                    )
                })
            }

            // ─ Roots ─────────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn sqrt_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sqrt_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sqrt_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cbrt_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cbrt_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cbrt_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn hypot_strict(self, other: Self) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.hypot_strict(wide_other))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::hypot_strict"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn hypot_strict_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.hypot_strict_with(wide_other, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::hypot_strict_with"),
                            SCALE,
                        )
                    })
            }

            // ─ Trig (forward) ────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn sin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sin_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn sin_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sin_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn sin_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sin_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn cos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cos_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cos_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cos_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cos_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn tan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tan_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn tan_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tan_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn tan_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tan_approx_with"),
                            SCALE,
                        )
                    })
            }

            // ─ Trig (inverse) ────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn atan_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atan_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn atan_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atan_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn atan_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atan_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn asin_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asin_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn asin_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asin_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn asin_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asin_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn acos_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acos_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn acos_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acos_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn acos_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acos_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn atan2_strict_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.atan2_strict_with(wide_other, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atan2_strict_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn atan2_approx(self, other: Self, working_digits: u32) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(
                    wide_self.atan2_approx(wide_other, working_digits),
                )
                .unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::atan2_approx"),
                        SCALE,
                    )
                })
            }
            #[inline]
            #[must_use]
            pub fn atan2_approx_with(
                self,
                other: Self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.atan2_approx_with(
                    wide_other,
                    working_digits,
                    mode,
                ))
                .unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::atan2_approx_with"),
                        SCALE,
                    )
                })
            }

            // ─ Hyperbolic ────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn sinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sinh_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn sinh_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sinh_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn sinh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sinh_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn cosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cosh_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cosh_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cosh_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cosh_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn tanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tanh_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn tanh_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_approx(working_digits)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tanh_approx"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn tanh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tanh_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn asinh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asinh_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn asinh_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_approx(working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asinh_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn asinh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asinh_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn acosh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acosh_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn acosh_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_approx(working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acosh_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn acosh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acosh_approx_with"),
                            SCALE,
                        )
                    })
            }

            #[inline]
            #[must_use]
            pub fn atanh_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atanh_strict_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn atanh_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_approx(working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atanh_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn atanh_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_approx_with(working_digits, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atanh_approx_with"),
                            SCALE,
                        )
                    })
            }

            // ─ Angle conversion ──────────────────────────────────
            #[inline]
            #[must_use]
            pub fn to_degrees_strict_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict_with(mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::to_degrees_strict_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn to_degrees_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_approx(working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::to_degrees_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn to_degrees_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(
                    wide.to_degrees_approx_with(working_digits, mode),
                )
                .unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::to_degrees_approx_with"),
                        SCALE,
                    )
                })
            }

            #[inline]
            #[must_use]
            pub fn to_radians_strict_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict_with(mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::to_radians_strict_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn to_radians_approx(self, working_digits: u32) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_approx(working_digits))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::to_radians_approx"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn to_radians_approx_with(
                self,
                working_digits: u32,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(
                    wide.to_radians_approx_with(working_digits, mode),
                )
                .unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::to_radians_approx_with"),
                        SCALE,
                    )
                })
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
