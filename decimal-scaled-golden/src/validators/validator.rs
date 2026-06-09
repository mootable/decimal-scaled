//! The pure `Validator` trait + the `ValidationContext` it judges against.

use crate::string_decimal::within;
use crate::collector::ExecutionResult;
use crate::function::Function;
use crate::loader::GoldenValue;
use crate::outcome::Outcome;
use crate::rounding::RoundingMode;
use crate::subject::{Capabilities, Computed, Limits, Overflow};

/// Everything a validator judges against, computed once per cell by the runner.
/// Stores only the irreducible inputs; `grade_precision` / `in_range` are derived.
pub struct ValidationContext<'a> {
    pub function: Function,
    /// The runner's record (§5.2): a `Computed` test outcome (incl. its `Panic`).
    pub result: &'a ExecutionResult,
    /// The oracle's value, parsed.
    pub golden_value: &'a GoldenValue,
    /// The LIBRARY's envelope at the golden value.
    pub limits: &'a Limits,
    /// The ORACLE's envelope — `{None, None, gen_precision − guard}`.
    pub oracle_limits: &'a Limits,
    /// The subject's capabilities (mode/overflow via `.function`, radix).
    pub capabilities: &'a Capabilities,
}

impl ValidationContext<'_> {
    /// The depth to grade at: the shallower of the library's reach and the oracle's.
    pub fn grade_precision(&self) -> u32 {
        self.limits.max_precision.min(self.oracle_limits.max_precision)
    }

    /// Does the true result fit the library's envelope (magnitude after rounding)?
    pub fn in_range(&self) -> bool {
        let v = self.golden_value.to_decimal_string_at_scale(self.grade_precision());
        within(&v, self.limits.min_value.as_deref(), self.limits.max_value.as_deref())
    }

    /// The function's tested rounding mode, if the subject supports the function.
    pub fn mode(&self) -> Option<RoundingMode> {
        self.capabilities.function(self.function).map(|s| s.mode)
    }

    /// The function's declared overflow policy, if supported.
    pub fn overflow(&self) -> Option<Overflow> {
        self.capabilities.function(self.function).map(|s| s.overflow)
    }

    /// The produced finite value string, if the result is `Computed::Value`.
    pub fn value(&self) -> Option<&str> {
        match self.result {
            ExecutionResult::Computed(Computed::Value(s)) => Some(s),
            _ => None,
        }
    }

    /// The test outcome, if the cell ran (else `None` — harness-error / skipped).
    pub fn computed(&self) -> Option<&Computed<String>> {
        match self.result {
            ExecutionResult::Computed(c) => Some(c),
            _ => None,
        }
    }
}

/// Scores a cell, pushing at most one verdict. `None` = abstain (out of this
/// validator's domain).
pub trait Validator {
    fn validate(&self, ctx: &ValidationContext) -> Option<Outcome>;
}

/// Normalize a subject's output string to a signed scaled-integer at `scale`. `None`
/// if unparseable (e.g. the subject emitted a non-decimal).
pub(super) fn to_scaled_int(got: &str, scale: u32) -> Option<String> {
    let gv = GoldenValue::parse(got)?;
    Some(gv.round_to(scale, RoundingMode::Trunc, false))
}
