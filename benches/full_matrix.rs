//! Comprehensive matrix bench: every decimal width × three scales
//! (smallest / midpoint / largest) × every comparable operation
//! family.
//!
//! Companion to the `docs/benchmarks.md` rewrite. Where the per-
//! topic benches (`d_w128_mul_div_paths`, `wide_int_backends`, etc.)
//! drill into one slice of the surface, this bench fans wide so the
//! report can show how each width and scale behaves uniformly.
//!
//! Scope per type:
//!
//! - `D9`: scales 0, 5, 9 (max).
//! - `D18`: scales 0, 9, 18 (max).
//! - `D38`: scales 0, 19, 38 (max).
//! - `D76`: scales 0, 35, 76 (max).
//! - `D153`: scales 0, 75, 153 (max).
//! - `D307`: scales 0, 150, 307 (max).
//!
//! Three operation families:
//!
//! - **arithmetic** — `add` / `sub` / `mul` / `div` / `rem` / `neg`
//!   on every type×scale combination. Two operands `from_int(2)` and
//!   `from_int(1)` keep results in range at every scale.
//! - **strict transcendentals** — `ln_strict` / `exp_strict` /
//!   `sin_strict` / `sqrt_strict` on every type×scale combo where
//!   they're meaningful (positive argument that fits storage).
//! - **fast transcendentals** — same four functions via the `f64`
//!   bridge. Available on D9 / D18 / D38 only — the wide tiers
//!   don't ship lossy paths.
//!
//! Cross-crate baselines:
//!
//! - `bnum_d76` — `bnum`-backed 256-bit decimal shim from
//!   `benches/bnum/` — pure-Rust big-integer crate, used as a
//!   reference for hand-rolled wide-tier decimal arithmetic.
//! - `rust_decimal::Decimal` — 96-bit-mantissa software decimal.
//! - `fixed::I64F64` — binary fixed-point.
//! - `ruint::U256` — see `wide_int_backends.rs` for raw integer
//!   comparison; not repeated here since this bench is decimal
//!   surface specifically.
//!
//! Run with:
//! ```
//! cargo bench --features "wide x-wide" --bench full_matrix
//! ```

mod bnum;

use bnum::BnumD76;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{D307, D38, D76, D9, D153, D18};
use fixed::types::I64F64;
use rust_decimal::Decimal;
#[cfg(not(feature = "strict"))]
use rust_decimal::MathematicalOps;

// ─────────────────────────────────────────────────────────────────────
// Arithmetic — six ops × eighteen type×scale configs.
// ─────────────────────────────────────────────────────────────────────

