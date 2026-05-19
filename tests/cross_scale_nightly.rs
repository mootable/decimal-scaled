//! Nightly-gated tests for the auto-inferred `cross::mul/add/sub/div/rem`
//! free functions. Requires the `cross-scale-ops` feature.

#![cfg(feature = "cross-scale-ops")]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use decimal_scaled::{D38, cross};

#[test]
fn cross_mul_same_width_picks_higher_scale() {
    let a: D38<6> = D38::<6>::from_int(7);
    let b: D38<12> = D38::<12>::from_int(11);
    // Output type is inferred: D38<max(6,12)> = D38<12>.
    let c = cross::mul(a, b);
    let expected: D38<12> = D38::<12>::from_int(77);
    assert_eq!(c, expected);
}

#[test]
fn cross_add_same_width() {
    let a: D38<6> = D38::<6>::from_int(5);
    let b: D38<12> = D38::<12>::from_int(7);
    let c = cross::add(a, b);
    let expected: D38<12> = D38::<12>::from_int(12);
    assert_eq!(c, expected);
}

#[test]
fn cross_sub_same_width() {
    let a: D38<12> = D38::<12>::from_int(10);
    let b: D38<6> = D38::<6>::from_int(3);
    let c = cross::sub(a, b);
    let expected: D38<12> = D38::<12>::from_int(7);
    assert_eq!(c, expected);
}

#[test]
fn cross_div_same_width() {
    let a: D38<12> = D38::<12>::from_int(20);
    let b: D38<6> = D38::<6>::from_int(4);
    let c = cross::div(a, b);
    let expected: D38<12> = D38::<12>::from_int(5);
    assert_eq!(c, expected);
}

#[test]
fn cross_rem_same_width() {
    let a: D38<12> = D38::<12>::from_int(17);
    let b: D38<6> = D38::<6>::from_int(5);
    let c = cross::rem(a, b);
    let expected: D38<12> = D38::<12>::from_int(2);
    assert_eq!(c, expected);
}

#[test]
fn cross_mul_equal_scales_keeps_scale() {
    let a: D38<6> = D38::<6>::from_int(3);
    let b: D38<6> = D38::<6>::from_int(4);
    let c = cross::mul(a, b);
    let expected: D38<6> = D38::<6>::from_int(12);
    assert_eq!(c, expected);
}

#[test]
fn cross_max_const_picks_higher() {
    assert_eq!(cross::max_const(6, 12), 12);
    assert_eq!(cross::max_const(20, 5), 20);
    assert_eq!(cross::max_const(7, 7), 7);
}
