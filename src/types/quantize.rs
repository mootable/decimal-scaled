// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Scale-changing operations for every decimal width.
//!
//! Each `Dxx<SCALE>` carries its scale in the type. Converting between
//! two scales — for instance accumulating cents (`D38<2>`) into a
//! picometre-precision running total (`D38<12>`) — requires an explicit
//! quantize.
//!
//! Three surfaces, emitted on every width:
//!
//! - `quantize::<TARGET>()` — shorthand that uses the crate-default
//!   rounding mode (`HalfToEven` unless overridden by a `rounding-*`
//!   Cargo feature). Suitable for the overwhelming majority of cases.
//! - `quantize_with::<TARGET>(mode)` — takes an explicit
//!   [`crate::support::rounding::RoundingMode`] for users whose accounting rules
//!   mandate a non-default rule.
//! - `with_scale::<TARGET>()` — builder-style alias for `quantize`.
//!
//! The 0.5.0 names `rescale` / `rescale_with` remain as deprecated
//! aliases and are removed in 0.6.0.
//!
//! Scale-up direction (target > source) is always exact: the stored
//! integer is multiplied by `10^diff`. Scale-down direction (target <
//! source) discards fractional digits using the requested rounding
//! mode.
//!
//! Overflow on the scale-up direction is detected via `checked_mul`
//! and panics with a clear message in both debug and release builds.

use crate::types::widths::{D18, D38};

// The quantize / quantize_with methods are emitted by
// `crate::macros::quantize::decl_decimal_quantize!` — same macro for
// every width; wide tiers receive it from `macros::full`.
crate::macros::quantize::decl_decimal_quantize!(wide D38, crate::int::types::Int<2>);
crate::macros::quantize::decl_decimal_quantize!(wide D18, crate::int::types::Int<1>);
