//! Wide-integer storage backend for the `D256` / `D512` / `D1024`
//! decimal tiers.
//!
//! These widths are gated behind the `d256` / `d512` / `d1024` Cargo
//! features (or the `wide` umbrella). The interim storage backend is
//! `bnum`'s fixed-width signed integers; a hand-rolled in-tree backend
//! is planned alongside it for benchmark comparison (see
//! `research/multi_width_decimals.md` §3).
//!
//! `bnum` is signed by construction, so two's-complement semantics
//! (signed compare, arithmetic shift, signed divide) come for free.
//! The `bnum` dependency is only pulled in when a wide feature is
//! active.
//!
//! Each tier's multiply / divide widens one size up to hold the
//! intermediate product, so a width's storage alias is also exposed
//! whenever the *next narrower* width is enabled:
//!
//! - `I256` — needed by D256.
//! - `I512` — needed by D512 *and* by D256's mul/div widening step.
//! - `I1024` — needed by D1024 *and* by D512's mul/div widening step.
//!
//! The matching unsigned types (`U256` / `U512` / `U1024`) are used by
//! the magnitude-formatting path in `Display`, which needs
//! `unsigned_abs()` to handle the `MIN` corner case without overflow.

#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) use bnum::types::{I256, U256};

#[cfg(any(feature = "d256", feature = "d512", feature = "wide"))]
pub(crate) use bnum::types::I512;

#[cfg(any(feature = "d512", feature = "d1024", feature = "wide"))]
pub(crate) use bnum::types::I1024;

#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) use bnum::types::U512;

#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) use bnum::types::U1024;
