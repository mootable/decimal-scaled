//! Decimal / fixed-point backend comparison across every width and
//! every comparable operation.
//!
//! The crate ships eleven decimal widths (D18, D38 plus the
//! wide tier D76/D115/D153/D230/D307 and the x-wide tier
//! D462/D616/D924/D1232). This bench fans the add / sub / mul /
//! div / rem / neg primitives across every available width and pits
//! them against the established baselines:
//!
//! - `BnumD76` — `bnum`-backed 256-bit decimal (benchmark baseline,
//!   see `benches/bnum/`);
//! - `rust_decimal::Decimal` — a 96-bit-mantissa decimal crate;
//! - `fixed::I64F64` — a binary fixed-point crate.
//!
//! `rust_decimal` and `fixed` do not have wide-tier counterparts, so
//! they appear only in the D38-and-narrower groups. `BnumD76` only
//! compares with D76.
//!
//! Transcendentals (`ln`, `exp`, `sqrt`, `sin`, `cos`, `atan2`, `pow`)
//! cover D38 and D76 in both the lossy (f64-bridge) and strict
//! (integer-only, correctly-rounded to 0.5 ULP) forms, alongside
//! `rust_decimal` and the crate's native fast-paths.
//!
//! Run with: `cargo bench --features wide --bench decimal_backends`.

#[path = "../bnum/mod.rs"]
mod bnum;

use bnum::BnumD76;
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(feature = "d153")]
use decimal_scaled::D153;
#[cfg(feature = "d307")]
use decimal_scaled::D307;
use decimal_scaled::{D18, D38, D76};
use fixed::types::I64F64;
use rust_decimal::{Decimal, MathematicalOps};
use std::hint::black_box;

const A: i64 = 1_234_567;
const B: i64 = 89_543;

/// Emits a six-op bench (`add` / `sub` / `mul` / `div` / `rem` / `neg`)
/// for one `(label, a, b)` triple. Centralised so adding a new width
/// is one macro call.
macro_rules! six_ops {
    ($g:expr, $label:literal, $a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        $g.bench_function(concat!($label, "/add"), |bn| {
            bn.iter(|| black_box(a) + black_box(b))
        });
        $g.bench_function(concat!($label, "/sub"), |bn| {
            bn.iter(|| black_box(a) - black_box(b))
        });
        $g.bench_function(concat!($label, "/mul"), |bn| {
            bn.iter(|| black_box(a) * black_box(b))
        });
        $g.bench_function(concat!($label, "/div"), |bn| {
            bn.iter(|| black_box(a) / black_box(b))
        });
        $g.bench_function(concat!($label, "/rem"), |bn| {
            bn.iter(|| black_box(a) % black_box(b))
        });
        $g.bench_function(concat!($label, "/neg"), |bn| bn.iter(|| -black_box(a)));
    }};
}

/// add / sub / mul / div / rem / neg across every available width.
fn bench_arithmetic(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal/arith");

    // Native-storage tier: i64, i128.
    six_ops!(g, "D18", D18::<12>::try_from(A).unwrap(), D18::<12>::try_from(B).unwrap());
    six_ops!(g, "D38", D38::<12>::from(A), D38::<12>::from(B));

    // Wide tier (256-bit and up, hand-rolled wide integers). Only
    // the powers-of-two D types are exposed publicly so far; the
    // intermediate widths (D115, D230, D462, D924) are queued.
    six_ops!(
        g,
        "D76",
        D76::<12>::try_from(A as i128).unwrap(),
        D76::<12>::try_from(B as i128).unwrap()
    );
    #[cfg(feature = "d153")]
    six_ops!(
        g,
        "D153",
        D153::<12>::try_from(A as i128).unwrap(),
        D153::<12>::try_from(B as i128).unwrap()
    );
    #[cfg(feature = "d307")]
    six_ops!(
        g,
        "D307",
        D307::<12>::try_from(A as i128).unwrap(),
        D307::<12>::try_from(B as i128).unwrap()
    );

    // Baselines.
    six_ops!(
        g,
        "bnum_d76",
        BnumD76::<12>::from_int(A as i128),
        BnumD76::<12>::from_int(B as i128)
    );
    six_ops!(g, "rust_decimal", Decimal::from(A), Decimal::from(B));
    six_ops!(g, "fixed_i64f64", I64F64::from_num(A), I64F64::from_num(B));

    g.finish();
}

