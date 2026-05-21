//! Focused microbench isolating the cost of the directed-rounding
//! correctness fixes on the strict transcendental kernels.
//!
//! Two questions:
//!
//!   1. NEAREST modes (`HalfToEven`) take the cheap fixed-guard path,
//!      gated behind an `is_nearest_mode` predicate. They should be
//!      unchanged before/after the fix — this bench provides the
//!      same-machine number to confirm "flat".
//!
//!   2. DIRECTED modes (`Floor`) now escalate the guard width near a
//!      rounding boundary (recompute at a wider scale until the
//!      result provably rounds the same way). This bench times the
//!      `Floor` path against its own `HalfToEven` sibling to expose
//!      the escalation overhead, on both "generic" inputs and inputs
//!      sitting near a rounding boundary.
//!
//! Each cell times `x.<fn>_strict_with(mode)` for both modes, so the
//! HalfToEven columns are directly comparable across a 0.4.3 / fixed
//! checkout, and the Floor-vs-HalfToEven delta is readable within a
//! single run.
//!
//! Kept deliberately short (10–15 samples, ~400 ms each) so the whole
//! binary finishes in a couple of minutes.

use std::hint::black_box;
use std::str::FromStr;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::{RoundingMode, D307, D38, D76};
#[cfg(feature = "xx-wide")]
use decimal_scaled::D1232;

/// Pin the benching thread to one core so the criterion timing is not
/// smeared by the scheduler migrating us across cores mid-run.
fn pin_core() {
    if let Some(ids) = core_affinity::get_core_ids() {
        if let Some(first) = ids.into_iter().next() {
            core_affinity::set_for_current(first);
        }
    }
}

/// The two modes we compare: the cheap nearest path and the directed
/// path that now Ziv-escalates.
const MODES: [(&str, RoundingMode); 2] = [
    ("HalfToEven", RoundingMode::HalfToEven),
    ("Floor", RoundingMode::Floor),
];

/// Time every (mode) variant of one strict transcendental call.
///
/// `$build` parses the input string into `$T`; `$call` is the method
/// to invoke with a `RoundingMode`. The `$class` tag distinguishes
/// "generic" inputs from "near-boundary" ones in the report names.
macro_rules! bench_fn {
    ($g:ident, $T:ty, $fname:literal, $class:literal, $input:literal, $call:ident) => {{
        let x: $T = <$T>::from_str($input).expect("parse bench input");
        for (mlabel, mode) in MODES {
            // The width tier is carried by the benchmark-group name.
            let id = format!(concat!($fname, "/", $class, "/{}"), mlabel);
            $g.bench_function(id, |bn| {
                bn.iter(|| black_box(x).$call(black_box(mode)))
            });
        }
    }};
}

/// One full tier's worth of cells: the six functions, each on a
/// generic and a near-boundary input.
macro_rules! tier_block {
    ($c:ident, $T:ty, $tier:literal) => {{
        let mut g = $c.benchmark_group(concat!("prec_fix/", $tier));
        g.sample_size(12);
        g.measurement_time(Duration::from_millis(400));
        g.warm_up_time(Duration::from_millis(150));

        // exp: generic 1.5; near-boundary tiny x (result hugs 1).
        bench_fn!(g, $T, "exp", "generic", "1.5", exp_strict_with);
        bench_fn!(g, $T, "exp", "boundary", "0.0000001", exp_strict_with);

        // ln: generic 2; near-boundary x just above 1 (result hugs 0).
        bench_fn!(g, $T, "ln", "generic", "2", ln_strict_with);
        bench_fn!(g, $T, "ln", "boundary", "1.0000001", ln_strict_with);

        // sin: generic 0.7; near-boundary tiny x (result hugs x ~ 0).
        bench_fn!(g, $T, "sin", "generic", "0.7", sin_strict_with);
        bench_fn!(g, $T, "sin", "boundary", "0.0000001", sin_strict_with);

        // cos: generic 0.7; near-boundary tiny x (result hugs 1).
        bench_fn!(g, $T, "cos", "generic", "0.7", cos_strict_with);
        bench_fn!(g, $T, "cos", "boundary", "0.0000001", cos_strict_with);

        // sqrt: generic 2; near-boundary perfect square 4 (exact tie).
        bench_fn!(g, $T, "sqrt", "generic", "2", sqrt_strict_with);
        bench_fn!(g, $T, "sqrt", "boundary", "4", sqrt_strict_with);

        // cbrt: generic 2; near-boundary perfect cube 8 (exact tie).
        bench_fn!(g, $T, "cbrt", "generic", "2", cbrt_strict_with);
        bench_fn!(g, $T, "cbrt", "boundary", "8", cbrt_strict_with);

        g.finish();
    }};
}

fn bench_d38(c: &mut Criterion) {
    tier_block!(c, D38<19>, "D38_s19");
}

fn bench_d76(c: &mut Criterion) {
    tier_block!(c, D76<35>, "D76_s35");
}

fn bench_d307(c: &mut Criterion) {
    tier_block!(c, D307<150>, "D307_s150");
}

/// One xx-wide tier cell to keep the directed-vs-nearest signal
/// visible at the widest backing integer, with a tiny sample count so
/// runtime stays sane. Only built when the `xx-wide` tier is enabled.
#[cfg(feature = "xx-wide")]
fn bench_d1232(c: &mut Criterion) {
    let mut g = c.benchmark_group("prec_fix/D1232_s600");
    g.sample_size(10);
    g.measurement_time(Duration::from_millis(400));
    g.warm_up_time(Duration::from_millis(150));

    bench_fn!(g, D1232<600>, "exp", "generic", "1.5", exp_strict_with);
    bench_fn!(g, D1232<600>, "ln", "generic", "2", ln_strict_with);
    bench_fn!(g, D1232<600>, "sin", "generic", "0.7", sin_strict_with);

    g.finish();
}

fn benches(c: &mut Criterion) {
    pin_core();
    bench_d38(c);
    bench_d76(c);
    bench_d307(c);
    #[cfg(feature = "xx-wide")]
    bench_d1232(c);
}

criterion_group!(group, benches);
criterion_main!(group);
