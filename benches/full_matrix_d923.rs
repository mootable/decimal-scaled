//! Per-width `full_matrix` slice for D923 (xx-wide tier).

#![cfg(feature = "xx-wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D923;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D923_s0",   D923<0>);
    arith_block!(g, "D923_s461", D923<461>);
    arith_block!(g, "D923_s923", D923<923>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D923_s0",   D923<0>);
    strict_block!(g, "D923_s461", D923<461>);
    strict_block!(g, "D923_s923", D923<923>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
