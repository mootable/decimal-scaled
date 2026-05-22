//! Regression: D115 strict transcendentals must not overflow their
//! work integer at high SCALE.
//!
//! D115 stores in `Int384` (384-bit) and runs its strict
//! transcendental kernels on a wider work integer at working scale
//! `w = SCALE + GUARD` (GUARD = 30). `exp_fixed` further lifts that to
//! `w_ext = w + extra` for the `2^k` range-reduction error budget.
//!
//! With a 1024-bit work integer the artanh `ln 2` series intermediate
//! `t = 10^w_ext / 3` squared (`t · t`, evaluated at full width before
//! the `÷ 10^w` step) reaches ~1040 bits at SCALE = 114
//! (`w_ext = 157`), exceeding the 1023-bit signed capacity and
//! panicking with `Int1024: mul overflow`. The `ln` path hit the
//! sibling `sqrt_fixed: |v| * 10^w overflows the working width` for the
//! same reason. D115's work integer is `Int2048`, matching the D153
//! tier above it; this test pins the in-domain high-SCALE inputs that
//! exposed the undersized tier.

#![cfg(feature = "wide")]

use decimal_scaled::{D115, DecimalConvert, DecimalTranscendental};

/// 1.5 at SCALE — the bench's `strict_block` ln/sin/sqrt input.
fn x_15<const S: u32>() -> D115<S> {
    let half = D115::<S>::from_int(1) / D115::<S>::from_int(2);
    D115::<S>::from_int(1) + half
}

/// 0.5 at SCALE — the bench's `strict_block` exp input.
fn xh_05<const S: u32>() -> D115<S> {
    D115::<S>::from_int(1) / D115::<S>::from_int(2)
}

/// Loose absolute tolerance for the `to_f64` cross-check. The kernels
/// are accurate to ~0.5 ULP at storage; `to_f64` itself caps usable
/// precision near f64's ~2.2e-16, so 1e-12 is generous and only proves
/// the result is the right number (not a wrapped/garbage value).
const F64_TOL: f64 = 1e-12;

#[track_caller]
fn close(label: &str, got: f64, want: f64) {
    let diff = (got - want).abs();
    assert!(
        diff <= F64_TOL,
        "{label}: got {got}, want {want} (diff {diff} > {F64_TOL})",
    );
}

// ── ln(1.5) ≈ 0.4054651081... ──────────────────────────────────────

#[test]
fn ln_1_5_scale_57_matches_math() {
    close("ln1.5 s57", x_15::<57>().ln_strict().to_f64(), 1.5f64.ln());
}

#[test]
fn ln_1_5_scale_114_no_overflow() {
    // The original overflow site (sqrt_fixed working-width overflow).
    close(
        "ln1.5 s114",
        x_15::<114>().ln_strict().to_f64(),
        1.5f64.ln(),
    );
}

// ── exp(0.5) ≈ 1.6487212707... ─────────────────────────────────────

#[test]
fn exp_0_5_scale_57_matches_math() {
    close(
        "exp0.5 s57",
        xh_05::<57>().exp_strict().to_f64(),
        0.5f64.exp(),
    );
}

#[test]
fn exp_0_5_scale_114_no_overflow() {
    // The original `Int1024: mul overflow` site (ln2_compute at w_ext).
    close(
        "exp0.5 s114",
        xh_05::<114>().exp_strict().to_f64(),
        0.5f64.exp(),
    );
}

// ── sin(1.5) ≈ 0.9974949866... and sqrt(1.5) ≈ 1.2247448714... ──────

#[test]
fn sin_1_5_scale_114_no_overflow() {
    close(
        "sin1.5 s114",
        x_15::<114>().sin_strict().to_f64(),
        1.5f64.sin(),
    );
}

#[test]
fn sqrt_1_5_scale_114_no_overflow() {
    close(
        "sqrt1.5 s114",
        x_15::<114>().sqrt_strict().to_f64(),
        1.5f64.sqrt(),
    );
}

// ── SCALE 0 (integer) sanity: ln(1)=0, exp(0)=1 after truncation ────
// At SCALE 0 the 1.5 / 0.5 inputs truncate to 1 / 0, so this only
// guards the s0 dispatch path against panicking, with exact checks.

#[test]
fn scale_0_dispatch_paths_are_exact() {
    assert_eq!(x_15::<0>().ln_strict().to_f64(), 1.0f64.ln(), "ln(1)=0");
    assert_eq!(xh_05::<0>().exp_strict().to_f64(), 0.0f64.exp(), "exp(0)=1");
    assert_eq!(
        x_15::<0>().sqrt_strict().to_f64(),
        1.0f64.sqrt(),
        "sqrt(1)=1"
    );
    // sin(1) ≈ 0.8415 rounds to the nearest integer (1) at SCALE 0.
    assert_eq!(
        x_15::<0>().sin_strict().to_f64(),
        1.0,
        "sin(1) rounds to 1 at SCALE 0"
    );
}
