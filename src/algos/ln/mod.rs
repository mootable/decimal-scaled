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

/// Width-generic Tang-style table-driven `ln` kernel. Collapses what were
/// thirteen per-tier Tang ln kernels (D57 .. D1232) into one, and — since
/// the kernel was lifted free of `WideTrigCore` — serves the NARROW tiers
/// (D18 / D38) from the same body. The `policy::ln` Tang arms call it with
/// the storage type, the work rung, the guard, the series cap, the
/// narrowing strategy and which baked table to read.
///
/// Always compiled: the narrow tiers exist in every build. Only the two
/// thin `C: WideTrigCore` convenience wrappers inside are
/// `_wide-support`-gated.
pub(crate) mod ln_tang;
