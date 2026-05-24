//! Decimal arbitrary-base logarithm algorithm family.
//!
//! One algorithm: `LnDivide` — `log(self, base) = ln(self) / ln(base)`.
//! The per-`(N, SCALE)` realisation lives in
//! [`log_ln_divide`](crate::algos::log::log_ln_divide):
//!
//! - the narrow tiers (D18, D38) route through the composition kernels in
//!   that module — D18 widens to D38, runs its log, and narrows back; D38
//!   calls the `ln::ln_series_2limb` log kernel directly;
//! - the wide tiers route through the per-tier `log_strict_with_kernel` /
//!   `log_approx_with_kernel` free functions emitted by
//!   `decl_wide_transcendental!` (the real Ziv-escalating computation),
//!   which already live outside the policy in `crate::types::widths`.
//!
//! The per-`(N, SCALE)` choice lives in [`crate::policy::log`], which
//! delegates *down* to these kernels.

pub(crate) mod log_ln_divide;
