//! Per-width `full_matrix` slice for D56 (wide-tier).

#![cfg(feature = "wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D56;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D56_s0",  D56<0>);
    arith_block!(g, "D56_s28", D56<28>);
    arith_block!(g, "D56_s56", D56<56>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D56_s0",  D56<0>);
    strict_block!(g, "D56_s28", D56<28>);
    strict_block!(g, "D56_s56", D56<56>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
