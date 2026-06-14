//! Head-to-head: AGM vs Taylor / artanh strict transcendentals at
//! D307<300>.
//!
//! Both paths are correctly-rounded to 0.5 ULP at storage scale:
//! `ln_strict_agm` and `exp_strict_agm` run at the lifted working
//! scale `w' = 2·SCALE + 4` via `guard_agm`, with `exp_strict_agm`
//! taking an additional `k_lift` to cover the post-Newton `x << k`
//! amplification. The bench measures pure throughput against the
//! chain-MG + narrow-GUARD artanh / Tang stack.
//!
//! Run with `cargo bench --features wide --bench agm_vs_taylor`.

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D307;
use std::hint::black_box;

type D = D307<300>;

// Mid-magnitude positive operand: 3.0 at SCALE=300. Same input for
// both paths so the comparison is apples-to-apples.
fn pos() -> D {
    D::try_from(3_i64).unwrap()
}

// Small positive operand for exp (keeps the result in range).
fn small() -> D {
    let one = D::try_from(1_i64).unwrap();
    let two = D::try_from(2_i64).unwrap();
    one / two // = 0.5
}

fn bench_ln(c: &mut Criterion) {
    let x = pos();
    let mut g = c.benchmark_group("D307<300>/ln");
    g.bench_function("artanh (canonical)", |b| {
        b.iter(|| black_box(x).ln_strict())
    });
    g.bench_function("agm", |b| b.iter(|| black_box(x).ln_strict_agm()));
    g.finish();
}

fn bench_exp(c: &mut Criterion) {
    let x = small();
    let mut g = c.benchmark_group("D307<300>/exp");
    g.bench_function("taylor (canonical)", |b| {
        b.iter(|| black_box(x).exp_strict())
    });
    g.bench_function("newton-on-agm", |b| {
        b.iter(|| black_box(x).exp_strict_agm())
    });
    g.finish();
}

criterion_group!(benches, bench_ln, bench_exp);
criterion_main!(benches);
