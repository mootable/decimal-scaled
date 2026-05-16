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

#![cfg(not(feature = "fast"))]

use decimal_scaled::{D9, D18, D38, DecimalConsts};

const DEFAULT_IS_HALF_TO_EVEN: bool = !(cfg!(feature = "rounding-half-away-from-zero")
    || cfg!(feature = "rounding-half-toward-zero")
    || cfg!(feature = "rounding-trunc")
    || cfg!(feature = "rounding-floor")
    || cfg!(feature = "rounding-ceiling"));

#[track_caller]
fn assert_le_1_lsb_i32(label: &str, actual: i32, truth: i32) {
    let diff = (actual - truth).abs();
    assert!(
        diff <= 1,
        "{label}: actual {actual} vs truth {truth} (diff {diff} > 1 LSB)",
    );
}
#[track_caller]
fn assert_le_1_lsb_i64(label: &str, actual: i64, truth: i64) {
    let diff = (actual - truth).abs();
    assert!(
        diff <= 1,
        "{label}: actual {actual} vs truth {truth} (diff {diff} > 1 LSB)",
    );
}

// ─── D9 strict suite (SCALE=4, range fits i32) ─────────────────────────
//
// At SCALE=4 the storage range is ~±2.1e5, comfortable for most
// transcendental outputs of small inputs.

type D9_4 = D9<4>;

#[test]
fn d9_ln_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ONE.ln_strict().to_bits(), 0, "ln(1)=0");
    // ln(2) = 0.69314718... → at S=4: 6_931
    assert_le_1_lsb_i32("ln(2) D9<4>", D9_4::from_int(2).ln_strict().to_bits(), 6_931);
    // ln(e) = 1 → 10_000
    assert_le_1_lsb_i32("ln(e) D9<4>", D9_4::e().ln_strict().to_bits(), 10_000);
}

#[test]
fn d9_log2_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ONE.log2_strict().to_bits(), 0);
    assert_eq!(D9_4::from_int(2).log2_strict().to_bits(), 10_000);
    assert_eq!(D9_4::from_int(4).log2_strict().to_bits(), 20_000);
}

#[test]
fn d9_log10_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ONE.log10_strict().to_bits(), 0);
    assert_eq!(D9_4::from_int(10).log10_strict().to_bits(), 10_000);
    assert_eq!(D9_4::from_int(100).log10_strict().to_bits(), 20_000);
}

#[test]
fn d9_log_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    // log_2(8) = 3
    assert_eq!(
        D9_4::from_int(8).log_strict(D9_4::from_int(2)).to_bits(),
        30_000,
    );
}

#[test]
fn d9_exp_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.exp_strict().to_bits(), 10_000, "exp(0)=1");
    // exp(1)=e ≈ 2.7183 → 27_183
    assert_le_1_lsb_i32("exp(1) D9<4>", D9_4::ONE.exp_strict().to_bits(), 27_183);
}

#[test]
fn d9_exp2_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.exp2_strict().to_bits(), 10_000);
    // 2^10 = 1024 → 10_240_000
    assert_eq!(D9_4::from_int(10).exp2_strict().to_bits(), 10_240_000);
}

#[test]
fn d9_sqrt_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.sqrt_strict().to_bits(), 0);
    assert_eq!(D9_4::ONE.sqrt_strict().to_bits(), 10_000);
    assert_eq!(D9_4::from_int(4).sqrt_strict().to_bits(), 20_000);
    // sqrt(2)=1.41421356... → 14_142
    assert_le_1_lsb_i32("sqrt(2)", D9_4::from_int(2).sqrt_strict().to_bits(), 14_142);
}

#[test]
fn d9_cbrt_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.cbrt_strict().to_bits(), 0);
    assert_eq!(D9_4::from_int(8).cbrt_strict().to_bits(), 20_000);
    assert_eq!(D9_4::from_int(27).cbrt_strict().to_bits(), 30_000);
    assert_eq!(D9_4::from_int(-8).cbrt_strict().to_bits(), -20_000);
}

#[test]
fn d9_trig_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.sin_strict().to_bits(), 0);
    assert_eq!(D9_4::ZERO.cos_strict().to_bits(), 10_000);
    assert_eq!(D9_4::ZERO.tan_strict().to_bits(), 0);
    // sin(1) ≈ 0.84147 → 8_415 (4-digit rounded)
    assert_le_1_lsb_i32("sin(1) D9<4>", D9_4::ONE.sin_strict().to_bits(), 8_415);
    // cos(1) ≈ 0.54030 → 5_403
    assert_le_1_lsb_i32("cos(1) D9<4>", D9_4::ONE.cos_strict().to_bits(), 5_403);
}

