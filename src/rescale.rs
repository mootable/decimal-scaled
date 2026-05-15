//! Scale-changing operations on [`D38`].
//!
//! `D38<SCALE>` carries its scale in the type. Converting between two
//! scales — for instance accumulating cents (`D38<2>`) into a picometre-
//! precision running total (`D38<12>`) — requires an explicit rescale.
//!
//! Two surfaces:
//!
//! - [`D38::rescale`] is a `const fn` shorthand that uses
//! round-half-to-even (IEEE-754 default; banker's rounding). Suitable
//! for the overwhelming majority of cases.
//! - [`D38::rescale_with`] takes an explicit [`RoundingMode`] for users
//! whose accounting rules mandate a non-default rule.
//!
//! Scale-up direction (target > source) is always exact: the stored
//! integer is multiplied by `10^diff`. Scale-down direction (target <
//! source) discards fractional digits using the requested rounding mode.
//!
//! Overflow on the scale-up direction is detected via `checked_mul` and
//! panics with a clear message in both debug and release builds.

use crate::core_type::{D38, D9, D18};

// The rescale / rescale_with methods are emitted by the
// `decl_decimal_rescale!` macro (see `src/decimal_rescale_macro.rs`).
crate::macros::rescale::decl_decimal_rescale!(D38, i128);
crate::macros::rescale::decl_decimal_rescale!(D18, i64);
crate::macros::rescale::decl_decimal_rescale!(D9, i32);
