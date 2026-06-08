use std::cmp::Ordering;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use crate::bigdec::cmp_mag;
use crate::computed::Computed;
use crate::execution::ExecutionStrategy;
use crate::function::Function;
use crate::outcome::{Outcome, ResultRecord};
use crate::parser::GoldenCase;
use crate::rounding::RoundingMode;
use crate::subject::{CaseOutput, Capabilities, Overflow, Subject};
use crate::validate::{
    DefaultOverflow, DefaultPrecision, DefaultRounding, NoOpOverflow, NoOpPrecision, NoOpRounding,
    ValidateOverflow, ValidatePrecision, ValidateRounding,
};
use crate::value::GoldenValue;

/// A tester scores a subject's output. It owns three validations (rounding,
/// overflow, precision) and provides the `run`/`run_parallel` loop as default
/// methods. The one shipped impl is [`Validator`].
pub trait Tester {
    fn validate_rounding(
        &self, got: &str, golden: &GoldenValue, width: u32, scale: u32, mode: RoundingMode,
    ) -> Outcome;
    fn validate_overflow(
        &self, got: &CaseOutput, golden: &GoldenValue, width: u32, scale: u32, storage_bits: u32,
        overflow: Overflow,
    ) -> Outcome;
    fn validate_precision(
        &self, got: &str, golden: &GoldenValue, width: u32, scale: u32, mode: RoundingMode,
    ) -> Option<String>;

    /// Whether anything validates. When false the runner skips the format/eval
    /// pass (a pure-timing or no-op run).
    fn validates(&self) -> bool {
        true
    }

    /// Run `function` over a list of subjects, serially. Each subject is one
    /// cell; for each supported mode, one `ResultRecord`. Subjects that don't
    /// support the function contribute nothing.
    fn run(
        &self,
        strategy: &dyn ExecutionStrategy,
        subjects: &[Box<dyn Subject>],
        function: Function,
        cases: &[GoldenCase],
    ) -> Vec<ResultRecord> {
        let mut out = Vec::new();
        for subject in subjects {
            let caps = subject.capabilities();
            let modes = match caps.function(function) {
                Some(s) => s.modes.clone(),
                None => continue,
            };
            for mode in modes {
                out.push(run_cell(self, subject.as_ref(), function, cases, &caps, mode, strategy));
            }
        }
        out
    }

