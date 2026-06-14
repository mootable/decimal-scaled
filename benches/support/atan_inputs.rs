//! Multi-input atan comparison at D38<19>. Each peer's `atan` is
//! timed across 8 input classes (zero, ±1, small-x, mid-range,
//! reciprocal-reduce range, very large) to expose which library
//! short-circuits which class.
//!
//! Background: `fastnum::Decimal::atan` returns signalling NaN
//! immediately for any |x| > 1 (atan.rs:37-39), so the headline
//! lib_cmp_d38 bench (which uses arg=2) measures fastnum's
//! early-return rather than its atan computation. This bench keeps
//! the artefact visible and adds the in-range and special-input
//! columns that actually exercise each library's algorithm.

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D38;
use fastnum::{D128, decimal::Context};
use g_math::canonical::{evaluate, gmath};
use std::hint::black_box;
use std::str::FromStr;

type Ours = D38<19>;

/// Build a Ours value from a decimal string. Avoids subtle
/// from-int + division rounding paths.
fn ours(s: &str) -> Ours {
    Ours::from_str(s).expect("decimal-scaled parse")
}

fn fastn(s: &str) -> D128 {
    D128::from_str(s, Context::default()).expect("fastnum parse")
}

// dashu-float exposes no trig API at all (no atan / sin / cos), so
// it cannot be benched here. rust_decimal / bigdecimal / decimal-rs
// likewise have no atan. Only fastnum and g_math are peer-comparable.

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("atan_inputs_D38s19");
    g.sample_size(40);
    g.measurement_time(std::time::Duration::from_secs(2));

    // (label, arg-as-string)
    let cases: &[(&str, &str)] = &[
        ("zero", "0"),
        ("one", "1"),
        ("neg_one", "-1"),
        ("small_x", "0.0000001"), // 1e-7, hits our small-x fast path
        ("milli", "0.001"),
        ("half", "0.5"),
        ("two", "2"),           // > 1; fastnum NaNs here
        ("large", "100000000"), // 1e8; reciprocal-reduce gives tiny inner arg
    ];

    for (label, s) in cases {
        let our = ours(s);
        let f = fastn(s);

        g.bench_function(format!("{}/decimal-scaled", label), |bn| {
            bn.iter(|| black_box(our).atan_strict());
        });
        g.bench_function(format!("{}/fastnum", label), |bn| {
            bn.iter(|| black_box(f).atan());
        });
        g.bench_function(format!("{}/g_math", label), |bn| {
            bn.iter(|| evaluate(&black_box(gmath(s)).atan()));
        });
    }

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
