//! Const-generic base-10 fixed-point decimal types for deterministic arithmetic.
//!
//! # Overview
//!
//! `decimal-scaled` provides `I128<const SCALE: u32>`, a fixed-point decimal
//! type backed by `i128`. The stored integer encodes `actual_value * 10^SCALE`,
//! so decimal literals like `1.1` round-trip exactly without any binary
//! approximation. All core arithmetic is integer-only and produces identical
//! bit-patterns on every platform.
//!
//! # Primary types
//!
//! - [`I128<SCALE>`] is the const-generic foundation. Every method is
//!   implemented once and is available at any scale.
//! - [`I128s12`] is the concrete alias `I128<12>`. At `SCALE = 12`, one LSB
//!   equals `10^-12` model units and the representable range is roughly
//!   +/-1.7e14 model units.
//! - Scale aliases [`I128s0`] through [`I128s38`] cover every supported scale.
//!   `SCALE = 39` is not supported because `10^39` overflows `i128`.
//!
//! # Equality and hashing
//!
//! Because each logical value has exactly one representation at a fixed scale,
//! `Hash`, `Eq`, `PartialEq`, `PartialOrd`, and `Ord` are all derived from
//! the underlying `i128`. Two `I128<S>` values compare equal if and only if
//! their raw bit patterns are identical. This gives predictable behaviour when
//! `I128` values are used as `HashMap` keys, unlike variable-scale decimal
//! types where `1.10` and `1.1` may hash differently.
//!
//! # `num-traits` compatibility
//!
//! [`I128<SCALE>`] implements the standard `num-traits` 0.2 surface,
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
mod i64f64_compat;
#[cfg(any(feature = "std", not(feature = "strict")))]
mod log_exp;
mod mg_divide;
mod num_traits_impls;
mod overflow_variants;
mod powers;
#[cfg(feature = "serde")]
pub mod serde_helpers;
#[cfg(any(feature = "std", not(feature = "strict")))]
mod trig;

pub use consts::DecimalConsts;
pub use conversions::DecimalConvertError;
pub use core_type::{
    I128, I128s0, I128s1, I128s2, I128s3, I128s4, I128s5, I128s6, I128s7, I128s8, I128s9, I128s10,
    I128s11, I128s12, I128s13, I128s14, I128s15, I128s16, I128s17, I128s18, I128s19, I128s20,
    I128s21, I128s22, I128s23, I128s24, I128s25, I128s26, I128s27, I128s28, I128s29, I128s30,
    I128s31, I128s32, I128s33, I128s34, I128s35, I128s36, I128s37, I128s38, ParseDecimalError,
};
