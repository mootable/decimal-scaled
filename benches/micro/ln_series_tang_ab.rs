// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the `ln` Series-vs-Tang policy choice
//! (`src/policy/ln.rs`): the table-driven Tang kernel skips the Series path's
//! wide argument-reduction sqrts, so it wins across each narrow-wide tier's
//! full scale range. `select` now routes the whole `1..=MAX_SCALE` range at
//! D57/D115/D153 to Tang (previously a narrow centred window). This bench
//! ranks production Tang against `ln_series` at sample scales spanning each
//! band (incl. the golden scale s28/s57/s76).
//!
//! Tiers (the `wide`-buildable narrow-wide widths):
//! - D57 (Int<3>): GUARD=8, CAP=100.
//! - D115 (Int<6>): GUARD=8, CAP=200.
//! - D153 (Int<8>): GUARD=10, CAP=200.
//!
//! Tang and Series are asserted bit-identical across the value spread × six
//! modes before timing (the validity wall). ln's domain is x > 0, so arguments
//! are x ∈ {0.5, 2.0, 7.5}. Correct rounding over the widened range is gated by
//! the golden-gate ln cells (the near-grid-line directed-mode case the
//! 3-value spread cannot probe).
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

/// Build `digit · 10^pow10` as a wide `Int<N>` magnitude by iterative ×10 on
/// the limb representation — NEVER via `10u128.pow(scale)`, which overflows
/// u128 (max ≈ 3.4e38) for every wide band (scale ≥ 39) and can never fill
/// more than two limbs. This builds true N-limb operands at any scale.
fn from_digit_pow10<const N: usize>(digit: u64, pow10: u32) -> Int<N> {
    let mut seed = [0u64; N];
    seed[0] = digit;
    let mut v = int_from_mag_limbs::<N>(&seed);
    let mut ten_mag = [0u64; N];
    ten_mag[0] = 10;
    let ten = int_from_mag_limbs::<N>(&ten_mag);
    for _ in 0..pow10 {
        v = v * ten;
    }
    v
}

#[derive(Clone)]
struct One<const N: usize> {
    label: &'static str,
    raw: Int<N>,
}

/// x ∈ {0.5, 2.0, 7.5} at decimal `SCALE` → raw = x · 10^SCALE. Expressed as
/// `digit · 10^(SCALE-1)` (0.5→5, 2.0→20, 7.5→75) so the magnitude is built
/// exactly with no division and no u128 intermediate. All > 0 (ln domain) and
/// inside the shared reduction window at every band. (SCALE ≥ 1 at every band.)
fn ln_inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    let e = scale - 1;
    vec![
        One { label: "x0.5", raw: from_digit_pow10::<N>(5, e) },
        One { label: "x2.0", raw: from_digit_pow10::<N>(20, e) },
        One { label: "x7.5", raw: from_digit_pow10::<N>(75, e) },
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
    // Probe each tier across its full Tang band (now 1..=MAX_SCALE in
    // `policy::ln::select`). Cells include the golden scale (s28/s57/s76)
    // plus low/mid/high samples. The top sample stops a couple of scales
    // below MAX_SCALE so the 7.5 operand (`75·10^(scale-1)`) still fits the
    // tier's storage bits.
    // D57 (Int<3>, MAX_SCALE 56).
    cell::<3>(c, "ln_d57_s1", 1, ln_series_d57::<1>, ln_tang_d57::<1>);
    cell::<3>(c, "ln_d57_s10", 10, ln_series_d57::<10>, ln_tang_d57::<10>);
    cell::<3>(c, "ln_d57_s28_gold", 28, ln_series_d57::<28>, ln_tang_d57::<28>);
    cell::<3>(c, "ln_d57_s40", 40, ln_series_d57::<40>, ln_tang_d57::<40>);
    cell::<3>(c, "ln_d57_s54", 54, ln_series_d57::<54>, ln_tang_d57::<54>);
    // D115 (Int<6>, MAX_SCALE 114).
    cell::<6>(c, "ln_d115_s1", 1, ln_series_d115::<1>, ln_tang_d115::<1>);
    cell::<6>(c, "ln_d115_s30", 30, ln_series_d115::<30>, ln_tang_d115::<30>);
    cell::<6>(c, "ln_d115_s57_gold", 57, ln_series_d115::<57>, ln_tang_d115::<57>);
    cell::<6>(c, "ln_d115_s90", 90, ln_series_d115::<90>, ln_tang_d115::<90>);
    cell::<6>(c, "ln_d115_s112", 112, ln_series_d115::<112>, ln_tang_d115::<112>);
    // D153 (Int<8>, MAX_SCALE 152).
    cell::<8>(c, "ln_d153_s1", 1, ln_series_d153::<1>, ln_tang_d153::<1>);
    cell::<8>(c, "ln_d153_s40", 40, ln_series_d153::<40>, ln_tang_d153::<40>);
    cell::<8>(c, "ln_d153_s76_gold", 76, ln_series_d153::<76>, ln_tang_d153::<76>);
    cell::<8>(c, "ln_d153_s120", 120, ln_series_d153::<120>, ln_tang_d153::<120>);
    cell::<8>(c, "ln_d153_s150", 150, ln_series_d153::<150>, ln_tang_d153::<150>);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
