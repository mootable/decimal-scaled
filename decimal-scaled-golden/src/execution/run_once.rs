//! `RunOnce` — compute once, no timing (correctness).

use crate::collector::{ExecutionCollector, ExecutionResult};
use crate::function::Function;
use crate::rounding::RoundingMode;
use crate::subject::{DecimalSubject, Overflow};

use super::strategy::{run_erased, ExecutionStrategy};

/// Run the op once. No timing.
pub struct RunOnce;

impl ExecutionStrategy for RunOnce {
    fn execute<S: DecimalSubject>(
        &self,
        subject: &S,
        input: &[String],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
        sink: &mut ExecutionCollector,
    ) {
        let op = subject.execute(function, mode, overflow);
        sink.record(ExecutionResult::Computed(run_erased(subject, &op, input)));
    }
}
