//! Focused microbench for D153<76> — the mid-storage popular scale on
//! the 512-bit storage / `Int<16>` working-integer tier.
//!
//! Mirrors `benches/d57_s20.rs` and `benches/d76_s6.rs`: one
//! `*_strict` call per public path that is a candidate for the D153
//! SCALE 76 narrow-GUARD / Tang-lookup acceleration push. Filter
//! via the Criterion positional argument; the per-op group prefix is
//! `D153_s76/<op>`.

#![cfg(feature = "wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D153;
use std::hint::black_box;

type D = D153<76>;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("D153_s76");
    g.sample_size(20);
    g.measurement_time(std::time::Duration::from_secs(4));

    let half: D = D::from_int(1) / D::from_int(2);
    let one_p_half: D = D::from_int(1) + half;
    let two: D = D::from_int(2);
    let one: D = D::from_int(1);

    g.bench_function("arith/mul", |bn| {
        bn.iter(|| black_box(two) * black_box(one))
    });
    g.bench_function("arith/div", |bn| {
        bn.iter(|| black_box(two) / black_box(one))
    });

    g.bench_function("strict/sqrt", |bn| {
        bn.iter(|| black_box(one_p_half).sqrt_strict())
    });
    g.bench_function("strict/cbrt", |bn| {
        bn.iter(|| black_box(one_p_half).cbrt_strict())
    });
    g.bench_function("strict/ln", |bn| {
        bn.iter(|| black_box(one_p_half).ln_strict())
    });
    g.bench_function("strict/exp", |bn| bn.iter(|| black_box(half).exp_strict()));
    g.bench_function("strict/sin", |bn| {
        bn.iter(|| black_box(one_p_half).sin_strict())
    });
    g.bench_function("strict/cos", |bn| {
        bn.iter(|| black_box(one_p_half).cos_strict())
    });
    g.bench_function("strict/tan", |bn| {
        bn.iter(|| black_box(one_p_half).tan_strict())
    });
    g.bench_function("strict/atan", |bn| {
        bn.iter(|| black_box(one_p_half).atan_strict())
    });
    g.bench_function("strict/asin", |bn| {
        bn.iter(|| black_box(half).asin_strict())
    });
    g.bench_function("strict/acos", |bn| {
        bn.iter(|| black_box(half).acos_strict())
    });
    g.bench_function("strict/sinh", |bn| {
        bn.iter(|| black_box(half).sinh_strict())
    });
    g.bench_function("strict/cosh", |bn| {
        bn.iter(|| black_box(half).cosh_strict())
    });
    g.bench_function("strict/tanh", |bn| {
        bn.iter(|| black_box(half).tanh_strict())
    });

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
