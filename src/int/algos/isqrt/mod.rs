// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer square-root algorithm family.
//!
//! - [`isqrt_newton`](isqrt_newton::isqrt_newton) — width-agnostic Newton
//!   integer square root with a hardware-`f64::sqrt` seed.
//! - [`isqrt_mag_fixed`](isqrt_mag_fixed::isqrt_mag_fixed) — the const-`N`
//!   fast-arm wrapper (`N == 1`/`2` native, `N >= 3` Newton) the
//!   fixed-width `Int<N>` types call.

pub(crate) mod isqrt_mag_fixed;
pub(crate) mod isqrt_newton;
