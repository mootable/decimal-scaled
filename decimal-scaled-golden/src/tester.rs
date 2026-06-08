//! The tester. For ONE subject it loads each function's golden cases (via the
//! configured `CaseLoader`), runs them through the configured `ExecutionStrategy`,
//! scores each cell with the configured `Validator`s, and returns that subject's
//! `SubjectCollector`. Two impls — `SeriesTester` (serial) and `ParallelTester`
//! (a work-queue over the subject's individual executions).
//!
//! `run` is generic over `Subject`, so different `Value` types need no erasure:
//! `strategy.execute(subject, ..)` is monomorphised where `S` is concrete. The
//! caller runs each subject and assembles the `TestCollector`; the collator sits
//! outside the tester.

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::caseloader::CaseLoader;
use crate::collector::{ExecutionCollector, FunctionCollector, SubjectCollector};
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::parser::GoldenCase;
use crate::subject::{Capabilities, FnSupport, Subject};
use crate::validator::Validator;
use crate::value::GoldenValue;

/// Runs one subject over a set of functions, producing its `SubjectCollector`.
/// Configured on construction with an `ExecutionStrategy`, a `CaseLoader`, and the
/// `Validator`s to apply.
pub trait Tester {
    fn run<S: Subject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector;
}

/// Runs the subject's executions serially.
pub struct SeriesTester<E: ExecutionStrategy> {
    pub strategy: E,
    pub loader: Box<dyn CaseLoader>,
    pub validators: Vec<Box<dyn Validator + Sync>>,
}

impl<E: ExecutionStrategy> Tester for SeriesTester<E> {
    fn run<S: Subject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector {
        let caps = subject.capabilities();
        let mut collector = SubjectCollector::new(caps.clone());
        for &function in functions {
            collector.add(match caps.function(function) {
                None => FunctionCollector::unsupported(function),
                Some(&support) => {
                    let mut fc = FunctionCollector::new(function, support);
                    for case in self.loader.load(function) {
                        fc.add(run_cell(
                            subject, &self.strategy, &self.validators, function, &caps, support, &case,
                        ));
                    }
                    fc
                }
            });
        }
        collector
    }
}

/// Distributes the subject's individual executions across `threads` workers (a
/// shared atomic work-index). The per-execution work is identical to serial.
pub struct ParallelTester<E: ExecutionStrategy + Sync> {
    pub threads: usize,
    pub strategy: E,
    pub loader: Box<dyn CaseLoader>,
    pub validators: Vec<Box<dyn Validator + Sync>>,
}

/// One function's place in the run: unsupported, or supported with its cases.
struct Plan {
    function: Function,
    support: Option<FnSupport>,
    cases: Vec<GoldenCase>,
}

impl<E: ExecutionStrategy + Sync> Tester for ParallelTester<E> {
    fn run<S: Subject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector {
        let caps = subject.capabilities();

        // Pre-pass (serial, cheap): per-function support + cases, and a flat job
        // list of every execution.
        let plans: Vec<Plan> = functions
            .iter()
            .map(|&function| match caps.function(function) {
                None => Plan { function, support: None, cases: Vec::new() },
                Some(&support) => {
                    Plan { function, support: Some(support), cases: self.loader.load(function) }
                }
            })
            .collect();
        let mut jobs: Vec<(usize, usize)> = Vec::new(); // (plan index, case index)
        for (pi, plan) in plans.iter().enumerate() {
            if plan.support.is_some() {
                for ci in 0..plan.cases.len() {
                    jobs.push((pi, ci));
                }
            }
        }

        let results = run_jobs(
            subject, &self.strategy, &self.validators, &caps, &plans, &jobs, self.threads,
        );

        // Assemble: results are in job order (function then case), drain them in.
        let mut results = results.into_iter();
        let mut collector = SubjectCollector::new(caps.clone());
        for plan in &plans {
            collector.add(match plan.support {
                None => FunctionCollector::unsupported(plan.function),
                Some(support) => {
                    let mut fc = FunctionCollector::new(plan.function, support);
                    for _ in 0..plan.cases.len() {
                        fc.add(results.next().expect("one result per job"));
                    }
                    fc
                }
            });
        }
        collector
    }
}

