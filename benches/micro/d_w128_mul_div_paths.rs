//! Native D38 mul/div (Möller–Granlund) vs the wide-arm algorithm
//! (widen to `Int<4>`, multiply, divide by `10^SCALE` via the generic
//! `limbs_divmod`, narrow back).
//!
//! D38 currently uses its hand-written `mg_divide::mul_div_pow10` /
//! `div_pow10_div` path — a 256-bit schoolbook product followed by a
//! Möller–Granlund magic-number divide for `10^SCALE`. The wide tiers
//! (D76 / D115 / …) take a different path through
//! `decl_decimal_arithmetic!(wide …)`: widen to the next-up `Int<N>`,
//! multiply, divide by `10^SCALE` using the generic `limbs_divmod`
//! (which has hardware fast paths for divisors fitting `u64`).
//!
//! This bench applies the *wide-arm* algorithm to a D38-sized
//! problem using `Int<4>`, so the two paths can be compared
//! head-to-head on identical operands.
//!
//! Run with: `cargo bench --features wide --bench d_w128_mul_div_paths`.

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::{D38, Int};
use std::hint::black_box;

/// `(a · b) / 10^SCALE`, computed wide-arm style with `Int<4>` as the
/// widening intermediate — mirroring `decl_decimal_arithmetic!(wide D,
/// I, Int<4>)`.
#[inline]
fn mul_wide_style<const SCALE: u32>(a: D38<SCALE>, b: D38<SCALE>) -> D38<SCALE> {
    let a256 = a.to_bits().widen::<4>();
    let b256 = b.to_bits().widen::<4>();
    let m = Int::<4>::from_str_radix("10", 10)
        .expect("base-10 literal")
        .pow(SCALE);
    let r = (a256 * b256) / m;
    D38::<SCALE>::from_bits(Int::<2>::try_from(r.to_i128_checked().expect("fits i128")).unwrap())
}

/// `(a · 10^SCALE) / b`, wide-arm style.
#[inline]
fn div_wide_style<const SCALE: u32>(a: D38<SCALE>, b: D38<SCALE>) -> D38<SCALE> {
    let a256 = a.to_bits().widen::<4>();
    let b256 = b.to_bits().widen::<4>();
    let m = Int::<4>::from_str_radix("10", 10)
        .expect("base-10 literal")
        .pow(SCALE);
    let r = (a256 * m) / b256;
    D38::<SCALE>::from_bits(Int::<2>::try_from(r.to_i128_checked().expect("fits i128")).unwrap())
}

/// Mid-range operands at SCALE = 12: comfortably above the i64
/// boundary so the widening matters, comfortably below i128::MAX so
/// nothing overflows.
const A_BITS: i128 = 1_234_567_890_123_456_789;
const B_BITS: i128 = 9_876_543_210_987;

fn bench_d38_mul(c: &mut Criterion) {
    let mut g = c.benchmark_group("d38/mul");
    let a = D38::<12>::from_bits(Int::<2>::try_from(A_BITS).unwrap());
    let b = D38::<12>::from_bits(Int::<2>::try_from(B_BITS).unwrap());

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
    let a = D38::<12>::from_bits(Int::<2>::try_from(A_BITS).unwrap());
    let b = D38::<12>::from_bits(Int::<2>::try_from(B_BITS).unwrap());

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
