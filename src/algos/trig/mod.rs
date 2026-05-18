//! Trigonometric algorithm family — narrow-tier kernels.
//!
//! Covers sin / cos / tan plus the inverse family
//! (atan / asin / acos / atan2). Wide-tier trig stays macro-emitted
//! (same deferral as [`crate::algos::ln`]).
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38 forward / inverse trig on the 256-bit `Fixed`
//!   intermediate via the shared `sin_fixed`, `atan_fixed`, and
//!   `atan2_kernel` cores. The fast paths (`x == 0`, the ±1 endpoints
//!   for atan / acos, and the small-x linear band) are preserved.
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::*` → narrow.

pub(crate) mod fixed_d38;
pub(crate) mod widen_to_d38;
