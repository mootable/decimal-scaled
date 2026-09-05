// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `ln_narrow_series_tang_ab` — the narrow-tier (D18 / D38) Series-vs-Tang
//! race, the sibling of `ln_wide_series_tang_ab` (whose module doc derives
//! the operand contract; it is restated here only as far as this file
//! enforces it).
//!
//! **Why this bench exists.** The bench-branch-compare diagnostic row `ln_nd`
//! benches `ln(7.0)`, and `7.0 = 2^2 · 1.75` with `1.75 = 1 + 96/128` EXACTLY
//! — Tang table slot 96, residual `t = 0`, the artanh loop breaks on its first
//! iteration. Under the narrow Series that operand does real work (the only
//! Series degeneracy is the `m == 1` power-of-two trap), so `ln_nd` was a
//! valid BEFORE. Once `policy::ln` routes the narrow tiers to Tang
//! (`(1, _) | (2, _) => Tang`) it becomes a table read, and an AFTER measured
//! on it would be a lookup dressed as a kernel. This bench is the sound
//! instrument: both kernels in ONE process (same-machine within a cell), on
//! operands that defeat BOTH traps, validity-gated (Tang == Series under all
//! eight modes) before a single timing.
//!
//! **Operand contract** (`assert_non_degenerate`, per operand): `raw` ODD
//! (defeats the power-of-two `m == 1` short-circuit in both kernels) AND, for
//! `SCALE >= 1`, `raw % 5 != 0` (so `5^SCALE | raw` is unreachable and the Tang
//! residual cannot be exactly zero); at `SCALE = 0` the boundary test collapses
//! to `k <= 7`, so `raw` odd AND `raw >= 257`. Operands: `1/3` and `7/3` at
//! full width (one either side of 1, repeating decimals, both end in 3) and
//! `333` / `2333` at scale 0 — the sibling's spread.
//!
//! **What it measures.** Per narrow cell: Series (the kept `_`-arm path, i.e.
//! what shipped) against the routed Tang (`Int<12>` work width, `GUARD 8`,
//! `CAP 100`) and one guard alternative. D18 and D38 share the `Int<2>` kernel
//! entry (D18 widens), so the cells are keyed by SCALE over the union of the
//! two tiers' golden scale sets. D57 and D76 run in the SAME binary at their
//! scales shared with the narrow set, so the narrow-vs-wide SAME-SCALE ratio
//! — the inversion itself — is same-machine, not cross-VM.
//!
//! **The `_p2` control** (`x = 2.0`) is kept deliberately: it is the
//! short-circuit both kernels take, the shape `ln_nd(7.0)` now has under Tang,
//! and it shows on the ranking table what a degenerate operand looks like.
//!
//! `LN_AB_ONLY=<substring>` bounds wall time by skipping unselected groups
//! (`compare_all`'s coarse re-time is not filtered by criterion's own filter).

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;

