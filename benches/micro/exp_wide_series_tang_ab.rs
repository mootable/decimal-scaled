// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier exp Series-vs-Tang map (the N-way width × scale × algorithm ×
//! table-size sweep). For each wide tier, at a set of scales, it asserts each
//! Tang candidate == Series across the operand spread × all six modes (the
//! **validity wall** — if any disagree, single-shot Tang is not correctly-
//! rounded there, that candidate is reported INVALID and dropped), then ranks
//! the surviving candidates against Series.
//!
//! Wire a wide Tang `select`/`tang_routed` arm ONLY for a cell where a Tang
//! candidate is BOTH bit-identical to Series AND faster here.
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

/// `acc = acc * m + add`, in place across the little-endian limb array. Pure
/// big-int limb arithmetic so any decimal scale (even scale >= 39, where
/// `10^scale` overflows u128) is representable in the wide `Int<N>`.
fn mul_add_small<const N: usize>(acc: &mut [u64; N], m: u64, add: u64) {
    let mut carry = add as u128;
    for limb in acc.iter_mut() {
        let prod = (*limb as u128) * (m as u128) + carry;
        *limb = prod as u64;
        carry = prod >> 64;
    }
}

/// Build raw = (x_num/x_den) * 10^scale directly in `Int<N>` limbs, never via
/// `10u128.pow` (which overflows for scale >= 39). Computes `x_num * 10^scale`
/// limb-wise then divides exactly by the small denominator (2 or 1 here).
fn build_raw<const N: usize>(x_num: u64, x_den: u64, scale: u32) -> Int<N> {
    let mut mag = [0u64; N];
    mag[0] = x_num;
    for _ in 0..scale {
        mul_add_small::<N>(&mut mag, 10, 0);
    }
    if x_den != 1 {
        let mut rem = 0u128;
        for limb in mag.iter_mut().rev() {
            let cur = (rem << 64) | (*limb as u128);
            *limb = (cur / x_den as u128) as u64;
            rem = cur % x_den as u128;
        }
    }
    int_from_mag_limbs::<N>(&mag)
}

#[derive(Clone)]
struct One<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// exp arguments at decimal `SCALE`: x in {0.5, 1.5, 3.0} -> raw = x*10^SCALE.
/// 0.5 is the bbc operand; the others stay inside the shared reduction window.
fn exp_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![
        One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<N>(3, 2, scale) },
        One { label: "x3.0", raw: build_raw::<N>(3, 1, scale) },
    ]
}

type ExpFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

/// One cell: validate each Tang candidate against Series (bit-identical across
/// the spread x all modes), drop the invalid ones, then rank the survivors and
/// Series in a single `compare_all` run.
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: ExpFn<N>,
    candidates: &[(&'static str, ExpFn<N>)],
) {
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("series", Box::new(move |o: One<N>| series(o.raw, MODE)))];
    for &(label, tang) in candidates {
        let mut valid = true;
        'outer: for o in exp_inputs::<N>(scale) {
            for m in ALL_MODES {
                if tang(o.raw, m) != series(o.raw, m) {
                    println!("VALIDITY [{group}]: {label} != series ({}, mode {m:?}) -> INVALID, skipping", o.label);
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            runs.push((label, Box::new(move |o: One<N>| tang(o.raw, MODE))));
        }
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), exp_inputs::<N>(scale), runs);
}

fn benches(c: &mut Criterion) {
    // The bbc `exp/D###` cells run at SCALE 30 (operand 0.5). For each wide
    // tier rank Series vs Tang at M=128 and M=512 (G=30) — the production wide
    // arms currently pin M=128, so M=512 is the open candidate. A higher
    // representative scale per tier maps the Tang->Series crossover with width.
    // D307 (Int<16>).
    cell::<16>(c, "exp_d307_s30", 30, exp_series_d307::<30>,
        &[("tang_m128", exp_tang_d307::<30, 128, 30>), ("tang_m512", exp_tang_d307::<30, 512, 30>)]);
    cell::<16>(c, "exp_d307_s150", 150, exp_series_d307::<150>,
        &[("tang_m128", exp_tang_d307::<150, 128, 30>), ("tang_m512", exp_tang_d307::<150, 512, 30>)]);
    // D462 (Int<24>).
    cell::<24>(c, "exp_d462_s30", 30, exp_series_d462::<30>,
        &[("tang_m128", exp_tang_d462::<30, 128, 30>), ("tang_m512", exp_tang_d462::<30, 512, 30>)]);
    cell::<24>(c, "exp_d462_s230", 230, exp_series_d462::<230>,
        &[("tang_m128", exp_tang_d462::<230, 128, 30>), ("tang_m512", exp_tang_d462::<230, 512, 30>)]);
    // D616 (Int<32>).
    cell::<32>(c, "exp_d616_s30", 30, exp_series_d616::<30>,
        &[("tang_m128", exp_tang_d616::<30, 128, 30>), ("tang_m512", exp_tang_d616::<30, 512, 30>)]);
    cell::<32>(c, "exp_d616_s308", 308, exp_series_d616::<308>,
        &[("tang_m128", exp_tang_d616::<308, 128, 30>), ("tang_m512", exp_tang_d616::<308, 512, 30>)]);
    // D924 (Int<48>).
    cell::<48>(c, "exp_d924_s30", 30, exp_series_d924::<30>,
        &[("tang_m128", exp_tang_d924::<30, 128, 30>), ("tang_m512", exp_tang_d924::<30, 512, 30>)]);
    cell::<48>(c, "exp_d924_s461", 461, exp_series_d924::<461>,
        &[("tang_m128", exp_tang_d924::<461, 128, 30>), ("tang_m512", exp_tang_d924::<461, 512, 30>)]);
    // D1232 (Int<64>).
    cell::<64>(c, "exp_d1232_s30", 30, exp_series_d1232::<30>,
        &[("tang_m128", exp_tang_d1232::<30, 128, 30>), ("tang_m512", exp_tang_d1232::<30, 512, 30>)]);
    cell::<64>(c, "exp_d1232_s616", 616, exp_series_d1232::<616>,
        &[("tang_m128", exp_tang_d1232::<616, 128, 30>), ("tang_m512", exp_tang_d1232::<616, 512, 30>)]);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
