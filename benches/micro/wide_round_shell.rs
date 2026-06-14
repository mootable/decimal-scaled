//! Wide-tier nearest-mode rounding-shell microbench — the `sin`/`cos`/`atan`
//! fixed-overhead cells the bbc trig-s0 cluster regressed, plus a fractional
//! mid-scale control. Exercises `round_to_storage_directed_impl_g`'s nearest
//! branch (the default-mode narrowing every wide transcendental pays).
//!
//! Run pinned: `powershell.exe -NoProfile -File scripts/pin_run.ps1 -Mask 0xF00000 -Bench wide_round_shell`

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{D115, D462};
use std::hint::black_box;
use std::time::Duration;

fn micro() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .measurement_time(Duration::from_millis(400))
        .warm_up_time(Duration::from_millis(150))
}

fn bench_shell(c: &mut Criterion) {
    let z0: D115<0> = "0".parse().unwrap();
    let one: D115<0> = "1".parse().unwrap();
    let frac: D115<28> = "0.1".parse().unwrap();
    let z462: D462<0> = "0".parse().unwrap();

    let mut g = c.benchmark_group("wide_round_shell");
    g.bench_function("sin_D115s0_x0", |bn| bn.iter(|| black_box(z0).sin_strict()));
    g.bench_function("sin_D115s0_x1", |bn| bn.iter(|| black_box(one).sin_strict()));
    g.bench_function("sin_D115s28_xfrac", |bn| bn.iter(|| black_box(frac).sin_strict()));
    g.bench_function("cos_D115s0_x0", |bn| bn.iter(|| black_box(z0).cos_strict()));
    g.bench_function("atan_D115s0_x0", |bn| bn.iter(|| black_box(z0).atan_strict()));
    g.bench_function("sin_D462s0_x0", |bn| bn.iter(|| black_box(z462).sin_strict()));
    g.finish();
}

criterion_group! {
    name = benches;
    config = micro();
    targets = bench_shell
}
criterion_main!(benches);
