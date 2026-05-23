//! Cross-cutting plumbing: rounding modes, error types, diagnostics,
//! text I/O, serde glue, bench-alt aliases.
//!
//! Everything here is consumed by the typed surface in [`crate::types`]
//! and the kernels in [`crate::algos`] without imposing a layering
//! dependency in the other direction.

#[cfg(feature = "bench-alt")]
pub(crate) mod bench_alt;
pub(crate) mod diagnostics;
pub(crate) mod display;
pub(crate) mod error;
pub(crate) mod int_fmt;
pub(crate) mod rounding;
#[cfg(feature = "serde")]
pub mod serde_helpers;
