//! Error types used across the decimal-scaled crate.
//!
//! Two enums live here:
//!
//! - [`ConvertError`] — returned by fallible `TryFrom` impls between
//! primitive types (`i128`, `u128`, `f32`, `f64`) and any decimal
//! width. Distinguishes overflow from non-finite float input.
//! - [`ParseError`] — returned by `FromStr` when the input is not a
//! valid canonical decimal literal. One variant per failure mode so
//! callers can surface a precise diagnostic.
//!
//! Both types are width-neutral — the same enum is returned by D9,
//! D18, D38, and the future wider widths.

/// Error returned by the fallible [`TryFrom`] impls.
///
/// Covers the two distinct failure modes:
/// - [`ConvertError::Overflow`] — the input, after scaling by
/// `10^SCALE`, exceeds the destination's representable range.
/// - [`ConvertError::NotFinite`] — the float input is `NaN`,
/// `+inf`, or `-inf`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConvertError {
    /// Input magnitude is outside the destination type's range after scaling.
    Overflow,
    /// Input is `NaN`, `+inf`, or `-inf` (only reachable from the
    /// `TryFrom<f32>` / `TryFrom<f64>` impls).
    NotFinite,
}

impl core::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow => f.write_str("decimal conversion overflow"),
            Self::NotFinite => f.write_str("decimal conversion from non-finite float"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConvertError {}

/// Error returned by `FromStr` when the input is not a valid canonical
/// decimal literal.
///
/// Each variant identifies one specific failure mode so callers can
/// surface a precise diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseError {
    /// Input string is empty.
    Empty,
    /// Input is `-` or `+` with no following digits.
    SignOnly,
    /// Integer part has a redundant leading zero (e.g. `01.5`).
    LeadingZero,
    /// Fractional part has more digits than `SCALE`.
    OverlongFractional,
    /// Input uses scientific notation (`1e3`); not accepted.
    ScientificNotation,
    /// Input contains a character that is not a digit, sign, or dot.
    InvalidChar,
    /// Parsed value exceeds the destination type's range after scaling.
    OutOfRange,
    /// Decimal point with no digit on one side (e.g. `.5` or `5.`).
    MissingDigits,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            Self::Empty => "empty input",
            Self::SignOnly => "sign with no digits",
            Self::LeadingZero => "redundant leading zero in integer part",
            Self::OverlongFractional => "fractional part exceeds SCALE digits",
            Self::ScientificNotation => "scientific notation not accepted",
            Self::InvalidChar => "invalid character",
            Self::OutOfRange => "value out of representable range",
            Self::MissingDigits => "decimal point with no adjacent digits",
        };
        f.write_str(msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}
