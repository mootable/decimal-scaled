//! Floating-point power algorithm family — narrow-tier kernels.
//!
//! Only D18 / D38 have policy-routed `powf_strict` today. The wide
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
//!   with an explicit-rounding sibling). The D38 realisation of the
//!   `powf_exp_with_ln` (`ExpWithLn`) algorithm.

pub(crate) mod fixed_d38;
