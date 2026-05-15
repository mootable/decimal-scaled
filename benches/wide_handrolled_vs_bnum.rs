//! Head-to-head benchmark of the native 256-bit decimal backend
//! against a `bnum`-backed baseline.
//!
//! `decimal-scaled`'s wide tier `D256` runs on the in-tree hand-rolled
//! integer backend. `BnumD256` (see `benches/bnum/`) is a minimal
//! `bnum`-backed decimal kept solely as a benchmark baseline — `bnum`
//! is a dev-dependency, not part of normal compilation.
//!
//! Both expose `from_int` and the four arithmetic operators. This
//! bench runs the hot operations — add, sub, mul (rescaling), div
//! (rescaling) — on each backend so they can be compared directly.
//!
//! Operands are `black_box`-ed to defeat constant folding; results are
//! returned from each closure so the call cannot be optimised away.
//!
//! Run with: `cargo bench --features wide --bench wide_handrolled_vs_bnum`.

mod bnum;

use bnum::BnumD256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::D256;

/// Representative `SCALE = 12` operands, as raw logical integers fed
/// through each backend's `from_int`. Chosen to be non-trivial and to
/// exercise the full-width multiply/divide path.
const A_INT: i128 = 1_234_567_890;
const B_INT: i128 = 9_876_543;

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/add");

    let a = BnumD256::<12>::from_int(A_INT);
    let b = BnumD256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) + black_box(b)));

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("native", |bn| bn.iter(|| black_box(a) + black_box(b)));

    group.finish();
}

fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/sub");

    let a = BnumD256::<12>::from_int(A_INT);
    let b = BnumD256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) - black_box(b)));

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("native", |bn| bn.iter(|| black_box(a) - black_box(b)));

    group.finish();
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/mul");

    let a = BnumD256::<12>::from_int(A_INT);
    let b = BnumD256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) * black_box(b)));

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("native", |bn| bn.iter(|| black_box(a) * black_box(b)));

    group.finish();
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/div");

    let a = BnumD256::<12>::from_int(A_INT);
    let b = BnumD256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) / black_box(b)));

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("native", |bn| bn.iter(|| black_box(a) / black_box(b)));

    group.finish();
}

/// Large-scale operands (`SCALE = 50`). The divisor `10^50` no longer
/// fits a 64-bit word, so `div` falls through the hardware fast paths
/// onto the bounded bit-loop — this group characterises that fallback
/// against `bnum`.
fn bench_large_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/scale50");

    let a = BnumD256::<50>::from_int(A_INT);
    let b = BnumD256::<50>::from_int(B_INT);
    group.bench_function("bnum/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    group.bench_function("bnum/div", |bn| bn.iter(|| black_box(a) / black_box(b)));

    let a = D256::<50>::from_int(A_INT);
    let b = D256::<50>::from_int(B_INT);
    group.bench_function("native/mul", |bn| bn.iter(|| black_box(a) * black_box(b)));
    group.bench_function("native/div", |bn| bn.iter(|| black_box(a) / black_box(b)));

    group.finish();
}

criterion_group!(
    benches,
    bench_add,
    bench_sub,
    bench_mul,
    bench_div,
    bench_large_scale
);
criterion_main!(benches);
