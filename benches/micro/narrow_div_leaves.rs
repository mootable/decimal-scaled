// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Per-call timing of the D38 public ops that route through the narrow-tier
//! divide leaves in `algos::support::{mg_divide, fixed}` — the cells where
//! the bit-serial shift-subtract fallbacks fire (divisor above `u64::MAX`):
//!
//! - `to_degrees` — `Fixed::div` by full-precision pi (`div_u512_by_u256`;
//!   the divisor `pi · 10^w` is above `u64::MAX` at every working scale).
//! - `cbrt`       — `icbrt_384`'s per-Newton-iteration `div_384_by_256`.
//! - `acosh`      — `Fixed::sqrt` → `isqrt_u512` → `div_u512_by_u256` per
//!   Newton iteration (plus the seed's iteration count).
//! - `atan`       — the halving cascade: `Fixed::div` + `Fixed::sqrt` per
//!   rung.
//! - `to_radians` — CONTROL: divides by 180 via `div_small`'s u64 fast
//!   path, so it should not move when the wide-divisor leaves change.
//!
//! Before/after comparison comes from criterion's saved-baseline change
//! report: run once on the base commit, again after the kernel change, and
//! read the `change:` lines.
//!
//! Run: `powershell.exe -NoProfile -File scripts/pin_run.ps1 -Core 22 -Bench narrow_div_leaves`

use core::str::FromStr;
use criterion::{black_box, Criterion};
use decimal_scaled::D38;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::micro_criterion;

fn bench(c: &mut Criterion) {
    // to_degrees: the divisor is always full-precision pi at the working
    // scale, so the wide-divisor divide fires on every call at every SCALE.
    {
        let mut g = c.benchmark_group("narrow_div/to_degrees");
        let x0 = D38::<0>::from_str("45").unwrap();
        g.bench_function("D38_s0_45", |b| b.iter(|| black_box(x0).to_degrees_strict()));
        let x9 = D38::<9>::from_str("0.1").unwrap();
        g.bench_function("D38_s9_0.1", |b| b.iter(|| black_box(x9).to_degrees_strict()));
        let x30 = D38::<30>::from_str("0.1").unwrap();
        g.bench_function("D38_s30_0.1", |b| b.iter(|| black_box(x30).to_degrees_strict()));
        let x37 = D38::<37>::from_str("0.1").unwrap();
        g.bench_function("D38_s37_0.1", |b| b.iter(|| black_box(x37).to_degrees_strict()));
        g.finish();
    }
    // Control: same file, same shape, but the divisor 180 takes `div_small`'s
    // u64 hardware fast path — should not move with the wide-divisor fix.
    {
        let mut g = c.benchmark_group("narrow_div/to_radians_control");
        let x30 = D38::<30>::from_str("0.1").unwrap();
        g.bench_function("D38_s30_0.1", |b| b.iter(|| black_box(x30).to_radians_strict()));
        g.finish();
    }
    // cbrt: `div_384_by_256` per Newton iteration (slow even at SCALE 0).
    {
        let mut g = c.benchmark_group("narrow_div/cbrt");
        let x0 = D38::<0>::from_str("987654321987654321").unwrap();
        g.bench_function("D38_s0_9.9e17", |b| b.iter(|| black_box(x0).cbrt_strict()));
        let x20 = D38::<20>::from_str("1234.56789").unwrap();
        g.bench_function("D38_s20_1234.6", |b| b.iter(|| black_box(x20).cbrt_strict()));
        g.finish();
    }
    // acosh: `Fixed::sqrt` → `isqrt_u512` → `div_u512_by_u256` per iteration.
    {
        let mut g = c.benchmark_group("narrow_div/acosh");
        let x = D38::<30>::from_str("1.5").unwrap();
        g.bench_function("D38_s30_1.5", |b| b.iter(|| black_box(x).acosh_strict()));
        g.finish();
    }
    // atan: the halving cascade (`Fixed::div` + `Fixed::sqrt` per rung).
    {
        let mut g = c.benchmark_group("narrow_div/atan");
        let x = D38::<30>::from_str("0.7").unwrap();
        g.bench_function("D38_s30_0.7", |b| b.iter(|| black_box(x).atan_strict()));
        g.finish();
    }
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
