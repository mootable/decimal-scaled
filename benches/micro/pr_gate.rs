//! Per-PR perf-regression micro-bench harness — a small, focused
//! `criterion` bench over the cells most likely to regress under
//! kernel tuning work.
//!
//! Deliberately small so each run stays quick; the set is kept tight
//! and focused rather than exhaustive.
//!
//! Coverage:
//!
//! - D57<20> — the project's primary D57 optimisation target band.
//!   Touches every bespoke kernel that lands at SCALE 20.
//! - D38<19> — narrow-tier regression sentinel.
//! - D307<150> — deep-storage wide-tier sentinel for wide-int
//!   regression detection.
//!
//! Functions exercised: add, mul, div, sqrt_strict, ln_strict,
//! exp_strict, sin_strict, cos_strict, atan_strict.
//!
//! When adding a new bespoke kernel for a new `(width, scale)` cell,
//! add a matching bench here so the per-PR gate covers regressions
//! against the new path.

// Plain `criterion`, the workspace's bench toolchain.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use decimal_scaled::{D38, D57, D307};

type T38 = D38<19>;
type T57 = D57<20>;
type T307 = D307<150>;

fn inputs_d38() -> (T38, T38) {
    let a: T38 = "1.5".parse().unwrap();
    let b: T38 = "2.5".parse().unwrap();
    (a, b)
}

fn inputs_d57() -> (T57, T57) {
    let a: T57 = "1.5".parse().unwrap();
    let b: T57 = "2.5".parse().unwrap();
    (a, b)
}

fn inputs_d307() -> (T307, T307) {
    let a: T307 = "1.5".parse().unwrap();
    let b: T307 = "2.5".parse().unwrap();
    (a, b)
}

fn bench_arith(c: &mut Criterion) {
    let (a38, b38) = inputs_d38();
    let (a56, b56) = inputs_d57();
    let (a307, b307) = inputs_d307();

    let mut g = c.benchmark_group("arith");
    g.bench_function("D38_add", |bn| bn.iter(|| black_box(a38) + black_box(b38)));
    g.bench_function("D38_mul", |bn| bn.iter(|| black_box(a38) * black_box(b38)));
    g.bench_function("D38_div", |bn| bn.iter(|| black_box(a38) / black_box(b38)));

    g.bench_function("D57_add", |bn| bn.iter(|| black_box(a56) + black_box(b56)));
    g.bench_function("D57_mul", |bn| bn.iter(|| black_box(a56) * black_box(b56)));
    g.bench_function("D57_div", |bn| bn.iter(|| black_box(a56) / black_box(b56)));

    g.bench_function("D307_add", |bn| {
        bn.iter(|| black_box(a307) + black_box(b307))
    });
    g.bench_function("D307_mul", |bn| {
        bn.iter(|| black_box(a307) * black_box(b307))
    });
    g.bench_function("D307_div", |bn| {
        bn.iter(|| black_box(a307) / black_box(b307))
    });
    g.finish();
}

fn bench_transcendentals(c: &mut Criterion) {
    let (a38, _) = inputs_d38();
    let (a56, _) = inputs_d57();
    let (a307, _) = inputs_d307();

    let mut g = c.benchmark_group("sqrt_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).sqrt_strict()));
    g.bench_function("D57", |bn| bn.iter(|| black_box(a56).sqrt_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).sqrt_strict()));
    g.finish();

    let mut g = c.benchmark_group("ln_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).ln_strict()));
    g.bench_function("D57", |bn| bn.iter(|| black_box(a56).ln_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).ln_strict()));
    g.finish();

    let mut g = c.benchmark_group("exp_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).exp_strict()));
    g.bench_function("D57", |bn| bn.iter(|| black_box(a56).exp_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).exp_strict()));
    g.finish();

    let mut g = c.benchmark_group("sin_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).sin_strict()));
    g.bench_function("D57", |bn| bn.iter(|| black_box(a56).sin_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).sin_strict()));
    g.finish();

    let mut g = c.benchmark_group("cos_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).cos_strict()));
    g.bench_function("D57", |bn| bn.iter(|| black_box(a56).cos_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).cos_strict()));
    g.finish();

    let mut g = c.benchmark_group("atan_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).atan_strict()));
    g.bench_function("D57", |bn| bn.iter(|| black_box(a56).atan_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).atan_strict()));
    g.finish();
}

criterion_group!(benches, bench_arith, bench_transcendentals);
criterion_main!(benches);
