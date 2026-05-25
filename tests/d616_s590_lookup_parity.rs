//! Parity check: D616<590> Tang-lookup `ln_strict` vs composed-identity
//! round trips through `exp_strict`. The lookup kernel for `ln_strict`
//! at SCALE 585..=595 must agree with `exp_strict ∘ ln_strict` round
//! trip to within a small multiple of storage LSB.
//!
//! Only `ln_strict` is wired through the new deep-band lookup at this
//! width; exp / hyperbolics still route the canonical `wide_kernel`.
//! The composed `exp(ln(x)) ≈ x` identity therefore mixes both paths
//! and is a strong end-to-end correctness probe.

#![cfg(all(feature = "x-wide", not(feature = "fast")))]

use decimal_scaled::D616;

type D = D616<590>;

fn from_int(n: i128) -> D {
    D::try_from(n).unwrap()
}

#[track_caller]
fn agree_within_n_storage_lsb(label: &str, a: D, b: D, n_lsb: u128) {
    let diff = if a >= b { a - b } else { b - a };
    let lsb = D::EPSILON;
    let limit = D::try_from(n_lsb as i128).unwrap() * lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn exp_ln_round_trip_at_s590_half() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(1.5)) D616<590>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s590_two() {
    let x = from_int(2);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(2)) D616<590>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s590_three() {
    let x = from_int(3);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(3)) D616<590>", round, x, 8);
}

#[test]
fn ln_lookup_at_one_is_zero_at_s590() {
    let one = from_int(1);
    let z = one.ln_strict();
    assert_eq!(z, D::ZERO, "ln(1) D616<590>: expected ZERO, got {z:?}");
}

#[test]
fn ln_lookup_band_lower_bound_s585() {
    let x = D616::<585>::from(3) / D616::<585>::from(2);
    let y = x.ln_strict();
    assert!(y < D616::<585>::from(1));
    assert!(y > D616::<585>::ZERO);
}

#[test]
fn ln_lookup_band_upper_bound_s595() {
    let x = D616::<595>::from(3) / D616::<595>::from(2);
    let y = x.ln_strict();
    assert!(y < D616::<595>::from(1));
    assert!(y > D616::<595>::ZERO);
}
