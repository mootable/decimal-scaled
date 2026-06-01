// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Focused zero-tolerance golden gate for the D57 atan **Tang band**
//! (`SCALE ∈ 44..=56`), the cell the generic `ulp_strict_golden` D57 row
//! (which runs at `SCALE = 28`, OUTSIDE the band) does not exercise.
//!
//! The atan Tang kernel `algos::trig::atan_tang_3limb_s44_56` now reads
//! `atan(c_j)` from the BAKED binary Tang table
//! (`algos::support::atan_tang_table`) instead of recomputing it per call
//! via the generic `atan_fixed` halving chain. This test pins that the
//! baked-table path is correctly rounded (`lsbe == 0`) at the band edge
//! `SCALE = 56` across every `RoundingMode` and every golden input — the
//! `0.5`-ULP guarantee the perf change must not erode.
//!
//! Built only with the `wide` feature (the D57 Tang path is a wide-tier
//! kernel); the harness includes mirror `ulp_strict_golden.rs`.

#![cfg(feature = "wide")]

#[path = "support/precision_harness.rs"]
mod harness;

#[path = "support/precision_subject_ds.rs"]
mod subject_ds;

use decimal_scaled::RoundingMode;
use harness::{
    GoldenCase, Harness, Input, Method, PrecisionResult, PrecisionSubject, Width,
    parse_golden_line,
};
use subject_ds::DecimalScaledSubject;

const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Run one atan golden table at an explicit `scale` and assert every
/// scored (input, mode) cell is correctly rounded (`lsbe == 0`).
fn check_atan(scale: u32, table: &str) {
    let subject = DecimalScaledSubject;
    let mut failures = 0usize;
    let mut cases = 0usize;
    for line in table.lines() {
        let Some(GoldenCase {
            input,
            input2,
            floor,
            cls,
        }) = parse_golden_line(line)
        else {
            continue;
        };
        cases += 1;
        let case = GoldenCase {
            input: input.clone(),
            input2: input2.clone(),
            floor: floor.clone(),
            cls,
        };
        let inp = Input {
            raw: input.clone(),
            input2: input2.clone(),
            width: Width::D57,
            scale,
        };
        for &mode in MODES.iter() {
            let out = subject.eval(Method::Atan, Width::D57, scale, &inp, mode);
            match Harness::score(&out, &case, scale) {
                PrecisionResult::NotApplicable => {
                    eprintln!(
                        "FAIL: atan D57 s{scale} mode={mode:?} input={input}: NotApplicable",
                    );
                    failures += 1;
                }
                PrecisionResult::Executed {
                    lsbe, ulp, value, ..
                } => {
                    if lsbe != 0 {
                        eprintln!(
                            "FAIL: atan D57 s{scale} mode={mode:?} input={input} \
                             floor={floor} cls={cls:?} value={value} lsbe={lsbe} ulp={ulp}",
                        );
                        failures += 1;
                    }
                }
            }
        }
    }
    assert!(cases > 0, "atan D57 s{scale}: golden table had no cases");
    assert!(
        failures == 0,
        "atan D57 s{scale}: {failures} (case, mode) pairs not correctly rounded (lsbe != 0)",
    );
}

/// The band EDGE (`SCALE = 56`) — the worst bbc cell `atan_D57_s56`, and
/// the deepest scale the baked-table reconstruction must round correctly.
#[test]
fn atan_d57_s56_tang_band_correctly_rounded() {
    check_atan(56, include_str!("golden/atan_d57_s56.txt"));
}
