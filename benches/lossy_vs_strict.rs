//! Lossy (f64-bridge) vs strict (integer-only, correctly-rounded)
//! transcendentals on `D128`.
//!
//! Each transcendental ships in two forms: a lossy `f64`-bridge form
//! that round-trips through the platform intrinsic, and a `*_strict`
//! integer-only form that is correctly rounded to within 0.5 ULP. The
//! strict form buys determinism and accuracy; this bench measures what
//! it costs.
//!
//! Built with default features, so both surfaces are present.
//!
//! Run with: `cargo bench --bench lossy_vs_strict`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::D128;

/// A representative `D128<9>` operand, `≈ 2.345678901`.
fn val() -> D128<9> {
    D128::<9>::from_bits(2_345_678_901)
}

fn bench_lossy_vs_strict(c: &mut Criterion) {
    let mut g = c.benchmark_group("transcendental");
    let x = val();

    g.bench_function("ln/lossy", |b| b.iter(|| black_box(x).ln()));
    g.bench_function("ln/strict", |b| b.iter(|| black_box(x).ln_strict()));

    g.bench_function("exp/lossy", |b| b.iter(|| black_box(x).exp()));
    g.bench_function("exp/strict", |b| b.iter(|| black_box(x).exp_strict()));

    g.bench_function("sqrt/lossy", |b| b.iter(|| black_box(x).sqrt()));
    g.bench_function("sqrt/strict", |b| b.iter(|| black_box(x).sqrt_strict()));

    g.bench_function("cbrt/lossy", |b| b.iter(|| black_box(x).cbrt()));
    g.bench_function("cbrt/strict", |b| b.iter(|| black_box(x).cbrt_strict()));

    g.bench_function("sin/lossy", |b| b.iter(|| black_box(x).sin()));
    g.bench_function("sin/strict", |b| b.iter(|| black_box(x).sin_strict()));

    g.bench_function("cos/lossy", |b| b.iter(|| black_box(x).cos()));
    g.bench_function("cos/strict", |b| b.iter(|| black_box(x).cos_strict()));

    g.bench_function("atan/lossy", |b| b.iter(|| black_box(x).atan()));
    g.bench_function("atan/strict", |b| b.iter(|| black_box(x).atan_strict()));

    g.bench_function("powf/lossy", |b| {
        let y = D128::<9>::from_bits(1_500_000_000);
        b.iter(|| black_box(x).powf(black_box(y)))
    });
    g.bench_function("powf/strict", |b| {
        let y = D128::<9>::from_bits(1_500_000_000);
        b.iter(|| black_box(x).powf_strict(black_box(y)))
    });

    g.finish();
}

criterion_group!(benches, bench_lossy_vs_strict);
criterion_main!(benches);
