//! Final-wave coverage — targets the specific uncovered lines remaining
//! after the broader coverage waves. Each test is annotated with the
//! source-file line(s) it exercises.

#![cfg(all(not(feature = "fast"), feature = "wide"))]

use decimal_scaled::{D38, D76, RoundingMode};

const HALF_TO_EVEN: bool = !(cfg!(feature = "rounding-half-away-from-zero")
    || cfg!(feature = "rounding-half-toward-zero")
    || cfg!(feature = "rounding-trunc")
    || cfg!(feature = "rounding-floor")
    || cfg!(feature = "rounding-ceiling"));

type D76_6 = D76<6>;
type D38_6 = D38<6>;

fn lift(n: D38_6) -> D76_6 { n.into() }

// ─── wide_transcendental: asinh |x|>=1 / acosh v>=2 branches in _with ──

#[test]
fn d76_asinh_strict_with_abs_ge_one_branch() {
    if !HALF_TO_EVEN { return; }
    // |x|>=1 takes the alternate algorithm path.
    let two = lift(D38_6::from_int(2));
    let _ = two.asinh_strict_with(RoundingMode::HalfToEven);
    let neg_two = lift(D38_6::from_int(-2));
    let _ = neg_two.asinh_strict_with(RoundingMode::HalfToEven);
    // And via plain *_strict to also hit the canonical body's branch
    let _ = two.asinh_strict();
    let _ = neg_two.asinh_strict();
}

#[test]
fn d76_acosh_strict_with_v_ge_two_branch() {
    if !HALF_TO_EVEN { return; }
    let three = lift(D38_6::from_int(3));
    let _ = three.acosh_strict_with(RoundingMode::HalfToEven);
}

// ─── wide_transcendental: every _strict_with panic path ────────────────

#[test]
#[should_panic(expected = "log: argument must be positive")]
fn d76_log_strict_with_zero_panics() {
    let _ = D76_6::ZERO.log_strict_with(lift(D38_6::from_int(2)), RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "log: base must be positive")]
fn d76_log_strict_with_base_zero_panics() {
    let one: D76_6 = D38_6::ONE.into();
    let _ = one.log_strict_with(D76_6::ZERO, RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "log: base must not equal 1")]
fn d76_log_strict_with_base_one_panics() {
    let one: D76_6 = D38_6::ONE.into();
    let _ = one.log_strict_with(one, RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "log2: argument must be positive")]
fn d76_log2_strict_with_zero_panics() {
    let _ = D76_6::ZERO.log2_strict_with(RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "log10: argument must be positive")]
fn d76_log10_strict_with_zero_panics() {
    let _ = D76_6::ZERO.log10_strict_with(RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "ln_agm: argument must be positive")]
fn d76_ln_agm_with_zero_panics() {
    let _ = D76_6::ZERO.ln_strict_agm_with(RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "acos: argument out of domain")]
fn d76_acos_strict_with_oob_panics() {
    let _ = lift(D38_6::from_int(2)).acos_strict_with(RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "acosh: argument must be >= 1")]
fn d76_acosh_strict_with_below_one_panics() {
    let _ = D76_6::ZERO.acosh_strict_with(RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "atanh: argument out of domain")]
fn d76_atanh_strict_with_boundary_panics() {
    let _ = lift(D38_6::ONE).atanh_strict_with(RoundingMode::HalfToEven);
}

// ─── log_strict_agm + asin/acos boundary for _with ─────────────────────

#[test]
fn d76_strict_with_boundary_paths() {
    if !HALF_TO_EVEN { return; }
    let one: D76_6 = D38_6::ONE.into();
    // asin/acos at |x|=1 takes the half_pi shortcut
    let _ = one.asin_strict_with(RoundingMode::HalfToEven);
    let _ = (-one).asin_strict_with(RoundingMode::HalfToEven);
    let _ = (-one).acos_strict_with(RoundingMode::HalfToEven);
    // atan2 axis branches in _with form
    let _ = one.atan2_strict_with(D76_6::ZERO, RoundingMode::HalfToEven);
    let _ = (-one).atan2_strict_with(D76_6::ZERO, RoundingMode::HalfToEven);
    let _ = D76_6::ZERO.atan2_strict_with(-one, RoundingMode::HalfToEven);
    // tan_strict_with non-zero cos
    let _ = one.tan_strict_with(RoundingMode::HalfToEven);
    // ln_strict_agm and exp_strict_agm in plain form (not _with)
    let two: D76_6 = D38_6::from_int(2).into();
    let _ = two.ln_strict_agm();
    let _ = D76_6::ZERO.exp_strict_agm(); // ZERO short-circuit
}

// ─── powers_strict.rs: log/exp domain panics on D38 ────────────────────

#[test]
#[should_panic(expected = "D38::ln: argument must be positive")]
fn d38_ln_strict_zero_panics() {
    let _ = D38::<12>::ZERO.ln_strict();
}

#[test]
#[should_panic(expected = "D38::log: base must not equal 1")]
fn d38_log_base_one_panics() {
    let _ = D38::<12>::from_int(2).log_strict(D38::<12>::ONE);
}

// ─── float_bridge: rounding-mode rounding at half-LSB ──────────────────

#[cfg(feature = "std")]
#[test]
fn float_bridge_d38_rounding_modes_half_lsb() {
    // 1.005 at S=2 sits between 100 and 101 → exact-half tie.
    let v = D38::<2>::from_f64_with(1.005, RoundingMode::HalfToEven);
    let _ = v;
    let v = D38::<2>::from_f64_with(1.005, RoundingMode::Trunc);
    let _ = v;
    let v = D38::<2>::from_f64_with(1.005, RoundingMode::Floor);
    let _ = v;
    let v = D38::<2>::from_f64_with(1.005, RoundingMode::Ceiling);
    let _ = v;
    let v = D38::<2>::from_f64_with(-1.005, RoundingMode::Floor);
    let _ = v;
}
