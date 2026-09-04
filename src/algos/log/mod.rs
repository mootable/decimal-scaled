// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal arbitrary-base logarithm algorithm family.
//!
//! One composition, `log(self, base) = ln(self) / ln(base)`, in two guards.
//! Both realisations live in
//! [`log_ln_divide`](crate::algos::log::log_ln_divide):
//!
//! - `LnDivide`, the fixed guard (`SCALE + 30`), for every ordinary base:
//!   the narrow tiers (D18, D38) route through the composition kernels in
//!   that module -- D18 widens to D38, runs its log, and narrows back; D38
//!   calls the `ln::ln_series_2limb` log kernel directly -- and the wide
//!   tiers through the per-tier `log_strict_with_kernel` free functions
//!   emitted by `decl_wide_transcendental!` (the Ziv-escalating shell),
//!   which live outside the policy in `crate::types::widths`;
//! - `LnDivideConditioned`, for a base within 0.1 of 1: the ratio formed
//!   from the exact `d = b_raw - 10^SCALE` as `(ln x / g(eps)) * 10^SCALE / d`
//!   with `g(eps) = ln(1+eps)/eps` by its own series -- never `ln b` -- and,
//!   when `x` is within 0.1 of 1 as well, from BOTH exact offsets as
//!   `(g(a) / g(eps)) * d_x / d_b` -- never `ln x` either -- at a guard
//!   sized from the base's conditioning number `k = ceil(-log10 |b - 1|)`
//!   (`SCALE + 30 + k`, the result's own integer digits): one generic
//!   kernel over the storage width and a work integer the policy picks from
//!   `k`. The error law that makes the lift necessary, why this form needs
//!   `k` where the naive quotient needs `2k`, and why the numerator takes
//!   the same treatment (a directed rounding's deciding digit can sit
//!   `3p - q` down when both arguments are near 1), are derived on that
//!   module.
//!
//! The per-`(N, SCALE, base)` choice lives in [`crate::policy::log`], which
//! delegates *down* to these kernels.
//!
//! Variants:
//!
//! - [`log_ln_divide`] -- the production `ln(x)/ln(b)` kernels, fixed-guard
//!   and conditioned.
//! - [`log_schoolbook`] -- correctness reference: naive `ln(x)/ln(b)`
//!   using the schoolbook ln. Registered as the unrouted
//!   `Algorithm::Schoolbook` variant.

pub(crate) mod log_ln_divide;
/// Schoolbook base-b logarithm -- naive `ln(x)/ln(b)` composition,
/// correctness reference. Registered as the unrouted `Algorithm::Schoolbook`
/// arm; not connected to `select`.
pub(crate) mod log_schoolbook;
