// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The collector tree the runner fills as it runs:
//! `RunCollector` ⊃ `SubjectCollector` ⊃ `FunctionCollector` ⊃ `ExecutionCollector`.
//!
//! The execution strategy records an `ExecutionResult`; validators' verdicts are
//! written into the same `ExecutionCollector` by the runner. A reporter (which
//! lives OUTSIDE the runner) reads a finished `RunCollector`.

use crate::function::Function;
use crate::outcome::Outcome;
use crate::subject::{Capabilities, Computed, FnSupport};

/// The runner's record of attempting one cell. `Computed` is the *test*'s outcome
/// (including the subject's own `Panic`/`Timeout`); the other two are *runner*-level
/// events the test can't report itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionResult {
    /// The test ran — here's its (erased) outcome.
    Computed(Computed<String>),
    /// The harness itself failed — NOT the test (bad golden data / internal fault).
    HarnessError(String),
    /// The runner did not run it (unrepresentable input / unsupported).
    Skipped,
}

/// Whether a cell ran, and its execution result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellStatus {
    /// Not run yet.
    Pending,
    /// Attempted; here's the result.
    Done(ExecutionResult),
}

/// One golden case for one subject: the inputs, the expected (raw golden) output,
/// the execution result (via `status`), timing, the `oracle_limited` flag, and the
/// validators' verdicts.
#[derive(Clone, Debug)]
pub struct ExecutionCollector {
    pub inputs: Vec<String>,
    pub expected: String,
    /// 1-based source line in the golden file (`0` if the case had no file),
    /// so a failing cell can be traced straight back to its golden line.
    pub line: usize,
    pub status: CellStatus,
    pub timing: Option<u64>,
    /// Set when grading was clamped to the oracle's depth (§4.3): a Pass here is
    /// honest-but-unverified-beyond-the-oracle.
    pub oracle_limited: bool,
    pub validations: Vec<Outcome>,
}

impl ExecutionCollector {
    pub fn new(inputs: Vec<String>, expected: String, line: usize) -> ExecutionCollector {
        ExecutionCollector {
            inputs,
            expected,
            line,
            status: CellStatus::Pending,
            timing: None,
            oracle_limited: false,
            validations: Vec::new(),
        }
    }
    /// The runner did not run this cell.
    pub fn mark_skipped(&mut self) {
        self.status = CellStatus::Done(ExecutionResult::Skipped);
    }
    /// Record the execution result (written by the execution strategy).
    pub fn record(&mut self, r: ExecutionResult) {
        self.status = CellStatus::Done(r);
    }
    /// Record the average execution time (written by a timed strategy).
    pub fn record_timing(&mut self, nanos: u64) {
        self.timing = Some(nanos);
    }
    /// The execution result, if the cell was attempted.
    pub fn result(&self) -> Option<&ExecutionResult> {
        match &self.status {
            CellStatus::Done(r) => Some(r),
            CellStatus::Pending => None,
        }
    }
    /// The test outcome, if the cell produced one (i.e. it ran, not skipped / harness-errored).
    pub fn computed(&self) -> Option<&Computed<String>> {
        match &self.status {
            CellStatus::Done(ExecutionResult::Computed(c)) => Some(c),
            _ => None,
        }
    }
    /// The produced finite value string, if any.
    pub fn value(&self) -> Option<&str> {
        match self.computed() {
            Some(Computed::Value(s)) => Some(s),
            _ => None,
        }
    }
    /// Record a validator's verdict (written by the runner).
    pub fn add_validation(&mut self, outcome: Outcome) {
        self.validations.push(outcome);
    }
}

/// All cells for one function on one subject. `support` is the subject's
/// `FnSupport` for the function, or `None` when unsupported (no cells run).
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

/// Everything tested for one subject (one cell). Carries the subject's full
/// `Capabilities` so a reporter has its `config` metadata.
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

/// One whole run: every subject. A reporter reads this.
#[derive(Clone, Debug, Default)]
pub struct RunCollector {
    pub subjects: Vec<SubjectCollector>,
}

impl RunCollector {
    pub fn new() -> RunCollector {
        RunCollector::default()
    }
    pub fn add(&mut self, subject: SubjectCollector) {
        self.subjects.push(subject);
    }
}
