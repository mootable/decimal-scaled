//! Cube-root algorithm family.
//!
//! Mirrors [`crate::algos::sqrt`]: free kernel functions taking raw
//! storage + scale + rounding mode and returning raw storage. Sign of
//! the input is preserved (`cbrt(-x) = -cbrt(x)`); the rounding mode
//! resolves direction relative to the signed result.
//!
//! Variants:
//!
//! - [`generic_wide`] — Newton iteration on `mag · 10^(2·SCALE)` over a
//!   wide work integer. Width default for D56 / D76 / D114 / D153 /
//!   D230 / D307 / D461 / D615 / D923 / D1231.
//! - [`mg_divide_d38`] — hand-tuned 384-bit cube-root path tailored to
//!   D38's `i128` storage. **Width specialisation for D38**, captures
//!   the kernel that has shipped with D38 since before the algorithm
//!   library existed.
//! - [`widen_to_d38`] — widen → `mg_divide_d38::cbrt` → narrow.
//!   **Width specialisation for D9 and D18**.

pub(crate) mod generic_wide;
pub(crate) mod mg_divide_d38;
pub(crate) mod widen_to_d38;
