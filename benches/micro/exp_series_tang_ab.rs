// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the `exp` Series-vs-Tang policy choice
//! (`src/policy/exp.rs`).
//!
//! `policy::exp::select` routes a discrete set of `(N, SCALE)` bands to the
//! Tang table-driven kernel and everything else to the generic `exp_series`.
//! Those bands were tuned via the standalone `benches/lookup/*` kernel-
//! ISOLATION benches — never an N-way `compare_all` of Series-vs-Tang at the
//! dispatch seam. This bench closes that gap: at each Tang band it A/Bs the
//! production Tang kernel (same `Core`/`M`/`GUARD`/flag params the policy
//! passes) against `exp_series`, and probes one SCALE just-OUT-of-band on each
//! edge so the coordinator can confirm the crossover sits where `select` puts
//! it (Tang should win in-band; Series should win just-out-of-band).
//!
//! Bands covered (the `wide`-buildable narrow-wide tiers):
//! - D57 (Int<3>): 18..=22 (M=128, G=8) and 45..=56 (M=512, G=30).
//! - D115 (Int<6>): 50..=60 (M=128, G=8).
//! - D153 (Int<8>): 70..=82 (M=128, G=10).
//!
//! IMPORTANT: this is an OPTIMALITY A/B over a VALID region. Tang and Series
//! are bit-identical inside a band (both correctly-rounded), so the harness
//! asserts `tang == series` across the operand spread and all six modes before
//! timing. The JUST-OUT-OF-BAND probe cells exercise Tang at a SCALE the policy
//! does NOT route to it — Tang must stay numerically correct there for the A/B
//! to be meaningful; if a probe asserts unequal, that scale is outside Tang's
//! validity and the coordinator should treat the in-band edge as the validity
//! wall (drop that probe cell), not widen the band past it.
//!
//! Run with:
//! `cargo bench --features "wide bench-alt" --bench exp_series_tang_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    exp_series_d115, exp_series_d153, exp_series_d57, exp_tang_d115, exp_tang_d153, exp_tang_d57,
    int_from_mag_limbs,
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

/// Build an `Int<N>` from a non-negative magnitude (low two u64 limbs).
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

/// A small spread of arguments to `exp` at decimal `SCALE`: x in
/// {0.25, 1.5, 3.0} → raw = x · 10^SCALE. These stay inside the convergence /
/// range-reduction window the Tang and Series kernels share at every band.
fn exp_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale);
    vec![
        One { label: "x0.25", raw: fromu::<N>(p / 4) },
        One { label: "x1.5", raw: fromu::<N>(p + p / 2) },
        One { label: "x3.0", raw: fromu::<N>(3 * p) },
    ]
}

/// One band-cell A/B: assert Tang == Series across spread × all modes, then
/// rank them. `series` / `tang` are `Copy` `fn(Int<N>, RoundingMode) ->
/// Int<N>` pointers (the band M/G is baked into the chosen `tang` export).
fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: fn(Int<N>, RoundingMode) -> Int<N>,
    tang: fn(Int<N>, RoundingMode) -> Int<N>,
) {
    for o in exp_inputs::<N>(scale) {
        for m in ALL_MODES {
            assert_eq!(tang(o.raw, m), series(o.raw, m), "{group} {} mode {m:?}", o.label);
        }
    }
    compare_all(
        c,
        group,
        |o: &One<N>| o.label.to_string(),
        exp_inputs::<N>(scale),
        vec![
            ("tang", Box::new(move |o: One<N>| tang(o.raw, MODE)) as Box<dyn Fn(One<N>) -> Int<N>>),
            ("series", Box::new(move |o: One<N>| series(o.raw, MODE))),
        ],
    );
}

fn benches(c: &mut Criterion) {
    // D57 band 18..=22 (M=128, G=8): in-band edges + just-out probes.
    cell::<3>(c, "exp_d57_s18_in", 18, exp_series_d57::<18>, exp_tang_d57::<18, 128, 8>);
    cell::<3>(c, "exp_d57_s22_in", 22, exp_series_d57::<22>, exp_tang_d57::<22, 128, 8>);
    cell::<3>(c, "exp_d57_s17_out", 17, exp_series_d57::<17>, exp_tang_d57::<17, 128, 8>);
    cell::<3>(c, "exp_d57_s23_out", 23, exp_series_d57::<23>, exp_tang_d57::<23, 128, 8>);
    // D57 band 45..=56 (M=512, G=30): in-band edges + just-out probes.
    cell::<3>(c, "exp_d57_s45_in", 45, exp_series_d57::<45>, exp_tang_d57::<45, 512, 30>);
    cell::<3>(c, "exp_d57_s56_in", 56, exp_series_d57::<56>, exp_tang_d57::<56, 512, 30>);
    cell::<3>(c, "exp_d57_s44_out", 44, exp_series_d57::<44>, exp_tang_d57::<44, 512, 30>);
    cell::<3>(c, "exp_d57_s57_out", 57, exp_series_d57::<57>, exp_tang_d57::<57, 512, 30>);
    // D115 band 50..=60 (M=128, G=8).
    cell::<6>(c, "exp_d115_s50_in", 50, exp_series_d115::<50>, exp_tang_d115::<50>);
    cell::<6>(c, "exp_d115_s60_in", 60, exp_series_d115::<60>, exp_tang_d115::<60>);
    cell::<6>(c, "exp_d115_s49_out", 49, exp_series_d115::<49>, exp_tang_d115::<49>);
    cell::<6>(c, "exp_d115_s61_out", 61, exp_series_d115::<61>, exp_tang_d115::<61>);
    // D153 band 70..=82 (M=128, G=10).
    cell::<8>(c, "exp_d153_s70_in", 70, exp_series_d153::<70>, exp_tang_d153::<70>);
    cell::<8>(c, "exp_d153_s82_in", 82, exp_series_d153::<82>, exp_tang_d153::<82>);
    cell::<8>(c, "exp_d153_s69_out", 69, exp_series_d153::<69>, exp_tang_d153::<69>);
    cell::<8>(c, "exp_d153_s83_out", 83, exp_series_d153::<83>, exp_tang_d153::<83>);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
