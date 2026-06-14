// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Exponential algorithm family.
//!
//! Narrow tier (D18 / D38) calls the `Fixed` 256-bit intermediate
//! kernels; wide tier (D57 .. D1232) runs the tier-generic
//! `exp_tang` / `*_series` kernels via `crate::policy::exp`.
//! Both tiers route through `crate::policy::exp`.
//!
//! Variants:
//!
//! - [`exp_series_2limb`] -- D38's `Fixed` 256-bit intermediate `exp_fixed`
//!   path, four-variant matrix entry shape.
//! - [`exp_schoolbook`] -- correctness reference: direct Maclaurin series
//!   with `ln(2)` range reduction, no Smith squarings.
//!   Registered as the unrouted `Algorithm::Schoolbook` variant.

/// Width-generic guard-digit `exp` core, generic over the work integer
/// `S: BigInt` (whose scratch carrier `S::Scratch` impls `ComputeLimbs`).
/// Always compiled (the narrow D18/D38 build
/// reaches it for the integer-regime large-result / sub-resolution cells
/// whose 256-bit `Fixed` intermediate cannot host the integer-digit lift),
/// and reused by the wide tiers' `Wexp` large-result path.
pub(crate) mod exp_generic;
pub(crate) mod exp_series_2limb;
/// Schoolbook exponential -- direct Maclaurin series correctness reference.
/// Registered as the unrouted `Algorithm::Schoolbook` arm; not connected
/// to `select`.
pub(crate) mod exp_schoolbook;
/// Tier-generic Tang-style table-driven `exp_strict` kernel, generic over
/// `WideTrigCore`. Collapses the per-tier D57 (18..=22 / 45..=56), D115
/// and D153 Tang exp kernels into one. The `policy::exp` Tang arms call
/// it with the tier's `Core`, `SCALE`, table size, narrow guard and the
/// per-band reduction/narrowing flags; the trig hyperbolic kernels reuse
/// [`exp_tang::tang_exp_fixed`] for their `(e^v, e^-v)` pair.
#[cfg(feature = "_wide-support")]
pub(crate) mod exp_tang;
