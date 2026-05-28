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
//! Scale spread per tier: {0, 30, S/2, S-1} where S is the tier's design max
//! scale. The S-1 cell is the MAX-SCALE EXTREME — previously UNTESTED, and
//! where the bench-branch-compare regressions live (`exp_D76_s75 +1160%`,
//! `powf_D76_s75 +838%`). D76 (Int<4>) and D230 (Int<12>) had NO A/B export
//! at all and are added here.
//!
//! Tang configs probed per cell: M=128/G=30 (the production wide config),
//! M=512/G=30 (wider table), and M=512/G=60 (wider table + wider guard — the
//! candidate for the max-scale extreme, where the narrow guard may be too thin
//! for single-shot correctness or the larger table amortises better).
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench exp_wide_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    exp_series_d115, exp_series_d1232, exp_series_d153, exp_series_d230, exp_series_d307,
    exp_series_d462, exp_series_d616, exp_series_d76, exp_series_d924, exp_tang_d115_p,
    exp_tang_d1232, exp_tang_d153_p, exp_tang_d230, exp_tang_d307, exp_tang_d462, exp_tang_d616,
    exp_tang_d76, exp_tang_d924, int_from_mag_limbs,
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
    // compare_all needs >= 2 survivors; if every Tang candidate is INVALID the
    // cell is Series-only (report it and skip the timed run).
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all Tang candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), exp_inputs::<N>(scale), runs);
}

/// One (tier, scale) cell: Series vs the three Tang configs (M=128/G=30 = the
/// production wide config; M=512/G=30 = wider table; M=512/G=60 = wider table
/// + wider guard, the max-scale-extreme candidate). SCALE is a literal const
/// generic so each cell is its own monomorphisation, as the policy sees it.
macro_rules! cell4 {
    ($c:expr, $n:literal, $name:literal, $scale:literal, $series:ident, $tang:ident) => {
        cell::<$n>(
            $c,
            concat!("exp_", $name, "_s", stringify!($scale)),
            $scale,
            $series::<$scale>,
            &[
                ("tang_m128_g30", $tang::<$scale, 128, 30>),
                ("tang_m512_g30", $tang::<$scale, 512, 30>),
                ("tang_m512_g60", $tang::<$scale, 512, 60>),
            ],
        );
    };
}

/// Sweep a tier across the FIVE coarse scale points
/// `{0, S/4, S/2, 3S/4, S-1}` (the owner-standard sampling) as literals.
macro_rules! tier {
    ($c:expr, $n:literal, $name:literal, $series:ident, $tang:ident,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        cell4!($c, $n, $name, $s0, $series, $tang);
        cell4!($c, $n, $name, $s1, $series, $tang);
        cell4!($c, $n, $name, $s2, $series, $tang);
        cell4!($c, $n, $name, $s3, $series, $tang);
        cell4!($c, $n, $name, $s4, $series, $tang);
    }};
}

fn benches(c: &mut Criterion) {
    // Per tier: scales {0, S/4, S/2, 3S/4, S-1} (the owner-standard 5-point
    // coarse sampling). The TOP point is the tier's MAX-SCALE EXTREME, capped
    // at `capacity - 2` so the x3.0 input (`e^3 ≈ 20`, two integer digits)
    // still fits storage — a larger top scale leaves < 2 integer digits and
    // the strict-transcendental range check panics during cell setup.
    // D76 (Int<4>, cap 76) — the bbc `exp_D76_s75`/`powf_D76_s75` regression.
    tier!(c, 4, "d76", exp_series_d76, exp_tang_d76, 0, 19, 38, 57, 74);
    // D115 (Int<6>, cap 115).
    tier!(c, 6, "d115", exp_series_d115, exp_tang_d115_p, 0, 28, 57, 86, 113);
    // D153 (Int<8>, cap 153).
    tier!(c, 8, "d153", exp_series_d153, exp_tang_d153_p, 0, 38, 76, 114, 151);
    // D230 (Int<12>, cap 230).
    tier!(c, 12, "d230", exp_series_d230, exp_tang_d230, 0, 57, 115, 172, 228);
    // D307 (Int<16>, cap 307).
    tier!(c, 16, "d307", exp_series_d307, exp_tang_d307, 0, 76, 153, 230, 305);
    // D462 (Int<24>, cap 462).
    tier!(c, 24, "d462", exp_series_d462, exp_tang_d462, 0, 115, 231, 346, 460);
    // D616 (Int<32>, cap 616).
    tier!(c, 32, "d616", exp_series_d616, exp_tang_d616, 0, 154, 308, 462, 614);
    // D924 (Int<48>, cap 924).
    tier!(c, 48, "d924", exp_series_d924, exp_tang_d924, 0, 231, 462, 693, 922);
    // D1232 (Int<64>, cap 1232).
    tier!(c, 64, "d1232", exp_series_d1232, exp_tang_d1232, 0, 308, 616, 924, 1230);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