macro_rules! arith_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        let a: $T = <$T>::from_int(2);
        let b: $T = <$T>::from_int(1);
        $g.bench_function(concat!($tag, "/add"), |bn| {
            bn.iter(|| black_box(a) + black_box(b))
        });
        $g.bench_function(concat!($tag, "/sub"), |bn| {
            bn.iter(|| black_box(a) - black_box(b))
        });
        $g.bench_function(concat!($tag, "/mul"), |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
        $g.bench_function(concat!($tag, "/div"), |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        $g.bench_function(concat!($tag, "/rem"), |bn| {
            bn.iter(|| black_box(a) % black_box(b))
        });
        $g.bench_function(concat!($tag, "/neg"), |bn| {
            bn.iter(|| -black_box(a))
        });
    }};
}

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);

    arith_block!(g, "D9_s0", D9<0>);
    arith_block!(g, "D9_s5", D9<5>);
    arith_block!(g, "D9_s9", D9<9>);

    arith_block!(g, "D18_s0", D18<0>);
    arith_block!(g, "D18_s9", D18<9>);
    arith_block!(g, "D18_s18", D18<18>);

    arith_block!(g, "D38_s0", D38<0>);
    arith_block!(g, "D38_s19", D38<19>);
    arith_block!(g, "D38_s38", D38<38>);

    arith_block!(g, "D76_s0", D76<0>);
    arith_block!(g, "D76_s35", D76<35>);
    arith_block!(g, "D76_s76", D76<76>);

    arith_block!(g, "D153_s0", D153<0>);
    arith_block!(g, "D153_s75", D153<75>);
    arith_block!(g, "D153_s153", D153<153>);

    arith_block!(g, "D307_s0", D307<0>);
    arith_block!(g, "D307_s150", D307<150>);
    arith_block!(g, "D307_s307", D307<307>);

    // Cross-crate baselines.
    {
        let a = BnumD76::<35>::from_int(2);
        let b = BnumD76::<35>::from_int(1);
        g.bench_function("bnum_d76_s35/add", |bn| bn.iter(|| black_box(a) + black_box(b)));
        g.bench_function("bnum_d76_s35/sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
        g.bench_function("bnum_d76_s35/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
        g.bench_function("bnum_d76_s35/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("bnum_d76_s35/rem", |bn| bn.iter(|| black_box(a) % black_box(b)));
        g.bench_function("bnum_d76_s35/neg", |bn| bn.iter(|| -black_box(a)));
    }
    {
        let a = Decimal::from(2);
        let b = Decimal::from(1);
        g.bench_function("rust_decimal/add", |bn| bn.iter(|| black_box(a) + black_box(b)));
        g.bench_function("rust_decimal/sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
        g.bench_function("rust_decimal/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
        g.bench_function("rust_decimal/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("rust_decimal/rem", |bn| bn.iter(|| black_box(a) % black_box(b)));
        g.bench_function("rust_decimal/neg", |bn| bn.iter(|| -black_box(a)));
    }
    {
        let a = I64F64::from_num(2);
        let b = I64F64::from_num(1);
        g.bench_function("fixed_i64f64/add", |bn| bn.iter(|| black_box(a) + black_box(b)));
        g.bench_function("fixed_i64f64/sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
        g.bench_function("fixed_i64f64/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
        g.bench_function("fixed_i64f64/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("fixed_i64f64/rem", |bn| bn.iter(|| black_box(a) % black_box(b)));
        g.bench_function("fixed_i64f64/neg", |bn| bn.iter(|| -black_box(a)));
    }

    g.finish();
}

// ─────────────────────────────────────────────────────────────────────
// Fast transcendentals — f64-bridge form. D9 / D18 / D38 only.
// (The wide tiers don't ship a fast path: f64 can't carry their
// precision and the strict path is the only correct one.)
// ─────────────────────────────────────────────────────────────────────

#[cfg(not(feature = "strict"))]
macro_rules! fast_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        // Arguments are picked so the result fits at every type×
        // scale combo the bench fans out across — D38<38> with
        // max ≈ 1.7 is the tightest, then D9<9> at ≈ 2.14.
        //
        // ln / sin / sqrt: arg = 1.5 (constructed as 1 + 1/2).
        //   At scale 0 the 1/2 division floors to 0 so the arg is
        //   1; ln(1) / sin(1) / sqrt(1) all hit cheap evaluations
        //   but don't panic. At scale ≥ 1 the arg is exactly 1.5
        //   and the series runs in full.
        // exp: arg = 0.5 (constructed as 1/2). At scale 0 this is
        //   0 and `exp(0) = 1` hits a fast path; at scale ≥ 1 the
        //   series runs and `exp(0.5) ≈ 1.65` clears D38<38>'s
        //   1.7 ceiling.
        let half: $T = <$T>::from_int(1) / <$T>::from_int(2);
        let x: $T = <$T>::from_int(1) + half;
        let xh: $T = half;
        $g.bench_function(concat!($tag, "/ln"),   |bn| bn.iter(|| black_box(x).ln()));
        $g.bench_function(concat!($tag, "/exp"),  |bn| bn.iter(|| black_box(xh).exp()));
        $g.bench_function(concat!($tag, "/sin"),  |bn| bn.iter(|| black_box(x).sin()));
        $g.bench_function(concat!($tag, "/sqrt"), |bn| bn.iter(|| black_box(x).sqrt()));
    }};
}

#[cfg(not(feature = "strict"))]
fn bench_lossy(c: &mut Criterion) {
    let mut g = c.benchmark_group("lossy");
    g.sample_size(80);

    fast_block!(g, "D9_s0", D9<0>);
    fast_block!(g, "D9_s5", D9<5>);
    fast_block!(g, "D9_s9", D9<9>);

    fast_block!(g, "D18_s0", D18<0>);
    fast_block!(g, "D18_s9", D18<9>);
    fast_block!(g, "D18_s18", D18<18>);

    fast_block!(g, "D38_s0", D38<0>);
    fast_block!(g, "D38_s19", D38<19>);
    fast_block!(g, "D38_s38", D38<38>);

    {
        let r = Decimal::from(2);
        g.bench_function("rust_decimal/ln",   |b| b.iter(|| black_box(r).ln()));
        g.bench_function("rust_decimal/exp",  |b| b.iter(|| black_box(r).exp()));
        g.bench_function("rust_decimal/sin",  |b| b.iter(|| black_box(r).sin()));
        g.bench_function("rust_decimal/sqrt", |b| b.iter(|| black_box(r).sqrt()));
    }

    g.finish();
}

#[cfg(feature = "strict")]
fn bench_lossy(_c: &mut Criterion) {
    // With `strict` the canonical `ln`/`exp`/etc. dispatch to the
    // strict path; skip this group so the rows aren't a duplicate
    // of the strict numbers.
}

// ─────────────────────────────────────────────────────────────────────
// Strict transcendentals — integer-only, correctly-rounded.
//
// At D307<307> a single strict-ln call is ~123 ms (see the
// agm_vs_taylor bench), so the wide tiers use a smaller sample size
// to keep the run finite. Functions kept to the headline four:
// ln, exp, sin, sqrt.
// ─────────────────────────────────────────────────────────────────────

macro_rules! strict_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        // Same argument convention as the fast block: 1.5 for the
        // domain-positive series (ln / sin / sqrt), 0.5 for exp.
        // See the fast_block doc for the reasoning.
        let half: $T = <$T>::from_int(1) / <$T>::from_int(2);
        let x: $T = <$T>::from_int(1) + half;
        let xh: $T = half;
        $g.bench_function(concat!($tag, "/ln"),   |bn| bn.iter(|| black_box(x).ln_strict()));
        $g.bench_function(concat!($tag, "/exp"),  |bn| bn.iter(|| black_box(xh).exp_strict()));
        $g.bench_function(concat!($tag, "/sin"),  |bn| bn.iter(|| black_box(x).sin_strict()));
        $g.bench_function(concat!($tag, "/sqrt"), |bn| bn.iter(|| black_box(x).sqrt_strict()));
    }};
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict");
    g.sample_size(50);

    strict_block!(g, "D9_s0", D9<0>);
    strict_block!(g, "D9_s5", D9<5>);
    strict_block!(g, "D9_s9", D9<9>);

    strict_block!(g, "D18_s0", D18<0>);
    strict_block!(g, "D18_s9", D18<9>);
    strict_block!(g, "D18_s18", D18<18>);

    strict_block!(g, "D38_s0", D38<0>);
    strict_block!(g, "D38_s19", D38<19>);
    strict_block!(g, "D38_s38", D38<38>);

    g.finish();

    // Wide tiers — separate group with a smaller sample size since
    // each iter at high scales is in the µs–ms range.
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));

    strict_block!(g, "D76_s0", D76<0>);
    strict_block!(g, "D76_s35", D76<35>);
    strict_block!(g, "D76_s76", D76<76>);

    strict_block!(g, "D153_s0", D153<0>);
    strict_block!(g, "D153_s75", D153<75>);
    strict_block!(g, "D153_s153", D153<153>);

    strict_block!(g, "D307_s0", D307<0>);
    strict_block!(g, "D307_s150", D307<150>);
    strict_block!(g, "D307_s307", D307<307>);

    g.finish();
}

criterion_group!(benches, bench_arith, bench_lossy, bench_strict);
criterion_main!(benches);
