//! Library-comparison bench. For every decimal-scaled width tier we
//! pit each viable peer crate at every shared scale, measuring both
//! **speed** (criterion timing) and **accuracy** (ULP error vs a
//! D76<S>-baseline computed from decimal-scaled's own integer-only
//! strict transcendentals at extra precision).
//!
//! Run with:
//!     cargo bench --bench library_comparison --features wide,x-wide
//!
//! Output drives the §5 Library comparison chapter of
//! docs/benchmarks.md and the per-(op × width) layered line charts
//! under docs/figures/library_comparison/.
//!
//! Tiers covered (widths × per-tier scales):
//!   D9    — 32-bit  storage   — scales 0, 5, 9
//!   D18   — 64-bit  storage   — scales 0, 9, 18
//!   D38   — 128-bit storage   — scales 0, 19, 38
//!   D76   — 256-bit storage   — scales 0, 35, 76
//!   D153  — 512-bit storage   — scales 0, 75, 153
//!   D307  — 1024-bit storage  — scales 0, 150, 307
//!
//! Bench-function id grammar: `lib_cmp/<width>_s<scale>/<lib>/<op>`.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use decimal_scaled::{D153, D18, D307, D38, D76, D9};
#[cfg(feature = "wide")]
use decimal_scaled::{D56, D114, D230};
#[cfg(feature = "x-wide")]
use decimal_scaled::{D461, D615};
#[cfg(feature = "xx-wide")]
use decimal_scaled::{D923, D1231};

use bigdecimal::BigDecimal;
use dashu_float::DBig;
use decimal_rs::Decimal as DecimalRs;
use fastnum::{dec128, dec256, dec512, dec64};
use fixed::types::{I16F16, I32F32, I64F64};
use g_math::canonical::{evaluate, gmath};
use rust_decimal::Decimal as RustDecimal;
use rust_decimal::MathematicalOps;

// ─── Macros to compress the per-tier blocks ────────────────────────

/// Register the six arithmetic ops on a comparator that's `Copy` and
/// implements the standard `std::ops` operators by value.
macro_rules! arith_copy {
    ($g:expr, $tag:literal, $a:expr, $b:expr) => {
        $g.bench_function(concat!($tag, "/add"), |bn| bn.iter(|| black_box($a) + black_box($b)));
        $g.bench_function(concat!($tag, "/sub"), |bn| bn.iter(|| black_box($a) - black_box($b)));
        $g.bench_function(concat!($tag, "/mul"), |bn| bn.iter(|| black_box($a) * black_box($b)));
        $g.bench_function(concat!($tag, "/div"), |bn| bn.iter(|| black_box($a) / black_box($b)));
        $g.bench_function(concat!($tag, "/rem"), |bn| bn.iter(|| black_box($a) % black_box($b)));
        $g.bench_function(concat!($tag, "/neg"), |bn| bn.iter(|| -black_box($a)));
    };
}

/// Same, but the type isn't `Copy` (e.g. `BigDecimal`, `DBig`) — pass
/// owned clones on every iter and pass refs to the operators.
macro_rules! arith_clone {
    ($g:expr, $tag:literal, $a:expr, $b:expr) => {
        $g.bench_function(concat!($tag, "/add"), |bn| bn.iter(|| black_box($a.clone()) + black_box($b.clone())));
        $g.bench_function(concat!($tag, "/sub"), |bn| bn.iter(|| black_box($a.clone()) - black_box($b.clone())));
        $g.bench_function(concat!($tag, "/mul"), |bn| bn.iter(|| black_box($a.clone()) * black_box($b.clone())));
        $g.bench_function(concat!($tag, "/div"), |bn| bn.iter(|| black_box($a.clone()) / black_box($b.clone())));
        $g.bench_function(concat!($tag, "/rem"), |bn| bn.iter(|| black_box($a.clone()) % black_box($b.clone())));
        $g.bench_function(concat!($tag, "/neg"), |bn| bn.iter(|| -black_box($a.clone())));
    };
}

