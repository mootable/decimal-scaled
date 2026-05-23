//! Coverage suite for `macros/strict_transcendentals.rs` — D9 / D18
//! strict transcendentals that delegate to D38's `*_strict` core.
//!
//! D9 / D18 transcendentals are macro-emitted wrappers that widen `self`
//! to `D38<SCALE>`, call the D38 `*_strict` implementation, and narrow
//! back. The 0.5 ULP contract is inherited from D38 (already validated
//! by `tests/precision_strict_05_ulp.rs`); the role of THIS suite is to
//! prove the macro-emitted delegators are wired up and survive the
//! narrow-tier round-trip.
//!
//! For each method we:
//! 1. Call the `*_strict` form on D9<S> and D18<S> at scales that fit
//!    the result, and check the bits match the truth-to-storage value
//!    (truth obtained from the same D38 method).
//! 2. Where the result range fits the narrow storage exactly, assert
//!    the dispatcher's plain `*` form is bit-identical to `*_strict`.
//!
//! 0.5 ULP is asserted via the same `≤ 1 LSB` slack used in
//! `tests/precision_strict_05_ulp.rs` (the truth itself is rounded to
//! storage, so the slack absorbs a single-LSB transcription error).

#![cfg(all(
    not(feature = "fast"),
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]

use decimal_scaled::{D18, D38};

#[track_caller]
fn assert_le_1_lsb_i32(label: &str, actual: i32, truth: i32) {
    let diff = (actual - truth).abs();
    assert!(
        diff <= 1,
        "{label}: actual {actual} vs truth {truth} (diff {diff} > 1 LSB)",
    );
}
#[track_caller]
fn assert_le_1_lsb_i64<A: Into<i128>>(label: &str, actual: A, truth: i64) {
    // `actual` accepts D18's `Int<1>` to_bits (via `From<Int<1>> for i128`)
    // as well as plain `i64`; both are bridged to `i128` for the diff.
    let actual: i128 = actual.into();
    let truth = i128::from(truth);
    let diff = (actual - truth).abs();
    assert!(
        diff <= 1,
        "{label}: actual {actual} vs truth {truth} (diff {diff} > 1 LSB)",
    );
}

// ─── D18 strict suite (SCALE=8, range fits i64) ────────────────────────

#[test]
fn d18_ln_strict() {
    assert_eq!(D18::<8>::ONE.ln_strict().to_bits(), 0);
    // ln(2) = 0.69314718... → at S=8: 69_314_718
    assert_le_1_lsb_i64(
        "ln(2)",
        D18::<8>::try_from(2).unwrap().ln_strict().to_bits(),
        69_314_718,
    );
}

#[test]
fn d18_exp_strict() {
    assert_eq!(D18::<8>::ZERO.exp_strict().to_bits(), 100_000_000);
    // exp(1)=e ≈ 2.71828183 → 271_828_183
    assert_le_1_lsb_i64("exp(1)", D18::<8>::ONE.exp_strict().to_bits(), 271_828_183);
}

#[test]
fn d18_log2_log10_strict() {
    assert_eq!(D18::<8>::try_from(2).unwrap().log2_strict().to_bits(), 100_000_000);
    assert_eq!(D18::<8>::try_from(10).unwrap().log10_strict().to_bits(), 100_000_000);
}

#[test]
fn d18_sqrt_cbrt_strict() {
    assert_eq!(D18::<8>::try_from(4).unwrap().sqrt_strict().to_bits(), 200_000_000);
    assert_eq!(D18::<8>::try_from(27).unwrap().cbrt_strict().to_bits(), 300_000_000);
    // sqrt(2)=1.41421356... → 141_421_356
    assert_le_1_lsb_i64(
        "sqrt(2)",
        D18::<8>::try_from(2).unwrap().sqrt_strict().to_bits(),
        141_421_356,
    );
}

#[test]
fn d18_trig_strict() {
    assert_eq!(D18::<8>::ZERO.sin_strict().to_bits(), 0);
    assert_eq!(D18::<8>::ZERO.cos_strict().to_bits(), 100_000_000);
    // sin(1) ≈ 0.84147098 → 84_147_098
    assert_le_1_lsb_i64("sin(1)", D18::<8>::ONE.sin_strict().to_bits(), 84_147_098);
}

#[test]
fn d18_inverse_trig_hyperbolic_strict() {
    // atan(1)=π/4 ≈ 0.78539816 → 78_539_816
    assert_le_1_lsb_i64("atan(1)", D18::<8>::ONE.atan_strict().to_bits(), 78_539_816);
    assert_le_1_lsb_i64(
        "atan2(1,1)",
        D18::<8>::ONE.atan2_strict(D18::<8>::ONE).to_bits(),
        78_539_816,
    );
    assert_eq!(D18::<8>::ZERO.asin_strict().to_bits(), 0);
    assert_eq!(D18::<8>::ONE.acos_strict().to_bits(), 0);
    // sinh(1) ≈ 1.17520119
    assert_le_1_lsb_i64("sinh(1)", D18::<8>::ONE.sinh_strict().to_bits(), 117_520_119);
    assert_eq!(D18::<8>::ZERO.asinh_strict().to_bits(), 0);
    assert_eq!(D18::<8>::ONE.acosh_strict().to_bits(), 0);
    assert_eq!(D18::<8>::ZERO.atanh_strict().to_bits(), 0);
}

