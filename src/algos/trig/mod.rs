//! Trigonometric algorithm family.
//!
//! Narrow tier (D18 / D38) carries forward + inverse + atan2 on
//! the 256-bit `Fixed` intermediate. Wide tier (D57 .. D1232) runs the
//! tier-generic `*_series` kernels via `crate::policy::trig::TrigPolicy`;
//! the inverse family (asin / acos / atan2) for the wide tiers remains
//! macro-emitted on inherent methods. Both tiers route through
//! `crate::policy::trig::TrigPolicy`.
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38 forward / inverse trig on the 256-bit `Fixed`
//!   intermediate via the shared `sin_fixed`, `atan_fixed`, and
//!   `atan2_kernel` cores. The fast paths (`x == 0`, the ±1 endpoints
//!   for atan / acos, and the small-x linear band) are preserved.
//! - [`wide_kernel`] — the D57 `atan_strict_d57` free function the D38
//!   inverse family borrows (see [`borrow_d57`]).

#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod borrow_d57;
pub(crate) mod fixed_d38;
#[cfg(feature = "_wide-support")]
pub(crate) mod hyper_exp_identity;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_inverse;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s18_22_sincos;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod lookup_d57_s44_56_atan;
pub(crate) mod near_pole_tan;
#[cfg(feature = "_wide-support")]
pub(crate) mod sincos_narrow;
#[cfg(feature = "_wide-support")]
pub(crate) mod sincos_tang;
pub(crate) mod wide_kernel;
