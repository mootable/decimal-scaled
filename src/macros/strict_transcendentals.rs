// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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
//! - `<method>_strict` — integer-only; `no_std`-compatible.
//! - `<method>` — a dispatcher forwarding to `<method>_strict`.

/// Emits the strict-mode transcendental surface for `$Type<SCALE>` by
/// delegating to the D38 `*_strict` implementations.
macro_rules! decl_strict_transcendentals_via_d38 {
    ($Type:ident) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// `ln` — delegates to the policy-registered ln
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                Self::from_bits($crate::policy::ln::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `log1p` — `ln(1 + self)`, delegating to the
            /// policy-registered log1p kernel for this `(width, SCALE)`
            /// cell. **0.5 ULP correctly-rounded** at storage scale.
            /// Provided for API parity; at this crate's fixed-point
            /// scales it is equivalent to `(1 + self).ln()`.
            /// Panics if `self <= -1` or the result doesn't fit `Self`'s
            /// range.
            #[inline]
            #[must_use]
            pub fn log1p(self) -> Self {
                Self::from_bits($crate::policy::log1p::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `expm1` — `e^self - 1`, delegating to the
            /// policy-registered expm1 kernel for this `(width, SCALE)`
            /// cell. **0.5 ULP correctly-rounded** at storage scale.
            /// Reaches slightly further than `exp`: the `- 1` is
            /// applied at the working scale, ahead of the range check, so
            /// the domain is `self <= ln(1 + MAX)` rather than `ln(MAX)`
            /// — a band `ln(1 + 1/MAX)` wide (a few hundredths). Total
            /// below (it tends to `-1`). Panics if the result doesn't fit
            /// `Self`'s range.
            #[inline]
            #[must_use]
            pub fn expm1(self) -> Self {
                Self::from_bits($crate::policy::expm1::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `log2` — delegates to [`crate::types::widths::D38::log2`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::log2"),
                        SCALE,
                    )
                })
            }
            /// `log10` — delegates to [`crate::types::widths::D38::log10`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::log10"),
                        SCALE,
                    )
                })
            }
            /// `exp` — delegates to the policy-registered exp
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                Self::from_bits($crate::policy::exp::dispatch::<_, SCALE>(
                    self.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `exp2` — delegates to [`crate::types::widths::D38::exp2`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::exp2"),
                        SCALE,
                    )
                })
            }
            /// `sqrt` — delegates to the policy-registered sqrt
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            ///
            /// For the narrow tier this widens to the `Int<2>` storage
            /// and resolves to `algos::sqrt::sqrt_mg_divide`; see
            /// `policy::sqrt` for the `(N, SCALE)` matcher.
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                Self($crate::policy::sqrt::dispatch::<_, SCALE>(
                    self.0,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `cbrt` — delegates to the policy-registered cbrt
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                Self($crate::policy::cbrt::dispatch::<_, SCALE>(
                    self.0,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }
            /// `sin` — delegates to the policy-registered sin
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                Self::from_bits($crate::policy::trig::sin_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `cos` — delegates to the policy-registered cos
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                Self::from_bits($crate::policy::trig::cos_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `tan` — delegates to the policy-registered tan
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                Self::from_bits($crate::policy::trig::tan_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `asin` — delegates to the policy-registered asin
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                Self::from_bits($crate::policy::trig::asin_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `acos` — delegates to the policy-registered acos
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                Self::from_bits($crate::policy::trig::acos_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `atan` — delegates to the policy-registered atan
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                Self::from_bits($crate::policy::trig::atan_dispatch::<_, SCALE>(self.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `sinh` — delegates to [`crate::types::widths::D38::sinh`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::sinh"),
                        SCALE,
                    )
                })
            }
            /// `cosh` — delegates to [`crate::types::widths::D38::cosh`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::cosh"),
                        SCALE,
                    )
                })
            }
            /// `tanh` — delegates to [`crate::types::widths::D38::tanh`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::tanh"),
                        SCALE,
                    )
                })
            }
            /// `asinh` — delegates to [`crate::types::widths::D38::asinh`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::asinh"),
                        SCALE,
                    )
                })
            }
            /// `acosh` — delegates to [`crate::types::widths::D38::acosh`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::acosh"),
                        SCALE,
                    )
                })
            }
            /// `atanh` — delegates to [`crate::types::widths::D38::atanh`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::atanh"),
                        SCALE,
                    )
                })
            }
            /// `to_degrees` — delegates to [`crate::types::widths::D38::to_degrees`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::to_degrees"),
                        SCALE,
                    )
                })
            }
            /// `to_radians` — delegates to [`crate::types::widths::D38::to_radians`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians()).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::to_radians"),
                        SCALE,
                    )
                })
            }
            /// `log` — delegates to [`crate::types::widths::D38::log`] via widen → strict → narrow. **0.5 ULP correctly-rounded** at storage scale. Panics if the result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_base: $crate::D<$crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log(wide_base)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log"),
                            SCALE,
                        )
                    },
                )
            }
            /// `atan2` — delegates to the policy-registered atan2
            /// kernel for this `(width, SCALE)` cell.
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                Self::from_bits($crate::policy::trig::atan2_dispatch::<_, SCALE>(self.to_bits(), other.to_bits(), $crate::support::rounding::DEFAULT_ROUNDING_MODE))
            }
            /// `powf` — delegates to the policy-registered powf
            /// kernel for this `(width, SCALE)` cell. **0.5 ULP
            /// correctly-rounded** at storage scale. Panics if the
            /// result doesn't fit `Self`'s range.
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                Self::from_bits($crate::policy::pow::dispatch::<_, SCALE>(
                    self.to_bits(),
                    exp.to_bits(),
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                ))
            }

            // ── Mode-aware (`_strict_with`) siblings ──────────────
            //
            // Each method widens to `D38<SCALE>`, calls the matching
            // D38 inherent, and narrows the result back. This is the
            // same widen-strict-narrow shape used by the `_strict`
            // methods above; the extra `mode` argument is forwarded
            // verbatim to the D38 call.
            //
            // Without these the
            // `decl_decimal_transcendental_impl!` macro would emit
            // trait method bodies that resolve back to themselves,
            // causing infinite recursion at runtime on D18.

            // ─ Logarithms ────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn ln_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.ln_with(mode)).unwrap_or_else(|_| {
                    $crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($Type), "::ln_with"),
                        SCALE,
                    )
                })
            }
            // `log1p` routes straight to its own policy at this width
            // (matching `ln` above), so all four variants keep
            // the storage-scale domain guard `self > -1` and never take
            // a detour through the D38 shell.
            #[inline]
            #[must_use]
            pub fn log1p_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::log1p::dispatch::<_, SCALE>(
                    self.to_bits(),
                    mode,
                ))
            }
            // `expm1` likewise routes straight to its own policy at this
            // width, so all four variants keep the working-scale `- 1`
            // (and with it the extended domain) instead of taking a
            // detour through the D38 shell.
            #[inline]
            #[must_use]
            pub fn expm1_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self::from_bits($crate::policy::expm1::dispatch::<_, SCALE>(
                    self.to_bits(),
                    mode,
                ))
            }
            #[inline]
            #[must_use]
            pub fn log_with(
                self,
                base: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_base: $crate::D<$crate::int::types::Int<2>, SCALE> = base.into();
                ::core::convert::TryInto::try_into(wide_self.log_with(wide_base, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn log2_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log2_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log2_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn log10_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.log10_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::log10_with"),
                            SCALE,
                        )
                    },
                )
            }
            // ─ Exponentials ──────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn exp_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn exp2_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.exp2_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::exp2_with"),
                            SCALE,
                        )
                    },
                )
            }
            // ─ Power ─────────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn powf_with(
                self,
                exp: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_exp: $crate::D<$crate::int::types::Int<2>, SCALE> = exp.into();
                ::core::convert::TryInto::try_into(wide_self.powf_with(wide_exp, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::powf_with"),
                            SCALE,
                        )
                    })
            }
            // ─ Roots ─────────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn sqrt_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sqrt_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sqrt_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cbrt_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cbrt_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cbrt_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn hypot(self, other: Self) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.hypot(wide_other))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::hypot"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn hypot_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.hypot_with(wide_other, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::hypot_with"),
                            SCALE,
                        )
                    })
            }

            // ─ Trig (forward) ────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn sin_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sin_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sin_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cos_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cos_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cos_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn tan_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tan_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tan_with"),
                            SCALE,
                        )
                    },
                )
            }
            // ─ Trig (inverse) ────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn atan_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atan_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atan_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn asin_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asin_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asin_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn acos_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acos_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acos_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn atan2_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide_self: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                let wide_other: $crate::D<$crate::int::types::Int<2>, SCALE> = other.into();
                ::core::convert::TryInto::try_into(wide_self.atan2_with(wide_other, mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atan2_with"),
                            SCALE,
                        )
                    })
            }
            // ─ Hyperbolic ────────────────────────────────────────
            #[inline]
            #[must_use]
            pub fn sinh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::sinh_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn cosh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::cosh_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn tanh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::tanh_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn asinh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::asinh_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn acosh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::acosh_with"),
                            SCALE,
                        )
                    },
                )
            }
            #[inline]
            #[must_use]
            pub fn atanh_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_with(mode)).unwrap_or_else(
                    |_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::atanh_with"),
                            SCALE,
                        )
                    },
                )
            }
            // ─ Angle conversion ──────────────────────────────────
            #[inline]
            #[must_use]
            pub fn to_degrees_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_with(mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::to_degrees_with"),
                            SCALE,
                        )
                    })
            }
            #[inline]
            #[must_use]
            pub fn to_radians_with(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                let wide: $crate::D<$crate::int::types::Int<2>, SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_with(mode))
                    .unwrap_or_else(|_| {
                        $crate::support::diagnostics::overflow_panic_with_scale(
                            concat!(stringify!($Type), "::to_radians_with"),
                            SCALE,
                        )
                    })
            }
        }
    };
}

pub(crate) use decl_strict_transcendentals_via_d38;