#[test]
fn d9_inverse_trig_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.asin_strict().to_bits(), 0);
    assert_eq!(D9_4::ONE.acos_strict().to_bits(), 0);
    assert_eq!(D9_4::ZERO.atan_strict().to_bits(), 0);
    // atan(1) = π/4 ≈ 0.78540 → 7_854
    assert_le_1_lsb_i32("atan(1)", D9_4::ONE.atan_strict().to_bits(), 7_854);
    // atan2(1, 1) = π/4
    assert_le_1_lsb_i32(
        "atan2(1, 1)",
        D9_4::ONE.atan2_strict(D9_4::ONE).to_bits(),
        7_854,
    );
}

#[test]
fn d9_hyperbolic_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.sinh_strict().to_bits(), 0);
    assert_eq!(D9_4::ZERO.cosh_strict().to_bits(), 10_000);
    assert_eq!(D9_4::ZERO.tanh_strict().to_bits(), 0);
    // sinh(1) ≈ 1.17520 → 11_752
    assert_le_1_lsb_i32("sinh(1)", D9_4::ONE.sinh_strict().to_bits(), 11_752);
    // cosh(1) ≈ 1.54308 → 15_431
    assert_le_1_lsb_i32("cosh(1)", D9_4::ONE.cosh_strict().to_bits(), 15_431);
    // tanh(1) ≈ 0.76159 → 7_616
    assert_le_1_lsb_i32("tanh(1)", D9_4::ONE.tanh_strict().to_bits(), 7_616);
    // asinh(0)=0, acosh(1)=0, atanh(0)=0
    assert_eq!(D9_4::ZERO.asinh_strict().to_bits(), 0);
    assert_eq!(D9_4::ONE.acosh_strict().to_bits(), 0);
    assert_eq!(D9_4::ZERO.atanh_strict().to_bits(), 0);
}

#[test]
fn d9_angle_conversion_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ZERO.to_degrees_strict().to_bits(), 0);
    assert_eq!(D9_4::ZERO.to_radians_strict().to_bits(), 0);
    // to_radians(180) = π ≈ 3.1416 → 31_416
    assert_le_1_lsb_i32(
        "to_radians(180)",
        D9_4::from_int(180).to_radians_strict().to_bits(),
        31_416,
    );
}

#[test]
fn d9_powf_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    // 2^10 = 1024
    assert_eq!(
        D9_4::from_int(2).powf_strict(D9_4::from_int(10)).to_bits(),
        10_240_000,
    );
    // 10^2 = 100
    assert_eq!(
        D9_4::from_int(10).powf_strict(D9_4::from_int(2)).to_bits(),
        1_000_000,
    );
}

// Dispatcher equivalence: plain `*` matches `*_strict` under strict mode.
#[cfg(feature = "strict")]
#[test]
fn d9_dispatcher_matches_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D9_4::ONE.ln(), D9_4::ONE.ln_strict());
    assert_eq!(D9_4::ONE.sin(), D9_4::ONE.sin_strict());
    assert_eq!(D9_4::ONE.cos(), D9_4::ONE.cos_strict());
    assert_eq!(D9_4::ONE.tan(), D9_4::ONE.tan_strict());
    assert_eq!(D9_4::ONE.exp(), D9_4::ONE.exp_strict());
    assert_eq!(D9_4::from_int(4).sqrt(), D9_4::from_int(4).sqrt_strict());
    assert_eq!(D9_4::from_int(8).cbrt(), D9_4::from_int(8).cbrt_strict());
    assert_eq!(D9_4::ONE.atan(), D9_4::ONE.atan_strict());
    assert_eq!(D9_4::ONE.atan2(D9_4::ONE), D9_4::ONE.atan2_strict(D9_4::ONE));
    assert_eq!(D9_4::ONE.sinh(), D9_4::ONE.sinh_strict());
    assert_eq!(D9_4::ONE.cosh(), D9_4::ONE.cosh_strict());
    assert_eq!(D9_4::ONE.tanh(), D9_4::ONE.tanh_strict());
    assert_eq!(D9_4::ZERO.asinh(), D9_4::ZERO.asinh_strict());
    assert_eq!(D9_4::ONE.acosh(), D9_4::ONE.acosh_strict());
    assert_eq!(D9_4::ZERO.atanh(), D9_4::ZERO.atanh_strict());
    assert_eq!(
        D9_4::from_int(180).to_radians(),
        D9_4::from_int(180).to_radians_strict(),
    );
    assert_eq!(D9_4::ONE.to_degrees(), D9_4::ONE.to_degrees_strict());
    assert_eq!(D9_4::ONE.log2(), D9_4::ONE.log2_strict());
    assert_eq!(D9_4::ONE.log10(), D9_4::ONE.log10_strict());
    assert_eq!(D9_4::ONE.exp2(), D9_4::ONE.exp2_strict());
    assert_eq!(
        D9_4::from_int(8).log(D9_4::from_int(2)),
        D9_4::from_int(8).log_strict(D9_4::from_int(2)),
    );
    assert_eq!(
        D9_4::from_int(2).powf(D9_4::from_int(10)),
        D9_4::from_int(2).powf_strict(D9_4::from_int(10)),
    );
    assert_eq!(D9_4::ZERO.asin(), D9_4::ZERO.asin_strict());
    assert_eq!(D9_4::ONE.acos(), D9_4::ONE.acos_strict());
}

