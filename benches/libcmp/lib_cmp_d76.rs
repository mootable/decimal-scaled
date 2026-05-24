//! Per-tier library_comparison bench for D76 (256-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d76

#[macro_use]
mod lib_cmp_common;

use bigdecimal::BigDecimal;
use criterion::{Criterion, criterion_group, criterion_main};
use dashu_float::DBig;
use decimal_scaled::D76;
use fastnum::dec256;
use std::hint::black_box;

fn bench(c: &mut Criterion) {
    for &scale in &[0_usize, 35, 76] {
        let group_name = format!("lib_cmp/256bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D76::<0>::try_from(2).unwrap();
                let b = D76::<0>::try_from(1).unwrap();
                arith_copy!(g, "decimal-scaled", a, b);
            }
            35 => {
                let a = D76::<35>::try_from(2).unwrap();
                let b = D76::<35>::try_from(1).unwrap();
                arith_copy!(g, "decimal-scaled", a, b);
                g.bench_function("decimal-scaled/ln", |bn| {
                    bn.iter(|| black_box(a).ln_strict())
                });
                g.bench_function("decimal-scaled/exp", |bn| {
                    bn.iter(|| black_box(a).exp_strict())
                });
                g.bench_function("decimal-scaled/sin", |bn| {
                    bn.iter(|| black_box(a).sin_strict())
                });
                g.bench_function("decimal-scaled/sqrt", |bn| {
                    bn.iter(|| black_box(a).sqrt_strict())
                });
                g.bench_function("decimal-scaled/cos", |bn| {
                    bn.iter(|| black_box(a).cos_strict())
                });
                g.bench_function("decimal-scaled/tan", |bn| {
                    bn.iter(|| black_box(a).tan_strict())
                });
                g.bench_function("decimal-scaled/atan", |bn| {
                    bn.iter(|| black_box(a).atan_strict())
                });
                g.bench_function("decimal-scaled/sinh", |bn| {
                    bn.iter(|| black_box(a).sinh_strict())
                });
                g.bench_function("decimal-scaled/cosh", |bn| {
                    bn.iter(|| black_box(a).cosh_strict())
                });
                g.bench_function("decimal-scaled/tanh", |bn| {
                    bn.iter(|| black_box(a).tanh_strict())
                });
            }
            76 => {
                let a = D76::<76>::try_from(2).unwrap();
                let b = D76::<76>::try_from(1).unwrap();
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        {
            let a = dec256!(2);
            let b = dec256!(1);
            arith_copy!(g, "fastnum", a, b);
            if scale == 35 {
                g.bench_function("fastnum/ln", |bn| bn.iter(|| black_box(a).ln()));
                g.bench_function("fastnum/exp", |bn| bn.iter(|| black_box(a).exp()));
                g.bench_function("fastnum/sin", |bn| bn.iter(|| black_box(a).sin()));
                g.bench_function("fastnum/sqrt", |bn| bn.iter(|| black_box(a).sqrt()));
                g.bench_function("fastnum/cos", |bn| bn.iter(|| black_box(a).cos()));
                g.bench_function("fastnum/tan", |bn| bn.iter(|| black_box(a).tan()));
                g.bench_function("fastnum/atan", |bn| bn.iter(|| black_box(a).atan()));
                g.bench_function("fastnum/sinh", |bn| bn.iter(|| black_box(a).sinh()));
                g.bench_function("fastnum/cosh", |bn| bn.iter(|| black_box(a).cosh()));
                g.bench_function("fastnum/tanh", |bn| bn.iter(|| black_box(a).tanh()));
            }
        }

        {
            let mant_a = lib_cmp_common::num_bigint_from_two_at_scale(scale);
            let mant_b = lib_cmp_common::num_bigint_from_one_at_scale(scale);
            let a = BigDecimal::new(mant_a, scale as i64);
            let b = BigDecimal::new(mant_b, scale as i64);
            arith_clone!(g, "bigdecimal", a, b);
            if scale == 35 {
                let a2 = a.clone();
                g.bench_function("bigdecimal/sqrt", |bn| {
                    bn.iter(|| black_box(a2.clone()).sqrt())
                });
            }
        }

        {
            let prec = scale.max(1) as usize;
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == 35 {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln", |bn| {
                    bn.iter(|| black_box(a2.clone()).ln())
                });
                g.bench_function("dashu-float/exp", |bn| {
                    bn.iter(|| black_box(a2.clone()).exp())
                });
            }
        }

        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
