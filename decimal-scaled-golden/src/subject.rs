use std::collections::BTreeMap;

use crate::computed::Computed;
use crate::function::Function;
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
/// and how it behaves when that function's result overflows the cell. A library
/// that rounds differently per function expresses that here.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FnSupport {
    pub modes: Vec<RoundingMode>,
    pub overflow: Overflow,
}

/// What a subject can do. A subject is pinned to exactly one `(width, scale)`.
/// The function map's keys are the supported functions (absence == unsupported);
/// each value carries that function's modes + overflow behaviour. `width` /
/// `scale` / `storage_bits` are for the runner (golden derivation, overflow
/// detection + exact `Wrap` validation) — not passed to `execute`.
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
    /// The per-function support for `func`, or `None` if the subject does not
    /// support it.
    pub fn function(&self, func: Function) -> Option<&FnSupport> {
        self.functions.get(&func)
    }
}

/// A decimal implementation under test, in terms of its native `Value` type. A
/// subject covers exactly one `(width, scale)`. Implemented by a thin adapter in
/// each library — never in this crate.
pub trait DecimalSubject {
    type Value;

    fn capabilities(&self) -> Capabilities;

    /// Parse one golden field into the subject's value, or `Skip` if it is not
    /// exactly representable at this cell (the subject knows its own scale).
    fn from_text(&self, s: &str) -> Computed<Self::Value>;

    fn to_text(&self, v: &Self::Value) -> String;

    /// Run `func` over `inputs`, rounding with `mode`; `overflow` selects the
    /// overflow-handling variant where the op has one (otherwise ignored).
    fn execute(
        &self,
        func: Function,
        inputs: &[Self::Value],
        mode: RoundingMode,
        overflow: Overflow,
    ) -> Computed<Self::Value>;
}

/// Object-safe, `Value`-erased view a runner uses. Provided for every
/// `DecimalSubject` by the blanket impl below.
pub trait ErasedSubject {
    fn capabilities(&self) -> Capabilities;
    /// Correctness path: per-case `from_text -> execute -> to_text` (no timing).
    fn eval(&self, func: Function, inputs: &[&str], mode: RoundingMode, overflow: Overflow)
        -> Computed<String>;
    /// Timing path: pre-convert the whole cell (UNTIMED), warm up, then time one
    /// `black_box`-guarded loop over only `execute`. `None` if nothing to time.
    fn time_batch(&self, func: Function, cases: &[Vec<String>], mode: RoundingMode,
                  overflow: Overflow, warmup: u32) -> Option<u64>;
}

impl<T: DecimalSubject> ErasedSubject for T {
    fn capabilities(&self) -> Capabilities {
        <T as DecimalSubject>::capabilities(self)
    }

    fn eval(&self, func: Function, inputs: &[&str], mode: RoundingMode, overflow: Overflow)
        -> Computed<String>
    {
        let mut vals = Vec::with_capacity(inputs.len());
        for x in inputs {
            match self.from_text(x) {
                Computed::Value(v) => vals.push(v),
                Computed::Skip => return Computed::Skip,
                Computed::Error(e) => return Computed::Error(e),
            }
        }
        match self.execute(func, &vals, mode, overflow) {
            Computed::Value(v) => Computed::Value(self.to_text(&v)),
            Computed::Skip => Computed::Skip,
            Computed::Error(e) => Computed::Error(e),
        }
    }

    fn time_batch(&self, func: Function, cases: &[Vec<String>], mode: RoundingMode,
                  overflow: Overflow, warmup: u32) -> Option<u64>
    {
        // pre-convert the whole cell (UNTIMED); drop cases that don't convert.
        let mut batch: Vec<Vec<T::Value>> = Vec::with_capacity(cases.len());
        'next: for case in cases {
            let mut vals = Vec::with_capacity(case.len());
            for x in case {
                match self.from_text(x) {
                    Computed::Value(v) => vals.push(v),
                    _ => continue 'next,
                }
            }
            batch.push(vals);
        }
        if batch.is_empty() {
            return None;
        }
        let run = || {
            for vals in &batch {
                std::hint::black_box(self.execute(
                    func, std::hint::black_box(vals.as_slice()), mode, overflow,
                ));
            }
        };
        for _ in 0..warmup {
            run();
        }
        let start = std::time::Instant::now();
        run();
        Some(start.elapsed().as_nanos() as u64)
    }
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

    // A trivial typed subject: native type f64, sqrt only, 4-dp text.
    struct Sqrt64;
    impl DecimalSubject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self) -> Capabilities {
            caps_sqrt_only("sqrt64", 38, 15)
        }
        fn to_text(&self, v: &f64) -> String { format!("{:.4}", v) }
        fn from_text(&self, s: &str) -> Computed<f64> {
            match s.parse::<f64>() {
                Ok(x) if x >= 0.0 => Computed::Value(x),
                Ok(_) => Computed::Skip,
                Err(_) => Computed::Error("parse".into()),
            }
        }
        fn execute(&self, _f: Function, inputs: &[f64], _m: RoundingMode, _o: Overflow)
            -> Computed<f64> { Computed::Value(inputs[0].sqrt()) }
    }

    #[test]
    fn capabilities_lists_supported_functions() {
        let caps = DecimalSubject::capabilities(&Sqrt64);
        assert!(caps.function(Function::Sqrt).is_some());
        assert!(caps.function(Function::Exp).is_none());
        assert_eq!(caps.width, 38);
        assert_eq!(caps.scale, 15);
    }

    #[test]
    fn erased_eval_runs_through_value() {
        let s: &dyn ErasedSubject = &Sqrt64;
        assert_eq!(
            s.eval(Function::Sqrt, &["2"], RoundingMode::HalfToEven, Overflow::Panic),
            Computed::Value("1.4142".to_string())
        );
    }
    #[test]
    fn erased_eval_skips_out_of_domain() {
        let s: &dyn ErasedSubject = &Sqrt64;
        assert_eq!(
            s.eval(Function::Sqrt, &["-1"], RoundingMode::HalfToEven, Overflow::Panic),
            Computed::Skip
        );
    }
    #[test]
    fn time_batch_returns_some_for_value_cases() {
        let s: &dyn ErasedSubject = &Sqrt64;
        let cases = vec![vec!["2".to_string()], vec!["3".to_string()]];
        assert!(s
            .time_batch(Function::Sqrt, &cases, RoundingMode::HalfToEven, Overflow::Panic, 1)
            .is_some());
    }
    #[test]
    fn time_batch_none_when_all_skip() {
        let s: &dyn ErasedSubject = &Sqrt64;
        let cases = vec![vec!["-1".to_string()]];
        assert_eq!(
            s.time_batch(Function::Sqrt, &cases, RoundingMode::HalfToEven, Overflow::Panic, 1),
            None
        );
    }
}
