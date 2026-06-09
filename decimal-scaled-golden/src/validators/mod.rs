//! Validation. One pure `Validator` trait — context in, `Option<Outcome>` out; the
//! runner collects the verdicts. Validators only ever see the subject's output as an
//! erased `Computed<String>`; `Value` never reaches them. The three have DISJOINT
//! domains and self-gate.

mod overflow;
mod precision;
mod rounding;
mod validator;

pub use overflow::OverflowValidator;
pub use precision::PrecisionValidator;
pub use rounding::RoundingValidator;
pub use validator::{ValidationContext, Validator};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::ExecutionResult;
    use crate::function::Function;
    use crate::outcome::Outcome;
    use crate::rounding::RoundingMode;
    use crate::subject::{Capabilities, Computed, FnSupport, Limits, Overflow, Radix};
    use crate::loader::GoldenValue;
    use std::collections::BTreeMap;

    const GP: usize = 1233;

    fn caps(mode: RoundingMode, overflow: Overflow) -> Capabilities {
        let mut functions = BTreeMap::new();
        functions.insert(Function::Sqrt, FnSupport { mode, overflow });
        Capabilities { name: "t".into(), radix: Radix::Decimal, config: BTreeMap::new(), functions }
    }

    fn ctx<'a>(
        result: &'a ExecutionResult,
        golden: &'a GoldenValue,
        limits: &'a Limits,
        oracle: &'a Limits,
        caps: &'a Capabilities,
    ) -> ValidationContext<'a> {
        ValidationContext {
            function: Function::Sqrt,
            result,
            golden_value: golden,
            limits,
            oracle_limits: oracle,
            capabilities: caps,
        }
    }

    fn value(s: &str) -> ExecutionResult {
        ExecutionResult::Computed(Computed::Value(s.to_string()))
    }

    #[test]
    fn rounding_passes_correct() {
        let g = GoldenValue::parse("1.4142135").unwrap();
        let lim = Limits { min_value: None, max_value: None, max_precision: 4 };
        let oracle = Limits { min_value: None, max_value: None, max_precision: GP as u32 };
        let c = caps(RoundingMode::HalfToEven, Overflow::Panic);
        let r = value("1.4142");
        let cx = ctx(&r, &g, &lim, &oracle, &c);
        assert_eq!(RoundingValidator { gen_precision: GP }.validate(&cx), Some(Outcome::Pass));
    }

    #[test]
    fn overflow_panic_expected_when_out_of_range() {
        let g = GoldenValue::parse("9").unwrap();
        let lim = Limits { min_value: Some("-5".into()), max_value: Some("5".into()), max_precision: 0 };
        let oracle = Limits { min_value: None, max_value: None, max_precision: GP as u32 };
        let c = caps(RoundingMode::HalfToEven, Overflow::Panic);
        let r = ExecutionResult::Computed(Computed::Panic("overflow".into()));
        let cx = ctx(&r, &g, &lim, &oracle, &c);
        assert_eq!(OverflowValidator.validate(&cx), Some(Outcome::Pass));
    }

    #[test]
    fn overflow_in_range_panic_is_bug() {
        let g = GoldenValue::parse("3").unwrap();
        let lim = Limits { min_value: Some("-5".into()), max_value: Some("5".into()), max_precision: 0 };
        let oracle = Limits { min_value: None, max_value: None, max_precision: GP as u32 };
        let c = caps(RoundingMode::HalfToEven, Overflow::Panic);
        let r = ExecutionResult::Computed(Computed::Panic("oops".into()));
        let cx = ctx(&r, &g, &lim, &oracle, &c);
        assert_eq!(OverflowValidator.validate(&cx), Some(Outcome::Panic));
    }
}
