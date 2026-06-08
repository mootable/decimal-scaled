//! The collector tree the tester fills as it runs:
//! `TestCollector` ⊃ `SubjectCollector` ⊃ `FunctionCollector` ⊃ `ExecutionCollector`.
//!
//! The execution strategy records an `ExecutionCollector`'s result + timing; the
//! validators read that result and push their verdicts into the same
//! `ExecutionCollector`. The collator (which lives OUTSIDE the tester) reads a
//! finished `TestCollector`.

use crate::function::Function;
use crate::outcome::Outcome;
use crate::subject::{Capabilities, FnSupport};

/// The outcome of executing one input-set: the actual output value, or a panic.
/// (A `Timeout` variant arrives with the runner's timeout guard.)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionResult {
    Value(String),
    Panic,
}

/// Whether a cell ran, and its execution result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellStatus {
    /// Not run yet.
    Pending,
    /// The subject can't represent this case at its width/scale — not run.
    Unsupported,
    /// Ran; here's the execution result (the actual output value, or a panic).
    Executed(ExecutionResult),
}

/// One golden case for one subject: the inputs, the expected (golden) output, the
/// subject's actual result (via `status`), timing, and the validators' verdicts.
/// The execution strategy writes `status`/`timing`; the validators read the
/// result and push into `validations`.
#[derive(Clone, Debug)]
pub struct ExecutionCollector {
    pub inputs: Vec<String>,
    pub expected: String,
    pub status: CellStatus,
    pub timing: Option<u64>,
    pub validations: Vec<Outcome>,
}

impl ExecutionCollector {
    pub fn new(inputs: Vec<String>, expected: String) -> ExecutionCollector {
        ExecutionCollector {
            inputs,
            expected,
            status: CellStatus::Pending,
            timing: None,
            validations: Vec::new(),
        }
    }
    /// The subject can't represent this case at its cell — record + skip.
    pub fn mark_unsupported(&mut self) {
        self.status = CellStatus::Unsupported;
    }
    /// Record the execution result (written by the execution strategy).
    pub fn record(&mut self, r: ExecutionResult) {
        self.status = CellStatus::Executed(r);
    }
    /// Record the average execution time (written by a timed strategy).
    pub fn record_timing(&mut self, nanos: u64) {
        self.timing = Some(nanos);
    }
    /// The execution result, if the cell ran.
    pub fn execution_result(&self) -> Option<&ExecutionResult> {
        match &self.status {
            CellStatus::Executed(r) => Some(r),
            _ => None,
        }
    }
    /// The actual output value, if the cell produced one.
    pub fn value(&self) -> Option<&str> {
        match &self.status {
            CellStatus::Executed(ExecutionResult::Value(s)) => Some(s),
            _ => None,
        }
    }
    /// Record a validator's verdict (written by the validators).
    pub fn add_validation(&mut self, outcome: Outcome) {
        self.validations.push(outcome);
    }
}

/// All cells for one function on one subject. `support` is the subject's
/// `FnSupport` (mode + overflow) for the function, or `None` when the subject
/// doesn't support it (no cells run).
#[derive(Clone, Debug)]
pub struct FunctionCollector {
    pub function: Function,
    pub support: Option<FnSupport>,
    pub cells: Vec<ExecutionCollector>,
}

impl FunctionCollector {
    pub fn new(function: Function, support: FnSupport) -> FunctionCollector {
        FunctionCollector { function, support: Some(support), cells: Vec::new() }
    }
    /// The subject doesn't support this function.
    pub fn unsupported(function: Function) -> FunctionCollector {
        FunctionCollector { function, support: None, cells: Vec::new() }
    }
    pub fn supported(&self) -> bool {
        self.support.is_some()
    }
    pub fn add(&mut self, cell: ExecutionCollector) {
        self.cells.push(cell);
    }
}

/// Everything tested for one subject (one `(width, scale)`). Carries the
/// subject's full `Capabilities` (name/width/scale + the per-function
/// modes/overflow) so the collator has it all.
#[derive(Clone, Debug)]
pub struct SubjectCollector {
    pub capabilities: Capabilities,
    pub functions: Vec<FunctionCollector>,
}

impl SubjectCollector {
    pub fn new(capabilities: Capabilities) -> SubjectCollector {
        SubjectCollector { capabilities, functions: Vec::new() }
    }
    pub fn add(&mut self, function: FunctionCollector) {
        self.functions.push(function);
    }
}

/// The whole run: every subject. The collator reads this.
#[derive(Clone, Debug, Default)]
pub struct TestCollector {
    pub subjects: Vec<SubjectCollector>,
}

impl TestCollector {
    pub fn new() -> TestCollector {
        TestCollector::default()
    }
    pub fn add(&mut self, subject: SubjectCollector) {
        self.subjects.push(subject);
    }
}
