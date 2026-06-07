use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::bigdec::{abs_diff, cmp_mag};
use crate::computed::Computed;
use crate::function::Function;
use crate::outcome::{Outcome, ResultRecord};
use crate::overflow::expected_overflow;
use crate::parser::GoldenCase;
use crate::rounding::RoundingMode;
use crate::subject::{Capabilities, ErasedSubject, Overflow};
use crate::validator::validate_one;
use crate::value::GoldenValue;

/// A tester runs one cell (all cases for a function at the subject's fixed
/// width/scale, one rounding mode) against a subject and produces one
/// `ResultRecord`. The cell's width/scale/storage/overflow come from the
/// subject's `capabilities()`.
pub trait Tester {
    fn name(&self) -> &str;
    fn run_cell(
        &self,
        subject: &dyn ErasedSubject,
        function: Function,
        cases: &[GoldenCase],
        caps: &Capabilities,
        mode: RoundingMode,
        gen_precision: usize,
    ) -> ResultRecord;
}

/// Runs + validates every case; records the WORST outcome and the offending
/// input. A result that fits the cell is checked for correct rounding; a result
/// that overflows the cell is validated against the subject's declared
/// [`Overflow`] policy (never silently skipped).
pub struct CorrectnessTester;

impl Tester for CorrectnessTester {
    fn name(&self) -> &str { "correctness" }

    fn run_cell(
        &self,
        subject: &dyn ErasedSubject,
        function: Function,
        cases: &[GoldenCase],
        caps: &Capabilities,
        mode: RoundingMode,
        gen_precision: usize,
    ) -> ResultRecord {
        let (width, scale, storage_bits) = (caps.width, caps.scale, caps.storage_bits);
        let overflow = caps
            .function(function)
            .map(|f| f.overflow)
            .unwrap_or(Overflow::Panic);
        let mut worst = Outcome::Skipped;
        let mut detail: Option<String> = None;
        for case in cases {
            let golden = match GoldenValue::parse(&case.output_raw) {
                Some(g) => g,
                None => continue,
            };
            let refs: Vec<&str> = case.inputs.iter().map(|s| s.as_str()).collect();
            let outcome = if golden.fits(width, scale) {
                match catch_unwind(AssertUnwindSafe(|| {
                    subject.eval(function, &refs, mode, overflow)
                })) {
                    Err(_) => Outcome::Panic,
                    Ok(Computed::Value(got)) => {
                        validate_one(&got, &golden, width, scale, mode, gen_precision)
                    }
                    Ok(Computed::Skip) => Outcome::Skipped,
                    Ok(Computed::Error(reason)) => Outcome::Error { reason },
                }
            } else {
                validate_overflow(
                    subject, function, &refs, mode, overflow, &golden, width, scale, storage_bits,
                )
            };
            if worse_than(&outcome, &worst) {
                worst = outcome;
                detail = Some(case.inputs.join(" "));
            }
        }
        if matches!(worst, Outcome::Pass | Outcome::Skipped) {
            detail = None;
        }
        ResultRecord {
            library: caps.name.clone(), function, width, scale, mode,
            outcome: worst, detail, nanos: None,
        }
    }
}

