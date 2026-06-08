//! Execution strategies — HOW one input-set is run: once, or repeated for
//! timing. Generic over `Subject`, so the strategy itself is the typed→string
//! boundary (it parses, computes, catches panics, and formats `Value`→`String`),
//! pushing an `ExecutionResult` (and timing) into a `ExecutionCollector`. `Value`
//! never escapes. The runner loops the cell's cases and pre-filters
//! unrepresentable inputs, so there's no "skip" here.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

use crate::collector::{ExecutionCollector, ExecutionResult};
use crate::function::Function;
use crate::rounding::RoundingMode;
use crate::subject::{Overflow, Subject};

/// How one input-set's compute is run.
pub trait ExecutionStrategy {
    fn execute<S: Subject>(
        &self,
        subject: &S,
        input: &[String], // one input-set: one or two operands
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
        sink: &mut ExecutionCollector,
    );
}

/// Run the op once. No timing.
pub struct RunOnce;

impl ExecutionStrategy for RunOnce {
    fn execute<S: Subject>(
        &self,
        subject: &S,
        input: &[String],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
        sink: &mut ExecutionCollector,
    ) {
        let op = subject.execute(function, mode, overflow);
        sink.record(match parse(subject, input) {
            Some(vals) => value(subject, &op, &vals),
            None => ExecutionResult::Panic,
        });
    }
}

/// Run the op `number_of_executions` times and report the average time; the
/// stringified value comes from the first run. Crude — TODO: wire a real bench
/// library. Parsing is hoisted out (untimed) and the timed loop is compute only.
pub struct Timed {
    pub number_of_executions: u32,
}

impl ExecutionStrategy for Timed {
    fn execute<S: Subject>(
        &self,
        subject: &S,
        input: &[String],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
        sink: &mut ExecutionCollector,
    ) {
        let op = subject.execute(function, mode, overflow);
        let vals = match parse(subject, input) {
            Some(vals) => vals,
            None => {
                sink.record(ExecutionResult::Panic);
                return;
            }
        };
        // The first value. If it panicked, there's nothing worth timing — record
        // and stop.
        match value(subject, &op, &vals) {
            ExecutionResult::Panic => {
                sink.record(ExecutionResult::Panic);
                return;
            }
            ok => sink.record(ok),
        }
        let n = self.number_of_executions.max(1);
        let timed = catch_unwind(AssertUnwindSafe(|| {
            let start = Instant::now();
            for _ in 0..n {
                std::hint::black_box(op(&vals));
            }
            start.elapsed().as_nanos() as u64 / n as u64
        }));
        if let Ok(avg) = timed {
            sink.record_timing(avg);
        }
    }
}

/// Parse an input-set to values (untimed), or `None` if parsing panicked.
fn parse<S: Subject>(subject: &S, input: &[String]) -> Option<Vec<S::Value>> {
    catch_unwind(AssertUnwindSafe(|| {
        input.iter().map(|s| subject.string_to_value(s)).collect::<Vec<S::Value>>()
    }))
    .ok()
}

/// Compute + format one result, catching a panic into [`ExecutionResult::Panic`].
fn value<S: Subject>(
    subject: &S,
    op: &impl Fn(&[S::Value]) -> S::Value,
    vals: &[S::Value],
) -> ExecutionResult {
    catch_unwind(AssertUnwindSafe(|| subject.value_to_string(&op(vals))))
        .map(ExecutionResult::Value)
        .unwrap_or(ExecutionResult::Panic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subject::{Capabilities, FnSupport};
    use std::collections::BTreeMap;

    struct Sqrt64;
    impl Subject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            Capabilities { name: "sqrt64".into(), width: 38, scale: 15, functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse::<f64>().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn representable(&self, _value: &crate::value::GoldenValue) -> bool {
            true
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> f64 {
            |inputs| inputs[0].sqrt()
        }
    }

    fn one(s: &str) -> Vec<String> {
        vec![s.to_string()]
    }

    #[test]
    fn run_once_result_no_timing() {
        let mut c = ExecutionCollector::new(one("2"), "1.4142".into());
        RunOnce.execute(&Sqrt64, &one("2"), Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic, &mut c);
        assert_eq!(c.execution_result(), Some(&ExecutionResult::Value("1.4142".into())));
        assert_eq!(c.timing, None);
    }

    #[test]
    fn run_once_catches_panic() {
        let mut c = ExecutionCollector::new(one("bad"), "x".into());
        RunOnce.execute(&Sqrt64, &one("bad"), Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic, &mut c);
        assert_eq!(c.execution_result(), Some(&ExecutionResult::Panic));
    }

    #[test]
    fn timed_first_value_and_average() {
        let mut c = ExecutionCollector::new(one("2"), "1.4142".into());
        Timed { number_of_executions: 32 }.execute(&Sqrt64, &one("2"), Function::Sqrt,
            RoundingMode::HalfToEven, Overflow::Panic, &mut c);
        assert_eq!(c.execution_result(), Some(&ExecutionResult::Value("1.4142".into())));
        assert!(c.timing.is_some());
    }
}
