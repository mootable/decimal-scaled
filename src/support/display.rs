//! [`core::fmt`] formatters and [`core::str::FromStr`] for [`D38`].
//! The same surface is emitted for every width via
//! [`crate::macros::display::decl_decimal_display!`] and
//! [`crate::macros::from_str::decl_decimal_from_str!`]; this file
//! contains the hand-written D38 implementation and serves as the
//! shape reference for the macro emissions.
//!
//! # Parser factoring
//!
//! [`parse_components`] is the shared string-parsing front-end (sign /
//! dot / digit-character validation, plus the overlong-fractional and
//! leading-zero checks). The arithmetic accumulator that turns the
//! integer / fractional digit slices into a storage value is
//! *per-storage*:
//!
//! - Narrow tier (D18 / D38) accumulates in `u128` inside
//!   [`parse_decimal_bits`] — fast and the `10^SCALE` multiplier always
//!   fits since SCALE ≤ 38.
//! - Wide tier (D76 … D1231) accumulates in the storage type itself
//!   via the per-width body emitted by
//!   [`crate::macros::from_str::decl_decimal_from_str!`]'s `wide` arm.
//!   The integer arithmetic happens at the storage width so the
//!   `10^SCALE` multiplier never overflows even at SCALE = 1230.
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
//! / `1.5E0`). Trailing zeros in the mantissa are stripped.
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

use crate::types::widths::{D38, ParseError};

#[cfg(feature = "alloc")]
extern crate alloc;

// ──────────────────────────────────────────────────────────────────────
// Display and Debug are emitted by the `decl_decimal_display!` macro
// invoked from `types/widths.rs`; the macro itself lives in
// `src/macros/display.rs` and handles all widths uniformly.
// ──────────────────────────────────────────────────────────────────────

// ──────────────────────────────────────────────────────────────────────
// LowerExp / UpperExp -- scientific notation
// ──────────────────────────────────────────────────────────────────────

impl<const SCALE: u32> fmt::LowerExp for D38<SCALE> {
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
        format_exp(self.0.as_i128(), SCALE, false, f)
    }
}

impl<const SCALE: u32> fmt::UpperExp for D38<SCALE> {
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
        format_exp(self.0.as_i128(), SCALE, true, f)
    }
}

