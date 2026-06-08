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
use crate::overflow::expected_overflow;
use crate::rounding::RoundingMode;
use crate::subject::Overflow;
use crate::value::GoldenValue;

/// Scores a finished cell against the golden value and pushes verdict(s) into it.
/// `width`/`scale`/`storage_bits`/`mode`/`overflow` describe the cell.
pub trait Validator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        storage_bits: u32,
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
        width: u32,
        scale: u32,
        _storage_bits: u32,
        mode: RoundingMode,
        _overflow: Overflow,
    ) {
        if !golden.fits(width, scale) {
            return; // out of range — OverflowValidator's domain
        }
        let got = match cell.execution_result() {
            Some(ExecutionResult::Value(s)) => s.clone(),
            // Panicked on a representable input — a bug.
            Some(ExecutionResult::Panic) => {
                cell.add_validation(Outcome::Panic);
                return;
            }
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

/// Overflow-policy check for out-of-range results. Pushes nothing for in-range
/// cells (that's `RoundingValidator`'s domain).
pub struct OverflowValidator;

impl Validator for OverflowValidator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        storage_bits: u32,
        _mode: RoundingMode,
        overflow: Overflow,
    ) {
        if golden.fits(width, scale) {
            return; // in range — RoundingValidator's domain
        }
        let expected = expected_overflow(golden, width, scale, storage_bits, overflow);
        let outcome = match cell.execution_result() {
            Some(ExecutionResult::Panic) => {
                if overflow == Overflow::Panic { Outcome::Pass } else { Outcome::Panic }
            }
            Some(ExecutionResult::Value(s)) => match expected {
                None => Outcome::Error {
                    reason: "declared overflow=Panic but returned a value".to_string(),
                },
                Some(exp) => match to_scaled_int(s, scale) {
                    Some(got_scaled) if got_scaled == exp => Outcome::Pass,
                    Some(got_scaled) => Outcome::MisRounded { delta: abs_diff(&got_scaled, &exp) },
                    None => Outcome::MisRounded { delta: "nan".to_string() },
                },
            },
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
        width: u32,
        scale: u32,
        _storage_bits: u32,
        mode: RoundingMode,
        _overflow: Overflow,
    ) {
        if !golden.fits(width, scale) {
            return;
        }
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
/// form `round_to`/`expected_overflow` produce). `None` if unparseable.
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
            &mut c, &g, 38, 4, 128, RoundingMode::HalfToEven, Overflow::Panic,
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
            &mut c, &g, 38, 6, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::WrongMode { used: RoundingMode::HalfTowardZero }]);
    }

    #[test]
    fn rounding_reports_mis_rounded_delta() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = value_cell("1.4140");
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 38, 4, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::MisRounded { delta: "2".to_string() }]);
    }

    #[test]
    fn rounding_silent_out_of_range() {
        // 123.4 doesn't fit scale 37 in 38 digits -> rounding is not its domain.
        let g = GoldenValue::parse("123.4").unwrap();
        let mut c = value_cell("123.4");
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 38, 37, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert!(c.validations.is_empty());
    }

    #[test]
    fn rounding_flags_panic_on_representable() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = ExecutionCollector::new(vec!["2".into()], "1.4142".into());
        c.record(ExecutionResult::Panic);
        RoundingValidator { gen_precision: GP }.validate(
            &mut c, &g, 38, 4, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::Panic]);
    }

    #[test]
    fn precision_zero_when_exact() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = value_cell("1.4142");
        PrecisionValidator { gen_precision: GP }.validate(
            &mut c, &g, 38, 4, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert_eq!(c.validations, vec![Outcome::Precision { ulps: "0".to_string() }]);
    }

    #[test]
    fn overflow_expects_wrap() {
        // 2^127 wraps to i128::MIN in 128-bit storage (out of range at width 38, scale 0).
        let g = GoldenValue::parse("170141183460469231731687303715884105728").unwrap();
        let mut c = value_cell("-170141183460469231731687303715884105728");
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Wrap);
        assert_eq!(c.validations, vec![Outcome::Pass]);
    }

    #[test]
    fn overflow_silent_in_range() {
        let g = GoldenValue::parse("5").unwrap();
        let mut c = value_cell("5");
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Wrap);
        assert!(c.validations.is_empty());
    }

    #[test]
    fn overflow_panic_expected_passes() {
        let g = GoldenValue::parse("123").unwrap();
        let mut c = ExecutionCollector::new(vec!["x".into()], "x".into());
        c.record(ExecutionResult::Panic);
        OverflowValidator.validate(&mut c, &g, 2, 0, 128, RoundingMode::HalfToEven, Overflow::Panic);
        assert_eq!(c.validations, vec![Outcome::Pass]);
    }
}
