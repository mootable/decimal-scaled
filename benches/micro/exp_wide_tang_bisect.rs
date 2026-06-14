// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Targeted bisection bench for the D307 and D462 Tang/Series crossover.
//!
//! The 5-point sweep (`exp_wide_series_tang_ab`) leaves a coarse picture; this
//! bench BISECTS the win-region boundaries:
//!
//! - **D307**: 5-point sweep shows Tang wins s0/s76, Series wins s153/s230/s305.
//!   The crossover sits between s76 (Tang +3%) and s153 (Series +4%). Bisect at
//!   s100 (existing gate) and s115 (midpoint candidate) and s130 to find the
//!   true crossover.
//! - **D462**: 5-point sweep shows Series wins at s0/s231/s346/s460; Tang ~ties
//!   at s115 (+1.01% Tang). Bisect at s58 (midpoint s0..s115) to confirm Tang
//!   never WINS there, only ties.
//!
//! Validity wall (bit-identical to Series across the operand spread × all six
//! modes) applies as in the main bench.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench exp_wide_tang_bisect`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    exp_series_d307, exp_series_d462, exp_tang_d307, exp_tang_d462, int_from_mag_limbs,
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

/// `acc = acc * m + add`, in place across the little-endian limb array.
fn mul_add_small<const N: usize>(acc: &mut [u64; N], m: u64, add: u64) {
    let mut carry = add as u128;
    for limb in acc.iter_mut() {
        let prod = (*limb as u128) * (m as u128) + carry;
        *limb = prod as u64;
        carry = prod >> 64;
    }
}

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

fn exp_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![
        One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<N>(3, 2, scale) },
        One { label: "x3.0", raw: build_raw::<N>(3, 1, scale) },
    ]
}

type ExpFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

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
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all Tang candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), exp_inputs::<N>(scale), runs);
}

macro_rules! cell2 {
    ($c:expr, $n:literal, $name:literal, $scale:literal, $series:ident, $tang:ident) => {
        cell::<$n>(
            $c,
            concat!("exp_", $name, "_s", stringify!($scale)),
            $scale,
            $series::<$scale>,
            &[
                ("tang_m128_g30", $tang::<$scale, 128, 30>),
                ("tang_m512_g30", $tang::<$scale, 512, 30>),
            ],
        );
    };
}

fn benches(c: &mut Criterion) {
    // D307 — coarse 5-point map: Tang wins s0/s76 (~tie/+1.03×). First bisect
    // (s90/s100/s115/s130) showed Series wins s90+ (1.01–1.07×). So the real
    // crossover is BETWEEN s76 (Tang) and s90 (Series). Second-pass bisect:
    // probe s78/s80/s84 to nail the gate boundary.
    cell2!(c, 16, "d307", 78, exp_series_d307, exp_tang_d307);
    cell2!(c, 16, "d307", 80, exp_series_d307, exp_tang_d307);
    cell2!(c, 16, "d307", 84, exp_series_d307, exp_tang_d307);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
