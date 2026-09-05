// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Natural-logarithm algorithm family.
//!
//! Narrow tier (D18 / D38) calls the `Fixed` 256-bit intermediate
//! kernels; wide tier (D57 .. D1232) runs the tier-generic `ln_tang` /
//! `*_series` kernels via `crate::policy::ln`.
//! Both tiers route through `crate::policy::ln`.
//!
//! Variants:
//!
//! - [`ln_series_2limb`] -- D38's hand-tuned ln on the 256-bit `Fixed`
//!   intermediate with the configurable working-scale guard. Carries
//!   the four-variant matrix entry points (strict mode + approximation
//!   mode, each with an explicit-rounding sibling).
//! - [`ln_schoolbook`] -- correctness reference: atanh series with binary
//!   exponent split. Registered as the unrouted `Algorithm::Schoolbook` variant.

pub(crate) mod ln_series_2limb;
/// Schoolbook natural logarithm -- atanh series correctness reference.
/// Registered as the unrouted `Algorithm::Schoolbook` arm; not connected
/// to `select`.
pub(crate) mod ln_schoolbook;

/// Width-generic Tang-style table-driven `ln` kernel. `ln_tang_g` is generic
/// over the storage `St`, the rung work width `Wk` and the fall-up width
/// `Wtier`, sourcing storage bounds as value params — no `WideTrigCore`. It
/// serves every tier: the wide `policy::ln` Tang arms (through the thin
/// `ln_tang<C>`/`tang_at_rung` `C`-forwarders) AND the narrow tiers (D18/D38)
/// at a fixed `Int<12>` work width. Always compiled.
pub(crate) mod ln_tang;
