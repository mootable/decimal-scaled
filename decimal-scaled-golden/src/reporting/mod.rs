//! Reporting. One pluggable `Reporter` consumes a collection of finished runs and
//! renders one or more named outputs — IO stays at the edge. `TsvReporter` emits the
//! flattened per-cell TSV.

mod reporting;
mod tsv;

pub use reporting::{ReportArtifact, ReportOutput, Reporter};
pub use tsv::TsvReporter;
