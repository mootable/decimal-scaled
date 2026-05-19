//! Head-to-head: AGM vs Taylor / artanh strict transcendentals at
//! D1232 across SCALE 615 / 800 / 1000 / 1200 — Brent's AGM ln
//! crossover sits around ~300 decimal digits in his original
//! complexity analysis (Brent-Salamin 1976). At D616 the crossover
//! failed to fire through SCALE 500; D1232 is the deepest tier the
//! crate ships, and SCALE 800-1200 stretches well past the textbook
//! threshold. If AGM still loses here, the textbook crossover does
//! not hold for our chain-MG + narrow-GUARD artanh stack.
//!
//! Both paths are correctly-rounded in principle, but the AGM
//! implementation runs at the canonical working scale `w = SCALE +
//! GUARD`; beyond `w ~ 30` it drops to ~p/2 bits of precision (see
//! `Dxx::ln_strict_agm` doc and `ALGORITHMS.md`). This bench measures
//! pure throughput; precision is the canonical path's contract, not
//! the AGM path's at this depth.
//!
//! Run with `cargo bench --features xx-wide --bench agm_vs_taylor_d1232`.

#![cfg(feature = "xx-wide")]

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D1232;

fn bench_at<const SCALE: u32>(c: &mut Criterion, label: &str) {
    let three: D1232<SCALE> = D1232::<SCALE>::from_int(3);

    // Warm both code paths (table seeds, etc.).
    let _ = three.ln_strict();
    let _ = three.ln_strict_agm();

    let group_ln = format!("D1232_s{}/ln", label);
    let mut g = c.benchmark_group(group_ln);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(8));
    g.bench_function("artanh (canonical)", |b| b.iter(|| black_box(three).ln_strict()));
    g.bench_function("agm", |b| b.iter(|| black_box(three).ln_strict_agm()));
    g.finish();
}

fn bench(c: &mut Criterion) {
    bench_at::<615>(c, "615");
    bench_at::<800>(c, "800");
    bench_at::<1000>(c, "1000");
    bench_at::<1200>(c, "1200");
}

criterion_group!(benches, bench);
criterion_main!(benches);
