//! M2 gate for the `ln_approx(working_digits)` API. Times ln_strict
//! (guard=30) against ln_approx with guard=6, 10, 15 at D38<19>.
//! Gate: ln_approx(6) must show ≥ 1.5× over strict before promoting
//! to the full *_approx family across all transcendentals.

use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D38;
use std::str::FromStr;

fn at(s: &str) -> D38<19> {
    D38::<19>::from_str(s).expect("parse")
}

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("m2_ln_approx_D38s19");
    g.sample_size(50);
    g.measurement_time(std::time::Duration::from_secs(2));

    // ln(3) — a non-trivial arg that misses all fast paths.
    let arg = at("3");

    g.bench_function("ln_strict_guard30", |bn| {
        bn.iter(|| black_box(arg).ln_strict());
    });
    for g_d in [6u32, 10, 15, 20, 25] {
        g.bench_function(format!("ln_approx_guard{g_d}"), |bn| {
            bn.iter(|| black_box(arg).ln_approx(g_d));
        });
    }

    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
