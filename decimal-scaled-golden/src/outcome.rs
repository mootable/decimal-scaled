use crate::function::Function;
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
    /// The library gracefully signalled failure on an input it should have handled.
    Error { reason: String },
    /// The subject exceeded its time budget (produced in Phase 5).
    Timeout,
    /// The subject panicked.
    Panic,
}

impl Outcome {
    /// "Worst across a cell" order: Skipped < Pass < WrongMode < MisRounded <
    /// Error < Timeout < Panic. (MisRounded ties broken by delta size at the call site.)
    pub fn severity(&self) -> u8 {
        match self {
            Outcome::Skipped => 0,
            Outcome::Pass => 1,
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
            Outcome::Error { .. } => "error",
            Outcome::Timeout => "timeout",
            Outcome::Panic => "panic",
        }
    }
}

/// One unified record — serves both the precision shootout and the timing bench.
#[derive(Clone, Debug, PartialEq)]
pub struct ResultRecord {
    pub library: String,
    pub function: Function,
    pub width: u32,
    pub scale: u32,
    pub mode: RoundingMode,
    /// The strict result: rounding (when the value fits) or overflow (when it
    /// doesn't) — whichever applied. `Pass` when the relevant validator is NoOp.
    pub outcome: Outcome,
    /// Graded precision distance (ULP magnitude) when `ValidatePrecision` is
    /// active; `None` when it's NoOp or not applicable.
    pub precision: Option<String>,
    /// Correctness: the offending input (for a failure). Timing: None.
    pub detail: Option<String>,
    /// Timing: batch nanoseconds. Correctness: None.
    pub nanos: Option<u64>,
}
