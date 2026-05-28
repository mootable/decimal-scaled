// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier hyperbolic (sinh/cosh/tanh) Series-baseline-vs-Tang-composed A/B
//! map at D462/D616/D924/D1232.
//!
//! Audit lead (2026-05-28 Finding #4 "Pattern A"): the wide-tier hyperbolic
//! family at D462/D616/D924/D1232 currently routes every cell through the
//! `wide_trig_hyper_inherent!` macro to `hyper_schoolbook::*_schoolbook`,
//! which composes `(e^|x| ± e^-|x|)` over `exp_fixed` (Series at these tiers
//! per `policy::exp::select` returning `Algorithm::Series` for N ≥ 24).
//! The audit hypothesis: routing the inner exp through `tang_exp_fixed`
//! (Tang) at these widths, mirroring D307's `hyper_exp_identity_with_tang`
//! arm, will recover the ~25–106% regression on the bbc cluster.
//!
//! This bench TESTS that hypothesis directly. For each cell it:
//!
//! 1. Validity-walls each Tang candidate (M=512/GUARD=30, M=512/GUARD=60,
//!    M=128/GUARD=30) against the schoolbook baseline across the operand
//!    spread × all 6 rounding modes — a single bit mismatch DROPS the
//!    candidate as INVALID.
//! 2. Ranks the surviving candidates against the schoolbook baseline.
//!
//! `tang_exp_fixed` runs with `INTERNAL_EXTRA = true` so the kernel covers
//! arbitrary `|k|` at the working scale (matching the existing D57/D115/
//! D153/D307 production `hyper_exp_identity_with_tang` shape).
//!
//! A cell is a Tang win ONLY if at least one validity-valid Tang candidate
//! beats the schoolbook baseline AT THIS CELL. The full prior-art context:
//! `policy/exp.rs` records that bare exp Tang LOSES across the entire
//! D462–D1232 scale range (`exp_d462_*` 1.04–1.20× regression, `exp_d616`+
//! 1.29–2.06× regression in the existing `exp_wide_series_tang_ab` map).
//! Since the hyperbolic identity is `~2 × exp + 1 cheap divide`, the
//! direct prediction is that Tang composition LOSES at the same factor.
//! This bench is the empirical confirmation/falsification.
//!
//! Scale spread per tier: `{0, S/4, S/2, 3S/4, S-2}` (the owner-standard
//! 5-point coarse sampling; the top is `cap - 2` so the `x3.0` input keeps
//! its two integer digits inside storage). Operands `{0.5, 1.5, 3.0}` —
//! identical to `exp_wide_series_tang_ab` so the hyperbolic verdict is
//! directly comparable to the bare-exp verdict.
//!
//! Run:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench hyper_wide_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    cosh_schoolbook_d1232, cosh_schoolbook_d462, cosh_schoolbook_d616, cosh_schoolbook_d924,
    cosh_tang_compose_d1232, cosh_tang_compose_d462, cosh_tang_compose_d616, cosh_tang_compose_d924,
    int_from_mag_limbs, sinh_schoolbook_d1232, sinh_schoolbook_d462, sinh_schoolbook_d616,
    sinh_schoolbook_d924, sinh_tang_compose_d1232, sinh_tang_compose_d462, sinh_tang_compose_d616,
    sinh_tang_compose_d924, tanh_schoolbook_d1232, tanh_schoolbook_d462, tanh_schoolbook_d616,
    tanh_schoolbook_d924, tanh_tang_compose_d1232, tanh_tang_compose_d462, tanh_tang_compose_d616,
    tanh_tang_compose_d924,
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

/// Hyperbolic inputs: `{0.5, 1.5, 3.0}` (same as exp A/B). 0.5 sits well
/// outside the tiny-band cubic early-return (`|raw| ≤ 10^(2·SCALE/3)`) at
/// every benched scale, so each input runs the full exp-identity path.
fn hyper_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![
        One { label: "x0.5", raw: build_raw::<N>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<N>(3, 2, scale) },
        One { label: "x3.0", raw: build_raw::<N>(3, 1, scale) },
    ]
}

type HyperFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    baseline: HyperFn<N>,
    candidates: &[(&'static str, HyperFn<N>)],
) {
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("schoolbook", Box::new(move |o: One<N>| baseline(o.raw, MODE)))];
    for &(label, tang) in candidates {
        let mut valid = true;
        'outer: for o in hyper_inputs::<N>(scale) {
            for m in ALL_MODES {
                if tang(o.raw, m) != baseline(o.raw, m) {
                    println!(
                        "VALIDITY [{group}]: {label} != schoolbook ({}, mode {m:?}) -> INVALID, skipping",
                        o.label
                    );
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
        println!("A/B verdict [{group}]: all Tang candidates INVALID -> stays schoolbook (Series)");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), hyper_inputs::<N>(scale), runs);
}

macro_rules! cell4 {
    ($c:expr, $n:literal, $name:literal, $fn:literal, $scale:literal,
     $school:ident, $tang:ident) => {
        cell::<$n>(
            $c,
            concat!($fn, "_", $name, "_s", stringify!($scale)),
            $scale,
            $school::<$scale>,
            &[
                ("tang_m512_g30", $tang::<$scale, 512, 30>),
                ("tang_m512_g60", $tang::<$scale, 512, 60>),
                ("tang_m128_g30", $tang::<$scale, 128, 30>),
            ],
        );
    };
}

macro_rules! tier_one_fn {
    ($c:expr, $n:literal, $name:literal, $fn:literal,
     $school:ident, $tang:ident,
     $s0:literal, $s1:literal, $s2:literal, $s3:literal, $s4:literal) => {{
        cell4!($c, $n, $name, $fn, $s0, $school, $tang);
        cell4!($c, $n, $name, $fn, $s1, $school, $tang);
        cell4!($c, $n, $name, $fn, $s2, $school, $tang);
        cell4!($c, $n, $name, $fn, $s3, $school, $tang);
        cell4!($c, $n, $name, $fn, $s4, $school, $tang);
    }};
}

fn benches(c: &mut Criterion) {
    // D462 (Int<24>, cap 462): scales {0, 115, 231, 346, 460}.
    tier_one_fn!(c, 24, "d462", "sinh", sinh_schoolbook_d462, sinh_tang_compose_d462,
        0, 115, 231, 346, 460);
    tier_one_fn!(c, 24, "d462", "cosh", cosh_schoolbook_d462, cosh_tang_compose_d462,
        0, 115, 231, 346, 460);
    tier_one_fn!(c, 24, "d462", "tanh", tanh_schoolbook_d462, tanh_tang_compose_d462,
        0, 115, 231, 346, 460);
    // D616 (Int<32>, cap 616): scales {0, 154, 308, 462, 614}.
    tier_one_fn!(c, 32, "d616", "sinh", sinh_schoolbook_d616, sinh_tang_compose_d616,
        0, 154, 308, 462, 614);
    tier_one_fn!(c, 32, "d616", "cosh", cosh_schoolbook_d616, cosh_tang_compose_d616,
        0, 154, 308, 462, 614);
    tier_one_fn!(c, 32, "d616", "tanh", tanh_schoolbook_d616, tanh_tang_compose_d616,
        0, 154, 308, 462, 614);
    // D924 (Int<48>, cap 924): scales {0, 231, 462, 693, 922}.
    tier_one_fn!(c, 48, "d924", "sinh", sinh_schoolbook_d924, sinh_tang_compose_d924,
        0, 231, 462, 693, 922);
    tier_one_fn!(c, 48, "d924", "cosh", cosh_schoolbook_d924, cosh_tang_compose_d924,
        0, 231, 462, 693, 922);
    tier_one_fn!(c, 48, "d924", "tanh", tanh_schoolbook_d924, tanh_tang_compose_d924,
        0, 231, 462, 693, 922);
    // D1232 (Int<64>, cap 1232): scales {0, 308, 616, 924, 1230}.
    tier_one_fn!(c, 64, "d1232", "sinh", sinh_schoolbook_d1232, sinh_tang_compose_d1232,
        0, 308, 616, 924, 1230);
    tier_one_fn!(c, 64, "d1232", "cosh", cosh_schoolbook_d1232, cosh_tang_compose_d1232,
        0, 308, 616, 924, 1230);
    tier_one_fn!(c, 64, "d1232", "tanh", tanh_schoolbook_d1232, tanh_tang_compose_d1232,
        0, 308, 616, 924, 1230);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
