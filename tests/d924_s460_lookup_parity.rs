//! Parity check: D924<460> Tang-lookup ln vs composed-identity round
//! trips through `exp_strict`. The lookup kernel for `ln_strict` at
//! SCALE 455..=465 must agree with the `exp_strict ∘ ln_strict` round
//! trip to within the wide-tier x-witness tolerance (a small multiple
//! of storage LSB).
//!
//! Only `ln_strict` is wired through the lookup at this width; exp /
//! hyperbolics still route the canonical `wide_kernel` (Tang exp loses
//! at D462+ per wave 3 measurements, and the D924 working width
//! Int12288 only widens that gap). The composed `exp(ln(x)) ≈ x`
//! identity therefore mixes both paths and is a strong end-to-end
//! correctness probe: lookup error feeds exp_strict's reverse mapping
//! and must come back within a few storage LSBs.
//!
//! NOTE: `ln_strict_agm` is documented to drop to `~p/2` bits beyond
//! `w ~ 30`, so at D924's working width AGM is the *lower*-accuracy
//! kernel — not a useful cross-witness. The identity round trip is the
//! correct probe.

#![cfg(all(
    feature = "xx-wide",
    feature = "x-wide",
    feature = "wide",
    not(feature = "fast")
))]

use decimal_scaled::D924;

type D = D924<460>;

fn from_int(n: i128) -> D {
    D::try_from(n).unwrap()
}

#[track_caller]
fn agree_within_n_storage_lsb(label: &str, a: D, b: D, n_lsb: u128) {
    let diff = if a >= b { a - b } else { b - a };
    let one = D::try_from(1).unwrap();
    let lsb = one / D::try_from(10).unwrap().pow(460);
    let limit = D::try_from(n_lsb as i128).unwrap() * lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn exp_ln_round_trip_at_s460_half() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(1.5)) D924<460>", round, x, 1);
}

#[test]
fn exp_ln_round_trip_at_s460_two() {
    let x = from_int(2);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(2)) D924<460>", round, x, 1);
}

#[test]
fn exp_ln_round_trip_at_s460_three() {
    let x = from_int(3);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(3)) D924<460>", round, x, 1);
}

#[test]
fn ln_lookup_at_one_is_zero_at_s460() {
    // ln(1) = 0 must hold exactly through the Stage-1 short-circuit.
    let one = from_int(1);
    let z = one.ln_strict();
    assert_eq!(z, D::ZERO, "ln(1) D924<460>: expected ZERO, got {z:?}");
}

#[test]
fn ln_lookup_band_lower_bound_s455() {
    // Confirms SCALE = 455 enters the lookup band (no panic / no overflow).
    let x = D924::<455>::try_from(3).unwrap() / D924::<455>::try_from(2).unwrap();
    let y = x.ln_strict();
    assert!(y < D924::<455>::try_from(1).unwrap());
    assert!(y > D924::<455>::ZERO);
}

#[test]
fn ln_lookup_band_upper_bound_s465() {
    // Confirms SCALE = 465 enters the lookup band (no panic / no overflow).
    let x = D924::<465>::try_from(3).unwrap() / D924::<465>::try_from(2).unwrap();
    let y = x.ln_strict();
    assert!(y < D924::<465>::try_from(1).unwrap());
    assert!(y > D924::<465>::ZERO);
}
