// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the forward-trig (`sin`) Series-vs-Tang policy choice
//! (`src/policy/trig.rs`, `forward::select`).
//!
//! `forward::select` routes a discrete set of `(N, SCALE)` bands to the Tang
//! table-driven kernel (`sin_tang_with_taylor`) and everything else to the
//! generic `sin_series`. Those bands were tuned via the standalone
//! `benches/lookup/*` kernel-ISOLATION benches — never an N-way `compare_all`
//! of Series-vs-Tang at the dispatch seam. This bench closes that gap for the
//! one band cleanly buildable under plain `wide`:
//!
//! - D57 (Int<3>): band 44..=56, M=512.
//!
//! NOTE on the other forward tiers: the wide-tier `wide_trig_forward_series!`
//! macro (used by N=4/6/12/32/48/64) routes BOTH its `Series` and `Tang` match
//! arms to `sin_series` — i.e. those tiers have NO real Tang wiring, and
//! `forward::select` never returns `Tang` at those N anyway (consistent, not a
//! live bug, but worth knowing). Only N ∈ {3, 8, 16, 24} have a real
//! `sin_tang_with_taylor` wiring; D153/D307/D462 (N=8/16/24) are bespoke impls
//! and need x-wide builds + their own export cells — left for a follow-up.
//!
//! OPTIMALITY A/B over a VALID region: Tang and Series are bit-identical
//! in-band (both correctly-rounded), asserted across the spread × six modes
//! before timing. sin arguments x ∈ {0.3, 1.0, 1.5} rad (inside [-π/2, π/2]
//! style residues both kernels reduce to). A just-out-of-band probe runs Tang
//! at a SCALE the policy does NOT route to it; if it asserts unequal, that
//! scale is past Tang's validity (treat the in-band edge as the validity wall).
//!
//! Run with:
//! `cargo bench --features "wide bench-alt" --bench trig_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{int_from_mag_limbs, sin_series_d57, sin_tang_d57};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;
const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

fn fromu<const N: usize>(v: u128) -> Int<N> {
    let mut mag = [0u64; N];
    mag[0] = v as u64;
    if N > 1 {
        mag[1] = (v >> 64) as u64;
    }
    int_from_mag_limbs::<N>(&mag)
}

#[derive(Clone)]
struct One {
    label: &'static str,
    raw: Int<3>,
}

/// x ∈ {0.3, 1.0, 1.5} rad at decimal `SCALE` → raw = x · 10^SCALE.
fn sin_inputs(scale: u32) -> Vec<One> {
    let p = 10u128.pow(scale);
    vec![
        One { label: "x0.3", raw: fromu::<3>(3 * p / 10) },
        One { label: "x1.0", raw: fromu::<3>(p) },
        One { label: "x1.5", raw: fromu::<3>(p + p / 2) },
    ]
}

fn cell(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: fn(Int<3>, RoundingMode) -> Int<3>,
    tang: fn(Int<3>, RoundingMode) -> Int<3>,
) {
    for o in sin_inputs(scale) {
        for m in ALL_MODES {
            assert_eq!(tang(o.raw, m), series(o.raw, m), "{group} {} mode {m:?}", o.label);
        }
    }
    compare_all(
        c,
        group,
        |o: &One| o.label.to_string(),
        sin_inputs(scale),
        vec![
            ("tang", Box::new(move |o: One| tang(o.raw, MODE)) as Box<dyn Fn(One) -> Int<3>>),
            ("series", Box::new(move |o: One| series(o.raw, MODE))),
        ],
    );
}

fn benches(c: &mut Criterion) {
    // D57 forward-sin band 44..=56 (M=512): in-band edges + just-out probes.
    cell(c, "sin_d57_s44_in", 44, sin_series_d57::<44>, sin_tang_d57::<44>);
    cell(c, "sin_d57_s56_in", 56, sin_series_d57::<56>, sin_tang_d57::<56>);
    cell(c, "sin_d57_s43_out", 43, sin_series_d57::<43>, sin_tang_d57::<43>);
    cell(c, "sin_d57_s57_out", 57, sin_series_d57::<57>, sin_tang_d57::<57>);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
