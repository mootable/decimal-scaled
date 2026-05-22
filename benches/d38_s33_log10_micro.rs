//! Focused micro-bench for `D38<33>::log10_strict` before/after the
//! borrow-D57 migration. Kept tiny so a full run completes in well
//! under a minute (per the project's "micro-bench first" rule).

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D38;
use std::hint::black_box;

fn bench_d38_s33_log10(c: &mut Criterion) {
    let mut g = c.benchmark_group("d38_s33_log10_micro");
    g.sample_size(50);

    let seven: D38<33> = D38::<33>::from_i32(7);
    let two: D38<33> = D38::<33>::from_i32(2);

    g.bench_function("D38_s33_ln_strict_of_7", |bn| {
        bn.iter(|| black_box(seven).ln_strict())
    });
    g.bench_function("D38_s33_log10_strict_of_7", |bn| {
        bn.iter(|| black_box(seven).log10_strict())
    });
    g.bench_function("D38_s33_log2_strict_of_7", |bn| {
        bn.iter(|| black_box(seven).log2_strict())
    });
    g.bench_function("D38_s33_log_strict_of_7_base_2", |bn| {
        bn.iter(|| black_box(seven).log_strict(black_box(two)))
    });
    g.bench_function("D38_s33_exp2_strict_of_2", |bn| {
        bn.iter(|| black_box(two).exp2_strict())
    });
    // exp2 of a small fraction (0.3) exercises the full Taylor series,
    // not just the integer power-of-two shortcut path.
    let frac: D38<33> = D38::<33>::from_bits(300_000_000_000_000_000_000_000_000_000_000_i128);
    g.bench_function("D38_s33_exp2_strict_of_0p3", |bn| {
        bn.iter(|| black_box(frac).exp2_strict())
    });

    g.finish();
}

criterion_group!(benches, bench_d38_s33_log10);
criterion_main!(benches);
