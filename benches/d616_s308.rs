//! Focused microbench for D616<308> — the mid-storage popular scale
//! on the 2048-bit storage / Int8192 working-integer tier.
//!
//! Mirrors `benches/d153_s76.rs` and `benches/d57_s20.rs`: one
//! `*_strict` call per public path that is a candidate for the
//! Tang-lookup / AGM acceleration push at D616. Filter via the
//! Criterion positional argument; the per-op group prefix is
//! `D616_s308/<op>`.

#![cfg(feature = "x-wide")]

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D616;

type D = D616<308>;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("D616_s308");
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(6));

    let half: D = D::from_int(1) / D::from_int(2);
    let one_p_half: D = D::from_int(1) + half;
    let two: D = D::from_int(2);
    let one: D = D::from_int(1);
    let three: D = D::from_int(3);

    // Warm the per-thread Tang tables so the first measured iteration
    // doesn't pay the one-shot M*ln_fixed seed cost.
    let _ = one_p_half.ln_strict();
    let _ = half.exp_strict();
    let _ = half.sinh_strict();
    let _ = half.cosh_strict();
    let _ = half.tanh_strict();

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
    g.bench_function("strict/tanh", |bn| bn.iter(|| black_box(half).tanh_strict()));

    // AGM probes — capture the data point so AGM-crossover decisions
    // at deeper tiers have a baseline. `ln_strict_agm` and
    // `exp_strict_agm` are the macro-emitted alternates next to the
    // canonical strict paths.
    g.bench_function("strict/ln_agm",  |bn| bn.iter(|| black_box(three).ln_strict_agm()));
    g.bench_function("strict/exp_agm", |bn| bn.iter(|| black_box(half).exp_strict_agm()));

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
