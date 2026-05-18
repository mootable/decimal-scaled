//! Per-width `full_matrix` slice for D18 (64-bit storage).
//! Hosts the `fixed::I64F64` cross-crate baseline (matched
//! native 64-bit storage).

#[macro_use]
mod full_matrix_common;

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D18;
use fixed::types::I64F64;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D18_s0", D18<0>);
    arith_block!(g, "D18_s9", D18<9>);
    arith_block!(g, "D18_s18", D18<18>);
    let a = I64F64::from_num(2);
    let b = I64F64::from_num(1);
    g.bench_function("fixed_i64f64/add", |bn| bn.iter(|| black_box(a) + black_box(b)));
    g.bench_function("fixed_i64f64/sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
    g.bench_function("fixed_i64f64/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    g.bench_function("fixed_i64f64/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
    g.bench_function("fixed_i64f64/rem", |bn| bn.iter(|| black_box(a) % black_box(b)));
    g.bench_function("fixed_i64f64/neg", |bn| bn.iter(|| -black_box(a)));
    g.finish();
}

#[cfg(not(feature = "strict"))]
fn bench_lossy(c: &mut Criterion) {
    let mut g = c.benchmark_group("lossy");
    g.sample_size(80);
    fast_block!(g, "D18_s0", D18<0>);
    fast_block!(g, "D18_s9", D18<9>);
    fast_block!(g, "D18_s18", D18<18>);
    g.finish();
}

#[cfg(feature = "strict")]
fn bench_lossy(_c: &mut Criterion) {}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict");
    g.sample_size(50);
    strict_block!(g, "D18_s0", D18<0>);
    strict_block!(g, "D18_s9", D18<9>);
    strict_block!(g, "D18_s18", D18<18>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_lossy, bench_strict);
criterion_main!(benches);
