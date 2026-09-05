// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Targeted bisection bench for the D57 sin/cos Tang band's LOWER EDGE.
//! Mirrors `atan_d57_band_bisect.rs` in SHAPE only — the kernel under test
//! is different, and that difference is the whole reason this bench exists.
//!
//! # Why this cannot reuse the atan measurement
//!
//! `policy::trig::forward::select` wires `(3, 44..=56)` to `Algorithm::Tang`
//! for sin/cos, and at D57 that arm realises as
//! `algos::trig::sincos_tang::{sin,cos}_tang_with_taylor::<Core, SCALE, 512>`
//! (`policy/trig.rs`, the `policy_sin` / `policy_cos` Tang arms). atan's
//! same-named `Tang` arm realises as a DIFFERENT kernel,
//! `algos::trig::atan_tang_3limb::atan` (`forward::select_atan`), which is
//! what `atan_d57_band_bisect` measured. Two different kernels cannot share
//! an edge, so atan's bisected `(3, 0..=56)` result carries no information
//! about this arm. The obvious shortcut — reuse the atan bench and change a
//! constant — would measure the wrong kernel and return a confident wrong
//! answer.
//!
//! The `44` here was never bisected: `forward::select`'s own comment records
//! it as left where it was measured for atan, i.e. asserted rather than
//! established for sin/cos.
//!
//! # What this bench compares — production, not a straw man
//!
//! At D57 `policy_sin` / `policy_cos` can reach three kernels, and WHICH one
//! production runs depends on SCALE:
//!
//! - outside `18..=22` — `{sin,cos}_rung_d57`, the Series arm at GUARD=30
//!   (working scale `SCALE + 30`);
//! - inside `18..=22` — `{sin,cos}_rung_narrow_d57`, the Series arm at
//!   GUARD=8 (working scale `SCALE + 8`);
//! - inside `44..=56` — `{sin,cos}_tang_d57`, the Tang table kernel.
//!
//! The narrow band matters to the bisection: at `18..=22` production works at
//! `SCALE + 8` while Tang is hardwired to `SCALE + 30`, so Tang carries ~22
//! more working digits there than it does anywhere else in the sweep. The win
//! region therefore need NOT be contiguous, and those scales are ranked
//! THREE-way (`narrow_g8` in the ranking) so the question is measured rather
//! than interpolated across the band.
//!
//! `{sin,cos}_rung_d57` (the canonical Ziv-escalating path) is the validity
//! oracle everywhere; every candidate is ranked against it.
//!
//! # The probe range
//!
//! Below 44 densely, and up to 56. There is nothing above 56 to probe: D57's
//! maximum SCALE **is** 56 (`types/widths.rs`, the `decl_decimal_full!(wide
//! D57, …)` max-scale argument), so the band's upper edge is the tier ceiling
//! rather than a routing choice. Only the lower edge is a real decision.
//!
//! # Validity wall
//!
//! A candidate MUST be bit-identical to the oracle across the operand spread
//! x all 8 rounding modes, else it is INELIGIBLE at that cell and is dropped
//! from the ranking (a `VALIDITY` line says so). A faster-but-not-bit-
//! identical kernel is never a candidate.
//!
//! Run with:
//! `powershell.exe -NoProfile -File scripts/pin_run.ps1 -Core 22 -Bench sincos_d57_band_bisect`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    cos_rung_d57, cos_rung_narrow_d57, cos_tang_d57, int_from_mag_limbs, sin_rung_d57,
    sin_rung_narrow_d57, sin_tang_d57,
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

/// sin/cos timing inputs spanning the kernel's dominant value-dependent
/// axis — the quadrant count `k = round(x·2/π)`, which sets how much of the
/// stage-1 reduction runs. `x0.5` folds no quadrant (`k = 0`), `x1.5` sits
/// just under `π/2` (`k = 1`), `x10.0` reduces through six (`k = 6`). All
/// three are representable across D57's whole legal scale range (at SCALE 56
/// the tier still holds |x| < ~31).
fn sincos_inputs(scale: u32) -> Vec<One> {
    vec![
        One { label: "x0.5", raw: build_raw::<3>(1, 2, scale) },
        One { label: "x1.5", raw: build_raw::<3>(3, 2, scale) },
        One { label: "x10.0", raw: build_raw::<3>(10, 1, scale) },
    ]
}

type TrigFn = fn(Int<3>, RoundingMode) -> Int<3>;

