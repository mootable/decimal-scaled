//! Head-to-head: AGM vs Taylor / artanh strict transcendentals at
//! D1024<300>.
//!
//! Both paths are correctly-rounded in principle, but the AGM
//! implementation runs at the canonical working scale w = SCALE +
//! GUARD; at storage scales beyond ~30 it drops to ~p/2 bits of
//! precision (see the caveat on `D1024::ln_strict_agm` and
//! ALGORITHMS.md). The bench measures pure throughput; precision is
//! the canonical path's contract, not the AGM path's at this depth.
//!
//! Run with `cargo bench --features wide --bench agm_vs_taylor`.

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D1024;

type D = D1024<300>;

// Mid-magnitude positive operand: 3.0 at SCALE=300. Same input for
// both paths so the comparison is apples-to-apples.
fn pos() -> D {
    D::from_int(3)
}

// Small positive operand for exp (keeps the result in range).
fn small() -> D {
    let one = D::from_int(1);
    let two = D::from_int(2);
    one / two // = 0.5
}

fn bench_ln(c: &mut Criterion) {
    let x = pos();
    let mut g = c.benchmark_group("D1024<300>/ln");
    g.bench_function("artanh (canonical)", |b| {
        b.iter(|| black_box(x).ln_strict())
    });
    g.bench_function("agm", |b| b.iter(|| black_box(x).ln_strict_agm()));
    g.finish();
}

fn bench_exp(c: &mut Criterion) {
    let x = small();
    let mut g = c.benchmark_group("D1024<300>/exp");
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