use decimal_scaled::__bench_internals::{
    int_from_mag_limbs, ln_series_d57, ln_series_d76, ln_series_narrow, ln_tang_d57,
    ln_tang_d76_p, ln_tang_narrow_p,
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

/// `acc = acc · m + add`, little-endian limbs, wrapping past the top (never
/// reached for the magnitudes built here).
fn mul_add_small<const N: usize>(acc: &mut [u64; N], m: u64, add: u64) {
    let mut carry = add as u128;
    for limb in acc.iter_mut() {
        let prod = (*limb as u128) * (m as u128) + carry;
        *limb = prod as u64;
        carry = prod >> 64;
    }
}

/// `floor(x_num · 10^scale / x_den)` as `N` limbs — the raw storage integer of
/// `x_num / x_den` at `scale`, truncated (the sibling's construction).
fn build_mag<const N: usize>(x_num: u64, x_den: u64, scale: u32) -> [u64; N] {
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
    mag
}

/// `(raw mod 2, raw mod 5)` off the limbs: parity is limb 0's low bit; `2^64
/// ≡ 1 (mod 5)`, so `raw mod 5` is the limb sum mod 5.
fn parity_and_mod5<const N: usize>(mag: &[u64; N]) -> (u64, u64) {
    let mut sum_mod5 = 0u64;
    for &limb in mag.iter() {
        sum_mod5 = (sum_mod5 + limb % 5) % 5;
    }
    (mag[0] & 1, sum_mod5)
}

/// The operand contract, asserted — a degenerate operand fails the bench
/// rather than silently timing a short-circuit.
fn assert_non_degenerate<const N: usize>(mag: &[u64; N], label: &str, scale: u32) {
    let nonzero = mag.iter().any(|&l| l != 0);
    assert!(nonzero, "operand {label} at scale {scale} is zero -- outside ln's domain");
    let (parity, mod5) = parity_and_mod5::<N>(mag);
    assert_eq!(
        parity, 1,
        "operand {label} at scale {scale}: raw must be ODD or it can take the \
         power-of-two short-circuit (ln_tang.rs `mantissa_w == one`, \
         exp_generic::ln_fixed `mantissa_w == one`)"
    );
    if scale == 0 {
        let above_256 = mag[1..].iter().any(|&l| l != 0) || mag[0] > 256;
        assert!(
            above_256,
            "operand {label} at scale 0: raw must be >= 257 or its mantissa \
             terminates within 7 fraction bits and lands exactly on a Tang \
             table boundary (t == 0)"
        );
    } else {
        assert_ne!(
            mod5, 0,
            "operand {label} at scale {scale}: raw must NOT be divisible by 5 or \
             `5^scale | raw` becomes reachable and the Tang residual t can be \
             exactly zero"
        );
    }
}

fn build_raw<const N: usize>(x_num: u64, x_den: u64, scale: u32, label: &str) -> Int<N> {
    let mag = build_mag::<N>(x_num, x_den, scale);
    assert_non_degenerate::<N>(&mag, label, scale);
    int_from_mag_limbs::<N>(&mag)
}

/// The `_p2` control: `2.0 · 10^scale`, an exact power of two times `10^scale`
/// — the short-circuit both kernels take.
fn build_raw_pow2<const N: usize>(scale: u32) -> Int<N> {
    let mag = build_mag::<N>(2, 1, scale);
    let (parity, _) = parity_and_mod5::<N>(&mag);
    assert_eq!(parity, 0, "the _p2 control must be an exact power of two times 10^scale");
    int_from_mag_limbs::<N>(&mag)
}

#[derive(Clone)]
struct One<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

fn ln_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    if scale == 0 {
        vec![
            One { label: "x333", raw: build_raw::<N>(333, 1, 0, "x333") },
            One { label: "x2333", raw: build_raw::<N>(2333, 1, 0, "x2333") },
        ]
    } else {
        vec![
            One { label: "x_lo", raw: build_raw::<N>(1, 3, scale, "x_lo") },
            One { label: "x_hi", raw: build_raw::<N>(7, 3, scale, "x_hi") },
        ]
    }
}

fn ln_pow2_input<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![One { label: "x2.0", raw: build_raw_pow2::<N>(scale) }]
}

/// The `_band` control: `x = 1 + 3·10^-scale`, inside the linear near-1
/// class (`|δ| = 3 ≤ 10^⌊(scale−1)/2⌋` for `scale ≥ 3`) — the input class on
/// which Series has always exited on its first term and the Tang arm now
/// does too (`ln_linear_band_exit`). Odd and `≡ 3 (mod 5)`, so it also meets
/// the kernel contract, but its point is the exit, not the kernel.
fn build_raw_band<const N: usize>(scale: u32) -> Int<N> {
    assert!(scale >= 3, "the _band control needs a band of at least 10 (scale >= 3)");
    let mut mag = build_mag::<N>(1, 1, scale);
    mul_add_small::<N>(&mut mag, 1, 3);
    assert_non_degenerate::<N>(&mag, "x_band", scale);
    int_from_mag_limbs::<N>(&mag)
}

fn ln_band_input<const N: usize>(scale: u32) -> Vec<One<N>> {
    vec![One { label: "x_band", raw: build_raw_band::<N>(scale) }]
}

type LnFn<const N: usize> = fn(Int<N>, RoundingMode) -> Int<N>;

fn selected(group: &str) -> bool {
    match std::env::var("LN_AB_ONLY") {
        Ok(filter) if !filter.is_empty() => group.contains(&filter),
        _ => true,
    }
}

