//! Natural-logarithm algorithm family — narrow-tier kernels.
//!
//! Only D9 / D18 / D38 have policy-routed `ln_strict` today. The wide
//! tiers (D56+) still call their per-tier `ln_fixed` directly from the
//! macro-emitted method shell; migrating those through the policy
//! trait requires opening up the per-tier core modules
//! (`wide_trig_<tier>`) and is deferred to a separate effort.
//!
//! Variants:
//!
//! - [`fixed_d38`] — D38's hand-tuned ln on the 256-bit `Fixed`
//!   intermediate with the configurable working-scale guard. Carries
//!   the four-variant matrix entry points (strict mode + approximation
//!   mode, each with an explicit-rounding sibling).
//! - [`widen_to_d38`] — D9 / D18 widen → `fixed_d38::ln` → narrow.

pub(crate) mod fixed_d38;
pub(crate) mod widen_to_d38;
