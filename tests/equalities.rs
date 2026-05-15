//! Cross-type equality tests: `D38 == primitive` (integer and float).
//! Moved out of `src/equalities.rs` so that file carries only macro
//! invocations.

use decimal_scaled::{D38, D38s12};

// --- signed integers --------------------------------------------------

#[test]
fn eq_signed_exact_match() {
    assert!(D38s12::from_int(5) == 5_i32);
    assert!(5_i32 == D38s12::from_int(5));
    assert!(D38s12::from_int(-7) == -7_i64);
    assert!(D38s12::ZERO == 0_i8);
}

#[test]
fn eq_signed_fractional_is_false() {
    let one_and_a_half = D38s12::from_bits(1_500_000_000_000);
    assert!(!(one_and_a_half == 1_i32));
    assert!(!(one_and_a_half == 2_i32));
}

#[test]
fn eq_signed_one_lsb_is_false() {
    let just_above_zero = D38s12::from_bits(1);
    assert!(!(just_above_zero == 0_i32));
}

#[test]
fn eq_i128_no_overflow_at_extremes() {
    let huge = i128::MAX / D38s12::multiplier();
    let d = D38s12::from_bits(huge * D38s12::multiplier());
    assert!(d == huge);
}

#[test]
fn eq_i128_negative() {
    let d = D38s12::from_bits(-42_000_000_000_000);
    assert!(d == -42_i128);
    assert!(-42_i128 == d);
}

// --- unsigned integers ------------------------------------------------

#[test]
fn eq_unsigned_exact_match() {
    assert!(D38s12::from_int(5) == 5_u32);
    assert!(5_u64 == D38s12::from_int(5));
    assert!(D38s12::ZERO == 0_u8);
}

#[test]
fn eq_unsigned_negative_is_false() {
    let neg = D38s12::from_int(-1);
    assert!(!(neg == 0_u32));
    assert!(!(neg == 1_u32));
}

#[test]
fn eq_u128_large_value() {
    let n: u128 = 1_000_000_u128;
    let d = D38s12::from_bits((n as i128) * D38s12::multiplier());
    assert!(d == n);
}

#[test]
fn eq_u128_out_of_d128_range_is_false() {
    // A u128 value larger than D38::MAX after scaling cannot match.
    let too_big: u128 = u128::MAX;
    let d = D38s12::MAX;
    assert!(!(d == too_big));
}

// --- floats -----------------------------------------------------------

#[cfg(feature = "std")]
#[test]
fn eq_float_exact_representable() {
    // 1.5 is exactly representable in both f64 and D38s12.
    let d = D38s12::from_bits(1_500_000_000_000);
    assert!(d == 1.5_f64);
    assert!(1.5_f64 == d);
    assert!(d == 1.5_f32);
}

#[cfg(feature = "std")]
#[test]
fn eq_float_zero_and_one() {
    assert!(D38s12::ZERO == 0.0_f64);
    assert!(D38s12::ONE == 1.0_f64);
    assert!(D38s12::ZERO == 0.0_f32);
    assert!(D38s12::ONE == 1.0_f32);
}

#[cfg(feature = "std")]
#[test]
fn eq_float_nan_is_false() {
    assert!(!(D38s12::ZERO == f64::NAN));
    assert!(!(D38s12::ZERO == f32::NAN));
}

#[cfg(feature = "std")]
#[test]
fn eq_float_infinity_is_false() {
    assert!(!(D38s12::MAX == f64::INFINITY));
    assert!(!(D38s12::MIN == f64::NEG_INFINITY));
    assert!(!(D38s12::MAX == f32::INFINITY));
}

#[cfg(feature = "std")]
#[test]
fn eq_float_negative() {
    let d = D38s12::from_bits(-2_500_000_000_000);
    assert!(d == -2.5_f64);
    assert!(-2.5_f64 == d);
}

// --- D9 / D18 cross-equality (uses the macro just like D38) --------

#[test]
fn eq_d32_with_integer() {
    use decimal_scaled::D9s2;
    let v = D9s2::from_bits(150); // 1.50
    assert!(!(v == 1_i32));
    assert!(!(v == 2_i32));
    let int = D9s2::from_bits(500); // 5.00
    assert!(int == 5_i32);
    assert!(5_i32 == int);
    assert!(int == 5_u8);
}

#[test]
fn eq_d64_with_integer() {
    use decimal_scaled::D18s9;
    let v = D18s9::from_bits(7_000_000_000); // 7.0
    assert!(v == 7_i64);
    assert!(v == 7_u64);
    let neg = D18s9::from_bits(-7_000_000_000);
    assert!(neg == -7_i32);
    assert!(!(neg == 7_u32));
}
