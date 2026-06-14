// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Focused decimal multiply microbench for the wide-tier regression
//! cells (`mul_D230_s229` and `mul_D924_s462`).
//!
//! Measures the `*` operator on `D230<229>` and `D924<462>` (the
//! `mul_widen_divide` slow path -- full 2N product then divide by
//! 10^SCALE). Run pinned on cores 20-23 for clean numbers:
//!
//!   powershell.exe -NoProfile -File scripts/pin_run.ps1 \
//!     -Mask 0xF00000 -Bench dec_mul_wide_ab \
//!     -Features "wide x-wide xx-wide"

use criterion::{BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime};
use std::hint::black_box;

#[cfg(all(feature = "wide", feature = "x-wide"))]
use decimal_scaled::D230;
#[cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]
use decimal_scaled::D924;

fn bench_group<T, F>(g: &mut BenchmarkGroup<WallTime>, id: &str, mut f: F)
where
    F: FnMut() -> T,
{
    g.bench_function(id, |b| b.iter(&mut f));
}

#[cfg(all(feature = "wide", feature = "x-wide"))]
fn bench_d230(c: &mut Criterion) {
    let mut g = c.benchmark_group("dec_mul_wide");
    g.sample_size(20);
    g.warm_up_time(std::time::Duration::from_millis(300));
    g.measurement_time(std::time::Duration::from_millis(600));

    // Full-magnitude operands -- forces the slow (2N-wide) path.
    let a: D230<229> = D230::try_from(999_999_999_999_i64).unwrap();
    let b: D230<229> = D230::try_from(999_999_999_997_i64).unwrap();
    bench_group(&mut g, "D230_s229/mul", || black_box(a) * black_box(b));

    // Half-scale neighbour widths for coverage.
    let a115: D230<115> = D230::try_from(999_999_999_999_i64).unwrap();
    let b115: D230<115> = D230::try_from(999_999_999_997_i64).unwrap();
    bench_group(&mut g, "D230_s115/mul", || black_box(a115) * black_box(b115));

    g.finish();
}

#[cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]
fn bench_d924(c: &mut Criterion) {
    let mut g = c.benchmark_group("dec_mul_wide");
    g.sample_size(20);
    g.warm_up_time(std::time::Duration::from_millis(300));
    g.measurement_time(std::time::Duration::from_millis(600));

    let a: D924<462> = D924::try_from(999_999_999_999_i64).unwrap();
    let b: D924<462> = D924::try_from(999_999_999_997_i64).unwrap();
    bench_group(&mut g, "D924_s462/mul", || black_box(a) * black_box(b));

    let a231: D924<231> = D924::try_from(999_999_999_999_i64).unwrap();
    let b231: D924<231> = D924::try_from(999_999_999_997_i64).unwrap();
    bench_group(&mut g, "D924_s231/mul", || black_box(a231) * black_box(b231));

    g.finish();
}

#[cfg(all(feature = "wide", feature = "x-wide"))]
criterion_group!(benches_d230, bench_d230);
#[cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]
criterion_group!(benches_d924, bench_d924);

#[cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]
criterion_main!(benches_d230, benches_d924);
#[cfg(all(feature = "wide", feature = "x-wide", not(feature = "xx-wide")))]
criterion_main!(benches_d230);
