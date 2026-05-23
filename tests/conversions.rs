//! Integration tests for the conversion surface
//! (From<integer> / TryFrom<i128|u128|f32|f64> / from_int /
//! from_i32 / to_int / from_f64 / to_f64 /
//! to_f32).
//!
//! Bodies live in src/macros/conversions.rs and float_bridge.rs;
//! these tests exercise the resulting public API for D38 specifically.

use decimal_scaled::{ConvertError, D38, D38s12};

// --- widen / narrow ergonomic methods -------------------------------

#[test]
fn widen_narrow_one_tier_hop_narrow_arm() {
    use decimal_scaled::{D18s6, D38s6};
    let a = D18s6::from_int(123);
    let b: D38s6 = a.widen(); // D18 → D38
    assert_eq!(b.to_bits(), i128::from(a.to_bits()));
    let c: D18s6 = b.narrow().unwrap(); // D38 → D18
    assert_eq!(c.to_bits(), a.to_bits());
}

#[cfg(feature = "wide")]
#[test]
fn widen_narrow_into_wide_tier() {
    use decimal_scaled::{D38s12, D57};
    // After the 0.3 widen-chain rework, D38.widen() steps to D57
    // (the immediate next tier in the ladder) instead of jumping
    // straight to D76. The .narrow() symmetric is D57 -> D38.
    let a = D38s12::from_int(1_000_000);
    let b: D57<12> = a.widen();
    let back = b.narrow().unwrap();
    assert_eq!(back, a);
}

// from_int / from_i32 -- foundation wrappers around From<iN>

#[test]
fn from_int_zero_is_zero() {
    assert_eq!(D38s12::from_int(0), D38s12::ZERO);
}

#[test]
fn from_i32_zero_is_zero() {
    assert_eq!(D38s12::from_i32(0), D38s12::ZERO);
}

#[test]
fn from_int_one_is_one() {
    assert_eq!(D38s12::from_int(1), D38s12::ONE);
}

#[test]
fn from_i32_one_is_one() {
    assert_eq!(D38s12::from_i32(1), D38s12::ONE);
}

#[test]
fn from_int_negative() {
    assert_eq!(D38s12::from_int(-1), -D38s12::ONE);
    assert_eq!(D38s12::from_int(-42).to_bits(), -42_000_000_000_000_i128);
}

// Lossless From<iN> / From<uN> -- bit-exact scaling

#[test]
fn from_i8_scales_correctly() {
    assert_eq!(D38s12::from(0_i8).to_bits(), 0);
    assert_eq!(D38s12::from(1_i8).to_bits(), 1_000_000_000_000);
    assert_eq!(D38s12::from(-1_i8).to_bits(), -1_000_000_000_000);
    assert_eq!(D38s12::from(i8::MAX).to_bits(), 127_000_000_000_000);
    assert_eq!(D38s12::from(i8::MIN).to_bits(), -128_000_000_000_000);
}

#[test]
fn from_i64_scales_correctly() {
    assert_eq!(D38s12::from(0_i64).to_bits(), 0);
    assert_eq!(
        D38s12::from(i64::MAX).to_bits(),
        (i64::MAX as i128) * 1_000_000_000_000
    );
    assert_eq!(
        D38s12::from(i64::MIN).to_bits(),
        (i64::MIN as i128) * 1_000_000_000_000
    );
}

#[test]
fn from_u64_at_boundary_is_lossless() {
    let v = D38s12::from(u64::MAX);
    assert_eq!(v.to_bits(), (u64::MAX as i128) * 1_000_000_000_000);
}

// to_int

#[test]
fn to_int_lossy_default_rounds_half_to_even() {
    // 2.5 with HalfToEven default -> 2 (even neighbour).
    assert_eq!(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_500_000_000_000) as i128).unwrap()).to_int(), 2);
    // 3.5 with HalfToEven -> 4 (even).
    assert_eq!(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((3_500_000_000_000) as i128).unwrap()).to_int(), 4);
}

#[test]
fn to_int_lossy_saturates() {
    assert_eq!(D38s12::MAX.to_int(), i64::MAX);
    assert_eq!(D38s12::MIN.to_int(), i64::MIN);
}

// from_f64 + to_f64

#[test]
fn from_f64_lossy_zero_is_zero() {
    assert_eq!(D38s12::from_f64(0.0), D38s12::ZERO);
}

#[test]
fn zero_to_f64_lossy_is_zero() {
    assert_eq!(D38s12::ZERO.to_f64(), 0.0);
}

#[test]
fn from_f64_lossy_one_is_one() {
    assert_eq!(D38s12::from_f64(1.0), D38s12::ONE);
}

#[test]
fn from_f64_lossy_negative() {
    assert_eq!(D38s12::from_f64(-1.0), -D38s12::ONE);
}

#[test]
fn from_f64_lossy_infinity_saturates_max() {
    assert_eq!(D38s12::from_f64(f64::INFINITY), D38s12::MAX);
}

#[test]
fn from_f64_lossy_neg_infinity_saturates_min() {
    assert_eq!(D38s12::from_f64(f64::NEG_INFINITY), D38s12::MIN);
}

