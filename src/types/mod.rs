// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Type definitions, per-width aliases, and per-family method shells.
//!
//! This bucket holds the generic `D<S, SCALE>` newtype, the per-width
//! aliases (`D18`, `D38`, …), the `DecimalConstants` constants
//! surface, the public-trait surface in [`traits`], and the per-family
//! inherent-impl shells (`log_exp`, `trig`, `powers`, …).
//!
//! Lower-layer kernels live in [`crate::algos`] and routing lives in
//! [`crate::policy`]; this bucket is the typed surface that calls into
//! them.

pub(crate) mod traits;

pub(crate) mod consts;
pub(crate) mod unified;
pub(crate) mod widths;

pub(crate) mod num_traits;
pub(crate) mod quantize;

// The integer-only transcendental shells.
pub(crate) mod log_exp;
pub(crate) mod powers;
pub(crate) mod trig;

// `checked_*` siblings of the transcendental family — one generic impl
// over `(N, SCALE)`.
pub(crate) mod checked_transcendentals;
