// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the `ln` Series-vs-Tang policy choice
//! (`src/policy/ln.rs`).
//!
//! `policy::ln::select` routes many discrete `(N, SCALE)` bands to the Tang
//! table-driven kernel and everything else to the generic `ln_series`. Those
//! bands were tuned via the standalone `benches/lookup/*` kernel-ISOLATION
//! benches (and `m2_ln_approx`) — never an N-way `compare_all` of Series-vs-Tang
//! at the dispatch seam. This bench closes that gap: at each Tang band it A/Bs
//! the production Tang kernel (same `Core`/`GUARD`/`ITERS`/flag params the
//! policy passes) against `ln_series`, plus a just-OUT-of-band probe on each
//! edge so the coordinator can confirm the crossover sits where `select` puts
//! it (Tang in-band, Series just-out-of-band).
//!
//! Bands covered (the `wide`-buildable narrow-wide tiers):
//! - D57 (Int<3>): 18..=22 (GUARD=8, ITERS=100, false).
//! - D115 (Int<6>): 50..=60 (GUARD=8, ITERS=200, true).
//! - D153 (Int<8>): 70..=82 (GUARD=10, ITERS=200, true).
//!
//! The wider double-band tiers (D307/D616/D924/D1232 each have a low + high
//! Tang window with a Series gap between them) need x-wide / xx-wide builds and
//! their own export cells — left for a follow-up; the inter-band-gap probe the
//! ledger calls for belongs there.
//!
//! OPTIMALITY A/B over a VALID region: Tang and Series are bit-identical
//! in-band (both correctly-rounded), asserted across the spread × six modes
//! before timing. ln's domain is x > 0, so arguments are x ∈ {0.5, 2.0, 7.5}.
//! A just-out-of-band probe runs Tang at a SCALE the policy does NOT route to
//! it; if such a probe asserts unequal, that scale is past Tang's validity and
//! the in-band edge is the validity wall (drop the probe), not a widen point.
//!
//! Run with:
//! `cargo bench --features "wide bench-alt" --bench ln_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    int_from_mag_limbs, ln_series_d115, ln_series_d153, ln_series_d57, ln_tang_d115, ln_tang_d153,
    ln_tang_d57,
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

/// x ∈ {0.5, 2.0, 7.5} at decimal `SCALE` → raw = x · 10^SCALE. All > 0
/// (ln domain) and inside the shared reduction window at every band.
fn ln_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale);
    vec![
        One { label: "x0.5", raw: fromu::<N>(p / 2) },
        One { label: "x2.0", raw: fromu::<N>(2 * p) },
        One { label: "x7.5", raw: fromu::<N>(7 * p + p / 2) },
    ]
}

fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: fn(Int<N>, RoundingMode) -> Int<N>,
    tang: fn(Int<N>, RoundingMode) -> Int<N>,
) {
    for o in ln_inputs::<N>(scale) {
        for m in ALL_MODES {
            assert_eq!(tang(o.raw, m), series(o.raw, m), "{group} {} mode {m:?}", o.label);
        }
    }
    compare_all(
        c,
        group,
        |o: &One<N>| o.label.to_string(),
        ln_inputs::<N>(scale),
        vec![
            ("tang", Box::new(move |o: One<N>| tang(o.raw, MODE)) as Box<dyn Fn(One<N>) -> Int<N>>),
            ("series", Box::new(move |o: One<N>| series(o.raw, MODE))),
        ],
    );
}

fn benches(c: &mut Criterion) {
    // D57 band 18..=22.
    cell::<3>(c, "ln_d57_s18_in", 18, ln_series_d57::<18>, ln_tang_d57::<18>);
    cell::<3>(c, "ln_d57_s22_in", 22, ln_series_d57::<22>, ln_tang_d57::<22>);
    cell::<3>(c, "ln_d57_s17_out", 17, ln_series_d57::<17>, ln_tang_d57::<17>);
    cell::<3>(c, "ln_d57_s23_out", 23, ln_series_d57::<23>, ln_tang_d57::<23>);
    // D115 band 50..=60.
    cell::<6>(c, "ln_d115_s50_in", 50, ln_series_d115::<50>, ln_tang_d115::<50>);
    cell::<6>(c, "ln_d115_s60_in", 60, ln_series_d115::<60>, ln_tang_d115::<60>);
    cell::<6>(c, "ln_d115_s49_out", 49, ln_series_d115::<49>, ln_tang_d115::<49>);
    cell::<6>(c, "ln_d115_s61_out", 61, ln_series_d115::<61>, ln_tang_d115::<61>);
    // D153 band 70..=82.
    cell::<8>(c, "ln_d153_s70_in", 70, ln_series_d153::<70>, ln_tang_d153::<70>);
    cell::<8>(c, "ln_d153_s82_in", 82, ln_series_d153::<82>, ln_tang_d153::<82>);
    cell::<8>(c, "ln_d153_s69_out", 69, ln_series_d153::<69>, ln_tang_d153::<69>);
    cell::<8>(c, "ln_d153_s83_out", 83, ln_series_d153::<83>, ln_tang_d153::<83>);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
