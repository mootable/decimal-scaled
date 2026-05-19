//! Focused microbench for D76<6> — a narrow-SCALE wide-tier cell
//! where the wide_transcendental guard-digit working scale
//! `w = SCALE + GUARD = 36` falls inside the MG (magic-multiply)
//! base-2^128 long-divide kernel's `1..=38` window.
//!
//! This is the per-op probe used to confirm the `round_div_pow10`
//! MG routing actually shows on hot loops; D57<20> at w=50 sits
//! above the MG range and only sees the one-shot
//! `round_to_storage_with` saving per call.

#![cfg(feature = "wide")]

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D76;

type D = D76<6>;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("D76_s6");
    g.sample_size(30);
    g.measurement_time(std::time::Duration::from_secs(3));

    let half: D = D::from_int(1) / D::from_int(2);
    let one_p_half: D = D::from_int(1) + half;
    let two: D = D::from_int(2);
    let one: D = D::from_int(1);

    g.bench_function("arith/mul", |bn| bn.iter(|| black_box(two) * black_box(one)));
    g.bench_function("arith/div", |bn| bn.iter(|| black_box(two) / black_box(one)));

    g.bench_function("strict/sqrt", |bn| bn.iter(|| black_box(one_p_half).sqrt_strict()));
    g.bench_function("strict/cbrt", |bn| bn.iter(|| black_box(one_p_half).cbrt_strict()));
    g.bench_function("strict/ln",   |bn| bn.iter(|| black_box(one_p_half).ln_strict()));
    g.bench_function("strict/exp",  |bn| bn.iter(|| black_box(half).exp_strict()));
    g.bench_function("strict/sin",  |bn| bn.iter(|| black_box(one_p_half).sin_strict()));
    g.bench_function("strict/cos",  |bn| bn.iter(|| black_box(one_p_half).cos_strict()));
    g.bench_function("strict/tan",  |bn| bn.iter(|| black_box(one_p_half).tan_strict()));
    g.bench_function("strict/atan", |bn| bn.iter(|| black_box(one_p_half).atan_strict()));
    g.bench_function("strict/asin", |bn| bn.iter(|| black_box(half).asin_strict()));
    g.bench_function("strict/acos", |bn| bn.iter(|| black_box(half).acos_strict()));
    g.bench_function("strict/sinh", |bn| bn.iter(|| black_box(half).sinh_strict()));
    g.bench_function("strict/cosh", |bn| bn.iter(|| black_box(half).cosh_strict()));

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
