// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Floating-point power algorithm family -- narrow-tier kernels.
//!
//! Only D18 / D38 have policy-routed `powf_strict` today. The wide
//! tiers still ship `powf` through their per-tier macro shells; migrating
//! those mirrors the deferral on [`crate::algos::ln`] / [`crate::algos::exp`].
//!
//! `powf` is the composition `exp(y * ln(x))` performed entirely in the
//! 256-bit `Fixed` guard-digit intermediate, so the round-trip never
//! drops precision below the working scale before the final rounding.
//!
//! Variants:
//!
//! - [`powf_series_2limb`] -- D38's hand-tuned `powf` on the `Fixed` intermediate,
//!   carrying the four-variant matrix entry shape (strict + approx, each
//!   with an explicit-rounding sibling). The D38 realisation of the
//!   `powf_exp_with_ln` (`ExpWithLn`) algorithm.
//! - [`pow_schoolbook`] -- correctness reference: naive `exp(y*ln(x))`
//!   using the schoolbook exp and ln. Registered as the unrouted
//!   `Algorithm::Schoolbook` variant.

pub(crate) mod powf_series_2limb;
/// Exact integer-power pin shared by the narrow + wide `powf` kernels: when
/// the base and exponent are exact integers, `base^exp` is an exact rational
/// and its correctly-directed-rounded value is emitted directly instead of
/// the to-nearest `exp(exp*ln base)` composition.
pub(crate) mod powi_exact;
/// Schoolbook floating-point power -- naive `exp(y*ln(x))` composition,
/// correctness reference. Registered as the unrouted `Algorithm::Schoolbook`
/// arm; not connected to `select`.
pub(crate) mod pow_schoolbook;
