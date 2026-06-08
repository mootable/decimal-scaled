//! `decimal-scaled-golden` — the golden corpus + validation/comparison harness.
//! Library-agnostic: a library implements `subject::Subject` (one per
//! `(width, scale)` cell) to be validated/benched against the singular golden
//! values. No subject impls live in this crate.

pub mod rounding;
pub mod function;
pub mod value;
pub mod parser;
pub mod subject;
pub mod outcome;
pub mod bigdec;
pub mod overflow;
pub mod validator;
pub mod execution;
pub mod collector;
pub mod caseloader;
pub mod tester;
pub mod collator;

pub use function::Function;
pub use rounding::RoundingMode;
pub use value::GoldenValue;
pub use outcome::{Outcome, ResultRecord};
pub use subject::{Capabilities, FnSupport, Overflow, Subject};
pub use collector::{
    CellStatus, ExecutionCollector, ExecutionResult, FunctionCollector, SubjectCollector,
    TestCollector,
};
pub use execution::{ExecutionStrategy, RunOnce, Timed};
pub use caseloader::{CaseLoader, FileCaseLoader};
pub use tester::{ParallelTester, SeriesTester, Tester};
pub use overflow::expected_overflow;
pub use validator::{OverflowValidator, PrecisionValidator, RoundingValidator, Validator};
pub use collator::Collator;
