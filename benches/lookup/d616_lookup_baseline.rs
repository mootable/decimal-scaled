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
    let half_k = D616::<S_KERNEL>::from(1) / D616::<S_KERNEL>::from(2);
    let one_p_half_k = D616::<S_KERNEL>::from(1) + half_k;
    let half_l = D616::<S_LOOKUP>::from(1) / D616::<S_LOOKUP>::from(2);
    let one_p_half_l = D616::<S_LOOKUP>::from(1) + half_l;

    // Warm both code paths (lookup table seeds, etc.).
    let _ = one_p_half_l.ln_strict();
    let _ = half_l.exp_strict();
    let _ = half_l.sinh_strict();
    let _ = half_l.cosh_strict();
    let _ = half_l.tanh_strict();

    let mut g = c.benchmark_group(label);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(5));

    g.bench_function("ln/kernel_s295", |b| {
        b.iter(|| black_box(one_p_half_k).ln_strict())
    });
    g.bench_function("ln/lookup_s308", |b| {
        b.iter(|| black_box(one_p_half_l).ln_strict())
    });

    g.bench_function("exp/kernel_s295", |b| {
        b.iter(|| black_box(half_k).exp_strict())
    });
    g.bench_function("exp/lookup_s308", |b| {
        b.iter(|| black_box(half_l).exp_strict())
    });

    g.bench_function("sinh/kernel_s295", |b| {
        b.iter(|| black_box(half_k).sinh_strict())
    });
    g.bench_function("sinh/lookup_s308", |b| {
        b.iter(|| black_box(half_l).sinh_strict())
    });

    g.bench_function("cosh/kernel_s295", |b| {
        b.iter(|| black_box(half_k).cosh_strict())
    });
    g.bench_function("cosh/lookup_s308", |b| {
        b.iter(|| black_box(half_l).cosh_strict())
    });

    g.bench_function("tanh/kernel_s295", |b| {
        b.iter(|| black_box(half_k).tanh_strict())
    });
    g.bench_function("tanh/lookup_s308", |b| {
        b.iter(|| black_box(half_l).tanh_strict())
    });

    g.finish();
}

fn bench(c: &mut Criterion) {
    bench_pair::<295, 308>(c, "D616_lookup_vs_kernel");
}

criterion_group!(benches, bench);
criterion_main!(benches);
