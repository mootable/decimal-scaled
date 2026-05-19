//! Like-for-like cross-version benchmark.
//!
//! Same Criterion harness, same input distributions, same operations —
//! the only thing that changes across CI cells is the `decimal-scaled`
//! dependency version in `../Cargo.toml`.
//!
//! Scope is deliberately small: a directional read across versions, not
//! a full perf sweep. Expand as the API surface stabilises.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use decimal_scaled::{D38, D76, D307};

// One canonical SCALE per width — picked roughly mid-range so the
// kernel is exercised at a representative working precision rather than
// the trivial SCALE=0 path.
// Note: D307<30> rather than something deeper. The published v0.2.x /
// v0.3.x lines have a u128 ceiling in FromStr (10^39 overflows the
// intermediate); strings like "1234.5" only parse up to SCALE <= 38.
// v0.4.0+ FromStr handles arbitrary SCALE, but for cross-version
// like-for-like we cap at SCALE = 30 so every version cell can run
// the same harness.
type T38 = D38<10>;
type T76 = D76<20>;
type T307 = D307<30>;

fn d38_inputs() -> (T38, T38) {
    let a: T38 = "1234.5".parse().unwrap();
    let b: T38 = "67.89".parse().unwrap();
    (a, b)
}

fn d76_inputs() -> (T76, T76) {
    let a: T76 = "1234.5".parse().unwrap();
    let b: T76 = "67.89".parse().unwrap();
    (a, b)
}

fn d307_inputs() -> (T307, T307) {
    let a: T307 = "1234.5".parse().unwrap();
    let b: T307 = "67.89".parse().unwrap();
    (a, b)
}

fn bench_arith(c: &mut Criterion) {
    let (a38, b38) = d38_inputs();
    let (a76, b76) = d76_inputs();
    let (a307, b307) = d307_inputs();

    let mut g = c.benchmark_group("arith_add");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38) + black_box(b38)));
    g.bench_function("D76", |bn| bn.iter(|| black_box(a76) + black_box(b76)));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307) + black_box(b307)));
    g.finish();

    let mut g = c.benchmark_group("arith_mul");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38) * black_box(b38)));
    g.bench_function("D76", |bn| bn.iter(|| black_box(a76) * black_box(b76)));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307) * black_box(b307)));
    g.finish();

    let mut g = c.benchmark_group("arith_div");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38) / black_box(b38)));
    g.bench_function("D76", |bn| bn.iter(|| black_box(a76) / black_box(b76)));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307) / black_box(b307)));
    g.finish();
}

fn bench_transcendentals(c: &mut Criterion) {
    let (a38, _) = d38_inputs();
    let (a76, _) = d76_inputs();
    let (a307, _) = d307_inputs();

    let mut g = c.benchmark_group("sqrt_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).sqrt_strict()));
    g.bench_function("D76", |bn| bn.iter(|| black_box(a76).sqrt_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).sqrt_strict()));
    g.finish();

    let mut g = c.benchmark_group("ln_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).ln_strict()));
    g.bench_function("D76", |bn| bn.iter(|| black_box(a76).ln_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).ln_strict()));
    g.finish();

    let mut g = c.benchmark_group("sin_strict");
    g.bench_function("D38", |bn| bn.iter(|| black_box(a38).sin_strict()));
    g.bench_function("D76", |bn| bn.iter(|| black_box(a76).sin_strict()));
    g.bench_function("D307", |bn| bn.iter(|| black_box(a307).sin_strict()));
    g.finish();
}

criterion_group!(benches, bench_arith, bench_transcendentals);
criterion_main!(benches);
