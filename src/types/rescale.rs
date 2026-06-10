// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Scale-changing operations for every decimal width.
//!
//! Each `Dxx<SCALE>` carries its scale in the type. Converting between
//! two scales — for instance accumulating cents (`D38<2>`) into a
//! picometre-precision running total (`D38<12>`) — requires an explicit
//! rescale.
//!
//! Two surfaces, emitted on every width:
//!
//! - `rescale::<TARGET>()` — a `const fn` shorthand that uses the
//!   crate-default rounding mode (`HalfToEven` unless overridden by a
//!   `rounding-*` Cargo feature). Suitable for the overwhelming
//!   majority of cases.
//! - `rescale_with::<TARGET>(mode)` — takes an explicit
//!   [`crate::support::rounding::RoundingMode`] for users whose accounting rules
//!   mandate a non-default rule.
//! - `with_scale::<TARGET>()` — builder-style alias for `rescale`.
//!
//! Scale-up direction (target > source) is always exact: the stored
//! integer is multiplied by `10^diff`. Scale-down direction (target <
//! source) discards fractional digits using the requested rounding
//! mode.
//!
//! Overflow on the scale-up direction is detected via `checked_mul`
//! and panics with a clear message in both debug and release builds.

use crate::types::widths::{D18, D38};

// The rescale / rescale_with methods are emitted by
// `crate::macros::rescale::decl_decimal_rescale!` — same macro for
// every width; wide tiers receive it from `macros::full`.
crate::macros::rescale::decl_decimal_rescale!(wide D38, crate::int::types::Int<2>);
crate::macros::rescale::decl_decimal_rescale!(wide D18, crate::int::types::Int<1>);
