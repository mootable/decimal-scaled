//! Macro-generated f64-bridge (lossy) transcendentals for every width
//! except D38.
//!
//! D38 has the fast transcendentals hand-written in
//! `log_exp.rs` / `trig.rs` / `powers.rs`; D9 / D18 and the wide tiers
//! D76 / D153 / D307 all share the same delegation shape — convert
//! to `f64`, call the platform intrinsic, convert back — so it lives
//! in one macro reused per width.
//!
//! Each emitted method is gated `std` and "not strict (or fast
//! overrides strict)": exactly the configuration in which the plain
//! method dispatches to the fast bridge rather than the integer-only
//! `*_strict` path. The `*_strict` methods themselves are emitted by
//! `strict_transcendentals.rs` (for D9 / D18, via the D38 path) and
//! by `wide_transcendental.rs` (for the wide tiers).
//!
//! # Precision
//!
//! Lossy: results round-trip through `f64`'s ~15-digit mantissa, so
//! for high `SCALE` the precision degrades. Use the corresponding
//! `*_strict` form for correctly-rounded results.

/// Emits the f64-bridge transcendental surface for `$Type<SCALE>`.
macro_rules! decl_fast_transcendentals_via_f64 {
    ($Type:ident) => {
        #[cfg(all(feature = "std", any(not(feature = "strict"), feature = "fast")))]
        impl<const SCALE: u32> $Type<SCALE> {
            // ── Logarithms ───────────────────────────────────────────
            /// Natural logarithm via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                Self::from_f64(self.to_f64().ln())
            }
            /// Logarithm in the given base via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn log(self, base: Self) -> Self {
                Self::from_f64(self.to_f64().log(base.to_f64()))
            }
            /// Base-2 logarithm via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                Self::from_f64(self.to_f64().log2())
            }
            /// Base-10 logarithm via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn log10(self) -> Self {
                Self::from_f64(self.to_f64().log10())
            }

            // ── Exponentials ─────────────────────────────────────────
            /// `e^self` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                Self::from_f64(self.to_f64().exp())
            }
            /// `2^self` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                Self::from_f64(self.to_f64().exp2())
            }

            // ── Powers / roots ───────────────────────────────────────
            /// Square root via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                Self::from_f64(self.to_f64().sqrt())
            }
            /// Cube root via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                Self::from_f64(self.to_f64().cbrt())
            }
            /// `self ^ exp` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn powf(self, exp: Self) -> Self {
                Self::from_f64(self.to_f64().powf(exp.to_f64()))
            }
            /// `sqrt(self^2 + other^2)` via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn hypot(self, other: Self) -> Self {
                Self::from_f64(self.to_f64().hypot(other.to_f64()))
            }

            // ── Forward trig ─────────────────────────────────────────
            /// Sine (radians) via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                Self::from_f64(self.to_f64().sin())
            }
            /// Cosine (radians) via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                Self::from_f64(self.to_f64().cos())
            }
            /// Tangent (radians) via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                Self::from_f64(self.to_f64().tan())
            }

            // ── Inverse trig ─────────────────────────────────────────
            /// Arcsine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                Self::from_f64(self.to_f64().asin())
            }
            /// Arccosine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                Self::from_f64(self.to_f64().acos())
            }
            /// Arctangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                Self::from_f64(self.to_f64().atan())
            }
            /// Four-quadrant arctangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn atan2(self, other: Self) -> Self {
                Self::from_f64(self.to_f64().atan2(other.to_f64()))
            }

            // ── Hyperbolic ───────────────────────────────────────────
            /// Hyperbolic sine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn sinh(self) -> Self {
                Self::from_f64(self.to_f64().sinh())
            }
            /// Hyperbolic cosine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn cosh(self) -> Self {
                Self::from_f64(self.to_f64().cosh())
            }
            /// Hyperbolic tangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn tanh(self) -> Self {
                Self::from_f64(self.to_f64().tanh())
            }
            /// Inverse hyperbolic sine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn asinh(self) -> Self {
                Self::from_f64(self.to_f64().asinh())
            }
            /// Inverse hyperbolic cosine via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn acosh(self) -> Self {
                Self::from_f64(self.to_f64().acosh())
            }
            /// Inverse hyperbolic tangent via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn atanh(self) -> Self {
                Self::from_f64(self.to_f64().atanh())
            }

            // ── Angle conversions ────────────────────────────────────
            /// Radians → degrees via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn to_degrees(self) -> Self {
                Self::from_f64(self.to_f64().to_degrees())
            }
            /// Degrees → radians via the f64 bridge.
            #[inline]
            #[must_use]
            pub fn to_radians(self) -> Self {
                Self::from_f64(self.to_f64().to_radians())
            }
        }
    };
}

pub(crate) use decl_fast_transcendentals_via_f64;
