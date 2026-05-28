// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Targeted bisection bench for the D462 forward-trig (sin/cos/tan/atan)
//! narrow-vs-series crossover. Mirrors `exp_wide_tang_bisect.rs`.
//!
//! `policy::trig::forward::select` wires `(24, 225..=235)` as an 11-cell band
//! that routes sin/cos/atan to the "Tang" arm (actually
//! `sincos_narrow::*_with_taylor` — a narrowed-GUARD Taylor reclaim per the
//! kernel module doc, not real Tang lookup), with GUARD=10 for sin/cos/tan
//! and GUARD=12 for atan; outside the band the policy uses the canonical
//! `wide_trig_core::*_series` kernels (GUARD=30).
//!
//! Audit Finding #2 (2026-05-28): the 11-cell range is suspected to be a
//! coarse-bench fit. This bisection probes:
//!
//! - **inside the claimed band**: s225, s230, s235 — confirm the win still
//!   holds at each cell;
//! - **outward bisection**: s210, s218 (below s225) and s240, s260 (above
//!   s235) — find the true continuous crossover;
//! - **neighbour-width smooth-line check**: D307 s150 + D616 s300 — sanity
//!   that the per-tier picture is sensible.
//!
//! Validity wall: every narrow kernel MUST produce bit-identical results to
//! the Series reference across the operand spread × all 6 rounding modes,
//! else it is INELIGIBLE at that cell.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench trig_wide_tang_bisect`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    atan_narrow_d462, atan_series_d462, cos_narrow_d462, cos_series_d462, int_from_mag_limbs,
    sin_narrow_d462, sin_series_d462, tan_narrow_d462, tan_series_d462,
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

/// Sin/cos/tan inputs — three representative magnitudes inside [0, ~10π]
/// so range reduction is exercised. `x0.5` (small), `x1.5` (~π/2 region),
/// `x3.0` (multi-quadrant).
fn forward_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![
        One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<N>(3, 2, scale) },
        One { label: "x3.0", raw: build_raw::<N>(3, 1, scale) },
    ]
}

/// Atan inputs — three representative magnitudes spanning [0, large]. Atan
/// reduces by quadrant, the magnitudes mainly affect series-term count.
fn atan_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![
        One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<N>(3, 2, scale) },
        One { label: "x10.0", raw: build_raw::<N>(10, 1, scale) },
    ]
}

type TrigFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    inputs: Vec<One<N>>,
    series: TrigFn<N>,
    candidates: &[(&'static str, TrigFn<N>)],
) {
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("series", Box::new(move |o: One<N>| series(o.raw, MODE)))];
    for &(label, narrow) in candidates {
        let mut valid = true;
        'outer: for o in inputs.iter() {
            for m in ALL_MODES {
                if narrow(o.raw, m) != series(o.raw, m) {
                    println!(
                        "VALIDITY [{group}]: {label} != series ({}, mode {m:?}) -> INVALID, skipping",
                        o.label
                    );
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            runs.push((label, Box::new(move |o: One<N>| narrow(o.raw, MODE))));
        }
    }
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all narrow candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), inputs, runs);
}

/// One forward-trig cell at D462 SCALE = $scale: A/B against `narrow_g10`
/// (the production narrow GUARD). Macro takes both kernel idents because the
/// suite has no `paste` dep.
macro_rules! cell_d462_fwd {
    ($c:expr, $name:literal, $series:ident, $narrow:ident, $scale:literal) => {
        cell::<24>(
            $c,
            concat!($name, "_d462_s", stringify!($scale)),
            forward_inputs::<24>($scale),
            $series::<$scale>,
            &[("narrow_g10", $narrow::<$scale, 10>)],
        );
    };
}

/// Atan cell at D462 SCALE = $scale: A/B against `narrow_g12` (production).
macro_rules! cell_d462_atan {
    ($c:expr, $scale:literal) => {
        cell::<24>(
            $c,
            concat!("atan_d462_s", stringify!($scale)),
            atan_inputs::<24>($scale),
            atan_series_d462::<$scale>,
            &[("narrow_g12", atan_narrow_d462::<$scale, 12>)],
        );
    };
}

/// All four forward fns at one scale.
macro_rules! all_forward_at {
    ($c:expr, $scale:literal) => {
        cell_d462_fwd!($c, "sin", sin_series_d462, sin_narrow_d462, $scale);
        cell_d462_fwd!($c, "cos", cos_series_d462, cos_narrow_d462, $scale);
        cell_d462_fwd!($c, "tan", tan_series_d462, tan_narrow_d462, $scale);
        cell_d462_atan!($c, $scale);
    };
}

fn benches(c: &mut Criterion) {
    // ── First-pass: 225..=235 + outward to s210/s218 (below) and s240/s260/s290
    // (above) showed narrow_g10/g12 wins EVERYWHERE. The 11-cell production
    // gate is way too narrow. Second-pass widens the search: probe the lower
    // tail (s0/s50/s100/s150/s180/s200) to find the true LOWER edge of the
    // win-region, and upper tail (s330/s400/s450 near MAX scale 461) for the
    // upper edge.
    all_forward_at!(c, 0);
    all_forward_at!(c, 50);
    all_forward_at!(c, 100);
    all_forward_at!(c, 150);
    all_forward_at!(c, 180);
    all_forward_at!(c, 200);
    all_forward_at!(c, 330);
    all_forward_at!(c, 400);
    all_forward_at!(c, 450);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
