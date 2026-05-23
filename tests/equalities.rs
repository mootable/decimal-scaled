//! Cross-type equality tests: `D38 == primitive` (integer and float).
//! Moved out of `src/equalities.rs` so that file carries only macro
//! invocations.

use decimal_scaled::D38s12;

// --- signed integers --------------------------------------------------

#[test]
fn eq_signed_exact_match() {
    assert!(D38s12::try_from(5).unwrap() == 5_i32);
    assert!(5_i32 == D38s12::try_from(5).unwrap());
    assert!(D38s12::try_from(-7).unwrap() == -7_i64);
    assert!(D38s12::ZERO == 0_i8);
}

#[test]
fn eq_signed_fractional_is_false() {
    let one_and_a_half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap());
    assert!(!(one_and_a_half == 1_i32));
    assert!(!(one_and_a_half == 2_i32));
}

#[test]
fn eq_signed_one_lsb_is_false() {
    let just_above_zero = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap());
    assert!(!(just_above_zero == 0_i32));
}

#[test]
fn eq_i128_no_overflow_at_extremes() {
    let huge = i128::MAX / i128::from(D38s12::multiplier());
    let d = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(huge * i128::from(D38s12::multiplier())).unwrap());
    assert!(d == huge);
}

#[test]
fn eq_i128_negative() {
    let d = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(-42_000_000_000_000_i128).unwrap());
    assert!(d == -42_i128);
    assert!(-42_i128 == d);
}

// --- unsigned integers ------------------------------------------------

#[test]
fn eq_unsigned_exact_match() {
    assert!(D38s12::try_from(5).unwrap() == 5_u32);
    assert!(5_u64 == D38s12::try_from(5).unwrap());
    assert!(D38s12::ZERO == 0_u8);
}

#[test]
fn eq_unsigned_negative_is_false() {
    let neg = D38s12::try_from(-1).unwrap();
    assert!(!(neg == 0_u32));
    assert!(!(neg == 1_u32));
}

#[test]
fn eq_u128_large_value() {
    let n: u128 = 1_000_000_u128;
    let d = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((n as i128) * i128::from(D38s12::multiplier())).unwrap());
    assert!(d == n);
}

#[test]
fn eq_u128_out_of_d38_range_is_false() {
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
    let d = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap());
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
#[allow(clippy::eq_op)]
fn eq_float_nan_is_false() {
    // Intentional: tests the crate's `D38 == f64` impl rejects NaN
    // (any NaN comparison returns false). The lint flags direct
    // `nan == nan` style code in general; this is the correct
    // semantics for the type's PartialEq impl with a NaN operand.
    #[allow(invalid_nan_comparisons)]
    {
        assert!(!(D38s12::ZERO == f64::NAN));
        assert!(!(D38s12::ZERO == f32::NAN));
    }
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
fn eq_float_exact_not_round_trip() {
    // `==` is EXACT value equality, not a `from_f64`/`to_f64` round-trip.
    // `1.1_f64` is the dyadic 1.1000000000000000888..., which is NOT the
    // decimal 1.1, so it must compare unequal to the decimal storing 1.1.
    let one_point_one: D38s12 = "1.1".parse().unwrap();
    assert!(!(one_point_one == 1.1_f64));
    assert!(!(1.1_f64 == one_point_one));
    // The exact dyadic value 1.5 still matches the decimal 1.5.
    let one_point_five: D38s12 = "1.5".parse().unwrap();
    assert!(one_point_five == 1.5_f64);
}

#[cfg(feature = "std")]
#[test]
fn eq_float_negative() {
    let d = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-2_500_000_000_000) as i128).unwrap());
    assert!(d == -2.5_f64);
    assert!(-2.5_f64 == d);
}

// --- D9 / D18 cross-equality (uses the macro just like D38) --------

#[test]
fn eq_d18_with_integer() {
    use decimal_scaled::D18s9;
    let v = D18s9::from_bits(decimal_scaled::Int::<1>::from((7_000_000_000) as i64)); // 7.0
    assert!(v == 7_i64);
    assert!(v == 7_u64);
    let neg = D18s9::from_bits(decimal_scaled::Int::<1>::from((-7_000_000_000) as i64));
    assert!(neg == -7_i32);
    assert!(!(neg == 7_u32));
}
