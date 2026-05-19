//! Head-to-head: AGM vs Taylor / artanh strict transcendentals at
//! D924 across SCALE 500 / 700 / 800 / 900 — the textbook Brent AGM
//! ln crossover is ~300 decimal digits but every wider tier we've
//! shipped so far (D230 / D307 / D462 / D616) has shown AGM *losing*
//! against the chain-MG + narrow-GUARD artanh kernel through SCALE
//! 500. D924's storage (Int3072, MAX_SCALE 923) reaches 200-400
//! more digits of precision than D616 — this is the first half-width
//! tier whose **upper** working scales (700+) might tip past the
//! crossover.
//!
//! Both paths are correctly-rounded in principle, but the AGM
//! implementation runs at the canonical working scale `w = SCALE +
//! GUARD`; beyond `w ~ 30` it drops to ~p/2 bits of precision (see
//! `Dxx::ln_strict_agm` doc and `ALGORITHMS.md`). This bench measures
//! pure throughput; precision is the canonical path's contract, not
//! the AGM path's at this depth. AGM is the candidate replacement only
//! if it lands faster *and* the calling site can tolerate the looser
//! accuracy budget.
//!
//! Run with `cargo bench --features xx-wide --bench agm_vs_taylor_d924`.

#![cfg(feature = "xx-wide")]

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D924;

fn bench_at<const SCALE: u32>(c: &mut Criterion, label: &str) {
    let three: D924<SCALE> = D924::<SCALE>::from_int(3);
    let half: D924<SCALE> = D924::<SCALE>::from_int(1) / D924::<SCALE>::from_int(2);

    // Warm both code paths (table seeds, etc.).
    let _ = three.ln_strict();
    let _ = three.ln_strict_agm();
    let _ = half.exp_strict();
    let _ = half.exp_strict_agm();

    let group_ln = format!("D924_s{}/ln", label);
    let mut g = c.benchmark_group(group_ln);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(8));
    g.bench_function("artanh (canonical)", |b| b.iter(|| black_box(three).ln_strict()));
    g.bench_function("agm", |b| b.iter(|| black_box(three).ln_strict_agm()));
    g.finish();

    let group_exp = format!("D924_s{}/exp", label);
    let mut g = c.benchmark_group(group_exp);
    g.sample_size(10);
    g.measurement_time(std::time::Duration::from_secs(8));
    g.bench_function("taylor (canonical)", |b| b.iter(|| black_box(half).exp_strict()));
    g.bench_function("newton-on-agm", |b| b.iter(|| black_box(half).exp_strict_agm()));
    g.finish();
}

fn bench(c: &mut Criterion) {
    bench_at::<500>(c, "500");
    bench_at::<700>(c, "700");
    bench_at::<800>(c, "800");
    bench_at::<900>(c, "900");
}

criterion_group!(benches, bench);
criterion_main!(benches);
