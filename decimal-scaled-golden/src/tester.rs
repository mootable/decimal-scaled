use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::bigdec::cmp_mag;
use crate::computed::Computed;
use crate::function::Function;
use crate::outcome::{Outcome, ResultRecord};
use crate::parser::GoldenCase;
use crate::rounding::RoundingMode;
use crate::subject::ErasedSubject;
use crate::validator::validate_one;
use crate::value::GoldenValue;

/// A tester runs one cell (all cases for a function at a fixed width/scale/mode)
/// against a subject and produces one `ResultRecord`.
pub trait Tester {
    fn name(&self) -> &str;
    fn run_cell(
        &self,
        subject: &dyn ErasedSubject,
        function: Function,
        cases: &[GoldenCase],
        width: u32,
        scale: u32,
        mode: RoundingMode,
        gen_precision: usize,
    ) -> ResultRecord;
}

/// Runs + validates every case; records the WORST outcome and the offending input.
pub struct CorrectnessTester;

impl Tester for CorrectnessTester {
    fn name(&self) -> &str { "correctness" }

    fn run_cell(
        &self,
        subject: &dyn ErasedSubject,
        function: Function,
        cases: &[GoldenCase],
        width: u32,
        scale: u32,
        mode: RoundingMode,
        gen_precision: usize,
    ) -> ResultRecord {
        let library = subject.capabilities(function).name;
        let mut worst = Outcome::Skipped;
        let mut detail: Option<String> = None;
        for case in cases {
            let golden = match GoldenValue::parse(&case.output_raw) {
                Some(g) => g,
                None => continue,
            };
            let refs: Vec<&str> = case.inputs.iter().map(|s| s.as_str()).collect();
            let outcome = match catch_unwind(AssertUnwindSafe(|| {
                subject.eval(function, &refs, width, scale, mode)
            })) {
                Err(_) => Outcome::Panic,
                Ok(Computed::Value(got)) => validate_one(&got, &golden, width, scale, mode, gen_precision),
                Ok(Computed::Skip) => Outcome::Skipped,
                Ok(Computed::Error(reason)) => Outcome::Error { reason },
            };
            if worse_than(&outcome, &worst) {
                worst = outcome;
                detail = Some(case.inputs.join(" "));
            }
        }
        if matches!(worst, Outcome::Pass | Outcome::Skipped) {
            detail = None;
        }
        ResultRecord { library, function, width, scale, mode, outcome: worst, detail, nanos: None }
    }
}

/// Times the whole cell (one warm-up, then a measured batch over all cases).
pub struct TimingTester {
    pub warmup: u32,
}

impl Tester for TimingTester {
    fn name(&self) -> &str { "timing" }

    fn run_cell(
        &self,
        subject: &dyn ErasedSubject,
        function: Function,
        cases: &[GoldenCase],
        width: u32,
        scale: u32,
        mode: RoundingMode,
        _gen_precision: usize,
    ) -> ResultRecord {
        let library = subject.capabilities(function).name;
        let batch: Vec<Vec<String>> = cases.iter().map(|c| c.inputs.clone()).collect();
        let timed = catch_unwind(AssertUnwindSafe(|| {
            subject.time_batch(function, &batch, width, scale, mode, self.warmup)
        }));
        let (outcome, nanos) = match timed {
            Err(_) => (Outcome::Panic, None),
            Ok(Some(ns)) => (Outcome::Pass, Some(ns)),
            Ok(None) => (Outcome::Skipped, None),
        };
        ResultRecord { library, function, width, scale, mode, outcome, detail: None, nanos }
    }
}

/// Strict "more severe than", breaking MisRounded ties by larger delta.
fn worse_than(a: &Outcome, b: &Outcome) -> bool {
    match (a, b) {
        (Outcome::MisRounded { delta: da }, Outcome::MisRounded { delta: db }) => {
            cmp_mag(da, db) == std::cmp::Ordering::Greater
        }
        _ => a.severity() > b.severity(),
    }
}

