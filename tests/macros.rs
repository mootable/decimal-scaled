#![cfg(feature = "macros")]
//! Integration tests for the `d38!` proc-macro.
//!
//! See `macros/Macros.md` for the macro spec. This corpus covers the
//! Phase 1C feature set: decimal literals (positive/negative,
//! fractional, underscore separators), scientific notation, the
//! `scale N` and `rounded` qualifiers, and the compile-time error path
//! when a literal is lossy without `rounded`.

use decimal_scaled::{D38, D38s0, D38s2, D38s5, D38s6, D38s12, d38};

// --- Auto-scale inference --------------------------------------------

#[test]
fn auto_scale_inference_simple() {
    let v = d38!(1.23);
    assert_eq!(v, D38s2::from_bits(123));
}

#[test]
fn auto_scale_inference_integer() {
    let v = d38!(123);
    assert_eq!(v, D38s0::from_bits(123));
}

#[test]
fn auto_scale_inference_zero_fractional() {
    let v = d38!(1.0);
    assert_eq!(v, D38::<1>::from_bits(10));
}

#[test]
fn auto_scale_inference_trailing_zero_preserved() {
    let v = d38!(1.230);
    assert_eq!(v, D38::<3>::from_bits(1230));
}

#[test]
fn auto_scale_inference_leading_zero_after_dot() {
    let v = d38!(0.001);
    assert_eq!(v, D38::<3>::from_bits(1));
}

#[test]
fn auto_scale_inference_negative() {
    let v = d38!(-1.23);
    assert_eq!(v, D38s2::from_bits(-123));
}

#[test]
fn auto_scale_inference_with_underscores() {
    let v = d38!(1_234.567_89);
    assert_eq!(v, D38s5::from_bits(123_456_789));
}

// --- Scientific notation ---------------------------------------------

#[test]
fn sci_simple() {
    // 1.5e3 = 1500; mantissa scale 1, exponent 3 -> natural scale = max(0, 1-3) = 0.
    let v = d38!(1.5e3);
    assert_eq!(v, D38s0::from_bits(1500));
}

#[test]
fn sci_negative_exponent() {
    // 1.5e-3 = 0.0015; mantissa scale 1, exponent -3 -> natural scale = 4.
    let v = d38!(1.5e-3);
    assert_eq!(v, D38::<4>::from_bits(15));
}

#[test]
fn sci_zero_exponent() {
    let v = d38!(1.5e0);
    assert_eq!(v, D38::<1>::from_bits(15));
}

#[test]
fn sci_integer_with_exponent() {
    // 1e6 = 1_000_000; mantissa scale 0, exponent 6 -> natural scale 0,
    // digits padded by 6 zeros.
    let v = d38!(1e6);
    assert_eq!(v, D38s0::from_bits(1_000_000));
}

#[test]
fn sci_trailing_zero_mantissa() {
    // 1.500e2 = 150.0; mantissa scale 3, exponent 2 -> natural scale 1.
    let v = d38!(1.500e2);
    assert_eq!(v, D38::<1>::from_bits(1500));
}

#[test]
fn sci_negative_value() {
    let v = d38!(-2.5e-2);
    assert_eq!(v, D38::<3>::from_bits(-25));
}

// --- `scale N` qualifier ---------------------------------------------

#[test]
fn explicit_scale_up_pads_with_zeros() {
    let v = d38!(1.23, scale 4);
    assert_eq!(v, D38::<4>::from_bits(12_300));
}

#[test]
fn explicit_scale_equal_is_identity() {
    let v = d38!(1.23, scale 2);
    assert_eq!(v, D38s2::from_bits(123));
}

#[test]
fn explicit_scale_zero_on_integer() {
    let v = d38!(42, scale 0);
    assert_eq!(v, D38s0::from_bits(42));
}

#[test]
fn explicit_scale_with_sci_notation() {
    // 1.5e3 = 1500 (natural scale 0); explicit scale 5 -> pad with 5 zeros.
    let v = d38!(1.5e3, scale 5);
    assert_eq!(v, D38::<5>::from_bits(150_000_000));
}

// --- `rounded` qualifier ---------------------------------------------

#[test]
fn rounded_half_to_even_below_half() {
    // 1.234999 at scale 2 -> 1.23 (below half).
    let v = d38!(1.234999, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(123));
}

#[test]
fn rounded_half_to_even_above_half() {
    // 1.235001 at scale 2 -> 1.24 (above half).
    let v = d38!(1.235001, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(124));
}

#[test]
fn rounded_half_to_even_exact_half_ties_to_even() {
    // 1.235 at scale 2 -> 1.24 (tie, 4 is even).
    let v = d38!(1.235, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(124));

    // 1.225 at scale 2 -> 1.22 (tie, 2 is even).
    let v = d38!(1.225, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(122));
}

#[test]
fn rounded_negative_value() {
    let v = d38!(-1.235, scale 2, rounded);
    assert_eq!(v, D38s2::from_bits(-124));
}

// --- More named precision tiers --------------------------------------

#[test]
fn explicit_scale_to_max_supported() {
    let v = d38!(1, scale 38);
    assert_eq!(v.to_bits(), 10_i128.pow(38));
}

#[test]
fn primary_alias_d38s12() {
    let v = d38!(1.500_000_000_000);
    assert_eq!(v, D38s12::from_bits(1_500_000_000_000));
}

#[test]
fn financial_cents() {
    let v = d38!(0.50);
    assert_eq!(v, D38s2::from_bits(50));
}

#[test]
fn satoshi_grade() {
    // 0.123_456_78 at scale 8.
    let v = d38!(0.123_456_78);
    assert_eq!(v, D38::<8>::from_bits(12_345_678));
}

#[test]
fn small_value_via_sci() {
    let v = d38!(1e-9);
    assert_eq!(v, D38::<9>::from_bits(1));
}

#[test]
fn six_digit_micro() {
    let v = d38!(1.234_567, scale 6);
    assert_eq!(v, D38s6::from_bits(1_234_567));
}

// --- Inline expressions ---------------------------------------------

#[test]
fn expression_simple_arithmetic() {
    let v = d38!(10 * 12 + 3, scale 0);
    assert_eq!(v, D38s0::from_bits(123));
}

#[test]
fn expression_with_scale_factor() {
    // Scale 4 means each input unit becomes 10_000 bits.
    let v = d38!(5, scale 4);
    assert_eq!(v, D38::<4>::from_bits(50_000));
}

#[test]
fn expression_with_variable() {
    let x: i128 = 42;
    let v = d38!(x, scale 2);
    assert_eq!(v, D38s2::from_bits(4_200));
}

#[test]
fn expression_with_const() {
    const N: i128 = 100;
    let v = d38!(N + 23, scale 2);
    assert_eq!(v, D38s2::from_bits(12_300));
}

#[test]
fn expression_function_call() {
    fn produce() -> i128 {
        7
    }
    let v = d38!(produce() * 2, scale 0);
    assert_eq!(v, D38s0::from_bits(14));
}

#[test]
fn expression_negative_result() {
    let v = d38!(0 - 5, scale 2);
    assert_eq!(v, D38s2::from_bits(-500));
}

#[test]
fn expression_const_context_works() {
    const N: i128 = 42;
    const V: D38s2 = d38!(N * 3, scale 2);
    assert_eq!(V, D38s2::from_bits(12_600));
}
