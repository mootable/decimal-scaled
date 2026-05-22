//! Per-width `full_matrix` slice for D153.

#[macro_use]
mod full_matrix_common;

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D153;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D153_s0", D153<0>);
    arith_block!(g, "D153_s75", D153<75>);
    arith_block!(g, "D153_s153", D153<153>);
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D153_s0", D153<0>);
    strict_block!(g, "D153_s75", D153<75>);
    strict_block!(g, "D153_s153", D153<153>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
