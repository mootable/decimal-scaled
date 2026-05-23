//! Shared harness for the per-width `compare_d{N}` bench family.
//!
//! Side-by-side comparison of two `decimal-scaled` versions across the
//! whole public function surface, split one bench target per width so
//! each width runs as its own parallel CI job (wall time ≈ the slowest
//! single width, instead of one >90 min sweep over every width).
//!
//! `branch` = the branch under test (the `decimal_scaled` path dep);
//! `prod` = the latest published release (the baseline). Both run the
//! same functions at the same SCALE, with operands constructed via
//! `FromStr` so the harness depends only on the public surface common
//! to both versions. Each per-width target produces both columns for
//! its width; pair `<fn>/<W>/branch` against `<fn>/<W>/prod`.
//!
//! Each per-width file declares `#[macro_use] mod compare_common;` so
//! the macros are crate-local-visible without re-exporting.
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
#[macro_export]
macro_rules! funcs {
    ($c:expr, $w:literal, $side:literal, $ty:ty) => {{
        use ::std::hint::black_box;
        let x: $ty = "2.0".parse().unwrap();
        let s: $ty = "0.5".parse().unwrap();
        let b: $ty = "67.89".parse().unwrap();
        let e: $ty = "1.5".parse().unwrap(); // powf exponent
        let ten: $ty = "10.0".parse().unwrap(); // log base

        // ── arithmetic ──
        $crate::bench_one!($c, "add", $w, $side, |bn| {
            bn.iter(|| black_box(x) + black_box(b))
        });
        $crate::bench_one!($c, "sub", $w, $side, |bn| {
            bn.iter(|| black_box(x) - black_box(b))
        });
        $crate::bench_one!($c, "mul", $w, $side, |bn| {
            bn.iter(|| black_box(x) * black_box(b))
        });
        $crate::bench_one!($c, "div", $w, $side, |bn| {
            bn.iter(|| black_box(x) / black_box(b))
        });
        $crate::bench_one!($c, "rem", $w, $side, |bn| {
            bn.iter(|| black_box(x) % black_box(b))
        });
        $crate::bench_one!($c, "neg", $w, $side, |bn| bn.iter(|| -black_box(x)));

        // ── roots ──
        $crate::bench_one!($c, "sqrt", $w, $side, |bn| bn.iter(|| black_box(x).sqrt()));
        $crate::bench_one!($c, "cbrt", $w, $side, |bn| bn.iter(|| black_box(x).cbrt()));

        // ── transcendental, single argument ──
        $crate::bench_one!($c, "exp", $w, $side, |bn| bn.iter(|| black_box(s).exp()));
        $crate::bench_one!($c, "ln", $w, $side, |bn| bn.iter(|| black_box(x).ln()));
        $crate::bench_one!($c, "sin", $w, $side, |bn| bn.iter(|| black_box(s).sin()));
        $crate::bench_one!($c, "cos", $w, $side, |bn| bn.iter(|| black_box(s).cos()));
        $crate::bench_one!($c, "tan", $w, $side, |bn| bn.iter(|| black_box(s).tan()));
        $crate::bench_one!($c, "asin", $w, $side, |bn| bn.iter(|| black_box(s).asin()));
        $crate::bench_one!($c, "acos", $w, $side, |bn| bn.iter(|| black_box(s).acos()));
        $crate::bench_one!($c, "atan", $w, $side, |bn| bn.iter(|| black_box(s).atan()));
        $crate::bench_one!($c, "sinh", $w, $side, |bn| bn.iter(|| black_box(s).sinh()));
        $crate::bench_one!($c, "cosh", $w, $side, |bn| bn.iter(|| black_box(s).cosh()));
        $crate::bench_one!($c, "tanh", $w, $side, |bn| bn.iter(|| black_box(s).tanh()));
        $crate::bench_one!($c, "asinh", $w, $side, |bn| bn.iter(|| black_box(s).asinh()));
        $crate::bench_one!($c, "acosh", $w, $side, |bn| bn.iter(|| black_box(x).acosh()));
        $crate::bench_one!($c, "atanh", $w, $side, |bn| bn.iter(|| black_box(s).atanh()));
        $crate::bench_one!($c, "to_degrees", $w, $side, |bn| {
            bn.iter(|| black_box(s).to_degrees())
        });
        $crate::bench_one!($c, "to_radians", $w, $side, |bn| {
            bn.iter(|| black_box(s).to_radians())
        });

        // ── binary / other ──
        $crate::bench_one!($c, "powf", $w, $side, |bn| {
            bn.iter(|| black_box(x).powf(black_box(e)))
        });
        $crate::bench_one!($c, "log", $w, $side, |bn| {
            bn.iter(|| black_box(x).log(black_box(ten)))
        });
        // `hypot` is benched separately (D38-only) — see `hypot_d38!`.
    }};
}

/// `hypot`'s default public method (`x.hypot(y)`) is `D38`-only in both
/// versions under `strict`. Bench the Pythagorean triple `hypot(3, 4)`
/// for one `D38<SCALE>`-shaped type under the given `$side` label.
#[macro_export]
macro_rules! hypot_d38 {
    ($c:expr, $w:literal, $side:literal, $ty:ty) => {{
        use ::std::hint::black_box;
        let c3: $ty = "3.0".parse().unwrap();
        let d4: $ty = "4.0".parse().unwrap();
        $crate::bench_one!($c, "hypot", $w, $side, |bn| {
            bn.iter(|| black_box(c3).hypot(black_box(d4)))
        });
    }};
}

/// Register one function's bench into a Criterion group named
/// `<fn>/<W>`, under the row label `<side>`.
#[macro_export]
macro_rules! bench_one {
    ($c:expr, $fn:literal, $w:literal, $side:literal, $body:expr) => {{
        let mut g = $c.benchmark_group(concat!($fn, "/", $w));
        g.bench_function($side, $body);
        g.finish();
    }};
}

/// Run the full function surface for one width: the branch and prod at
/// the same SCALE, so every `<fn>/<W>` group holds a `branch` row and a
/// `prod` row. Each per-width bench target invokes this once.
///
/// Scales are kept <= 30 — proven to parse cleanly on every published
/// version (the bench-history harness uses D307<30>); width, not scale,
/// drives fixed-point arithmetic cost, so this still exercises each
/// storage tier's full limb width.
#[macro_export]
macro_rules! width_bench {
    ($w:literal, $scale:literal, $newmod:ident, $oldmod:ident) => {
        fn bench(c: &mut ::criterion::Criterion) {
            $crate::funcs!(c, $w, "branch", ::decimal_scaled::$newmod<$scale>);
            $crate::funcs!(c, $w, "prod", ::prod::$oldmod<$scale>);
        }

        ::criterion::criterion_group!(benches, bench);
        ::criterion::criterion_main!(benches);
    };
}
