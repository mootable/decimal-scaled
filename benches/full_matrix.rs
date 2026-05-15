//! Comprehensive matrix bench: every decimal width × three scales
//! (smallest / midpoint / largest) × every comparable operation
//! family.
//!
//! Companion to the `docs/benchmarks.md` rewrite. Where the per-
//! topic benches (`d128_mul_div_paths`, `wide_int_backends`, etc.)
//! drill into one slice of the surface, this bench fans wide so the
//! report can show how each width and scale behaves uniformly.
//!
//! Scope per type:
//!
//! - `D32`: scales 0, 5, 9 (max).
//! - `D64`: scales 0, 9, 18 (max).
//! - `D128`: scales 0, 19, 38 (max).
//! - `D256`: scales 0, 35, 76 (max).
//! - `D512`: scales 0, 75, 153 (max).
//! - `D1024`: scales 0, 150, 307 (max).
//!
//! Three operation families:
//!
//! - **arithmetic** — `add` / `sub` / `mul` / `div` / `rem` / `neg`
//!   on every type×scale combination. Two operands `from_int(2)` and
//!   `from_int(1)` keep results in range at every scale.
//! - **strict transcendentals** — `ln_strict` / `exp_strict` /
//!   `sin_strict` / `sqrt_strict` on every type×scale combo where
//!   they're meaningful (positive argument that fits storage).
//! - **lossy transcendentals** — same four functions via the `f64`
//!   bridge. Available on D32 / D64 / D128 only — the wide tiers
//!   don't ship lossy paths.
//!
//! Cross-crate baselines:
//!
//! - `bnum_d256` — `bnum`-backed 256-bit decimal shim from
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

use bnum::BnumD256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::{D1024, D128, D256, D32, D512, D64};
use fixed::types::I64F64;
use rust_decimal::{Decimal, MathematicalOps};

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

    arith_block!(g, "D32_s0", D32<0>);
    arith_block!(g, "D32_s5", D32<5>);
    arith_block!(g, "D32_s9", D32<9>);

    arith_block!(g, "D64_s0", D64<0>);
    arith_block!(g, "D64_s9", D64<9>);
    arith_block!(g, "D64_s18", D64<18>);

    arith_block!(g, "D128_s0", D128<0>);
    arith_block!(g, "D128_s19", D128<19>);
    arith_block!(g, "D128_s38", D128<38>);

    arith_block!(g, "D256_s0", D256<0>);
    arith_block!(g, "D256_s35", D256<35>);
    arith_block!(g, "D256_s76", D256<76>);

    arith_block!(g, "D512_s0", D512<0>);
    arith_block!(g, "D512_s75", D512<75>);
    arith_block!(g, "D512_s153", D512<153>);

    arith_block!(g, "D1024_s0", D1024<0>);
    arith_block!(g, "D1024_s150", D1024<150>);
    arith_block!(g, "D1024_s307", D1024<307>);

    // Cross-crate baselines.
    {
        let a = BnumD256::<35>::from_int(2);
        let b = BnumD256::<35>::from_int(1);
        g.bench_function("bnum_d256_s35/add", |bn| bn.iter(|| black_box(a) + black_box(b)));
        g.bench_function("bnum_d256_s35/sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
        g.bench_function("bnum_d256_s35/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
        g.bench_function("bnum_d256_s35/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
        g.bench_function("bnum_d256_s35/rem", |bn| bn.iter(|| black_box(a) % black_box(b)));
        g.bench_function("bnum_d256_s35/neg", |bn| bn.iter(|| -black_box(a)));
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
// Lossy transcendentals — f64-bridge form. D32 / D64 / D128 only.
// (The wide tiers don't ship a lossy path: f64 can't carry their
// precision and the strict path is the only correct one.)
// ─────────────────────────────────────────────────────────────────────

macro_rules! lossy_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        // Arguments are picked so the result fits at every type×
        // scale combo the bench fans out across — D128<38> with
        // max ≈ 1.7 is the tightest, then D32<9> at ≈ 2.14.
        //
        // ln / sin / sqrt: arg = 1.5 (constructed as 1 + 1/2).
        //   At scale 0 the 1/2 division floors to 0 so the arg is
        //   1; ln(1) / sin(1) / sqrt(1) all hit cheap evaluations
        //   but don't panic. At scale ≥ 1 the arg is exactly 1.5
        //   and the series runs in full.
        // exp: arg = 0.5 (constructed as 1/2). At scale 0 this is
        //   0 and `exp(0) = 1` hits a fast path; at scale ≥ 1 the
        //   series runs and `exp(0.5) ≈ 1.65` clears D128<38>'s
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

    lossy_block!(g, "D32_s0", D32<0>);
    lossy_block!(g, "D32_s5", D32<5>);
    lossy_block!(g, "D32_s9", D32<9>);

    lossy_block!(g, "D64_s0", D64<0>);
    lossy_block!(g, "D64_s9", D64<9>);
    lossy_block!(g, "D64_s18", D64<18>);

    lossy_block!(g, "D128_s0", D128<0>);
    lossy_block!(g, "D128_s19", D128<19>);
    lossy_block!(g, "D128_s38", D128<38>);

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
// At D1024<307> a single strict-ln call is ~123 ms (see the
// agm_vs_taylor bench), so the wide tiers use a smaller sample size
// to keep the run finite. Functions kept to the headline four:
// ln, exp, sin, sqrt.
// ─────────────────────────────────────────────────────────────────────

macro_rules! strict_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        // Same argument convention as the lossy block: 1.5 for the
        // domain-positive series (ln / sin / sqrt), 0.5 for exp.
        // See the lossy_block doc for the reasoning.
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

    strict_block!(g, "D32_s0", D32<0>);
    strict_block!(g, "D32_s5", D32<5>);
    strict_block!(g, "D32_s9", D32<9>);

    strict_block!(g, "D64_s0", D64<0>);
    strict_block!(g, "D64_s9", D64<9>);
    strict_block!(g, "D64_s18", D64<18>);

    strict_block!(g, "D128_s0", D128<0>);
    strict_block!(g, "D128_s19", D128<19>);
    strict_block!(g, "D128_s38", D128<38>);

    g.finish();

    // Wide tiers — separate group with a smaller sample size since
    // each iter at high scales is in the µs–ms range.
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));

    strict_block!(g, "D256_s0", D256<0>);
    strict_block!(g, "D256_s35", D256<35>);
    strict_block!(g, "D256_s76", D256<76>);

    strict_block!(g, "D512_s0", D512<0>);
    strict_block!(g, "D512_s75", D512<75>);
    strict_block!(g, "D512_s153", D512<153>);

    strict_block!(g, "D1024_s0", D1024<0>);
    strict_block!(g, "D1024_s150", D1024<150>);
    strict_block!(g, "D1024_s307", D1024<307>);

    g.finish();
}

criterion_group!(benches, bench_arith, bench_lossy, bench_strict);
criterion_main!(benches);
