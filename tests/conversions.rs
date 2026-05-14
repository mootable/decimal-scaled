//! Integration tests for the conversion surface
//! (From<integer> / TryFrom<i128|u128|f32|f64> / from_int /
//! from_i32 / to_int_lossy / from_f64_lossy / to_f64_lossy /
//! to_f32_lossy).
//!
//! Bodies live in src/macros/conversions.rs and float_bridge.rs;
//! these tests exercise the resulting public API for D128 specifically.

use decimal_scaled::{ConvertError, D128, D128s12};

// from_int / from_i32 -- foundation wrappers around From<iN>

#[test]
fn from_int_zero_is_zero() {
    assert_eq!(D128s12::from_int(0), D128s12::ZERO);
}

#[test]
fn from_i32_zero_is_zero() {
    assert_eq!(D128s12::from_i32(0), D128s12::ZERO);
}

#[test]
fn from_int_one_is_one() {
    assert_eq!(D128s12::from_int(1), D128s12::ONE);
}

#[test]
fn from_i32_one_is_one() {
    assert_eq!(D128s12::from_i32(1), D128s12::ONE);
}

#[test]
fn from_int_negative() {
    assert_eq!(D128s12::from_int(-1), -D128s12::ONE);
    assert_eq!(D128s12::from_int(-42).to_bits(), -42_000_000_000_000_i128);
}

// Lossless From<iN> / From<uN> -- bit-exact scaling

#[test]
fn from_i8_scales_correctly() {
    assert_eq!(D128s12::from(0_i8).to_bits(), 0);
    assert_eq!(D128s12::from(1_i8).to_bits(), 1_000_000_000_000);
    assert_eq!(D128s12::from(-1_i8).to_bits(), -1_000_000_000_000);
    assert_eq!(D128s12::from(i8::MAX).to_bits(), 127_000_000_000_000);
    assert_eq!(D128s12::from(i8::MIN).to_bits(), -128_000_000_000_000);
}

#[test]
fn from_i64_scales_correctly() {
    assert_eq!(D128s12::from(0_i64).to_bits(), 0);
    assert_eq!(D128s12::from(i64::MAX).to_bits(), (i64::MAX as i128) * 1_000_000_000_000);
    assert_eq!(D128s12::from(i64::MIN).to_bits(), (i64::MIN as i128) * 1_000_000_000_000);
}

#[test]
fn from_u64_at_boundary_is_lossless() {
    let v = D128s12::from(u64::MAX);
    assert_eq!(v.to_bits(), (u64::MAX as i128) * 1_000_000_000_000);
}

// to_int_lossy

#[test]
fn to_int_lossy_default_rounds_half_to_even() {
    // 2.5 with HalfToEven default -> 2 (even neighbour).
    assert_eq!(D128s12::from_bits(2_500_000_000_000).to_int_lossy(), 2);
    // 3.5 with HalfToEven -> 4 (even).
    assert_eq!(D128s12::from_bits(3_500_000_000_000).to_int_lossy(), 4);
}

#[test]
fn to_int_lossy_saturates() {
    assert_eq!(D128s12::MAX.to_int_lossy(), i64::MAX);
    assert_eq!(D128s12::MIN.to_int_lossy(), i64::MIN);
}

// from_f64_lossy + to_f64_lossy

#[test]
fn from_f64_lossy_zero_is_zero() {
    assert_eq!(D128s12::from_f64_lossy(0.0), D128s12::ZERO);
}

#[test]
fn zero_to_f64_lossy_is_zero() {
    assert_eq!(D128s12::ZERO.to_f64_lossy(), 0.0);
}

#[test]
fn from_f64_lossy_one_is_one() {
    assert_eq!(D128s12::from_f64_lossy(1.0), D128s12::ONE);
}

#[test]
fn from_f64_lossy_negative() {
    assert_eq!(D128s12::from_f64_lossy(-1.0), -D128s12::ONE);
}

#[test]
fn from_f64_lossy_infinity_saturates_max() {
    assert_eq!(D128s12::from_f64_lossy(f64::INFINITY), D128s12::MAX);
}

#[test]
fn from_f64_lossy_neg_infinity_saturates_min() {
    assert_eq!(D128s12::from_f64_lossy(f64::NEG_INFINITY), D128s12::MIN);
}

#[test]
fn from_f64_lossy_nan_is_zero() {
    assert_eq!(D128s12::from_f64_lossy(f64::NAN), D128s12::ZERO);
}

#[test]
fn from_f64_lossy_finite_out_of_range_saturates() {
    assert_eq!(D128s12::from_f64_lossy(1e30), D128s12::MAX);
    assert_eq!(D128s12::from_f64_lossy(-1e30), D128s12::MIN);
}

// TryFrom<i128> / TryFrom<u128>

#[test]
fn try_from_i128_in_range_succeeds() {
    let v: D128s12 = 1_000_000_i128.try_into().expect("in-range fits");
    assert_eq!(v.to_bits(), 1_000_000 * 1_000_000_000_000);
}

#[test]
fn try_from_i128_overflow_returns_err() {
    let result: Result<D128s12, _> = i128::MAX.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

#[test]
fn try_from_u128_max_returns_err() {
    let result: Result<D128s12, _> = u128::MAX.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

// TryFrom<f64> / TryFrom<f32>

#[test]
fn try_from_f64_one_succeeds() {
    let v: D128s12 = 1.0_f64.try_into().expect("one fits");
    assert_eq!(v, D128s12::ONE);
}

#[test]
fn try_from_f64_nan_returns_err() {
    let result: Result<D128s12, _> = f64::NAN.try_into();
    assert_eq!(result, Err(ConvertError::NotFinite));
}

#[test]
fn try_from_f64_out_of_range_returns_err() {
    let result: Result<D128s12, _> = 1e30_f64.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

#[test]
fn try_from_f32_infinity_returns_err() {
    let result: Result<D128s12, _> = f32::INFINITY.try_into();
    assert_eq!(result, Err(ConvertError::NotFinite));
}

// Cross-scale sanity

#[test]
fn from_int_works_at_scale_6() {
    type D6 = D128<6>;
    let v: D6 = D6::from(1_000_i64);
    assert_eq!(v.to_bits(), 1_000_000_000); // 10^9
    assert_eq!(v.to_int_lossy(), 1_000);
}

#[test]
fn from_int_works_at_scale_0() {
    type D0 = D128<0>;
    let v: D0 = D0::from(42_i64);
    assert_eq!(v.to_bits(), 42);
    assert_eq!(v.to_int_lossy(), 42);
}
