//! Wide-tier `FromStr` coverage at deep scales.
//!
//! Regression guard for the historical `u128` ceiling on the shared
//! parser. Before the fix, the parser materialised `10^SCALE` in
//! `u128`, which overflows at `SCALE > 38` and made any wide-tier
//! `from_str` call at deep scale fail with
//! [`decimal_scaled::ParseError::OutOfRange`] (or worse — silently
//! wrap to garbage). After the fix the wide-tier from-str macro
//! arm does the arithmetic at the storage width, so deep-scale
//! literals round-trip cleanly.

#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use core::str::FromStr;
use decimal_scaled::{D1232, D307, D38, D76};

#[test]
fn d76_deep_scale_parses_one_point_five() {
    // SCALE = 60 was the first failure point: 10^60 overflows u128
    // (u128::MAX ~ 3.4e38).
    let v = D76::<60>::from_str("1.5").expect("D76<60>::from_str(\"1.5\")");
    let round_trip = v.to_string();
    // SCALE = 60 fractional digits, leading "1.5" then 59 zeros.
    let expected = format!("1.5{}", "0".repeat(59));
    assert_eq!(round_trip, expected);
}

#[test]
fn d307_deep_scale_parses_one_point_five() {
    let v = D307::<150>::from_str("1.5").expect("D307<150>::from_str(\"1.5\")");
    let round_trip = v.to_string();
    let expected = format!("1.5{}", "0".repeat(149));
    assert_eq!(round_trip, expected);
}

#[test]
fn d307_deep_scale_round_trip_back_to_value() {
    let s_in = "1.5";
    let v = D307::<150>::from_str(s_in).expect("parse");
    let s_out = v.to_string();
    let reparsed = D307::<150>::from_str(&s_out).expect("reparse");
    assert_eq!(v, reparsed);
}

#[test]
fn d1231_deepest_scale_parses_one_point_five() {
    // Deepest supported tier × scale combination.
    let v = D1232::<1230>::from_str("1.5").expect("D1232<1230>::from_str(\"1.5\")");
    let round_trip = v.to_string();
    let expected = format!("1.5{}", "0".repeat(1229));
    assert_eq!(round_trip, expected);
}

#[test]
fn d1231_deepest_scale_handles_negative() {
    // Negative path: the per-storage accumulator subtracts each digit
    // rather than negating after the fact, so the asymmetric two's-
    // complement `MIN` boundary is reachable without overflowing on
    // the positive side. Spot-check a non-boundary negative value.
    let v = D1232::<1230>::from_str("-1.5").expect("negative parse");
    let round_trip = v.to_string();
    let expected = format!("-1.5{}", "0".repeat(1229));
    assert_eq!(round_trip, expected);
}

#[test]
fn d307_deep_scale_many_fractional_digits() {
    // Exercise the per-digit `checked_mul(10) + checked_add(digit)`
    // loop with a long fractional run.
    let frac = "1234567890".repeat(10); // 100 digits
    let input = format!("0.{frac}");
    let v = D307::<150>::from_str(&input).expect("parse");
    let s_out = v.to_string();
    // Round-trip preserves the value bit-exactly; the displayed form
    // has SCALE = 150 fractional digits, so 50 trailing zeros pad
    // the 100-digit input.
    let expected = format!("0.{frac}{}", "0".repeat(50));
    assert_eq!(s_out, expected);
}

#[test]
fn d307_deep_scale_zero() {
    let v = D307::<150>::from_str("0").expect("parse 0");
    assert_eq!(v.to_string(), format!("0.{}", "0".repeat(150)));
    let v = D307::<150>::from_str("0.0").expect("parse 0.0");
    assert_eq!(v.to_string(), format!("0.{}", "0".repeat(150)));
}

#[test]
fn d307_deep_scale_overlong_fractional_is_err() {
    // SCALE = 150, fractional length 151 → reject.
    let frac = "0".repeat(151);
    let input = format!("0.{frac}");
    assert!(D307::<150>::from_str(&input).is_err());
}

#[test]
fn d76_deep_scale_integer_only() {
    // No decimal point: only the integer-scale path is exercised.
    let v = D76::<60>::from_str("42").expect("parse 42");
    let expected = format!("42.{}", "0".repeat(60));
    assert_eq!(v.to_string(), expected);
}

#[test]
fn d38_shallow_scale_unchanged_after_refactor() {
    // Sanity: the narrow-tier parser still produces the same value
    // after the front-end was extracted to `parse_components`.
    let v: D38<12> = "1.5".parse().expect("D38<12> parses");
    assert_eq!(v.to_string(), "1.500000000000");

    let neg: D38<12> = "-1.5".parse().expect("D38<12> negative parses");
    assert_eq!(neg.to_string(), "-1.500000000000");
}
