//! Focused microbench for D924<460> — the mid-storage popular scale
//! on the 3072-bit storage / Int12288 working-integer tier.
//!
//! Mirrors `benches/d616_s308.rs` and `benches/d153_s76.rs`: one
//! `*_strict` call per public path that is a candidate for the
//! Tang-lookup / AGM acceleration push at D924. Filter via the
//! Criterion positional argument; the per-op group prefix is
//! `D924_s460/<op>`.

#![cfg(feature = "xx-wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D924;
use std::hint::black_box;

type D = D924<460>;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("D924_s460");
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(8));

    let half: D = D::from_int(1) / D::from_int(2);
    let one_p_half: D = D::from_int(1) + half;
    let two: D = D::from_int(2);
    let one: D = D::from_int(1);
    let three: D = D::from_int(3);

    // Warm the per-thread Tang ln table so the first measured iteration
    // doesn't pay the one-shot M*ln_fixed seed cost.
    let _ = one_p_half.ln_strict();

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

    // AGM probes — captures the AGM crossover question at the deepest
    // half-width tier shipped. `ln_strict_agm` and `exp_strict_agm` are
    // the macro-emitted alternates next to the canonical strict paths.
    // NOTE: AGM accuracy degrades to ~p/2 bits beyond w ~ 30; this
    // measurement is pure throughput, the answer is not bit-equal to
    // the canonical paths.
    g.bench_function("strict/ln_agm", |bn| {
        bn.iter(|| black_box(three).ln_strict_agm())
    });
    g.bench_function("strict/exp_agm", |bn| {
        bn.iter(|| black_box(half).exp_strict_agm())
    });

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
