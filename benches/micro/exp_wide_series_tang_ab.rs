// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier exp Series-vs-Tang map (the N-way width × scale × algorithm
//! sweep), the empirical re-derivation that supersedes the stale
//! `research/2026_05_25_*` wide-exp framing.
//!
//! The `bench-branch-compare` `exp/D###` cells all run at **SCALE 30** (operand
//! 0.5) — `bench-compare/benches/compare_d{307..1232}.rs`. 0.4.4 routed wide
//! exp (D307–D1232) through the Smith `wide_kernel` (Series), NOT Tang — it
//! trialed and REJECTED Tang for wide exp on perf grounds (D462 ~75% slower,
//! D616 break-even). This bench checks that empirically against the CURRENT
//! 0.5.0 Series (now with the u128-packed square): for each wide tier, at the
//! bbc SCALE 30 and a representative higher scale, it asserts Tang ==
//! Series across the operand spread × all six modes (the **validity wall** —
//! if they disagree, single-shot Tang is not correctly-rounded there and the
//! cell is reported INVALID and skipped), then ranks Tang vs Series.
//!
//! Wire a wide Tang `select`/`tang_routed` arm ONLY for a cell that is BOTH
//! bit-identical AND faster here.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench exp_wide_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    exp_series_d1232, exp_series_d307, exp_series_d462, exp_series_d616, exp_series_d924,
    exp_tang_d1232, exp_tang_d307, exp_tang_d462, exp_tang_d616, exp_tang_d924, int_from_mag_limbs,
};

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

/// Build an `Int<N>` from a non-negative magnitude (low two u64 limbs).
fn fromu<const N: usize>(v: u128) -> Int<N> {
    let mut mag = [0u64; N];
    mag[0] = v as u64;
    if N > 1 {
        mag[1] = (v >> 64) as u64;
    }
    int_from_mag_limbs::<N>(&mag)
}

#[derive(Clone)]
struct One<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// exp arguments at decimal `SCALE`: x in {0.5, 1.5, 3.0} → raw = x·10^SCALE.
/// 0.5 is the bbc operand; the others stay inside the shared reduction window.
fn exp_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale);
    vec![
        One { label: "x0.5", raw: fromu::<N>(p / 2) },
        One { label: "x1.5", raw: fromu::<N>(p + p / 2) },
        One { label: "x3.0", raw: fromu::<N>(3 * p) },
    ]
}

/// One cell: assert Tang == Series across the spread × all modes (the validity
/// wall). If any disagree, report INVALID and skip timing. Otherwise rank them.
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: fn(Int<N>, RoundingMode) -> Int<N>,
    tang: fn(Int<N>, RoundingMode) -> Int<N>,
) {
    let mut valid = true;
    'outer: for o in exp_inputs::<N>(scale) {
        for m in ALL_MODES {
            if tang(o.raw, m) != series(o.raw, m) {
                println!("VALIDITY [{group}]: Tang != Series ({}, mode {m:?}) -> INVALID, skipping", o.label);
                valid = false;
                break 'outer;
            }
        }
    }
    if !valid {
        return;
    }
    compare_all(
        c,
        group,
        |o: &One<N>| o.label.to_string(),
        exp_inputs::<N>(scale),
        vec![
            ("tang", Box::new(move |o: One<N>| tang(o.raw, MODE)) as Box<dyn Fn(One<N>) -> Int<N>>),
            ("series", Box::new(move |o: One<N>| series(o.raw, MODE))),
        ],
    );
}

fn benches(c: &mut Criterion) {
    // D307 (Int<16>): bbc SCALE 30 + the 0.4.4 narrow band 150. M=128/512, G=30.
    cell::<16>(c, "exp_d307_s30_m128", 30, exp_series_d307::<30>, exp_tang_d307::<30, 128, 30>);
    cell::<16>(c, "exp_d307_s30_m512", 30, exp_series_d307::<30>, exp_tang_d307::<30, 512, 30>);
    cell::<16>(c, "exp_d307_s150_m128", 150, exp_series_d307::<150>, exp_tang_d307::<150, 128, 30>);
    // D462 (Int<24>): bbc SCALE 30 + band 230.
    cell::<24>(c, "exp_d462_s30_m128", 30, exp_series_d462::<30>, exp_tang_d462::<30, 128, 30>);
    cell::<24>(c, "exp_d462_s30_m512", 30, exp_series_d462::<30>, exp_tang_d462::<30, 512, 30>);
    cell::<24>(c, "exp_d462_s230_m128", 230, exp_series_d462::<230>, exp_tang_d462::<230, 128, 30>);
    // D616 (Int<32>): bbc SCALE 30 + band 308.
    cell::<32>(c, "exp_d616_s30_m128", 30, exp_series_d616::<30>, exp_tang_d616::<30, 128, 30>);
    cell::<32>(c, "exp_d616_s30_m512", 30, exp_series_d616::<30>, exp_tang_d616::<30, 512, 30>);
    cell::<32>(c, "exp_d616_s308_m128", 308, exp_series_d616::<308>, exp_tang_d616::<308, 128, 30>);
    // D924 (Int<48>): bbc SCALE 30 + band 461.
    cell::<48>(c, "exp_d924_s30_m128", 30, exp_series_d924::<30>, exp_tang_d924::<30, 128, 30>);
    cell::<48>(c, "exp_d924_s30_m512", 30, exp_series_d924::<30>, exp_tang_d924::<30, 512, 30>);
    cell::<48>(c, "exp_d924_s461_m512", 461, exp_series_d924::<461>, exp_tang_d924::<461, 512, 30>);
    // D1232 (Int<64>): bbc SCALE 30 + band 616.
    cell::<64>(c, "exp_d1232_s30_m128", 30, exp_series_d1232::<30>, exp_tang_d1232::<30, 128, 30>);
    cell::<64>(c, "exp_d1232_s30_m512", 30, exp_series_d1232::<30>, exp_tang_d1232::<30, 512, 30>);
    cell::<64>(c, "exp_d1232_s616_m512", 616, exp_series_d1232::<616>, exp_tang_d1232::<616, 512, 30>);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
