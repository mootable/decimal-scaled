//! Const-generic base-10 fixed-point decimal types for deterministic arithmetic.
//!
//! # Overview
//!
//! `decimal-scaled` provides `D128<const SCALE: u32>`, a fixed-point decimal
//! type backed by `i128`. The stored integer encodes `actual_value * 10^SCALE`,
//! so decimal literals like `1.1` round-trip exactly without any binary
//! approximation. All core arithmetic is integer-only and produces identical
//! bit-patterns on every platform.
//!
//! # Primary types
//!
//! - [`D128<SCALE>`] is the const-generic foundation. Every method is
//!   implemented once and is available at any scale.
//! - [`D128e12`] is the concrete alias `D128<12>`. At `SCALE = 12`, one LSB
//!   equals `10^-12` model units and the representable range is roughly
//!   +/-1.7e14 model units.
//! - Scale aliases [`D128e0`] through [`D128e38`] cover every supported scale.
//!   `SCALE = 39` is not supported because `10^39` overflows `i128`.
//!
//! # Equality and hashing
//!
//! Because each logical value has exactly one representation at a fixed scale,
//! `Hash`, `Eq`, `PartialEq`, `PartialOrd`, and `Ord` are all derived from
//! the underlying `i128`. Two `D128<S>` values compare equal if and only if
//! their raw bit patterns are identical. This gives predictable behaviour when
//! `D128` values are used as `HashMap` keys, unlike variable-scale decimal
//! types where `1.10` and `1.1` may hash differently.
//!
//! # `num-traits` compatibility
//!
//! [`D128<SCALE>`] implements the standard `num-traits` 0.2 surface,
//! including [`num_traits::Zero`], [`num_traits::One`], [`num_traits::Num`],
//! [`num_traits::Bounded`], [`num_traits::Signed`],
//! [`num_traits::FromPrimitive`], [`num_traits::ToPrimitive`], and the
//! `Checked{Add,Sub,Mul,Div,Rem,Neg}` family. These impls are unconditional
//! (not behind a feature flag) because generic numeric code in the wider
//! ecosystem consumes this surface by default.
//!
//! # `no_std` support
//!
//! The crate compiles with `no_std + alloc` when default features are
//! disabled. `alloc` is required for `Display::to_string` and
//! `FromStr::from_str`. Targets without `alloc` are not supported.
//!
//! # Feature flags
//!
//! - `std` (default): enables the lossy implementations of transcendental
//!   functions (trigonometry, logarithms, exponentials, square root, cube
//!   root, float power) that delegate to platform `f64` intrinsics.
//! - `alloc`: pulled in automatically; required for string formatting and
//!   parsing.
//! - `serde`: enables `serde_helpers` for serialisation and deserialisation.
//! - `strict`: enables integer-only implementations of all transcendental
//!   functions. When `strict` is active each function that would otherwise
//!   route through `f64` is instead implemented using integer-only
//!   algorithms. Explicit float-conversion methods (`to_f64_lossy`,
//!   `from_f64_lossy`, etc.) remain available regardless; they are type
//!   conversions, not mathematical operations. `strict` does not require
//!   `std`; the integer transcendental implementations compile under
//!   `no_std + alloc`.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::module_name_repetitions)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod arithmetic;
mod bitwise;
mod consts;
mod conversions;
mod core_type;
mod display;
mod equalities;
mod fixed_compat;
mod log_exp;
mod mg_divide;
mod num_traits_impls;
mod overflow_variants;
mod powers;
#[cfg(feature = "serde")]
pub mod serde_helpers;
#[cfg(all(feature = "std", not(feature = "strict")))]
mod trig;

pub use consts::DecimalConsts;
pub use conversions::D128ConvertError;
pub use core_type::{
    D128, D128e0, D128e1, D128e2, D128e3, D128e4, D128e5, D128e6, D128e7, D128e8, D128e9, D128e10,
    D128e11, D128e12, D128e13, D128e14, D128e15, D128e16, D128e17, D128e18, D128e19, D128e20,
    D128e21, D128e22, D128e23, D128e24, D128e25, D128e26, D128e27, D128e28, D128e29, D128e30,
    D128e31, D128e32, D128e33, D128e34, D128e35, D128e36, D128e37, D128e38, ParseD128Error,
};

#[cfg(feature = "macros")]
pub use decimal_scaled_macros::i128s;
