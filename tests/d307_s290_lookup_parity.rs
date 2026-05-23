//! Parity check: D307<290> Tang-lookup `ln_strict` vs composed-identity
//! round trips through `exp_strict`. The lookup kernel for `ln_strict`
//! at SCALE 285..=295 must agree with `exp_strict ∘ ln_strict` round
//! trip to within a small multiple of storage LSB.
//!
//! Only `ln_strict` is wired through the lookup at this width; exp /
//! hyperbolics still route the canonical `wide_kernel`. The composed
//! `exp(ln(x)) ≈ x` identity therefore mixes both paths and is a strong
//! end-to-end correctness probe: lookup error feeds `exp_strict`'s
//! reverse mapping and must come back within a few storage LSBs.

#![cfg(all(feature = "x-wide", not(feature = "fast")))]

use decimal_scaled::D307;

type D = D307<290>;

fn from_int(n: i128) -> D {
    D::try_from(n).unwrap()
}

#[track_caller]
fn agree_within_n_storage_lsb(label: &str, a: D, b: D, n_lsb: u128) {
    let diff = if a >= b { a - b } else { b - a };
    // EPSILON = one storage LSB (the smallest representable positive
    // value at this `(width, SCALE)`). Avoids the `10.pow(SCALE)`
    // overflow that bites deep-band tests where SCALE × 2 exceeds the
    // storage width's representable range.
    let lsb = D::EPSILON;
    let limit = D::try_from(n_lsb as i128).unwrap() * lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn exp_ln_round_trip_at_s290_half() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(1.5)) D307<290>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s290_two() {
    let x = from_int(2);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(2)) D307<290>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s290_three() {
    let x = from_int(3);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(3)) D307<290>", round, x, 8);
}

#[test]
fn ln_lookup_at_one_is_zero_at_s290() {
    let one = from_int(1);
    let z = one.ln_strict();
    assert_eq!(z, D::ZERO, "ln(1) D307<290>: expected ZERO, got {z:?}");
}

#[test]
fn ln_lookup_band_lower_bound_s285() {
    let x = D307::<285>::try_from(3).unwrap() / D307::<285>::try_from(2).unwrap();
    let y = x.ln_strict();
    assert!(y < D307::<285>::try_from(1).unwrap());
    assert!(y > D307::<285>::ZERO);
}

#[test]
fn ln_lookup_band_upper_bound_s295() {
    let x = D307::<295>::try_from(3).unwrap() / D307::<295>::try_from(2).unwrap();
    let y = x.ln_strict();
    assert!(y < D307::<295>::try_from(1).unwrap());
    assert!(y > D307::<295>::ZERO);
}
