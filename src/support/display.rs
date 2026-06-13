// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! [`core::fmt`] formatters and [`core::str::FromStr`] for the decimal
//! widths. `Display` / `Debug` and `FromStr` are emitted per width via
//! [`crate::macros::display::decl_decimal_display!`] and
//! [`crate::macros::from_str::decl_decimal_from_str!`]; the
//! scientific-notation formatters ([`fmt::LowerExp`] / [`fmt::UpperExp`])
//! and the shared [`parse_components`] front-end live here as single
//! generic implementations covering every width.
//!
//! # Parser factoring
//!
//! [`parse_components`] is the shared string-parsing front-end (sign /
//! dot / digit-character validation, plus the overlong-fractional and
//! leading-zero checks). The arithmetic accumulator that turns the
//! integer / fractional digit slices into a storage value is emitted
//! per-storage by [`crate::macros::from_str::decl_decimal_from_str!`]'s
//! `wide` arm: the base-10 accumulation happens directly in the storage
//! type so the `10^SCALE` multiplier never overflows even at SCALE = 1230.
//!
//! # Display format
//!
//! [`fmt::Display`] formats as a base-10 decimal literal: integer digits,
//! a `.`, then exactly `SCALE` fractional digits (trailing zeros are always
//! emitted). At `SCALE = 12`, `1.5` displays as `1.500000000000`. The output
//! is bit-faithful: parsing it back through [`core::str::FromStr`] returns
//! the identical storage value.
//!
//! # Debug format
//!
//! [`fmt::Debug`] wraps the [`fmt::Display`] output with a scale annotation:
//! `D38<SCALE>(...)`. This replaces the default derived format, which would
//! show only the raw `i128` storage.
//!
//! # Scientific notation
//!
//! [`fmt::LowerExp`] and [`fmt::UpperExp`] emit scientific notation (`1.5e0`
//! / `1.5E0`). Trailing zeros in the mantissa are stripped. Unlike
//! `Display` / `Debug` (emitted per width by the macro so `Debug` can name
//! its concrete type), these are a SINGLE generic `impl` over
//! `D<Int<N>, SCALE>` covering every width: scientific notation reshapes the
//! magnitude's decimal digit string and places the point per the const
//! `SCALE`, independent of limb count, so one generic kernel serves all
//! tiers.
//!
//! # Storage-level radix formats
//!
//! [`fmt::LowerHex`], [`fmt::UpperHex`], [`fmt::Octal`], and [`fmt::Binary`]
//! format the **raw `i128` storage** (= `value * 10^SCALE`), not the decimal
//! value. For example, `D38s12::ONE` (storage `10^12`) prints in lower-hex
//! as `e8d4a51000`.
//!
//! # `FromStr`
//!
//! Parses canonical decimal literals. Accepted forms:
//! - Integer-only: `42` parses as `42 * 10^SCALE`.
//! - Decimal with up to `SCALE` fractional digits: `1.5`, `1.500`.
//! - Optional sign prefix: `-` or `+`.
//! - Bare zero: `0` or `0.0`.
//!
//! Rejected forms (with the corresponding [`ParseError`] variant):
//! - Empty string: [`ParseError::Empty`].
//! - Sign with no digits: [`ParseError::SignOnly`].
//! - Redundant leading zeros (`01`, `00`): [`ParseError::LeadingZero`].
//! - More than `SCALE` fractional digits: [`ParseError::OverlongFractional`].
//! - Scientific notation (`1e3`): [`ParseError::ScientificNotation`].
//! - Missing digits on either side of the point (`.5`, `5.`):
//! [`ParseError::MissingDigits`].
//! - Non-digit, non-sign, non-dot characters: [`ParseError::InvalidChar`].
//! - Magnitudes outside `[D38::MIN, D38::MAX]`: [`ParseError::OutOfRange`].

use core::fmt;

use crate::int::types::Int;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::support::int_fmt::fmt_into;
use crate::types::widths::ParseError;