/// A WIDE validity spread — used by the exhaustive per-scale wall, not by the
/// timing cells. Spans the kernel's branch structure: the `sin(0)=0` /
/// `cos(0)=1` short-circuit, sub-ULP and tiny magnitudes, values across the
/// `|r| ≤ π/4` residue bound, values straddling each quadrant boundary
/// (`π/2`, `π`, `3π/2`, `2π` — where `k` steps and the quadrant permutation
/// changes), values spanning the 512-entry table's index range, the negative
/// mirror, and a magnitude near the tier's representable ceiling.
fn validity_inputs(scale: u32) -> Vec<One> {
    let mut v = vec![
        One { label: "0", raw: Int::<3>::ZERO },
        One { label: "1ulp", raw: build_raw::<3>(1, 1, 0) },
        One { label: "x0.001", raw: build_raw::<3>(1, 1000, scale) },
        // Across the table's index range (c_j = j·π/(4M), M = 512).
        One { label: "x1_512", raw: build_raw::<3>(1, 512, scale) },
        One { label: "x255_512", raw: build_raw::<3>(255, 512, scale) },
        One { label: "x511_512", raw: build_raw::<3>(511, 512, scale) },
        One { label: "x0.5", raw: build_raw::<3>(1, 2, scale) },
        // ~π/4 — the |r| residue bound.
        One { label: "xpi_4", raw: build_raw::<3>(7853981634, 10000000000, scale) },
        One { label: "x1.0", raw: build_raw::<3>(1, 1, scale) },
        // Straddle the k = 0 -> 1 quadrant step at π/2.
        One { label: "x1.5", raw: build_raw::<3>(3, 2, scale) },
        One { label: "xpi_2", raw: build_raw::<3>(15707963268, 10000000000, scale) },
        One { label: "x1.6", raw: build_raw::<3>(16, 10, scale) },
        One { label: "x2.0", raw: build_raw::<3>(2, 1, scale) },
        // The remaining quadrant steps: π, 3π/2, 2π.
        One { label: "xpi", raw: build_raw::<3>(31415926536, 10000000000, scale) },
        One { label: "x3pi_2", raw: build_raw::<3>(47123889804, 10000000000, scale) },
        One { label: "x2pi", raw: build_raw::<3>(62831853072, 10000000000, scale) },
        One { label: "x10.0", raw: build_raw::<3>(10, 1, scale) },
        One { label: "x20.0", raw: build_raw::<3>(20, 1, scale) },
        // Non-terminating fractions — no exact short representation.
        One { label: "x1_3", raw: build_raw::<3>(1, 3, scale) },
        One { label: "x2_7", raw: build_raw::<3>(2, 7, scale) },
    ];
    // The negative mirror of every non-zero entry — sin is odd, cos is even,
    // and the sign fold is a separate branch that must round identically.
    let mirrored: Vec<One> = v
        .iter()
        .filter(|o| o.raw != Int::<3>::ZERO)
        .map(|o| One { label: "neg", raw: -o.raw })
        .collect();
    v.extend(mirrored);
    v
}

/// Exhaustive validity wall at ONE scale for ONE function: Tang vs the
/// canonical oracle over the wide spread x all 8 rounding modes. Returns the
/// counts so the caller can print COUNTS, never an exit code — absence of a
/// failure line is not evidence that the check ran.
fn validity_at(fname: &str, scale: u32, oracle: TrigFn, tang: TrigFn) -> (u32, u32) {
    let inputs = validity_inputs(scale);
    let mut checked = 0u32;
    let mut bad = 0u32;
    for o in inputs.iter() {
        for m in ALL_MODES {
            checked += 1;
            if tang(o.raw, m) != oracle(o.raw, m) {
                bad += 1;
                println!("VALIDITY-FAIL {fname} s{scale}: input {} mode {m:?}", o.label);
            }
        }
    }
    (checked, bad)
}

