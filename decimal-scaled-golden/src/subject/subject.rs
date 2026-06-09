//! The `DecimalSubject` trait — the library-facing interface.

use crate::function::Function;
use crate::rounding::RoundingMode;

use super::{Capabilities, Computed, Limits, Overflow};

/// A decimal implementation under test, pinned to exactly one `(width, scale)`
/// cell, typed to its native `Value`. **Pure**: it parses, computes, and formats.
/// It never decides to skip and never catches — the skip/catch policy lives in the
/// runner. The one judgement that depends on its internals is `limits`.
pub trait DecimalSubject {
    /// The library's one native decimal type.
    type Value;

    /// Identity, radix, per-function support, and report metadata.
    fn capabilities(&self) -> Capabilities;

    /// A short human label for this subject — used in diagnostics such as the
    /// `ParallelRunner`'s worker thread names. Defaults to the `capabilities`
    /// name plus its `config` (e.g. `"decimal-scaled[scale=19,width=38]"`); a
    /// subject may override it with a tidier per-cell identifier.
    fn name(&self) -> String {
        let caps = self.capabilities();
        if caps.config.is_empty() {
            caps.name
        } else {
            let cfg: Vec<String> = caps.config.iter().map(|(k, v)| format!("{k}={v}")).collect();
            format!("{}[{}]", caps.name, cfg.join(","))
        }
    }

    /// Parse one input string to a value. Panics on malformed/unrepresentable
    /// input (the runner pre-filters such inputs and catches anything else).
    fn string_to_value(&self, s: &str) -> Self::Value;

    /// Format a value back to its canonical decimal string.
    fn value_to_string(&self, v: &Self::Value) -> String;

    /// The subject's representability envelope *at* a given value — the one fact
    /// that depends on the subject's internals. The runner does the comparison.
    fn limits(&self, value: &str) -> Limits;

    /// Curry `func`/`mode`/`overflow` into a compute-only closure over pre-parsed
    /// inputs — no parse, no format — so a timing run measures exactly the op. The
    /// closure yields `Computed<Value>` so a peer can report a non-value outcome
    /// (absent / non-real / error) without panicking.
    fn execute(
        &self,
        func: Function,
        mode: RoundingMode,
        overflow: Overflow,
    ) -> impl Fn(&[Self::Value]) -> Computed<Self::Value>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subject::{FnSupport, Radix};
    use std::collections::BTreeMap;

    /// Trivial subject: native f64, sqrt only, 4-dp text.
    struct Sqrt64;
    impl DecimalSubject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            let mut functions = BTreeMap::new();
            functions.insert(
                Function::Sqrt,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
            let mut config = BTreeMap::new();
            config.insert("width".into(), "38".into());
            config.insert("scale".into(), "4".into());
            Capabilities { name: "sqrt64".into(), radix: Radix::Decimal, config, functions }
        }
        fn string_to_value(&self, s: &str) -> f64 {
            s.parse::<f64>().expect("parse f64")
        }
        fn value_to_string(&self, v: &f64) -> String {
            format!("{v:.4}")
        }
        fn limits(&self, _value: &str) -> Limits {
            Limits { min_value: None, max_value: None, max_precision: 4 }
        }
        fn execute(&self, _f: Function, _m: RoundingMode, _o: Overflow) -> impl Fn(&[f64]) -> Computed<f64> {
            |inputs| Computed::Value(inputs[0].sqrt())
        }
    }

    #[test]
    fn capabilities_lists_supported_functions() {
        let caps = Sqrt64.capabilities();
        assert!(caps.function(Function::Sqrt).is_some());
        assert!(caps.function(Function::Exp).is_none());
        assert_eq!(caps.config.get("scale").map(String::as_str), Some("4"));
    }

    #[test]
    fn parse_compute_format() {
        let s = Sqrt64;
        let op = s.execute(Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic);
        match op(&[s.string_to_value("2")]) {
            Computed::Value(v) => assert_eq!(s.value_to_string(&v), "1.4142"),
            other => panic!("expected Value, got {other:?}"),
        }
    }
}
