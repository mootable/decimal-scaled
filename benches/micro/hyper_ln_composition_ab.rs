// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `acosh` / `atanh` routing cost at the dispatch seam.
//!
//! Both entry points route through `policy::trig`'s inverse-hyperbolic
//! matcher, whose two arms are the single-shot `LnComposition` kernel
//! (`algos::trig::hyper_ln_composition`) and the Ziv-escalating
//! `Schoolbook` reference reached via the near-special work rung. This
//! bench measures the PUBLIC entry, so whichever arm `select` currently
//! returns is what is timed: run it once as shipped, then again with
//! `inverse_hyper::select` flipped to `Schoolbook`, and compare.
//!
//! The cells are the ones that regressed when every cell was put on the
//! schoolbook arm: D57 s28, D115 s86, D230 s115, D616 s462.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

use decimal_scaled::{D115, D230, D57, D616};

/// Criterion preset matching `benches/support/ab_microbench.rs`: short
/// enough to stay under a minute, long enough to be stable.
fn micro() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .warm_up_time(std::time::Duration::from_millis(150))
        .measurement_time(std::time::Duration::from_millis(400))
}

macro_rules! cell {
    ($group:expr, $label:literal, $ty:ty) => {{
        type T = $ty;
        let one = <T>::ONE;
        let ulp = <T>::MIN_POSITIVE;
        // acosh: just above the domain wall (the near-1 `log1p` branch,
        // where the radicand correction lives) and well clear of it
        // (the `x >= 2` factored branch).
        let near_one = one + ulp;
        let far = <T>::try_from(7i64).expect("7 fits every benched cell");
        // atanh: hard against the +1 wall, and mid-range.
        let near_wall = one - ulp;
        let mid = one / (one + one);

        $group.bench_with_input(BenchmarkId::new("acosh_near1", $label), &near_one, |b, x| {
            b.iter(|| black_box(black_box(*x).acosh()))
        });
        $group.bench_with_input(BenchmarkId::new("acosh_far", $label), &far, |b, x| {
            b.iter(|| black_box(black_box(*x).acosh()))
        });
        $group.bench_with_input(BenchmarkId::new("atanh_wall", $label), &near_wall, |b, x| {
            b.iter(|| black_box(black_box(*x).atanh()))
        });
        $group.bench_with_input(BenchmarkId::new("atanh_mid", $label), &mid, |b, x| {
            b.iter(|| black_box(black_box(*x).atanh()))
        });
    }};
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("inverse_hyper_route");
    cell!(group, "D57_s28", D57<28>);
    cell!(group, "D115_s86", D115<86>);
    cell!(group, "D230_s115", D230<115>);
    cell!(group, "D616_s462", D616<462>);
    group.finish();
}

criterion_group! {
    name = benches;
    config = micro();
    targets = bench
}
criterion_main!(benches);
