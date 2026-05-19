//! Exponential algorithm family.
//!
//! Narrow tier (D9 / D18 / D38) calls the `Fixed` 256-bit intermediate
//! kernels; wide tier (D57 .. D1232) calls per-tier free functions in
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
//!   the wide tiers (D57 / D76 / D115 / D153 / D230 / D307 / D462 /
//!   D616 / D924 / D1232).

#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod borrow_d57;
pub(crate) mod fixed_d38;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_tang;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s45_56;
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) mod lookup_d153_s70_82_tang;
pub(crate) mod wide_kernel;
pub(crate) mod widen_to_d38;
