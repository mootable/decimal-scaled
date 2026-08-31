//! Head-to-head: AGM vs Taylor / artanh strict transcendentals at
//! D230<115> (popular mid-scale of the D230 tier).
//!
//! The AGM crossover for ln-style routines (Brent 1976) is around
//! ~300 decimal digits; D230<115> sits at ~131 working-bit decimal
//! digits (`w = 115 + 30 = 145`, so `p_bits ≈ 436`). The expectation
//! is that AGM still LOSES here against the Brent-reduced artanh
//! path. This bench captures the data point so the picture is
//! complete; promotion to a width override would require a
//! benchmarked-margin win (see the `src/support/bench_alt.rs` module
//! docs for the override policy).
//!
//! Both paths are correctly-rounded to 0.5 ULP at storage scale.
//! `ln_strict_agm` runs at the lifted working scale
//! `w' = 2·SCALE + 4` via `guard_agm`; `exp_strict_agm` takes an
//! additional `k_lift` to cover the post-Newton `x << k`
//! amplification. This bench measures pure throughput.
//!
//! Run with `cargo bench --features wide --bench agm_vs_taylor_d230`.

#![cfg(feature = "wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D230;
use std::hint::black_box;

type D = D230<115>;

// Mid-magnitude positive operand: 3.0 at SCALE=115. Same input for
// both paths so the comparison is apples-to-apples.
fn pos() -> D {
    D::from(3)
}

// Small positive operand for exp (keeps the result in range).
fn small() -> D {
    let one = D::from(1);
    let two = D::from(2);
    one / two // = 0.5
}

fn bench_ln(c: &mut Criterion) {
    let x = pos();
    let mut g = c.benchmark_group("D230_s115/ln");
    g.bench_function("artanh (canonical)", |b| {
        b.iter(|| black_box(x).ln_strict())
    });
    g.bench_function("agm", |b| b.iter(|| black_box(x).ln_strict_agm()));
    g.finish();
}

fn bench_exp(c: &mut Criterion) {
    let x = small();
    let mut g = c.benchmark_group("D230_s115/exp");
    g.bench_function("taylor (canonical)", |b| {
        b.iter(|| black_box(x).exp_strict())
    });
    g.bench_function("newton-on-agm", |b| {
        b.iter(|| black_box(x).exp_strict_agm())
    });
    g.finish();
}

criterion_group!(benches, bench_ln, bench_exp);
criterion_main!(benches);
