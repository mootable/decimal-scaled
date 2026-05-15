//! Wide-integer backend comparison: the crate's in-tree hand-rolled
//! 256-bit integer against established big-integer crates.
//!
//! Compares `decimal_scaled::Int256` (the native hand-rolled backend)
//! against `bnum`'s `I256` and `ruint`'s `U256` on the four hot
//! operations. `bnum` and `ruint` are dev-dependencies only.
//!
//! Operands are `black_box`-ed to defeat constant folding; each
//! closure returns its result so the call cannot be optimised away.
//!
//! Run with: `cargo bench --features wide --bench wide_int_backends`.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bnum::cast::As;
use decimal_scaled::Int256;

// Representative non-trivial 256-bit operands. The same logical values
// are built into each backend so the comparison is like-for-like.
const A: i128 = 1_234_567_890_123_456_789;
const B: i128 = 987_654_321_098_765;

fn bench_add(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int/add");

    let a = A.as_::<bnum::types::I256>();
    let b = B.as_::<bnum::types::I256>();
    g.bench_function("bnum", |bn| bn.iter(|| black_box(a) + black_box(b)));

    let a = ruint::aliases::U256::from(A as u128);
    let b = ruint::aliases::U256::from(B as u128);
    g.bench_function("ruint", |bn| bn.iter(|| black_box(a) + black_box(b)));

    let a = Int256::from_i128(A);
    let b = Int256::from_i128(B);
    g.bench_function("native", |bn| bn.iter(|| black_box(a) + black_box(b)));

    g.finish();
}

fn bench_sub(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int/sub");

    let a = A.as_::<bnum::types::I256>();
    let b = B.as_::<bnum::types::I256>();
    g.bench_function("bnum", |bn| bn.iter(|| black_box(a) - black_box(b)));

    let a = ruint::aliases::U256::from(A as u128);
    let b = ruint::aliases::U256::from(B as u128);
    g.bench_function("ruint", |bn| bn.iter(|| black_box(a) - black_box(b)));

    let a = Int256::from_i128(A);
    let b = Int256::from_i128(B);
    g.bench_function("native", |bn| bn.iter(|| black_box(a) - black_box(b)));

    g.finish();
}

fn bench_mul(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int/mul");

    let a = A.as_::<bnum::types::I256>();
    let b = B.as_::<bnum::types::I256>();
    g.bench_function("bnum", |bn| bn.iter(|| black_box(a) * black_box(b)));

    let a = ruint::aliases::U256::from(A as u128);
    let b = ruint::aliases::U256::from(B as u128);
    g.bench_function("ruint", |bn| bn.iter(|| black_box(a) * black_box(b)));

    let a = Int256::from_i128(A);
    let b = Int256::from_i128(B);
    g.bench_function("native", |bn| bn.iter(|| black_box(a) * black_box(b)));

    g.finish();
}

fn bench_div(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int/div");

    let a = A.as_::<bnum::types::I256>();
    let b = B.as_::<bnum::types::I256>();
    g.bench_function("bnum", |bn| bn.iter(|| black_box(a) / black_box(b)));

    let a = ruint::aliases::U256::from(A as u128);
    let b = ruint::aliases::U256::from(B as u128);
    g.bench_function("ruint", |bn| bn.iter(|| black_box(a) / black_box(b)));

    let a = Int256::from_i128(A);
    let b = Int256::from_i128(B);
    g.bench_function("native", |bn| bn.iter(|| black_box(a) / black_box(b)));

    g.finish();
}

criterion_group!(benches, bench_add, bench_sub, bench_mul, bench_div);
criterion_main!(benches);