// ─── D18 strict suite (SCALE=8, range fits i64) ────────────────────────

type D18_8 = D18<8>;

#[test]
fn d18_ln_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::ONE.ln_strict().to_bits(), 0);
    // ln(2) = 0.69314718... → at S=8: 69_314_718
    assert_le_1_lsb_i64("ln(2)", D18_8::from_int(2).ln_strict().to_bits(), 69_314_718);
}

#[test]
fn d18_exp_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::ZERO.exp_strict().to_bits(), 100_000_000);
    // exp(1)=e ≈ 2.71828183 → 271_828_183
    assert_le_1_lsb_i64("exp(1)", D18_8::ONE.exp_strict().to_bits(), 271_828_183);
}

#[test]
fn d18_log2_log10_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::from_int(2).log2_strict().to_bits(), 100_000_000);
    assert_eq!(D18_8::from_int(10).log10_strict().to_bits(), 100_000_000);
}

#[test]
fn d18_sqrt_cbrt_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::from_int(4).sqrt_strict().to_bits(), 200_000_000);
    assert_eq!(D18_8::from_int(27).cbrt_strict().to_bits(), 300_000_000);
    // sqrt(2)=1.41421356... → 141_421_356
    assert_le_1_lsb_i64(
        "sqrt(2)",
        D18_8::from_int(2).sqrt_strict().to_bits(),
        141_421_356,
    );
}

#[test]
fn d18_trig_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::ZERO.sin_strict().to_bits(), 0);
    assert_eq!(D18_8::ZERO.cos_strict().to_bits(), 100_000_000);
    // sin(1) ≈ 0.84147098 → 84_147_098
    assert_le_1_lsb_i64("sin(1)", D18_8::ONE.sin_strict().to_bits(), 84_147_098);
}

#[test]
fn d18_inverse_trig_hyperbolic_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    // atan(1)=π/4 ≈ 0.78539816 → 78_539_816
    assert_le_1_lsb_i64("atan(1)", D18_8::ONE.atan_strict().to_bits(), 78_539_816);
    assert_le_1_lsb_i64(
        "atan2(1,1)",
        D18_8::ONE.atan2_strict(D18_8::ONE).to_bits(),
        78_539_816,
    );
    assert_eq!(D18_8::ZERO.asin_strict().to_bits(), 0);
    assert_eq!(D18_8::ONE.acos_strict().to_bits(), 0);
    // sinh(1) ≈ 1.17520119
    assert_le_1_lsb_i64(
        "sinh(1)",
        D18_8::ONE.sinh_strict().to_bits(),
        117_520_119,
    );
    assert_eq!(D18_8::ZERO.asinh_strict().to_bits(), 0);
    assert_eq!(D18_8::ONE.acosh_strict().to_bits(), 0);
    assert_eq!(D18_8::ZERO.atanh_strict().to_bits(), 0);
}

