//! `decimal-scaled-golden` — the golden corpus + validation/comparison harness.
//! Library-agnostic: a library implements `subject::Subject` (one per
//! `(width, scale)` cell) to be validated/benched against the singular golden
//! values. No subject impls live in this crate.

pub mod rounding;
pub mod function;
pub mod value;
pub mod parser;
pub mod subject;
pub mod loader;
pub mod computed;
pub mod outcome;
pub mod bigdec;
pub mod overflow;
pub mod validator;
pub mod validate;
pub mod execution;
pub mod tester;
pub mod collator;

pub use function::Function;
pub use rounding::RoundingMode;
pub use value::GoldenValue;
pub use computed::Computed;
pub use outcome::{Outcome, ResultRecord};
pub use overflow::expected_overflow;
pub use validator::validate_one;
pub use subject::{Capabilities, CaseOutput, FnSupport, Overflow, Subject};
pub use validate::{
    DefaultOverflow, DefaultPrecision, DefaultRounding, NoOpOverflow, NoOpPrecision, NoOpRounding,
    ValidateOverflow, ValidatePrecision, ValidateRounding,
};
pub use execution::{ExecutionStrategy, RunOnce, Timed};
pub use tester::{run, run_parallel, Tester, Validator};
pub use collator::Collator;
