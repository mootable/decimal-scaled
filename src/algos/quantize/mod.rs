// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Decimal `quantize` algorithm family — changing `SCALE` at a fixed
//! storage width.
//!
//! One algorithm: [`quantize_pow10`] — scale the stored integer by
//! `10^|TARGET_SCALE − SCALE|`, exactly on the way up and with `mode`
//! rounding on the way down. The per-`(N, SCALE, TARGET_SCALE)` choice
//! lives in [`crate::policy::quantize`], which delegates *down* to this
//! kernel.
//!
//! [`quantize_pow10`]: crate::algos::quantize::quantize_pow10::quantize_pow10

pub(crate) mod quantize_pow10;