#[cfg(feature = "alloc")]
extern crate alloc;

// ──────────────────────────────────────────────────────────────────────
// Display and Debug are emitted by the `decl_decimal_display!` macro
// invoked from `types/widths.rs`; the macro itself lives in
// `src/macros/display.rs` and handles all widths uniformly.
// ──────────────────────────────────────────────────────────────────────

// ──────────────────────────────────────────────────────────────────────
// LowerExp / UpperExp -- scientific notation
//
// A SINGLE generic blanket over `D<Int<N>, SCALE>` (the same shape as the
// `PartialEq` / `Ord` blankets in `types/unified.rs`), so every width gets
// `{:e}` / `{:E}` from one source. Scientific notation reshapes the
// magnitude's decimal digit string and places the point per the const
// `SCALE`; it is independent of limb count, so no per-tier impl is needed.
// `Debug` stays macro-emitted because it must name its concrete type; these
// formatters name nothing, so the generic impl is sound and collision-free.
// ──────────────────────────────────────────────────────────────────────

impl<const N: usize, const SCALE: u32> fmt::LowerExp for crate::D<Int<N>, SCALE>
where
    Limbs<N>: ComputeLimbs,
{
    /// Formats the value in scientific notation with a lowercase `e`.
    ///
    /// Trailing zeros in the mantissa are stripped, so `1.500000000000`
    /// formats as `1.5e0`. Zero formats as `0e0`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D38s12;
    ///
    /// let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
    /// assert_eq!(format!("{v:e}"), "1.5e0");
    ///
    /// let sub = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_i128).unwrap());
    /// assert_eq!(format!("{sub:e}"), "1.5e-3");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_exp::<N>(self.0, SCALE, false, f)
    }
}

impl<const N: usize, const SCALE: u32> fmt::UpperExp for crate::D<Int<N>, SCALE>
where
    Limbs<N>: ComputeLimbs,
{
    /// Formats the value in scientific notation with an uppercase `E`.
    ///
    /// Identical to [`fmt::LowerExp`] except the exponent separator is `E`.
    ///
    /// # Precision
    ///
    /// Strict: all arithmetic is integer-only; result is bit-exact.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::D38s12;
    ///
    /// let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
    /// assert_eq!(format!("{v:E}"), "1.5E0");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_exp::<N>(self.0, SCALE, true, f)
    }
}

/// Shared scientific-notation kernel for `LowerExp` and `UpperExp`, generic
/// over the storage width `N`.
///
/// Extracts the magnitude's decimal digit string via the same per-width
/// [`fmt_into`] path that `Int<N>` / `Uint<N>`'s `Display` use — writing into
/// the per-`N` `digit_formatting_limbs_u8` stack buffer, so no heap is
/// touched — then places the decimal point after the leading digit and emits
/// the exponent `(digits − 1) − SCALE`. Width-agnostic: the reshaping only
/// sees the digit string and the scale.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
fn format_exp<const N: usize>(
    value: Int<N>,
    scale: u32,
    upper: bool,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    Limbs<N>: ComputeLimbs,
{
    let exp_char = if upper { 'E' } else { 'e' };
    if value.is_zero() {
        return write!(f, "0{exp_char}0");
    }
    let negative = value.is_negative();

    // Decimal digits of the magnitude, MSB-first with no leading zeros,
    // written into the per-`N` formatting buffer (the identical extraction
    // path the integer `Display` impls take — pure stack, no heap).
    let mag = *value.unsigned_abs().as_limbs();
    let mut buf = Limbs::<N>::digit_formatting_limbs_u8();
    let digits = fmt_into::<N>(&mag, 10, true, buf.as_mut()).as_bytes();
    let len = digits.len();

    // The decimal exponent for the leading digit is `(len - 1) - scale`.
    let exp: i32 = (len as i32 - 1) - scale as i32;

    // Strip trailing zeros from the mantissa digit string.
    let mut frac_end = len;
    while frac_end > 1 && digits[frac_end - 1] == b'0' {
        frac_end -= 1;
    }
    let mantissa_int = digits[0] as char;
    let mantissa_frac = &digits[1..frac_end];

    if negative {
        f.write_str("-")?;
    }
    if mantissa_frac.is_empty() {
        // Single-digit mantissa: emit without a decimal point.
        write!(f, "{mantissa_int}{exp_char}{exp}")
    } else {
        // mantissa_frac contains only ASCII digit bytes; from_utf8 cannot fail.
        let frac_str = core::str::from_utf8(mantissa_frac).map_err(|_| fmt::Error)?;
        write!(f, "{mantissa_int}.{frac_str}{exp_char}{exp}")
    }
}

// ──────────────────────────────────────────────────────────────────────
// `ParseError`'s `Display` and `Error` impls live in `src/error.rs`.

/// Outcome of the string-parsing front-end: sign and the integer / fractional
/// digit slices. Both byte slices contain only ASCII digits.
///
/// Centralises the sign / dot / digit-character state machine so the
/// per-storage accumulators (`i128` for the narrow tier; the wide signed
/// integers emitted via the from-str macro for the wide tier) only need
/// to do the base-10 arithmetic.
pub(crate) struct ParseComponents<'a> {
    pub negative: bool,
    pub int_str: &'a [u8],
    pub frac_str: &'a [u8],
}