/// Sweep `(width, scale)` targets x `modes`, one record per cell. Cells beyond the
/// subject's capability are `Skipped` without calling `eval`/`time_batch`.
pub fn run(
    tester: &dyn Tester,
    subject: &dyn ErasedSubject,
    function: Function,
    cases: &[GoldenCase],
    targets: &[(u32, u32)],
    modes: &[RoundingMode],
    gen_precision: usize,
) -> Vec<ResultRecord> {
    let caps = subject.capabilities(function);
    let mut out = Vec::new();
    for &(width, scale) in targets {
        let cell_ok = caps.supported && width <= caps.max_width && scale <= caps.max_scale;
        for &mode in modes {
            if !(cell_ok && caps.rounding_modes.contains(&mode)) {
                out.push(ResultRecord {
                    library: caps.name.clone(), function, width, scale, mode,
                    outcome: Outcome::Skipped, detail: None, nanos: None,
                });
                continue;
            }
            out.push(tester.run_cell(subject, function, cases, width, scale, mode, gen_precision));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::computed::Computed;
    use crate::function::Function;
    use crate::parser::GoldenCase;
    use crate::rounding::RoundingMode;
    use crate::subject::{Capabilities, DecimalSubject};
    const GP: usize = 1233;

    struct Fixed(&'static str);
    impl DecimalSubject for Fixed {
        type Value = String;
        fn capabilities(&self, _f: Function) -> Capabilities {
            Capabilities { name: "fixed".into(), supported: true, max_width: 38, max_scale: 37,
                           rounding_modes: vec![RoundingMode::HalfToEven] }
        }
        fn to_text(&self, v: &String) -> String { v.clone() }
        fn from_text(&self, s: &str, _w: u32, _sc: u32) -> Computed<String> { Computed::Value(s.to_string()) }
        fn execute(&self, _f: Function, _i: &[String], _w: u32, _sc: u32, _m: RoundingMode)
            -> Computed<String> { Computed::Value(self.0.to_string()) }
    }
    struct Panicker;
    impl DecimalSubject for Panicker {
        type Value = String;
        fn capabilities(&self, _f: Function) -> Capabilities {
            Capabilities { name: "panicker".into(), supported: true, max_width: 38, max_scale: 37,
                           rounding_modes: vec![RoundingMode::HalfToEven] }
        }
        fn to_text(&self, v: &String) -> String { v.clone() }
        fn from_text(&self, s: &str, _w: u32, _sc: u32) -> Computed<String> { Computed::Value(s.to_string()) }
        fn execute(&self, _f: Function, _i: &[String], _w: u32, _sc: u32, _m: RoundingMode)
            -> Computed<String> { panic!("boom") }
    }

    fn case(input: &str, out: &str) -> GoldenCase {
        GoldenCase { inputs: vec![input.to_string()], output_raw: out.to_string() }
    }

    #[test]
    fn correctness_pass() {
        let r = CorrectnessTester.run_cell(&Fixed("1.4142"), Function::Sqrt,
            &[case("2", "1.4142135")], 38, 4, RoundingMode::HalfToEven, GP);
        assert_eq!(r.outcome, Outcome::Pass);
        assert_eq!(r.library, "fixed");
        assert_eq!(r.nanos, None);
    }
    #[test]
    fn correctness_mis_rounded_worst() {
        let r = CorrectnessTester.run_cell(&Fixed("1.4140"), Function::Sqrt,
            &[case("2", "1.4142135")], 38, 4, RoundingMode::HalfToEven, GP);
        assert_eq!(r.outcome, Outcome::MisRounded { delta: "2".into() });
        assert_eq!(r.detail.as_deref(), Some("2"));
    }
    #[test]
    fn correctness_catches_panic() {
        let r = CorrectnessTester.run_cell(&Panicker, Function::Sqrt,
            &[case("2", "1.4142135")], 38, 4, RoundingMode::HalfToEven, GP);
        assert_eq!(r.outcome, Outcome::Panic);
    }
    #[test]
    fn timing_records_nanos() {
        let r = TimingTester { warmup: 1 }.run_cell(&Fixed("1.4142"), Function::Sqrt,
            &[case("2", "1.4142135"), case("3", "1.7320508")], 38, 4, RoundingMode::HalfToEven, GP);
        assert!(r.nanos.is_some());
        assert_eq!(r.outcome, Outcome::Pass);
    }
    #[test]
    fn run_skips_unsupported() {
        let recs = run(&CorrectnessTester, &Fixed("1.4142"), Function::Sqrt,
            &[case("2", "1.4142135")], &[(38, 4), (76, 4)], &[RoundingMode::HalfToEven], GP);
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[0].outcome, Outcome::Pass);
        assert_eq!(recs[1].outcome, Outcome::Skipped);
    }
}
