//! Mathematical constants (`pi`, `e`, `ln(2)`, …).
//!
//! The [`DecimalConstants`] trait carries the public surface; a single
//! generic impl in [`d38`] serves every width (narrow and wide),
//! sourced from the unified per-scale table in [`crate::consts`].

pub(crate) mod d38;

pub use d38::DecimalConstants;

// Re-export every `pub(crate)` helper from `d38` (the narrow-tier
// constant tables) so the macros emitted by `macros/consts.rs` can
// reach them through `$crate::types::consts::<name>` from the
// per-width impl invocations.
pub(crate) use d38::*;
