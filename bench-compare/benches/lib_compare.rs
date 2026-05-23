//! Side-by-side comparison of two `decimal-scaled` versions across the
//! whole public function surface.
//!
//! `branch` = the branch under test (the `decimal_scaled` path dep);
//! `prod` = the latest published release (the baseline). Both run the
//! same functions across every shared width, with operands constructed
//! via `FromStr` so the harness depends only on the public surface
//! common to both versions. One Criterion run produces both columns;
//! pair `<fn>/<W>/branch` against `<fn>/<W>/prod`.
//!
//! Coverage (the public surface present in BOTH prod 0.4.4 and the
//! branch — i.e. prod's surface, of which the branch is a superset):
//!   * arith:               add, sub, mul, div, rem, neg
//!   * roots:               sqrt, cbrt
//!   * transcendental unary: exp, ln, sin, cos, tan, asin, acos, atan,
//!                           sinh, cosh, tanh, asinh, acosh, atanh,
//!                           to_degrees, to_radians
//!   * binary / other:      powf, log, hypot
//!
//! `hypot`'s default public method (`x.hypot(y)`) is only exposed on the
//! `D38` tier in BOTH versions under `strict` (the wider tiers expose
//! only `hypot_strict`/`hypot_strict_with`, not the plain dispatcher), so
//! it is benched at `D38` alone — the one width where the default method
//! pairs cleanly branch-vs-prod. Every other function is benched across
//! the full width set.
//!
//! Each function is exercised through its DEFAULT public method (e.g.
//! `x.sqrt()`, `x.powf(y)`); under the harness's `strict` feature these
//! resolve to the correctly-rounded strict kernels. Operands are chosen
//! to be domain-valid for every function (e.g. `asin`/`acos` in [-1,1],
//! `ln`/`log` positive, `acosh` >= 1, `atanh` in (-1,1)) so nothing
//! panics or returns a degenerate result.
//!
//! `D9` is intentionally absent — it was removed in 0.5.0, so it has no
//! counterpart on the `branch` side.

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

/// Bench every covered function for one `Copy` decimal type, under the
/// given `$side` label, grouping each function into its own Criterion
/// group so the branch/prod pair is `<fn>/<W>/<side>`.
///
/// Operands (all domain-valid for every function below):
///   * `x  = 2.0`  — positive (sqrt/cbrt/ln/log base arg/powf base),
///                   and >= 1 for `acosh`.
///   * `s  = 0.5`  — in [-1, 1] and (-1, 1): `asin`/`acos`/`atanh`;
///                   a benign small angle / argument for the periodic
///                   and hyperbolic functions and `to_*`.
///   * `b  = 67.89`, `e = 1.5`, `ten = 10.0` — second operands for the
///                   arithmetic ops, `powf` (exponent) and `log` (base).
macro_rules! funcs {
    ($c:expr, $w:literal, $side:literal, $ty:ty) => {{
        let x: $ty = "2.0".parse().unwrap();
        let s: $ty = "0.5".parse().unwrap();
        let b: $ty = "67.89".parse().unwrap();
        let e: $ty = "1.5".parse().unwrap(); // powf exponent
        let ten: $ty = "10.0".parse().unwrap(); // log base

        // ── arithmetic ──
        bench_one!($c, "add", $w, $side, |bn| {
            bn.iter(|| black_box(x) + black_box(b))
        });
        bench_one!($c, "sub", $w, $side, |bn| {
            bn.iter(|| black_box(x) - black_box(b))
        });
        bench_one!($c, "mul", $w, $side, |bn| {
            bn.iter(|| black_box(x) * black_box(b))
        });
        bench_one!($c, "div", $w, $side, |bn| {
            bn.iter(|| black_box(x) / black_box(b))
        });
        bench_one!($c, "rem", $w, $side, |bn| {
            bn.iter(|| black_box(x) % black_box(b))
        });
        bench_one!($c, "neg", $w, $side, |bn| bn.iter(|| -black_box(x)));

        // ── roots ──
        bench_one!($c, "sqrt", $w, $side, |bn| bn.iter(|| black_box(x).sqrt()));
        bench_one!($c, "cbrt", $w, $side, |bn| bn.iter(|| black_box(x).cbrt()));

        // ── transcendental, single argument ──
        bench_one!($c, "exp", $w, $side, |bn| bn.iter(|| black_box(s).exp()));
        bench_one!($c, "ln", $w, $side, |bn| bn.iter(|| black_box(x).ln()));
        bench_one!($c, "sin", $w, $side, |bn| bn.iter(|| black_box(s).sin()));
        bench_one!($c, "cos", $w, $side, |bn| bn.iter(|| black_box(s).cos()));
        bench_one!($c, "tan", $w, $side, |bn| bn.iter(|| black_box(s).tan()));
        bench_one!($c, "asin", $w, $side, |bn| bn.iter(|| black_box(s).asin()));
        bench_one!($c, "acos", $w, $side, |bn| bn.iter(|| black_box(s).acos()));
        bench_one!($c, "atan", $w, $side, |bn| bn.iter(|| black_box(s).atan()));
        bench_one!($c, "sinh", $w, $side, |bn| bn.iter(|| black_box(s).sinh()));
        bench_one!($c, "cosh", $w, $side, |bn| bn.iter(|| black_box(s).cosh()));
        bench_one!($c, "tanh", $w, $side, |bn| bn.iter(|| black_box(s).tanh()));
        bench_one!($c, "asinh", $w, $side, |bn| bn.iter(|| black_box(s).asinh()));
        bench_one!($c, "acosh", $w, $side, |bn| bn.iter(|| black_box(x).acosh()));
        bench_one!($c, "atanh", $w, $side, |bn| bn.iter(|| black_box(s).atanh()));
        bench_one!($c, "to_degrees", $w, $side, |bn| {
            bn.iter(|| black_box(s).to_degrees())
        });
        bench_one!($c, "to_radians", $w, $side, |bn| {
            bn.iter(|| black_box(s).to_radians())
        });

        // ── binary / other ──
        bench_one!($c, "powf", $w, $side, |bn| {
            bn.iter(|| black_box(x).powf(black_box(e)))
        });
        bench_one!($c, "log", $w, $side, |bn| {
            bn.iter(|| black_box(x).log(black_box(ten)))
        });
        // `hypot` is benched separately (D38-only) — see `hypot_d38!`.
    }};
}

