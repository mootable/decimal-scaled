//! Parity check: D1232<615> Tang-lookup `ln_strict` vs the canonical
//! wide_kernel path. The lookup kernel at SCALE 610..=620 must agree
//! with the wide_kernel to within the same 2-LSB-of-storage tolerance
//! the wide tier already uses against the D38 cross-witness (see
//! `tests/d616_s308_lookup_parity.rs`).
//!
//! At D1232 the lookup band covers only `ln_strict` — `exp_strict`
//! routes the wide_kernel (the Tang exp surface was rejected at D616
//! and remains rejected at D1232), and the hyperbolic shells use the
//! macro-emitted `exp(-v) ≡ 1/exp(v)` identity but no Tang lookup.

#![cfg(all(feature = "xx-wide", not(feature = "fast")))]

use decimal_scaled::D1232;

type D = D1232<615>;

fn from_int(n: i128) -> D {
    D::try_from(n).unwrap()
}

#[track_caller]
fn agree_within_1_storage_lsb(label: &str, a: D, b: D) {
    let diff = if a >= b { a - b } else { b - a };
    // One storage LSB: two correctly-rounded paths may legitimately
    // disagree by at most 1 ULP at half-ULP-tie boundaries (one rounds
    // up, the other rounds down). Tighter than 1 LSB would false-positive
    // on those legitimate cases.
    let one = D::try_from(1).unwrap();
    let lsb = one / D::try_from(10).unwrap().pow(615);
    let limit = lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn ln_lookup_matches_wide_kernel_at_s615() {
    // Public _strict goes through the lookup kernel at SCALE 615; the
    // `*_strict_with` shell collapses to wide_kernel inside the policy.
    let half = from_int(1) / from_int(2);
    let one_p_half = from_int(1) + half; // 1.5

    let lookup = one_p_half.ln_strict();
    let kernel = one_p_half.ln_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_1_storage_lsb("ln(1.5) D1232<615>", lookup, kernel);
}

// Identity probe: exp(ln(x)) ≈ x. exp uses the canonical wide_kernel
// (no Tang surface at D1232), ln uses the Tang lookup; agreement
// confirms the lookup's 0.5-ULP contract.
#[test]
fn exp_ln_round_trip_at_s615() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_1_storage_lsb("exp(ln(1.5)) D1232<615>", round, x);
}

// Boundary probe: ln(2) at the table boundary (m = 2, k = 1).
#[test]
fn ln_two_at_s615() {
    let two = from_int(2);
    let lookup = two.ln_strict();
    let kernel = two.ln_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_1_storage_lsb("ln(2) D1232<615>", lookup, kernel);
}

// Boundary probe: ln(1) = 0 short-circuit.
#[test]
fn ln_one_at_s615() {
    let one = from_int(1);
    let r = one.ln_strict();
    assert_eq!(r, D::ZERO, "ln(1) D1232<615> must be exactly zero");
}
