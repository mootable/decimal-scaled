//! Execution strategies — HOW one input-set is run: once, or repeated for timing.
//! Generic over `DecimalSubject`, so the strategy is the typed→string boundary
//! (`Value` never escapes it).

mod run_once;
mod strategy;
mod timed;

pub use run_once::RunOnce;
pub use strategy::ExecutionStrategy;
pub use timed::Timed;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::ExecutionCollector;
    use crate::function::Function;
    use crate::rounding::RoundingMode;
    use crate::subject::{Capabilities, Computed, DecimalSubject, FnSupport, Limits, Overflow, Radix};
    use std::collections::BTreeMap;

    struct Sqrt64;
    impl DecimalSubject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            Capabilities { name: "sqrt64".into(), radix: Radix::Decimal, config: BTreeMap::new(), functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse::<f64>().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn limits(&self, _value: &str) -> Limits {
            Limits { min_value: None, max_value: None, max_precision: 4 }
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> Computed<f64> {
            |inputs| Computed::Value(inputs[0].sqrt())
        }
    }

    fn one(s: &str) -> Vec<String> {
        vec![s.to_string()]
    }

    #[test]
    fn run_once_records_value() {
        let mut c = ExecutionCollector::new(one("2"), "1.4142".into(), 0);
        RunOnce.execute(&Sqrt64, &one("2"), Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic, &mut c);
        assert_eq!(c.value(), Some("1.4142"));
        assert_eq!(c.timing, None);
    }

    #[test]
    fn run_once_catches_panic() {
        let mut c = ExecutionCollector::new(one("bad"), "x".into(), 0);
        RunOnce.execute(&Sqrt64, &one("bad"), Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic, &mut c);
        assert!(matches!(c.computed(), Some(Computed::Panic(_))));
    }

    #[test]
    fn timed_first_value_and_average() {
        let mut c = ExecutionCollector::new(one("2"), "1.4142".into(), 0);
        Timed { number_of_executions: 32 }.execute(
            &Sqrt64, &one("2"), Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic, &mut c,
        );
        assert_eq!(c.value(), Some("1.4142"));
        assert!(c.timing.is_some());
    }
}