/// String-parsing front-end shared by every width.
///
/// Validates and splits the input into sign / integer-digits / fractional-
/// digits. The `SCALE` parameter is needed only to reject overlong fractional
/// parts — no arithmetic happens here, so wide-tier callers can drive their
/// own storage-typed accumulator without overflow risk.
///
/// # Precision
///
/// Strict: integer-only string slicing; no arithmetic.
pub(crate) fn parse_components<const SCALE: u32>(
    s: &str,
) -> Result<ParseComponents<'_>, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let bytes = s.as_bytes();
    let mut idx = 0usize;

    // Consume an optional leading sign byte.
    let negative = match bytes[0] {
        b'-' => {
            idx += 1;
            true
        }
        b'+' => {
            idx += 1;
            false
        }
        _ => false,
    };
    if idx == bytes.len() {
        // Sign byte with nothing following it.
        return Err(ParseError::SignOnly);
    }

    // Single forward pass: locate the decimal point; reject scientific
    // notation and invalid characters immediately.
    let mut dot_pos: Option<usize> = None;
    {
        let mut i = idx;
        while i < bytes.len() {
            let c = bytes[i];
            match c {
                b'0'..=b'9' => {}
                b'.' => {
                    if dot_pos.is_some() {
                        // A second dot is an invalid character, not a
                        // missing-digit case.
                        return Err(ParseError::InvalidChar);
                    }
                    dot_pos = Some(i);
                }
                b'e' | b'E' => {
                    return Err(ParseError::ScientificNotation);
                }
                _ => return Err(ParseError::InvalidChar),
            }
            i += 1;
        }
    }

    let (int_str, frac_str) = match dot_pos {
        Some(p) => (&bytes[idx..p], &bytes[p + 1..]),
        None => (&bytes[idx..], &[][..]),
    };

    if dot_pos.is_some() {
        // Both sides of the dot must have at least one digit.
        if int_str.is_empty() || frac_str.is_empty() {
            return Err(ParseError::MissingDigits);
        }
    } else if int_str.is_empty() {
        return Err(ParseError::SignOnly);
    }

    // Allow `0` and `0.x` but reject `00`, `01`, `01.5`.
    if int_str.len() > 1 && int_str[0] == b'0' {
        return Err(ParseError::LeadingZero);
    }

    // More than SCALE fractional digits would lose precision on round-trip.
    if frac_str.len() > SCALE as usize {
        return Err(ParseError::OverlongFractional);
    }

    Ok(ParseComponents {
        negative,
        int_str,
        frac_str,
    })
}
