//! Per-width `full_matrix` slice for D307.

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D307;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D307_s0",   D307<0>);
    arith_block!(g, "D307_s150", D307<150>);
    arith_block!(g, "D307_s307", D307<307>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D307_s0",   D307<0>);
    strict_block!(g, "D307_s150", D307<150>);
    strict_block!(g, "D307_s307", D307<307>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
