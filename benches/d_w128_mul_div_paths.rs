//! Native D38 mul/div (Möller–Granlund) vs the wide-arm algorithm
//! (widen to `Int256`, multiply, divide by `10^SCALE` via the generic
//! `limbs_divmod`, narrow back).
//!
//! D38 currently uses its hand-written `mg_divide::mul_div_pow10` /
//! `div_pow10_div` path — a 256-bit schoolbook product followed by a
//! Möller–Granlund magic-number divide for `10^SCALE`. The wide tiers
//! (D76 / D115 / …) take a different path through
//! `decl_decimal_arithmetic!(wide …)`: widen to the next-up `Int*`,
//! multiply, divide by `10^SCALE` using the generic `limbs_divmod`
//! (which has hardware fast paths for divisors fitting `u64`).
//!
//! This bench applies the *wide-arm* algorithm to a D38-sized
//! problem using `Int256`, so the two paths can be compared
//! head-to-head on identical operands.
//!
//! Run with: `cargo bench --features wide --bench d_w128_mul_div_paths`.

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{D38, Int256};

/// `(a · b) / 10^SCALE`, computed wide-arm style with `Int256` as the
/// widening intermediate — mirroring `decl_decimal_arithmetic!(wide D,
/// I, Int256)`.
#[inline]
fn mul_wide_style<const SCALE: u32>(a: D38<SCALE>, b: D38<SCALE>) -> D38<SCALE> {
    let a256 = Int256::from_i128(a.to_bits());
    let b256 = Int256::from_i128(b.to_bits());
    let m = Int256::from_str_radix("10", 10)
        .expect("base-10 literal")
        .pow(SCALE);
    let r = (a256 * b256) / m;
    D38::<SCALE>::from_bits(r.to_i128_checked().expect("fits i128"))
}

/// `(a · 10^SCALE) / b`, wide-arm style.
#[inline]
fn div_wide_style<const SCALE: u32>(a: D38<SCALE>, b: D38<SCALE>) -> D38<SCALE> {
    let a256 = Int256::from_i128(a.to_bits());
    let b256 = Int256::from_i128(b.to_bits());
    let m = Int256::from_str_radix("10", 10)
        .expect("base-10 literal")
        .pow(SCALE);
    let r = (a256 * m) / b256;
    D38::<SCALE>::from_bits(r.to_i128_checked().expect("fits i128"))
}

/// Mid-range operands at SCALE = 12: comfortably above the i64
/// boundary so the widening matters, comfortably below i128::MAX so
/// nothing overflows.
const A_BITS: i128 = 1_234_567_890_123_456_789;
const B_BITS: i128 = 9_876_543_210_987;

fn bench_d38_mul(c: &mut Criterion) {
    let mut g = c.benchmark_group("d38/mul");
    let a = D38::<12>::from_bits(A_BITS);
    let b = D38::<12>::from_bits(B_BITS);

    g.bench_function("native_mg_divide", |bn| {
        bn.iter(|| black_box(a) * black_box(b))
    });
    g.bench_function("wide_arm_int256", |bn| {
        bn.iter(|| mul_wide_style(black_box(a), black_box(b)))
    });

    g.finish();
}

fn bench_d38_div(c: &mut Criterion) {
    let mut g = c.benchmark_group("d38/div");
    let a = D38::<12>::from_bits(A_BITS);
    let b = D38::<12>::from_bits(B_BITS);

    g.bench_function("native_mg_divide", |bn| {
        bn.iter(|| black_box(a) / black_box(b))
    });
    g.bench_function("wide_arm_int256", |bn| {
        bn.iter(|| div_wide_style(black_box(a), black_box(b)))
    });

    g.finish();
}

criterion_group!(benches, bench_d38_mul, bench_d38_div);
criterion_main!(benches);
