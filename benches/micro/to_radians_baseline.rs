// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Per-call timing of the 0.5.0 public `to_radians_strict_with`, matched
//! to the v0.4.4 `to_radians_v044` baseline bench (same tiers, scales,
//! inputs, and `from_str` construction) so the numbers compare directly.
//!
//! Run: `cargo bench --features wide --bench to_radians_baseline`

use core::str::FromStr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use decimal_scaled::{RoundingMode, D115, D153, D57, D76};

const MODE: RoundingMode = RoundingMode::HalfToEven;

fn bench(c: &mut Criterion) {
    {
        let mut g = c.benchmark_group("to_radians/D57_s19");
        for (lbl, deg) in [("30deg", "30"), ("45deg", "45"), ("180deg", "180")] {
            let x = D57::<19>::from_str(deg).unwrap();
            g.bench_function(lbl, |b| b.iter(|| black_box(x).to_radians_strict_with(MODE)));
        }
        g.finish();
    }
    {
        let mut g = c.benchmark_group("to_radians/D76_s24");
        for (lbl, deg) in [("30deg", "30"), ("45deg", "45"), ("180deg", "180")] {
            let x = D76::<24>::from_str(deg).unwrap();
            g.bench_function(lbl, |b| b.iter(|| black_box(x).to_radians_strict_with(MODE)));
        }
        g.finish();
    }
    {
        let mut g = c.benchmark_group("to_radians/D115_s38");
        for (lbl, deg) in [("30deg", "30"), ("45deg", "45"), ("180deg", "180")] {
            let x = D115::<38>::from_str(deg).unwrap();
            g.bench_function(lbl, |b| b.iter(|| black_box(x).to_radians_strict_with(MODE)));
        }
        g.finish();
    }
    {
        let mut g = c.benchmark_group("to_radians/D153_s50");
        for (lbl, deg) in [("30deg", "30"), ("45deg", "45"), ("180deg", "180")] {
            let x = D153::<50>::from_str(deg).unwrap();
            g.bench_function(lbl, |b| b.iter(|| black_box(x).to_radians_strict_with(MODE)));
        }
        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
