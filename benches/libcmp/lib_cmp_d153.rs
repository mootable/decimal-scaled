//! Per-tier library_comparison bench for D153 (512-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d153
//!      cargo bench --features "wide" --bench lib_cmp_d153 -- _s76/   (scale 76)
//!
//! Scale set {0, S/4, S/2, 3S/4, S-1} with S=153: {0, 38, 76, 114, 152}. Each
//! scale's groups are `lib_cmp/512bit_s<scale>`; select one with `-- _s<scale>/`.
//! Arith runs at every scale; the transcendental peer comparison at the
//! reference scale (76) only.

#[macro_use]
mod lib_cmp_common;

use bigdecimal::BigDecimal;
use criterion::{Criterion, criterion_group, criterion_main};
use dashu_float::DBig;
use decimal_scaled::D153;
use fastnum::dec512;
use std::hint::black_box;

const TRANSC: usize = 76;

macro_rules! ds_scale {
    ($g:expr, $s:literal) => {{
        let a = D153::<$s>::try_from(2_i64).unwrap();
        let b = D153::<$s>::try_from(1_i64).unwrap();
        arith_copy!($g, "decimal-scaled", a, b);
        if $s == TRANSC {
            $g.bench_function("decimal-scaled/ln", |bn| bn.iter(|| black_box(a).ln_strict()));
            $g.bench_function("decimal-scaled/exp", |bn| bn.iter(|| black_box(a).exp_strict()));
            $g.bench_function("decimal-scaled/sin", |bn| bn.iter(|| black_box(a).sin_strict()));
            $g.bench_function("decimal-scaled/sqrt", |bn| bn.iter(|| black_box(a).sqrt_strict()));
            $g.bench_function("decimal-scaled/cos", |bn| bn.iter(|| black_box(a).cos_strict()));
            $g.bench_function("decimal-scaled/tan", |bn| bn.iter(|| black_box(a).tan_strict()));
            $g.bench_function("decimal-scaled/atan", |bn| bn.iter(|| black_box(a).atan_strict()));
            $g.bench_function("decimal-scaled/sinh", |bn| bn.iter(|| black_box(a).sinh_strict()));
            $g.bench_function("decimal-scaled/cosh", |bn| bn.iter(|| black_box(a).cosh_strict()));
            $g.bench_function("decimal-scaled/tanh", |bn| bn.iter(|| black_box(a).tanh_strict()));
        }
    }};
}

macro_rules! peers {
    ($g:expr, $scale:expr) => {{
        let g = &mut $g;
        let scale: usize = $scale;

        {
            let a = dec512!(2);
            let b = dec512!(1);
            arith_copy!(g, "fastnum", a, b);
            if scale == TRANSC {
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
        }

        {
            let prec = scale.max(1);
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == TRANSC {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln", |bn| bn.iter(|| black_box(a2.clone()).ln()));
                g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
            }
        }
    }};
}

fn bench(c: &mut Criterion) {
    macro_rules! cell {
        ($s:literal) => {{
            let mut g = c.benchmark_group(concat!("lib_cmp/512bit_s", $s));
            ds_scale!(g, $s);
            peers!(g, $s);
            g.finish();
        }};
    }
    cell!(0);
    cell!(38);
    cell!(76);
    cell!(114);
    cell!(152);
}

criterion_group!(benches, bench);
criterion_main!(benches);
