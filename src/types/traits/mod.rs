//! Public trait surface for the decimal types.
//!
//! Five traits expose the cross-width API:
//!
//! - [`Decimal`] — marker supertrait combining the three families below
//!   (arithmetic, convert, transcendental).
//! - [`DecimalArithmetic`] — operator surface (add/sub/mul/div, sign,
//!   rounding, overflow variants).
//! - [`DecimalConvert`] — width / scale conversions and primitive
//!   bridges.
//! - [`DecimalTranscendental`] — `sqrt`, `cbrt`, `exp`, `ln`, trig,
//!   hyperbolic, `pow`.
//! - [`DynDecimal`] — object-safe view (feature `dyn`).
//!
//! Each lives in its own module; this `mod.rs` re-exports the names so
//! `lib.rs` can surface them as `crate::Decimal` etc.

pub(crate) mod decimal;
pub(crate) mod arithmetic;
pub(crate) mod convert;
pub(crate) mod transcendental;
pub(crate) mod width_le;
#[cfg(feature = "dyn")]
pub mod dyn_decimal;

pub use decimal::Decimal;
pub use arithmetic::DecimalArithmetic;
pub use convert::DecimalConvert;
pub use transcendental::DecimalTranscendental;
pub use width_le::WidthLE;
