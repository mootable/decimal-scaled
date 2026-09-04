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
//! Coverage (the public surface present in BOTH prod and the branch — i.e.
//! prod's surface, of which the branch is a superset). `prod` is whatever
//! `bench-compare/Cargo.toml` pins, currently 0.5.1; name the version there,
//! not here, so this list cannot drift out of date behind the pin:
//!   * arith:               add, sub, mul, div, rem, neg
//!   * roots:               sqrt, cbrt
//!   * transcendental unary: exp, ln, log2, log10, sin, cos, tan, asin, acos,
//!                           atan, sinh, cosh, tanh, asinh, acosh, atanh,
//!                           to_degrees, to_radians
//!   * binary / other:      powf, log, hypot
//!
//! `log2` / `log10` are public API on both sides and were previously unbenched
//! anywhere in the sweep. `ln_nd` (VARIANT A below) is a second `ln` row at a
//! non-degenerate argument; it is diagnostic, not part of the prod surface
//! contract, and can be dropped without touching anything else.
//!
//! `hypot` is benched across the full width set via the integer-only
//! correctly-rounded form — the one method whose NAME differs between the
//! two sides, so `scale_funcs!` passes the ident per side:
//!
//!   * branch — `hypot`. The API reduction dropped the `_strict` suffix, so
//!     the bare name IS the correctly-rounded form at every width.
//!   * prod   — `hypot_strict`. The pinned baseline predates that rename and
//!     exposes no plain `hypot` on the wide tiers at all (its `wide_roots`
//!     emits `hypot_strict`/`hypot_strict_with` only); the plain dispatcher
//!     exists on `D38` alone, where it delegates straight to `hypot_strict`.
//!
//! Both idents therefore name the SAME kernel on their own side, which is what
//! keeps the branch÷prod ratio a like-for-like comparison. Benching it at every
//! width closes the prior coverage hole where `hypot` was benched at `D38`
//! alone, leaving a regression at any wider width invisible.
//!
//! Each function is exercised through its DEFAULT public method (e.g.
//! `x.sqrt()`, `x.powf(y)`). On the branch those bare names are the
//! correctly-rounded kernels unconditionally. On prod they resolve to the same
//! kernels because the baseline is built WITH `strict` and WITHOUT `fast` —
//! its bare names are `#[cfg(all(feature = "strict", not(feature = "fast")))]`,
//! so dropping `strict` from the `prod` dep would not slow the baseline down,
//! it would delete the entire surface and fail the build. Operands are chosen
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
/// `$hypot` is the side's spelling of the correctly-rounded hypotenuse
/// (`hypot` on the branch, `hypot_strict` on the pinned baseline — see the
/// module docs). Every other function is spelled identically on both sides.
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
///   * `base = 7.0` (→ `7`) — log base (> 0, ≠ 1); `log(2, 7)` valid. Also the
///                            `ln_nd` argument (VARIANT A): unlike `x = 2.0` it
///                            is not a power of two, so it does not collapse
///                            the log range reduction to `m = 1`.
///   * `sw = 0.1` (→ `2`)   — VARIANT B: `s` with a NON-degenerate scale-0
///                            spelling, for the wide-domain small-argument
///                            functions. Identical to `s` at every scale >= 1,
///                            so no cell except s0 moves and the S-1 analysis
///                            above is untouched — the integer spelling is
///                            reached ONLY at SCALE 0, where every tier has
///                            ample integer room (`to_degrees(2) ≈ 114.6` fits
///                            at s0; it is the S-1 cell, on `0.1`, that the
///                            single-integer-digit rule binds).
#[macro_export]
macro_rules! funcs {
    ($c:expr, $w:literal, $scale:literal, $side:literal, $ty:ty, $hypot:ident) => {{
        use ::std::hint::black_box;
        let x: $ty = $crate::op_str!($scale, "2.0", "2").parse().unwrap();
        let s: $ty = $crate::op_str!($scale, "0.1", "0").parse().unwrap();
        let b: $ty = $crate::op_str!($scale, "3.5", "3").parse().unwrap();
        let e: $ty = $crate::op_str!($scale, "1.5", "1").parse().unwrap(); // powf exponent
        let ten: $ty = $crate::op_str!($scale, "7.0", "7").parse().unwrap(); // log base
        // ── VARIANT B (revert = change this "2" back to "0") ─────────────
        // `s` above spells `0` at SCALE 0, so the whole s0 column measures
        // `f(0)` — which every small-argument function short-circuits (the s0
        // cells run 1-4 ns against thousands at s9). `sw` is the SAME operand
        // at every scale >= 1 ("0.1", so no non-s0 cell moves and no S-1
        // bound changes) and a non-degenerate `2` at SCALE 0.
        //
        // Only the wide-domain functions can take it. `asin`/`acos` are capped
        // at |v| <= 1, and `atanh` needs |v| < 1 STRICTLY — the sole integer in
        // that open interval is 0 — so those three keep `s` and their s0 cell
        // stays irreducibly degenerate at SCALE 0. That is forced by the
        // domains, not a choice.
        let sw: $ty = $crate::op_str!($scale, "0.1", "2").parse().unwrap();
        // ── end VARIANT B declaration ───────────────────────────────────
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
        $crate::bench_one!($c, "exp", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).exp()));
        $crate::bench_one!($c, "ln", $w, $scale, $side, |bn| bn.iter(|| black_box(x).ln()));
        // `log2` / `log10` — the CONSTANT-base logarithms, public API on both
        // sides and previously unbenched anywhere in the sweep. They share the
        // narrow `log` kernel MINUS the base's own `ln` series: `log(x, base)`
        // computes `ln(base)` at run time, whereas these divide by the baked
        // `wide_ln2` / `wide_ln10` constant. Read against the `log` row they
        // therefore isolate that one term.
        $crate::bench_one!($c, "log2", $w, $scale, $side, |bn| bn.iter(|| black_box(x).log2()));
        $crate::bench_one!($c, "log10", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).log10())
        });
        // ── VARIANT A (drop this one row to revert) ──────────────────────
        // `ln` above is measured at `x = 2.0`, which is DEGENERATE for the
        // logarithm: range reduction gives mantissa m = 1 exactly, so the
        // artanh argument t = (m-1)/(m+1) is 0 and the series breaks on its
        // first iteration at every working scale. That is why the `ln` row is
        // flat in scale while every other transcendental rises.
        //
        // This row re-runs `ln` at the `base` operand (7.0 → m = 1.75,
        // t = 3/11), which exercises the series properly. It ADDS a row and
        // changes nothing existing, so the historical `ln` baseline is intact.
        // 7.0 is already an operand of this harness, so it needs no new S-1
        // bound check: ln(7) ≈ 1.946 < 10.
        //
        // NAMING IS LOAD-BEARING: the `_nd` suffix is what keeps this row OUT
        // of the published Performance page. `scripts/render_docs.py`
        // (`is_diagnostic_op`) drops it at both TSV readers, because `ln_nd`
        // names no callable function. Any future row that measures a kernel
        // rather than documenting public API must carry that suffix — or add
        // its own marker there — otherwise it WILL be published. The row stays
        // in the bbc artifacts either way; only publication is filtered.
        $crate::bench_one!($c, "ln_nd", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(ten).ln())
        });
        // ── end VARIANT A ───────────────────────────────────────────────
        $crate::bench_one!($c, "sin", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).sin()));
        $crate::bench_one!($c, "cos", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).cos()));
        $crate::bench_one!($c, "tan", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).tan()));
        // `asin` / `acos` stay on `s`: capped at |v| <= 1, so SCALE 0 admits
        // only 0 or the domain edge +-1, and the edge is a pathological
        // near-boundary case rather than a representative one.
        $crate::bench_one!($c, "asin", $w, $scale, $side, |bn| bn.iter(|| black_box(s).asin()));
        $crate::bench_one!($c, "acos", $w, $scale, $side, |bn| bn.iter(|| black_box(s).acos()));
        $crate::bench_one!($c, "atan", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).atan()));
        $crate::bench_one!($c, "sinh", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).sinh()));
        $crate::bench_one!($c, "cosh", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).cosh()));
        $crate::bench_one!($c, "tanh", $w, $scale, $side, |bn| bn.iter(|| black_box(sw).tanh()));
        $crate::bench_one!($c, "asinh", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(sw).asinh())
        });
        $crate::bench_one!($c, "acosh", $w, $scale, $side, |bn| bn.iter(|| black_box(x).acosh()));
        // `atanh` stays on `s`: its domain is the OPEN interval (-1, 1), whose
        // only integer is 0, so its SCALE 0 cell cannot be de-degenerated.
        $crate::bench_one!($c, "atanh", $w, $scale, $side, |bn| bn.iter(|| black_box(s).atanh()));
        $crate::bench_one!($c, "to_degrees", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(sw).to_degrees())
        });
        $crate::bench_one!($c, "to_radians", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(sw).to_radians())
        });

        // ── binary / other ──
        $crate::bench_one!($c, "powf", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).powf(black_box(e)))
        });
        $crate::bench_one!($c, "log", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(x).log(black_box(ten)))
        });
        // `hypot` via this side's spelling of the integer-only
        // correctly-rounded form — the only `hypot` method exposed at EVERY
        // width in both versions (at D38 the plain dispatcher delegates
        // straight to it). Benched at every width × scale, closing the prior
        // D38-only coverage hole. The group id stays "hypot" on both sides so
        // the branch and prod rows still pair.
        $crate::bench_one!($c, "hypot", $w, $scale, $side, |bn| {
            bn.iter(|| black_box(c3).$hypot(black_box(d4)))
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
        // Only the hypotenuse ident differs: the branch dropped the `_strict`
        // suffix, the pinned baseline still carries it. Same kernel each side.
        //
        // WHEN THE BASELINE ROLLS PAST THE API REDUCTION (i.e. once the newest
        // published release is one built from this branch), the prod side must
        // become `hypot` too — prod will no longer have `hypot_strict`, and this
        // line is the only place that needs the edit. It is left as a plain
        // ident rather than a `#[cfg]` on purpose: a cfg arm that no build
        // exercises until the day it activates is a landmine, whereas this
        // fails at compile time with `no method named hypot_strict`, on the
        // build gate, naming its own fix. The `strict` feature on the prod dep
        // stops being added by itself (the workflow probes the pinned version),
        // so this ident is the one manual step of that release.
        $crate::funcs!($c, $w, $scale, "branch", ::decimal_scaled::$newmod<$scale>, hypot);
        $crate::funcs!($c, $w, $scale, "prod", ::prod::$oldmod<$scale>, hypot_strict);
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

        // Faster bbc turnaround (owner 2026-06-03): 0.5s warm-up + 2.5s
        // measurement (was the criterion default 3s + 5s). The branch÷prod
        // ratio is read WITHIN one run (both sides at this same config), so the
        // central ratio is config-robust; the shorter budgets trade some
        // measurement variance — watch the slow wide-tier (ms-scale) cells,
        // which warm/sample least. CLI args (--save-baseline etc.) still apply
        // on top via the macro's internal configure_from_args.
        ::criterion::criterion_group! {
            name = benches;
            config = ::criterion::Criterion::default()
                .warm_up_time(::core::time::Duration::from_millis(500))
                .measurement_time(::core::time::Duration::from_millis(2500));
            targets = bench
        }
        ::criterion::criterion_main!(benches);
    };
}
