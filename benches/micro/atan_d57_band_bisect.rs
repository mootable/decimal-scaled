// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Targeted bisection bench for the D57 `atan` Tang band's LOWER EDGE.
//! Mirrors `trig_wide_tang_bisect.rs` (the bench that bisected the D462
//! arm — the one trig band whose edge was actually measured).
//!
//! `policy::trig::forward::select` wires `(3, 44..=56)`, so D57 `atan`
//! reaches `atan_tang_3limb_s44_56` only from SCALE 44 up. The kernel's own
//! header records that this lower edge was **asserted, not measured**, and
//! dismantles the original justification: every scale in `30..=56` runs the
//! SAME 6 halvings on the generic path, the work integer is `Int<16>` at
//! every scale, and the Tang kernel gets CHEAPER as SCALE falls (a 3-limb
//! table prefix at SCALE 0 against 6 at SCALE 56) while paying no halvings
//! at any scale.
//!
//! # What this bench compares — production, not a straw man
//!
//! At D57 `policy_atan` can reach three kernels, and WHICH one production
//! runs depends on SCALE:
//!
//! - outside `18..=22` — `atan_rung_d57`, the Series arm at GUARD=30
//!   (working scale `SCALE + 30`), the halving-chain path;
//! - inside `18..=22` — `atan_rung_narrow_d57`, the Series arm at GUARD=10
//!   (working scale `SCALE + 10`);
//! - inside `44..=56` — `atan_tang_d57`, the Tang table kernel.
//!
//! The narrow band matters to the bisection: at `18..=22` production works
//! at `SCALE + 10` while Tang is hardwired to `SCALE + 30`, so Tang carries
//! ~20 more working digits there than it does anywhere else in the sweep.
//! The win region therefore need NOT be contiguous, and the band scales are
//! probed with `narrow_g10` in the ranking so the question is measured
//! rather than interpolated.
//!
//! `atan_rung_d57` (the canonical Ziv-escalating path) is the validity
//! oracle everywhere; every candidate is ranked against it.
//!
//! # Validity wall
//!
//! A candidate MUST be bit-identical to the oracle across the operand
//! spread x all 8 rounding modes, else it is INELIGIBLE at that cell and is
//! dropped from the ranking (a `VALIDITY` line says so). A faster-but-not-
//! bit-identical kernel is never a candidate.
//!
//! Run with:
//! `powershell.exe -NoProfile -File scripts/pin_run.ps1 -Core 22 -Bench atan_d57_band_bisect`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    atan_rung_d57, atan_rung_narrow_d57, atan_tang_d57, int_from_mag_limbs,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;
const ALL_MODES: [RoundingMode; 8] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::AwayFromZero,
    RoundingMode::ZeroFiveUp,
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
struct One {
    label: &'static str,
    raw: Int<3>,
}

/// Atan inputs spanning the kernel's two shapes: `x0.5` takes no reciprocal
/// fold, `x1.5` and `x10.0` do. All three are representable at D57's whole
/// legal scale range (at SCALE 56 the tier still holds |x| < ~31).
fn atan_inputs(scale: u32) -> Vec<One> {
    vec![
        One { label: "x0.5", raw: build_raw::<3>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<3>(3, 2, scale) },
        One { label: "x10.0", raw: build_raw::<3>(10, 1, scale) },
    ]
}

type AtanFn = fn(Int<3>, RoundingMode) -> Int<3>;

