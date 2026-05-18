//! Cross-cutting plumbing: rounding modes, error types, diagnostics,
//! text I/O, serde glue, bench-alt aliases.
//!
//! Everything here is consumed by the typed surface in [`crate::types`]
//! and the kernels in [`crate::algos`] without imposing a layering
//! dependency in the other direction.
