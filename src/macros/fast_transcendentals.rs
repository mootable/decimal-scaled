//! Macro-generated f64-bridge (lossy) transcendentals for every width
//! except D38.
//!
//! D38 has the fast transcendentals hand-written in
//! `log_exp_fast.rs` / `trig_fast.rs` / `powers_fast.rs`; D18 and
//! the wide tiers D76 / D153 / D307 all share the same delegation
//! shape — convert to `f64`, call the platform intrinsic, convert back —
//! so it lives in one macro reused per width.
//!
//! Two surfaces are emitted per width, mirroring the strict layout:
//!
//! - `<method>_fast` — always present when `feature = "std"`; explicit
//! per-call opt-in to the f64 bridge regardless of the build's
//! default strict/fast mode.
//! - `<method>` — a dispatcher present only under
//! `#[cfg(all(feature = "std", feature = "fast", not(feature = "strict")))]`,
//! forwarding to `<method>_fast`. Plain `<method>` for D18 / wide
//! widths therefore tracks the same dispatch rule as the strict file's
//! plain `<method>` and the two are mutually exclusive.
//!
//! # Precision
//!
//! Lossy: results round-trip through `f64`'s ~15-digit mantissa, so
//! for high `SCALE` the precision degrades. Use the corresponding
//! `*_strict` form for correctly-rounded results.

