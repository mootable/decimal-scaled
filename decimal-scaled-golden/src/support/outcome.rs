use crate::rounding::RoundingMode;

/// What happened for one trial (one input, or aggregated, one cell).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    /// Wrong under every mode; `delta` = decimal ULP distance at the target scale.
    MisRounded { delta: String },
    /// Wrong for the claimed mode, but matches another mode's correct rounding.
    WrongMode { used: RoundingMode },
    /// Not applicable (didn't fit the tier, or the subject doesn't expose the cell).
    Skipped,
    /// Informational ULP distance of a fitting result from the correctly-rounded
    /// value (the precision shootout's measure, not a pass/fail verdict).
    Precision { ulps: String },
    /// The library gracefully signalled failure on an input it should have handled.
    Error { reason: String },
    /// The subject exceeded its time budget (produced in Phase 5).
    Timeout,
    /// The subject panicked.
    Panic,
}

impl Outcome {
    /// "Worst across a cell" order: Skipped < Pass/Precision < WrongMode <
    /// MisRounded < Error < Timeout < Panic. (`Precision` is informational, so it
    /// ranks with `Pass` and never dominates a real failure; MisRounded ties are
    /// broken by delta size at the call site.)
    pub fn severity(&self) -> u8 {
        match self {
            Outcome::Skipped => 0,
            Outcome::Pass => 1,
            Outcome::Precision { .. } => 1,
            Outcome::WrongMode { .. } => 2,
            Outcome::MisRounded { .. } => 3,
            Outcome::Error { .. } => 4,
            Outcome::Timeout => 5,
            Outcome::Panic => 6,
        }
    }
    pub fn tag(&self) -> &'static str {
        match self {
            Outcome::Pass => "pass",
            Outcome::MisRounded { .. } => "mis-rounded",
            Outcome::WrongMode { .. } => "wrong-mode",
            Outcome::Skipped => "skipped",
            Outcome::Precision { .. } => "precision",
            Outcome::Error { .. } => "error",
            Outcome::Timeout => "timeout",
            Outcome::Panic => "panic",
        }
    }
}
