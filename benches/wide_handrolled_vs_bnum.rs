//! Head-to-head benchmark of the two 256-bit decimal backends.
//!
//! `decimal-scaled` ships the 256-bit wide tier in two implementations:
//!
//! - `D256` — backed by the `bnum` big-integer crate;
//! - `D256H` — backed by the in-tree hand-rolled `HInt256` (a
//!   two's-complement `[u128; 2]` reusing the crate's own 256×256→512
//!   multiply and 512/256 divide).
//!
//! Both expose the same core decimal surface. This bench runs the four
//! hot operations — add, sub, mul (rescaling), div (rescaling) — on
//! each backend at `SCALE = 12`, so the two can be compared directly.
//!
//! Operands are `black_box`-ed to defeat constant folding; results are
//! returned from each closure so the call cannot be optimised away.
//!
//! Run with: `cargo bench --features wide --bench wide_handrolled_vs_bnum`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::{D256, D256H};

/// Representative `SCALE = 12` operands, as raw logical integers fed
/// through each backend's `from_int`. Chosen to be non-trivial and to
/// exercise the full-width multiply/divide path.
const A_INT: i128 = 1_234_567_890;
const B_INT: i128 = 9_876_543;

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/add");

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) + black_box(b)));

    let a = D256H::<12>::from_int(A_INT);
    let b = D256H::<12>::from_int(B_INT);
    group.bench_function("handrolled", |bn| bn.iter(|| black_box(a) + black_box(b)));

    group.finish();
}

fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/sub");

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) - black_box(b)));

    let a = D256H::<12>::from_int(A_INT);
    let b = D256H::<12>::from_int(B_INT);
    group.bench_function("handrolled", |bn| bn.iter(|| black_box(a) - black_box(b)));

    group.finish();
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/mul");

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) * black_box(b)));

    let a = D256H::<12>::from_int(A_INT);
    let b = D256H::<12>::from_int(B_INT);
    group.bench_function("handrolled", |bn| bn.iter(|| black_box(a) * black_box(b)));

    group.finish();
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide256/div");

    let a = D256::<12>::from_int(A_INT);
    let b = D256::<12>::from_int(B_INT);
    group.bench_function("bnum", |bn| bn.iter(|| black_box(a) / black_box(b)));

    let a = D256H::<12>::from_int(A_INT);
    let b = D256H::<12>::from_int(B_INT);
    group.bench_function("handrolled", |bn| bn.iter(|| black_box(a) / black_box(b)));

    group.finish();
}

criterion_group!(benches, bench_add, bench_sub, bench_mul, bench_div);
criterion_main!(benches);
