//! Per-width `full_matrix` slice for D462 (x-wide tier).

#![cfg(feature = "x-wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D462;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D462_s0",   D462<0>);
    arith_block!(g, "D462_s230", D462<230>);
    arith_block!(g, "D462_s461", D462<461>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D462_s0",   D462<0>);
    strict_block!(g, "D462_s230", D462<230>);
    strict_block!(g, "D462_s461", D462<461>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
