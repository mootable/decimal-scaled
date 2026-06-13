// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `num_traits`-bridge methods on every decimal width.
//!
//! `from_num` / `to_num` are saturating, never-panicking constructors
//! and readers that thread the input through the [`num_traits::NumCast`]
//! ecosystem, dispatching to the width's [`num_traits::FromPrimitive`] /
//! [`num_traits::ToPrimitive`] impls.
//!
//! Idiomatic call sites should prefer the direct surface — `From<T>` /
//! `TryFrom<T>` for construction, `from_f64` / `to_f64` for the float
//! bridge — for readability and stricter overflow handling. The `from_num` /
//! `to_num` pair is provided for code that needs a single saturating
//! `NumCast`-style entry point regardless of input type.
//!
//! # Saturation policy
//!
//! Conversions never panic. Out-of-range inputs are saturated:
//!
//! - `NaN` maps to [`D38::ZERO`].
//! - `+Infinity` maps to [`D38::MAX`].
//! - `-Infinity` maps to [`D38::MIN`].
//! - Finite values outside the representable range saturate to `MAX` or
//!   `MIN` by sign.
//!
//! # Examples
//!
//! ```
//! use decimal_scaled::D38s12;
//!
//! // `from_num` routes any `T: ToPrimitive` through `NumCast`:
//! let d = D38s12::from_num(42_i32);
//! assert_eq!(d, D38s12::try_from(42_i32).unwrap());
//!
//! // `to_num` returns any `T: NumCast + Bounded`, saturating on
//! // out-of-range targets.
//! let f: f32 = d.to_num();
//! assert_eq!(f, 42.0_f32);
//! assert_eq!(D38s12::from_num(f64::INFINITY), D38s12::MAX);
//! ```

// `from_num` / `to_num` are emitted by
// `decl_decimal_num_traits_basics!` in `src/macros/num_traits.rs`,
// so every width (D18 / D38 / D57 / D76 / D115 / D153 / D230 /
// D307 / D462 / D616 / D924 / D1232) gets the same surface. The D38
// behaviour tests live in `decimal-scale-test/tests/api/num_traits.rs`
// (`from_src_num_traits`); the macro emits the same code for the wide
// widths, which the same file's `from_macros_num_traits` module covers.
