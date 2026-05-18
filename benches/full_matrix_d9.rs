//! Per-width `full_matrix` slice for D9 (32-bit storage).

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D9;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D9_s0", D9<0>);
    arith_block!(g, "D9_s5", D9<5>);
    arith_block!(g, "D9_s9", D9<9>);
    g.finish();
}

#[cfg(not(feature = "strict"))]
fn bench_lossy(c: &mut Criterion) {
    let mut g = c.benchmark_group("lossy");
    g.sample_size(80);
    fast_block!(g, "D9_s0", D9<0>);
    fast_block!(g, "D9_s5", D9<5>);
    fast_block!(g, "D9_s9", D9<9>);
    g.finish();
}

#[cfg(feature = "strict")]
fn bench_lossy(_c: &mut Criterion) {}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict");
    g.sample_size(50);
    strict_block!(g, "D9_s0", D9<0>);
    strict_block!(g, "D9_s5", D9<5>);
    strict_block!(g, "D9_s9", D9<9>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_lossy, bench_strict);
criterion_main!(benches);
