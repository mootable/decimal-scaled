//! Internal `decl_*!` macros that emit per-width surface for the
//! decimal type family.
//!
//! Every macro takes the target `$Type` (e.g. `D32`, `D64`, `D128`)
//! and the relevant storage / widening types as parameters, then
//! emits the corresponding impl block. Each width's `core_type.rs`
//! entry then becomes a series of one-line macro invocations.
//!
//! Naming convention: the file name reflects the surface
//! (`arithmetic.rs` for the Add/Sub/Mul/Div family, `from_str.rs` for
//! the `FromStr` parser, etc.) and the macro inside is named
//! `decl_decimal_<surface>!` or a more specific variant.

pub(crate) mod arithmetic;
pub(crate) mod basics;
pub(crate) mod consts;
pub(crate) mod conversions;
pub(crate) mod display;
pub(crate) mod equalities;
pub(crate) mod float_bridge;
pub(crate) mod from_str;
pub(crate) mod num_traits;
pub(crate) mod overflow;
pub(crate) mod rescale;
pub(crate) mod sign;
pub(crate) mod storage_formatters;
pub(crate) mod strict_transcendentals;