    /// Parallel sibling of [`Tester::run`]: distributes `(subject, mode)` across
    /// `threads` workers via a shared atomic index. `threads <= 1` runs serially.
    /// CORRECTNESS ONLY — timing must use the serial `run` (concurrent workers
    /// corrupt timing). Records returned in stable `(subject, mode)` order.
    fn run_parallel(
        &self,
        strategy: &(dyn ExecutionStrategy + Sync),
        subjects: &[Box<dyn Subject + Sync>],
        function: Function,
        cases: &[GoldenCase],
        threads: usize,
    ) -> Vec<ResultRecord>
    where
        Self: Sync,
    {
        let caps: Vec<Capabilities> = subjects.iter().map(|s| s.capabilities()).collect();
        let mut work: Vec<(usize, RoundingMode)> = Vec::new();
        for (i, c) in caps.iter().enumerate() {
            if let Some(s) = c.function(function) {
                for &m in &s.modes {
                    work.push((i, m));
                }
            }
        }
        if work.is_empty() {
            return Vec::new();
        }
        if threads <= 1 {
            return work
                .into_iter()
                .map(|(i, m)| {
                    run_cell(self, subjects[i].as_ref(), function, cases, &caps[i], m, strategy)
                })
                .collect();
        }
        let next = AtomicUsize::new(0);
        let n_workers = threads.min(work.len());
        let work_ref = &work;
        let caps_ref = &caps;
        let next_ref = &next;
        let nested: Vec<Vec<(usize, ResultRecord)>> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..n_workers)
                .map(|_| {
                    scope.spawn(move || {
                        let mut local: Vec<(usize, ResultRecord)> = Vec::new();
                        loop {
                            let k = next_ref.fetch_add(1, AtomicOrdering::Relaxed);
                            if k >= work_ref.len() {
                                break;
                            }
                            let (i, m) = work_ref[k];
                            local.push((
                                k,
                                run_cell(
                                    self, subjects[i].as_ref(), function, cases, &caps_ref[i], m,
                                    strategy,
                                ),
                            ));
                        }
                        local
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        let mut indexed: Vec<(usize, ResultRecord)> = nested.into_iter().flatten().collect();
        indexed.sort_by_key(|&(k, _)| k);
        indexed.into_iter().map(|(_, r)| r).collect()
    }
}

/// Process one cell: a validation pass (per-case `eval`, only when something
/// validates) plus the strategy's timing measurement.
fn run_cell(
    tester: &(impl Tester + ?Sized),
    subject: &dyn Subject,
    function: Function,
    cases: &[GoldenCase],
    caps: &Capabilities,
    mode: RoundingMode,
    strategy: &dyn ExecutionStrategy,
) -> ResultRecord {
    let (width, scale, storage_bits) = (caps.width, caps.scale, caps.storage_bits);
    let overflow = caps.function(function).map(|f| f.overflow).unwrap_or(Overflow::Panic);

    let mut worst = Outcome::Skipped;
    let mut worst_prec: Option<String> = None;
    let mut detail: Option<String> = None;

    if tester.validates() {
        for case in cases {
            let golden = match GoldenValue::parse(&case.output_raw) {
                Some(g) => g,
                None => continue,
            };
            let refs: Vec<&str> = case.inputs.iter().map(|s| s.as_str()).collect();
            let out = match catch_unwind(AssertUnwindSafe(|| {
                subject.eval(function, &refs, mode, overflow)
            })) {
                Err(_) => CaseOutput::Panic,
                Ok(Computed::Value(s)) => CaseOutput::Value(s),
                Ok(Computed::Skip) => CaseOutput::Skip,
                Ok(Computed::Error(e)) => CaseOutput::Error(e),
            };
            let fits = golden.fits(width, scale);
            let outcome = if fits {
                match &out {
                    CaseOutput::Value(got) => {
                        tester.validate_rounding(got, &golden, width, scale, mode)
                    }
                    CaseOutput::Skip => Outcome::Skipped,
                    CaseOutput::Error(e) => Outcome::Error { reason: e.clone() },
                    CaseOutput::Panic => Outcome::Panic,
                }
            } else {
                tester.validate_overflow(&out, &golden, width, scale, storage_bits, overflow)
            };
            let prec = if fits {
                if let CaseOutput::Value(got) = &out {
                    tester.validate_precision(got, &golden, width, scale, mode)
                } else {
                    None
                }
            } else {
                None
            };
            if worse_than(&outcome, &worst) {
                worst = outcome;
                detail = Some(case.inputs.join(" "));
            }
            worst_prec = max_delta(worst_prec, prec);
        }
        if matches!(worst, Outcome::Pass | Outcome::Skipped) {
            detail = None;
        }
    }

    let nanos = strategy.measure(subject, cases, function, mode, overflow);

    ResultRecord {
        library: caps.name.clone(), function, width, scale, mode,
        outcome: worst, precision: worst_prec, detail, nanos,
    }
}

/// Strict "more severe than", breaking MisRounded ties by larger delta.
fn worse_than(a: &Outcome, b: &Outcome) -> bool {
    match (a, b) {
        (Outcome::MisRounded { delta: da }, Outcome::MisRounded { delta: db }) => {
            cmp_mag(da, db) == Ordering::Greater
        }
        _ => a.severity() > b.severity(),
    }
}

/// Keep the larger of two ULP deltas (cell precision = worst case).
fn max_delta(a: Option<String>, b: Option<String>) -> Option<String> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(x), Some(y)) => Some(if cmp_mag(&x, &y) == Ordering::Less { y } else { x }),
    }
}

