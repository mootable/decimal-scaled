//! Square-root algorithm family.
//!
//! Each variant in this module is a kernel — a free function taking the
//! raw storage integer plus the runtime scale and rounding mode, and
//! returning the raw storage integer of the square root. The per-`(N,
//! SCALE)` choice between them lives in [`crate::policy::sqrt`].
//!
//! Surviving algorithms (Phase-4 consolidation):
//!
//! - [`sqrt_newton`] — Newton integer `isqrt` over a work integer `W`
//!   that strictly covers `raw · 10^SCALE`. Generic over the storage and
//!   work widths `(S, W)`; the default for every wide tier (D57 … D1232)
//!   and, via the policy's widen-to-`Int<2>` strategy, the narrow tiers.
//!   The result is exact to the last representable place (within 0.5 ULP)
//!   under any of the six [`RoundingMode`]s.
//! - [`sqrt_mg_divide`] — hand-tuned `mul_u128_to_u256` + 256-bit
//!   `isqrt_256` tailored to D38's `Int<2>` storage. **Width-bespoke for
//!   `N == 2`.**
//! - [`sqrt_newton_with_table_seed`] — `f64`-seeded narrow-work Newton
//!   bespoke for the `(D57, 20)` cell.
//!
//! [`sqrt_newton`]: crate::algos::sqrt::sqrt_newton::sqrt_newton
//! [`sqrt_mg_divide`]: crate::algos::sqrt::sqrt_mg_divide::sqrt_mg_divide
//! [`sqrt_newton_with_table_seed`]: crate::algos::sqrt::sqrt_newton_with_table_seed::sqrt_newton_with_table_seed
//! [`RoundingMode`]: crate::support::rounding::RoundingMode

pub(crate) mod sqrt_mg_divide;
pub(crate) mod sqrt_newton;
pub(crate) mod sqrt_newton_with_table_seed;
