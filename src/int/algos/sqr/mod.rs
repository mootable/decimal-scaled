// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer squaring algorithm family.
//!
//! - [`sqr_half_product`] — the squaring algorithm fn: half-product
//!   squaring over the const [`sqr_low_fixed`] kernel. The per-`N` choice
//!   lives in [`crate::int::policy::sqr`].
//! - [`sqr_low_fixed`] — the truncated half-product squaring KERNEL the
//!   algorithm (and the cube / pow square-and-multiply loops) compute on.
//!
//! [`sqr_half_product`]: crate::int::algos::sqr::sqr_half_product::sqr_half_product
//! [`sqr_low_fixed`]: crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed

pub(crate) mod sqr_half_product;
pub(crate) mod sqr_low_fixed;
