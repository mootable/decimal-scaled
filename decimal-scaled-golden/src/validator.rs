//! Validation. One `Validator` trait; the tester holds a `Vec` of them and runs
//! each over a finished `ExecutionCollector`, and each pushes its own verdict(s)
//! via `add_validation`. There are no no-op or default validators — an empty
//! `Vec` is the "validate nothing" case. Validators only ever see the subject's
//! output as a `String` (in the cell); the `Value` type never reaches them.
//!
//! The three concrete validators have DISJOINT domains and self-gate, so a user
//! composes whichever they want:
//! - `RoundingValidator` — fitting (in-range) values: correctly rounded?
//! - `OverflowValidator` — out-of-range values: per the declared overflow policy?
//! - `PrecisionValidator` — fitting values: how far from the true value (ULPs)?

use crate::bigdec::abs_diff;
use crate::collector::{ExecutionCollector, ExecutionResult};
use crate::outcome::Outcome;
use crate::rounding::RoundingMode;
use crate::subject::Overflow;
use crate::value::GoldenValue;

/// Scores a finished cell against the golden value and pushes verdict(s) into it.
/// `scale`/`mode`/`overflow` describe the cell; `representable` = whether the
/// subject can represent the true value (the subject answered, via
/// [`Subject::representable`](crate::subject::Subject::representable)) — the
/// tester computes no storage details itself.
pub trait Validator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        golden: &GoldenValue,
        scale: u32,
        representable: bool,
        mode: RoundingMode,
        overflow: Overflow,
    );
}

/// Correct-rounding check for fitting (in-range) results. Carries the corpus
/// generation precision (the rounding sticky-bit threshold). Pushes nothing for
/// out-of-range cells (that's `OverflowValidator`'s domain).
pub struct RoundingValidator {
    pub gen_precision: usize,
}

impl Validator for RoundingValidator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        golden: &GoldenValue,
        scale: u32,
        _representable: bool,
        mode: RoundingMode,
        _overflow: Overflow,
    ) {
        // "How does it round." Judges a PRODUCED value only — range is never
        // consulted here: an overflowing result (the subject panics, or
        // wraps/saturates) is `OverflowValidator`'s domain, so a panic / no
        // value is silent for rounding.
        let got = match cell.value() {
            Some(s) => s.to_string(),
            None => return,
        };
        let truncated = golden.truncated_at(self.gen_precision);
        let got_scaled = match to_scaled_int(&got, scale) {
            Some(s) => s,
            None => {
                cell.add_validation(Outcome::MisRounded { delta: "nan".to_string() });
                return;
            }
        };
        let expected = golden.round_to(scale, mode, truncated);
        if got_scaled == expected {
            cell.add_validation(Outcome::Pass);
            return;
        }
        // Matches another mode's correct rounding? Report the first in ALL order.
        for m in RoundingMode::ALL {
            if m == mode {
                continue;
            }
            if got_scaled == golden.round_to(scale, m, truncated) {
                cell.add_validation(Outcome::WrongMode { used: m });
                return;
            }
        }
        cell.add_validation(Outcome::MisRounded { delta: abs_diff(&got_scaled, &expected) });
    }
}

/// "How does it overflow." The ONE validator that cares about the result's
/// range — but it does NOT compute the range itself: the subject already
/// answered ([`Subject::representable`](crate::subject::Subject::representable)),
/// and that verdict arrives as `representable`. It also sees whether the subject
/// PANICKED, which for an overflow may be the expected outcome (the default
/// strict contract panics on overflow). It judges: an in-range value is silent
/// (rounding/precision's domain); an in-range panic is an unexpected bug; an
/// out-of-range result must match the declared policy — a `Panic` policy expects
/// the panic.
pub struct OverflowValidator;

impl Validator for OverflowValidator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        _golden: &GoldenValue,
        _scale: u32,
        representable: bool,
        _mode: RoundingMode,
        overflow: Overflow,
    ) {
        // Read the execution result into an owned form first (so we can mutate
        // the cell below without holding a borrow of it).
        let result = cell.execution_result().cloned();
        let outcome = match result {
            Some(ExecutionResult::Panic) => {
                if representable {
                    // Representable result, yet the subject panicked — a real bug.
                    Outcome::Panic
                } else if overflow == Overflow::Panic {
                    // Overflow, and panicking IS the declared policy (the strict
                    // contract) — the expected outcome.
                    Outcome::Pass
                } else {
                    // Overflow, but the subject panicked under a non-panic policy.
                    Outcome::Panic
                }
            }
            Some(ExecutionResult::Value(_)) => {
                if representable {
                    return; // in range — rounding/precision judge the value
                }
                match overflow {
                    // Out of range, Panic policy, yet a value came back instead.
                    Overflow::Panic => Outcome::Error {
                        reason: "declared overflow=Panic but returned a value".to_string(),
                    },
                    // Wrap/Saturate/Truncate need a subject-provided expected
                    // value (no such subject exists yet) — left unvalidated until
                    // one does, rather than baking a storage model into the tester.
                    _ => return,
                }
            }
            None => return,
        };
        cell.add_validation(outcome);
    }
}

