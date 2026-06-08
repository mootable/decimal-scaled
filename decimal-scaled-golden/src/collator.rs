use crate::outcome::{Outcome, ResultRecord};
use crate::rounding::RoundingMode;

/// Collects `ResultRecord`s and renders them as one filterable TSV.
#[derive(Default)]
pub struct Collator {
    records: Vec<ResultRecord>,
}

impl Collator {
    pub fn new() -> Collator { Collator::default() }
    pub fn push(&mut self, r: ResultRecord) { self.records.push(r); }
    pub fn records(&self) -> &[ResultRecord] { &self.records }

    /// One header row + one row per record. `detail` carries
    /// `<input>=<delta|used:Mode|err:reason>` for failures, the bare input otherwise.
    pub fn to_tsv(&self) -> String {
        let mut out =
            String::from("library\tfunction\twidth\tscale\tmode\toutcome\tprecision\tdetail\tnanos\n");
        for r in &self.records {
            let input = r.detail.as_deref().unwrap_or("");
            let detail = match &r.outcome {
                Outcome::MisRounded { delta } => format!("{input}={delta}"),
                Outcome::WrongMode { used } => format!("{input}=used:{}", mode_name(*used)),
                Outcome::Error { reason } => format!("{input}=err:{reason}"),
                _ => r.detail.clone().unwrap_or_default(),
            };
            let precision = r.precision.clone().unwrap_or_default();
            let nanos = r.nanos.map(|n| n.to_string()).unwrap_or_default();
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                r.library, r.function.name(), r.width, r.scale,
                mode_name(r.mode), r.outcome.tag(), precision, detail, nanos,
            ));
        }
        out
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
    use crate::function::Function;
    use crate::outcome::{Outcome, ResultRecord};
    use crate::rounding::RoundingMode;

    fn rec(lib: &str, f: Function, outcome: Outcome, detail: Option<&str>, nanos: Option<u64>) -> ResultRecord {
        ResultRecord { library: lib.into(), function: f, width: 38, scale: 19,
            mode: RoundingMode::HalfToEven, outcome, precision: None,
            detail: detail.map(|s| s.into()), nanos }
    }

    #[test]
    fn writes_header_and_rows() {
        let mut c = Collator::new();
        c.push(rec("decimal-scaled", Function::Sqrt, Outcome::Pass, None, None));
        c.push(rec("rust_decimal", Function::Tan, Outcome::MisRounded { delta: "67".into() }, Some("1.3"), None));
        c.push(rec("g_math", Function::Exp, Outcome::Error { reason: "overflow".into() }, Some("700"), None));
        let lines: Vec<String> = c.to_tsv().lines().map(|s| s.to_string()).collect();
        assert_eq!(lines[0], "library\tfunction\twidth\tscale\tmode\toutcome\tprecision\tdetail\tnanos");
        assert_eq!(lines[1], "decimal-scaled\tsqrt\t38\t19\tHalfToEven\tpass\t\t\t");
        assert_eq!(lines[2], "rust_decimal\ttan\t38\t19\tHalfToEven\tmis-rounded\t\t1.3=67\t");
        assert_eq!(lines[3], "g_math\texp\t38\t19\tHalfToEven\terror\t\t700=err:overflow\t");
    }
}
