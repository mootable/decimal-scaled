//! The `GoldenRunner` trait — runs one subject over a set of functions.

use crate::collector::SubjectCollector;
use crate::function::Function;
use crate::subject::DecimalSubject;

/// Runs one subject over a set of functions, producing its `SubjectCollector`.
pub trait GoldenRunner {
    fn run<S: DecimalSubject + Sync>(&self, subject: &S, functions: &[Function]) -> SubjectCollector;
}