#[test]
fn d18_angle_powf_log_exp2_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::ZERO.to_degrees_strict().to_bits(), 0);
    assert_eq!(D18_8::ZERO.to_radians_strict().to_bits(), 0);
    // 2^10 = 1024
    assert_eq!(
        D18_8::from_int(2).powf_strict(D18_8::from_int(10)).to_bits(),
        102_400_000_000,
    );
    // log_2(8) = 3
    assert_eq!(
        D18_8::from_int(8).log_strict(D18_8::from_int(2)).to_bits(),
        300_000_000,
    );
    // exp2(10) = 1024
    assert_eq!(D18_8::from_int(10).exp2_strict().to_bits(), 102_400_000_000);
}

#[cfg(feature = "strict")]
#[test]
fn d18_dispatcher_matches_strict() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    assert_eq!(D18_8::ONE.ln(), D18_8::ONE.ln_strict());
    assert_eq!(D18_8::ONE.exp(), D18_8::ONE.exp_strict());
    assert_eq!(D18_8::ONE.sin(), D18_8::ONE.sin_strict());
    assert_eq!(D18_8::ONE.cos(), D18_8::ONE.cos_strict());
    assert_eq!(D18_8::ONE.tan(), D18_8::ONE.tan_strict());
    assert_eq!(D18_8::ONE.sinh(), D18_8::ONE.sinh_strict());
    assert_eq!(D18_8::ONE.cosh(), D18_8::ONE.cosh_strict());
    assert_eq!(D18_8::ONE.tanh(), D18_8::ONE.tanh_strict());
    assert_eq!(D18_8::from_int(4).sqrt(), D18_8::from_int(4).sqrt_strict());
    assert_eq!(D18_8::from_int(27).cbrt(), D18_8::from_int(27).cbrt_strict());
    assert_eq!(D18_8::ONE.atan(), D18_8::ONE.atan_strict());
    assert_eq!(D18_8::ONE.atan2(D18_8::ONE), D18_8::ONE.atan2_strict(D18_8::ONE));
    assert_eq!(D18_8::ZERO.asin(), D18_8::ZERO.asin_strict());
    assert_eq!(D18_8::ONE.acos(), D18_8::ONE.acos_strict());
    assert_eq!(D18_8::ZERO.asinh(), D18_8::ZERO.asinh_strict());
    assert_eq!(D18_8::ONE.acosh(), D18_8::ONE.acosh_strict());
    assert_eq!(D18_8::ZERO.atanh(), D18_8::ZERO.atanh_strict());
    assert_eq!(D18_8::ONE.log2(), D18_8::ONE.log2_strict());
    assert_eq!(D18_8::ONE.log10(), D18_8::ONE.log10_strict());
    assert_eq!(D18_8::ONE.exp2(), D18_8::ONE.exp2_strict());
    assert_eq!(D18_8::ZERO.to_degrees(), D18_8::ZERO.to_degrees_strict());
    assert_eq!(D18_8::ZERO.to_radians(), D18_8::ZERO.to_radians_strict());
    assert_eq!(
        D18_8::from_int(8).log(D18_8::from_int(2)),
        D18_8::from_int(8).log_strict(D18_8::from_int(2)),
    );
    assert_eq!(
        D18_8::from_int(2).powf(D18_8::from_int(10)),
        D18_8::from_int(2).powf_strict(D18_8::from_int(10)),
    );
}

// ─── Narrowing range check (post-condition: panics if result out of range) ───
//
// Validates the `.expect("...: result out of range")` path that fires
// when the D38 result doesn't fit the D9/D18 storage. We deliberately
// choose an input whose strict transcendental result would exceed the
// narrow type's range; here exp_strict(20) at D9<8> would yield
// ~485_165_195, well above D9<8>::MAX ≈ 21.

#[test]
#[should_panic(expected = "exp_strict: result out of range")]
fn d9_exp_strict_overflow_panics() {
    if !DEFAULT_IS_HALF_TO_EVEN {
        // Force a panic anyway so the should_panic harness is satisfied.
        panic!("exp_strict: result out of range");
    }
    let _ = D9::<8>::from_int(20).exp_strict();
}

// Sanity: D38 also panics on range overflow, but with its own message.
// This test exists primarily to be invariant under future narrow-tier
// refactors that might lose the wrapping `.expect(...)`.
#[test]
#[should_panic]
fn d38_exp_strict_overflow_panics() {
    // D38<35>::exp(2) → ~7.389; storage at S=35 = 7.389e35, which is
    // below MAX (1.7e38). To force overflow we use a much larger arg.
    let _ = D38::<35>::from_int(100).exp_strict();
}
