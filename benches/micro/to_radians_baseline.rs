// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Per-call timing of the 0.5.0 public `to_radians` matched to the
//! `bench-branch-compare` cells (input `0.1`, default rounding, the
//! per-tier {30, S-1} scales) and to the v0.4.4 `to_radians_v044`
//! baseline so the numbers compare directly.
//!
//! Run: `cargo bench --features wide --bench to_radians_baseline`

use core::str::FromStr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::{D115, D153, D57, D76};

macro_rules! cell {
    ($c:expr, $grp:literal, $ty:ty) => {{
        let mut g = $c.benchmark_group($grp);
        let x = <$ty>::from_str("0.1").unwrap();
        g.bench_function("0.1", |b| b.iter(|| black_box(x).to_radians()));
        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    cell!(c, "to_radians/D57_s30", D57<30>);
    cell!(c, "to_radians/D57_s56", D57<56>);
    cell!(c, "to_radians/D76_s30", D76<30>);
    cell!(c, "to_radians/D76_s75", D76<75>);
    cell!(c, "to_radians/D115_s30", D115<30>);
    cell!(c, "to_radians/D115_s114", D115<114>);
    cell!(c, "to_radians/D153_s30", D153<30>);
    cell!(c, "to_radians/D153_s152", D153<152>);
}

criterion_group!(benches, bench);
criterion_main!(benches);
