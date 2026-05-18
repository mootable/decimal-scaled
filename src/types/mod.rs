//! Type definitions, per-width aliases, and per-family method shells.
//!
//! This bucket holds the generic `D<S, SCALE>` newtype, the per-width
//! aliases (`D9`, `D18`, `D38`, …), the `DecimalConstants` constants
//! surface, the public-trait surface in [`traits`], and the per-family
//! inherent-impl shells.
//!
//! Lower-layer kernels live in [`crate::algos`] and routing lives in
//! [`crate::policy`]; this bucket is the typed surface that calls into
//! them.

pub(crate) mod traits;
