//! Shared support kernels consumed across the algorithm families.
//!
//! Unlike the per-family trees (`algos::sqrt`, `algos::ln`, …) these are
//! not tied to one mathematical family; they are the cross-cutting
//! building blocks the families and the arithmetic layer share:
//!
//! - [`mg_divide`] — the Moller-Granlund magic-number divide used by
//!   every multiplicative `÷ 10^SCALE` rescale path.
//! - [`fixed_d38`] — the 256-bit sign-magnitude `Fixed` work integer the
//!   strict-transcendental fallback paths evaluate their series in.
//! - [`newton_reciprocal`] — the Newton-Raphson reciprocal divide for
//!   `n / 10^SCALE` at the wide tiers, head-to-head benched against
//!   [`mg_divide::div_wide_pow10_chain_with`] and routed in by
//!   [`newton_reciprocal::dispatch_wide_pow10_with`] at the cells where
//!   the bench matrix shows it wins.

pub(crate) mod fixed_d38;
pub(crate) mod mg_divide;

// Typed-`W` Newton-root seed bridge over the cross-algorithm seed leaf
// (`algo_x_support::seed`). Used by the wide fixed-point `sqrt` kernel,
// which needs the over-estimate seed (not the final floor root) to start
// its own scaled Newton loop. The floor-root surface itself is
// `W::isqrt` / `W::icbrt`.
pub(crate) mod seed_bridge;

// Per-thread, working-scale-keyed table memoisation for the Tang-style
// lookup kernels (`decl_table_cache!`). Encapsulates the std (thread_local
// cache) vs no_std (rebuild-per-call) divergence inside the macro body so
// the lookup kernels stay cfg-free. A support memo, not a policy matcher.
pub(crate) mod table_cache;

// Tier-generic surface over the per-tier wide guard-digit cores. The
// `WideTrigCore` trait + six `*_series` functions collapse the 60
// per-tier `*_strict_<tier>` wrappers in `algos::{exp,ln,trig}::wide_kernel`
// to one generic function per transcendental. Always compiled; the
// per-tier `impl WideTrigCore` blocks are emitted by
// `decl_wide_transcendental!`.
pub(crate) mod wide_trig_core;

// Newton-Raphson reciprocal divide for `n / 10^SCALE` at the wide tiers.
// Always compiled: the unified `decl_decimal_arithmetic!` mul/div path
// used by D18/D38 (default features) references the dispatcher in a
// const-folded `SCALE > 38` branch — dead for the narrow tiers but still
// type-checked. The kernels are generic over `Int<N>`, so this adds
// compile time, not a feature.
pub(crate) mod newton_reciprocal;