/// The one `Tester` impl: holds the three validation strategies and routes to
/// them. Build with [`Validator::validation_tester`] / [`Validator::no_validation_tester`].
pub struct Validator<R, O, P> {
    pub rounding: R,
    pub overflow: O,
    pub precision: P,
}

impl<R: ValidateRounding, O: ValidateOverflow, P: ValidatePrecision> Tester for Validator<R, O, P> {
    fn validate_rounding(
        &self, got: &str, golden: &GoldenValue, width: u32, scale: u32, mode: RoundingMode,
    ) -> Outcome {
        self.rounding.validate_rounding(got, golden, width, scale, mode)
    }
    fn validate_overflow(
        &self, got: &CaseOutput, golden: &GoldenValue, width: u32, scale: u32, storage_bits: u32,
        overflow: Overflow,
    ) -> Outcome {
        self.overflow.validate_overflow(got, golden, width, scale, storage_bits, overflow)
    }
    fn validate_precision(
        &self, got: &str, golden: &GoldenValue, width: u32, scale: u32, mode: RoundingMode,
    ) -> Option<String> {
        self.precision.validate_precision(got, golden, width, scale, mode)
    }
    fn validates(&self) -> bool {
        self.rounding.active() || self.overflow.active() || self.precision.active()
    }
}

impl Validator<DefaultRounding, DefaultOverflow, DefaultPrecision> {
    /// All three checks active. `gen_precision` = the corpus generation precision.
    pub const fn validation_tester(
        gen_precision: usize,
    ) -> Validator<DefaultRounding, DefaultOverflow, DefaultPrecision> {
        Validator {
            rounding: DefaultRounding { gen_precision },
            overflow: DefaultOverflow,
            precision: DefaultPrecision { gen_precision },
        }
    }
}

impl Validator<NoOpRounding, NoOpOverflow, NoOpPrecision> {
    /// No checks (e.g. a pure-timing run): skips the format/eval pass entirely.
    pub const fn no_validation_tester() -> Validator<NoOpRounding, NoOpOverflow, NoOpPrecision> {
        Validator { rounding: NoOpRounding, overflow: NoOpOverflow, precision: NoOpPrecision }
    }
}

/// Run `function` over `subjects` with `tester` + `strategy` (serial). Delegates
/// to [`Tester::run`].
pub fn run(
    tester: &dyn Tester,
    strategy: &dyn ExecutionStrategy,
    subjects: &[Box<dyn Subject>],
    function: Function,
    cases: &[GoldenCase],
) -> Vec<ResultRecord> {
    tester.run(strategy, subjects, function, cases)
}

