//! Per-width `full_matrix` slice for D924 (xx-wide tier).

#![cfg(feature = "xx-wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D924;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D924_s0", D924<0>);
    arith_block!(g, "D924_s461", D924<461>);
    arith_block!(g, "D924_s923", D924<923>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D924_s0", D924<0>);
    strict_block!(g, "D924_s461", D924<461>);
    strict_block!(g, "D924_s923", D924<923>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
