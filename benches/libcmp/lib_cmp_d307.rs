//! Per-tier library_comparison bench for D307 (1024-bit storage).
//! Run: cargo bench --features "wide x-wide xx-wide" --bench lib_cmp_d307
//!      cargo bench --features "wide" --bench lib_cmp_d307 -- _s153/  (scale 153)
//!
//! Scale set dedup{0, 30, S/2, S-1} with S=307: {0, 30, 153, 306}. Each scale's
//! groups are `lib_cmp/1024bit_s<scale>`; select one with `-- _s<scale>/`.
//! Arith runs at every scale; the transcendental peer comparison at the
//! reference scale (153) only.

#[macro_use]
mod lib_cmp_common;

use bigdecimal::BigDecimal;
use criterion::{Criterion, criterion_group, criterion_main};
use dashu_float::DBig;
use decimal_scaled::D307;
use std::hint::black_box;

const TRANSC: usize = 153;

macro_rules! ds_scale {
    ($g:expr, $s:literal) => {{
        let a = D307::<$s>::from(2);
        let b = D307::<$s>::from(1);
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
            let mut g = c.benchmark_group(concat!("lib_cmp/1024bit_s", $s));
            ds_scale!(g, $s);
            peers!(g, $s);
            g.finish();
        }};
    }
    cell!(0);
    cell!(30);
    cell!(153);
    cell!(306);
}

criterion_group!(benches, bench);
criterion_main!(benches);
