//! The runner: drives a subject over the golden cases. `GoldenRunner` is the trait;
//! `SeriesRunner` / `ParallelRunner` differ only in scheduling. The shared per-cell
//! work (`run_cell`, the input filter) lives here.

mod parallel;
mod runner;
mod series;

pub use parallel::ParallelRunner;
pub use runner::GoldenRunner;
pub use series::SeriesRunner;

use crate::string_decimal::within;
use crate::collector::ExecutionCollector;
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::loader::{GoldenCase, GoldenValue};
use crate::subject::{Capabilities, DecimalSubject, FnSupport, Limits};
use crate::validators::{ValidationContext, Validator};

/// Run + classify + validate one golden case: build the cell, skip unrepresentable
/// inputs, execute via the strategy, then (if there are validators and the golden
/// parses) build a `ValidationContext` and collect each validator's verdict.
#[allow(clippy::too_many_arguments)]
fn run_cell<S: DecimalSubject, E: ExecutionStrategy>(
    subject: &S,
    strategy: &E,
    validators: &[Box<dyn Validator + Sync>],
    function: Function,
    caps: &Capabilities,
    support: FnSupport,
    case: &GoldenCase,
    oracle: &Limits,
) -> ExecutionCollector {
    let mut cell = ExecutionCollector::new(case.inputs.clone(), case.output_raw.clone(), case.line);

    // Input filter: every input must be exactly representable by the subject.
    if !case.inputs.iter().all(|s| input_representable(subject, s)) {
        cell.mark_skipped();
        return cell;
    }

    strategy.execute(subject, &case.inputs, function, support.mode, support.overflow, &mut cell);

    if !validators.is_empty() {
        if let Some(golden) = GoldenValue::parse(&case.output_raw) {
            let limits = subject.limits(&case.output_raw);
            cell.oracle_limited = limits.max_precision > oracle.max_precision;
            // Collect verdicts while the context borrows the cell's result, then
            // release the borrow before pushing them back into the cell.
            let outcomes: Vec<_> = {
                let ctx = ValidationContext {
                    function,
                    result: cell.result().expect("the cell ran"),
                    golden_value: &golden,
                    limits: &limits,
                    oracle_limits: oracle,
                    capabilities: caps,
                };
                validators.iter().filter_map(|v| v.validate(&ctx)).collect()
            };
            for o in outcomes {
                cell.add_validation(o);
            }
        }
    }
    cell
}

/// True if `input` is *exactly* representable by the subject: its significant
/// fraction digits fit the subject's depth at that value, and its magnitude fits
/// the envelope. This is the INPUT skip.
fn input_representable<S: DecimalSubject>(subject: &S, input: &str) -> bool {
    let lim = subject.limits(input);
    significant_frac_digits(input) <= lim.max_precision as usize
        && within(input, lim.min_value.as_deref(), lim.max_value.as_deref())
}

/// Count of fraction digits up to the last non-zero one (trailing zeros ignored):
/// the depth at which the value is exactly representable. `1.00` → 0, `1.50` → 1.
fn significant_frac_digits(s: &str) -> usize {
    s.split_once('.').map(|(_, f)| f.trim_end_matches('0').len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::{CellStatus, ExecutionResult};
    use crate::execution::RunOnce;
    use crate::outcome::Outcome;
    use crate::rounding::RoundingMode;
    use crate::subject::{Computed, Overflow, Radix};
    use crate::validators::RoundingValidator;
    use crate::CaseLoader;
    use std::borrow::Cow;
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

    struct FixedLoader;
    impl CaseLoader for FixedLoader {
        fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
            Cow::Owned(vec![GoldenCase { inputs: vec!["2".into()], output_raw: "1.4142135".into(), line: 0 }])
        }
        fn oracle_limits(&self) -> Limits {
            Limits { min_value: None, max_value: None, max_precision: 1231 }
        }
    }

    #[test]
    fn series_runs_a_cell_and_passes() {
        let runner = SeriesRunner {
            strategy: RunOnce,
            loader: Box::new(FixedLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let sc = runner.run(&Sqrt64, &[Function::Sqrt]);
        let fc = &sc.functions[0];
        assert!(fc.supported());
        assert_eq!(fc.cells.len(), 1);
        assert_eq!(fc.cells[0].value(), Some("1.4142"));
        assert_eq!(fc.cells[0].validations, vec![Outcome::Pass]);
    }

    #[test]
    fn too_precise_input_skipped() {
        struct PreciseLoader;
        impl CaseLoader for PreciseLoader {
            fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
                Cow::Owned(vec![GoldenCase { inputs: vec!["1.234567".into()], output_raw: "1.1111".into(), line: 0 }])
            }
            fn oracle_limits(&self) -> Limits {
                Limits { min_value: None, max_value: None, max_precision: 1231 }
            }
        }
        let runner = SeriesRunner {
            strategy: RunOnce,
            loader: Box::new(PreciseLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let sc = runner.run(&Sqrt64, &[Function::Sqrt]);
        assert_eq!(sc.functions[0].cells[0].status, CellStatus::Done(ExecutionResult::Skipped));
    }

    #[test]
    fn parallel_matches_series() {
        struct ManyLoader;
        impl CaseLoader for ManyLoader {
            fn load(&self, _f: Function) -> Cow<'_, [GoldenCase]> {
                Cow::Owned(
                    (1..=20)
                        .map(|n| GoldenCase {
                            inputs: vec![n.to_string()],
                            output_raw: format!("{:.7}", (n as f64).sqrt()),
                            line: n,
                        })
                        .collect(),
                )
            }
            fn oracle_limits(&self) -> Limits {
                Limits { min_value: None, max_value: None, max_precision: 1231 }
            }
        }
        let par = ParallelRunner {
            threads: 4,
            strategy: RunOnce,
            loader: Box::new(ManyLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let ser = SeriesRunner {
            strategy: RunOnce,
            loader: Box::new(ManyLoader),
            validators: vec![Box::new(RoundingValidator { gen_precision: 1233 })],
        };
        let pc = par.run(&Sqrt64, &[Function::Sqrt]);
        let sc = ser.run(&Sqrt64, &[Function::Sqrt]);
        let (pcells, scells) = (&pc.functions[0].cells, &sc.functions[0].cells);
        assert_eq!(pcells.len(), scells.len());
        for (p, s) in pcells.iter().zip(scells) {
            assert_eq!(p.value(), s.value());
            assert_eq!(p.validations, s.validations);
        }
    }
}
