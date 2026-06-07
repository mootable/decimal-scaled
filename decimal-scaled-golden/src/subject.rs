use crate::computed::Computed;
use crate::function::Function;
use crate::rounding::RoundingMode;

/// What a subject can do for a given function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub name: String,
    pub supported: bool,
    pub max_width: u32,
    pub max_scale: u32,
    pub rounding_modes: Vec<RoundingMode>,
}

/// A decimal implementation under test, in terms of its native `Value` type.
/// Implemented by a thin adapter in each library — never in this crate.
pub trait DecimalSubject {
    type Value;
    fn capabilities(&self, func: Function) -> Capabilities;
    fn to_text(&self, v: &Self::Value) -> String;
    fn from_text(&self, s: &str, width: u32, scale: u32) -> Computed<Self::Value>;
    fn execute(
        &self,
        func: Function,
        inputs: &[Self::Value],
        width: u32,
        scale: u32,
        mode: RoundingMode,
    ) -> Computed<Self::Value>;
}

/// Object-safe, `Value`-erased view a runner uses. Provided for every
/// `DecimalSubject` by the blanket impl below.
pub trait ErasedSubject {
    fn capabilities(&self, func: Function) -> Capabilities;
    /// Correctness path: per-case `from_text -> execute -> to_text` (no timing).
    fn eval(&self, func: Function, inputs: &[&str], width: u32, scale: u32, mode: RoundingMode)
        -> Computed<String>;
    /// Timing path: pre-convert the whole cell (UNTIMED), warm up, then time one
    /// `black_box`-guarded loop over only `execute`. `None` if nothing to time.
    fn time_batch(&self, func: Function, cases: &[Vec<String>], width: u32, scale: u32,
                  mode: RoundingMode, warmup: u32) -> Option<u64>;
}

impl<T: DecimalSubject> ErasedSubject for T {
    fn capabilities(&self, func: Function) -> Capabilities {
        <T as DecimalSubject>::capabilities(self, func)
    }

    fn eval(&self, func: Function, inputs: &[&str], width: u32, scale: u32, mode: RoundingMode)
        -> Computed<String>
    {
        let mut vals = Vec::with_capacity(inputs.len());
        for x in inputs {
            match self.from_text(x, width, scale) {
                Computed::Value(v) => vals.push(v),
                Computed::Skip => return Computed::Skip,
                Computed::Error(e) => return Computed::Error(e),
            }
        }
        match self.execute(func, &vals, width, scale, mode) {
            Computed::Value(v) => Computed::Value(self.to_text(&v)),
            Computed::Skip => Computed::Skip,
            Computed::Error(e) => Computed::Error(e),
        }
    }

    fn time_batch(&self, func: Function, cases: &[Vec<String>], width: u32, scale: u32,
                  mode: RoundingMode, warmup: u32) -> Option<u64>
    {
        // pre-convert the whole cell (UNTIMED); drop cases that don't convert.
        let mut batch: Vec<Vec<T::Value>> = Vec::with_capacity(cases.len());
        'next: for case in cases {
            let mut vals = Vec::with_capacity(case.len());
            for x in case {
                match self.from_text(x, width, scale) {
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
                    func, std::hint::black_box(vals.as_slice()), width, scale, mode,
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

    // A trivial typed subject: native type f64, sqrt only, 4-dp text.
    struct Sqrt64;
    impl DecimalSubject for Sqrt64 {
        type Value = f64;
        fn capabilities(&self, _f: Function) -> Capabilities {
            Capabilities {
                name: "sqrt64".into(), supported: true, max_width: 38, max_scale: 15,
                rounding_modes: vec![RoundingMode::HalfToEven],
            }
        }
        fn to_text(&self, v: &f64) -> String { format!("{:.4}", v) }
        fn from_text(&self, s: &str, _w: u32, _sc: u32) -> Computed<f64> {
            match s.parse::<f64>() {
                Ok(x) if x >= 0.0 => Computed::Value(x),
                Ok(_) => Computed::Skip,
                Err(_) => Computed::Error("parse".into()),
            }
        }
        fn execute(&self, _f: Function, inputs: &[f64], _w: u32, _sc: u32, _m: RoundingMode)
            -> Computed<f64> { Computed::Value(inputs[0].sqrt()) }
    }

    #[test]
    fn erased_eval_runs_through_value() {
        let s: &dyn ErasedSubject = &Sqrt64;
        assert_eq!(
            s.eval(Function::Sqrt, &["2"], 38, 4, RoundingMode::HalfToEven),
            Computed::Value("1.4142".to_string())
        );
    }
    #[test]
    fn erased_eval_skips_out_of_domain() {
        let s: &dyn ErasedSubject = &Sqrt64;
        assert_eq!(s.eval(Function::Sqrt, &["-1"], 38, 4, RoundingMode::HalfToEven), Computed::Skip);
    }
    #[test]
    fn time_batch_returns_some_for_value_cases() {
        let s: &dyn ErasedSubject = &Sqrt64;
        let cases = vec![vec!["2".to_string()], vec!["3".to_string()]];
        assert!(s.time_batch(Function::Sqrt, &cases, 38, 4, RoundingMode::HalfToEven, 1).is_some());
    }
    #[test]
    fn time_batch_none_when_all_skip() {
        let s: &dyn ErasedSubject = &Sqrt64;
        let cases = vec![vec!["-1".to_string()]];
        assert_eq!(s.time_batch(Function::Sqrt, &cases, 38, 4, RoundingMode::HalfToEven, 1), None);
    }
}
