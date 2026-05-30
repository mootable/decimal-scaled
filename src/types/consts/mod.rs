//! Mathematical constants (`pi`, `e`, and more).
//!
//! The [`DecimalConstants`] trait carries the public surface; a single
//! generic impl in [`decimal_constants`] serves every width (narrow and wide),
//! sourced from the unified per-scale table in [`crate::consts`].

pub(crate) mod decimal_constants;

// DecimalConstants is defined in types/traits/consts.rs; the impl lives in
// decimal_constants.rs. Re-export here to preserve the crate::types::consts
// path for in-crate consumers that still import from this module.
pub use crate::types::traits::DecimalConstants;

// Re-export PI_RAW and checked_storage from decimal_constants so the
// crate::types::consts::PI_RAW path remains reachable for trig kernels.
pub(crate) use decimal_constants::*;
