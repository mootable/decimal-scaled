//! Per-width `full_matrix` slice for D615 (x-wide tier).

#![cfg(feature = "x-wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D615;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D615_s0",   D615<0>);
    arith_block!(g, "D615_s308", D615<308>);
    arith_block!(g, "D615_s615", D615<615>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D615_s0",   D615<0>);
    strict_block!(g, "D615_s308", D615<308>);
    strict_block!(g, "D615_s615", D615<615>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
