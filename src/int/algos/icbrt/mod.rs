// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cube-root algorithm family.
//!
//! - [`icbrt_newton`](icbrt_newton::icbrt_newton) -- width-agnostic
//!   Brent-Zimmermann integer Newton cube root with a hardware-`f64::cbrt`
//!   seed. The per-`N` choice lives in [`crate::int::policy::icbrt`].
//! - [`icbrt_schoolbook`](icbrt_schoolbook::icbrt_schoolbook) -- bit-by-bit
//!   restoring cube root; pure integer, no division, no float seed.

pub(crate) mod icbrt_newton;
pub(crate) mod icbrt_schoolbook;