/// A WIDE validity spread — used by the exhaustive per-scale wall, not by
/// the timing cells. Spans the kernel's branch structure: the `atan(0) = 0`
/// short-circuit, sub-ULP and tiny magnitudes (where the near-tie escape
/// decides), values either side of the `|x| = 1` reciprocal fold, values
/// landing exactly on and between Tang table indices (`j/512`), the
/// negative mirror, and the tier's representable ceiling.
fn validity_inputs(scale: u32) -> Vec<One> {
    let mut v = vec![
        One { label: "0", raw: Int::<3>::ZERO },
        One { label: "1ulp", raw: build_raw::<3>(1, 1, 0) },
        One { label: "x0.001", raw: build_raw::<3>(1, 1000, scale) },
        One { label: "x0.5", raw: build_raw::<3>(1, 2, scale) },
        // Exactly on a table index: 256/512 = 0.5 is already above; 1/512
        // and 511/512 sit at the ends of the table's reach.
        One { label: "x1/512", raw: build_raw::<3>(1, 512, scale) },
        One { label: "x511/512", raw: build_raw::<3>(511, 512, scale) },
        // Straddle the reciprocal fold at |x| = 1.
        One { label: "x0.999", raw: build_raw::<3>(999, 1000, scale) },
        One { label: "x1.0", raw: build_raw::<3>(1, 1, scale) },
        One { label: "x1.001", raw: build_raw::<3>(1001, 1000, scale) },
        One { label: "x1.5", raw: build_raw::<3>(3, 2, scale) },
        One { label: "x2.0", raw: build_raw::<3>(2, 1, scale) },
        One { label: "x10.0", raw: build_raw::<3>(10, 1, scale) },
        One { label: "x1_3", raw: build_raw::<3>(1, 3, scale) },
        One { label: "x2_7", raw: build_raw::<3>(2, 7, scale) },
    ];
    // The negative mirror of every non-zero entry — the sign fold is a
    // separate branch and must round identically.
    let mirrored: Vec<One> = v
        .iter()
        .filter(|o| o.raw != Int::<3>::ZERO)
        .map(|o| One { label: "neg", raw: -o.raw })
        .collect();
    v.extend(mirrored);
    v
}

/// Exhaustive validity wall at ONE scale: Tang vs the canonical oracle over
/// the wide spread x all 8 rounding modes. Returns the mismatch count so
/// the caller can print COUNTS, never an exit code — absence of a failure
/// line is not evidence that the check ran.
fn validity_at(scale: u32, oracle: AtanFn, tang: AtanFn) -> (u32, u32) {
    let inputs = validity_inputs(scale);
    let mut checked = 0u32;
    let mut bad = 0u32;
    for o in inputs.iter() {
        for m in ALL_MODES {
            checked += 1;
            if tang(o.raw, m) != oracle(o.raw, m) {
                bad += 1;
                println!("VALIDITY-FAIL s{scale}: input {} mode {m:?}", o.label);
            }
        }
    }
    (checked, bad)
}

