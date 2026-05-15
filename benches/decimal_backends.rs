//! Decimal / fixed-point backend comparison.
//!
//! Compares the crate's decimal types against established
//! decimal / fixed-point crates on the four hot operations:
//!
//! - `D128` — the crate's 128-bit primitive-backed decimal;
//! - `D256` — the crate's 256-bit hand-rolled-wide-integer decimal;
//! - `BnumD256` — a `bnum`-backed 256-bit decimal (benchmark baseline,
//!   see `benches/bnum/`);
//! - `rust_decimal::Decimal` — a 96-bit-mantissa decimal crate;
//! - `fixed::I64F64` — a binary fixed-point crate.
//!
//! The wide (`D256`) tier is expected to be slower than the
//! primitive-backed `D128`; this bench quantifies by how much, and
//! places both against the ecosystem.
//!
//! All baseline crates are dev-dependencies only.
//!
//! Run with: `cargo bench --features wide --bench decimal_backends`.

mod bnum;

use bnum::BnumD256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::{D128, D256};
use fixed::types::I64F64;
use rust_decimal::Decimal;

const A: i64 = 1_234_567;
const B: i64 = 89_543;

macro_rules! four_ops {
    ($c:expr, $label:literal, $a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        $c.bench_function(concat!($label, "/add"), |bn| {
            bn.iter(|| black_box(a) + black_box(b))
        });
        $c.bench_function(concat!($label, "/sub"), |bn| {
            bn.iter(|| black_box(a) - black_box(b))
        });
        $c.bench_function(concat!($label, "/mul"), |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
        $c.bench_function(concat!($label, "/div"), |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
    }};
}

fn bench_decimal_backends(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal");

    four_ops!(g, "D128", D128::<12>::from_int(A), D128::<12>::from_int(B));
    four_ops!(g, "D256", D256::<12>::from_int(A as i128), D256::<12>::from_int(B as i128));
    four_ops!(g, "bnum_d256", BnumD256::<12>::from_int(A as i128), BnumD256::<12>::from_int(B as i128));
    four_ops!(g, "rust_decimal", Decimal::from(A), Decimal::from(B));
    four_ops!(g, "fixed_i64f64", I64F64::from_num(A), I64F64::from_num(B));

    g.finish();
}

criterion_group!(benches, bench_decimal_backends);
criterion_main!(benches);
