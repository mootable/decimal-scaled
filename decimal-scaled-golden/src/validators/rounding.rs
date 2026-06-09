//! `RoundingValidator` — decimal compliance (the verdict). In-range values only:
//! is the produced value the correctly-rounded golden under the declared mode?

use crate::string_decimal::abs_diff;
use crate::outcome::Outcome;
use crate::rounding::RoundingMode;

use super::validator::{to_scaled_int, ValidationContext, Validator};

/// `gen_precision` = the golden set's full stored precision (the rounding
/// sticky-bit threshold).
pub struct RoundingValidator {
    pub gen_precision: usize,
}

impl Validator for RoundingValidator {
    fn validate(&self, ctx: &ValidationContext) -> Option<Outcome> {
        if !ctx.in_range() {
            return None; // out of range — overflow's domain
        }
        let got = ctx.value()?; // no produced value ⇒ silent here
        let mode = ctx.mode()?;
        let grade = ctx.grade_precision();
        let truncated = ctx.golden_value.truncated_at(self.gen_precision);
        let got_scaled = match to_scaled_int(got, grade) {
            Some(s) => s,
            None => return Some(Outcome::MisRounded { delta: "nan".to_string() }),
        };
        let expected = ctx.golden_value.round_to(grade, mode, truncated);
        if got_scaled == expected {
            return Some(Outcome::Pass);
        }
        for m in RoundingMode::ALL {
            if m == mode {
                continue;
            }
            if got_scaled == ctx.golden_value.round_to(grade, m, truncated) {
                return Some(Outcome::WrongMode { used: m });
            }
        }
        Some(Outcome::MisRounded { delta: abs_diff(&got_scaled, &expected) })
    }
}
