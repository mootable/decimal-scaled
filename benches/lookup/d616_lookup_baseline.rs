//! Side-by-side: D616 ln/exp/sinh/cosh/tanh at SCALE 295 (outside the
//! Tang-lookup band, wide_kernel path) vs SCALE 308 (centre of the
//! Tang-lookup band). Both scales work on the same `Int<128>` working
//! integer so the per-op cost of the underlying primitives is matched;
//! the speed delta isolates the lookup-vs-kernel routing.

#![cfg(feature = "x-wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D616;
use std::hint::black_box;

fn bench_pair<const S_KERNEL: u32, const S_LOOKUP: u32>(c: &mut Criterion, label: &str) {
    let half_k = D616::<S_KERNEL>::try_from(1_i64).unwrap() / D616::<S_KERNEL>::try_from(2_i64).unwrap();
    let one_p_half_k = D616::<S_KERNEL>::try_from(1_i64).unwrap() + half_k;
    let half_l = D616::<S_LOOKUP>::try_from(1_i64).unwrap() / D616::<S_LOOKUP>::try_from(2_i64).unwrap();
    let one_p_half_l = D616::<S_LOOKUP>::try_from(1_i64).unwrap() + half_l;

    // Warm both code paths (lookup table seeds, etc.).
    let _ = one_p_half_l.ln();
    let _ = half_l.exp();
    let _ = half_l.sinh();
    let _ = half_l.cosh();
    let _ = half_l.tanh();

    let mut g = c.benchmark_group(label);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(5));

    g.bench_function("ln/kernel_s295", |b| {
        b.iter(|| black_box(one_p_half_k).ln())
    });
    g.bench_function("ln/lookup_s308", |b| {
        b.iter(|| black_box(one_p_half_l).ln())
    });

    g.bench_function("exp/kernel_s295", |b| {
        b.iter(|| black_box(half_k).exp())
    });
    g.bench_function("exp/lookup_s308", |b| {
        b.iter(|| black_box(half_l).exp())
    });

    g.bench_function("sinh/kernel_s295", |b| {
        b.iter(|| black_box(half_k).sinh())
    });
    g.bench_function("sinh/lookup_s308", |b| {
        b.iter(|| black_box(half_l).sinh())
    });

    g.bench_function("cosh/kernel_s295", |b| {
        b.iter(|| black_box(half_k).cosh())
    });
    g.bench_function("cosh/lookup_s308", |b| {
        b.iter(|| black_box(half_l).cosh())
    });

    g.bench_function("tanh/kernel_s295", |b| {
        b.iter(|| black_box(half_k).tanh())
    });
    g.bench_function("tanh/lookup_s308", |b| {
        b.iter(|| black_box(half_l).tanh())
    });

    g.finish();
}

fn bench(c: &mut Criterion) {
    bench_pair::<295, 308>(c, "D616_lookup_vs_kernel");
}

criterion_group!(benches, bench);
criterion_main!(benches);
