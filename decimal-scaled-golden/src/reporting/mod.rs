//! Reporting. A `Reporter` consumes finished runs and renders named file outputs
//! (IO at the edge); `TsvReporter` emits the flattened per-cell TSV. An
//! `InlineReporter` instead streams a human-readable summary straight to a writer —
//! the live console feedback a gate or shootout prints after validation;
//! `ConsoleReporter` is the terminal impl.

mod inline;
mod reporting;
mod tsv;

pub use inline::{ConsoleReporter, InlineReporter, RunSummary};
pub use reporting::{ReportArtifact, ReportOutput, Reporter};
pub use tsv::TsvReporter;
