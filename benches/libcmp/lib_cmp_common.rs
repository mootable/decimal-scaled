//! Shared helpers + macros for the per-width `lib_cmp_d{N}` bench
//! family. Each per-tier bench file declares `#[macro_use] mod
//! lib_cmp_common;` so the macros are crate-local-visible without
//! re-exporting.

/// Register the six arithmetic ops on a `Copy` comparator that
/// implements the standard `std::ops` operators by value.
#[macro_export]
macro_rules! arith_copy {
    ($g:expr, $tag:literal, $a:expr, $b:expr) => {
        $g.bench_function(concat!($tag, "/add"), |bn| {
            bn.iter(|| ::std::hint::black_box($a) + ::std::hint::black_box($b))
        });
        $g.bench_function(concat!($tag, "/sub"), |bn| {
            bn.iter(|| ::std::hint::black_box($a) - ::std::hint::black_box($b))
        });
        $g.bench_function(concat!($tag, "/mul"), |bn| {
            bn.iter(|| ::std::hint::black_box($a) * ::std::hint::black_box($b))
        });
        $g.bench_function(concat!($tag, "/div"), |bn| {
            bn.iter(|| ::std::hint::black_box($a) / ::std::hint::black_box($b))
        });
        $g.bench_function(concat!($tag, "/rem"), |bn| {
            bn.iter(|| ::std::hint::black_box($a) % ::std::hint::black_box($b))
        });
        $g.bench_function(concat!($tag, "/neg"), |bn| {
            bn.iter(|| -::std::hint::black_box($a))
        });
    };
}

/// Same but the type isn't `Copy` (`BigDecimal`, `DBig`) — pass
/// owned clones on every iter.
#[macro_export]
macro_rules! arith_clone {
    ($g:expr, $tag:literal, $a:expr, $b:expr) => {
        $g.bench_function(concat!($tag, "/add"), |bn| {
            bn.iter(|| ::std::hint::black_box($a.clone()) + ::std::hint::black_box($b.clone()))
        });
        $g.bench_function(concat!($tag, "/sub"), |bn| {
            bn.iter(|| ::std::hint::black_box($a.clone()) - ::std::hint::black_box($b.clone()))
        });
        $g.bench_function(concat!($tag, "/mul"), |bn| {
            bn.iter(|| ::std::hint::black_box($a.clone()) * ::std::hint::black_box($b.clone()))
        });
        $g.bench_function(concat!($tag, "/div"), |bn| {
            bn.iter(|| ::std::hint::black_box($a.clone()) / ::std::hint::black_box($b.clone()))
        });
        $g.bench_function(concat!($tag, "/rem"), |bn| {
            bn.iter(|| ::std::hint::black_box($a.clone()) % ::std::hint::black_box($b.clone()))
        });
        $g.bench_function(concat!($tag, "/neg"), |bn| {
            bn.iter(|| -::std::hint::black_box($a.clone()))
        });
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

/// Standard new-tier bench body, parameterised over the type + the tier's
/// explicit SCALE SET + the transcendental reference scale + bit-width label
/// used in the criterion group name. Each per-tier file invokes this macro
/// once with its own constants.
///
/// The scale set mirrors the golden / `bench-branch-compare` coverage —
/// `{0, S/4, S/2, 3S/4, S-1}` (S = the tier's max scale, floor division) — so
/// a peer-comparison regression is visible at the scale it occurs, not just
/// one canonical scale. Each scale's groups are named
/// `lib_cmp/<bit>bit_s<scale>`, so a single scale is selectable by a criterion
/// name-filter — `-- _s9/` (the trailing `/` anchors the scale: `_s9/` matches
/// `..._s9/decimal-scaled` but NOT `..._s90/...`). The arith ops run at every
/// scale; the transcendentals run at `$transc` (the reference scale = the
/// tier's S/2 point) only, where the dashu-float peer comparison for ln/exp is
/// also attached.
#[macro_export]
macro_rules! new_tier_body {
    ($T:ident, $bit:literal, [$($scale:literal),+ $(,)?], $transc:literal) => {
        use bigdecimal::BigDecimal;
        use dashu_float::DBig;
        use decimal_scaled::$T;
        use std::hint::black_box;

        // One (decimal-scaled + peers) block at one const-generic SCALE.
        // `$s:literal` lets `$T::<$s>` monomorphise and names the group.
        macro_rules! tier_scale {
            ($c:expr, $s:literal) => {{
                let group_name = concat!("lib_cmp/", $bit, "bit_s", $s);
                let mut g = $c.benchmark_group(group_name);

                let a = $T::<$s>::try_from(2).unwrap();
                let b = $T::<$s>::try_from(1).unwrap();
                $crate::arith_copy!(g, "decimal-scaled", a, b);
                // Transcendentals only at the reference scale.
                if $s == $transc {
                    g.bench_function("decimal-scaled/ln", |bn| bn.iter(|| black_box(a).ln_strict()));
                    g.bench_function("decimal-scaled/exp", |bn| bn.iter(|| black_box(a).exp_strict()));
                    g.bench_function("decimal-scaled/sin", |bn| bn.iter(|| black_box(a).sin_strict()));
                    g.bench_function("decimal-scaled/sqrt", |bn| bn.iter(|| black_box(a).sqrt_strict()));
                    g.bench_function("decimal-scaled/cos", |bn| bn.iter(|| black_box(a).cos_strict()));
                    g.bench_function("decimal-scaled/tan", |bn| bn.iter(|| black_box(a).tan_strict()));
                    g.bench_function("decimal-scaled/atan", |bn| bn.iter(|| black_box(a).atan_strict()));
                    g.bench_function("decimal-scaled/sinh", |bn| bn.iter(|| black_box(a).sinh_strict()));
                    g.bench_function("decimal-scaled/cosh", |bn| bn.iter(|| black_box(a).cosh_strict()));
                    g.bench_function("decimal-scaled/tanh", |bn| bn.iter(|| black_box(a).tanh_strict()));
                }

                {
                    let mant_a = $crate::lib_cmp_common::num_bigint_from_two_at_scale($s);
                    let mant_b = $crate::lib_cmp_common::num_bigint_from_one_at_scale($s);
                    let a = BigDecimal::new(mant_a, $s as i64);
                    let b = BigDecimal::new(mant_b, $s as i64);
                    $crate::arith_clone!(g, "bigdecimal", a, b);
                }
                {
                    let prec = ($s as usize).max(1);
                    let a = DBig::from_parts(2.into(), 0).with_precision(prec).value();
                    let b = DBig::from_parts(1.into(), 0).with_precision(prec).value();
                    $crate::arith_clone!(g, "dashu-float", a, b);
                    if $s == $transc {
                        let a2 = a.clone();
                        g.bench_function("dashu-float/ln", |bn| bn.iter(|| black_box(a2.clone()).ln()));
                        g.bench_function("dashu-float/exp", |bn| bn.iter(|| black_box(a2.clone()).exp()));
                    }
                }

                g.finish();
            }};
        }

        pub fn bench(c: &mut criterion::Criterion) {
            $( tier_scale!(c, $scale); )+
        }
    };
}
