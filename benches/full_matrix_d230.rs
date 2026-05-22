//! Per-width `full_matrix` slice for D230 (wide-tier).

#![cfg(feature = "wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D230;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D230_s0", D230<0>);
    arith_block!(g, "D230_s115", D230<115>);
    arith_block!(g, "D230_s230", D230<230>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D230_s0", D230<0>);
    strict_block!(g, "D230_s115", D230<115>);
    strict_block!(g, "D230_s230", D230<230>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
