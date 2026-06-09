//! `PrecisionValidator` — informational ULP distance of an in-range value from the
//! correctly-rounded true value.

use crate::string_decimal::abs_diff;
use crate::outcome::Outcome;

use super::validator::{to_scaled_int, ValidationContext, Validator};

/// `gen_precision` = the golden set's full stored precision.
pub struct PrecisionValidator {
    pub gen_precision: usize,
}

impl Validator for PrecisionValidator {
    fn validate(&self, ctx: &ValidationContext) -> Option<Outcome> {
        if !ctx.in_range() {
            return None;
        }
        let got = ctx.value()?;
        let mode = ctx.mode()?;
        let grade = ctx.grade_precision();
        let got_scaled = to_scaled_int(got, grade)?;
        let truncated = ctx.golden_value.truncated_at(self.gen_precision);
        let expected = ctx.golden_value.round_to(grade, mode, truncated);
        Some(Outcome::Precision { ulps: abs_diff(&got_scaled, &expected) })
    }
}