/// Validate an overflowing cell against the declared [`Overflow`] policy:
/// `Panic` expects a panic; `Saturate`/`Truncate`/`Wrap` expect a specific
/// value the harness derives independently from the true result.
#[allow(clippy::too_many_arguments)]
fn validate_overflow(
    subject: &dyn ErasedSubject,
    function: Function,
    refs: &[&str],
    mode: RoundingMode,
    overflow: Overflow,
    golden: &GoldenValue,
    width: u32,
    scale: u32,
    storage_bits: u32,
) -> Outcome {
    let expected = expected_overflow(golden, width, scale, storage_bits, overflow);
    match catch_unwind(AssertUnwindSafe(|| subject.eval(function, refs, mode, overflow))) {
        // A panic is correct iff the subject declared it overflows by panicking.
        Err(_) => {
            if overflow == Overflow::Panic { Outcome::Pass } else { Outcome::Panic }
        }
        // The input itself isn't representable at this cell -> nothing to check.
        Ok(Computed::Skip) => Outcome::Skipped,
        Ok(Computed::Error(reason)) => Outcome::Error { reason },
        Ok(Computed::Value(got)) => match expected {
            None => Outcome::Error {
                reason: "declared overflow=Panic but returned a value".to_string(),
            },
            Some(exp) => match GoldenValue::parse(&got) {
                Some(g) => {
                    let got_scaled = g.round_to(scale, RoundingMode::Trunc, false);
                    if got_scaled == exp {
                        Outcome::Pass
                    } else {
                        Outcome::MisRounded { delta: abs_diff(&got_scaled, &exp) }
                    }
                }
                None => Outcome::MisRounded { delta: "nan".to_string() },
            },
        },
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
        caps: &Capabilities,
        mode: RoundingMode,
        _gen_precision: usize,
    ) -> ResultRecord {
        let overflow = caps
            .function(function)
            .map(|f| f.overflow)
            .unwrap_or(Overflow::Panic);
        let batch: Vec<Vec<String>> = cases.iter().map(|c| c.inputs.clone()).collect();
        let timed = catch_unwind(AssertUnwindSafe(|| {
            subject.time_batch(function, &batch, mode, overflow, self.warmup)
        }));
        let (outcome, nanos) = match timed {
            Err(_) => (Outcome::Panic, None),
            Ok(Some(ns)) => (Outcome::Pass, Some(ns)),
            Ok(None) => (Outcome::Skipped, None),
        };
        ResultRecord {
            library: caps.name.clone(), function, width: caps.width, scale: caps.scale, mode,
            outcome, detail: None, nanos,
        }
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

/// Run `function` over a list of subjects: each subject is one cell, so for each
/// subject that supports the function we run its declared modes, one record per
/// `(subject, mode)`. A subject that doesn't support the function contributes no
/// records. Serial — use for timing.
pub fn run(
    tester: &dyn Tester,
    subjects: &[Box<dyn ErasedSubject>],
    function: Function,
    cases: &[GoldenCase],
    gen_precision: usize,
) -> Vec<ResultRecord> {
    let mut out = Vec::new();
    for subject in subjects {
        let caps = subject.capabilities();
        let modes = match caps.function(function) {
            Some(s) => s.modes.clone(),
            None => continue,
        };
        for mode in modes {
            out.push(tester.run_cell(subject.as_ref(), function, cases, &caps, mode, gen_precision));
        }
    }
    out
}

/// Parallel sibling of [`run`]: distributes `(subject, mode)` work items across
/// `threads` workers via a shared atomic index (dynamic load-balancing across
/// the uneven per-cell cost). `threads <= 1` runs serially.
///
/// USE FOR CORRECTNESS ONLY. Timing / bench comparisons MUST go through the
/// serial [`run`] — concurrent workers contend for the CPU and corrupt the
/// measured nanoseconds, so `TimingTester` is only ever driven by `run`. The
/// per-item work is the same `tester.run_cell` the serial path uses, so swapping
/// in a pool (e.g. rayon) would replace only the dispatch loop.
///
/// Subjects + tester must be `Sync` to share across the scope (a stateless
/// adapter is trivially so). Records are returned in stable `(subject, mode)`
/// order, identical to `run`.
pub fn run_parallel(
    tester: &(dyn Tester + Sync),
    subjects: &[Box<dyn ErasedSubject + Sync>],
    function: Function,
    cases: &[GoldenCase],
    gen_precision: usize,
    threads: usize,
) -> Vec<ResultRecord> {
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
                tester.run_cell(subjects[i].as_ref(), function, cases, &caps[i], m, gen_precision)
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
                        let k = next_ref.fetch_add(1, Ordering::Relaxed);
                        if k >= work_ref.len() {
                            break;
                        }
                        let (i, m) = work_ref[k];
                        local.push((
                            k,
                            tester.run_cell(
                                subjects[i].as_ref(), function, cases, &caps_ref[i], m, gen_precision,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subject::{DecimalSubject, FnSupport};
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
    impl DecimalSubject for Fixed {
        type Value = String;
        fn capabilities(&self) -> Capabilities { caps_sqrt("fixed") }
        fn from_text(&self, s: &str) -> Computed<String> { Computed::Value(s.to_string()) }
        fn to_text(&self, v: &String) -> String { v.clone() }
        fn execute(&self, _f: Function, _i: &[String], _m: RoundingMode, _o: Overflow)
            -> Computed<String> { Computed::Value(self.0.to_string()) }
    }
    struct Panicker;
    impl DecimalSubject for Panicker {
        type Value = String;
        fn capabilities(&self) -> Capabilities { caps_sqrt("panicker") }
        fn from_text(&self, s: &str) -> Computed<String> { Computed::Value(s.to_string()) }
        fn to_text(&self, v: &String) -> String { v.clone() }
        fn execute(&self, _f: Function, _i: &[String], _m: RoundingMode, _o: Overflow)
            -> Computed<String> { panic!("boom") }
    }

    fn case(input: &str, out: &str) -> GoldenCase {
        GoldenCase { inputs: vec![input.to_string()], output_raw: out.to_string() }
    }

    #[test]
    fn correctness_pass() {
        let r = CorrectnessTester.run_cell(
            &Fixed("1.4142"), Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("fixed"), RoundingMode::HalfToEven, GP,
        );
        assert_eq!(r.outcome, Outcome::Pass);
        assert_eq!(r.library, "fixed");
        assert_eq!(r.nanos, None);
    }
    #[test]
    fn correctness_mis_rounded_worst() {
        let r = CorrectnessTester.run_cell(
            &Fixed("1.4140"), Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("fixed"), RoundingMode::HalfToEven, GP,
        );
        assert_eq!(r.outcome, Outcome::MisRounded { delta: "2".into() });
        assert_eq!(r.detail.as_deref(), Some("2"));
    }
    #[test]
    fn correctness_catches_panic() {
        let r = CorrectnessTester.run_cell(
            &Panicker, Function::Sqrt, &[case("2", "1.4142135")],
            &caps_sqrt("panicker"), RoundingMode::HalfToEven, GP,
        );
        assert_eq!(r.outcome, Outcome::Panic);
    }
    #[test]
    fn run_walks_subject_vec() {
        let subjects: Vec<Box<dyn ErasedSubject>> = vec![Box::new(Fixed("1.4142"))];
        let recs = run(&CorrectnessTester, &subjects, Function::Sqrt, &[case("2", "1.4142135")], GP);
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].outcome, Outcome::Pass);
    }
    #[test]
    fn run_skips_unsupported_function() {
        let subjects: Vec<Box<dyn ErasedSubject>> = vec![Box::new(Fixed("1.4142"))];
        // Fixed supports only Sqrt -> Exp yields no records.
        let recs = run(&CorrectnessTester, &subjects, Function::Exp, &[case("2", "1.0")], GP);
        assert!(recs.is_empty());
    }
    #[test]
    fn parallel_matches_serial() {
        let subjects: Vec<Box<dyn ErasedSubject + Sync>> =
            vec![Box::new(Fixed("1.4142")), Box::new(Fixed("1.4140"))];
        let serial: Vec<Box<dyn ErasedSubject>> =
            vec![Box::new(Fixed("1.4142")), Box::new(Fixed("1.4140"))];
        let cases = [case("2", "1.4142135")];
        let p = run_parallel(&CorrectnessTester, &subjects, Function::Sqrt, &cases, GP, 4);
        let s = run(&CorrectnessTester, &serial, Function::Sqrt, &cases, GP);
        assert_eq!(p.len(), s.len());
        assert_eq!(p[0].outcome, s[0].outcome);
        assert_eq!(p[1].outcome, s[1].outcome);
    }
    #[test]
    fn timing_records_nanos() {
        let r = TimingTester { warmup: 1 }.run_cell(
            &Fixed("1.4142"), Function::Sqrt,
            &[case("2", "1.4142135"), case("3", "1.7320508")],
            &caps_sqrt("fixed"), RoundingMode::HalfToEven, GP,
        );
        assert!(r.nanos.is_some());
        assert_eq!(r.outcome, Outcome::Pass);
    }
}
