//! Coverage suite for `macros/fast_transcendentals.rs` — the f64-bridge
//! transcendental surface emitted for every non-D38 width (D9 / D18 and
//! the wide tiers).
//!
//! These methods are now always-callable via the explicit `*_fast` form
//! when `feature = "std"` is on, regardless of strict/fast mode. The
//! plain `*` dispatcher resolves to them under
//! `any(not(strict), fast)` — both call paths are exercised here so the
//! routing fix stays covered.
//!
//! Accuracy contract: f64 round-trip introduces ~1 LSB of quantisation
//! noise per `to_f64` / `from_f64`, and the transcendental itself can
//! introduce another LSB or two. At SCALE=4 (D9) the LSB is 10⁻⁴, well
//! above f64 noise, so the test tolerance is wide enough to never be a
//! source of false failure. We just check the macro-emitted bodies are
//! reachable and produce sensible results.

#![cfg(feature = "std")]

use decimal_scaled::{D9, D18};

type D9_4 = D9<4>;
type D18_8 = D18<8>;

/// Loose tolerance for the f64-bridge: 4 LSB at S=4 is 4·10⁻⁴ ≈ 4e-4,
/// orders of magnitude above the f64 quantisation noise.
const D9_TOL: i32 = 8;
const D18_TOL: i64 = 64;

#[track_caller]
fn close_d9(label: &str, actual: D9_4, expected_bits: i32) {
    let diff = (actual.to_bits() - expected_bits).abs();
    assert!(
        diff <= D9_TOL,
        "{label}: bits {} vs expected {expected_bits} (diff {diff} > {D9_TOL})",
        actual.to_bits()
    );
}
#[track_caller]
fn close_d18(label: &str, actual: D18_8, expected_bits: i64) {
    let diff = (actual.to_bits() - expected_bits).abs();
    assert!(
        diff <= D18_TOL,
        "{label}: bits {} vs expected {expected_bits} (diff {diff} > {D18_TOL})",
        actual.to_bits()
    );
}

// ─── D9 fast ───────────────────────────────────────────────────────────

#[test]
fn d9_logs_fast() {
    assert_eq!(D9_4::ONE.ln_fast().to_bits(), 0);
    close_d9("ln(2)", D9_4::from_int(2).ln_fast(), 6_931);
    assert_eq!(D9_4::ONE.log2_fast().to_bits(), 0);
    close_d9("log2(2)", D9_4::from_int(2).log2_fast(), 10_000);
    assert_eq!(D9_4::ONE.log10_fast().to_bits(), 0);
    close_d9("log10(10)", D9_4::from_int(10).log10_fast(), 10_000);
    // log_base
    close_d9(
        "log_2(8)",
        D9_4::from_int(8).log_fast(D9_4::from_int(2)),
        30_000,
    );
}

#[test]
fn d9_exps_fast() {
    assert_eq!(D9_4::ZERO.exp_fast().to_bits(), 10_000);
    close_d9("exp(1)", D9_4::ONE.exp_fast(), 27_183);
    assert_eq!(D9_4::ZERO.exp2_fast().to_bits(), 10_000);
    close_d9("exp2(10)", D9_4::from_int(10).exp2_fast(), 10_240_000);
}

#[test]
fn d9_roots_pow_fast() {
    assert_eq!(D9_4::ZERO.sqrt_fast().to_bits(), 0);
    assert_eq!(D9_4::ONE.sqrt_fast().to_bits(), 10_000);
    close_d9("sqrt(2)", D9_4::from_int(2).sqrt_fast(), 14_142);
    assert_eq!(D9_4::ZERO.cbrt_fast().to_bits(), 0);
    close_d9("cbrt(8)", D9_4::from_int(8).cbrt_fast(), 20_000);
    close_d9("cbrt(-8)", D9_4::from_int(-8).cbrt_fast(), -20_000);
    close_d9(
        "2^10",
        D9_4::from_int(2).powf_fast(D9_4::from_int(10)),
        10_240_000,
    );
    // hypot(3, 4) = 5
    close_d9(
        "hypot(3,4)",
        D9_4::from_int(3).hypot_fast(D9_4::from_int(4)),
        50_000,
    );
}

