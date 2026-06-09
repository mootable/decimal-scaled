//! `SeriesRunner` — runs the subject's executions serially.

use crate::collector::{FunctionCollector, SubjectCollector};
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::loader::CaseLoader;
use crate::subject::DecimalSubject;
use crate::validators::Validator;

use super::run_cell;
use super::runner::GoldenRunner;

/// Runs the subject's executions serially.
pub struct SeriesRunner<E: ExecutionStrategy> {
    pub strategy: E,
    pub loader: Box<dyn CaseLoader>,
    pub validators: Vec<Box<dyn Validator + Sync>>,
}

impl<E: ExecutionStrategy> GoldenRunner for SeriesRunner<E> {
    fn run<S: DecimalSubject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector {
        let caps = subject.capabilities();
        let oracle = self.loader.oracle_limits();
        let mut collector = SubjectCollector::new(caps.clone());
        for &function in functions {
            collector.add(match caps.function(function) {
                None => FunctionCollector::unsupported(function),
                Some(&support) => {
                    let mut fc = FunctionCollector::new(function, support);
                    for case in self.loader.load(function).iter() {
                        fc.add(run_cell(
                            subject, &self.strategy, &self.validators, function, &caps, support,
                            case, &oracle,
                        ));
                    }
                    fc
                }
            });
        }
        collector
    }
}
