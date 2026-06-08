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
use crate::overflow::{expected_overflow, overflows_storage};
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
        _width: u32,
        scale: u32,
        _storage_bits: u32,
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

/// "How does it overflow." The ONE validator that consults the result's range
/// (via [`overflows_storage`], the storage range — the library's real `MAX`).
/// It also sees whether the subject PANICKED, which for an overflow may be the
/// expected outcome (the default strict contract panics on overflow). It judges:
/// an in-range value is silent (rounding/precision's domain); an in-range panic
/// is an unexpected bug; an overflowing result must match the declared policy —
/// a `Panic` policy expects the panic, a `Wrap`/`Saturate`/`Truncate` policy
/// expects the corresponding value.
pub struct OverflowValidator;

impl Validator for OverflowValidator {
    fn validate(
        &self,
        cell: &mut ExecutionCollector,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        storage_bits: u32,
        mode: RoundingMode,
        overflow: Overflow,
    ) {
        let would_overflow = overflows_storage(golden, scale, storage_bits, mode);
        // Read the execution result into an owned form first (so we can mutate
        // the cell below without holding a borrow of it).
        let result = cell.execution_result().cloned();
        let outcome = match result {
            Some(ExecutionResult::Panic) => {
                if !would_overflow {
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
            Some(ExecutionResult::Value(s)) => {
                if !would_overflow {
                    return; // in range — rounding/precision judge the value
                }
                match expected_overflow(golden, width, scale, storage_bits, mode, overflow) {
                    // Policy = Panic, but it returned a value instead of panicking.
                    None => Outcome::Error {
                        reason: "declared overflow=Panic but returned a value".to_string(),
                    },
                    Some(exp) => match to_scaled_int(&s, scale) {
                        Some(got_scaled) if got_scaled == exp => Outcome::Pass,
                        Some(got_scaled) => Outcome::MisRounded { delta: abs_diff(&got_scaled, &exp) },
                        None => Outcome::MisRounded { delta: "nan".to_string() },
                    },
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
        _width: u32,
        scale: u32,
        _storage_bits: u32,
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
            &mut c, &g, 38, 4, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert!(c.validations.is_empty());
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
    fn precision_silent_on_panic() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let mut c = panic_cell();
        PrecisionValidator { gen_precision: GP }.validate(
            &mut c, &g, 38, 4, 128, RoundingMode::HalfToEven, Overflow::Panic,
        );
        assert!(c.validations.is_empty());
    }

    #[test]
    fn overflow_expects_wrap() {
        // 2^127 overflows i128 (out of range at storage_bits 128) and wraps to
        // i128::MIN = -2^127 under a Wrap policy.
        let g = GoldenValue::parse("170141183460469231731687303715884105728").unwrap();
        let mut c = value_cell("-170141183460469231731687303715884105728");
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Wrap);
        assert_eq!(c.validations, vec![Outcome::Pass]);
    }

    #[test]
    fn overflow_silent_in_range_value() {
        // 5 fits the storage — overflow is not its concern (rounding/precision are).
        let g = GoldenValue::parse("5").unwrap();
        let mut c = value_cell("5");
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Wrap);
        assert!(c.validations.is_empty());
    }

    #[test]
    fn overflow_panic_is_expected_outcome_of_overflow() {
        // 2^127 overflows i128; under a Panic policy, the panic IS correct.
        let g = GoldenValue::parse("170141183460469231731687303715884105728").unwrap();
        let mut c = panic_cell();
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Panic);
        assert_eq!(c.validations, vec![Outcome::Pass]);
    }

    #[test]
    fn overflow_flags_in_range_panic_as_bug() {
        // 123 fits i128 — a panic on a representable result is an unexpected bug.
        let g = GoldenValue::parse("123").unwrap();
        let mut c = panic_cell();
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Panic);
        assert_eq!(c.validations, vec![Outcome::Panic]);
    }

    #[test]
    fn overflow_value_under_panic_policy_is_error() {
        // Overflowing result, Panic policy, yet a value came back — it should have panicked.
        let g = GoldenValue::parse("170141183460469231731687303715884105728").unwrap();
        let mut c = value_cell("5");
        OverflowValidator.validate(&mut c, &g, 38, 0, 128, RoundingMode::HalfToEven, Overflow::Panic);
        assert!(matches!(c.validations.as_slice(), [Outcome::Error { .. }]));
    }
}
