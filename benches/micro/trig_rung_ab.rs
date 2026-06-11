// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the forward-trig SCALE-derived work-rung
//! (`src/policy/trig.rs` `forward_rung` + `src/policy/work_rung.rs`).
//!
//! The rung routes a low-working-scale, small-|x| cell to the ONE
//! generic kernel monomorphised at the narrowest valid work integer
//! (e.g. `Int<12>` at SCALE 0) instead of the tier-fixed `$Work`
//! (`Int<64>` at D307, `Int<176>` at D1232). This bench compares the
//! tier-width Series kernel against the rung-routed production path
//! across the scale axis at D307 (`x-wide`) and D1232 (`xx-wide`),
//! plus an out-of-budget large-|x| probe that must route to (and
//! therefore match the cost of) the tier path.
//!
//! VALIDITY before timing: rung and tier are bit-identical at every
//! probed cell across all six rounding modes (the integer ops are
//! width-agnostic; the assert is the wall).
//!
//! Run with:
//! `cargo bench --features "x-wide xx-wide bench-alt" --bench trig_rung_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    asin_rung_d1232, asin_rung_d307, asin_schoolbook_d1232, asin_schoolbook_d307,
    asinh_rung_d1232, asinh_rung_d307, asinh_schoolbook_d1232, asinh_schoolbook_d307,
    atan_rung_d1232, atan_rung_d307, atan_series_d1232, atan_series_d307, cos_rung_d1232,
    cos_rung_d307, cos_series_d1232, cos_series_d307, exp_rung_d1232, exp_rung_d307,
    exp_series_d1232, exp_series_d307, int_from_mag_limbs, sin_rung_d1232, sin_rung_d307,
    sin_series_d1232, sin_series_d307, sinh_rung_d1232, sinh_rung_d307, sinh_schoolbook_d1232,
    sinh_schoolbook_d307, tan_rung_d1232, tan_rung_d307, tan_series_d1232, tan_series_d307,
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

/// Small-|x| (in-budget) inputs: x ∈ {0.3, 1.0, 1.5} rad at decimal
/// `scale`, plus the near-π-multiple 3141 (4 integer digits, still
/// in-budget) — raw = x · 10^scale.
fn inputs<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale.min(36));
    let mut v = vec![
        One { label: "x0.3", raw: fromu::<N>(3 * p / 10) },
        One { label: "x1.0", raw: fromu::<N>(p) },
        One { label: "x1.5", raw: fromu::<N>(p + p / 2) },
    ];
    if scale <= 34 {
        v.push(One { label: "x3141", raw: fromu::<N>(3141 * p) });
    }
    v
}

/// Out-of-budget probe: x = 10^9 rad (past `D_BUDGET = 8` integer
/// digits) — the gate must route this to the tier path.
fn big_input<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale.min(27));
    vec![One { label: "x1e9", raw: fromu::<N>(1_000_000_000 * p) }]
}

/// In-domain inputs for asin (|x| ≤ 1): the |x| ≤ 1/2 direct branch,
/// the half-angle branch, and the exact endpoint.
fn inputs_unit<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale.min(36));
    vec![
        One { label: "x0.3", raw: fromu::<N>(3 * p / 10) },
        One { label: "x0.9", raw: fromu::<N>(9 * p / 10) },
        One { label: "x1.0", raw: fromu::<N>(p) },
    ]
}

/// In-budget exp/sinh inputs (|x| < 10, the `EXP_ARG_BUDGET` region)
/// plus the budget edge 9.5.
fn inputs_small<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale.min(36));
    vec![
        One { label: "x0.3", raw: fromu::<N>(3 * p / 10) },
        One { label: "x1.0", raw: fromu::<N>(p) },
        One { label: "x1.5", raw: fromu::<N>(p + p / 2) },
        One { label: "x9.5", raw: fromu::<N>(9 * p + p / 2) },
    ]
}

/// Out-of-budget exp/sinh probe (|x| = 50 ≥ 10, result still
/// representable at the probed scales) — must route to the tier path.
fn big_input_exp<const N: usize>(scale: u32) -> Vec<One<N>> {
    let p = 10u128.pow(scale.min(34));
    vec![One { label: "x50", raw: fromu::<N>(50 * p) }]
}

