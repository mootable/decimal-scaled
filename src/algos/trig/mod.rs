//! Trigonometric algorithm family — narrow-tier kernels.
//!
//! Covers sin / cos / tan today; atan and the inverse family land in
//! a follow-up. Wide-tier trig stays macro-emitted (same deferral as
//! [`crate::algos::ln`]).
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38 sin / cos / tan on the 256-bit `Fixed`
//!   intermediate via the shared `sin_fixed` core. The fast paths
//!   (`x == 0` and the small-x linear band) are preserved.
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::*` → narrow.

pub(crate) mod fixed_d38;
pub(crate) mod widen_to_d38;
