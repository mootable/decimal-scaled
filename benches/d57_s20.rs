//! Focused microbench for D57<20> — the primary production scale
//! for the wide-tier (financial precision, scientific measurements).
//!
//! Runs every public production-path operation that's currently a
//! candidate for SCALE 20 acceleration. Filter via Criterion's
//! positional argument; the per-op group prefix is `D57_s20/<op>`.
//!
//! Target wall-clock: ~30 s end-to-end on the bench-trial workflow
//! so iterations stay under the "<60 s focused bench" discipline.

#![cfg(feature = "wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D57;
use std::hint::black_box;

type D = D57<20>;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("D57_s20");
    g.sample_size(30);
    g.measurement_time(std::time::Duration::from_secs(3));

    // Inputs: a midrange value (=1.5) for log/sin/sqrt/cbrt, a small
    // for exp (=0.5), and a 2-arg pair for mul/div.
    let half: D = D::try_from(1).unwrap() / D::try_from(2).unwrap();
    let one_p_half: D = D::try_from(1).unwrap() + half; // 1.5
    let two: D = D::try_from(2).unwrap();
    let one: D = D::try_from(1).unwrap();

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

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