/// One cell: validity-wall every candidate against `oracle`, then rank the
/// survivors (oracle included) over the input spread.
fn cell(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    oracle: TrigFn,
    candidates: &[(&'static str, TrigFn)],
) {
    let inputs = sincos_inputs(scale);
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
    ($c:expr, $prefix:literal, $oracle:ident, $tang:ident, $scale:literal) => {
        cell(
            $c,
            concat!($prefix, stringify!($scale)),
            $scale,
            $oracle::<$scale>,
            &[("tang", $tang::<$scale>)],
        );
    };
}

/// A cell in the `18..=22` narrow band: `narrow_g8` joins the ranking because
/// it, not the oracle, is what production runs at 18..=22.
macro_rules! cell_at_band {
    ($c:expr, $prefix:literal, $oracle:ident, $tang:ident, $narrow:ident, $scale:literal) => {
        cell(
            $c,
            concat!($prefix, stringify!($scale)),
            $scale,
            $oracle::<$scale>,
            &[("tang", $tang::<$scale>), ("narrow_g8", $narrow::<$scale>)],
        );
    };
}

/// The bisection surface for ONE function, in scale order.
///
/// The low tail (`0..=15`) is where a crossover could hide if Tang's win does
/// not reach all the way down; the `18..=22` cells are ranked THREE-way
/// because production runs a narrow-GUARD Series there; and `42/43` against
/// `44/45` straddle the OLD edge, which is what licenses moving it (or
/// leaves it where it is). `46..=56` re-confirm the band's interior, so a
/// re-run reproduces the whole evidence base rather than one pass's slice.
macro_rules! sweep_family {
    ($c:expr, $prefix:literal, $oracle:ident, $tang:ident, $narrow:ident) => {{
        cell_at!($c, $prefix, $oracle, $tang, 0);
        cell_at!($c, $prefix, $oracle, $tang, 1);
        cell_at!($c, $prefix, $oracle, $tang, 2);
        cell_at!($c, $prefix, $oracle, $tang, 3);
        cell_at!($c, $prefix, $oracle, $tang, 5);
        cell_at!($c, $prefix, $oracle, $tang, 8);
        cell_at!($c, $prefix, $oracle, $tang, 10);
        cell_at!($c, $prefix, $oracle, $tang, 15);
        cell_at_band!($c, $prefix, $oracle, $tang, $narrow, 18);
        cell_at_band!($c, $prefix, $oracle, $tang, $narrow, 20);
        cell_at_band!($c, $prefix, $oracle, $tang, $narrow, 22);
        cell_at!($c, $prefix, $oracle, $tang, 25);
        cell_at!($c, $prefix, $oracle, $tang, 30);
        cell_at!($c, $prefix, $oracle, $tang, 35);
        cell_at!($c, $prefix, $oracle, $tang, 40);
        cell_at!($c, $prefix, $oracle, $tang, 42);
        cell_at!($c, $prefix, $oracle, $tang, 43);
        cell_at!($c, $prefix, $oracle, $tang, 44);
        cell_at!($c, $prefix, $oracle, $tang, 45);
        cell_at!($c, $prefix, $oracle, $tang, 46);
        cell_at!($c, $prefix, $oracle, $tang, 48);
        cell_at!($c, $prefix, $oracle, $tang, 50);
        cell_at!($c, $prefix, $oracle, $tang, 52);
        cell_at!($c, $prefix, $oracle, $tang, 54);
        cell_at!($c, $prefix, $oracle, $tang, 56);
    }};
}

/// Run the validity wall across an explicit scale list for one function,
/// accumulating counts.
macro_rules! validity_sweep {
    ($fname:literal, $oracle:ident, $tang:ident, [$($scale:literal),+ $(,)?]) => {{
        let mut checked = 0u32;
        let mut bad = 0u32;
        let mut scales = 0u32;
        $(
            let (c, b) = validity_at($fname, $scale, $oracle::<$scale>, $tang::<$scale>);
            checked += c;
            bad += b;
            scales += 1;
        )+
        (scales, checked, bad)
    }};
}

/// [`validity_sweep`] over every scale D57 supports: 0..=56 (56 is the
/// tier's max SCALE — see `types/widths.rs`). The list is written out here
/// rather than passed in from a `scales!()` helper because a macro argument
/// is NOT eagerly expanded: the inner macro must receive real literal tokens
/// to use each one as a const-generic argument.
macro_rules! validity_all {
    ($fname:literal, $oracle:ident, $tang:ident) => {
        validity_sweep!(
            $fname,
            $oracle,
            $tang,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
            ]
        )
    };
}

/// The wall the routing decision rests on: Tang must be bit-identical to the
/// canonical Ziv-escalating oracle at EVERY scale the arm will route, for
/// BOTH functions — not just the ones the timing sweep sampled. Prints counts
/// so the run proves the mechanism FIRED rather than merely failing to
/// complain.
fn validity_all_scales() {
    let (s_scales, s_checked, s_bad) = validity_all!("sin", sin_rung_d57, sin_tang_d57);
    let (c_scales, c_checked, c_bad) = validity_all!("cos", cos_rung_d57, cos_tang_d57);
    // The per-scale input count VARIES: `build_raw` truncates a small
    // fraction to zero at low scales (`x0.001` is 0 at SCALE 0), and the
    // negative mirror skips zeros — so report the accumulated comparison
    // count rather than a fixed inputs-per-scale figure.
    println!(
        "VALIDITY-SWEEP sin: {s_scales} scales (0..=56) x 8 modes x (scale-dependent inputs) = {s_checked} comparisons, {s_bad} mismatches"
    );
    println!(
        "VALIDITY-SWEEP cos: {c_scales} scales (0..=56) x 8 modes x (scale-dependent inputs) = {c_checked} comparisons, {c_bad} mismatches"
    );
    println!(
        "VALIDITY-SWEEP total: {} comparisons, {} mismatches",
        s_checked + c_checked,
        s_bad + c_bad
    );
    assert_eq!(s_bad, 0, "sin Tang is NOT bit-identical to the oracle — INELIGIBLE to route");
    assert_eq!(c_bad, 0, "cos Tang is NOT bit-identical to the oracle — INELIGIBLE to route");
}

fn benches(c: &mut Criterion) {
    sweep_family!(c, "sin_d57_s", sin_rung_d57, sin_tang_d57, sin_rung_narrow_d57);
    sweep_family!(c, "cos_d57_s", cos_rung_d57, cos_tang_d57, cos_rung_narrow_d57);
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
