//! Mathematical constants (`pi`, `e`, `ln(2)`, …).
//!
//! The [`DecimalConstants`] trait carries the public surface; per-width
//! impls live in [`d38`] (narrow tier — D9 / D18 / D38) and [`wide`]
//! (wide tier — D57 and above).

pub(crate) mod d38;
pub(crate) mod wide;

pub use d38::DecimalConstants;
#[allow(deprecated)]
pub use d38::DecimalConsts;

// Re-export every `pub(crate)` helper from `d38` (the narrow-tier
// constant tables) so the macros emitted by `macros/consts.rs` can
// reach them through `$crate::consts::<name>`. Path-shimmed for one
// release cycle; PR 3 sweeps to the final paths.
pub(crate) use d38::*;
