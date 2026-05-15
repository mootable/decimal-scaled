//! Decimal / fixed-point backend comparison.
//!
//! Compares the crate's decimal types against established
//! decimal / fixed-point crates, on both arithmetic and
//! transcendentals:
//!
//! - `D128` — the crate's 128-bit primitive-backed decimal;
//! - `D256` — the crate's 256-bit hand-rolled-wide-integer decimal;
//! - `BnumD256` — a `bnum`-backed 256-bit decimal (benchmark baseline,
//!   see `benches/bnum/`);
//! - `rust_decimal::Decimal` — a 96-bit-mantissa decimal crate;
//! - `fixed::I64F64` — a binary fixed-point crate.
//!
//! # What this measures
//!
//! - **Arithmetic** — add / sub / mul / div across every backend. The
//!   wide (`D256`) tier is expected to be slower than the
//!   primitive-backed `D128`; this quantifies by how much.
//! - **Transcendentals** — `ln` / `exp` / `sqrt` / `sin`, comparing
//!   the crate's *lossy* (f64-bridge) and *strict* (integer-only,
//!   correctly-rounded to 0.5 ULP) variants against `rust_decimal`.
//!   The strict variants are a capability the binary fixed-point and
//!   big-integer baselines do not offer at all — `fixed` has no
//!   transcendentals, so it sits out this group.
//!
//! All baseline crates are dev-dependencies only.
//!
//! Run with: `cargo bench --features wide --bench decimal_backends`.

mod bnum;

use bnum::BnumD256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::{D128, D256};
use fixed::types::I64F64;
use rust_decimal::{Decimal, MathematicalOps};

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

/// add / sub / mul / div across every backend.
fn bench_arithmetic(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal/arith");

    four_ops!(g, "D128", D128::<12>::from_int(A), D128::<12>::from_int(B));
    four_ops!(g, "D256", D256::<12>::from_int(A as i128), D256::<12>::from_int(B as i128));
    four_ops!(g, "bnum_d256", BnumD256::<12>::from_int(A as i128), BnumD256::<12>::from_int(B as i128));
    four_ops!(g, "rust_decimal", Decimal::from(A), Decimal::from(B));
    four_ops!(g, "fixed_i64f64", I64F64::from_num(A), I64F64::from_num(B));

    g.finish();
}

/// `ln` / `exp` / `sqrt` / `sin`, comparing the crate's lossy and
/// strict variants against `rust_decimal`.
fn bench_transcendentals(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal/transc");

    // `≈ 2.345678901` in each representation.
    let ours = D128::<9>::from_bits(2_345_678_901);
    let rd = Decimal::new(2_345_678_901, 9);

    g.bench_function("D128_lossy/ln", |b| b.iter(|| black_box(ours).ln()));
    g.bench_function("D128_strict/ln", |b| b.iter(|| black_box(ours).ln_strict()));
    g.bench_function("rust_decimal/ln", |b| b.iter(|| black_box(rd).ln()));

    g.bench_function("D128_lossy/exp", |b| b.iter(|| black_box(ours).exp()));
    g.bench_function("D128_strict/exp", |b| b.iter(|| black_box(ours).exp_strict()));
    g.bench_function("rust_decimal/exp", |b| b.iter(|| black_box(rd).exp()));

    g.bench_function("D128_lossy/sqrt", |b| b.iter(|| black_box(ours).sqrt()));
    g.bench_function("D128_strict/sqrt", |b| b.iter(|| black_box(ours).sqrt_strict()));
    g.bench_function("rust_decimal/sqrt", |b| b.iter(|| black_box(rd).sqrt()));

    g.bench_function("D128_lossy/sin", |b| b.iter(|| black_box(ours).sin()));
    g.bench_function("D128_strict/sin", |b| b.iter(|| black_box(ours).sin_strict()));
    g.bench_function("rust_decimal/sin", |b| b.iter(|| black_box(rd).sin()));

    g.finish();
}

criterion_group!(benches, bench_arithmetic, bench_transcendentals);
criterion_main!(benches);
