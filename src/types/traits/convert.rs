//! The [`DecimalConvert`] trait — round-trip + integer conversion +
//! float bridge shared by every decimal width.
//!
//! Split out of the original [`crate::Decimal`] trait alongside
//! [`crate::DecimalArithmetic`], [`crate::DecimalTranscendental`],
//! and [`crate::DecimalConstants`]; `Decimal` is now a marker
//! supertrait that requires all four. Callers who only need
//! conversions (not the operator surface or transcendentals) can
//! target this narrower bound; supertraits [`DecimalArithmetic`] so
//! `Self::Storage` and `Self::ZERO` / `Self::MAX` / `Self::MIN`
//! (needed for saturation paths) are in scope.
//!
//! See [`crate::types::traits::decimal`] for the full scope rationale.

use crate::support::rounding::RoundingMode;
use crate::types::traits::arithmetic::DecimalArithmetic;

/// Round-trip + conversion surface shared by every decimal width.
///
/// Supertraits [`DecimalArithmetic`] so the storage type and
/// boundary constants are accessible for saturation logic.
pub trait DecimalConvert: DecimalArithmetic {
    // ── Round-trip ───────────────────────────────────────────────────

    /// Constructs from a raw storage value.
    fn from_bits(raw: Self::Storage) -> Self;

    /// Returns the raw storage value.
    fn to_bits(self) -> Self::Storage;

    /// Returns the decimal scale of this value.
    fn scale(self) -> u32;

    // ── Integer conversion ───────────────────────────────────────────

    /// Construct from an `i32`, scaling by `10^SCALE`.
    fn from_i32(value: i32) -> Self;

    /// Convert to `i64` using the crate-default rounding mode.
    fn to_int(self) -> i64;

    /// Convert to `i64` using the supplied rounding mode.
    fn to_int_with(self, mode: RoundingMode) -> i64;

    // ── Float bridge (lossy) ─────────────────────────────────────────

    /// Construct from `f64` using the crate-default rounding mode.
    #[cfg(feature = "std")]
    fn from_f64(value: f64) -> Self;

    /// Construct from `f64` using the supplied rounding mode.
    #[cfg(feature = "std")]
    fn from_f64_with(value: f64, mode: RoundingMode) -> Self;

    /// Convert to `f64`. Lossy when the storage magnitude exceeds
    /// `f64`'s ~15-digit exact range.
    #[cfg(feature = "std")]
    fn to_f64(self) -> f64;

    /// Convert to `f32`. Lossy.
    #[cfg(feature = "std")]
    fn to_f32(self) -> f32;
}