/// ULP distance of a fitting result from the correctly-rounded true value
/// (informational, for the precision shootout). Pushes nothing when out of range
/// or the cell produced no value.
pub struct PrecisionValidator {
    pub gen_precision: usize,
}

impl Validator for PrecisionValidator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        golden: &GoldenValue,
        scale: u32,
        _representable: bool,
        mode: RoundingMode,
        _overflow: Overflow,
    ) {
        // "Is it accurate." ULP distance of a PRODUCED value — like rounding,
        // range is never consulted, and a panic / no value is silent (nothing to
        // measure). The cell carries no value on a panic, so `cell.value()` is
        // `None` and we record nothing.
        let got = match cell.value() {
            Some(s) => s.to_string(),
            None => return,
        };
        if let Some(got_scaled) = to_scaled_int(&got, scale) {
            let truncated = golden.truncated_at(self.gen_precision);
            let expected = golden.round_to(scale, mode, truncated);
            cell.add_validation(Outcome::Precision { ulps: abs_diff(&got_scaled, &expected) });
        }
    }
}

/// Normalize a subject's output string to a signed scaled-integer at `scale` (the
/// form `round_to` produces). `None` if unparseable.
fn to_scaled_int(got: &str, scale: u32) -> Option<String> {
    let gv = GoldenValue::parse(got)?;
    Some(gv.round_to(scale, RoundingMode::Trunc, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    const GP: usize = 1233;

    fn value_cell(v: &str) -> ExecutionCollector {
        let mut c = ExecutionCollector::new(vec!["x".into()], "x".into());
        c.record(ExecutionResult::Value(v.to_string()));
        c
    }

    #[test]
    fn rounding_passes_correct() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = value_cell("1.4142");
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 4, true, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::Pass]);
    }

    #[test]
    fn rounding_detects_wrong_mode() {
        // 1.4142135 at scale 6 is an exact tie (terminating 5). HalfToEven rounds
        // UP to 1.414214; the truncated 1.414213 matches the modes that round a
        // positive tie DOWN -- the FIRST in ALL order = HalfTowardZero.
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = value_cell("1.414213");
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 6, true, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::WrongMode { used: RoundingMode::HalfTowardZero }]);
    }

    #[test]
    fn rounding_reports_mis_rounded_delta() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = value_cell("1.4140");
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 4, true, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::MisRounded { delta: "2".to_string() }]);
    }

    fn panic_cell() -> ExecutionCollector {
        let mut c = ExecutionCollector::new(vec!["x".into()], "x".into());
        c.record(ExecutionResult::Panic);
        c
    }

    #[test]
    fn rounding_silent_on_panic() {
        // No value to judge — a panic is OverflowValidator's domain, not rounding's.
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = panic_cell();
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 4, true, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert!(c.validations.is_empty());
    }

    #[test]
    fn precision_zero_when_exact() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = value_cell("1.4142");
        PrecisionValidator { gen_precision: GP }.validate(
            &mut c, &g, 4, true, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::Precision { ulps: "0".to_string() }]);
    }

    #[test]
    fn precision_silent_on_panic() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = panic_cell();
        PrecisionValidator { gen_precision: GP }.validate(
            &mut c, &g, 4, true, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert!(c.validations.is_empty());
    }

    #[test]
    fn overflow_silent_in_range_value() {
        // Representable + a value — overflow is not its concern (rounding/precision are).
        let g = GoldenValue::parse("5").unwrap();
        let mut c = value_cell("5");
        OverflowValidator.validate(&mut c, &g, 0, true, RoundingMode::HalfToEven, Overflow::Panic);
        assert!(c.validations.is_empty());
    }

    #[test]
    fn overflow_panic_is_expected_outcome_of_overflow() {
        // Out of range (representable=false); under a Panic policy, the panic IS correct.
        let g = GoldenValue::parse("9").unwrap();
        let mut c = panic_cell();
        OverflowValidator.validate(&mut c, &g, 0, false, RoundingMode::HalfToEven, Overflow::Panic);
        assert_eq!(c.validations, vec![Outcome::Pass]);
    }

    #[test]
    fn overflow_flags_in_range_panic_as_bug() {
        // Representable, yet the subject panicked — an unexpected bug.
        let g = GoldenValue::parse("123").unwrap();
        let mut c = panic_cell();
        OverflowValidator.validate(&mut c, &g, 0, true, RoundingMode::HalfToEven, Overflow::Panic);
        assert_eq!(c.validations, vec![Outcome::Panic]);
    }

    #[test]
    fn overflow_value_under_panic_policy_is_error() {
        // Out of range, Panic policy, yet a value came back — it should have panicked.
        let g = GoldenValue::parse("9").unwrap();
        let mut c = value_cell("5");
        OverflowValidator.validate(&mut c, &g, 0, false, RoundingMode::HalfToEven, Overflow::Panic);
        assert!(matches!(c.validations.as_slice(), [Outcome::Error { .. }]));
    }
}
