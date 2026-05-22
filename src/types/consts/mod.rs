//! Mathematical constants (`pi`, `e`, `ln(2)`, …).
//!
//! The [`DecimalConstants`] trait carries the public surface; per-width
//! impls live in [`d38`] (narrow tier — D18 / D38) and [`wide`]
//! (wide tier — D57 and above).

pub(crate) mod d38;
pub(crate) mod wide;

pub use d38::DecimalConstants;

// Re-export every `pub(crate)` helper from `d38` (the narrow-tier
// constant tables) so the macros emitted by `macros/consts.rs` can
// reach them through `$crate::types::consts::<name>` from the
// per-width impl invocations.
pub(crate) use d38::*;
