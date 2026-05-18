//! Natural-logarithm algorithm family.
//!
//! Narrow tier (D9 / D18 / D38) calls the `Fixed` 256-bit intermediate
//! kernels; wide tier (D57 .. D1232) calls per-tier free functions in
//! [`wide_kernel`] that wrap the `wide_trig_<tier>::ln_fixed` core the
//! `decl_wide_transcendental!` macro emits next to each `Dxx` struct.
//! Both tiers route through `crate::policy::ln::LnPolicy`.
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38's hand-tuned ln on the 256-bit `Fixed`
//!   intermediate with the configurable working-scale guard. Carries
//!   the four-variant matrix entry points (strict mode + approximation
//!   mode, each with an explicit-rounding sibling).
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::ln` → narrow.
//! - [`wide_kernel`] — per-tier `ln_strict_<tier>` free functions for
//!   the wide tiers (D57 / D76 / D115 / D153 / D230 / D307 / D462 /
//!   D616 / D924 / D1232).

#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod borrow_d57;
pub(crate) mod fixed_d38;
pub(crate) mod wide_kernel;
pub(crate) mod widen_to_d38;