/// Parallel sibling of [`run`]. Delegates to [`Tester::run_parallel`].
pub fn run_parallel(
    tester: &(dyn Tester + Sync),
    strategy: &(dyn ExecutionStrategy + Sync),
    subjects: &[Box<dyn Subject + Sync>],
    function: Function,
    cases: &[GoldenCase],
    threads: usize,
) -> Vec<ResultRecord> {
    tester.run_parallel(strategy, subjects, function, cases, threads)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::RunOnce;
    use crate::subject::{Capabilities, FnSupport};
    use std::collections::BTreeMap;
    const GP: usize = 1233;

    fn caps_sqrt(name: &str) -> Capabilities {
        let mut functions = BTreeMap::new();
        functions.insert(
            Function::Sqrt,
            FnSupport { modes: vec![RoundingMode::HalfToEven], overflow: Overflow::Panic },
        );
        Capabilities { name: name.into(), width: 38, scale: 4, storage_bits: 128, functions }
    }

    struct Fixed(&'static str);
    impl Subject for Fixed {
        fn capabilities(&self) -> Capabilities {
            caps_sqrt("fixed")
        }
        fn eval(&self, _f: Function, _i: &[&str], _m: RoundingMode, _o: Overflow) -> Computed<String> {
            Computed::Value(self.0.to_string())
        }
        fn compute_thunk<'a>(
            &'a self, _c: &'a [GoldenCase], _f: Function, _m: RoundingMode, _o: Overflow,
        ) -> Option<Box<dyn FnMut() + 'a>> {
            None
        }
    }
    struct Panicker;
    impl Subject for Panicker {
        fn capabilities(&self) -> Capabilities {
            caps_sqrt("panicker")
        }
        fn eval(&self, _f: Function, _i: &[&str], _m: RoundingMode, _o: Overflow) -> Computed<String> {
            panic!("boom")
        }
        fn compute_thunk<'a>(
            &'a self, _c: &'a [GoldenCase], _f: Function, _m: RoundingMode, _o: Overflow,
        ) -> Option<Box<dyn FnMut() + 'a>> {
            None
        }
    }

    fn case(input: &str, out: &str) -> GoldenCase {
        GoldenCase { inputs: vec![input.to_string()], output_raw: out.to_string() }
    }

    #[test]
    fn correctness_pass() {
        let v = Validator::validation_tester(GP);
        let r = run_cell(
            &v, &Fixed("1.4142"), Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("fixed"), RoundingMode::HalfToEven, &RunOnce,
        );
        assert_eq!(r.outcome, Outcome::Pass);
        assert_eq!(r.library, "fixed");
        assert_eq!(r.precision.as_deref(), Some("0"));
    }

    #[test]
    fn correctness_mis_rounded() {
        let v = Validator::validation_tester(GP);
        let r = run_cell(
            &v, &Fixed("1.4140"), Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("fixed"), RoundingMode::HalfToEven, &RunOnce,
        );
        assert_eq!(r.outcome, Outcome::MisRounded { delta: "2".into() });
        assert_eq!(r.detail.as_deref(), Some("2"));
    }

    #[test]
    fn correctness_catches_panic() {
        let v = Validator::validation_tester(GP);
        let r = run_cell(
            &v, &Panicker, Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("panicker"), RoundingMode::HalfToEven, &RunOnce,
        );
        assert_eq!(r.outcome, Outcome::Panic);
    }

    #[test]
    fn no_validation_skips() {
        let v = Validator::no_validation_tester();
        assert!(!v.validates());
        let r = run_cell(
            &v, &Panicker, Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("panicker"), RoundingMode::HalfToEven, &RunOnce,
        );
        // never evaluated -> never panicked -> Skipped, no precision
        assert_eq!(r.outcome, Outcome::Skipped);
        assert_eq!(r.precision, None);
    }

    #[test]
    fn run_walks_subject_vec() {
        let v = Validator::validation_tester(GP);
        let subjects: Vec<Box<dyn Subject>> = vec![Box::new(Fixed("1.4142"))];
        let recs = run(&v, &RunOnce, &subjects, Function::Sqrt, &[case("2", "1.4142135")]);
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].outcome, Outcome::Pass);
    }

    #[test]
    fn parallel_matches_serial() {
        let v = Validator::validation_tester(GP);
        let par: Vec<Box<dyn Subject + Sync>> =
            vec![Box::new(Fixed("1.4142")), Box::new(Fixed("1.4140"))];
        let ser: Vec<Box<dyn Subject>> =
            vec![Box::new(Fixed("1.4142")), Box::new(Fixed("1.4140"))];
        let cases = [case("2", "1.4142135")];
        let p = run_parallel(&v, &RunOnce, &par, Function::Sqrt, &cases, 4);
        let s = run(&v, &RunOnce, &ser, Function::Sqrt, &cases);
        assert_eq!(p.len(), s.len());
        assert_eq!(p[0].outcome, s[0].outcome);
        assert_eq!(p[1].outcome, s[1].outcome);
    }
}
