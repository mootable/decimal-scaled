//! The [`Decimal`] trait — width-generic marker that requires the
//! full surface (arithmetic + conversion + transcendentals + constants).
//!
//! `Decimal` itself has no methods; the surface lives on four
//! narrower traits and `Decimal` is a supertrait union:
//!
//! - [`DecimalArithmetic`] — operators, sign, integer-style division,
//!   pow / checked / wrapping / saturating / overflowing, plus the
//!   foundational type info (`Storage`, `SCALE`, `MAX_SCALE`, `ZERO`,
//!   `ONE`, `MAX`, `MIN`, `multiplier()`) and reductions (`sum`,
//!   `product`).
//! - [`DecimalConvert`] — `from_bits` / `to_bits` / `scale`, the
//!   integer bridges (`from_i32`, `to_int`, `to_int_with`), and the
//!   `std`-gated f64 / f32 bridge (`from_f64`, `from_f64_with`,
//!   `to_f64`, `to_f32`).
//! - [`DecimalTranscendental`] — the four-variant matrix on every
//!   transcendental + roots (`_strict` / `_strict_with(mode)` /
//!   `_approx(g)` / `_approx_with(g, mode)`).
//! - [`DecimalConstants`] — `pi` / `tau` / `half_pi` / `quarter_pi` /
//!   `golden` / `e` plus their `_with(mode)` siblings.
//!
//! Implemented automatically by a blanket impl, so any type that
//! impls all four halves is `Decimal` for free. Every shipped width
//! (`D9`, `D18`, `D38`, `D57`, `D76`, `D115`, `D153`, `D230`, `D307`,
//! `D462`, `D616`, `D924`, `D1232`) already satisfies the bound via
//! the per-tier macros (`decl_decimal_basics!` for arithmetic +
//! convert, `decl_decimal_consts!` for constants,
//! `decl_decimal_transcendental_impl!` for the transcendental
//! surface).
//!
//! # Usage
//!
//! ```ignore
//! use decimal_scaled::Decimal;
//!
//! // Pulls in the full surface — arithmetic, conversion,
//! // transcendentals, and constants.
//! fn radius_to_area<T: Decimal>(r: T) -> T {
//!     T::pi() * r * r
//! }
//! ```
//!
//! When you only need a slice of the surface, target the narrower
//! trait directly to keep bounds tight:
//!
//! ```ignore
//! use decimal_scaled::DecimalArithmetic;
//!
//! fn dot<T: DecimalArithmetic + Copy>(a: &[T], b: &[T]) -> T {
//!     a.iter().zip(b).map(|(x, y)| (*x) * (*y))
//!         .fold(T::ZERO, |acc, p| acc + p)
//! }
//! ```
//!
//! # Out of scope on `Decimal` (and all four sub-traits)
//!
//! - **Rescale** (`rescale<TARGET>` / `rescale_with`) takes a
//!   `const`-generic target `SCALE` parameter; const-generic trait
//!   methods aren't stable. Use the inherent method on the concrete
//!   type.
//! - **`from_int`** takes a different source integer per width;
//!   [`DecimalConvert::from_i32`] is the width-generic constructor.
//! - **Joint kernels** (`sin_cos`, `sinh_cosh`) exist only on the
//!   wide tiers; reach for them on the concrete type.

pub use crate::arithmetic_trait::DecimalArithmetic;
pub use crate::convert_trait::DecimalConvert;

/// Marker supertrait combining the four halves of the decimal API
/// surface ([`DecimalArithmetic`], [`DecimalConvert`],
/// [`crate::DecimalTranscendental`], [`crate::DecimalConstants`]).
///
/// Implemented automatically for any type that implements all four;
/// see the module-level documentation for usage.
pub trait Decimal:
    DecimalArithmetic
    + DecimalConvert
    + crate::transcendental_trait::DecimalTranscendental
    + crate::consts::DecimalConstants
{
}

impl<T> Decimal for T where
    T: DecimalArithmetic
        + DecimalConvert
        + crate::transcendental_trait::DecimalTranscendental
        + crate::consts::DecimalConstants
{
}