#[test]
fn d9_trig_fast() {
    assert_eq!(D9_4::ZERO.sin_fast().to_bits(), 0);
    assert_eq!(D9_4::ZERO.cos_fast().to_bits(), 10_000);
    assert_eq!(D9_4::ZERO.tan_fast().to_bits(), 0);
    close_d9("sin(1)", D9_4::ONE.sin_fast(), 8_415);
    close_d9("cos(1)", D9_4::ONE.cos_fast(), 5_403);
    // asin(0)=0, acos(1)=0
    assert_eq!(D9_4::ZERO.asin_fast().to_bits(), 0);
    assert_eq!(D9_4::ONE.acos_fast().to_bits(), 0);
    assert_eq!(D9_4::ZERO.atan_fast().to_bits(), 0);
    close_d9("atan(1)", D9_4::ONE.atan_fast(), 7_854);
    close_d9("atan2(1,1)", D9_4::ONE.atan2_fast(D9_4::ONE), 7_854);
}

#[test]
fn d9_hyperbolic_fast() {
    assert_eq!(D9_4::ZERO.sinh_fast().to_bits(), 0);
    assert_eq!(D9_4::ZERO.cosh_fast().to_bits(), 10_000);
    assert_eq!(D9_4::ZERO.tanh_fast().to_bits(), 0);
    close_d9("sinh(1)", D9_4::ONE.sinh_fast(), 11_752);
    close_d9("cosh(1)", D9_4::ONE.cosh_fast(), 15_431);
    close_d9("tanh(1)", D9_4::ONE.tanh_fast(), 7_616);
    assert_eq!(D9_4::ZERO.asinh_fast().to_bits(), 0);
    assert_eq!(D9_4::ONE.acosh_fast().to_bits(), 0);
    assert_eq!(D9_4::ZERO.atanh_fast().to_bits(), 0);
}

#[test]
fn d9_angle_conversion_fast() {
    assert_eq!(D9_4::ZERO.to_degrees_fast().to_bits(), 0);
    assert_eq!(D9_4::ZERO.to_radians_fast().to_bits(), 0);
    close_d9(
        "to_radians(180)",
        D9_4::from_int(180).to_radians_fast(),
        31_416,
    );
}

// ─── D18 fast ──────────────────────────────────────────────────────────

#[test]
fn d18_logs_exps_fast() {
    assert_eq!(D18_8::ONE.ln_fast().to_bits(), 0);
    assert_eq!(D18_8::ZERO.exp_fast().to_bits(), 100_000_000);
    close_d18("ln(2)", D18_8::from_int(2).ln_fast(), 69_314_718);
    close_d18("exp(1)", D18_8::ONE.exp_fast(), 271_828_183);
    assert_eq!(D18_8::from_int(2).log2_fast().to_bits(), 100_000_000);
    assert_eq!(D18_8::from_int(10).log10_fast().to_bits(), 100_000_000);
    assert_eq!(D18_8::from_int(10).exp2_fast().to_bits(), 102_400_000_000);
}

#[test]
fn d18_roots_pow_fast() {
    assert_eq!(D18_8::from_int(4).sqrt_fast().to_bits(), 200_000_000);
    assert_eq!(D18_8::from_int(27).cbrt_fast().to_bits(), 300_000_000);
    close_d18(
        "2^10",
        D18_8::from_int(2).powf_fast(D18_8::from_int(10)),
        102_400_000_000,
    );
    close_d18(
        "hypot(3,4)",
        D18_8::from_int(3).hypot_fast(D18_8::from_int(4)),
        500_000_000,
    );
    close_d18(
        "log_2(8)",
        D18_8::from_int(8).log_fast(D18_8::from_int(2)),
        300_000_000,
    );
}

