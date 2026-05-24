//! Natural-logarithm algorithm family.
//!
//! Narrow tier (D18 / D38) calls the `Fixed` 256-bit intermediate
//! kernels; wide tier (D57 .. D1232) runs the tier-generic `ln_tang` /
//! `*_series` kernels via `crate::policy::ln::LnPolicy`.
//! Both tiers route through `crate::policy::ln::LnPolicy`.
//!
//! Variants:
//!
//! - [`ln_series_2limb`] — D38's hand-tuned ln on the 256-bit `Fixed`
//!   intermediate with the configurable working-scale guard. Carries
//!   the four-variant matrix entry points (strict mode + approximation
//!   mode, each with an explicit-rounding sibling).

pub(crate) mod ln_series_2limb;

/// Tier-generic Tang-style table-driven `ln_strict` kernel, generic over
/// `WideTrigCore`. Collapses the thirteen per-tier Tang ln
/// kernels (D57 .. D1232) into one. The `policy::ln` Tang arms call it
/// with the tier's `Core`, `SCALE`, narrow guard, series cap and
/// narrowing strategy.
#[cfg(feature = "_wide-support")]
pub(crate) mod ln_tang;
