//! The `CaseLoader` trait and the `GoldenCase` it yields.

use std::borrow::Cow;

use crate::function::Function;
use crate::subject::Limits;

/// One parsed golden line: the inputs (arity per `Function`) + the raw output
/// string, both unparsed `digits.digits`, plus the 1-based source line it came
/// from (`0` for an in-memory case with no file) so a failing cell can be traced
/// back to its golden file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoldenCase {
    pub inputs: Vec<String>,
    pub output_raw: String,
    pub line: usize,
}

/// Yields the golden cases for a function, and the oracle's representability reach.
/// `load` returns a reusable, re-iterable view (never a one-shot stream): the
/// runner consults it once per `(subject, function)` and many subjects share a
/// function's cases.
pub trait CaseLoader {
    fn load(&self, func: Function) -> Cow<'_, [GoldenCase]>;
    /// The oracle's envelope: unbounded magnitude, `max_precision = gen_precision
    /// − guard` (the rounding-guard digits are not verifiable depth).
    fn oracle_limits(&self) -> Limits;
}
