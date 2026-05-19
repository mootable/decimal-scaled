//! Trigonometric algorithm family.
//!
//! Narrow tier (D9 / D18 / D38) carries forward + inverse + atan2 on
//! the 256-bit `Fixed` intermediate. Wide tier (D57 .. D1232)
//! currently covers only sin / cos / tan / atan via per-tier kernels
//! in [`wide_kernel`]; the inverse family (asin / acos / atan2) for
//! the wide tiers remains macro-emitted on inherent methods. Both
//! tiers route through `crate::policy::trig::TrigPolicy`.
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38 forward / inverse trig on the 256-bit `Fixed`
//!   intermediate via the shared `sin_fixed`, `atan_fixed`, and
//!   `atan2_kernel` cores. The fast paths (`x == 0`, the ±1 endpoints
//!   for atan / acos, and the small-x linear band) are preserved.
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::*` → narrow.
//! - [`wide_kernel`] — per-tier `sin_strict_<tier>` / `cos_strict_<tier>`
//!   / `tan_strict_<tier>` / `atan_strict_<tier>` free functions for
//!   the wide tiers (D57 / D76 / D115 / D153 / D230 / D307 / D462 /
//!   D616 / D924 / D1232).

#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod borrow_d57;
pub(crate) mod fixed_d38;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_atan;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_hyper;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_inverse;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_sincos;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_sincos_tang;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s44_56_atan;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s44_56_sincos;
#[cfg(any(feature = "d115", feature = "wide"))]
pub(crate) mod lookup_d115_s57_hyper;
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) mod lookup_d153_s70_82_atan;
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) mod lookup_d153_s70_82_hyper;
#[cfg(any(feature = "d153", feature = "wide"))]
pub(crate) mod lookup_d153_s70_82_sincos;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub(crate) mod lookup_d462_s225_235_atan;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub(crate) mod lookup_d462_s225_235_hyper;
#[cfg(any(feature = "d462", feature = "x-wide"))]
pub(crate) mod lookup_d462_s225_235_sincos;
pub(crate) mod wide_kernel;
pub(crate) mod widen_to_d38;
