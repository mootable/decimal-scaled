// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Type definitions, per-width aliases, and per-family method shells.
//!
//! This bucket holds the generic `D<S, SCALE>` newtype, the per-width
//! aliases (`D18`, `D38`, …), the `DecimalConstants` constants
//! surface, the public-trait surface in [`traits`], and the per-family
//! inherent-impl shells (`arithmetic`, `overflow_variants`,
//! `log_exp`, `trig`, `powers`, …).
//!
//! Lower-layer kernels live in [`crate::algos`] and routing lives in
//! [`crate::policy`]; this bucket is the typed surface that calls into
//! them.

pub(crate) mod traits;

pub(crate) mod consts;
pub(crate) mod unified;
pub(crate) mod widths;

pub(crate) mod arithmetic;
pub(crate) mod num_traits;
pub(crate) mod overflow_variants;
pub(crate) mod rescale;

// Strict (integer-only) transcendental shells. Strict is the crate's
// default; the lossy f64-bridge variants below carry the explicit
// `_fast` suffix to mark them as the opt-in.
pub(crate) mod log_exp;
pub(crate) mod powers;
#[cfg(any(not(feature = "fast"), feature = "std"))]
pub(crate) mod trig;

// Fast (f64-bridge) transcendental shells.
pub(crate) mod log_exp_fast;
pub(crate) mod powers_fast;
pub(crate) mod trig_fast;
