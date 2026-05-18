//! Per-width `full_matrix` slice for D115 (wide-tier).

#![cfg(feature = "wide")]

#[macro_use]
mod full_matrix_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D115;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D115_s0",   D115<0>);
    arith_block!(g, "D115_s57",  D115<57>);
    arith_block!(g, "D115_s114", D115<114>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D115_s0",   D115<0>);
    strict_block!(g, "D115_s57",  D115<57>);
    strict_block!(g, "D115_s114", D115<114>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