/// Shared implementation for `LowerExp` and `UpperExp`.
///
/// Builds the decimal digit string in a fixed 40-byte stack buffer
/// (a `u128` has at most 39 digits) so no heap allocation is needed.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
fn format_exp(raw: i128, scale: u32, upper: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let exp_char = if upper { 'E' } else { 'e' };
    if raw == 0 {
        return write!(f, "0{exp_char}0");
    }
    let negative = raw < 0;
    let mag: u128 = raw.unsigned_abs();

    // Collect decimal digits of `mag` LSB-first into the buffer,
    // then reverse to get MSB-first order.
    let mut buf = [0u8; 40];
    let mut len = 0usize;
    let mut n = mag;
    while n > 0 {
        let digit = (n % 10) as u8;
        buf[len] = b'0' + digit;
        len += 1;
        n /= 10;
    }
    buf[..len].reverse();
    let digits = &buf[..len];

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
        f.write_fmt(format_args!("{mantissa_int}."))?;
        // mantissa_frac contains only ASCII digit bytes; from_utf8 cannot fail.
        let frac_str = core::str::from_utf8(mantissa_frac).map_err(|_| fmt::Error)?;
        write!(f, "{frac_str}{exp_char}{exp}")
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

/// Core decimal string parser for `D38`-class native-`i128` storage.
///
/// Drives [`parse_components`] and accumulates the storage value in `u128`
/// (which avoids the `i128::MIN` asymmetry), then applies the sign.
///
/// The wide tier (D76 … D1231) uses [`crate::macros::from_str`] to emit a
/// per-storage accumulator with the same shape; the front-end is shared but
/// the arithmetic happens at the storage width so `10^SCALE` cannot
/// overflow.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
pub(crate) fn parse_decimal_bits<const SCALE: u32>(s: &str) -> Result<i128, ParseError> {
    parse_decimal::<SCALE>(s).map(|d| d.to_bits().as_i128())
}

fn parse_decimal<const SCALE: u32>(s: &str) -> Result<D38<SCALE>, ParseError> {
    let ParseComponents {
        negative,
        int_str,
        frac_str,
    } = parse_components::<SCALE>(s)?;

    // Accumulate the storage value as u128 (avoids the i128::MIN asymmetry)
    // and apply the sign at the very end.
    let multiplier: u128 = 10u128.pow(SCALE);

    // Parse the integer part and scale it by 10^SCALE.
    let mut int_value: u128 = 0;
    for &b in int_str {
        let digit = u128::from(b - b'0');
        int_value = match int_value.checked_mul(10).and_then(|v| v.checked_add(digit)) {
            Some(v) => v,
            None => return Err(ParseError::OutOfRange),
        };
    }
    let int_scaled = match int_value.checked_mul(multiplier) {
        Some(v) => v,
        None => return Err(ParseError::OutOfRange),
    };

    // Parse the fractional part, then pad to exactly SCALE digits by
    // multiplying by 10^(SCALE - frac_len).
    let mut frac_value: u128 = 0;
    let frac_len = frac_str.len();
    for &b in frac_str {
        let digit = u128::from(b - b'0');
        frac_value = match frac_value
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
        {
            Some(v) => v,
            None => return Err(ParseError::OutOfRange),
        };
    }
    let pad = (SCALE as usize) - frac_len;
    if pad > 0 {
        let pad_factor: u128 = 10u128.pow(pad as u32);
        frac_value = match frac_value.checked_mul(pad_factor) {
            Some(v) => v,
            None => return Err(ParseError::OutOfRange),
        };
    }

    let combined = match int_scaled.checked_add(frac_value) {
        Some(v) => v,
        None => return Err(ParseError::OutOfRange),
    };

    // Convert to i128. The negative branch handles i128::MIN whose absolute
    // value (i128::MAX + 1) is not representable as a positive i128.
    let raw: i128 = if negative {
        let neg_min_abs: u128 = (i128::MAX as u128) + 1;
        if combined > neg_min_abs {
            return Err(ParseError::OutOfRange);
        }
        if combined == neg_min_abs {
            i128::MIN
        } else {
            -(combined as i128)
        }
    } else {
        if combined > i128::MAX as u128 {
            return Err(ParseError::OutOfRange);
        }
        combined as i128
    };

    Ok(D38::<SCALE>::from_bits(crate::int::types::Int::<2>::from_i128(raw)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::widths::{D38, D38s12};
    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;

    // ── Display ──

    /// ZERO renders as `0.000000000000` at SCALE = 12.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_zero_renders() {
        assert_eq!(D38s12::ZERO.to_string(), "0.000000000000");
    }

    /// ONE renders as `1.000000000000` at SCALE = 12.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_one_renders() {
        assert_eq!(D38s12::ONE.to_string(), "1.000000000000");
    }

    /// `1.5` renders with full SCALE fractional digits.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_one_point_five_renders() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(v.to_string(), "1.500000000000");
    }

    /// Negative values get a leading `-`.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_negative_renders() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1_500_000_000_000));
        assert_eq!(v.to_string(), "-1.500000000000");
    }

    /// `0.001` (sub-unit positive) keeps leading-zero fractional.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_subunit_keeps_leading_zeros() {
        // 0.001 = 1_000_000_000 at SCALE 12
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_000_000_000));
        assert_eq!(v.to_string(), "0.001000000000");
    }

    /// MAX renders without panicking. Spot-check the canonical form
    /// at SCALE 12: `170141183460469231731687303.715884105727`.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_max_does_not_panic() {
        let s = D38s12::MAX.to_string();
        assert_eq!(s, "170141183460469231731687303.715884105727");
    }

    /// MIN renders without panicking. The unsigned-abs path handles
    /// the i128::MIN special case (|MIN| = MAX + 1, so the trailing
    /// digit is 8 not 7).
    #[cfg(feature = "alloc")]
    #[test]
    fn display_min_does_not_panic() {
        let s = D38s12::MIN.to_string();
        assert_eq!(s, "-170141183460469231731687303.715884105728");
    }

    /// SCALE = 0 has no decimal point.
    #[cfg(feature = "alloc")]
    #[test]
    fn display_scale_zero_no_dot() {
        type D0 = D38<0>;
        assert_eq!(D0::ONE.to_string(), "1");
        assert_eq!(D0::ZERO.to_string(), "0");
        assert_eq!(D0::from_bits(crate::int::types::Int::<2>::from_i128(-42)).to_string(), "-42");
    }

    // ── Debug ──

    /// Debug delegates to Display + SCALE annotation.
    #[cfg(feature = "alloc")]
    #[test]
    fn debug_includes_scale_and_value() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        let debug_str = format!("{v:?}");
        assert_eq!(debug_str, "D38<12>(1.500000000000)");
    }

    /// Debug on ZERO at a non-12 scale.
    #[cfg(feature = "alloc")]
    #[test]
    fn debug_other_scale() {
        type D6 = D38<6>;
        let v = D6::ZERO;
        assert_eq!(format!("{v:?}"), "D38<6>(0.000000)");
    }

    // ── LowerExp / UpperExp ──

    /// `1.0` -> `1e0` (single digit mantissa).
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_exp_one() {
        let v = D38s12::ONE;
        assert_eq!(format!("{v:e}"), "1e0");
    }

    /// `1.5` -> `1.5e0`.
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_exp_one_point_five() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(format!("{v:e}"), "1.5e0");
    }

    /// `15.0` -> `1.5e1`.
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_exp_fifteen() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(15_000_000_000_000));
        assert_eq!(format!("{v:e}"), "1.5e1");
    }

    /// `0.0` -> `0e0`.
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_exp_zero() {
        assert_eq!(format!("{:e}", D38s12::ZERO), "0e0");
    }

    /// Sub-unit value -> negative exponent. `0.0015 = 1.5e-3`.
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_exp_subunit_negative_exponent() {
        // 0.0015 at SCALE 12 = 1_500_000_000
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000));
        assert_eq!(format!("{v:e}"), "1.5e-3");
    }

    /// Negative value preserves sign.
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_exp_negative() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1_500_000_000_000));
        assert_eq!(format!("{v:e}"), "-1.5e0");
    }

    /// UpperExp uses `E`.
    #[cfg(feature = "alloc")]
    #[test]
    fn upper_exp_uses_capital_e() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(format!("{v:E}"), "1.5E0");
    }

    // ── LowerHex / UpperHex / Octal / Binary ──

    /// LowerHex of D38s12::ONE is the hex of 10^12 (= 0xe8d4a51000),
    /// NOT the hex of `1.0` formatted as a decimal in hex.
    #[cfg(feature = "alloc")]
    #[test]
    fn lower_hex_is_storage() {
        assert_eq!(format!("{:x}", D38s12::ONE), "e8d4a51000");
    }

    /// UpperHex of ONE: same digits in upper case.
    #[cfg(feature = "alloc")]
    #[test]
    fn upper_hex_is_storage() {
        assert_eq!(format!("{:X}", D38s12::ONE), "E8D4A51000");
    }

    /// Octal of ZERO is `0`.
    #[cfg(feature = "alloc")]
    #[test]
    fn octal_zero() {
        assert_eq!(format!("{:o}", D38s12::ZERO), "0");
    }

    /// Binary of ONE has the `10^12` bit pattern (40 bits).
    #[cfg(feature = "alloc")]
    #[test]
    fn binary_one() {
        // 10^12 in binary: 1110_1000_1101_0100_1010_0101_0001_0000_0000_0000
        let s = format!("{:b}", D38s12::ONE);
        assert_eq!(s, "1110100011010100101001010001000000000000");
    }

    // ── ParseError Display ──

    #[cfg(feature = "alloc")]
    #[test]
    fn parse_error_display_messages() {
        assert_eq!(ParseError::Empty.to_string(), "empty input");
        assert_eq!(ParseError::SignOnly.to_string(), "sign with no digits");
        assert_eq!(
            ParseError::LeadingZero.to_string(),
            "redundant leading zero in integer part"
        );
        assert_eq!(
            ParseError::OverlongFractional.to_string(),
            "fractional part exceeds SCALE digits"
        );
        assert_eq!(
            ParseError::ScientificNotation.to_string(),
            "scientific notation not accepted"
        );
        assert_eq!(ParseError::InvalidChar.to_string(), "invalid character");
        assert_eq!(
            ParseError::OutOfRange.to_string(),
            "value out of representable range"
        );
        assert_eq!(
            ParseError::MissingDigits.to_string(),
            "decimal point with no adjacent digits"
        );
    }

    // ── FromStr happy path ──

    #[test]
    fn from_str_zero() {
        let v: D38s12 = "0".parse().unwrap();
        assert_eq!(v, D38s12::ZERO);
        let v: D38s12 = "0.0".parse().unwrap();
        assert_eq!(v, D38s12::ZERO);
    }

    #[test]
    fn from_str_one() {
        let v: D38s12 = "1".parse().unwrap();
        assert_eq!(v, D38s12::ONE);
        let v: D38s12 = "1.0".parse().unwrap();
        assert_eq!(v, D38s12::ONE);
    }

    /// Headline base-10 claim: `1.1` parses bit-exact.
    #[test]
    fn from_str_one_point_one_parses_exactly() {
        let v: D38s12 = "1.1".parse().unwrap();
        assert_eq!(v.to_bits(), 1_100_000_000_000);
    }

    /// Sign prefix.
    #[test]
    fn from_str_signs() {
        let neg: D38s12 = "-1.5".parse().unwrap();
        assert_eq!(neg.to_bits(), -1_500_000_000_000);

        let pos: D38s12 = "+1.5".parse().unwrap();
        assert_eq!(pos.to_bits(), 1_500_000_000_000);
    }

    /// Fractional with fewer digits than SCALE pads correctly.
    #[test]
    fn from_str_short_fractional_pads() {
        // "0.5" at SCALE 12 -> 5_000_000_000 (= 0.5 * 10^12).
        let v: D38s12 = "0.5".parse().unwrap();
        assert_eq!(v.to_bits(), 500_000_000_000);
    }

    /// Fractional with exactly SCALE digits is the natural form.
    #[test]
    fn from_str_full_scale_fractional() {
        let v: D38s12 = "1.500000000000".parse().unwrap();
        assert_eq!(v.to_bits(), 1_500_000_000_000);
    }

    // ── FromStr error paths ──

    #[test]
    fn from_str_empty_is_err() {
        let r: Result<D38s12, _> = "".parse();
        assert_eq!(r, Err(ParseError::Empty));
    }

    #[test]
    fn from_str_sign_only_is_err() {
        assert_eq!("-".parse::<D38s12>(), Err(ParseError::SignOnly));
        assert_eq!("+".parse::<D38s12>(), Err(ParseError::SignOnly));
    }

    #[test]
    fn from_str_leading_zero_is_err() {
        assert_eq!("01".parse::<D38s12>(), Err(ParseError::LeadingZero));
        assert_eq!("01.5".parse::<D38s12>(), Err(ParseError::LeadingZero));
        assert_eq!("00".parse::<D38s12>(), Err(ParseError::LeadingZero));
    }

    #[test]
    fn from_str_overlong_fractional_is_err() {
        // SCALE 12, fractional length 13 -> reject.
        let r: Result<D38s12, _> = "0.1234567890123".parse();
        assert_eq!(r, Err(ParseError::OverlongFractional));
    }

    #[test]
    fn from_str_scientific_notation_is_err() {
        assert_eq!("1e3".parse::<D38s12>(), Err(ParseError::ScientificNotation));
        assert_eq!(
            "1.5E2".parse::<D38s12>(),
            Err(ParseError::ScientificNotation)
        );
    }

    #[test]
    fn from_str_invalid_char_is_err() {
        assert_eq!("garbage".parse::<D38s12>(), Err(ParseError::InvalidChar));
        assert_eq!("1.2x".parse::<D38s12>(), Err(ParseError::InvalidChar));
        assert_eq!("1..2".parse::<D38s12>(), Err(ParseError::InvalidChar));
    }

    #[test]
    fn from_str_missing_digits_is_err() {
        assert_eq!(".5".parse::<D38s12>(), Err(ParseError::MissingDigits));
        assert_eq!("5.".parse::<D38s12>(), Err(ParseError::MissingDigits));
        assert_eq!("-.5".parse::<D38s12>(), Err(ParseError::MissingDigits));
    }

    #[test]
    fn from_str_out_of_range_is_err() {
        // 10^39 > i128::MAX (~1.7e38). At SCALE 12, the maximum
        // integer part is i128::MAX / 10^12 ~= 1.7e26, so an integer
        // part of 1e27 already overflows.
        let r: Result<D38s12, _> = "1000000000000000000000000000".parse();
        assert_eq!(r, Err(ParseError::OutOfRange));
    }

    /// Parse exactly at i128::MIN -- the asymmetric two's-complement
    /// boundary. At SCALE 12:
    /// `i128::MIN = -170141183460469231731687303715884105728`
    /// which splits into integer `170141183460469231731687303` and
    /// fractional `715884105728` (the negative form has the same
    /// digits since |MIN| = MAX + 1).
    #[test]
    fn from_str_i128_min_boundary() {
        let s = "-170141183460469231731687303.715884105728";
        let v: D38s12 = s.parse().unwrap();
        assert_eq!(v.to_bits(), i128::MIN);
    }

    /// Parse exactly at i128::MAX boundary. At SCALE 12 the canonical
    /// form is `170141183460469231731687303.715884105727`.
    #[test]
    fn from_str_i128_max_boundary() {
        let s = "170141183460469231731687303.715884105727";
        let v: D38s12 = s.parse().unwrap();
        assert_eq!(v.to_bits(), i128::MAX);
    }

    /// One-past-MAX positive overflows.
    #[test]
    fn from_str_just_above_max_overflows() {
        // ...728 is one fractional LSB above i128::MAX.
        let s = "170141183460469231731687303.715884105728";
        let r: Result<D38s12, _> = s.parse();
        assert_eq!(r, Err(ParseError::OutOfRange));
    }

    // ── Property tests: parse(value.to_string()) round-trip ──

    /// Round-trip property for representative storage values.
    /// Uses safe-decimal-test-values (no clippy approx_constant traps).
    #[cfg(feature = "alloc")]
    #[test]
    fn round_trip_representative_values() {
        let cases: &[i128] = &[
            0,
            1,
            -1,
            1_000_000_000_000, // 1.0
            -1_000_000_000_000,
            1_500_000_000_000, // 1.5
            -1_500_000_000_000,
            1_100_000_000_000, // 1.1 (the headline base-10 claim)
            2_200_000_000_000, // 2.2
            3_300_000_000_000, // 3.3
            // Safe arbitrary-looking literal (avoids approx_constant
            // triggers like 3.14, 2.718, 1.414 etc.):
            1_234_567_890_123, // ~1.234567890123
            -1_234_567_890_123,
            4_567_891_234_567, // ~4.567891234567
            7_890_123_456_789, // ~7.890123456789
            i128::MAX,
            i128::MIN,
            i128::MAX / 2,
            i128::MIN / 2,
        ];
        for &raw in cases {
            let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let s = v.to_string();
            let parsed: D38s12 = s.parse().unwrap_or_else(|e| {
                panic!("round-trip parse failed for raw={raw}, s={s:?}, err={e:?}")
            });
            assert_eq!(
                parsed.to_bits(),
                raw,
                "round-trip mismatch: raw={raw}, s={s:?}, parsed_bits={}",
                parsed.to_bits()
            );
        }
    }

    /// Round-trip property at SCALE = 6 to exercise the const-generic
    /// path away from the v1 SCALE = 12.
    #[cfg(feature = "alloc")]
    #[test]
    fn round_trip_other_scale() {
        type D6 = D38<6>;
        let cases: &[i128] = &[
            0,
            1,
            -1,
            1_000_000,
            -1_000_000,
            1_500_000,
            i128::MAX,
            i128::MIN,
        ];
        for &raw in cases {
            let v = D6::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let s = v.to_string();
            let parsed: D6 = s.parse().expect("round-trip parse");
            assert_eq!(
                parsed.to_bits(),
                raw,
                "round-trip mismatch at SCALE=6, raw={raw}"
            );
        }
    }

    /// Round-trip at SCALE = 0 (integer-only) to exercise the
    /// no-decimal-point path.
    #[cfg(feature = "alloc")]
    #[test]
    fn round_trip_scale_zero() {
        type D0 = D38<0>;
        let cases: &[i128] = &[0, 1, -1, 42, -42, i128::MAX, i128::MIN];
        for &raw in cases {
            let v = D0::from_bits(crate::int::types::Int::<2>::from_i128(raw));
            let s = v.to_string();
            let parsed: D0 = s.parse().expect("round-trip parse");
            assert_eq!(
                parsed.to_bits(),
                raw,
                "round-trip mismatch at SCALE=0, raw={raw}"
            );
        }
    }
}
