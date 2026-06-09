//! The `Reporter` trait + the named-output artifact it produces.

use crate::collector::RunCollector;

/// One rendered output: a suggested name + its text content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportOutput {
    pub name: String,
    pub content: String,
}

/// What a reporter produces — one or more named outputs. The caller does the IO.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReportArtifact {
    pub outputs: Vec<ReportOutput>,
}

/// Renders a collection of run results into named outputs.
pub trait Reporter {
    fn report(&self, runs: &[RunCollector]) -> ReportArtifact;
}
