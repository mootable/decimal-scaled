//! `ParallelRunner` — distributes the subject's executions across `threads` workers
//! pulling from a shared atomic work-index. Per-execution work is identical to serial.

use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::collector::{ExecutionCollector, FunctionCollector, SubjectCollector};
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::loader::{CaseLoader, GoldenCase};
use crate::subject::{Capabilities, DecimalSubject, FnSupport, Limits};
use crate::validators::Validator;

use super::run_cell;
use super::runner::GoldenRunner;

/// Distributes the subject's individual executions across `threads` workers.
pub struct ParallelRunner<E: ExecutionStrategy + Sync> {
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

impl<E: ExecutionStrategy + Sync> GoldenRunner for ParallelRunner<E> {
    fn run<S: DecimalSubject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector {
        let caps = subject.capabilities();
        let oracle = self.loader.oracle_limits();

        let plans: Vec<Plan> = functions
            .iter()
            .map(|&function| match caps.function(function) {
                None => Plan { function, support: None, cases: Vec::new() },
                Some(&support) => Plan {
                    function,
                    support: Some(support),
                    cases: self.loader.load(function).into_owned(),
                },
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
            subject, &self.strategy, &self.validators, &caps, &oracle, &plans, &jobs, self.threads,
        );

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
#[allow(clippy::too_many_arguments)]
fn run_jobs<S: DecimalSubject + Sync, E: ExecutionStrategy + Sync>(
    subject: &S,
    strategy: &E,
    validators: &[Box<dyn Validator + Sync>],
    caps: &Capabilities,
    oracle: &Limits,
    plans: &[Plan],
    jobs: &[(usize, usize)],
    threads: usize,
) -> Vec<ExecutionCollector> {
    let run = |&(pi, ci): &(usize, usize)| {
        let plan = &plans[pi];
        run_cell(
            subject, strategy, validators, plan.function, caps, plan.support.unwrap(),
            &plan.cases[ci], oracle,
        )
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
                        let k = next_ref.fetch_add(1, AtomicOrdering::Relaxed);
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
