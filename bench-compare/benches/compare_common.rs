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
//! `hypot` is benched across the full width set via its
//! `hypot_strict` method (the integer-only correctly-rounded form). The
//! plain `x.hypot(y)` default method is only exposed on the `D38` tier in
//! BOTH versions under `strict` (the wider tiers expose only
//! `hypot_strict`/`hypot_strict_with`, not the plain dispatcher), and at
//! `D38` under `strict` the plain `hypot` delegates straight to
//! `hypot_strict` (same kernel, same numbers) — so benching `hypot_strict`
//! at every width pairs cleanly branch-vs-prod everywhere AND covers the
//! D38 cell with the identical kernel the plain method would route to.
//! This closes the prior coverage hole where `hypot` was benched at `D38`
//! alone, leaving a regression at any wider width invisible.
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

/// Pick the operand string for the current SCALE: a value with a fractional
/// part can only be parsed when `SCALE >= 1` (more fractional digits than
/// `SCALE` is `ParseError::OverlongFractional`), so at `SCALE == 0` — a tier
/// in every wide width's scale set — fall back to the integer form. Both
/// forms are still domain-valid for every function below.
///
/// `$scale:literal` is a compile-time constant, so this `if` const-folds to
/// the chosen branch — no runtime cost in the benched closures (operands are
/// built once, outside `bn.iter`).
#[macro_export]
macro_rules! op_str {
    ($scale:literal, $frac:literal, $int:literal) => {
        if $scale == 0 { $int } else { $frac }
    };
}

/// Bench every covered function for one `Copy` decimal type, under the
/// given `$side` label, grouping each function into its own Criterion group
/// `<fn>_<W>_s<scale>` (branch/prod as the two rows).
///
/// Operands (all domain-valid for every function below; the scale-0 integer
/// fallback in parens via `op_str!`). EVERY operand — and every arithmetic
/// RESULT — is kept to a single integer digit (|v| < 10), because the highest
/// scale in each tier's set is `S-1`, leaving exactly ONE integer digit; a
/// 2-digit operand or product (e.g. the old `67.89`, `10.0`, or `2 * 67.89`)
/// is `ParseError::OutOfRange` / an overflow there. The op set is identical
/// across scales, so the narrow-integer-room S-1 cell dictates the choice.
///   * `x  = 2.0` (→ `2`)   — positive (sqrt/cbrt/ln/log arg/powf base),
///                            >= 1 for `acosh`; results (sqrt≈1.41, ln≈0.69,
///                            exp/cosh of `s` ≈1.1–1.7, acosh≈1.32) all < 10.
///   * `s  = 0.1` (→ `0`)   — in [-1, 1] and (-1, 1): `asin`/`acos`/`atanh`;
///                            a benign small argument for the periodic and
///                            hyperbolic functions and `to_*` (0 stays in
///                            every domain at scale 0). Kept small enough that
///                            `to_degrees(0.1) ≈ 5.73 < 10` survives the S-1
///                            single-integer-digit cell (`to_degrees(0.5)≈28.6`
///                            would overflow it).
///   * `b  = 3.5` (→ `3`)   — second arithmetic operand; `x±b`, `x*b`(=7),
///                            `x/b`, `x%b` all stay |·| < 10.
///   * `e  = 1.5` (→ `1`)   — powf exponent; `x^e = 2^1.5 ≈ 2.83 < 10`.
///   * `base = 7.0` (→ `7`) — log base (> 0, ≠ 1); `log(2, 7)` valid.
#[macro_export]
macro_rules! funcs {
    ($c:expr, $w:literal, $scale:literal, $side:literal, $ty:ty) => {{
        use ::std::hint::black_box;
        let x: $ty = $crate::op_str!($scale, "2.0", "2").parse().unwrap();
        let s: $ty = $crate::op_str!($scale, "0.1", "0").parse().unwrap();
        let b: $ty = $crate::op_str!($scale, "3.5", "3").parse().unwrap();
        let e: $ty = $crate::op_str!($scale, "1.5", "1").parse().unwrap(); // powf exponent
        let ten: $ty = $crate::op_str!($scale, "7.0", "7").parse().unwrap(); // log base
        // `hypot` operands: the 3-4-5 Pythagorean triple — both legs and the
        // result (5) stay single-integer-digit (|·| < 10), surviving the S-1 cell.
        let c3: $ty = $crate::op_str!($scale, "3.0", "3").parse().unwrap();
        let d4: $ty = $crate::op_str!($scale, "4.0", "4").parse().unwrap();

        // ── arithmetic ──
        $crate::bench_one!($c, "add", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) + black_box(b))
        });
        $crate::bench_one!($c, "sub", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) - black_box(b))
        });
        $crate::bench_one!($c, "mul", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) * black_box(b))
        });
        $crate::bench_one!($c, "div", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) / black_box(b))
        });
        $crate::bench_one!($c, "rem", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x) % black_box(b))
        });
        $crate::bench_one!($c, "neg", $w, $scale, $side, |bn| bn.iter(|| -black_box(x)));

        // ── roots ──
        $crate::bench_one!($c, "sqrt", $w, $scale, $side, |bn| bn.iter(|| black_box(x).sqrt()));
        $crate::bench_one!($c, "cbrt", $w, $scale, $side, |bn| bn.iter(|| black_box(x).cbrt()));

        // ── transcendental, single argument ──
        $crate::bench_one!($c, "exp", $w, $scale, $side, |bn| bn.iter(|| black_box(s).exp()));
        $crate::bench_one!($c, "ln", $w, $scale, $side, |bn| bn.iter(|| black_box(x).ln()));
        $crate::bench_one!($c, "sin", $w, $scale, $side, |bn| bn.iter(|| black_box(s).sin()));
        $crate::bench_one!($c, "cos", $w, $scale, $side, |bn| bn.iter(|| black_box(s).cos()));
        $crate::bench_one!($c, "tan", $w, $scale, $side, |bn| bn.iter(|| black_box(s).tan()));
        $crate::bench_one!($c, "asin", $w, $scale, $side, |bn| bn.iter(|| black_box(s).asin()));
        $crate::bench_one!($c, "acos", $w, $scale, $side, |bn| bn.iter(|| black_box(s).acos()));
        $crate::bench_one!($c, "atan", $w, $scale, $side, |bn| bn.iter(|| black_box(s).atan()));
        $crate::bench_one!($c, "sinh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).sinh()));
        $crate::bench_one!($c, "cosh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).cosh()));
        $crate::bench_one!($c, "tanh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).tanh()));
        $crate::bench_one!($c, "asinh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).asinh()));
        $crate::bench_one!($c, "acosh", $w, $scale, $side, |bn| bn.iter(|| black_box(x).acosh()));
        $crate::bench_one!($c, "atanh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).atanh()));
        $crate::bench_one!($c, "to_degrees", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(s).to_degrees())
        });
        $crate::bench_one!($c, "to_radians", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(s).to_radians())
        });

        // ── binary / other ──
        $crate::bench_one!($c, "powf", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).powf(black_box(e)))
        });
        $crate::bench_one!($c, "log", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).log(black_box(ten)))
        });
        // `hypot` via `hypot_strict` (the integer-only correctly-rounded form,
        // the only `hypot` method exposed at EVERY width in both versions; at
        // D38 the plain `hypot` delegates straight to it). Benched at every
        // width × scale, closing the prior D38-only coverage hole.
        $crate::bench_one!($c, "hypot", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(c3).hypot_strict(black_box(d4)))
        });
    }};
}