/// Run every job (execution), serially or across `threads` workers pulling from a
/// shared atomic index. Returns the results in job order.
fn run_jobs<S: Subject + Sync, E: ExecutionStrategy + Sync>(
    subject: &S,
    strategy: &E,
    validators: &[Box<dyn Validator + Sync>],
    caps: &Capabilities,
    plans: &[Plan],
    jobs: &[(usize, usize)],
    threads: usize,
) -> Vec<ExecutionCollector> {
    let run = |&(pi, ci): &(usize, usize)| {
        let plan = &plans[pi];
        run_cell(subject, strategy, validators, plan.function, caps, plan.support.unwrap(), &plan.cases[ci])
    };
    if threads <= 1 || jobs.len() <= 1 {
        return jobs.iter().map(run).collect();
    }
    let workers = threads.min(jobs.len());
    let next = AtomicUsize::new(0);
    let next_ref = &next;
    let nested: Vec<Vec<(usize, ExecutionCollector)>> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(move || {
                    let mut local: Vec<(usize, ExecutionCollector)> = Vec::new();
                    loop {
                        let k = next_ref.fetch_add(1, Ordering::Relaxed);
                        if k >= jobs.len() {
                            break;
                        }
                        local.push((k, run(&jobs[k])));
                    }
                    local
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut indexed: Vec<(usize, ExecutionCollector)> = nested.into_iter().flatten().collect();
    indexed.sort_by_key(|&(k, _)| k);
    indexed.into_iter().map(|(_, c)| c).collect()
}

/// Run + validate one golden case: build the cell, skip unrepresentable inputs,
/// execute via the strategy (which produces the `String` result), then let each
/// validator score the cell.
fn run_cell<S: Subject, E: ExecutionStrategy>(
    subject: &S,
    strategy: &E,
    validators: &[Box<dyn Validator + Sync>],
    function: Function,
    caps: &Capabilities,
    support: FnSupport,
    case: &GoldenCase,
) -> ExecutionCollector {
    let mut cell = ExecutionCollector::new(case.inputs.clone(), case.output_raw.clone());
    if !case.inputs.iter().all(|s| representable(s, caps.width, caps.scale)) {
        cell.mark_unsupported();
        return cell;
    }
    strategy.execute(subject, &case.inputs, function, support.mode, support.overflow, &mut cell);
    if let Some(golden) = GoldenValue::parse(&case.output_raw) {
        for v in validators {
            v.validate(
                &mut cell, &golden, caps.width, caps.scale, caps.storage_bits, support.mode,
                support.overflow,
            );
        }
    }
    cell
}

/// True if `input` is exactly representable at `(width, scale)` — fractional
/// digits ≤ scale and integer digits ≤ width − scale.
fn representable(input: &str, width: u32, scale: u32) -> bool {
    match GoldenValue::parse(input) {
        Some(gv) => gv.frac_digits.len() <= scale as usize && gv.fits(width, scale),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::{CellStatus, ExecutionResult};
    use crate::execution::RunOnce;
    use crate::outcome::Outcome;
    use crate::rounding::RoundingMode;
    use crate::subject::{FnSupport, Overflow};
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
            Capabilities { name: "sqrt64".into(), width: 38, scale: 4, storage_bits: 128, functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse::<f64>().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> f64 {
            |inputs| inputs[0].sqrt()
        }
    }

    /// Toy validator: Pass when the actual matches the expected string.
    struct EqValidator;
    impl Validator for EqValidator {
        fn validate(
            &self, cell: &mut ExecutionCollector, _g: &GoldenValue, _w: u32, _s: u32, _b: u32,
            _m: RoundingMode, _o: Overflow,
        ) {
            let ok = cell.value() == Some(cell.expected.as_str());
            cell.add_validation(if ok { Outcome::Pass } else { Outcome::MisRounded { delta: "?".into() } });
        }
    }

    struct FixedLoader;
    impl CaseLoader for FixedLoader {
        fn load(&self, _f: Function) -> Vec<GoldenCase> {
            vec![GoldenCase { inputs: vec!["2".into()], output_raw: "1.4142".into() }]
        }
    }

    fn series() -> SeriesTester<RunOnce> {
        SeriesTester {
            strategy: RunOnce,
            loader: Box::new(FixedLoader),
            validators: vec![Box::new(EqValidator)],
        }
    }

    #[test]
    fn series_runs_a_cell() {
        let sc = series().run(&Sqrt64, &[Function::Sqrt]);
        let fc = &sc.functions[0];
        assert!(fc.supported());
        assert_eq!(fc.cells.len(), 1);
        assert_eq!(fc.cells[0].value(), Some("1.4142"));
        assert_eq!(fc.cells[0].validations, vec![Outcome::Pass]);
    }

    #[test]
    fn unsupported_function_recorded() {
        let sc = series().run(&Sqrt64, &[Function::Exp]);
        assert!(!sc.functions[0].supported());
        assert!(sc.functions[0].cells.is_empty());
    }

    #[test]
    fn too_precise_input_unsupported() {
        struct PreciseLoader;
        impl CaseLoader for PreciseLoader {
            fn load(&self, _f: Function) -> Vec<GoldenCase> {
                vec![GoldenCase { inputs: vec!["1.23456".into()], output_raw: "1.1111".into() }]
            }
        }
        let tester = SeriesTester {
            strategy: RunOnce,
            loader: Box::new(PreciseLoader),
            validators: vec![Box::new(EqValidator)],
        };
        let sc = tester.run(&Sqrt64, &[Function::Sqrt]);
        assert_eq!(sc.functions[0].cells[0].status, CellStatus::Unsupported);
    }

    #[test]
    fn parallel_matches_series() {
        // many cells -> the work-queue distributes executions across workers
        struct ManyLoader;
        impl CaseLoader for ManyLoader {
            fn load(&self, _f: Function) -> Vec<GoldenCase> {
                (1..=20)
                    .map(|n| GoldenCase {
                        inputs: vec![n.to_string()],
                        output_raw: format!("{:.4}", (n as f64).sqrt()),
                    })
                    .collect()
            }
        }
        let par = ParallelTester {
            threads: 4,
            strategy: RunOnce,
            loader: Box::new(ManyLoader),
            validators: vec![Box::new(EqValidator)],
        };
        let ser = SeriesTester {
            strategy: RunOnce,
            loader: Box::new(ManyLoader),
            validators: vec![Box::new(EqValidator)],
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