/// `ln` / `exp` / `sqrt` / `sin` / `cos` / `atan2` / `pow`, comparing
/// the crate's D38 / D76 (fast and strict) variants against
/// `rust_decimal`. The wide-tier strict variants quantify the
/// correctly-rounded-to-0.5-ULP cost.
fn bench_transcendentals(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal/transc");

    // `≈ 2.345678901` in each representation.
    let ours128 = D38::<9>::from_bits(
        decimal_scaled::Int::<2>::try_from(2_345_678_901_i128).unwrap(),
    );
    let ours256 = D76::<9>::from(2);
    let rd = Decimal::new(2_345_678_901, 9);

    macro_rules! one_arg {
        ($name:literal, $lossy:expr, $strict:expr) => {
            g.bench_function(concat!("D128_lossy/", $name), |b| b.iter(|| $lossy));
            g.bench_function(concat!("D128_strict/", $name), |b| b.iter(|| $strict));
        };
    }

    one_arg!(
        "ln",
        black_box(ours128).ln_fast(),
        black_box(ours128).ln_strict()
    );
    one_arg!(
        "exp",
        black_box(ours128).exp_fast(),
        black_box(ours128).exp_strict()
    );
    one_arg!(
        "sqrt",
        black_box(ours128).sqrt_fast(),
        black_box(ours128).sqrt_strict()
    );
    one_arg!(
        "cbrt",
        black_box(ours128).cbrt_fast(),
        black_box(ours128).cbrt_strict()
    );
    one_arg!(
        "sin",
        black_box(ours128).sin_fast(),
        black_box(ours128).sin_strict()
    );
    one_arg!(
        "cos",
        black_box(ours128).cos_fast(),
        black_box(ours128).cos_strict()
    );
    one_arg!(
        "tan",
        black_box(ours128).tan_fast(),
        black_box(ours128).tan_strict()
    );
    one_arg!(
        "atan",
        black_box(ours128).atan_fast(),
        black_box(ours128).atan_strict()
    );

    g.bench_function("rust_decimal/ln", |b| b.iter(|| black_box(rd).ln()));
    g.bench_function("rust_decimal/exp", |b| b.iter(|| black_box(rd).exp()));
    g.bench_function("rust_decimal/sqrt", |b| b.iter(|| black_box(rd).sqrt()));
    g.bench_function("rust_decimal/sin", |b| b.iter(|| black_box(rd).sin()));
    g.bench_function("rust_decimal/cos", |b| b.iter(|| black_box(rd).cos()));
    g.bench_function("rust_decimal/tan", |b| b.iter(|| black_box(rd).tan()));

    // Wide tier: D76 transcendentals. Strict variants are
    // correctly-rounded; lossy paths route through f64.
    g.bench_function("D256_lossy/ln", |b| b.iter(|| black_box(ours256).ln_fast()));
    g.bench_function("D256_strict/ln", |b| {
        b.iter(|| black_box(ours256).ln_strict())
    });
    g.bench_function("D256_lossy/exp", |b| {
        b.iter(|| black_box(ours256).exp_fast())
    });
    g.bench_function("D256_strict/exp", |b| {
        b.iter(|| black_box(ours256).exp_strict())
    });
    g.bench_function("D256_lossy/sqrt", |b| {
        b.iter(|| black_box(ours256).sqrt_fast())
    });
    g.bench_function("D256_strict/sqrt", |b| {
        b.iter(|| black_box(ours256).sqrt_strict())
    });
    g.bench_function("D256_lossy/sin", |b| {
        b.iter(|| black_box(ours256).sin_fast())
    });
    g.bench_function("D256_strict/sin", |b| {
        b.iter(|| black_box(ours256).sin_strict())
    });

    // Two-arg ops: pow, atan2.
    let p = D38::<9>::from_bits(decimal_scaled::Int::<2>::try_from(3_000_000_000_i128).unwrap()); // 3.0
    g.bench_function("D128_lossy/powf", |b| {
        b.iter(|| black_box(ours128).powf_fast(black_box(p)))
    });
    g.bench_function("D128_strict/powf", |b| {
        b.iter(|| black_box(ours128).powf_strict(black_box(p)))
    });
    g.bench_function("D128_lossy/atan2", |b| {
        b.iter(|| black_box(ours128).atan2_fast(black_box(p)))
    });
    g.bench_function("D128_strict/atan2", |b| {
        b.iter(|| black_box(ours128).atan2_strict(black_box(p)))
    });

    g.finish();
}

criterion_group!(benches, bench_arithmetic, bench_transcendentals);
criterion_main!(benches);
