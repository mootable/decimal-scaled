//! Parity check: D616<308> Tang-lookup vs the canonical wide_kernel
//! path. The lookup kernels for `ln_strict` / `exp_strict` /
//! `sinh_strict` / `cosh_strict` / `tanh_strict` at SCALE 300..=315
//! must agree with the wide_kernel to within the same 1-LSB-of-storage
//! tolerance the wide tier already uses against the D38 cross-witness
//! (see `tests/precision_wide_baseline.rs`).
//!
//! This file exercises the public `*_strict` surface at SCALE 308
//! (the centre of the lookup band) and confirms the result matches
//! a SCALE-308 reference value computed via the inherent
//! `*_strict_with` shell, which routes through the wide_kernel for the
//! `_with` collapsed-strict variant *outside* the lookup band. Since
//! the public `_strict` and the inherent `_strict_with` go through
//! different dispatch arms when the lookup kernel is active, the
//! agreement is meaningful.

#![cfg(all(feature = "x-wide", not(feature = "fast")))]

use decimal_scaled::D616;

type D = D616<308>;

fn from_int(n: i128) -> D {
    D::from_int(n)
}

#[track_caller]
fn agree_within_2_storage_lsb(label: &str, a: D, b: D) {
    let diff = if a >= b { a - b } else { b - a };
    // Two storage LSBs of slack: lookup error budget ~36 LSB-of-w with
    // GUARD_NARROW = 10 lands well below 1 storage LSB, but the
    // wide_kernel uses GUARD = 30 so the half-to-even rounding tie may
    // fall on a different side. 2 storage LSBs is the wide-tier x-witness
    // tolerance.
    let two = D::from_int(2);
    let one = D::from_int(1);
    let lsb = one / D::from_int(10).pow(308);
    // diff <= 2*lsb
    let limit = two * lsb;
    assert!(
        diff <= limit,
        "{label}: |a - b| = {diff:?}, limit = {limit:?}, a = {a:?}, b = {b:?}",
    );
}

#[test]
fn ln_lookup_matches_wide_kernel_at_s308() {
    // Public _strict goes through the lookup kernel at SCALE 308. The
    // `*_strict_with` shell uses wide_kernel directly via the macro
    // since the policy collapses to it when working_digits is ignored.
    // Both should agree.
    let half = from_int(1) / from_int(2);
    let one_p_half = from_int(1) + half; // 1.5

    let lookup = one_p_half.ln_strict();
    let kernel = one_p_half.ln_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_2_storage_lsb("ln(1.5) D616<308>", lookup, kernel);
}

#[test]
fn exp_lookup_matches_wide_kernel_at_s308() {
    let half = from_int(1) / from_int(2);

    let lookup = half.exp_strict();
    let kernel = half.exp_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_2_storage_lsb("exp(0.5) D616<308>", lookup, kernel);
}

#[test]
fn sinh_lookup_matches_wide_kernel_at_s308() {
    let half = from_int(1) / from_int(2);

    let lookup = half.sinh_strict();
    let kernel = half.sinh_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_2_storage_lsb("sinh(0.5) D616<308>", lookup, kernel);
}

#[test]
fn cosh_lookup_matches_wide_kernel_at_s308() {
    let half = from_int(1) / from_int(2);

    let lookup = half.cosh_strict();
    let kernel = half.cosh_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_2_storage_lsb("cosh(0.5) D616<308>", lookup, kernel);
}

#[test]
fn tanh_lookup_matches_wide_kernel_at_s308() {
    let half = from_int(1) / from_int(2);

    let lookup = half.tanh_strict();
    let kernel = half.tanh_strict_with(decimal_scaled::RoundingMode::HalfToEven);
    agree_within_2_storage_lsb("tanh(0.5) D616<308>", lookup, kernel);
}

// Identity probes: ln(exp(x)) ≈ x and exp(ln(x)) ≈ x. Composing both
// the lookup paths means errors must cancel; the round-trip should
// agree to within a small multiple of storage LSB.
#[test]
fn ln_exp_round_trip_at_s308() {
    let x = from_int(1) / from_int(4); // 0.25, |x| < ln(2)/2 well inside lookup
    let round = x.exp_strict().ln_strict();
    agree_within_2_storage_lsb("ln(exp(0.25)) D616<308>", round, x);
}

#[test]
fn exp_ln_round_trip_at_s308() {
    let x = from_int(3) / from_int(2); // 1.5
    let round = x.ln_strict().exp_strict();
    agree_within_2_storage_lsb("exp(ln(1.5)) D616<308>", round, x);
}
