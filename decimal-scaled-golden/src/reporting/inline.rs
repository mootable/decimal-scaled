// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `InlineReporter` — the console-streaming counterpart of [`Reporter`].
//!
//! Where a [`Reporter`](crate::reporting::Reporter) renders named file artifacts
//! for the caller to persist, an `InlineReporter` writes a human-readable summary
//! of the finished run straight to a writer (typically `stderr`) — the live
//! console feedback a gate or a shootout prints right after validation. It reads
//! the same verdicts the validators wrote into each
//! [`ExecutionCollector`](crate::collector::ExecutionCollector).

use std::io::{self, Write};

use crate::collector::{ExecutionResult, RunCollector};
use crate::outcome::Outcome;
use crate::subject::Capabilities;

/// Tallied verdict counts over one or more runs. `bad` is every non-`Pass`,
/// non-`Precision`, non-`Panic` verdict (mis-rounded / wrong-mode / error /
/// timeout); `panic` is counted separately so a gate can distinguish an
/// out-of-contract panic from a rounding miss.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunSummary {
    pub pass: usize,
    pub skip: usize,
    pub precision: usize,
    pub bad: usize,
    pub panic: usize,
}

impl RunSummary {
    /// Walk the collectors and tally every cell's verdicts.
    pub fn tally(runs: &[RunCollector]) -> RunSummary {
        let mut s = RunSummary::default();
        for run in runs {
            for subject in &run.subjects {
                s.add(subject_summary(subject));
            }
        }
        s
    }

    fn add(&mut self, o: RunSummary) {
        self.pass += o.pass;
        self.skip += o.skip;
        self.precision += o.precision;
        self.bad += o.bad;
        self.panic += o.panic;
    }
}

impl std::fmt::Display for RunSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} pass / {} skip / {} precision / {} bad / {} panic",
            self.pass, self.skip, self.precision, self.bad, self.panic
        )
    }
}

/// Streams a human-readable summary of finished runs to a writer. The live
/// counterpart to [`Reporter`](crate::reporting::Reporter): it does the IO itself
/// (rather than returning an artifact) and hands back the [`RunSummary`] so a gate
/// can assert on the counts in the same pass.
pub trait InlineReporter {
    fn report(&self, runs: &[RunCollector], out: &mut dyn Write) -> io::Result<RunSummary>;
}

/// A plain-text [`InlineReporter`] for the terminal. Two knobs cover both callers:
/// the 0-bad **gate** wants the few failing cells listed but not 500 all-pass
/// tallies (`per_subject = false`, `list_failures = true`), while the competitor
/// **shootout** wants every subject's numbers but not its thousands of expected
/// mis-rounds (`per_subject = true`, `list_failures = false`).
#[derive(Clone, Copy, Debug)]
pub struct ConsoleReporter {
    /// Print a one-line tally for every subject (not just failing ones).
    pub per_subject: bool,
    /// List each failing cell (mis-round / wrong-mode / error / panic) under its
    /// subject, with the golden line and inputs.
    pub list_failures: bool,
}

impl ConsoleReporter {
    /// The 0-bad gate preset: list the failing cells, suppress all-pass tallies.
    pub fn gate() -> ConsoleReporter {
        ConsoleReporter { per_subject: false, list_failures: true }
    }

    /// The shootout preset: one tally line per subject, no per-cell failure dump.
    pub fn shootout() -> ConsoleReporter {
        ConsoleReporter { per_subject: true, list_failures: false }
    }
}

impl Default for ConsoleReporter {
    fn default() -> ConsoleReporter {
        ConsoleReporter::gate()
    }
}

