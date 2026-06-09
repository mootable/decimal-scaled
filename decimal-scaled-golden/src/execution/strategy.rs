//! `ExecutionStrategy` — the typed→string boundary, plus the erase/catch helpers
//! the strategies share. It parses, computes, catches the subject's panics, and
//! erases the closure's `Computed<Value>` to a `Value`-free `Computed<String>`.

use std::any::Any;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::collector::ExecutionCollector;
use crate::function::Function;
use crate::rounding::RoundingMode;
use crate::subject::{Computed, DecimalSubject, Overflow};

/// How one input-set's compute is run.
pub trait ExecutionStrategy {
    fn execute<S: DecimalSubject>(
        &self,
        subject: &S,
        input: &[String],
        function: Function,
        mode: RoundingMode,
        overflow: Overflow,
        sink: &mut ExecutionCollector,
    );
}

/// Parse + compute + format one input-set, catching any *subject* panic as
/// `Computed::Panic`. `Value` is erased to `String` here, where it is still concrete.
pub(super) fn run_erased<S: DecimalSubject>(
    subject: &S,
    op: &impl Fn(&[S::Value]) -> Computed<S::Value>,
    input: &[String],
) -> Computed<String> {
    let caught = catch_unwind(AssertUnwindSafe(|| {
        let vals: Vec<S::Value> = input.iter().map(|s| subject.string_to_value(s)).collect();
        erase(subject, op(&vals))
    }));
    match caught {
        Ok(c) => c,
        Err(p) => Computed::Panic(panic_message(p)),
    }
}

/// Parse an input-set to values (untimed), or the panic message if parsing panicked.
pub(super) fn parse_vals<S: DecimalSubject>(
    subject: &S,
    input: &[String],
) -> Result<Vec<S::Value>, String> {
    catch_unwind(AssertUnwindSafe(|| {
        input.iter().map(|s| subject.string_to_value(s)).collect::<Vec<S::Value>>()
    }))
    .map_err(panic_message)
}

/// Compute + format pre-parsed values, catching a subject panic as `Computed::Panic`.
pub(super) fn compute_erased<S: DecimalSubject>(
    subject: &S,
    op: &impl Fn(&[S::Value]) -> Computed<S::Value>,
    vals: &[S::Value],
) -> Computed<String> {
    catch_unwind(AssertUnwindSafe(|| erase(subject, op(vals))))
        .unwrap_or_else(|p| Computed::Panic(panic_message(p)))
}

/// Erase a `Computed<Value>` to `Computed<String>`: only `Value` touches the native
/// type (stringified via `value_to_string`); every other arm passes through.
fn erase<S: DecimalSubject>(subject: &S, c: Computed<S::Value>) -> Computed<String> {
    match c {
        Computed::Value(v) => Computed::Value(subject.value_to_string(&v)),
        Computed::NonReal(n) => Computed::NonReal(n),
        Computed::Absent => Computed::Absent,
        Computed::Error(s) => Computed::Error(s),
        Computed::Timeout(t) => Computed::Timeout(t),
        Computed::Panic(s) => Computed::Panic(s),
    }
}

/// Best-effort message from a caught panic payload.
fn panic_message(p: Box<dyn Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".to_string()
    }
}
