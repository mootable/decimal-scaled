//! Sanity-witness the D462 SCALE 225..=235 narrow-GUARD Tang lookup
//! slot kernels. The contract is checked at the math-identity level:
//!
//! - `exp(ln(x)) == x` to within ~5 storage ULPs of D462<230>.
//! - `sin(x)^2 + cos(x)^2 == 1` to the same.
//! - `cosh(x)^2 - sinh(x)^2 == 1`.
//! - `tan(atan(x)) == x` for small x.
//!
//! 0.5-ULP correctness against the canonical path is covered by the
//! shared `precision_wide_baseline.rs` / `wide_strict_transcendentals.rs`
//! suites which run on D76 / D153 / D307 — for D462, this test only
//! confirms the new lookup arm produces mathematically self-consistent
//! values. Only runs under `x-wide` and the default-rounding build
//! (`HalfToEven`).

#![cfg(all(
    feature = "x-wide",
    not(feature = "fast"),
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]

use decimal_scaled::D462;

type D = D462<230>;

/// Allowed storage-bit drift after composing two correctly-rounded
/// transcendentals — each contributes ≤ 0.5 ULP, plus rounding noise
/// from the comparison arithmetic. Five LSB is generous and matches
/// the wider-tier identity-witness budget used elsewhere.
const IDENTITY_TOL_LSB: u32 = 5;

fn assert_close(label: &str, a: D, b: D) {
    let diff = if a >= b { a - b } else { b - a };
    // Tight identity-witness tolerance: IDENTITY_TOL_LSB storage LSBs.
    // Each composed correctly-rounded transcendental contributes ≤ 0.5
    // ULP, plus rounding noise from the comparison arithmetic. Five LSB
    // is the documented identity-witness budget for the wider tiers.
    // Build `IDENTITY_TOL_LSB * 10^-230` as `IDENTITY_TOL_LSB / 10^230`.
    let ten = D::from(10);
    let mut bound = ten;
    for _ in 1..230 {
        bound *= ten;
    }
    let lsb = D::ONE / bound;
    let tol = D::try_from(IDENTITY_TOL_LSB as i128).unwrap() * lsb;
    assert!(
        diff <= tol,
        "{label}: |a - b| = {diff:?} exceeds {IDENTITY_TOL_LSB} storage LSB at SCALE 230"
    );
}

#[test]
fn ln_exp_inverse_at_d462_s230() {
    // exp(ln(1.5)) == 1.5
    let x: D = D::from(3) / D::from(2);
    let y = x.ln_strict().exp_strict();
    assert_close("exp(ln(1.5))", y, x);

    // exp(ln(3)) == 3
    let three = D::from(3);
    let y3 = three.ln_strict().exp_strict();
    assert_close("exp(ln(3))", y3, three);
}

#[test]
fn pythagorean_identity_at_d462_s230() {
    let x: D = D::from(3) / D::from(2); // 1.5
    let s = x.sin_strict();
    let c = x.cos_strict();
    let s2 = s * s;
    let c2 = c * c;
    let sum = s2 + c2;
    assert_close("sin^2 + cos^2 = 1", sum, D::ONE);
}

#[test]
fn hyperbolic_identity_at_d462_s230() {
    // cosh(x)^2 - sinh(x)^2 = 1
    let x: D = D::from(1) / D::from(2); // 0.5
    let sh = x.sinh_strict();
    let ch = x.cosh_strict();
    let diff = ch * ch - sh * sh;
    assert_close("cosh^2 - sinh^2 = 1", diff, D::ONE);
}

#[test]
fn tan_atan_inverse_at_d462_s230() {
    // tan(atan(0.5)) == 0.5
    let x: D = D::from(1) / D::from(2);
    let y = x.atan_strict().tan_strict();
    assert_close("tan(atan(0.5))", y, x);
}

#[test]
fn ln_of_one_is_zero_d462_s230() {
    let one = D::ONE;
    assert_eq!(one.ln_strict(), D::ZERO);
}

#[test]
fn exp_of_zero_is_one_d462_s230() {
    let zero = D::ZERO;
    assert_eq!(zero.exp_strict(), D::ONE);
}

#[test]
fn sin_of_zero_is_zero_d462_s230() {
    let zero = D::ZERO;
    assert_eq!(zero.sin_strict(), D::ZERO);
    assert_eq!(zero.cos_strict(), D::ONE);
}
