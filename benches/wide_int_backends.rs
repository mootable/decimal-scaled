//! Wide-integer backend comparison: the crate's in-tree hand-rolled
//! 256/512/1024-bit integers against established big-integer crates.
//!
//! Native (`decimal_scaled::Int256` / `Int512` / `Int1024`) is compared
//! against `bnum`'s `I256` / `I512` / `I1024` and (for 256-bit only)
//! `ruint`'s `U256`. Operands are mid-magnitude so each backend
//! exercises a representative path.
//!
//! Run with: `cargo bench --features wide --bench wide_int_backends`.

use bnum::cast::As;
use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{Int256, Int512, Int1024};

const A: i128 = 1_234_567_890_123_456_789;
const B: i128 = 987_654_321_098_765;

macro_rules! four_ops_native_vs_bnum {
    ($g:expr, $width:literal, $Native:ty, $Bnum:ty) => {{
        let a_n = <$Native>::from_i128(A);
        let b_n = <$Native>::from_i128(B);
        let a_b: $Bnum = A.as_();
        let b_b: $Bnum = B.as_();

        $g.bench_function(concat!($width, "/native/add"), |bn| {
            bn.iter(|| black_box(a_n) + black_box(b_n))
        });
        $g.bench_function(concat!($width, "/bnum/add"), |bn| {
            bn.iter(|| black_box(a_b) + black_box(b_b))
        });
        $g.bench_function(concat!($width, "/native/sub"), |bn| {
            bn.iter(|| black_box(a_n) - black_box(b_n))
        });
        $g.bench_function(concat!($width, "/bnum/sub"), |bn| {
            bn.iter(|| black_box(a_b) - black_box(b_b))
        });
        $g.bench_function(concat!($width, "/native/mul"), |bn| {
            bn.iter(|| black_box(a_n) * black_box(b_n))
        });
        $g.bench_function(concat!($width, "/bnum/mul"), |bn| {
            bn.iter(|| black_box(a_b) * black_box(b_b))
        });
        $g.bench_function(concat!($width, "/native/div"), |bn| {
            bn.iter(|| black_box(a_n) / black_box(b_n))
        });
        $g.bench_function(concat!($width, "/bnum/div"), |bn| {
            bn.iter(|| black_box(a_b) / black_box(b_b))
        });
        $g.bench_function(concat!($width, "/native/rem"), |bn| {
            bn.iter(|| black_box(a_n) % black_box(b_n))
        });
        $g.bench_function(concat!($width, "/bnum/rem"), |bn| {
            bn.iter(|| black_box(a_b) % black_box(b_b))
        });
        $g.bench_function(concat!($width, "/native/neg"), |bn| {
            bn.iter(|| -black_box(a_n))
        });
        $g.bench_function(concat!($width, "/bnum/neg"), |bn| {
            bn.iter(|| -black_box(a_b))
        });
    }};
}

fn bench_int256(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int");

    // 256-bit width: native vs both bnum and ruint.
    four_ops_native_vs_bnum!(g, "Int256", Int256, bnum::types::I256);

    let a_r = ruint::aliases::U256::from(A as u128);
    let b_r = ruint::aliases::U256::from(B as u128);
    g.bench_function("Int256/ruint/add", |bn| bn.iter(|| black_box(a_r) + black_box(b_r)));
    g.bench_function("Int256/ruint/sub", |bn| bn.iter(|| black_box(a_r) - black_box(b_r)));
    g.bench_function("Int256/ruint/mul", |bn| bn.iter(|| black_box(a_r) * black_box(b_r)));
    g.bench_function("Int256/ruint/div", |bn| bn.iter(|| black_box(a_r) / black_box(b_r)));
    g.bench_function("Int256/ruint/rem", |bn| bn.iter(|| black_box(a_r) % black_box(b_r)));
    // ruint U256 has no Neg (unsigned), skip neg.

    g.finish();
}

fn bench_int512(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int");
    four_ops_native_vs_bnum!(g, "Int512", Int512, bnum::types::I512);
    g.finish();
}

fn bench_int1024(c: &mut Criterion) {
    let mut g = c.benchmark_group("wide_int");
    four_ops_native_vs_bnum!(g, "Int1024", Int1024, bnum::types::I1024);
    g.finish();
}

criterion_group!(benches, bench_int256, bench_int512, bench_int1024);
criterion_main!(benches);
