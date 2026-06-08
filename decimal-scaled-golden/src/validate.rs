//! Pluggable validation strategies. A `Validator` (the Tester impl) holds one of
//! each; the user composes them — `Default*` checks, `NoOp*` opts out. `active()`
//! lets the runner skip the format/eval pass entirely when nothing validates.

use crate::bigdec::abs_diff;
use crate::outcome::Outcome;
use crate::overflow::expected_overflow;
use crate::rounding::RoundingMode;
use crate::subject::{CaseOutput, Overflow};
use crate::validator::validate_one;
use crate::value::GoldenValue;

/// Does a fitting result round exactly as the spec'd mode requires?
pub trait ValidateRounding {
    fn validate_rounding(
        &self,
        got: &str,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        mode: RoundingMode,
    ) -> Outcome;
    /// Whether this checks anything (false => the runner may skip formatting).
    fn active(&self) -> bool {
        true
    }
}

/// Does an out-of-range result follow the declared overflow policy?
pub trait ValidateOverflow {
    fn validate_overflow(
        &self,
        got: &CaseOutput,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        storage_bits: u32,
        overflow: Overflow,
    ) -> Outcome;
    fn active(&self) -> bool {
        true
    }
}

/// How far is a fitting result from the true value (ULP magnitude)? `None` = not
/// measured.
pub trait ValidatePrecision {
    fn validate_precision(
        &self,
        got: &str,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        mode: RoundingMode,
    ) -> Option<String>;
    fn active(&self) -> bool {
        true
    }
}

// ── NoOp impls (opt out) ────────────────────────────────────────────────────

pub struct NoOpRounding;
impl ValidateRounding for NoOpRounding {
    fn validate_rounding(&self, _: &str, _: &GoldenValue, _: u32, _: u32, _: RoundingMode) -> Outcome {
        Outcome::Pass
    }
    fn active(&self) -> bool {
        false
    }
}

pub struct NoOpOverflow;
impl ValidateOverflow for NoOpOverflow {
    fn validate_overflow(
        &self, _: &CaseOutput, _: &GoldenValue, _: u32, _: u32, _: u32, _: Overflow,
    ) -> Outcome {
        Outcome::Pass
    }
    fn active(&self) -> bool {
        false
    }
}

pub struct NoOpPrecision;
impl ValidatePrecision for NoOpPrecision {
    fn validate_precision(&self, _: &str, _: &GoldenValue, _: u32, _: u32, _: RoundingMode) -> Option<String> {
        None
    }
    fn active(&self) -> bool {
        false
    }
}

// ── Default impls (the real checks) ─────────────────────────────────────────

/// Strict correct-rounding check (see [`validate_one`]). Carries the corpus
/// generation precision (the rounding sticky-bit threshold).
pub struct DefaultRounding {
    pub gen_precision: usize,
}
impl ValidateRounding for DefaultRounding {
    fn validate_rounding(
        &self, got: &str, golden: &GoldenValue, width: u32, scale: u32, mode: RoundingMode,
    ) -> Outcome {
        validate_one(got, golden, width, scale, mode, self.gen_precision)
    }
}

/// Validate an out-of-range result against the declared overflow policy.
pub struct DefaultOverflow;
impl ValidateOverflow for DefaultOverflow {
    fn validate_overflow(
        &self,
        got: &CaseOutput,
        golden: &GoldenValue,
        width: u32,
        scale: u32,
        storage_bits: u32,
        overflow: Overflow,
    ) -> Outcome {
        let expected = expected_overflow(golden, width, scale, storage_bits, overflow);
        match got {
            CaseOutput::Panic => {
                if overflow == Overflow::Panic { Outcome::Pass } else { Outcome::Panic }
            }
            CaseOutput::Skip => Outcome::Skipped,
            CaseOutput::Error(e) => Outcome::Error { reason: e.clone() },
            CaseOutput::Value(s) => match expected {
                None => Outcome::Error {
                    reason: "declared overflow=Panic but returned a value".to_string(),
                },
                Some(exp) => match GoldenValue::parse(s) {
                    Some(g) => {
                        let got_scaled = g.round_to(scale, RoundingMode::Trunc, false);
                        if got_scaled == exp {
                            Outcome::Pass
                        } else {
                            Outcome::MisRounded { delta: abs_diff(&got_scaled, &exp) }
                        }
                    }
                    None => Outcome::MisRounded { delta: "nan".to_string() },
                },
            },
        }
    }
}

/// ULP distance of a fitting result from the correctly-rounded true value.
pub struct DefaultPrecision {
    pub gen_precision: usize,
}
impl ValidatePrecision for DefaultPrecision {
    fn validate_precision(
        &self, got: &str, golden: &GoldenValue, width: u32, scale: u32, mode: RoundingMode,
    ) -> Option<String> {
        if !golden.fits(width, scale) {
            return None;
        }
        let truncated = golden.truncated_at(self.gen_precision);
        let exp = golden.round_to(scale, mode, truncated);
        let got_scaled = GoldenValue::parse(got)?.round_to(scale, RoundingMode::Trunc, false);
        Some(abs_diff(&got_scaled, &exp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GP: usize = 1233;

    #[test]
    fn default_rounding_passes_correct() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let r = DefaultRounding { gen_precision: GP };
        assert_eq!(r.validate_rounding("1.4142", &g, 38, 4, RoundingMode::HalfToEven), Outcome::Pass);
        assert!(r.active());
    }

    #[test]
    fn noop_rounding_always_pass_inactive() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let r = NoOpRounding;
        assert_eq!(r.validate_rounding("9.9", &g, 38, 4, RoundingMode::HalfToEven), Outcome::Pass);
        assert!(!r.active());
    }

    #[test]
    fn default_precision_zero_when_exact() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let p = DefaultPrecision { gen_precision: GP };
        assert_eq!(
            p.validate_precision("1.4142", &g, 38, 4, RoundingMode::HalfToEven),
            Some("0".to_string())
        );
    }

    #[test]
    fn noop_precision_none() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        assert_eq!(
            NoOpPrecision.validate_precision("1.4142", &g, 38, 4, RoundingMode::HalfToEven),
            None
        );
    }

    #[test]
    fn default_overflow_expects_wrap() {
        // 2^127 wraps to i128::MIN in 128-bit storage
        let g = GoldenValue::parse("170141183460469231731687303715884105728").unwrap();
        let o = DefaultOverflow;
        let got = CaseOutput::Value("-170141183460469231731687303715884105728".to_string());
        assert_eq!(o.validate_overflow(&got, &g, 38, 0, 128, Overflow::Wrap), Outcome::Pass);
    }

    #[test]
    fn default_overflow_expected_panic_passes_on_panic() {
        let g = GoldenValue::parse("170141183460469231731687303715884105728").unwrap();
        assert_eq!(
            DefaultOverflow.validate_overflow(&CaseOutput::Panic, &g, 38, 0, 128, Overflow::Panic),
            Outcome::Pass
        );
    }
}
