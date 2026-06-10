// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trigonometric algorithm family.
//!
//! Narrow tier (D18 / D38) carries forward + inverse + atan2 on
//! the 256-bit `Fixed` intermediate. Wide tier (D57 .. D1232) runs the
//! tier-generic `*_series` kernels via `crate::policy::trig`;
//! the inverse family (asin / acos / atan2) for the wide tiers remains
//! macro-emitted on inherent methods. Both tiers route through
//! `crate::policy::trig`. The D38 inverse family borrows D57
//! via the `borrow_d57` dispatch strategy in `crate::policy::trig` (a
//! policy-layer strategy, not an algorithm).
//!
//! Variants:
//!
//! - [`trig_series_2limb`] — D38 forward / inverse trig on the 256-bit `Fixed`
//!   intermediate via the shared `sin_fixed`, `atan_fixed`, and
//!   `atan2_kernel` cores. The fast paths (`x == 0`, the ±1 endpoints
//!   for atan / acos, and the small-x linear band) are preserved.

pub(crate) mod trig_series_2limb;
pub(crate) mod trig_schoolbook;
pub(crate) mod inverse_schoolbook;
pub(crate) mod hyper_schoolbook;
pub(crate) mod angle_mul_pi_ratio;
pub(crate) mod angle_schoolbook;
#[cfg(feature = "_wide-support")]
pub(crate) mod hyper_exp_identity;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod inverse_tang_3limb_s18_22;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod sincos_tang_3limb_s18_22;
#[cfg(any(feature = "d57", feature = "wide"))]
pub(crate) mod atan_tang_3limb_s44_56;
pub(crate) mod near_pole_tan;
#[cfg(feature = "_wide-support")]
pub(crate) mod sincos_narrow;
#[cfg(feature = "_wide-support")]
pub(crate) mod sincos_tang;
