//! Per-tier library_comparison bench for D307 (1024-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d307

#[macro_use]
mod lib_cmp_common;

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D307;
use bigdecimal::BigDecimal;
use dashu_float::DBig;

fn bench(c: &mut Criterion) {
    for &scale in &[0_usize, 150, 307] {
        let group_name = format!("lib_cmp/1024bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D307::<0>::from_int(2); let b = D307::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            150 => {
                let a = D307::<150>::from_int(2); let b = D307::<150>::from_int(1);
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
            307 => {
                let a = D307::<307>::from_int(2); let b = D307::<307>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        {
            let mant_a = lib_cmp_common::num_bigint_from_two_at_scale(scale);
            let mant_b = lib_cmp_common::num_bigint_from_one_at_scale(scale);
            let a = BigDecimal::new(mant_a, scale as i64);
            let b = BigDecimal::new(mant_b, scale as i64);
            arith_clone!(g, "bigdecimal", a, b);
        }
        {
            let prec = scale.max(1) as usize;
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == 150 {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln",  |bn| bn.iter(|| black_box(a2.clone()).ln()));
                g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
            }
        }

        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
