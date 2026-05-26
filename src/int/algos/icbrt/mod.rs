// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer cube-root algorithm family.
//!
//! - [`icbrt_newton`](icbrt_newton::icbrt_newton) -- width-agnostic
//!   Brent-Zimmermann integer Newton cube root with a hardware-`f64::cbrt`
//!   seed. The per-`N` choice lives in [`crate::int::policy::icbrt`].
//! - [`icbrt_schoolbook`](icbrt_schoolbook::icbrt_schoolbook) -- bit-by-bit
//!   restoring cube root; pure integer, no division, no float seed.
//! - [`icbrt_newton_recip`](icbrt_newton_recip::icbrt_newton_recip) --
//!   **candidate, UNWIRED**: division-free reciprocal-root Newton cube root
//!   (multiplies only in the hot loop, exact integer end-correction). Written
//!   to remove the per-iteration `n/s²` multi-precision divide the shipped
//!   Newton kernel pays; the coordinator benches + wires it where it wins.

pub(crate) mod icbrt_newton;
pub(crate) mod icbrt_newton_recip;
pub(crate) mod icbrt_schoolbook;
