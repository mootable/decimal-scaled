//! `decimal-scaled-golden` — the golden corpus + validation/comparison harness.
//! Library-agnostic: a library implements `subject::DecimalSubject` to be
//! validated/benched against the singular golden values. No subject impls live
//! in this crate.

pub mod rounding;
pub mod function;
pub mod value;
pub mod parser;
pub mod subject;
pub mod loader;
pub mod computed;
pub mod outcome;
pub mod bigdec;
pub mod validator;

pub use function::Function;
pub use rounding::RoundingMode;
pub use value::GoldenValue;
pub use computed::Computed;
pub use outcome::{Outcome, ResultRecord};
pub use validator::validate_one;
pub use subject::{Capabilities, DecimalSubject, ErasedSubject};
