//! `TsvReporter` — flattens every cell of every run into one filterable TSV. Its
//! per-cell row shape is its own private concern; `width`/`scale` ride in from each
//! subject's `config` metadata (strings), nothing typed.

use crate::collector::{ExecutionCollector, ExecutionResult, RunCollector};
use crate::outcome::Outcome;
use crate::rounding::RoundingMode;
use crate::subject::Computed;

use super::reporting::{ReportArtifact, ReportOutput, Reporter};

pub struct TsvReporter;

impl Reporter for TsvReporter {
    fn report(&self, runs: &[RunCollector]) -> ReportArtifact {
        let mut out =
            String::from("library\tfunction\twidth\tscale\tmode\toutcome\tprecision\tdetail\tnanos\n");
        for run in runs {
            for subject in &run.subjects {
                let caps = &subject.capabilities;
                let width = caps.config.get("width").map(String::as_str).unwrap_or("");
                let scale = caps.config.get("scale").map(String::as_str).unwrap_or("");
                for fc in &subject.functions {
                    let mode = fc.support.map(|s| mode_name(s.mode)).unwrap_or("");
                    for cell in &fc.cells {
                        let row = row_for(cell);
                        out.push_str(&format!(
                            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                            caps.name, fc.function.name(), width, scale, mode,
                            row.outcome, row.precision, row.detail, row.nanos,
                        ));
                    }
                }
            }
        }
        ReportArtifact { outputs: vec![ReportOutput { name: "results.tsv".into(), content: out }] }
    }
}

/// The reporter's internal per-cell row.
struct Row {
    outcome: String,
    precision: String,
    detail: String,
    nanos: String,
}

fn row_for(cell: &ExecutionCollector) -> Row {
    let input = cell.inputs.join(",");
    let mut precision = String::new();
    let mut worst: Option<&Outcome> = None;
    for v in &cell.validations {
        if let Outcome::Precision { ulps } = v {
            precision = ulps.clone();
            continue;
        }
        worst = Some(match worst {
            Some(w) if w.severity() >= v.severity() => w,
            _ => v,
        });
    }
    let (outcome, detail) = match worst {
        Some(o) => (outcome_tag(o, cell.oracle_limited), detail_for(o, &input)),
        None => (status_tag(cell), input.clone()),
    };
    Row {
        outcome,
        precision,
        detail,
        nanos: cell.timing.map(|n| n.to_string()).unwrap_or_default(),
    }
}

fn outcome_tag(o: &Outcome, oracle_limited: bool) -> String {
    if oracle_limited && matches!(o, Outcome::Pass) {
        "pass-oracle-limited".to_string()
    } else {
        o.tag().to_string()
    }
}

fn detail_for(o: &Outcome, input: &str) -> String {
    match o {
        Outcome::MisRounded { delta } => format!("{input}={delta}"),
        Outcome::WrongMode { used } => format!("{input}=used:{}", mode_name(*used)),
        Outcome::Error { reason } => format!("{input}=err:{reason}"),
        _ => input.to_string(),
    }
}

/// For a cell with no validations (skipped, harness-error, or a timing-only run).
fn status_tag(cell: &ExecutionCollector) -> String {
    match cell.result() {
        Some(ExecutionResult::Skipped) => "skipped".to_string(),
        Some(ExecutionResult::HarnessError(_)) => "harness-error".to_string(),
        Some(ExecutionResult::Computed(c)) => computed_tag(c).to_string(),
        None => "pending".to_string(),
    }
}

fn computed_tag(c: &Computed<String>) -> &'static str {
    match c {
        Computed::Value(_) => "value",
        Computed::NonReal(_) => "non-real",
        Computed::Absent => "absent",
        Computed::Error(_) => "error",
        Computed::Timeout(_) => "timeout",
        Computed::Panic(_) => "panic",
    }
}

fn mode_name(m: RoundingMode) -> &'static str {
    match m {
        RoundingMode::HalfToEven => "HalfToEven",
        RoundingMode::HalfAwayFromZero => "HalfAwayFromZero",
        RoundingMode::HalfTowardZero => "HalfTowardZero",
        RoundingMode::Ceiling => "Ceiling",
        RoundingMode::Floor => "Floor",
        RoundingMode::Trunc => "Trunc",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collector::{FunctionCollector, SubjectCollector};
    use crate::function::Function;
    use crate::subject::{Capabilities, FnSupport, Overflow, Radix};
    use std::collections::BTreeMap;

    fn caps() -> Capabilities {
        let mut functions = BTreeMap::new();
        functions.insert(
            Function::Sqrt,
            FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
        );
        let mut config = BTreeMap::new();
        config.insert("width".into(), "38".into());
        config.insert("scale".into(), "19".into());
        Capabilities { name: "decimal-scaled".into(), radix: Radix::Decimal, config, functions }
    }

    #[test]
    fn writes_header_and_pass_row() {
        let mut fc = FunctionCollector::new(
            Function::Sqrt,
            FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
        );
        let mut cell = ExecutionCollector::new(vec!["2".into()], "1.41".into(), 0);
        cell.record(ExecutionResult::Computed(Computed::Value("1.41".into())));
        cell.add_validation(Outcome::Pass);
        fc.add(cell);
        let mut sc = SubjectCollector::new(caps());
        sc.add(fc);
        let mut run = RunCollector::new();
        run.add(sc);

        let art = TsvReporter.report(&[run]);
        let lines: Vec<&str> = art.outputs[0].content.lines().collect();
        assert_eq!(lines[0], "library\tfunction\twidth\tscale\tmode\toutcome\tprecision\tdetail\tnanos");
        assert_eq!(lines[1], "decimal-scaled\tsqrt\t38\t19\tHalfToEven\tpass\t\t2\t");
    }
}
