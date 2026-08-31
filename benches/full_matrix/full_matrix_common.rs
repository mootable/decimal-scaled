//! Shared macros for the per-width `full_matrix_d*` benches.
//!
//! Why per-width: a monolithic `full_matrix` bench takes hours and a
//! power loss / lid close near the end loses everything. With one
//! bench binary per width, each width's criterion data is flushed
//! before the next starts, so a partial sweep survives interruption.
//!
//! Cross-crate baselines (bnum, rust_decimal, fixed) live in the
//! per-width bench that matches their natural width — see the
//! comments at the top of each `full_matrix_dN.rs` for the assignment.

#![allow(unused_macros)]

/// Six arithmetic ops on a `TryFrom<i64>`-constructible decimal type.
#[macro_export]
macro_rules! arith_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        let a: $T = <$T>::try_from(2).unwrap();
        let b: $T = <$T>::try_from(1).unwrap();
        $g.bench_function(concat!($tag, "/add"), |bn| {
            bn.iter(|| ::std::hint::black_box(a) + ::std::hint::black_box(b))
        });
        $g.bench_function(concat!($tag, "/sub"), |bn| {
            bn.iter(|| ::std::hint::black_box(a) - ::std::hint::black_box(b))
        });
        $g.bench_function(concat!($tag, "/mul"), |bn| {
            bn.iter(|| ::std::hint::black_box(a) * ::std::hint::black_box(b))
        });
        $g.bench_function(concat!($tag, "/div"), |bn| {
            bn.iter(|| ::std::hint::black_box(a) / ::std::hint::black_box(b))
        });
        $g.bench_function(concat!($tag, "/rem"), |bn| {
            bn.iter(|| ::std::hint::black_box(a) % ::std::hint::black_box(b))
        });
        $g.bench_function(concat!($tag, "/neg"), |bn| {
            bn.iter(|| -::std::hint::black_box(a))
        });
    }};
}

/// f64-bridge `ln` / `exp` / `sin` / `sqrt`. Only used in the D18 /
/// D38 per-width benches; the wide tiers have no fast path.
#[macro_export]
macro_rules! fast_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        let half: $T = <$T>::try_from(1).unwrap() / <$T>::try_from(2).unwrap();
        let x: $T = <$T>::try_from(1).unwrap() + half;
        let xh: $T = half;
        $g.bench_function(concat!($tag, "/ln"), |bn| {
            bn.iter(|| ::std::hint::black_box(x).ln())
        });
        $g.bench_function(concat!($tag, "/exp"), |bn| {
            bn.iter(|| ::std::hint::black_box(xh).exp())
        });
        $g.bench_function(concat!($tag, "/sin"), |bn| {
            bn.iter(|| ::std::hint::black_box(x).sin())
        });
        $g.bench_function(concat!($tag, "/sqrt"), |bn| {
            bn.iter(|| ::std::hint::black_box(x).sqrt())
        });
    }};
}

/// Integer-only `ln_strict` / `exp_strict` / `sin_strict` /
/// `sqrt_strict`.
#[macro_export]
macro_rules! strict_block {
    ($g:ident, $tag:literal, $T:ty) => {{
        let half: $T = <$T>::try_from(1).unwrap() / <$T>::try_from(2).unwrap();
        let x: $T = <$T>::try_from(1).unwrap() + half;
        let xh: $T = half;
        $g.bench_function(concat!($tag, "/ln"), |bn| {
            bn.iter(|| ::std::hint::black_box(x).ln_strict())
        });
        $g.bench_function(concat!($tag, "/exp"), |bn| {
            bn.iter(|| ::std::hint::black_box(xh).exp_strict())
        });
        $g.bench_function(concat!($tag, "/sin"), |bn| {
            bn.iter(|| ::std::hint::black_box(x).sin_strict())
        });
        $g.bench_function(concat!($tag, "/sqrt"), |bn| {
            bn.iter(|| ::std::hint::black_box(x).sqrt_strict())
        });
    }};
}
