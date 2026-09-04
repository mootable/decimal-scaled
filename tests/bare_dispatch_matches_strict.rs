//! The bare-named transcendentals must dispatch to the correctly-rounded
//! `*_strict` sibling. Since the f64-bridge `*_fast` surface was removed
//! there is only one definition of each bare name, so this now holds
//! unconditionally rather than in "every configuration except one".
//!
//! This is deliberately a CROSS-WIDTH check. D38's bare methods are
//! hand-written per type (`src/types/{log_exp,trig,powers}.rs`) while every
//! other width's come from `src/macros/strict_transcendentals.rs`. Those two
//! sets are written independently, so they can drift apart — and while the
//! bridge existed they did: under `--no-default-features --features std` the
//! D38 bridge claimed the bare name whenever `strict` was merely absent, so
//! `D38::ln` silently became the f64 bridge (426 ULP at SCALE 20) while
//! `D18::ln` stayed correctly rounded. One build, one method name, two
//! different guarantees depending on the width. Removing the bridge removed
//! that whole class — there is no second definition left to win the name —
//! but the cross-width pin is kept because the two definition sites remain.
//!
//! This file must name EVERY bare method, because it can only catch what it
//! names: `log1p` and `expm1` were added to D38 after the first pass here
//! and kept the old gate for a release, which this file did not catch
//! precisely because it did not list them. When a transcendental is added,
//! add it here too.

use decimal_scaled::{D18, D38};

// The scales are chosen so the f64 bridge is DISTINGUISHABLE from the
// strict path. f64 carries roughly 16 significant decimal digits, so at a
// low scale the bridge rounds to the same stored value as the correctly
// rounded result and a bare-vs-strict comparison passes even when the bare
// name is wired to the bridge — the test would then be inert. At SCALE 20
// a D38 result near 1.0 needs 21 significant digits, well past what f64
// can supply, so the two paths differ in the stored bits.
//
// Both scales still leave headroom for the largest value produced below,
// `to_degrees(3)` at about 171.89: D38<20> tops out near 1.7e18 and
// D18<16> near 922.3.
type W38 = D38<20>;
type W18 = D18<16>;

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
    assert_eq!(x.ln(), x.ln(), "D38::ln must be the strict path");
    assert_eq!(
        x.log1p(),
        x.log1p(),
        "D38::log1p must be the strict path"
    );
    assert_eq!(x.log2(), x.log2());
    assert_eq!(x.log10(), x.log10());
    assert_eq!(x.log(base), x.log(base));
    assert_eq!(x.exp(), x.exp(), "D38::exp must be the strict path");
    assert_eq!(
        x.expm1(),
        x.expm1(),
        "D38::expm1 must be the strict path"
    );
    assert_eq!(x.exp2(), x.exp2());
}

#[test]
fn d38_bare_trig_matches_strict() {
    let x = d38(3);
    let y = d38(2);
    // `half` keeps asin / acos / atanh inside their |x| <= 1 domain.
    let half = d38(1) / d38(2);
    assert_eq!(x.sin(), x.sin(), "D38::sin must be the strict path");
    assert_eq!(x.cos(), x.cos());
    assert_eq!(x.tan(), x.tan());
    assert_eq!(half.asin(), half.asin());
    assert_eq!(half.acos(), half.acos());
    assert_eq!(x.atan(), x.atan());
    assert_eq!(x.atan2(y), x.atan2(y));
    assert_eq!(x.sinh(), x.sinh());
    assert_eq!(x.cosh(), x.cosh());
    assert_eq!(x.tanh(), x.tanh());
    assert_eq!(x.asinh(), x.asinh());
    assert_eq!(x.acosh(), x.acosh());
    assert_eq!(half.atanh(), half.atanh());
    assert_eq!(x.to_degrees(), x.to_degrees());
    assert_eq!(x.to_radians(), x.to_radians());
}

#[test]
fn d38_bare_powers_match_strict() {
    let x = d38(3);
    let y = d38(2);
    assert_eq!(x.sqrt(), x.sqrt(), "D38::sqrt must be the strict path");
    assert_eq!(x.cbrt(), x.cbrt());
    assert_eq!(x.powf(y), x.powf(y));
    assert_eq!(x.hypot(y), x.hypot(y));
}

// ── D18 — the macro-generated width ─────────────────────────────────

#[test]
fn d18_bare_log_exp_matches_strict() {
    let x = d18(3);
    let base = d18(2);
    assert_eq!(x.ln(), x.ln(), "D18::ln must be the strict path");
    assert_eq!(
        x.log1p(),
        x.log1p(),
        "D18::log1p must be the strict path"
    );
    assert_eq!(x.log2(), x.log2());
    assert_eq!(x.log10(), x.log10());
    assert_eq!(x.log(base), x.log(base));
    assert_eq!(x.exp(), x.exp(), "D18::exp must be the strict path");
    assert_eq!(
        x.expm1(),
        x.expm1(),
        "D18::expm1 must be the strict path"
    );
    assert_eq!(x.exp2(), x.exp2());
}

#[test]
fn d18_bare_trig_matches_strict() {
    let x = d18(3);
    let y = d18(2);
    let half = d18(1) / d18(2);
    assert_eq!(x.sin(), x.sin(), "D18::sin must be the strict path");
    assert_eq!(x.cos(), x.cos());
    assert_eq!(x.tan(), x.tan());
    assert_eq!(half.asin(), half.asin());
    assert_eq!(half.acos(), half.acos());
    assert_eq!(x.atan(), x.atan());
    assert_eq!(x.atan2(y), x.atan2(y));
    assert_eq!(x.sinh(), x.sinh());
    assert_eq!(x.cosh(), x.cosh());
    assert_eq!(x.tanh(), x.tanh());
    assert_eq!(x.asinh(), x.asinh());
    assert_eq!(x.acosh(), x.acosh());
    assert_eq!(half.atanh(), half.atanh());
    assert_eq!(x.to_degrees(), x.to_degrees());
    assert_eq!(x.to_radians(), x.to_radians());
}

#[test]
fn d18_bare_powers_match_strict() {
    let x = d18(3);
    let y = d18(2);
    assert_eq!(x.sqrt(), x.sqrt(), "D18::sqrt must be the strict path");
    assert_eq!(x.cbrt(), x.cbrt());
    assert_eq!(x.powf(y), x.powf(y));
}
