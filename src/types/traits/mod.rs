// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Public trait surface for the decimal types.
//!
//! Six traits expose the cross-width API:
//!
//! - [`Decimal`] — marker supertrait combining the four families below
//!   (arithmetic, convert, transcendental, constants).
//! - [`DecimalArithmetic`] — operator surface (add/sub/mul/div, sign,
//!   rounding, overflow variants).
//! - [`DecimalConvert`] — width / scale conversions and primitive
//!   bridges.
//! - [`DecimalTranscendental`] — `sqrt`, `cbrt`, `exp`, `ln`, trig,
//!   hyperbolic, `pow`.
//! - [`DecimalConstants`] — `pi`, `tau`, `e`, `golden` and their
//!   `_with(mode)` siblings.
//! - [`DynDecimal`] — object-safe view (feature `dyn`).
//!
//! Each lives in its own module; this `mod.rs` re-exports the names so
//! `lib.rs` can surface them as `crate::Decimal` etc.

pub(crate) mod arithmetic;
pub(crate) mod consts;
pub(crate) mod convert;
pub(crate) mod decimal;
#[cfg(feature = "dyn")]
pub mod dyn_decimal;
pub(crate) mod transcendental;
pub(crate) mod width_le;

pub use arithmetic::DecimalArithmetic;
pub use consts::DecimalConstants;
pub use convert::DecimalConvert;
pub use decimal::Decimal;
pub use transcendental::DecimalTranscendental;
pub use width_le::WidthLE;