#[test]
fn from_f64_lossy_nan_is_zero() {
    assert_eq!(D38s12::from_f64(f64::NAN), D38s12::ZERO);
}

#[test]
fn from_f64_lossy_finite_out_of_range_saturates() {
    assert_eq!(D38s12::from_f64(1e30), D38s12::MAX);
    assert_eq!(D38s12::from_f64(-1e30), D38s12::MIN);
}

// TryFrom<i128> / TryFrom<u128>

#[test]
fn try_from_i128_in_range_succeeds() {
    let v: D38s12 = 1_000_000_i128.try_into().expect("in-range fits");
    assert_eq!(v.to_bits(), 1_000_000 * 1_000_000_000_000);
}

#[test]
fn try_from_i128_overflow_returns_err() {
    let result: Result<D38s12, _> = i128::MAX.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

#[test]
fn try_from_u128_max_returns_err() {
    let result: Result<D38s12, _> = u128::MAX.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

// TryFrom<f64> / TryFrom<f32>

#[test]
fn try_from_f64_one_succeeds() {
    let v: D38s12 = 1.0_f64.try_into().expect("one fits");
    assert_eq!(v, D38s12::ONE);
}

#[test]
fn try_from_f64_nan_returns_err() {
    let result: Result<D38s12, _> = f64::NAN.try_into();
    assert_eq!(result, Err(ConvertError::NotFinite));
}

#[test]
fn try_from_f64_out_of_range_returns_err() {
    let result: Result<D38s12, _> = 1e30_f64.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

#[test]
fn try_from_f32_infinity_returns_err() {
    let result: Result<D38s12, _> = f32::INFINITY.try_into();
    assert_eq!(result, Err(ConvertError::NotFinite));
}

// Cross-scale sanity

#[test]
fn from_int_works_at_scale_6() {
    type D6 = D38<6>;
    let v: D6 = D6::from(1_000_i64);
    assert_eq!(v.to_bits(), 1_000_000_000); // 10^9
    assert_eq!(v.to_int(), 1_000);
}

#[test]
fn from_int_works_at_scale_0() {
    type D0 = D38<0>;
    let v: D0 = D0::from(42_i64);
    assert_eq!(v.to_bits(), 42);
    assert_eq!(v.to_int(), 42);
}

// TryFrom<i64> / TryFrom<u64> for D18 (64-bit storage tier).
//
// D18 cannot offer an infallible `From<i64>` / `From<u64>` because
// `value * 10^SCALE` may overflow the i64 storage (and `u64` above
// `i64::MAX` overflows even at SCALE 0), so the standard surface is
// `TryFrom`. Wider tiers (D38+) keep their infallible `From<i64>`.

#[test]
fn try_from_i64_d18_in_range() {
    use decimal_scaled::D18;
    let v: D18<2> = 100_i64.try_into().unwrap();
    assert_eq!(v.to_bits(), 10_000);
    // SCALE 0: identity-ish, the i64 stores directly.
    let v: D18<0> = (-7_i64).try_into().unwrap();
    assert_eq!(v.to_bits(), -7);
}

#[test]
fn try_from_i64_d18_overflow_returns_err() {
    use decimal_scaled::D18;
    // i64::MAX scaled by 10^2 overflows the i64 storage.
    let result: Result<D18<2>, _> = i64::MAX.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

#[test]
fn try_from_u64_d18_in_range() {
    use decimal_scaled::D18;
    let v: D18<2> = 100_u64.try_into().unwrap();
    assert_eq!(v.to_bits(), 10_000);
}

#[test]
fn try_from_u64_d18_above_i64_max_returns_err() {
    use decimal_scaled::D18;
    // A u64 above i64::MAX cannot fit signed storage even at SCALE 0.
    let result: Result<D18<0>, _> = u64::MAX.try_into();
    assert_eq!(result, Err(ConvertError::Overflow));
}

// TryFrom<f64> rounds to scale via the crate-default RoundingMode
// (HalfToEven unless a `rounding-*` feature overrides it). Under the
// default build, a value whose scaled form lands on a .5 boundary
// rounds to even.

#[cfg(feature = "std")]
#[cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
)))]
#[test]
fn try_from_f64_rounds_half_to_even_default() {
    // 0.125 at SCALE 2 = 12.5 scaled units; HalfToEven -> 12.
    let v: D38<2> = 0.125_f64.try_into().unwrap();
    assert_eq!(v.to_bits(), 12);
    // 0.135 at SCALE 2 = 13.5 scaled units; HalfToEven -> 14.
    // (0.135 is not exactly representable in f64; it is slightly above
    // 0.135, so this also confirms the value is rounded, not truncated.)
    let v: D38<2> = 0.135_f64.try_into().unwrap();
    assert_eq!(v.to_bits(), 14);
    // A clearly-fractional value rounds rather than truncating: 1.6 at
    // SCALE 0 = 1.6 scaled units -> 2 (truncation would give 1).
    let v: D38<0> = 1.6_f64.try_into().unwrap();
    assert_eq!(v.to_bits(), 2);
}
