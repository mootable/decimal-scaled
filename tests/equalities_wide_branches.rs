//! Coverage for the remaining `macros/equalities.rs` branches: wide-arm
//! reciprocal directions (`prim == D76`) and the `to_*_checked()` `None`
//! branches for values that don't fit a primitive.

#![cfg(feature = "wide")]

use decimal_scaled::{D38, D76};
type D76_2 = D76<2>;
type D38_2 = D38<2>;

#[test]
fn reciprocal_signed_int_eq() {
    let v: D76_2 = D38_2::from_int(42).into();
    assert!(42_i32 == v);
    assert!(42_i64 == v);
    assert!(42_i128 == v);
    // Fractional vs primitive — false even from the primitive side.
    let frac: D76_2 = D38_2::from_bits(4201).into();
    assert!(!(42_i32 == frac));
    assert!(!(42_i64 == frac));
}

#[test]
fn reciprocal_unsigned_int_eq() {
    let v: D76_2 = D38_2::from_int(42).into();
    assert!(42_u8 == v);
    assert!(42_u16 == v);
    assert!(42_u32 == v);
    assert!(42_u64 == v);
    assert!(42_u128 == v);
    // Negative D76 vs unsigned — false from primitive side.
    let neg: D76_2 = D38_2::from_int(-1).into();
    assert!(!(0_u32 == neg));
}

#[test]
fn wide_i128_quotient_out_of_range_is_false() {
    // A D76 value whose integer quotient exceeds i128 should never equal
    // any i128.
    type D76_0 = D76<0>;
    let big = D76_0::MAX; // exceeds i128
    assert!(!(big == 0_i32));
    assert!(!(big == 0_i64));
    assert!(!(big == 0_i128));
    assert!(!(big == 0_u8));
    assert!(!(big == 0_u16));
    assert!(!(big == 0_u32));
    assert!(!(big == 0_u64));
    assert!(!(big == 0_u128));
    // Negative side
    let neg_big = D76_0::MIN;
    assert!(!(neg_big == 0_i64));
    assert!(!(neg_big == 0_i128));
}

#[test]
fn wide_fractional_vs_i128_is_false() {
    let frac: D76_2 = D38_2::from_bits(123).into(); // 0.... at S=2
    assert!(!(frac == 0_i128));
}
