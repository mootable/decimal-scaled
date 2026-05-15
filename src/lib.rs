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
//! implemented once and is available at any scale.
//! - [`D128s12`] is the concrete alias `D128<12>`. At `SCALE = 12`, one LSB
//! equals `10^-12` model units and the representable range is roughly
//! +/-1.7e14 model units.
//! - Scale aliases [`D128s0`] through [`D128s38`] cover every supported scale.
//! `SCALE = 39` is not supported because `10^39` overflows `i128`.
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
//! functions (trigonometry, logarithms, exponentials, square root, cube
//! root, float power) that delegate to platform `f64` intrinsics.
//! - `alloc`: pulled in automatically; required for string formatting and
//! parsing.
//! - `serde`: enables `serde_helpers` for serialisation and deserialisation.
//! - `strict`: enables integer-only implementations of all transcendental
//! functions. When `strict` is active each function that would otherwise
//! route through `f64` is instead implemented using integer-only
//! algorithms. Explicit float-conversion methods (`to_f64_lossy`,
//! `from_f64_lossy`, etc.) remain available regardless; they are type
//! conversions, not mathematical operations. `strict` does not require
//! `std`; the integer transcendental implementations compile under
//! `no_std + alloc`.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "experimental-floats", feature(f16, f128))]
#![allow(clippy::module_name_repetitions)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod arithmetic;
mod consts;
mod core_type;
mod decimal_trait;
mod display;
mod equalities;
mod error;
mod macros;
mod fixed_compat;
mod log_exp_strict;
mod log_exp_lossy;

// `bitwise` and `num_traits_impls` used to live here as test-only
// modules; their tests now run as Cargo integration tests under
// `tests/`. The macro-generated impls themselves are emitted by
// `decl_decimal_bitwise!` / `decl_decimal_num_traits_basics!` from
// `core_type.rs`, alongside every other surface.
mod rescale;
mod rounding;
mod mg_divide;
mod d128_kernels;
// `wide_int` is now unconditional. D128's strict transcendentals use
// `Int512` as their guard-digit work integer (replacing the previous
// `d128_kernels::Fixed` 256-bit sign-magnitude type), so the wide-
// integer family must be available in every feature configuration —
// not just `feature = "wide"` builds. Compile-time impact is modest:
// ~2k LOC of self-contained limb arithmetic plus the per-width
// `decl_wide_int!` instantiations.
mod wide_int;
mod overflow_variants;
mod powers_strict;
mod powers_lossy;

#[cfg(feature = "serde")]
pub mod serde_helpers;
// `trig` is compiled when it has any surface to emit: the integer-only
// `*_strict` methods (present unless `no_strict`) or the f64-bridge
// methods (present with `std`).
#[cfg(any(not(feature = "no_strict"), feature = "std"))]
mod trig_strict;
mod trig_lossy;


pub use consts::DecimalConsts;
pub use decimal_trait::Decimal;
pub use error::{ConvertError, ParseError};
pub use rounding::RoundingMode;

// D128 — the 128-bit foundation, plus every scale alias D128s0..=D128s38.
pub use core_type::{
    D128, D128s0, D128s1, D128s2, D128s3, D128s4, D128s5, D128s6, D128s7, D128s8, D128s9, D128s10,
    D128s11, D128s12, D128s13, D128s14, D128s15, D128s16, D128s17, D128s18, D128s19, D128s20,
    D128s21, D128s22, D128s23, D128s24, D128s25, D128s26, D128s27, D128s28, D128s29, D128s30,
    D128s31, D128s32, D128s33, D128s34, D128s35, D128s36, D128s37, D128s38,
};

// D32 — 32-bit storage, scale 0..=9.
pub use core_type::{
    D32, D32s0, D32s1, D32s2, D32s3, D32s4, D32s5, D32s6, D32s7, D32s8, D32s9,
};

// D64 — 64-bit storage, scale 0..=18.
pub use core_type::{
    D64, D64s0, D64s1, D64s2, D64s3, D64s4, D64s5, D64s6, D64s7, D64s8, D64s9, D64s10, D64s11,
    D64s12, D64s13, D64s14, D64s15, D64s16, D64s17, D64s18,
};

// D256 — 256-bit storage, behind the `d256` / `wide` features.
#[cfg(any(feature = "d256", feature = "wide"))]
pub use core_type::{
    D256, D256s0, D256s2, D256s6, D256s12, D256s18, D256s35, D256s50, D256s76,
};

// The hand-rolled wide-integer types — the storage backend for the
// wide decimal tiers, also useful on their own.
#[cfg(any(feature = "d256", feature = "d512", feature = "d1024", feature = "wide"))]
pub use wide_int::{
    Int256, Int512, Int1024, Int2048, Int4096, Uint256, Uint512, Uint1024, Uint2048, Uint4096,
};

// D512 — 512-bit storage, behind the `d512` / `wide` features.
#[cfg(any(feature = "d512", feature = "wide"))]
pub use core_type::{D512, D512s0, D512s35, D512s75, D512s150, D512s153};

// D1024 — 1024-bit storage, behind the `d1024` / `wide` features.
#[cfg(any(feature = "d1024", feature = "wide"))]
pub use core_type::{D1024, D1024s0, D1024s35, D1024s150, D1024s300, D1024s307};

#[cfg(feature = "macros")]
pub use decimal_scaled_macros::d128;
