//! `ParallelRunner` — runs a subject one function at a time, distributing that
//! function's cases across `threads` named workers pulling from a shared atomic
//! work-index. Per-execution work is identical to serial; processing one function
//! per batch lets each worker carry the `(subject, function)` it is draining in its
//! thread name, so a caught panic names the culprit.

use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::collector::{ExecutionCollector, FunctionCollector, SubjectCollector};
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::loader::CaseLoader;
use crate::subject::{Capabilities, DecimalSubject, FnSupport, Limits};
use crate::validators::Validator;

use super::run_cell;
use super::runner::GoldenRunner;

/// Runs a subject one function at a time, distributing that function's individual
/// executions across `threads` workers.
pub struct ParallelRunner<E: ExecutionStrategy + Sync> {
    pub threads: usize,
    pub strategy: E,
    pub loader: Box<dyn CaseLoader>,
    pub validators: Vec<Box<dyn Validator + Sync>>,
}

impl<E: ExecutionStrategy + Sync> GoldenRunner for ParallelRunner<E> {
    fn run<S: DecimalSubject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector {
        let caps = subject.capabilities();
        let oracle = self.loader.oracle_limits();
        let label = subject.name();

        let mut collector = SubjectCollector::new(caps.clone());
        for &function in functions {
            collector.add(match caps.function(function) {
                None => FunctionCollector::unsupported(function),
                Some(&support) => {
                    let cases = self.loader.load(function).into_owned();
                    let cells = run_function(
                        subject, &self.strategy, &self.validators, &caps, &oracle, function,
                        support, &cases, self.threads, &label,
                    );
                    let mut fc = FunctionCollector::new(function, support);
                    for cell in cells {
                        fc.add(cell);
                    }
                    fc
                }
            });
        }
        collector
    }
}

/// Run one function's cases — serially, or across `threads` workers pulling from a
/// shared atomic index — returning the results in case order. Each worker is named
/// `gold:w<N>:<function>:<subject>` so a caught panic prints which `(subject,
/// function)` drain hit it (the offending input + its golden line come from the cell).
#[allow(clippy::too_many_arguments)]
fn run_function<S: DecimalSubject + Sync, E: ExecutionStrategy + Sync>(
    subject: &S,
    strategy: &E,
    validators: &[Box<dyn Validator + Sync>],
    caps: &Capabilities,
    oracle: &Limits,
    function: Function,
    support: FnSupport,
    cases: &[crate::loader::GoldenCase],
    threads: usize,
    label: &str,
) -> Vec<ExecutionCollector> {
    let run = |ci: usize| {
        run_cell(subject, strategy, validators, function, caps, support, &cases[ci], oracle)
    };
    if threads <= 1 || cases.len() <= 1 {
        return (0..cases.len()).map(run).collect();
    }
    let workers = threads.min(cases.len());
    let next = AtomicUsize::new(0);
    let next_ref = &next;
    let nested: Vec<Vec<(usize, ExecutionCollector)>> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|w| {
                // `gold:w<worker>:<function>:<subject>` — the function is the
                // loader's per-batch contribution; the subject names itself.
                let name = format!("gold:w{w}:{}:{label}", function.name());
                std::thread::Builder::new()
                    .name(name)
                    .spawn_scoped(scope, move || {
                        let mut local: Vec<(usize, ExecutionCollector)> = Vec::new();
                        loop {
                            let k = next_ref.fetch_add(1, AtomicOrdering::Relaxed);
                            if k >= cases.len() {
                                break;
                            }
                            local.push((k, run(k)));
                        }
                        local
                    })
                    .expect("spawn named golden worker")
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut indexed: Vec<(usize, ExecutionCollector)> = nested.into_iter().flatten().collect();
    indexed.sort_by_key(|&(k, _)| k);
    indexed.into_iter().map(|(_, c)| c).collect()
}
