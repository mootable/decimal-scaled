//! Per-tier library_comparison bench for D38 (128-bit storage).
//! Run: cargo bench --bench lib_cmp_d38
//!
//! Richest peer set: rust_decimal, fastnum, fixed::I64F64, g_math,
//! bigdecimal, dashu-float, decimal-rs all show up here.

#[macro_use]
mod lib_cmp_common;

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D38;
use bigdecimal::BigDecimal;
use dashu_float::DBig;
use decimal_rs::Decimal as DecimalRs;
use fastnum::dec128;
use fixed::types::I64F64;
use g_math::canonical::{evaluate, gmath};
use rust_decimal::Decimal as RustDecimal;
use rust_decimal::MathematicalOps;

fn bench(c: &mut Criterion) {
    for &scale in &[0_usize, 19, 38] {
        let group_name = format!("lib_cmp/128bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D38::<0>::from_int(2); let b = D38::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
                g.bench_function("decimal-scaled/ln",   |bn| bn.iter(|| black_box(a).ln_strict()));
                g.bench_function("decimal-scaled/exp",  |bn| bn.iter(|| black_box(a).exp_strict()));
                g.bench_function("decimal-scaled/sin",  |bn| bn.iter(|| black_box(a).sin_strict()));
                g.bench_function("decimal-scaled/sqrt", |bn| bn.iter(|| black_box(a).sqrt_strict()));
                g.bench_function("decimal-scaled/cos",  |bn| bn.iter(|| black_box(a).cos_strict()));
                g.bench_function("decimal-scaled/tan",  |bn| bn.iter(|| black_box(a).tan_strict()));
                g.bench_function("decimal-scaled/atan", |bn| bn.iter(|| black_box(a).atan_strict()));
                g.bench_function("decimal-scaled/sinh", |bn| bn.iter(|| black_box(a).sinh_strict()));
                g.bench_function("decimal-scaled/cosh", |bn| bn.iter(|| black_box(a).cosh_strict()));
                g.bench_function("decimal-scaled/tanh", |bn| bn.iter(|| black_box(a).tanh_strict()));
            }
            19 => {
                let a = D38::<19>::from_int(2); let b = D38::<19>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
                g.bench_function("decimal-scaled/ln",   |bn| bn.iter(|| black_box(a).ln_strict()));
                g.bench_function("decimal-scaled/exp",  |bn| bn.iter(|| black_box(a).exp_strict()));
                g.bench_function("decimal-scaled/sin",  |bn| bn.iter(|| black_box(a).sin_strict()));
                g.bench_function("decimal-scaled/sqrt", |bn| bn.iter(|| black_box(a).sqrt_strict()));
                g.bench_function("decimal-scaled/cos",  |bn| bn.iter(|| black_box(a).cos_strict()));
                g.bench_function("decimal-scaled/tan",  |bn| bn.iter(|| black_box(a).tan_strict()));
                g.bench_function("decimal-scaled/atan", |bn| bn.iter(|| black_box(a).atan_strict()));
                g.bench_function("decimal-scaled/sinh", |bn| bn.iter(|| black_box(a).sinh_strict()));
                g.bench_function("decimal-scaled/cosh", |bn| bn.iter(|| black_box(a).cosh_strict()));
                g.bench_function("decimal-scaled/tanh", |bn| bn.iter(|| black_box(a).tanh_strict()));
            }
            38 => {
                let a = D38::<38>::from_bits(170_000_000_000_000_000_000_000_000_000_000_000_000_i128 / 2);
                let b = D38::<38>::from_bits(100_000_000_000_000_000_000_000_000_000_000_000_i128);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        if scale <= 28 {
            let mant_a = (2_i128) * 10_i128.pow(scale as u32);
            let mant_b = 10_i128.pow(scale as u32);
            let a = RustDecimal::from_i128_with_scale(mant_a, scale as u32);
            let b = RustDecimal::from_i128_with_scale(mant_b, scale as u32);
            arith_copy!(g, "rust_decimal", a, b);
            if scale == 19 {
                g.bench_function("rust_decimal/ln",   |bn| bn.iter(|| black_box(a).ln()));
                g.bench_function("rust_decimal/exp",  |bn| bn.iter(|| black_box(a).exp()));
                g.bench_function("rust_decimal/sin",  |bn| bn.iter(|| black_box(a).sin()));
                g.bench_function("rust_decimal/cos",  |bn| bn.iter(|| black_box(a).cos()));
                g.bench_function("rust_decimal/tan",  |bn| bn.iter(|| black_box(a).tan()));
                g.bench_function("rust_decimal/sqrt", |bn| bn.iter(|| black_box(a).sqrt()));
            }
        }

        {
            let a = dec128!(2); let b = dec128!(1);
            arith_copy!(g, "fastnum", a, b);
            if scale == 19 {
                g.bench_function("fastnum/ln",   |bn| bn.iter(|| black_box(a).ln()));
                g.bench_function("fastnum/exp",  |bn| bn.iter(|| black_box(a).exp()));
                g.bench_function("fastnum/sin",  |bn| bn.iter(|| black_box(a).sin()));
                g.bench_function("fastnum/sqrt", |bn| bn.iter(|| black_box(a).sqrt()));
                g.bench_function("fastnum/cos",  |bn| bn.iter(|| black_box(a).cos()));
                g.bench_function("fastnum/tan",  |bn| bn.iter(|| black_box(a).tan()));
                g.bench_function("fastnum/atan", |bn| bn.iter(|| black_box(a).atan()));
                g.bench_function("fastnum/sinh", |bn| bn.iter(|| black_box(a).sinh()));
                g.bench_function("fastnum/cosh", |bn| bn.iter(|| black_box(a).cosh()));
                g.bench_function("fastnum/tanh", |bn| bn.iter(|| black_box(a).tanh()));
            }
        }

        {
            let a = I64F64::from_num(2); let b = I64F64::from_num(1);
            arith_copy!(g, "fixed_i64f64", a, b);
        }

        if scale == 19 {
            g.bench_function("g_math/mul", |bn| {
                bn.iter(|| {
                    let r = evaluate(&(black_box(gmath("2.0")) * black_box(gmath("1.0"))));
                    black_box(r)
                })
            });
            g.bench_function("g_math/ln",   |bn| bn.iter(|| evaluate(&black_box(gmath("2.0")).ln())));
            g.bench_function("g_math/exp",  |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).exp())));
            g.bench_function("g_math/sin",  |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).sin())));
            g.bench_function("g_math/sqrt", |bn| bn.iter(|| evaluate(&black_box(gmath("2.0")).sqrt())));
            g.bench_function("g_math/cos",  |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).cos())));
            g.bench_function("g_math/tan",  |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).tan())));
            g.bench_function("g_math/atan", |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).atan())));
            g.bench_function("g_math/sinh", |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).sinh())));
            g.bench_function("g_math/cosh", |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).cosh())));
            g.bench_function("g_math/tanh", |bn| bn.iter(|| evaluate(&black_box(gmath("1.0")).tanh())));
        }

        {
            let a = BigDecimal::new(lib_cmp_common::num_bigint_from_two_at_scale(scale), scale as i64);
            let b = BigDecimal::new(lib_cmp_common::num_bigint_from_one_at_scale(scale), scale as i64);
            arith_clone!(g, "bigdecimal", a, b);
            if scale == 19 {
                let a2 = a.clone();
                g.bench_function("bigdecimal/sqrt", |bn| bn.iter(|| black_box(a2.clone()).sqrt()));
            }
        }

        {
            let prec = scale.max(1) as usize;
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == 19 {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln",  |bn| bn.iter(|| black_box(a2.clone()).ln()));
                g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
            }
        }

        if scale < 38 {
            let mant_a = 2_u128 * 10_u128.pow(scale as u32);
            let mant_b = 10_u128.pow(scale as u32);
            let a = DecimalRs::from_parts(mant_a, scale as i16, false).unwrap();
            let b = DecimalRs::from_parts(mant_b, scale as i16, false).unwrap();
            arith_copy!(g, "decimal-rs", a, b);
            if scale == 19 {
                g.bench_function("decimal-rs/ln",   |bn| bn.iter(|| black_box(a).ln()));
                g.bench_function("decimal-rs/exp",  |bn| bn.iter(|| black_box(a).exp()));
                g.bench_function("decimal-rs/sqrt", |bn| bn.iter(|| black_box(a).sqrt()));
            }
        }

        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
