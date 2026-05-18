//! Per-width `full_matrix` slice for D76. Hosts the `bnum`-backed
//! 256-bit decimal baseline (matched mantissa width).

mod bnum;

#[macro_use]
mod full_matrix_common;

use std::hint::black_box;
use bnum::BnumD76;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D76;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D76_s0",  D76<0>);
    arith_block!(g, "D76_s35", D76<35>);
    arith_block!(g, "D76_s76", D76<76>);
    let a = BnumD76::<35>::from_int(2);
    let b = BnumD76::<35>::from_int(1);
    g.bench_function("bnum_d76_s35/add", |bn| bn.iter(|| black_box(a) + black_box(b)));
    g.bench_function("bnum_d76_s35/sub", |bn| bn.iter(|| black_box(a) - black_box(b)));
    g.bench_function("bnum_d76_s35/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    g.bench_function("bnum_d76_s35/div", |bn| bn.iter(|| black_box(a) / black_box(b)));
    g.bench_function("bnum_d76_s35/rem", |bn| bn.iter(|| black_box(a) % black_box(b)));
    g.bench_function("bnum_d76_s35/neg", |bn| bn.iter(|| -black_box(a)));
    g.finish();
}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict_wide");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(5));
    strict_block!(g, "D76_s0",  D76<0>);
    strict_block!(g, "D76_s35", D76<35>);
    strict_block!(g, "D76_s76", D76<76>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_strict);
criterion_main!(benches);
