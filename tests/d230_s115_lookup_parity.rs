//! Parity check: D230<115> Tang-lookup `ln_strict` vs composed-identity
//! round trips through `exp_strict`. The lookup kernel for `ln_strict`
//! at SCALE 110..=120 must agree with `exp_strict ∘ ln_strict` round
//! trip to within a small multiple of storage LSB.
//!
//! Only `ln_strict` is wired through the lookup at this width; exp /
//! hyperbolics still route the canonical `wide_kernel`. The composed
//! `exp(ln(x)) ≈ x` identity therefore mixes both paths and is a strong
//! end-to-end correctness probe: lookup error feeds `exp_strict`'s
//! reverse mapping and must come back within a few storage LSBs.
//!
//! NOTE: `ln_strict_agm` is documented to drop to `~p/2` bits beyond
//! `w ~ 30`, so at D230's working width AGM is the *lower*-accuracy
//! kernel — not a useful cross-witness. The identity round trip is the
//! correct probe.

#![cfg(all(feature = "wide", not(feature = "fast")))]

use decimal_scaled::D230;

type D = D230<115>;

fn from_int(n: i128) -> D {
    D::from_int(n)
}

#[track_caller]
fn agree_within_n_storage_lsb(label: &str, a: D, b: D, n_lsb: u128) {
    let diff = if a >= b { a - b } else { b - a };
    let one = D::from_int(1);
    let lsb = one / D::from_int(10).pow(115);
    let limit = D::from_int(n_lsb as i128) * lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn exp_ln_round_trip_at_s115_half() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(1.5)) D230<115>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s115_two() {
    let x = from_int(2);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(2)) D230<115>", round, x, 8);
}

#[test]
fn exp_ln_round_trip_at_s115_three() {
    let x = from_int(3);
    let round = x.ln_strict().exp_strict();
    agree_within_n_storage_lsb("exp(ln(3)) D230<115>", round, x, 8);
}

#[test]
fn ln_lookup_at_one_is_zero_at_s115() {
    // ln(1) = 0 must hold exactly through the Stage-1 short-circuit.
    let one = from_int(1);
    let z = one.ln_strict();
    assert_eq!(z, D::ZERO, "ln(1) D230<115>: expected ZERO, got {z:?}");
}

#[test]
fn ln_lookup_band_lower_bound_s110() {
    // Confirms SCALE = 110 enters the lookup band (no panic / no overflow).
    let x = D230::<110>::from_int(3) / D230::<110>::from_int(2);
    let y = x.ln_strict();
    assert!(y < D230::<110>::from_int(1));
    assert!(y > D230::<110>::ZERO);
}

#[test]
fn ln_lookup_band_upper_bound_s120() {
    // Confirms SCALE = 120 enters the lookup band (no panic / no overflow).
    let x = D230::<120>::from_int(3) / D230::<120>::from_int(2);
    let y = x.ln_strict();
    assert!(y < D230::<120>::from_int(1));
    assert!(y > D230::<120>::ZERO);
}