/// `hypot`'s default public method (`x.hypot(y)`) is `D38`-only in both
/// versions under `strict`. Bench the Pythagorean triple `hypot(3, 4)`
/// for one `D38<SCALE>`-shaped type under the given `$side` label.
macro_rules! hypot_d38 {
    ($c:expr, $w:literal, $side:literal, $ty:ty) => {{
        let c3: $ty = "3.0".parse().unwrap();
        let d4: $ty = "4.0".parse().unwrap();
        bench_one!($c, "hypot", $w, $side, |bn| {
            bn.iter(|| black_box(c3).hypot(black_box(d4)))
        });
    }};
}

/// Register one function's bench into a Criterion group named
/// `<fn>/<W>`, under the row label `<side>`.
macro_rules! bench_one {
    ($c:expr, $fn:literal, $w:literal, $side:literal, $body:expr) => {{
        let mut g = $c.benchmark_group(concat!($fn, "/", $w));
        g.bench_function($side, $body);
        g.finish();
    }};
}

/// Register one width: run the full function surface for the branch and
/// for prod at the same SCALE, so every `<fn>/<W>` group holds a
/// `branch` row and a `prod` row.
macro_rules! width {
    ($c:expr, $w:literal, $scale:literal, $newmod:ident, $oldmod:ident) => {{
        funcs!($c, $w, "branch", ::decimal_scaled::$newmod<$scale>);
        funcs!($c, $w, "prod", ::prod::$oldmod<$scale>);
    }};
}

fn bench(c: &mut Criterion) {
    // Scales kept <= 30 — proven to parse cleanly on every published
    // version (the bench-history harness uses D307<30>); width, not scale,
    // drives fixed-point arithmetic cost, so this still exercises each
    // storage tier's full limb width.
    width!(c, "D18", 9, D18, D18); // base storage (no feature)
    width!(c, "D38", 10, D38, D38); // base storage
    width!(c, "D57", 20, D57, D57); // wide
    width!(c, "D76", 20, D76, D76); // wide
    width!(c, "D115", 25, D115, D115); // wide
    width!(c, "D153", 25, D153, D153); // wide
    width!(c, "D230", 30, D230, D230); // wide
    width!(c, "D307", 30, D307, D307); // x-wide
    width!(c, "D462", 30, D462, D462); // x-wide
    width!(c, "D616", 30, D616, D616); // x-wide
    width!(c, "D924", 30, D924, D924); // xx-wide
    width!(c, "D1232", 30, D1232, D1232); // xx-wide

    // `hypot`'s default public method is D38-only under `strict` in both
    // versions; pair branch-vs-prod at that one tier.
    hypot_d38!(c, "D38", "branch", ::decimal_scaled::D38<10>);
    hypot_d38!(c, "D38", "prod", ::prod::D38<10>);
}

criterion_group!(benches, bench);
criterion_main!(benches);