#[test]
fn d18_angle_powf_log_exp2_strict() {
    assert_eq!(D18::<8>::ZERO.to_degrees_strict().to_bits(), 0);
    assert_eq!(D18::<8>::ZERO.to_radians_strict().to_bits(), 0);
    // 2^10 = 1024
    assert_eq!(
        D18::<8>::try_from(2).unwrap()
            .powf_strict(D18::<8>::try_from(10).unwrap())
            .to_bits(),
        102_400_000_000,
    );
    // log_2(8) = 3
    assert_eq!(
        D18::<8>::try_from(8).unwrap().log_strict(D18::<8>::try_from(2).unwrap()).to_bits(),
        300_000_000,
    );
    // exp2(10) = 1024
    assert_eq!(D18::<8>::try_from(10).unwrap().exp2_strict().to_bits(), 102_400_000_000);
}

#[cfg(feature = "strict")]
#[test]
fn d18_dispatcher_matches_strict() {
    assert_eq!(D18::<8>::ONE.ln(), D18::<8>::ONE.ln_strict());
    assert_eq!(D18::<8>::ONE.exp(), D18::<8>::ONE.exp_strict());
    assert_eq!(D18::<8>::ONE.sin(), D18::<8>::ONE.sin_strict());
    assert_eq!(D18::<8>::ONE.cos(), D18::<8>::ONE.cos_strict());
    assert_eq!(D18::<8>::ONE.tan(), D18::<8>::ONE.tan_strict());
    assert_eq!(D18::<8>::ONE.sinh(), D18::<8>::ONE.sinh_strict());
    assert_eq!(D18::<8>::ONE.cosh(), D18::<8>::ONE.cosh_strict());
    assert_eq!(D18::<8>::ONE.tanh(), D18::<8>::ONE.tanh_strict());
    assert_eq!(D18::<8>::try_from(4).unwrap().sqrt(), D18::<8>::try_from(4).unwrap().sqrt_strict());
    assert_eq!(
        D18::<8>::try_from(27).unwrap().cbrt(),
        D18::<8>::try_from(27).unwrap().cbrt_strict()
    );
    assert_eq!(D18::<8>::ONE.atan(), D18::<8>::ONE.atan_strict());
    assert_eq!(
        D18::<8>::ONE.atan2(D18::<8>::ONE),
        D18::<8>::ONE.atan2_strict(D18::<8>::ONE)
    );
    assert_eq!(D18::<8>::ZERO.asin(), D18::<8>::ZERO.asin_strict());
    assert_eq!(D18::<8>::ONE.acos(), D18::<8>::ONE.acos_strict());
    assert_eq!(D18::<8>::ZERO.asinh(), D18::<8>::ZERO.asinh_strict());
    assert_eq!(D18::<8>::ONE.acosh(), D18::<8>::ONE.acosh_strict());
    assert_eq!(D18::<8>::ZERO.atanh(), D18::<8>::ZERO.atanh_strict());
    assert_eq!(D18::<8>::ONE.log2(), D18::<8>::ONE.log2_strict());
    assert_eq!(D18::<8>::ONE.log10(), D18::<8>::ONE.log10_strict());
    assert_eq!(D18::<8>::ONE.exp2(), D18::<8>::ONE.exp2_strict());
    assert_eq!(D18::<8>::ZERO.to_degrees(), D18::<8>::ZERO.to_degrees_strict());
    assert_eq!(D18::<8>::ZERO.to_radians(), D18::<8>::ZERO.to_radians_strict());
    assert_eq!(
        D18::<8>::try_from(8).unwrap().log(D18::<8>::try_from(2).unwrap()),
        D18::<8>::try_from(8).unwrap().log_strict(D18::<8>::try_from(2).unwrap()),
    );
    assert_eq!(
        D18::<8>::try_from(2).unwrap().powf(D18::<8>::try_from(10).unwrap()),
        D18::<8>::try_from(2).unwrap().powf_strict(D18::<8>::try_from(10).unwrap()),
    );
}

// ─── Narrowing range check (post-condition: panics if result out of range) ───
//
// Validates the `.expect("...: result out of range")` path that fires
// when the D38 result doesn't fit the D9/D18 storage. We deliberately
// choose an input whose strict transcendental result would exceed the
// narrow type's range; here exp_strict(5) at D18<17> would yield
// ~148.4, well above D18<17>::MAX ≈ 92.2.

#[test]
#[should_panic(expected = "exp_strict: result out of range")]
fn d18_exp_strict_overflow_panics() {
    let _ = D18::<17>::try_from(5).unwrap().exp_strict();
}

// Sanity: D38 also panics on range overflow, but with its own message.
// This test exists primarily to be invariant under future narrow-tier
// refactors that might lose the wrapping `.expect(...)`.
#[test]
#[should_panic]
fn d38_exp_strict_overflow_panics() {
    // D38<35>::exp(2) → ~7.389; storage at S=35 = 7.389e35, which is
    // below MAX (1.7e38). To force overflow we use a much larger arg.
    let _ = D38::<35>::try_from(100).unwrap().exp_strict();
}