fn cell_with<const N: usize>(
    c: &mut Criterion,
    group: &str,
    ins: Vec<One<N>>,
    series: fn(Int<N>, RoundingMode) -> Int<N>,
    rung: fn(Int<N>, RoundingMode) -> Int<N>,
) {
    for o in &ins {
        for m in ALL_MODES {
            assert_eq!(rung(o.raw, m), series(o.raw, m), "{group} {} mode {m:?}", o.label);
        }
    }
    compare_all(
        c,
        group,
        |o: &One<N>| o.label.to_string(),
        ins,
        vec![
            ("rung", Box::new(move |o: One<N>| rung(o.raw, MODE)) as Box<dyn Fn(One<N>) -> Int<N>>),
            ("tier", Box::new(move |o: One<N>| series(o.raw, MODE))),
        ],
    );
}

fn cell<const N: usize>(
    c: &mut Criterion,
    group: &str,
    scale: u32,
    series: fn(Int<N>, RoundingMode) -> Int<N>,
    rung: fn(Int<N>, RoundingMode) -> Int<N>,
    big: bool,
) {
    let ins = if big { big_input::<N>(scale) } else { inputs::<N>(scale) };
    cell_with::<N>(c, group, ins, series, rung);
}

fn benches(c: &mut Criterion) {
    // D307 (Int<16> storage, Int<64> $Work) — scale spread {0, S/4, S/2,
    // 3S/4, S-1} for sin; anchor scales for cos/tan; one large-|x| probe.
    cell::<16>(c, "sin_d307_s0", 0, sin_series_d307::<0>, sin_rung_d307::<0>, false);
    cell::<16>(c, "sin_d307_s76", 76, sin_series_d307::<76>, sin_rung_d307::<76>, false);
    cell::<16>(c, "sin_d307_s153", 153, sin_series_d307::<153>, sin_rung_d307::<153>, false);
    cell::<16>(c, "sin_d307_s229", 229, sin_series_d307::<229>, sin_rung_d307::<229>, false);
    cell::<16>(c, "sin_d307_s306", 306, sin_series_d307::<306>, sin_rung_d307::<306>, false);
    cell::<16>(c, "cos_d307_s0", 0, cos_series_d307::<0>, cos_rung_d307::<0>, false);
    cell::<16>(c, "cos_d307_s153", 153, cos_series_d307::<153>, cos_rung_d307::<153>, false);
    cell::<16>(c, "tan_d307_s0", 0, tan_series_d307::<0>, tan_rung_d307::<0>, false);
    cell::<16>(c, "tan_d307_s153", 153, tan_series_d307::<153>, tan_rung_d307::<153>, false);
    cell::<16>(c, "sin_d307_s0_big", 0, sin_series_d307::<0>, sin_rung_d307::<0>, true);

    // D1232 (Int<64> storage, Int<176> $Work).
    cell::<64>(c, "sin_d1232_s0", 0, sin_series_d1232::<0>, sin_rung_d1232::<0>, false);
    cell::<64>(c, "sin_d1232_s308", 308, sin_series_d1232::<308>, sin_rung_d1232::<308>, false);
    cell::<64>(c, "sin_d1232_s616", 616, sin_series_d1232::<616>, sin_rung_d1232::<616>, false);
    cell::<64>(c, "sin_d1232_s924", 924, sin_series_d1232::<924>, sin_rung_d1232::<924>, false);
    cell::<64>(c, "sin_d1232_s1231", 1231, sin_series_d1232::<1231>, sin_rung_d1232::<1231>, false);
    cell::<64>(c, "cos_d1232_s0", 0, cos_series_d1232::<0>, cos_rung_d1232::<0>, false);
    cell::<64>(c, "cos_d1232_s616", 616, cos_series_d1232::<616>, cos_rung_d1232::<616>, false);
    cell::<64>(c, "tan_d1232_s0", 0, tan_series_d1232::<0>, tan_rung_d1232::<0>, false);
    cell::<64>(c, "tan_d1232_s616", 616, tan_series_d1232::<616>, tan_rung_d1232::<616>, false);
    cell::<64>(c, "sin_d1232_s0_big", 0, sin_series_d1232::<0>, sin_rung_d1232::<0>, true);

    // ── Rung follow-ups: atan / asin / sinh / exp tier-vs-rung ─────────
    // atan (the biggest remaining s0 cells) — full inputs() spread, the
    // out-of-budget probe, anchor scales at both widths.
    cell::<16>(c, "atan_d307_s0", 0, atan_series_d307::<0>, atan_rung_d307::<0>, false);
    cell::<16>(c, "atan_d307_s153", 153, atan_series_d307::<153>, atan_rung_d307::<153>, false);
    cell::<16>(c, "atan_d307_s306", 306, atan_series_d307::<306>, atan_rung_d307::<306>, false);
    cell::<16>(c, "atan_d307_s0_big", 0, atan_series_d307::<0>, atan_rung_d307::<0>, true);
    cell::<64>(c, "atan_d1232_s0", 0, atan_series_d1232::<0>, atan_rung_d1232::<0>, false);
    cell::<64>(c, "atan_d1232_s616", 616, atan_series_d1232::<616>, atan_rung_d1232::<616>, false);
    cell::<64>(c, "atan_d1232_s1231", 1231, atan_series_d1232::<1231>, atan_rung_d1232::<1231>, false);

    // asin (the inverse composition; acos/atan2 share its kernel shape).
    cell_with::<16>(c, "asin_d307_s0", inputs_unit::<16>(0), asin_schoolbook_d307::<0>, asin_rung_d307::<0>);
    cell_with::<16>(c, "asin_d307_s153", inputs_unit::<16>(153), asin_schoolbook_d307::<153>, asin_rung_d307::<153>);
    cell_with::<64>(c, "asin_d1232_s0", inputs_unit::<64>(0), asin_schoolbook_d1232::<0>, asin_rung_d1232::<0>);
    cell_with::<64>(c, "asin_d1232_s616", inputs_unit::<64>(616), asin_schoolbook_d1232::<616>, asin_rung_d1232::<616>);

    // sinh (the exp-identity composition; cosh/tanh share its shape) —
    // the EXP_ARG_BUDGET region + the out-of-budget |x| = 50 probe.
    cell_with::<16>(c, "sinh_d307_s0", inputs_small::<16>(0), sinh_schoolbook_d307::<0>, sinh_rung_d307::<0>);
    cell_with::<16>(c, "sinh_d307_s153", inputs_small::<16>(153), sinh_schoolbook_d307::<153>, sinh_rung_d307::<153>);
    cell_with::<16>(c, "sinh_d307_s0_big", big_input_exp::<16>(0), sinh_schoolbook_d307::<0>, sinh_rung_d307::<0>);
    cell_with::<64>(c, "sinh_d1232_s0", inputs_small::<64>(0), sinh_schoolbook_d1232::<0>, sinh_rung_d1232::<0>);
    cell_with::<64>(c, "sinh_d1232_s616", inputs_small::<64>(616), sinh_schoolbook_d1232::<616>, sinh_rung_d1232::<616>);

    // asinh (the ln/sqrt composition; acosh/atanh share its shape, at
    // the near-special 2·SCALE selector).
    cell_with::<16>(c, "asinh_d307_s0", inputs_small::<16>(0), asinh_schoolbook_d307::<0>, asinh_rung_d307::<0>);
    cell_with::<16>(c, "asinh_d307_s153", inputs_small::<16>(153), asinh_schoolbook_d307::<153>, asinh_rung_d307::<153>);
    cell_with::<64>(c, "asinh_d1232_s0", inputs_small::<64>(0), asinh_schoolbook_d1232::<0>, asinh_rung_d1232::<0>);

    // exp (the Series storage-strict path).
    cell_with::<16>(c, "exp_d307_s0", inputs_small::<16>(0), exp_series_d307::<0>, exp_rung_d307::<0>);
    cell_with::<16>(c, "exp_d307_s153", inputs_small::<16>(153), exp_series_d307::<153>, exp_rung_d307::<153>);
    cell_with::<16>(c, "exp_d307_s0_big", big_input_exp::<16>(0), exp_series_d307::<0>, exp_rung_d307::<0>);
    cell_with::<64>(c, "exp_d1232_s0", inputs_small::<64>(0), exp_series_d1232::<0>, exp_rung_d1232::<0>);
    cell_with::<64>(c, "exp_d1232_s616", inputs_small::<64>(616), exp_series_d1232::<616>, exp_rung_d1232::<616>);
}

fn main() {
    let mut c = micro_criterion();
    benches(&mut c);
    c.final_summary();
}
