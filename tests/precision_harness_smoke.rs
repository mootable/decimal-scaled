//! Unit tests for the precision-harness machinery itself.
//!
//! The full correctly-rounded gate (every method × width × mode, 286
//! cells) is `tests/ulp_strict_golden.rs`, which drives the same harness
//! with `decimal-scaled` as the reference [`PrecisionSubject`]. This file
//! covers the parts the golden gate exercises only implicitly: the
//! LSBε / ULP accounting and the half-even-vs-truncation fairness fold,
//! on hand-built oracle cases.
//!
//! Gated off under `fast` (the strict path is not the dispatch target
//! there), mirroring the golden gate.

#![cfg(not(feature = "fast"))]

#[path = "support/precision_harness.rs"]
mod harness;
use harness::*;

use decimal_scaled::RoundingMode;

/// The harness's LSBε / ULP arithmetic is exact on hand-built cases.
#[test]
fn harness_lsbe_ulp_arithmetic() {
    // Bit-exact: floor matches, class exact ⇒ 0 LSBε.
    let case = GoldenCase {
        input: "20000000000000000000".into(),
        input2: None,
        floor: "10000000000000000000".into(),
        cls: Cls::Exact,
    };
    let out = SubjectOutput::Computed {
        value: raw_to_decimal_string("10000000000000000000", 19),
        rounding: RoundingMode::HalfToEven,
    };
    match Harness::score(&out, &case, 19) {
        PrecisionResult::Executed { lsbe, ulp, .. } => {
            assert_eq!(lsbe, 0);
            assert_eq!(ulp, 0.0);
        }
        _ => panic!("expected Executed"),
    }

    // Off by exactly 3 LSB ⇒ ULP 3.0, LSBε = 2 (bit-width of 3).
    let out_off = SubjectOutput::Computed {
        value: raw_to_decimal_string("10000000000000000003", 19),
        rounding: RoundingMode::HalfToEven,
    };
    match Harness::score(&out_off, &case, 19) {
        PrecisionResult::Executed { lsbe, ulp, .. } => {
            assert_eq!(ulp, 3.0);
            assert_eq!(lsbe, 2, "bit-width of 3 is 2");
        }
        _ => panic!("expected Executed"),
    }

    // NotApplicable propagates.
    assert!(matches!(
        Harness::score(&SubjectOutput::NotApplicable, &case, 19),
        PrecisionResult::NotApplicable
    ));
}

/// The fairness fold: a truncating subject scored against the truncated
/// oracle is bit-exact even when half-even would round up.
#[test]
fn harness_fairness_fold() {
    // True value just above floor (class High): half-even rounds UP to
    // floor+1, truncation keeps floor. A subject reporting Trunc that
    // emits floor must score 0 LSBε.
    let case = GoldenCase {
        input: "1".into(),
        input2: None,
        floor: "42".into(),
        cls: Cls::High,
    };
    let trunc_out = SubjectOutput::Computed {
        value: raw_to_decimal_string("42", 19),
        rounding: RoundingMode::Trunc,
    };
    assert!(
        Harness::score(&trunc_out, &case, 19).is_correctly_rounded(),
        "truncating subject emitting floor must be CR under Trunc fold"
    );
    // The SAME emitted value scored under HalfToEven (rounds up) is off
    // by 1 LSB.
    let he_out = SubjectOutput::Computed {
        value: raw_to_decimal_string("42", 19),
        rounding: RoundingMode::HalfToEven,
    };
    match Harness::score(&he_out, &case, 19) {
        PrecisionResult::Executed { ulp, .. } => assert_eq!(ulp, 1.0),
        _ => panic!("expected Executed"),
    }
}

/// A high-precision peer's emitted decimal is ROUNDED (not truncated) to
/// the storage grid under its reported mode before diffing, so a faithful
/// peer is not docked a full LSB by the parse.
#[test]
fn harness_subject_value_is_rounded_to_grid() {
    // Oracle CR value is floor+1 = 11 (class High under HalfAway).
    let case = GoldenCase {
        input: "1".into(),
        input2: None,
        floor: "10".into(),
        cls: Cls::High,
    };
    // Peer emits 1.05000000003 at scale 1 ⇒ rounds to 11 under HalfAway,
    // matching the oracle exactly. (Truncation would give 10 ⇒ 1 LSB off.)
    let out = SubjectOutput::Computed {
        value: "1.05000000003".into(),
        rounding: RoundingMode::HalfAwayFromZero,
    };
    match Harness::score(&out, &case, 1) {
        PrecisionResult::Executed { lsbe, ulp, .. } => {
            assert_eq!(lsbe, 0, "rounded-to-grid peer value must match oracle");
            assert_eq!(ulp, 0.0);
        }
        _ => panic!("expected Executed"),
    }
}
