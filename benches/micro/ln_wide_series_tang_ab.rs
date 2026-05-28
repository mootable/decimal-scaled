// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier ln Series-vs-Tang map (the N-way width x scale x algorithm x
//! GUARD sweep). For each wide tier, at the 5-point scale set
//! {0, S/4, S/2, 3S/4, S-1}, asserts each Tang candidate == Series across
//! the operand spread x all six modes (the **validity wall** -- if any
//! disagree, single-shot Tang is not correctly-rounded there, that candidate
//! is reported INVALID and dropped), then ranks the surviving Tang
//! candidates against Series.
//!
//! Wire a wide Tang `select`/`tang_routed` arm ONLY for a cell where a Tang
//! candidate is BOTH bit-identical to Series AND faster here.
//!
//! Tang configs probed per cell: (G=8, CAP=200) the production narrow-wide
//! config; (G=10, CAP=400) the wider-guard / wider-cap config the wider
//! tiers already use; (G=12, CAP=400) the max-scale-extreme candidate.
//!
//! Establishes the continuous win-region for each wide tier so the
//! `policy::ln::tang_routed` gates can be widened off their bbc-cell point
//! ranges (the Class-I single-cell fit) onto the bisected true crossover.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench ln_wide_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    int_from_mag_limbs, ln_series_d1232, ln_series_d230, ln_series_d307, ln_series_d462,
    ln_series_d616, ln_series_d76, ln_series_d924, ln_tang_d1232_p, ln_tang_d230_p,
    ln_tang_d307_p, ln_tang_d462_p, ln_tang_d616_p, ln_tang_d76_p, ln_tang_d924_p,
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
/// limb-wise then divides exactly by the small denominator.
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

/// ln arguments at decimal `SCALE`: x in {0.5, 2.0, 7.5} -> raw = x*10^SCALE.
/// All > 0 (ln domain) and inside the shared reduction window at every tier.
/// SCALE 0 is special-cased to x in {1, 2, 7} (the integer truncations of
/// the spread) so the operand stays > 0 at the boundary.
fn ln_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    if scale == 0 {
        vec![
            One { label: "x1", raw: build_raw::<N>(1, 1, 0) },
            One { label: "x2", raw: build_raw::<N>(2, 1, 0) },
            One { label: "x7", raw: build_raw::<N>(7, 1, 0) },
        ]
    } else {
        vec![
            One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
            One { label: "x2.0", raw: build_raw::<N>(2, 1, scale) },
            One { label: "x7.5", raw: build_raw::<N>(15, 2, scale) },
        ]
    }
}

type LnFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

/// One cell: validate each Tang candidate against Series (bit-identical
/// across the spread x all modes), drop the invalid ones, then rank the
/// survivors and Series in a single `compare_all` run.
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: LnFn<N>,
    candidates: &[(&'static str, LnFn<N>)],
) {
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("series", Box::new(move |o: One<N>| series(o.raw, MODE)))];
    for &(label, tang) in candidates {
        let mut valid = true;
        'outer: for o in ln_inputs::<N>(scale) {
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
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all Tang candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), ln_inputs::<N>(scale), runs);
}

/// One (tier, scale) cell: Series vs three Tang configs (G=8/CAP=200 = the
/// narrow-wide production config; G=10/CAP=400 = wider guard + wider cap
/// the wider tiers use; G=12/CAP=400 = wider-guard max-scale-extreme
/// candidate). SCALE is a literal const generic so each cell is its own
/// monomorphisation, as the policy sees it.
macro_rules! cell_ln {
    ($c:expr, $n:literal, $name:literal, $scale:literal, $series:ident, $tang:ident) => {
        cell::<$n>(
            $c,
            concat!("ln_", $name, "_s", stringify!($scale)),
            $scale,
            $series::<$scale>,
            &[
                ("tang_g8_c200", $tang::<$scale, 8, 200>),
                ("tang_g10_c400", $tang::<$scale, 10, 400>),
                ("tang_g12_c400", $tang::<$scale, 12, 400>),
            ],
        );
    };
}

/// Sweep a tier across the FIVE coarse scale points
/// `{0, S/4, S/2, 3S/4, S-1}` (the owner-standard sampling) as literals.
macro_rules! tier {
    ($c:expr, $n:literal, $name:literal, $series:ident, $tang:ident,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        cell_ln!($c, $n, $name, $s0, $series, $tang);
        cell_ln!($c, $n, $name, $s1, $series, $tang);
        cell_ln!($c, $n, $name, $s2, $series, $tang);
        cell_ln!($c, $n, $name, $s3, $series, $tang);
        cell_ln!($c, $n, $name, $s4, $series, $tang);
    }};
}

fn benches(c: &mut Criterion) {
    // Per tier: scales {0, S/4, S/2, 3S/4, S-1} (the owner-standard 5-point
    // coarse sampling). The top point is the tier's MAX-SCALE EXTREME; ln's
    // domain x > 0 plus the operand spread (0.5/2/7.5) fits storage at every
    // wide tier through s = cap-1 (unlike exp where the 3.0 input cannot
    // reach top scales).
    // D76 (Int<4>, cap 76).
    tier!(c, 4, "d76", ln_series_d76, ln_tang_d76_p, 0, 19, 38, 57, 75);
    // D230 (Int<12>, cap 230).
    tier!(c, 12, "d230", ln_series_d230, ln_tang_d230_p, 0, 57, 115, 172, 229);
    // D307 (Int<16>, cap 307).
    tier!(c, 16, "d307", ln_series_d307, ln_tang_d307_p, 0, 76, 153, 230, 306);
    // D462 (Int<24>, cap 462).
    tier!(c, 24, "d462", ln_series_d462, ln_tang_d462_p, 0, 115, 231, 346, 461);
    // D616 (Int<32>, cap 616).
    tier!(c, 32, "d616", ln_series_d616, ln_tang_d616_p, 0, 154, 308, 462, 615);
    // D924 (Int<48>, cap 924).
    tier!(c, 48, "d924", ln_series_d924, ln_tang_d924_p, 0, 231, 462, 693, 923);
    // D1232 (Int<64>, cap 1232).
    tier!(c, 64, "d1232", ln_series_d1232, ln_tang_d1232_p, 0, 308, 616, 924, 1231);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
