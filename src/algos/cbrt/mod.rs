//! Cube-root algorithm family.
//!
//! Mirrors [`crate::algos::sqrt`]: each variant is a kernel — a free
//! function taking the raw storage integer plus the runtime scale and
//! rounding mode, and returning the raw storage integer of the cube
//! root. Sign of the input is preserved (`cbrt(-x) = -cbrt(x)`); the
//! rounding mode resolves direction relative to the signed result. The
//! per-`(N, SCALE)` choice between them lives in [`crate::policy::cbrt`].
//!
//! Surviving algorithms (Phase-4 consolidation):
//!
//! - [`cbrt_newton`] — Newton integer cube root over a work integer `W`
//!   that strictly covers `|raw| · 10^(2·SCALE)`. Generic over the
//!   storage and work widths `(S, W)`; the default for every wide tier
//!   (D57 … D1232) and, via the policy's widen-to-`Int<2>` strategy, the
//!   narrow tiers. Exact to the last representable place (within 0.5 ULP)
//!   under any of the six [`RoundingMode`]s.
//! - [`cbrt_mg_divide`] — hand-tuned 384-bit cube-root path tailored to
//!   D38's `Int<2>` storage. **Width-bespoke for `N == 2`.**
//! - [`cbrt_newton_with_table_seed`] — `f64`-seeded narrow-work Newton
//!   bespoke for the `(D57, 20)` cell.
//!
//! [`cbrt_newton`]: crate::algos::cbrt::cbrt_newton::cbrt_newton
//! [`cbrt_mg_divide`]: crate::algos::cbrt::cbrt_mg_divide::cbrt_mg_divide
//! [`cbrt_newton_with_table_seed`]: crate::algos::cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed
//! [`RoundingMode`]: crate::support::rounding::RoundingMode

pub(crate) mod cbrt_mg_divide;
pub(crate) mod cbrt_newton;
pub(crate) mod cbrt_newton_with_table_seed;
