use std::collections::BTreeMap;

use crate::function::Function;
use crate::rounding::RoundingMode;

/// What a subject does when a true result exceeds its `(width, scale)`.
///
/// The runner uses this to VALIDATE an overflowing cell rather than skip it: the
/// subject declares its documented behaviour and the runner checks it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overflow {
    /// Panics on overflow (e.g. a checked op / debug build).
    Panic,
    /// Saturates to the largest/smallest value representable at the cell.
    Saturate,
    /// Keeps the low `width` decimal digits (decimal truncation of the top).
    Truncate,
    /// Wraps modulo the 2's-complement integer storage (see
    /// [`Capabilities::storage_bits`]).
    Wrap,
}

/// Per-function support: the rounding mode a subject is tested under for one
/// function (one mode per function for now), and how it behaves when that
/// function's result overflows the cell.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FnSupport {
    pub mode: RoundingMode,
    pub overflow: Overflow,
}

/// What a subject can do. A subject is pinned to exactly one `(width, scale)`.
/// The function map's keys are the supported functions (absence == unsupported).
/// `width`/`scale`/`storage_bits` are for the runner (golden derivation, the
/// representability skip, overflow detection + exact `Wrap` validation).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Capabilities {
    pub name: String,
    pub width: u32,
    pub scale: u32,
    /// Bit width of the 2's-complement integer storage, for exact
    /// [`Overflow::Wrap`] validation.
    pub storage_bits: u32,
    pub functions: BTreeMap<Function, FnSupport>,
}

impl Capabilities {
    /// The per-function support for `func`, or `None` if unsupported.
    pub fn function(&self, func: Function) -> Option<&FnSupport> {
        self.functions.get(&func)
    }
}

/// A decimal implementation under test, pinned to exactly one `(width, scale)`,
/// typed to its native `Value`. **Pure**: it parses, computes, and formats, and
/// **panics** on bad input — it never decides to skip and never catches. The
/// catching + skip policy live in the runner / execution strategy (our code).
pub trait Subject {
    type Value;

    fn capabilities(&self) -> Capabilities;

    /// Parse one input string to a value. Panics on malformed/unrepresentable
    /// input (the runner avoids feeding such inputs and catches anything else).
    fn string_to_value(&self, s: &str) -> Self::Value;

    /// Format a value back to its canonical decimal string.
    fn value_to_string(&self, v: &Self::Value) -> String;

    /// Curry `func`/`mode`/`overflow` into a closure that computes the op over
    /// pre-parsed inputs. The closure is compute only — no parse, no format — so
    /// the timing path measures exactly it.
    fn execute(
        &self,
        func: Function,
        mode: RoundingMode,
        overflow: Overflow,
    ) -> impl Fn(&[Self::Value]) -> Self::Value;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trivial subject: native f64, sqrt only, 4-dp text. Panics on bad input.
    struct Sqrt64;
    impl Subject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            Capabilities { name: "sqrt64".into(), width: 38, scale: 15, storage_bits: 128, functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse::<f64>().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn execute(
            &self, _func: Function, _mode: RoundingMode, _overflow: Overflow,
        ) -> impl Fn(&[f64]) -> f64 {
            |inputs| inputs[0].sqrt()
        }
    }

    #[test]
    fn capabilities_lists_supported_functions() {
        let caps = Sqrt64.capabilities();
        assert!(caps.function(Function::Sqrt).is_some());
        assert!(caps.function(Function::Exp).is_none());
    }

    #[test]
    fn parse_compute_format() {
        let s = Sqrt64;
        let op = s.execute(Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic);
        let v = op(&[s.string_to_value("2")]);
        assert_eq!(s.value_to_string(&v), "1.4142");
    }
}
