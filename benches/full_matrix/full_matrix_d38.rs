//! Per-width `full_matrix` slice for D38 (128-bit storage).
//! Hosts the `rust_decimal` cross-crate baseline (scale-19 pinned
//! to match D38<19>).

#[macro_use]
mod full_matrix_common;

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D38;
use rust_decimal::Decimal;
#[cfg(not(feature = "strict"))]
use rust_decimal::MathematicalOps;
use std::hint::black_box;

fn bench_arith(c: &mut Criterion) {
    let mut g = c.benchmark_group("arith");
    g.sample_size(50);
    arith_block!(g, "D38_s0", D38<0>);
    arith_block!(g, "D38_s19", D38<19>);
    arith_block!(g, "D38_s38", D38<38>);
    let a = Decimal::from_i128_with_scale(20_000_000_000_000_000_000_i128, 19);
    let b = Decimal::from_i128_with_scale(10_000_000_000_000_000_000_i128, 19);
    g.bench_function("rust_decimal_s19/add", |bn| {
        bn.iter(|| black_box(a) + black_box(b))
    });
    g.bench_function("rust_decimal_s19/sub", |bn| {
        bn.iter(|| black_box(a) - black_box(b))
    });
    g.bench_function("rust_decimal_s19/mul", |bn| {
        bn.iter(|| black_box(a) * black_box(b))
    });
    g.bench_function("rust_decimal_s19/div", |bn| {
        bn.iter(|| black_box(a) / black_box(b))
    });
    g.bench_function("rust_decimal_s19/rem", |bn| {
        bn.iter(|| black_box(a) % black_box(b))
    });
    g.bench_function("rust_decimal_s19/neg", |bn| bn.iter(|| -black_box(a)));
    g.finish();
}

#[cfg(not(feature = "strict"))]
fn bench_lossy(c: &mut Criterion) {
    let mut g = c.benchmark_group("lossy");
    g.sample_size(80);
    fast_block!(g, "D38_s0", D38<0>);
    fast_block!(g, "D38_s19", D38<19>);
    fast_block!(g, "D38_s38", D38<38>);
    let r = Decimal::from(2);
    g.bench_function("rust_decimal/ln", |b| b.iter(|| black_box(r).ln()));
    g.bench_function("rust_decimal/exp", |b| b.iter(|| black_box(r).exp()));
    g.bench_function("rust_decimal/sin", |b| b.iter(|| black_box(r).sin()));
    g.bench_function("rust_decimal/sqrt", |b| b.iter(|| black_box(r).sqrt()));
    g.finish();
}

#[cfg(feature = "strict")]
fn bench_lossy(_c: &mut Criterion) {}

fn bench_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("strict");
    g.sample_size(50);
    strict_block!(g, "D38_s0", D38<0>);
    strict_block!(g, "D38_s19", D38<19>);
    strict_block!(g, "D38_s38", D38<38>);
    g.finish();
}

criterion_group!(benches, bench_arith, bench_lossy, bench_strict);
criterion_main!(benches);
