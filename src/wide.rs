//! Wide-integer storage backend for the `D256` / `D512` / `D1024`
//! decimal tiers.
//!
//! These widths are gated behind the `d256` / `d512` / `d1024` Cargo
//! features (or the `wide` umbrella). The storage backend is the
//! in-tree hand-rolled `hint` integer family — `bnum` is no longer
//! compiled into normal builds (it is kept only as a benchmark
//! baseline; see `src/benchmark/`).
//!
//! The `hint` signed types are two's-complement by construction, so
//! signed compare, arithmetic shift, and signed divide all behave like
//! the primitive signed integers.
//!
//! Each tier's multiply / divide widens one size up to hold the
//! intermediate product, so a width's signed storage alias is exposed
//! whenever that width *or the next narrower one* is enabled:
//!
//! - `I256`  — D256 storage.
//! - `I512`  — D512 storage, and D256's mul/div widening step.
//! - `I1024` — D1024 storage, and D512's mul/div widening step.
//! - `I2048` — D1024's mul/div widening step.
//!
//! The matching unsigned types (`U256` / `U512` / `U1024`) are used by
//! the magnitude-formatting path in `Display`, which needs
//! `unsigned_abs()` to handle the `MIN` corner case without overflow.

#[cfg(any(feature = "d256", feature = "wide"))]
pub(crate) use crate::hint::{SInt256 as I256, WInt256 as U256};

#[cfg(any(feature = "d256", feature = "d512", feature = "wide"))]
pub(crate) use crate::hint::SInt512 as I512;

#[cfg(any(feature = "d512", feature = "wide"))]
pub(crate) use crate::hint::WInt512 as U512;

#[cfg(any(feature = "d512", feature = "d1024", feature = "wide"))]
pub(crate) use crate::hint::SInt1024 as I1024;

#[cfg(any(feature = "d1024", feature = "wide"))]
pub(crate) use crate::hint::{SInt2048 as I2048, WInt1024 as U1024};
