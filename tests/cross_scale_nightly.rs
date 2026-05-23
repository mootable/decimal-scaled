//! Nightly-gated tests for the auto-inferred `cross::mul/add/sub/div/rem`
//! free functions. Requires the `cross-scale-ops` feature.

#![cfg(feature = "cross-scale-ops")]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use decimal_scaled::{D38, cross};

#[test]
fn cross_mul_same_width_picks_higher_scale() {
    let a: D38<6> = D38::<6>::try_from(7).unwrap();
    let b: D38<12> = D38::<12>::try_from(11).unwrap();
    // Output type is inferred: D38<max(6,12)> = D38<12>.
    let c = cross::mul(a, b);
    let expected: D38<12> = D38::<12>::try_from(77).unwrap();
    assert_eq!(c, expected);
}

#[test]
fn cross_add_same_width() {
    let a: D38<6> = D38::<6>::try_from(5).unwrap();
    let b: D38<12> = D38::<12>::try_from(7).unwrap();
    let c = cross::add(a, b);
    let expected: D38<12> = D38::<12>::try_from(12).unwrap();
    assert_eq!(c, expected);
}

#[test]
fn cross_sub_same_width() {
    let a: D38<12> = D38::<12>::try_from(10).unwrap();
    let b: D38<6> = D38::<6>::try_from(3).unwrap();
    let c = cross::sub(a, b);
    let expected: D38<12> = D38::<12>::try_from(7).unwrap();
    assert_eq!(c, expected);
}

#[test]
fn cross_div_same_width() {
    let a: D38<12> = D38::<12>::try_from(20).unwrap();
    let b: D38<6> = D38::<6>::try_from(4).unwrap();
    let c = cross::div(a, b);
    let expected: D38<12> = D38::<12>::try_from(5).unwrap();
    assert_eq!(c, expected);
}

#[test]
fn cross_rem_same_width() {
    let a: D38<12> = D38::<12>::try_from(17).unwrap();
    let b: D38<6> = D38::<6>::try_from(5).unwrap();
    let c = cross::rem(a, b);
    let expected: D38<12> = D38::<12>::try_from(2).unwrap();
    assert_eq!(c, expected);
}

#[test]
fn cross_mul_equal_scales_keeps_scale() {
    let a: D38<6> = D38::<6>::try_from(3).unwrap();
    let b: D38<6> = D38::<6>::try_from(4).unwrap();
    let c = cross::mul(a, b);
    let expected: D38<6> = D38::<6>::try_from(12).unwrap();
    assert_eq!(c, expected);
}

#[test]
fn cross_max_const_picks_higher() {
    assert_eq!(cross::max_const(6, 12), 12);
    assert_eq!(cross::max_const(20, 5), 20);
    assert_eq!(cross::max_const(7, 7), 7);
}
