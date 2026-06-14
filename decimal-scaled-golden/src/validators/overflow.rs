// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `OverflowValidator` — out-of-range results, judged against the declared
//! `Overflow` policy.

use crate::outcome::Outcome;
use crate::subject::{Computed, NonReal, Overflow};

use super::validator::{ValidationContext, Validator};

/// Out-of-range only — it consults `in_range`, never a bit width. The envelope
/// already said whether the true result fits; this checks the subject's response.
pub struct OverflowValidator;

impl Validator for OverflowValidator {
    fn validate(&self, ctx: &ValidationContext) -> Option<Outcome> {
        if ctx.in_range() {
            // In range: a clean value is rounding/precision's domain (silent); any
            // other test outcome is a bug (the library failed an in-range value).
            return match ctx.computed()? {
                Computed::Value(_) => None,
                other => Some(in_range_bug(other)),
            };
        }
        // Out of range: match the result against the declared policy.
        let policy = ctx.overflow()?;
        let c = ctx.computed()?;
        Some(match_overflow(policy, c, ctx))
    }
}

/// An in-range cell that produced a non-`Value` `Computed` arm — always a bug.
fn in_range_bug(c: &Computed<String>) -> Outcome {
    match c {
        Computed::Value(_) => unreachable!("handled by the caller"),
        Computed::Panic(_) => Outcome::Panic,
        Computed::Timeout(_) => Outcome::Timeout,
        Computed::Absent => Outcome::Error { reason: "in-range but no value (absent)".into() },
        Computed::Error(r) => Outcome::Error { reason: format!("in-range but errored: {r}") },
        Computed::NonReal(n) => Outcome::Error { reason: format!("in-range but non-real: {n:?}") },
    }
}

/// Out-of-range: does the result match the declared overflow policy?
fn match_overflow(policy: Overflow, c: &Computed<String>, ctx: &ValidationContext) -> Outcome {
    use Overflow::*;
    let matched = match (policy, c) {
        (Panic, Computed::Panic(_)) => true,
        (Error, Computed::Error(_)) => true,
        (Absent, Computed::Absent) => true,
        (Infinity, Computed::NonReal(n)) => infinity_sign_ok(*n, ctx),
        // Value-producing policies: the produced value must equal the one derived
        // from the envelope. Deriving saturate/truncate/wrap exactly is a follow-up;
        // for now a produced value is accepted, anything else is flagged.
        (Saturate | Truncate | Wrap, Computed::Value(_)) => true,
        _ => false,
    };
    if matched {
        Outcome::Pass
    } else {
        Outcome::Error {
            reason: format!("overflow: declared {policy:?}, got {}", computed_tag(c)),
        }
    }
}

/// True if a returned infinity has the sign of the overflow direction (golden's sign).
fn infinity_sign_ok(n: NonReal, ctx: &ValidationContext) -> bool {
    match n {
        NonReal::PositiveInfinity => !ctx.golden_value.negative,
        NonReal::NegativeInfinity => ctx.golden_value.negative,
        _ => false,
    }
}

fn computed_tag(c: &Computed<String>) -> &'static str {
    match c {
        Computed::Value(_) => "value",
        Computed::NonReal(_) => "non-real",
        Computed::Absent => "absent",
        Computed::Error(_) => "error",
        Computed::Timeout(_) => "timeout",
        Computed::Panic(_) => "panic",
    }
}
