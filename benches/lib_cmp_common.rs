//! Shared helpers + macros for the per-width `lib_cmp_d{N}` bench
//! family. Each per-tier bench file declares `#[macro_use] mod
//! lib_cmp_common;` so the macros are crate-local-visible without
//! re-exporting.

/// Register the six arithmetic ops on a `Copy` comparator that
/// implements the standard `std::ops` operators by value.
#[macro_export]
macro_rules! arith_copy {
    ($g:expr, $tag:literal, $a:expr, $b:expr) => {
        $g.bench_function(concat!($tag, "/add"), |bn| bn.iter(|| ::std::hint::black_box($a) + ::std::hint::black_box($b)));
        $g.bench_function(concat!($tag, "/sub"), |bn| bn.iter(|| ::std::hint::black_box($a) - ::std::hint::black_box($b)));
        $g.bench_function(concat!($tag, "/mul"), |bn| bn.iter(|| ::std::hint::black_box($a) * ::std::hint::black_box($b)));
        $g.bench_function(concat!($tag, "/div"), |bn| bn.iter(|| ::std::hint::black_box($a) / ::std::hint::black_box($b)));
        $g.bench_function(concat!($tag, "/rem"), |bn| bn.iter(|| ::std::hint::black_box($a) % ::std::hint::black_box($b)));
        $g.bench_function(concat!($tag, "/neg"), |bn| bn.iter(|| -::std::hint::black_box($a)));
    };
}

/// Same but the type isn't `Copy` (`BigDecimal`, `DBig`) — pass
/// owned clones on every iter.
#[macro_export]
macro_rules! arith_clone {
    ($g:expr, $tag:literal, $a:expr, $b:expr) => {
        $g.bench_function(concat!($tag, "/add"), |bn| bn.iter(|| ::std::hint::black_box($a.clone()) + ::std::hint::black_box($b.clone())));
        $g.bench_function(concat!($tag, "/sub"), |bn| bn.iter(|| ::std::hint::black_box($a.clone()) - ::std::hint::black_box($b.clone())));
        $g.bench_function(concat!($tag, "/mul"), |bn| bn.iter(|| ::std::hint::black_box($a.clone()) * ::std::hint::black_box($b.clone())));
        $g.bench_function(concat!($tag, "/div"), |bn| bn.iter(|| ::std::hint::black_box($a.clone()) / ::std::hint::black_box($b.clone())));
        $g.bench_function(concat!($tag, "/rem"), |bn| bn.iter(|| ::std::hint::black_box($a.clone()) % ::std::hint::black_box($b.clone())));
        $g.bench_function(concat!($tag, "/neg"), |bn| bn.iter(|| -::std::hint::black_box($a.clone())));
    };
}

/// `2 × 10^scale` as a `BigInt` for the bigdecimal mantissa
/// constructor — used at scales past `i128`'s reach.
pub fn num_bigint_from_two_at_scale(scale: usize) -> bigdecimal::num_bigint::BigInt {
    use bigdecimal::num_bigint::BigInt;
    BigInt::from(2) * BigInt::from(10).pow(scale as u32)
}

/// `1 × 10^scale` as a `BigInt`.
pub fn num_bigint_from_one_at_scale(scale: usize) -> bigdecimal::num_bigint::BigInt {
    use bigdecimal::num_bigint::BigInt;
    BigInt::from(10).pow(scale as u32)
}

/// Standard new-tier bench body, parameterised over the type +
/// per-tier midpoint / max scale + bit-width label used in the
/// criterion group name. Each per-tier file invokes this macro
/// once with its own constants. Matches the previous monolith's
/// `decl_new_tier_bench!` semantics.
#[macro_export]
macro_rules! new_tier_body {
    ($T:ident, $bit:literal, $mid:literal, $max:literal) => {
        use bigdecimal::BigDecimal;
        use dashu_float::DBig;
        use decimal_scaled::$T;
        use std::hint::black_box;

        pub fn bench(c: &mut criterion::Criterion) {
            for &scale in &[0_usize, $mid, $max] {
                let group_name = format!(concat!("lib_cmp/", $bit, "bit_s{}"), scale);
                let mut g = c.benchmark_group(&group_name);

                match scale {
                    0 => {
                        let a = $T::<0>::from_int(2); let b = $T::<0>::from_int(1);
                        $crate::arith_copy!(g, "decimal-scaled", a, b);
                    }
                    s if s == $mid => {
                        let a = $T::<$mid>::from_int(2); let b = $T::<$mid>::from_int(1);
                        $crate::arith_copy!(g, "decimal-scaled", a, b);
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
                        $crate::arith_copy!(g, "decimal-scaled", a, b);
                    }
                    _ => unreachable!(),
                }

                {
                    let mant_a = $crate::lib_cmp_common::num_bigint_from_two_at_scale(scale);
                    let mant_b = $crate::lib_cmp_common::num_bigint_from_one_at_scale(scale);
                    let a = BigDecimal::new(mant_a, scale as i64);
                    let b = BigDecimal::new(mant_b, scale as i64);
                    $crate::arith_clone!(g, "bigdecimal", a, b);
                }
                {
                    let prec = scale.max(1) as usize;
                    let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
                    let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
                    $crate::arith_clone!(g, "dashu-float", a, b);
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
