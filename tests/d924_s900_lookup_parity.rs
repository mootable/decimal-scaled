//! Parity check: D924<900> Tang-lookup `ln_strict` vs composed-identity
//! round trips through `exp_strict`. The lookup kernel for `ln_strict`
//! at SCALE 895..=905 must agree with `exp_strict ∘ ln_strict` round
//! trip to within a small multiple of storage LSB.
//!
//! Only `ln_strict` is wired through the new deep-band lookup at this
//! width; exp / hyperbolics still route the canonical `wide_kernel`.
//! The composed `exp(ln(x)) ≈ x` identity therefore mixes both paths
//! and is a strong end-to-end correctness probe.

#![cfg(all(feature = "xx-wide", feature = "x-wide", feature = "wide", not(feature = "fast")))]

use decimal_scaled::D924;

type D = D924<900>;

fn from_int(n: i128) -> D {
    D::from_int(n)
}

#[track_caller]
fn agree_within_n_storage_lsb(label: &str, a: D, b: D, n_lsb: u128) {
    let diff = if a >= b { a - b } else { b - a };
    let lsb = D::EPSILON;
    let limit = D::from_int(n_lsb as i128) * lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn exp_ln_round_trip_at_s900_half() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(1.5)) D924<900>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s900_two() {
    let x = from_int(2);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(2)) D924<900>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s900_three() {
    let x = from_int(3);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(3)) D924<900>", round, x, 8);
}

#[test]
fn ln_lookup_at_one_is_zero_at_s900() {
    let one = from_int(1);
    let z = one.ln_strict();
    assert_eq!(z, D::ZERO, "ln(1) D924<900>: expected ZERO, got {z:?}");
}

#[test]
fn ln_lookup_band_lower_bound_s895() {
    let x = D924::<895>::from_int(3) / D924::<895>::from_int(2);
    let y = x.ln_strict();
    assert!(y < D924::<895>::from_int(1));
    assert!(y > D924::<895>::ZERO);
}

#[test]
fn ln_lookup_band_upper_bound_s905() {
    let x = D924::<905>::from_int(3) / D924::<905>::from_int(2);
    let y = x.ln_strict();
    assert!(y < D924::<905>::from_int(1));
    assert!(y > D924::<905>::ZERO);
}