/// Register one function's bench into a Criterion group named
/// `<fn>_<W>_s<scale>`, with the row label `<side>` (branch|prod). The full
/// benched id is therefore `<fn>_<W>_s<scale>/<side>`.
///
/// A single scale is selectable by a criterion name-filter: pass
/// `--bench _s<scale>/` — the trailing `/` (the group/function separator)
/// anchors the scale so `_s30/` matches `<op>_D307_s30/branch` but NOT
/// `<op>_D307_s306/branch`.
///
/// NOTE: Criterion 0.8 LOWERCASES the on-disk report directory (report.rs
/// `.to_lowercase()`), so the dir is `exp_d307_s153`, NOT the original-case
/// id. The collator (`summarise.py`) therefore reads the canonical-case
/// `group_id` from each `benchmark.json` (which preserves `exp_D307_s153`)
/// rather than parsing the lowercased path.
#[macro_export]
macro_rules! bench_one {
    ($c:expr, $fn:literal, $w:literal, $scale:literal, $side:literal, $body:expr) => {{
        let mut g = $c.benchmark_group(concat!($fn, "_", $w, "_s", $scale));
        g.bench_function($side, $body);
        g.finish();
    }};
}

/// Run the full function surface for one width at ONE SCALE: the branch
/// and prod at that scale, so every `<fn>_<W>_s<scale>` group holds a
/// `branch` row and a `prod` row.
///
/// The op set is identical across all scales (it just re-instantiates the
/// const-generic `D###<SCALE>` types), so a width's bench file invokes this
/// once per scale in its scale set. Width drives limb width; scale drives
/// the per-tier scale-dependent cost the matcher routes on — fanning out
/// over scale surfaces scale-dependent regressions a single-scale bench hides.
#[macro_export]
macro_rules! scale_funcs {
    ($c:expr, $w:literal, $scale:literal, $newmod:ident, $oldmod:ident) => {{
        $crate::funcs!($c, $w, $scale, "branch", ::decimal_scaled::$newmod<$scale>);
        $crate::funcs!($c, $w, $scale, "prod", ::prod::$oldmod<$scale>);
    }};
}

/// Run the full function surface for one width across that tier's scale set.
/// `$scale:literal` repetition is the width's chosen scales (see the per-width
/// files); each expands to one `scale_funcs!` (branch + prod at that scale).
/// Each per-width bench target invokes this once with its scale list.
#[macro_export]
macro_rules! width_bench {
    ($w:literal, $newmod:ident, $oldmod:ident, [$($scale:literal),+ $(,)?]) => {
        fn bench(c: &mut ::criterion::Criterion) {
            $(
                $crate::scale_funcs!(c, $w, $scale, $newmod, $oldmod);
            )+
        }

        ::criterion::criterion_group!(benches, bench);
        ::criterion::criterion_main!(benches);
    };
}
