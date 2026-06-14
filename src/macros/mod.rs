// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Internal `decl_*!` macros that emit per-width surface for the
//! decimal type family.
//!
//! Every macro takes the target `$Type` (e.g. `D18`, `D38`)
//! and the relevant storage / widening types as parameters, then
//! emits the corresponding impl block. Each width's `types/widths.rs`
//! entry then becomes a series of one-line macro invocations.
//!
//! Naming convention: the file name reflects the surface
//! (`arithmetic.rs` for the Add/Sub/Mul/Div family, `from_str.rs` for
//! the `FromStr` parser, etc.) and the macro inside is named
//! `decl_decimal_<surface>!` or a more specific variant.

pub(crate) mod arithmetic;
pub(crate) mod basics;
pub(crate) mod bitwise;
pub(crate) mod conversions;
pub(crate) mod cross_scale_ops;
pub(crate) mod cross_width_cmp;
pub(crate) mod display;
#[cfg(feature = "dyn")]
pub(crate) mod dyn_bridge;
pub(crate) mod equalities;
pub(crate) mod fast_transcendentals;
pub(crate) mod float_bridge;
pub(crate) mod from_str;
pub(crate) mod full;
pub(crate) mod helpers;
pub(crate) mod int_methods;
pub(crate) mod num_traits;
pub(crate) mod overflow;
pub(crate) mod pow;
pub(crate) mod rescale;
pub(crate) mod rounding_methods;
pub(crate) mod sign;
pub(crate) mod storage_formatters;
pub(crate) mod strict_transcendentals;
pub(crate) mod transcendental_trait;
#[cfg(feature = "_wide-support")]
pub(crate) mod wide_roots;
#[cfg(feature = "_wide-support")]
pub(crate) mod wide_transcendental;