impl InlineReporter for ConsoleReporter {
    fn report(&self, runs: &[RunCollector], out: &mut dyn Write) -> io::Result<RunSummary> {
        let mut total = RunSummary::default();
        for run in runs {
            for subject in &run.subjects {
                let s = subject_summary(subject);
                total.add(s);
                let failing = s.bad + s.panic > 0;
                if self.per_subject || (self.list_failures && failing) {
                    writeln!(out, "{}: {}", subject_label(&subject.capabilities), s)?;
                }
                if self.list_failures && failing {
                    for fc in &subject.functions {
                        for cell in &fc.cells {
                            if matches!(cell.result(), Some(ExecutionResult::Skipped)) {
                                continue;
                            }
                            for o in &cell.validations {
                                if is_failure(o) {
                                    writeln!(
                                        out,
                                        "  {} {} [{}.au:{}] {} inputs={:?}",
                                        o.tag(),
                                        fc.function.name(),
                                        fc.function.name(),
                                        cell.line,
                                        outcome_detail(o),
                                        cell.inputs,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }
        writeln!(out, "TOTAL: {total}")?;
        Ok(total)
    }
}

/// A cell verdict that should be listed/counted as a failure (everything except
/// `Pass`, the informational `Precision`, and the runner-level `Skipped`).
fn is_failure(o: &Outcome) -> bool {
    !matches!(o, Outcome::Pass | Outcome::Precision { .. } | Outcome::Skipped)
}

/// Per-subject tally: a `Skipped` cell counts once; otherwise each verdict counts.
fn subject_summary(subject: &crate::collector::SubjectCollector) -> RunSummary {
    let mut s = RunSummary::default();
    for fc in &subject.functions {
        for cell in &fc.cells {
            if matches!(cell.result(), Some(ExecutionResult::Skipped)) {
                s.skip += 1;
                continue;
            }
            for o in &cell.validations {
                match o {
                    Outcome::Pass => s.pass += 1,
                    Outcome::Skipped => s.skip += 1,
                    Outcome::Precision { .. } => s.precision += 1,
                    Outcome::Panic => s.panic += 1,
                    _ => s.bad += 1,
                }
            }
        }
    }
    s
}

/// `name [k=v, k=v]` from the capabilities — the config (width/scale/mode, …) is a
/// `BTreeMap`, so the keys render in a stable order.
fn subject_label(caps: &Capabilities) -> String {
    if caps.config.is_empty() {
        caps.name.clone()
    } else {
        let cfg: Vec<String> = caps.config.iter().map(|(k, v)| format!("{k}={v}")).collect();
        format!("{} [{}]", caps.name, cfg.join(", "))
    }
}

/// The variant-specific detail for a failing verdict (`tag()` gives the kind).
fn outcome_detail(o: &Outcome) -> String {
    match o {
        Outcome::WrongMode { used } => format!("(used {used:?})"),
        Outcome::MisRounded { delta } => format!("(delta {delta})"),
        Outcome::Error { reason } => format!("({reason})"),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::{ExecutionCollector, FunctionCollector, SubjectCollector};
    use crate::function::Function;
    use crate::rounding::RoundingMode;
    use crate::subject::{Capabilities, Computed, FnSupport, Overflow, Radix};
    use std::collections::BTreeMap;

    fn run_with(outcomes: Vec<Outcome>) -> RunCollector {
        let mut cfg = BTreeMap::new();
        cfg.insert("scale".to_string(), "2".to_string());
        let caps =
            Capabilities { name: "sub".into(), radix: Radix::Decimal, config: cfg, functions: BTreeMap::new() };
        let mut fc = FunctionCollector::new(
            Function::Sqrt,
            FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
        );
        for o in outcomes {
            let mut cell = ExecutionCollector::new(vec!["2".into()], "1.41".into(), 7);
            cell.record(ExecutionResult::Computed(Computed::Value("1.41".into())));
            cell.add_validation(o);
            fc.add(cell);
        }
        let mut sc = SubjectCollector::new(caps);
        sc.add(fc);
        let mut rc = RunCollector::new();
        rc.add(sc);
        rc
    }

    #[test]
    fn tally_counts_each_verdict() {
        let rc = run_with(vec![
            Outcome::Pass,
            Outcome::Pass,
            Outcome::Precision { ulps: "0".into() },
            Outcome::MisRounded { delta: "1".into() },
            Outcome::Panic,
        ]);
        let s = RunSummary::tally(&[rc]);
        assert_eq!((s.pass, s.precision, s.bad, s.panic), (2, 1, 1, 1));
    }

    #[test]
    fn gate_lists_failures_and_returns_summary() {
        let rc = run_with(vec![Outcome::Pass, Outcome::MisRounded { delta: "5".into() }]);
        let mut out = Vec::new();
        let s = ConsoleReporter::gate().report(&[rc], &mut out).unwrap();
        assert_eq!((s.pass, s.bad), (1, 1));
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("mis-rounded"), "lists the failing verdict: {text}");
        assert!(text.contains("scale=2"), "subject label carries config: {text}");
        assert!(text.contains("TOTAL:"), "prints a total: {text}");
    }

    #[test]
    fn shootout_lists_every_subject_but_no_cells() {
        let rc = run_with(vec![Outcome::Pass, Outcome::MisRounded { delta: "5".into() }]);
        let mut out = Vec::new();
        ConsoleReporter::shootout().report(&[rc], &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("sub [scale=2]:"), "one tally line per subject: {text}");
        assert!(!text.contains("mis-rounded"), "no per-cell dump in shootout: {text}");
    }
}
