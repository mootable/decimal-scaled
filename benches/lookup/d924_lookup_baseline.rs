//! Side-by-side: D924 `ln_strict` at SCALE 440 (outside the Tang
//! lookup band, `wide_kernel` path) vs SCALE 460 (centre of the Tang
//! lookup band). Both scales work on the same `Int<192>` working
//! integer so the per-op cost of the underlying primitives is matched;
//! the speed delta isolates the lookup-vs-kernel routing.
//!
//! exp / sinh / cosh / tanh are NOT covered: the Tang exp surface
//! dispatch was rejected at D462+ (per wave 3 measurements), so at
//! D924 those paths land on the canonical `wide_kernel` regardless of
//! SCALE band.
//!
//! Run with `cargo bench --features xx-wide --bench d924_lookup_baseline`.

#![cfg(feature = "xx-wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D924;
use std::hint::black_box;

fn bench_pair<const S_KERNEL: u32, const S_LOOKUP: u32>(c: &mut Criterion, label: &str) {
    let half_k = D924::<S_KERNEL>::try_from(1_i64).unwrap() / D924::<S_KERNEL>::try_from(2_i64).unwrap();
    let one_p_half_k = D924::<S_KERNEL>::try_from(1_i64).unwrap() + half_k;
    let half_l = D924::<S_LOOKUP>::try_from(1_i64).unwrap() / D924::<S_LOOKUP>::try_from(2_i64).unwrap();
    let one_p_half_l = D924::<S_LOOKUP>::try_from(1_i64).unwrap() + half_l;

    // Warm both code paths (lookup table seeds, etc.).
    let _ = one_p_half_l.ln_strict();
    let _ = one_p_half_k.ln_strict();

    let mut g = c.benchmark_group(label);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(8));

    g.bench_function("ln/kernel_s440", |b| {
        b.iter(|| black_box(one_p_half_k).ln_strict())
    });
    g.bench_function("ln/lookup_s460", |b| {
        b.iter(|| black_box(one_p_half_l).ln_strict())
    });

    g.finish();
}

fn bench(c: &mut Criterion) {
    bench_pair::<440, 460>(c, "D924_lookup_vs_kernel");
}

criterion_group!(benches, bench);
criterion_main!(benches);