/// Emits the f64-bridge transcendental surface for `$Type<SCALE>`.
macro_rules! decl_fast_transcendentals_via_f64 {
    ($Type:ident) => {
        #[cfg(feature = "std")]
        impl<const SCALE: u32> $Type<SCALE> {
            // ── Logarithms ───────────────────────────────────────────
            /// Natural logarithm via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn ln_fast(self) -> Self {
                Self::from_f64(self.to_f64().ln())
            }
            /// Logarithm in the given base via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn log_fast(self, base: Self) -> Self {
                Self::from_f64(self.to_f64().log(base.to_f64()))
            }
            /// Base-2 logarithm via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn log2_fast(self) -> Self {
                Self::from_f64(self.to_f64().log2())
            }
            /// Base-10 logarithm via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn log10_fast(self) -> Self {
                Self::from_f64(self.to_f64().log10())
            }

            // ── Exponentials ─────────────────────────────────────────
            /// `e^self` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn exp_fast(self) -> Self {
                Self::from_f64(self.to_f64().exp())
            }
            /// `2^self` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn exp2_fast(self) -> Self {
                Self::from_f64(self.to_f64().exp2())
            }

            // ── Powers / roots ───────────────────────────────────────
            /// Square root via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn sqrt_fast(self) -> Self {
                Self::from_f64(self.to_f64().sqrt())
            }
            /// Cube root via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn cbrt_fast(self) -> Self {
                Self::from_f64(self.to_f64().cbrt())
            }
            /// `self ^ exp` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn powf_fast(self, exp: Self) -> Self {
                Self::from_f64(self.to_f64().powf(exp.to_f64()))
            }
            /// `sqrt(self^2 + other^2)` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn hypot_fast(self, other: Self) -> Self {
                Self::from_f64(self.to_f64().hypot(other.to_f64()))
            }

            // ── Forward trig ─────────────────────────────────────────
            /// Sine (radians) via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn sin_fast(self) -> Self {
                Self::from_f64(self.to_f64().sin())
            }
            /// Cosine (radians) via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn cos_fast(self) -> Self {
                Self::from_f64(self.to_f64().cos())
            }
            /// Tangent (radians) via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn tan_fast(self) -> Self {
                Self::from_f64(self.to_f64().tan())
            }

            // ── Inverse trig ─────────────────────────────────────────
            /// Arcsine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn asin_fast(self) -> Self {
                Self::from_f64(self.to_f64().asin())
            }
            /// Arccosine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn acos_fast(self) -> Self {
                Self::from_f64(self.to_f64().acos())
            }
            /// Arctangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn atan_fast(self) -> Self {
                Self::from_f64(self.to_f64().atan())
            }
            /// Four-quadrant arctangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn atan2_fast(self, other: Self) -> Self {
                Self::from_f64(self.to_f64().atan2(other.to_f64()))
            }

            // ── Hyperbolic ───────────────────────────────────────────
            /// Hyperbolic sine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn sinh_fast(self) -> Self {
                Self::from_f64(self.to_f64().sinh())
            }
            /// Hyperbolic cosine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn cosh_fast(self) -> Self {
                Self::from_f64(self.to_f64().cosh())
            }
            /// Hyperbolic tangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn tanh_fast(self) -> Self {
                Self::from_f64(self.to_f64().tanh())
            }
            /// Inverse hyperbolic sine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn asinh_fast(self) -> Self {
                Self::from_f64(self.to_f64().asinh())
            }
            /// Inverse hyperbolic cosine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn acosh_fast(self) -> Self {
                Self::from_f64(self.to_f64().acosh())
            }
            /// Inverse hyperbolic tangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn atanh_fast(self) -> Self {
                Self::from_f64(self.to_f64().atanh())
            }

            // ── Angle conversions ────────────────────────────────────
            /// Radians → degrees via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn to_degrees_fast(self) -> Self {
                Self::from_f64(self.to_f64().to_degrees())
            }
            /// Degrees → radians via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn to_radians_fast(self) -> Self {
                Self::from_f64(self.to_f64().to_radians())
            }
        }

        #[cfg(all(feature = "std", feature = "fast", not(feature = "strict")))]
        impl<const SCALE: u32> $Type<SCALE> {
            // Dispatcher forms: plain `<method>` resolves to `*_fast` in
            // this feature mode. Mutually exclusive with the strict-mode
            // dispatchers emitted by `strict_transcendentals.rs` /
            // `wide_transcendental.rs`.
            /// Plain dispatcher: forwards to [`Self::ln_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                self.ln_fast()
            }
            /// Plain dispatcher: forwards to [`Self::log_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                self.log_fast(base)
            }
            /// Plain dispatcher: forwards to [`Self::log2_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                self.log2_fast()
            }
            /// Plain dispatcher: forwards to [`Self::log10_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                self.log10_fast()
            }
            /// Plain dispatcher: forwards to [`Self::exp_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                self.exp_fast()
            }
            /// Plain dispatcher: forwards to [`Self::exp2_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                self.exp2_fast()
            }
            /// Plain dispatcher: forwards to [`Self::sqrt_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                self.sqrt_fast()
            }
            /// Plain dispatcher: forwards to [`Self::cbrt_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                self.cbrt_fast()
            }
            /// Plain dispatcher: forwards to [`Self::powf_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                self.powf_fast(exp)
            }
            /// Plain dispatcher: forwards to [`Self::hypot_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn hypot(self, other: Self) -> Self {
                self.hypot_fast(other)
            }
            /// Plain dispatcher: forwards to [`Self::sin_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                self.sin_fast()
            }
            /// Plain dispatcher: forwards to [`Self::cos_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                self.cos_fast()
            }
            /// Plain dispatcher: forwards to [`Self::tan_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                self.tan_fast()
            }
            /// Plain dispatcher: forwards to [`Self::asin_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                self.asin_fast()
            }
            /// Plain dispatcher: forwards to [`Self::acos_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                self.acos_fast()
            }
            /// Plain dispatcher: forwards to [`Self::atan_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                self.atan_fast()
            }
            /// Plain dispatcher: forwards to [`Self::atan2_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                self.atan2_fast(other)
            }
            /// Plain dispatcher: forwards to [`Self::sinh_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                self.sinh_fast()
            }
            /// Plain dispatcher: forwards to [`Self::cosh_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                self.cosh_fast()
            }
            /// Plain dispatcher: forwards to [`Self::tanh_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                self.tanh_fast()
            }
            /// Plain dispatcher: forwards to [`Self::asinh_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                self.asinh_fast()
            }
            /// Plain dispatcher: forwards to [`Self::acosh_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                self.acosh_fast()
            }
            /// Plain dispatcher: forwards to [`Self::atanh_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                self.atanh_fast()
            }
            /// Plain dispatcher: forwards to [`Self::to_degrees_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                self.to_degrees_fast()
            }
            /// Plain dispatcher: forwards to [`Self::to_radians_fast`] in this feature mode.
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                self.to_radians_fast()
            }
        }
    };
}

pub(crate) use decl_fast_transcendentals_via_f64;