/// One cell: validity-wall every candidate against `oracle`, then rank the
/// survivors (oracle included) over the input spread.
fn cell(c: &mut Criterion, group: &str, scale: u32, oracle: AtanFn, candidates: &[(&'static str, AtanFn)]) {
    let inputs = atan_inputs(scale);
    let mut runs: Vec<(&'static str, Box<dyn Fn(One) -> Int<3>>)> =
        vec![("series_g30", Box::new(move |o: One| oracle(o.raw, MODE)))];
    for &(label, cand) in candidates {
        let mut valid = true;
        'outer: for o in inputs.iter() {
            for m in ALL_MODES {
                if cand(o.raw, m) != oracle(o.raw, m) {
                    println!(
                        "VALIDITY [{group}]: {label} != series_g30 ({}, mode {m:?}) -> INVALID, skipping",
                        o.label
                    );
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            println!("VALIDITY [{group}]: {label} bit-identical to series_g30 over 3 inputs x 8 modes");
            runs.push((label, Box::new(move |o: One| cand(o.raw, MODE))));
        }
    }
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One| o.label.to_string(), inputs, runs);
}

/// A cell outside the `18..=22` narrow band: production runs the oracle, so
/// the ranking is Tang vs what production actually executes there.
macro_rules! cell_at {
    ($c:expr, $scale:literal) => {
        cell(
            $c,
            concat!("atan_d57_s", stringify!($scale)),
            $scale,
            atan_rung_d57::<$scale>,
            &[("tang", atan_tang_d57::<$scale>)],
        );
    };
}

/// A cell in/next to the `18..=22` narrow band: `narrow_g10` joins the
/// ranking because it, not the oracle, is what production runs at 18..=22.
macro_rules! cell_at_band {
    ($c:expr, $scale:literal) => {
        cell(
            $c,
            concat!("atan_d57_s", stringify!($scale)),
            $scale,
            atan_rung_d57::<$scale>,
            &[
                ("tang", atan_tang_d57::<$scale>),
                ("narrow_g10", atan_rung_narrow_d57::<$scale>),
            ],
        );
    };
}

/// Run [`validity_at`] across an explicit scale list, accumulating counts.
macro_rules! validity_sweep {
    ($($scale:literal),+ $(,)?) => {{
        let mut checked = 0u32;
        let mut bad = 0u32;
        let mut scales = 0u32;
        $(
            let (c, b) = validity_at($scale, atan_rung_d57::<$scale>, atan_tang_d57::<$scale>);
            checked += c;
            bad += b;
            scales += 1;
        )+
        (scales, checked, bad)
    }};
}

/// The wall the routing decision rests on: Tang must be bit-identical to
/// the canonical Ziv-escalating oracle at EVERY scale the arm will route,
/// not just the ones the timing sweep sampled. Prints counts so the run
/// proves the mechanism FIRED rather than merely failing to complain.
fn validity_all_scales() {
    let (scales, checked, bad) = validity_sweep!(
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56,
    );
    // The per-scale input count VARIES: `build_raw` truncates a small
    // fraction to zero at low scales (`x0.001` is 0 at SCALE 0), and the
    // negative mirror skips zeros — so report the accumulated comparison
    // count rather than a fixed inputs-per-scale figure.
    println!(
        "VALIDITY-SWEEP: {scales} scales (0..=56) x 8 modes x (14..=27 inputs, scale-dependent) = {checked} comparisons, {bad} mismatches"
    );
    assert_eq!(bad, 0, "Tang is NOT bit-identical to the oracle — INELIGIBLE to route");
}

fn benches(c: &mut Criterion) {
    // Pass 2 — a REPLICATE of every pass-1 cell (a single run supports
    // nothing) plus a denser low tail. Pass 1 put Tang ahead at all 16
    // probed scales, so the crossover is at or below SCALE 0 and the low
    // tail is where a crossover could still hide; s5's thinner 1.37x also
    // wanted a second look.
    // The full bisection surface, so a re-run reproduces the whole evidence
    // base rather than any single pass's slice.
    //
    // The low tail (0..=15) and the straddle scales either side of the OLD
    // edge (42/43 vs 44) are what licensed dropping the edge to 0. The
    // 18..=22 cells are ranked THREE-way (`cell_at_band`) because
    // production runs a narrow-GUARD Series there, not the GUARD=30 oracle
    // — Tang carries ~20 more working digits in that band, so its win had
    // to be measured rather than interpolated across it.
    //
    // s46..s54 bracket s50: one run reported series_g30 ahead there by
    // 1.07x while two others put Tang ahead by 3.66x/3.86x, and Tang's own
    // absolute time had spiked ~4.5x. The bracket showed no gradient
    // (3.35 / 2.75 / 4.02 / 3.92 / 3.80x), i.e. interference, not a
    // crossover — a near-tie escape would have been input-deterministic and
    // fired in every pass. Kept so the check is repeatable.
    cell_at!(c, 0);
    cell_at!(c, 1);
    cell_at!(c, 2);
    cell_at!(c, 3);
    cell_at!(c, 5);
    cell_at!(c, 8);
    cell_at!(c, 10);
    cell_at!(c, 15);
    cell_at_band!(c, 18);
    cell_at_band!(c, 20);
    cell_at_band!(c, 22);
    cell_at!(c, 25);
    cell_at!(c, 30);
    cell_at!(c, 35);
    cell_at!(c, 40);
    cell_at!(c, 42);
    cell_at!(c, 43);
    cell_at!(c, 44);
    cell_at!(c, 46);
    cell_at!(c, 48);
    cell_at!(c, 50);
    cell_at!(c, 52);
    cell_at!(c, 54);
    cell_at!(c, 56);
}

fn main() {
    // The validity wall runs FIRST: if Tang is not bit-identical the timing
    // numbers are irrelevant, because an ineligible kernel cannot be routed
    // however fast it is.
    validity_all_scales();
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
