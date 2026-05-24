// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer hypotenuse algorithm family.
//!
//! `hypot(a, b) = round(sqrt(a^2 + b^2))` for `Int<N>`, generic over the
//! storage limb count `N`. The radicand `a^2 + b^2` is formed in a limb
//! scratch buffer (so no `W = Int<2N>` work type), rooted via the int
//! layer's width-agnostic slice `isqrt`, then a single round step lands the
//! result; [`None`] signals true overflow (the rounded root exceeds the
//! signed range of `Int<N>`). The decimal tier dispatches DOWN to this
//! family -- both decimal operands share `10^SCALE`, which cancels out of
//! the root.
//!
//! - [`hypot_isqrt`](hypot_isqrt::hypot_isqrt) -- the production kernel,
//!   floor root via the Newton slice `isqrt`.
//! - [`hypot_schoolbook`](hypot_schoolbook::hypot_schoolbook) -- the naive
//!   reference / benchmarkable seam, floor root via the division-free,
//!   float-free bitwise schoolbook `isqrt`. Numerically identical.
//!
//! The per-`N` choice between them lives in [`crate::int::policy::hypot`].

pub(crate) mod hypot_isqrt;
pub(crate) mod hypot_schoolbook;
