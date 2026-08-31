//! Head-to-head: AGM vs Taylor / artanh strict transcendentals at
//! D616 across SCALE 250 / 300 / 400 / 500 — Brent's AGM ln crossover
//! is around ~300 decimal digits, so D616's scale band straddles the
//! transition. This bench captures the data so a future width-override
//! decision (AGM as the bespoke `ln` / `exp` kernel for SCALE ≥ X) has
//! a measured anchor.
//!
//! Both paths are correctly-rounded to 0.5 ULP at storage scale.
//! `ln_strict_agm` runs at the lifted working scale
//! `w' = 2·SCALE + 4` via `guard_agm`; `exp_strict_agm` takes an
//! additional `k_lift` to cover the post-Newton `x << k`
//! amplification. This bench measures pure throughput against the
//! chain-MG + narrow-GUARD artanh / Tang stack.
//!
//! Run with `cargo bench --features x-wide --bench agm_vs_taylor_d616`.

#![cfg(feature = "x-wide")]

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D616;
use std::hint::black_box;

fn bench_at<const SCALE: u32>(c: &mut Criterion, label: &str) {
    let three: D616<SCALE> = D616::<SCALE>::from(3);
    let half: D616<SCALE> = D616::<SCALE>::from(1) / D616::<SCALE>::from(2);

    // Warm both code paths (table seeds, etc.).
    let _ = three.ln_strict();
    let _ = three.ln_strict_agm();
    let _ = half.exp_strict();
    let _ = half.exp_strict_agm();

    let group_ln = format!("D616_s{}/ln", label);
    let mut g = c.benchmark_group(group_ln);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(6));
    g.bench_function("artanh (canonical)", |b| {
        b.iter(|| black_box(three).ln_strict())
    });
    g.bench_function("agm", |b| b.iter(|| black_box(three).ln_strict_agm()));
    g.finish();

    let group_exp = format!("D616_s{}/exp", label);
    let mut g = c.benchmark_group(group_exp);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(6));
    g.bench_function("taylor (canonical)", |b| {
        b.iter(|| black_box(half).exp_strict())
    });
    g.bench_function("newton-on-agm", |b| {
        b.iter(|| black_box(half).exp_strict_agm())
    });
    g.finish();
}

fn bench(c: &mut Criterion) {
    bench_at::<250>(c, "250");
    bench_at::<300>(c, "300");
    bench_at::<400>(c, "400");
    bench_at::<500>(c, "500");
}

criterion_group!(benches, bench);
criterion_main!(benches);
