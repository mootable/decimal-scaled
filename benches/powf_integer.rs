//! Focused microbench for the `powf_strict` integer-exponent fast
//! path landed in `perf/powf-integer-fastpath-0.4.3`.
//!
//! Compares two integer exponents (2.0 and 3.0 — the headline cases
//! the fast path is meant to win) against two non-integer exponents
//! (0.5 and 2.5 — the slow `exp(y · ln x)` path remains the
//! sanity baseline). Each width measures both halves so any
//! regression of the non-integer case shows up as a slower
//! `powf_25` / `powf_05` cell next to the now-much-faster `powf_2`
//! / `powf_3`.
//!
//! Three widths: D38<19> (narrow tier), D76<35> (wide tier),
//! D307<150> (xx-wide tier — the worst-case-cost cell where the
//! integer fast path's ratio is largest).
//!
//! Run with:
//! ```text
//! cargo bench -p decimal-scaled --bench powf_integer \
//!     --features wide,x-wide,xx-wide
//! ```

#![cfg(all(feature = "wide", feature = "x-wide"))]

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{D38, D76, D307};

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("powf_integer");
    g.sample_size(30);
    g.measurement_time(std::time::Duration::from_secs(3));

    // ── D38<19> ────────────────────────────────────────────────────
    {
        type D = D38<19>;
        let base: D = D::from_int(2);   // 2.0
        let exp_2: D = D::from_int(2);
        let exp_3: D = D::from_int(3);
        let exp_05: D = D::from_int(1) / D::from_int(2);          // 0.5
        let exp_25: D = D::from_int(5) / D::from_int(2);          // 2.5

        g.bench_function("D38_s19/powf_2", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_2)))
        });
        g.bench_function("D38_s19/powf_3", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_3)))
        });
        g.bench_function("D38_s19/powf_05", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_05)))
        });
        g.bench_function("D38_s19/powf_25", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_25)))
        });
    }

    // ── D76<35> ────────────────────────────────────────────────────
    {
        type D = D76<35>;
        let base: D = D::from_int(2);
        let exp_2: D = D::from_int(2);
        let exp_3: D = D::from_int(3);
        let exp_05: D = D::from_int(1) / D::from_int(2);
        let exp_25: D = D::from_int(5) / D::from_int(2);

        g.bench_function("D76_s35/powf_2", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_2)))
        });
        g.bench_function("D76_s35/powf_3", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_3)))
        });
        g.bench_function("D76_s35/powf_05", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_05)))
        });
        g.bench_function("D76_s35/powf_25", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_25)))
        });
    }

    // ── D307<150> ──────────────────────────────────────────────────
    {
        type D = D307<150>;
        let base: D = D::from_int(2);
        let exp_2: D = D::from_int(2);
        let exp_3: D = D::from_int(3);
        let exp_05: D = D::from_int(1) / D::from_int(2);
        let exp_25: D = D::from_int(5) / D::from_int(2);

        g.bench_function("D307_s150/powf_2", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_2)))
        });
        g.bench_function("D307_s150/powf_3", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_3)))
        });
        g.bench_function("D307_s150/powf_05", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_05)))
        });
        g.bench_function("D307_s150/powf_25", |bn| {
            bn.iter(|| black_box(base).powf_strict(black_box(exp_25)))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