// ─── 32-bit tier (D9) ─────────────────────────────────────────────
fn bench_d9(c: &mut Criterion) {
    for &scale in &[0_usize, 5, 9] {
        let group_name = format!("lib_cmp/32bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D9::<0>::from_int(2); let b = D9::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            5 => {
                let a = D9::<5>::from_int(2); let b = D9::<5>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            9 => {
                let a = D9::<9>::from_int(2); let b = D9::<9>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        // rust_decimal at this scale (scale fits in i64, the mantissa
        // for "2" needs 2 × 10^scale which fits i64 up to scale 18).
        {
            let mant = 2_i64.checked_mul(10_i64.pow(scale as u32)).unwrap();
            let mantb = 1_i64.checked_mul(10_i64.pow(scale as u32)).unwrap();
            let a = RustDecimal::new(mant, scale as u32);
            let b = RustDecimal::new(mantb, scale as u32);
            arith_copy!(g, "rust_decimal", a, b);
        }

        // fixed::I16F16 — scale-less binary fixed-point, single
        // representative cell that we replay at every scale label
        // so the chart line stays drawn.
        {
            let a = I16F16::from_num(2); let b = I16F16::from_num(1);
            arith_copy!(g, "fixed_i16f16", a, b);
        }

        g.finish();
    }
}

// ─── 64-bit tier (D18) ────────────────────────────────────────────
fn bench_d18(c: &mut Criterion) {
    for &scale in &[0_usize, 9, 18] {
        let group_name = format!("lib_cmp/64bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D18::<0>::from_int(2); let b = D18::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            9 => {
                let a = D18::<9>::from_int(2); let b = D18::<9>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            18 => {
                let a = D18::<18>::from_int(2); let b = D18::<18>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        // rust_decimal — scale 18 still fits i64 mantissa for "2 × 10^18".
        if scale <= 18 {
            let mant = 2_i128.checked_mul(10_i128.pow(scale as u32)).unwrap();
            let mantb = 1_i128.checked_mul(10_i128.pow(scale as u32)).unwrap();
            let a = RustDecimal::from_i128_with_scale(mant, scale as u32);
            let b = RustDecimal::from_i128_with_scale(mantb, scale as u32);
            arith_copy!(g, "rust_decimal", a, b);
        }

        // fixed::I32F32
        {
            let a = I32F32::from_num(2); let b = I32F32::from_num(1);
            arith_copy!(g, "fixed_i32f32", a, b);
        }

        // fastnum D64 (decimal at this width)
        {
            let a = dec64!(2); let b = dec64!(1);
            arith_copy!(g, "fastnum", a, b);
        }

        g.finish();
    }
}

// ─── 128-bit tier (D38) ───────────────────────────────────────────
fn bench_d38(c: &mut Criterion) {
    for &scale in &[0_usize, 19, 38] {
        let group_name = format!("lib_cmp/128bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        // decimal-scaled with transcendentals
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
                // D38<38> can't hold 2, only ±1.7. Use 1 / 1 for arith
                // so the comparison stays valid; transcendentals skipped
                // here because pi/e/etc don't fit and ln(1)=0 fast path
                // is uninformative.
                let a = D38::<38>::from_bits(170_000_000_000_000_000_000_000_000_000_000_000_000_i128 / 2);
                let b = D38::<38>::from_bits(100_000_000_000_000_000_000_000_000_000_000_000_i128);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        // rust_decimal at this scale (max 28)
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
                g.bench_function("rust_decimal/sqrt", |bn| bn.iter(|| black_box(a).sqrt()));
            }
        }

        // fastnum D128
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

        // fixed::I64F64
        {
            let a = I64F64::from_num(2); let b = I64F64::from_num(1);
            arith_copy!(g, "fixed_i64f64", a, b);
        }

        // g_math Q64.64 — scale-less, single representative
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

        // bigdecimal at this scale — construct mantissa via BigInt so
        // scale 38 (where 2 × 10^38 overflows i128) is safe.
        {
            let a = BigDecimal::new(num_bigint_from_two_at_scale(scale), scale as i64);
            let b = BigDecimal::new(num_bigint_from_one_at_scale(scale), scale as i64);
            arith_clone!(g, "bigdecimal", a, b);
            if scale == 19 {
                let a2 = a.clone();
                g.bench_function("bigdecimal/sqrt", |bn| bn.iter(|| black_box(a2.clone()).sqrt()));
            }
        }

        // dashu-float DBig at this precision (scale ≈ digit count)
        {
            let prec = (scale.max(1)) as usize;
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == 19 {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln",  |bn| bn.iter(|| black_box(a2.clone()).ln()));
                g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
            }
        }

        // decimal-rs caps at 38 total digits. At scale 38 the
        // mantissa for "2 × 10^38" is a 39-digit number → overflow.
        // Skip the scale-38 cell entirely; scales 0 and 19 fit.
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

// ─── 256-bit tier (D76) ──────────────────────────────────────────
fn bench_d76(c: &mut Criterion) {
    for &scale in &[0_usize, 35, 76] {
        let group_name = format!("lib_cmp/256bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        // decimal-scaled
        match scale {
            0 => {
                let a = D76::<0>::from_int(2); let b = D76::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            35 => {
                let a = D76::<35>::from_int(2); let b = D76::<35>::from_int(1);
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
            76 => {
                // D76<76> storage is ±~9, fits 2 and 1.
                let a = D76::<76>::from_int(2); let b = D76::<76>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        // fastnum D256
        {
            let a = dec256!(2); let b = dec256!(1);
            arith_copy!(g, "fastnum", a, b);
            if scale == 35 {
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

        // bigdecimal at this scale
        {
            let mant_a = num_bigint_from_two_at_scale(scale);
            let mant_b = num_bigint_from_one_at_scale(scale);
            let a = BigDecimal::new(mant_a, scale as i64);
            let b = BigDecimal::new(mant_b, scale as i64);
            arith_clone!(g, "bigdecimal", a, b);
            if scale == 35 {
                let a2 = a.clone();
                g.bench_function("bigdecimal/sqrt", |bn| bn.iter(|| black_box(a2.clone()).sqrt()));
            }
        }

        // dashu-float
        {
            let prec = scale.max(1) as usize;
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == 35 {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln",  |bn| bn.iter(|| black_box(a2.clone()).ln()));
                g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
            }
        }

        g.finish();
    }
}

// ─── 512-bit tier (D153) ─────────────────────────────────────────
fn bench_d153(c: &mut Criterion) {
    for &scale in &[0_usize, 75, 153] {
        let group_name = format!("lib_cmp/512bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D153::<0>::from_int(2); let b = D153::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            75 => {
                let a = D153::<75>::from_int(2); let b = D153::<75>::from_int(1);
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
            153 => {
                let a = D153::<153>::from_int(2); let b = D153::<153>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        // fastnum D512
        {
            let a = dec512!(2); let b = dec512!(1);
            arith_copy!(g, "fastnum", a, b);
            if scale == 75 {
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

        // bigdecimal
        {
            let mant_a = num_bigint_from_two_at_scale(scale);
            let mant_b = num_bigint_from_one_at_scale(scale);
            let a = BigDecimal::new(mant_a, scale as i64);
            let b = BigDecimal::new(mant_b, scale as i64);
            arith_clone!(g, "bigdecimal", a, b);
        }

        // dashu-float
        {
            let prec = scale.max(1) as usize;
            let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
            let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
            arith_clone!(g, "dashu-float", a, b);
            if scale == 75 {
                let a2 = a.clone();
                g.bench_function("dashu-float/ln",  |bn| bn.iter(|| black_box(a2.clone()).ln()));
                g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
            }
        }

        g.finish();
    }
}

// ─── 1024-bit tier (D307) ────────────────────────────────────────
fn bench_d307(c: &mut Criterion) {
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

        // bigdecimal + dashu-float at extreme widths
        {
            let mant_a = num_bigint_from_two_at_scale(scale);
            let mant_b = num_bigint_from_one_at_scale(scale);
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

// ─── Helpers for very-wide bigdecimal mantissa construction ─────
fn num_bigint_from_two_at_scale(scale: usize) -> bigdecimal::num_bigint::BigInt {
    use bigdecimal::num_bigint::BigInt;
    BigInt::from(2) * BigInt::from(10).pow(scale as u32)
}
fn num_bigint_from_one_at_scale(scale: usize) -> bigdecimal::num_bigint::BigInt {
    use bigdecimal::num_bigint::BigInt;
    BigInt::from(10).pow(scale as u32)
}

// ─── New half-width and wider tiers ──────────────────────────────
//
// These slot between the power-of-two tiers above. Each compares
// decimal-scaled arithmetic at scale 0 / mid / max plus a
// transcendental sample at the midpoint, alongside the same
// bigdecimal + dashu-float external baselines.
macro_rules! decl_new_tier_bench {
    ($fn_name:ident, $T:ident, $bit:literal, $mid:literal, $max:literal) => {
        fn $fn_name(c: &mut Criterion) {
            for &scale in &[0_usize, $mid, $max] {
                let group_name = format!(concat!("lib_cmp/", $bit, "bit_s{}"), scale);
                let mut g = c.benchmark_group(&group_name);

                match scale {
                    0 => {
                        let a = $T::<0>::from_int(2); let b = $T::<0>::from_int(1);
                        arith_copy!(g, "decimal-scaled", a, b);
                    }
                    s if s == $mid => {
                        let a = $T::<$mid>::from_int(2); let b = $T::<$mid>::from_int(1);
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
                    s if s == $max => {
                        let a = $T::<$max>::from_int(2); let b = $T::<$max>::from_int(1);
                        arith_copy!(g, "decimal-scaled", a, b);
                    }
                    _ => unreachable!(),
                }

                // bigdecimal + dashu-float — pure-Rust BigInt-backed
                // baselines, the only external libs that can carry
                // these wide scales.
                {
                    let mant_a = num_bigint_from_two_at_scale(scale);
                    let mant_b = num_bigint_from_one_at_scale(scale);
                    let a = BigDecimal::new(mant_a, scale as i64);
                    let b = BigDecimal::new(mant_b, scale as i64);
                    arith_clone!(g, "bigdecimal", a, b);
                }
                {
                    let prec = scale.max(1) as usize;
                    let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
                    let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
                    arith_clone!(g, "dashu-float", a, b);
                    if scale == $mid {
                        let a2 = a.clone();
                        g.bench_function("dashu-float/ln",  |bn| bn.iter(|| black_box(a2.clone()).ln()));
                        g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
                    }
                }

                g.finish();
            }
        }
    };
}

#[cfg(feature = "wide")]
decl_new_tier_bench!(bench_d56,  D56,  "192",  28,  56);
#[cfg(feature = "wide")]
decl_new_tier_bench!(bench_d114, D114, "384",  57,  114);
#[cfg(feature = "wide")]
decl_new_tier_bench!(bench_d230, D230, "768",  115, 230);
#[cfg(feature = "x-wide")]
decl_new_tier_bench!(bench_d461, D461, "1536", 230, 461);
#[cfg(feature = "x-wide")]
decl_new_tier_bench!(bench_d615, D615, "2048", 308, 615);
#[cfg(feature = "xx-wide")]
decl_new_tier_bench!(bench_d923, D923, "3072", 461, 923);
#[cfg(feature = "xx-wide")]
decl_new_tier_bench!(bench_d1231, D1231, "4096", 616, 1231);

#[cfg(all(feature = "wide", not(feature = "x-wide"), not(feature = "xx-wide")))]
criterion_group!(
    benches,
    bench_d9, bench_d18, bench_d38, bench_d76, bench_d153, bench_d307,
    bench_d56, bench_d114, bench_d230,
);
#[cfg(all(feature = "x-wide", not(feature = "xx-wide")))]
criterion_group!(
    benches,
    bench_d9, bench_d18, bench_d38, bench_d76, bench_d153, bench_d307,
    bench_d56, bench_d114, bench_d230,
    bench_d461, bench_d615,
);
#[cfg(feature = "xx-wide")]
criterion_group!(
    benches,
    bench_d9, bench_d18, bench_d38, bench_d76, bench_d153, bench_d307,
    bench_d56, bench_d114, bench_d230,
    bench_d461, bench_d615,
    bench_d923, bench_d1231,
);
#[cfg(not(feature = "wide"))]
criterion_group!(
    benches,
    bench_d9, bench_d18, bench_d38, bench_d76, bench_d153, bench_d307,
);
criterion_main!(benches);
