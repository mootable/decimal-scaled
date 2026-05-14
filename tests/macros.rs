#![cfg(feature = "macros")]
//! Integration tests for the `d128!` proc-macro.
//!
//! See `macros/Macros.md` for the macro spec. This corpus covers the
//! Phase 1C feature set: decimal literals (positive/negative,
//! fractional, underscore separators), scientific notation, the
//! `scale N` and `rounded` qualifiers, and the compile-time error path
//! when a literal is lossy without `rounded`.

use decimal_scaled::{D128, D128s0, D128s2, D128s5, D128s6, D128s12, d128};

// --- Auto-scale inference --------------------------------------------

#[test]
fn auto_scale_inference_simple() {
    let v = d128!(1.23);
    assert_eq!(v, D128s2::from_bits(123));
}

#[test]
fn auto_scale_inference_integer() {
    let v = d128!(123);
    assert_eq!(v, D128s0::from_bits(123));
}

#[test]
fn auto_scale_inference_zero_fractional() {
    let v = d128!(1.0);
    assert_eq!(v, D128::<1>::from_bits(10));
}

#[test]
fn auto_scale_inference_trailing_zero_preserved() {
    let v = d128!(1.230);
    assert_eq!(v, D128::<3>::from_bits(1230));
}

#[test]
fn auto_scale_inference_leading_zero_after_dot() {
    let v = d128!(0.001);
    assert_eq!(v, D128::<3>::from_bits(1));
}

#[test]
fn auto_scale_inference_negative() {
    let v = d128!(-1.23);
    assert_eq!(v, D128s2::from_bits(-123));
}

#[test]
fn auto_scale_inference_with_underscores() {
    let v = d128!(1_234.567_89);
    assert_eq!(v, D128s5::from_bits(123_456_789));
}

// --- Scientific notation ---------------------------------------------

#[test]
fn sci_simple() {
    // 1.5e3 = 1500; mantissa scale 1, exponent 3 -> natural scale = max(0, 1-3) = 0.
    let v = d128!(1.5e3);
    assert_eq!(v, D128s0::from_bits(1500));
}

#[test]
fn sci_negative_exponent() {
    // 1.5e-3 = 0.0015; mantissa scale 1, exponent -3 -> natural scale = 4.
    let v = d128!(1.5e-3);
    assert_eq!(v, D128::<4>::from_bits(15));
}

#[test]
fn sci_zero_exponent() {
    let v = d128!(1.5e0);
    assert_eq!(v, D128::<1>::from_bits(15));
}

#[test]
fn sci_integer_with_exponent() {
    // 1e6 = 1_000_000; mantissa scale 0, exponent 6 -> natural scale 0,
    // digits padded by 6 zeros.
    let v = d128!(1e6);
    assert_eq!(v, D128s0::from_bits(1_000_000));
}

#[test]
fn sci_trailing_zero_mantissa() {
    // 1.500e2 = 150.0; mantissa scale 3, exponent 2 -> natural scale 1.
    let v = d128!(1.500e2);
    assert_eq!(v, D128::<1>::from_bits(1500));
}

#[test]
fn sci_negative_value() {
    let v = d128!(-2.5e-2);
    assert_eq!(v, D128::<3>::from_bits(-25));
}

// --- `scale N` qualifier ---------------------------------------------

#[test]
fn explicit_scale_up_pads_with_zeros() {
    let v = d128!(1.23, scale 4);
    assert_eq!(v, D128::<4>::from_bits(12_300));
}

#[test]
fn explicit_scale_equal_is_identity() {
    let v = d128!(1.23, scale 2);
    assert_eq!(v, D128s2::from_bits(123));
}

#[test]
fn explicit_scale_zero_on_integer() {
    let v = d128!(42, scale 0);
    assert_eq!(v, D128s0::from_bits(42));
}

#[test]
fn explicit_scale_with_sci_notation() {
    // 1.5e3 = 1500 (natural scale 0); explicit scale 5 -> pad with 5 zeros.
    let v = d128!(1.5e3, scale 5);
    assert_eq!(v, D128::<5>::from_bits(150_000_000));
}

// --- `rounded` qualifier ---------------------------------------------

#[test]
fn rounded_half_to_even_below_half() {
    // 1.234999 at scale 2 -> 1.23 (below half).
    let v = d128!(1.234999, scale 2, rounded);
    assert_eq!(v, D128s2::from_bits(123));
}

#[test]
fn rounded_half_to_even_above_half() {
    // 1.235001 at scale 2 -> 1.24 (above half).
    let v = d128!(1.235001, scale 2, rounded);
    assert_eq!(v, D128s2::from_bits(124));
}

#[test]
fn rounded_half_to_even_exact_half_ties_to_even() {
    // 1.235 at scale 2 -> 1.24 (tie, 4 is even).
    let v = d128!(1.235, scale 2, rounded);
    assert_eq!(v, D128s2::from_bits(124));

    // 1.225 at scale 2 -> 1.22 (tie, 2 is even).
    let v = d128!(1.225, scale 2, rounded);
    assert_eq!(v, D128s2::from_bits(122));
}

#[test]
fn rounded_negative_value() {
    let v = d128!(-1.235, scale 2, rounded);
    assert_eq!(v, D128s2::from_bits(-124));
}

// --- More named precision tiers --------------------------------------

#[test]
fn explicit_scale_to_max_supported() {
    let v = d128!(1, scale 38);
    assert_eq!(v.to_bits(), 10_i128.pow(38));
}

#[test]
fn primary_alias_d128s12() {
    let v = d128!(1.500_000_000_000);
    assert_eq!(v, D128s12::from_bits(1_500_000_000_000));
}

#[test]
fn financial_cents() {
    let v = d128!(0.50);
    assert_eq!(v, D128s2::from_bits(50));
}

#[test]
fn satoshi_grade() {
    // 0.123_456_78 at scale 8.
    let v = d128!(0.123_456_78);
    assert_eq!(v, D128::<8>::from_bits(12_345_678));
}

#[test]
fn small_value_via_sci() {
    let v = d128!(1e-9);
    assert_eq!(v, D128::<9>::from_bits(1));
}

#[test]
fn six_digit_micro() {
    let v = d128!(1.234_567, scale 6);
    assert_eq!(v, D128s6::from_bits(1_234_567));
}
