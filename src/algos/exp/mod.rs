//! Exponential algorithm family — narrow-tier kernels.
//!
//! Only D9 / D18 / D38 have policy-routed `exp_strict` today. Wide-
//! tier `exp_strict` continues to call its per-tier `exp_fixed`
//! directly from the macro-emitted method shell (same deferral as
//! [`crate::algos::ln`]).
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38's `Fixed` 256-bit intermediate `exp_fixed`
//!   path, four-variant matrix entry shape.
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::exp` → narrow.

pub(crate) mod fixed_d38;
pub(crate) mod widen_to_d38;
