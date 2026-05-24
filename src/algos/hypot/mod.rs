//! Hypotenuse algorithm family.
//!
//! `hypot` computes `sqrt(a² + b²)` by routing the root **down** through
//! the integer layer, exactly as the raw-storage root kernels in
//! [`crate::algos::sqrt`] do: it forms the radicand `a² + b²` in a wider
//! work integer and takes the floor root via
//! [`crate::int::types::traits::BigInt::isqrt`], then a single round step
//! — it does **not** call the decimal `sqrt` surface on the tier's own
//! value. The single variant serves every `(N, SCALE)` cell; the per-cell
//! selection lives in [`crate::policy::hypot`].
//!
//! - [`hypot_isqrt`] — `round(sqrt(a² + b²))` over a work width `W` that
//!   covers `a² + b²`. Generic over the storage and work widths `(S, W)`;
//!   the rounding step is identical to
//!   [`crate::algos::sqrt::sqrt_newton`].
//!
//! [`hypot_isqrt`]: crate::algos::hypot::hypot_isqrt::hypot_isqrt

pub(crate) mod hypot_isqrt;
