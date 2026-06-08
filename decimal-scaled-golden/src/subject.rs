use std::collections::BTreeMap;

use crate::computed::Computed;
use crate::function::Function;
use crate::parser::GoldenCase;
use crate::rounding::RoundingMode;

/// What a subject does when a true result exceeds its `(width, scale)`.
///
/// The runner uses this to VALIDATE an overflowing cell rather than skip it: the
/// subject declares its documented behaviour and the runner checks it actually
/// did that (e.g. an expected panic becomes a `Pass`, not a failure).
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

/// Per-function support: the rounding modes a subject exercises for one function,
/// and how it behaves when that function's result overflows the cell.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FnSupport {
    pub modes: Vec<RoundingMode>,
    pub overflow: Overflow,
}

/// What a subject can do. A subject is pinned to exactly one `(width, scale)`.
/// The function map's keys are the supported functions (absence == unsupported).
/// `width`/`scale`/`storage_bits` are for the runner (golden derivation, overflow
/// detection + exact `Wrap` validation).
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

/// One case's execution result, after catching any panic. Produced on the
/// correctness path; consumed by the validators.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CaseOutput {
    Value(String),
    Skip,
    Error(String),
    Panic,
}

/// A decimal implementation under test, pinned to exactly one `(width, scale)`.
/// Object-safe (no associated type) so a `Vec<Box<dyn Subject>>` can mix cells
/// and libraries. Implemented by a thin adapter in each library — never here.
pub trait Subject {
    fn capabilities(&self) -> Capabilities;

    /// Correctness path: parse inputs, compute, **format** the result — used only
    /// when something validates. `Skip` if an input isn't exactly representable.
    fn eval(&self, func: Function, inputs: &[&str], mode: RoundingMode, overflow: Overflow)
        -> Computed<String>;

    /// Timing path: parse `cases` into the typed batch **up front (untimed)** and
    /// return a closure that runs ONLY `execute` over it — no parse, no format.
    /// `None` if nothing converts. The caller (an `ExecutionStrategy`) times this.
    fn compute_thunk<'a>(
        &'a self,
        cases: &'a [GoldenCase],
        func: Function,
        mode: RoundingMode,
        overflow: Overflow,
    ) -> Option<Box<dyn FnMut() + 'a>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps_sqrt_only(name: &str, width: u32, scale: u32) -> Capabilities {
        let mut functions = BTreeMap::new();
        functions.insert(
            Function::Sqrt,
            FnSupport { modes: vec![RoundingMode::HalfToEven], overflow: Overflow::Panic },
        );
        Capabilities { name: name.into(), width, scale, storage_bits: 128, functions }
    }

    /// Trivial subject: native f64, sqrt only, 4-dp text.
    struct Sqrt64;
    impl Subject for Sqrt64 {
        fn capabilities(&self) -> Capabilities {
            caps_sqrt_only("sqrt64", 38, 15)
        }
        fn eval(&self, _f: Function, inputs: &[&str], _m: RoundingMode, _o: Overflow)
            -> Computed<String>
        {
            match inputs[0].parse::<f64>() {
                Ok(x) if x >= 0.0 => Computed::Value(format!("{:.4}", x.sqrt())),
                Ok(_) => Computed::Skip,
                Err(_) => Computed::Error("parse".into()),
            }
        }
        fn compute_thunk<'a>(
            &'a self,
            cases: &'a [GoldenCase],
            _func: Function,
            _mode: RoundingMode,
            _overflow: Overflow,
        ) -> Option<Box<dyn FnMut() + 'a>> {
            let batch: Vec<f64> = cases
                .iter()
                .filter_map(|c| c.inputs.first().and_then(|s| s.parse::<f64>().ok()))
                .collect();
            if batch.is_empty() {
                return None;
            }
            Some(Box::new(move || {
                for x in &batch {
                    std::hint::black_box(x.sqrt());
                }
            }))
        }
    }

    #[test]
    fn capabilities_lists_supported_functions() {
        let caps = Sqrt64.capabilities();
        assert!(caps.function(Function::Sqrt).is_some());
        assert!(caps.function(Function::Exp).is_none());
        assert_eq!(caps.width, 38);
        assert_eq!(caps.scale, 15);
    }

    #[test]
    fn eval_runs_and_formats() {
        assert_eq!(
            Sqrt64.eval(Function::Sqrt, &["2"], RoundingMode::HalfToEven, Overflow::Panic),
            Computed::Value("1.4142".to_string())
        );
    }

    #[test]
    fn eval_skips_out_of_domain() {
        assert_eq!(
            Sqrt64.eval(Function::Sqrt, &["-1"], RoundingMode::HalfToEven, Overflow::Panic),
            Computed::Skip
        );
    }

    #[test]
    fn compute_thunk_runs() {
        let cases = vec![
            GoldenCase { inputs: vec!["2".into()], output_raw: "1.4142".into() },
            GoldenCase { inputs: vec!["3".into()], output_raw: "1.7321".into() },
        ];
        let mut thunk = Sqrt64
            .compute_thunk(&cases, Function::Sqrt, RoundingMode::HalfToEven, Overflow::Panic)
            .expect("some");
        thunk(); // does not panic
    }
}
