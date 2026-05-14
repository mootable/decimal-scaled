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
//! It never appears in a public method *body* that a `D128`-or-narrower
//! user would compile; the `bnum` dependency is only pulled in when a
//! wide feature is active.

#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) use bnum::types::I256;

#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) use bnum::types::I512;

#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) use bnum::types::I1024;
