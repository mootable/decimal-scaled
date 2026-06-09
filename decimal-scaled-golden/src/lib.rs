//! `decimal-scaled-golden` — the golden values + validation/reporting harness.
//! Library-agnostic: a library implements `subject::DecimalSubject` (one per
//! `(width, scale)` cell) to be validated/benched against the singular golden
//! values. No subject impls live in this crate.
//!
//! One folder per extension point — `subject`, `loader`, `execution`, `runner`,
//! `validators`, `reporting` — with shared leaf types (`function`, `rounding`,
//! `outcome`, `string_decimal`, `collector`) gathered under `support`.

pub mod execution;
pub mod loader;
pub mod reporting;
pub mod runner;
pub mod subject;
pub mod support;
pub mod validators;

// Crate-root aliases for the shared leaves, so `crate::function`, `crate::collector`,
// etc. resolve regardless of where the type lives.
pub use support::{collector, function, outcome, rounding, string_decimal};

pub use collector::{
    CellStatus, ExecutionCollector, ExecutionResult, FunctionCollector, RunCollector,
    SubjectCollector,
};
pub use execution::{ExecutionStrategy, RunOnce, Timed};
#[cfg(feature = "bench")]
pub use execution::CriterionStrategy;
pub use function::Function;
pub use loader::{CaseLoader, FileLoader, GoldenCase, GoldenValue};
pub use outcome::Outcome;
pub use reporting::{
    ConsoleReporter, InlineReporter, ReportArtifact, ReportOutput, Reporter, RunSummary, TsvReporter,
};
pub use rounding::RoundingMode;
pub use runner::{GoldenRunner, ParallelRunner, SeriesRunner};
pub use subject::{
    Capabilities, Computed, DecimalSubject, FnSupport, Limits, NonReal, Overflow, Radix,
};
pub use validators::{
    OverflowValidator, PrecisionValidator, RoundingValidator, ValidationContext, Validator,
};
