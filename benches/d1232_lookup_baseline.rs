//! Side-by-side: D1232 ln at SCALE 600 (outside the Tang-lookup band,
//! wide_kernel path) vs SCALE 615 (centre of the Tang-lookup band).
//! Both scales work on the same `Int<256>` working integer so the per-op
//! cost of the underlying primitives is matched; the speed delta
//! isolates the lookup-vs-kernel routing.
//!
//! Filter via Criterion: `D1232_s615/ln_lookup` / `D1232_s615/ln_kernel`.

#![cfg(feature = "xx-wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D1232;
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    let half_k = D1232::<600>::try_from(1).unwrap() / D1232::<600>::try_from(2).unwrap();
    let one_p_half_k = D1232::<600>::try_from(1).unwrap() + half_k;
    let half_l = D1232::<615>::try_from(1).unwrap() / D1232::<615>::try_from(2).unwrap();
    let one_p_half_l = D1232::<615>::try_from(1).unwrap() + half_l;

    // Warm the per-thread Tang ln table seed at w = 623.
    let _ = one_p_half_l.ln_strict();

    let mut g = c.benchmark_group("D1232_s615");
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(8));

    g.bench_function("ln_kernel_s600", |b| {
        b.iter(|| black_box(one_p_half_k).ln_strict())
    });
    g.bench_function("ln_lookup_s615", |b| {
        b.iter(|| black_box(one_p_half_l).ln_strict())
    });

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
