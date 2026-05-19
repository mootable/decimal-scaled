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

#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_tang;
#[cfg(any(feature = "d115", feature = "wide"))]
pub(crate) mod lookup_d115_s57_tang;
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) mod lookup_d153_s70_82_tang;
#[cfg(any(feature = "d230", feature = "wide"))]
pub(crate) mod lookup_d230_s110_120_tang;
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
pub(crate) mod lookup_d307_s140_160_tang;
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
pub(crate) mod lookup_d307_s285_295_tang;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub(crate) mod lookup_d462_s225_235_tang;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub(crate) mod lookup_d616_s300_315_tang;
#[cfg(any(feature = "d616", feature = "x-wide"))]
pub(crate) mod lookup_d616_s585_595_tang;
#[cfg(any(feature = "d924", feature = "xx-wide"))]
pub(crate) mod lookup_d924_s455_465_tang;
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
pub(crate) mod lookup_d1232_s610_620_tang;