#[test]
fn d18_trig_inverse_hyperbolic_fast() {
    assert_eq!(D18_8::ZERO.sin_fast().to_bits(), 0);
    assert_eq!(D18_8::ZERO.cos_fast().to_bits(), 100_000_000);
    close_d18("sin(1)", D18_8::ONE.sin_fast(), 84_147_098);
    close_d18("atan(1)", D18_8::ONE.atan_fast(), 78_539_816);
    close_d18("atan2(1,1)", D18_8::ONE.atan2_fast(D18_8::ONE), 78_539_816);
    assert_eq!(D18_8::ZERO.tan_fast().to_bits(), 0);
    assert_eq!(D18_8::ZERO.asin_fast().to_bits(), 0);
    assert_eq!(D18_8::ONE.acos_fast().to_bits(), 0);
    close_d18("sinh(1)", D18_8::ONE.sinh_fast(), 117_520_119);
    close_d18("cosh(1)", D18_8::ONE.cosh_fast(), 154_308_063);
    close_d18("tanh(1)", D18_8::ONE.tanh_fast(), 76_159_416);
    assert_eq!(D18_8::ZERO.asinh_fast().to_bits(), 0);
    assert_eq!(D18_8::ONE.acosh_fast().to_bits(), 0);
    assert_eq!(D18_8::ZERO.atanh_fast().to_bits(), 0);
}

#[test]
fn d18_angle_conversion_fast() {
    assert_eq!(D18_8::ZERO.to_degrees_fast().to_bits(), 0);
    assert_eq!(D18_8::ZERO.to_radians_fast().to_bits(), 0);
}

// ─── Dispatcher under fast/no-strict mode ──────────────────────────────
//
// In strict mode the plain `*` resolves to `*_strict`, not `*_fast`. The
// dispatcher emitted by fast_transcendentals.rs is only active under
// `any(not(strict), fast)`; this test runs only in that mode.

#[cfg(any(not(feature = "strict"), feature = "fast"))]
#[test]
fn d9_dispatcher_matches_fast() {
    assert_eq!(D9_4::ONE.ln(), D9_4::ONE.ln_fast());
    assert_eq!(D9_4::ONE.sin(), D9_4::ONE.sin_fast());
    assert_eq!(D9_4::ONE.cos(), D9_4::ONE.cos_fast());
    assert_eq!(D9_4::ONE.tan(), D9_4::ONE.tan_fast());
    assert_eq!(D9_4::ONE.exp(), D9_4::ONE.exp_fast());
    assert_eq!(D9_4::from_int(4).sqrt(), D9_4::from_int(4).sqrt_fast());
    assert_eq!(D9_4::from_int(8).cbrt(), D9_4::from_int(8).cbrt_fast());
    assert_eq!(D9_4::ONE.atan(), D9_4::ONE.atan_fast());
    assert_eq!(D9_4::ONE.atan2(D9_4::ONE), D9_4::ONE.atan2_fast(D9_4::ONE));
    assert_eq!(D9_4::ONE.sinh(), D9_4::ONE.sinh_fast());
    assert_eq!(D9_4::ONE.cosh(), D9_4::ONE.cosh_fast());
    assert_eq!(D9_4::ONE.tanh(), D9_4::ONE.tanh_fast());
    assert_eq!(D9_4::ZERO.asinh(), D9_4::ZERO.asinh_fast());
    assert_eq!(D9_4::ONE.acosh(), D9_4::ONE.acosh_fast());
    assert_eq!(D9_4::ZERO.atanh(), D9_4::ZERO.atanh_fast());
    assert_eq!(
        D9_4::from_int(180).to_radians(),
        D9_4::from_int(180).to_radians_fast(),
    );
    assert_eq!(D9_4::ONE.to_degrees(), D9_4::ONE.to_degrees_fast());
    assert_eq!(D9_4::ONE.log2(), D9_4::ONE.log2_fast());
    assert_eq!(D9_4::ONE.log10(), D9_4::ONE.log10_fast());
    assert_eq!(D9_4::ONE.exp2(), D9_4::ONE.exp2_fast());
    assert_eq!(
        D9_4::from_int(8).log(D9_4::from_int(2)),
        D9_4::from_int(8).log_fast(D9_4::from_int(2)),
    );
    assert_eq!(
        D9_4::from_int(2).powf(D9_4::from_int(10)),
        D9_4::from_int(2).powf_fast(D9_4::from_int(10)),
    );
    assert_eq!(D9_4::ZERO.asin(), D9_4::ZERO.asin_fast());
    assert_eq!(D9_4::ONE.acos(), D9_4::ONE.acos_fast());
    assert_eq!(
        D9_4::from_int(3).hypot(D9_4::from_int(4)),
        D9_4::from_int(3).hypot_fast(D9_4::from_int(4)),
    );
}