/// One cell: Series as the reference run, every candidate validity-gated
/// (bit-identical to Series on every input under all eight modes) before it
/// is timed. An invalid candidate is reported and dropped, never timed.
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    inputs: Vec<One<N>>,
    series: LnFn<N>,
    candidates: &[(&'static str, LnFn<N>)],
) {
    if !selected(group) {
        return;
    }
    let mut runs: Vec<(&'static str, Box<dyn Fn(One<N>) -> Int<N>>)> =
        vec![("series", Box::new(move |o: One<N>| series(o.raw, MODE)))];
    for &(label, tang) in candidates {
        let mut valid = true;
        'outer: for o in inputs.iter() {
            for m in ALL_MODES {
                if tang(o.raw, m) != series(o.raw, m) {
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
            runs.push((label, Box::new(move |o: One<N>| tang(o.raw, MODE))));
        }
    }
    if runs.len() < 2 {
        println!("A/B verdict [{group}]: all Tang candidates INVALID -> stays Series");
        return;
    }
    compare_all(c, group, |o: &One<N>| o.label.to_string(), inputs, runs);
}

/// A narrow cell at `SCALE` (D18 and D38 share the `Int<2>` entry): Series vs
/// the routed Tang (`G8/CAP100`) and the `G10` alternative, plus the `_p2`
/// control.
macro_rules! narrow_cell {
    ($c:expr, $scale:literal) => {
        cell::<2>(
            $c,
            concat!("ln_narrow_s", stringify!($scale)),
            ln_inputs::<2>($scale),
            ln_series_narrow::<$scale>,
            &[
                ("tang_g8_c100", ln_tang_narrow_p::<$scale, 8, 100>),
                ("tang_g10_c100", ln_tang_narrow_p::<$scale, 10, 100>),
            ],
        );
        cell::<2>(
            $c,
            concat!("ln_narrow_s", stringify!($scale), "_p2"),
            ln_pow2_input::<2>($scale),
            ln_series_narrow::<$scale>,
            &[("tang_g8_c100", ln_tang_narrow_p::<$scale, 8, 100>)],
        );
    };
}

/// The linear-band control at `SCALE >= 3` (at s0 the band admits only
/// `x = 2`, which the `_p2` cell already is).
macro_rules! narrow_band_cell {
    ($c:expr, $scale:literal) => {
        cell::<2>(
            $c,
            concat!("ln_narrow_s", stringify!($scale), "_band"),
            ln_band_input::<2>($scale),
            ln_series_narrow::<$scale>,
            &[("tang_g8_c100", ln_tang_narrow_p::<$scale, 8, 100>)],
        );
    };
}

fn benches(c: &mut Criterion) {
    // D18 golden scale bands {0, 4, 9, 13, 17} ∪ D38 {0, 9, 19, 28, 37}.
    narrow_cell!(c, 0);
    narrow_cell!(c, 4);
    narrow_cell!(c, 9);
    narrow_cell!(c, 13);
    narrow_cell!(c, 17);
    narrow_cell!(c, 19);
    narrow_cell!(c, 28);
    narrow_cell!(c, 37);
    narrow_band_cell!(c, 4);
    narrow_band_cell!(c, 9);
    narrow_band_cell!(c, 13);
    narrow_band_cell!(c, 17);
    narrow_band_cell!(c, 19);
    narrow_band_cell!(c, 28);
    narrow_band_cell!(c, 37);
    // The wide reference, same binary, at the scales the narrow set shares:
    // D57 at s0 / s28 (its `G8/CAP100` Tang is fixed), D76 at s0 / s19.
    cell::<3>(c, "ln_d57_s0", ln_inputs::<3>(0), ln_series_d57::<0>, &[("tang", ln_tang_d57::<0>)]);
    cell::<3>(c, "ln_d57_s28", ln_inputs::<3>(28), ln_series_d57::<28>, &[("tang", ln_tang_d57::<28>)]);
    cell::<4>(c, "ln_d76_s0", ln_inputs::<4>(0), ln_series_d76::<0>, &[("tang", ln_tang_d76_p::<0, 10, 400>)]);
    cell::<4>(c, "ln_d76_s19", ln_inputs::<4>(19), ln_series_d76::<19>, &[("tang", ln_tang_d76_p::<19, 10, 400>)]);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    benches(&mut c);
    c.final_summary();
}
