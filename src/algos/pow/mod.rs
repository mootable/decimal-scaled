//! Floating-point power algorithm family — narrow-tier kernels.
//!
//! Only D9 / D18 / D38 have policy-routed `powf_strict` today. The wide
//! tiers still ship `powf` through their per-tier macro shells; migrating
//! those mirrors the deferral on [`crate::algos::ln`] / [`crate::algos::exp`].
//!
//! `powf` is the composition `exp(y · ln(x))` performed entirely in the
//! 256-bit `Fixed` guard-digit intermediate, so the round-trip never
//! drops precision below the working scale before the final rounding.
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38's hand-tuned `powf` on the `Fixed` intermediate,
//!   carrying the four-variant matrix entry shape (strict + approx, each
//!   with an explicit-rounding sibling). Retained as the D56-disabled
//!   fallback for the D38 surface.
//! - [`borrow_d56`] — D38 widen → D56 inherent `powf_strict_with` /
//!   `powf_approx_with` → narrow back. Picks up the ln + exp wide-tier
//!   speedups in composed form. Gated on `d56` / `wide`.
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::powf` → narrow.

#[cfg(any(feature = "d56", feature = "wide"))]
pub(crate) mod borrow_d56;
pub(crate) mod fixed_d38;
pub(crate) mod widen_to_d38;
