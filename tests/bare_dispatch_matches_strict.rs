//! The bare-named transcendentals must dispatch to the correctly-rounded
//! `*_strict` sibling in every configuration except the explicit
//! `fast`-without-`strict` opt-in documented in `Cargo.toml`.
//!
//! This is deliberately a CROSS-WIDTH check. D38's bare methods are
//! hand-written per type (`src/types/{log_exp,trig,powers}.rs`, with the
//! f64 bridge in the `*_fast.rs` siblings), while every other width's come
//! from `src/macros/strict_transcendentals.rs`. Those two gate sets are
//! written independently, so they can drift apart — and in 0.5.1 they did:
//! under `--no-default-features --features std` the D38 bridge claimed the
//! bare name whenever `strict` was merely absent, so `D38::ln` silently
//! became the f64 bridge (426 ULP at SCALE 20) while `D18::ln` stayed
//! correctly rounded. One build, one method name, two different guarantees
//! depending on the width.
//!
//! Asserting bit-equality against `*_strict` on both a hand-written width
//! and a macro width pins them together, so any future divergence fails
//! here rather than shipping. Run under both the default feature set and
//! `--no-default-features --features std`.

#![cfg(not(all(feature = "fast", not(feature = "strict"))))]

use decimal_scaled::{D18, D38};

// SCALE 10 on D38 and SCALE 6 on D18 both leave ample headroom for the
// largest value produced below (`exp(3)` is about 20.09).
type W38 = D38<10>;
type W18 = D18<6>;

fn d38(v: i64) -> W38 {
    W38::try_from(v).unwrap()
}

fn d18(v: i64) -> W18 {
    W18::try_from(v).unwrap()
}

// ── D38 — the hand-written per-type width ───────────────────────────

#[test]
fn d38_bare_log_exp_matches_strict() {
    let x = d38(3);
    let base = d38(2);
    assert_eq!(x.ln(), x.ln_strict(), "D38::ln must be the strict path");
    assert_eq!(x.log2(), x.log2_strict());
    assert_eq!(x.log10(), x.log10_strict());
    assert_eq!(x.log(base), x.log_strict(base));
    assert_eq!(x.exp(), x.exp_strict(), "D38::exp must be the strict path");
    assert_eq!(x.exp2(), x.exp2_strict());
}

#[test]
fn d38_bare_trig_matches_strict() {
    let x = d38(3);
    let y = d38(2);
    // `half` keeps asin / acos / atanh inside their |x| <= 1 domain.
    let half = d38(1) / d38(2);
    assert_eq!(x.sin(), x.sin_strict(), "D38::sin must be the strict path");
    assert_eq!(x.cos(), x.cos_strict());
    assert_eq!(x.tan(), x.tan_strict());
    assert_eq!(half.asin(), half.asin_strict());
    assert_eq!(half.acos(), half.acos_strict());
    assert_eq!(x.atan(), x.atan_strict());
    assert_eq!(x.atan2(y), x.atan2_strict(y));
    assert_eq!(x.sinh(), x.sinh_strict());
    assert_eq!(x.cosh(), x.cosh_strict());
    assert_eq!(x.tanh(), x.tanh_strict());
    assert_eq!(x.asinh(), x.asinh_strict());
    assert_eq!(x.acosh(), x.acosh_strict());
    assert_eq!(half.atanh(), half.atanh_strict());
    assert_eq!(x.to_degrees(), x.to_degrees_strict());
    assert_eq!(x.to_radians(), x.to_radians_strict());
}

#[test]
fn d38_bare_powers_match_strict() {
    let x = d38(3);
    let y = d38(2);
    assert_eq!(x.sqrt(), x.sqrt_strict(), "D38::sqrt must be the strict path");
    assert_eq!(x.cbrt(), x.cbrt_strict());
    assert_eq!(x.powf(y), x.powf_strict(y));
    assert_eq!(x.hypot(y), x.hypot_strict(y));
}

// ── D18 — the macro-generated width ─────────────────────────────────

#[test]
fn d18_bare_log_exp_matches_strict() {
    let x = d18(3);
    let base = d18(2);
    assert_eq!(x.ln(), x.ln_strict(), "D18::ln must be the strict path");
    assert_eq!(x.log2(), x.log2_strict());
    assert_eq!(x.log10(), x.log10_strict());
    assert_eq!(x.log(base), x.log_strict(base));
    assert_eq!(x.exp(), x.exp_strict(), "D18::exp must be the strict path");
    assert_eq!(x.exp2(), x.exp2_strict());
}

#[test]
fn d18_bare_trig_matches_strict() {
    let x = d18(3);
    let y = d18(2);
    let half = d18(1) / d18(2);
    assert_eq!(x.sin(), x.sin_strict(), "D18::sin must be the strict path");
    assert_eq!(x.cos(), x.cos_strict());
    assert_eq!(x.tan(), x.tan_strict());
    assert_eq!(half.asin(), half.asin_strict());
    assert_eq!(half.acos(), half.acos_strict());
    assert_eq!(x.atan(), x.atan_strict());
    assert_eq!(x.atan2(y), x.atan2_strict(y));
    assert_eq!(x.sinh(), x.sinh_strict());
    assert_eq!(x.cosh(), x.cosh_strict());
    assert_eq!(x.tanh(), x.tanh_strict());
    assert_eq!(x.asinh(), x.asinh_strict());
    assert_eq!(x.acosh(), x.acosh_strict());
    assert_eq!(half.atanh(), half.atanh_strict());
    assert_eq!(x.to_degrees(), x.to_degrees_strict());
    assert_eq!(x.to_radians(), x.to_radians_strict());
}

#[test]
fn d18_bare_powers_match_strict() {
    let x = d18(3);
    let y = d18(2);
    assert_eq!(x.sqrt(), x.sqrt_strict(), "D18::sqrt must be the strict path");
    assert_eq!(x.cbrt(), x.cbrt_strict());
    assert_eq!(x.powf(y), x.powf_strict(y));
}
