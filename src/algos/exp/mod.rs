//! Exponential algorithm family.
//!
//! Narrow tier (D9 / D18 / D38) calls the `Fixed` 256-bit intermediate
//! kernels; wide tier (D56 .. D1231) calls per-tier free functions in
//! [`wide_kernel`] that wrap the `wide_trig_<tier>::exp_fixed` core
//! the `decl_wide_transcendental!` macro emits next to each `Dxx`
//! struct. Both tiers route through `crate::policy::exp::ExpPolicy`.
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38's `Fixed` 256-bit intermediate `exp_fixed`
//!   path, four-variant matrix entry shape.
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::exp` → narrow.
//! - [`wide_kernel`] — per-tier `exp_strict_<tier>` free functions for
//!   the wide tiers (D56 / D76 / D114 / D153 / D230 / D307 / D461 /
//!   D615 / D923 / D1231).

pub(crate) mod fixed_d38;
pub(crate) mod wide_kernel;
pub(crate) mod widen_to_d38;
