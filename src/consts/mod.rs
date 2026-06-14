// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Mathematical-constant data — the single source of truth.
//!
//! One mpmath-sourced, committed, u64-limb, per-scale constant table for
//! every constant (pi / tau / half_pi / quarter_pi / e / golden / ln2 / ln10 /
//! deg_per_rad / rad_per_deg) at any scale or width. A wide tier at a narrow
//! scale zero-extends the stored limbs (high limbs = 0); the value is
//! width-independent.
//!
//! This is a bottom DATA leaf: it depends only on the integer layer (for the
//! generic zero-extend) and is consumed *down* into by BOTH the public
//! `DecimalConstants` API and the algorithm / transcendental kernels — so it
//! sits below both and is never reached "upward".
//!
//! [`table`] holds the generated per-scale arrays + the `const fn` `*_entry`
//! lookups + the `*_by_scale` (const working scale — const-folds) / `*_by_working_scale`
//! (runtime working scale — static lookup) accessors. Feature-gated bands:
//! the always-compiled NARROW band covers scales `0..=38` for most constants,
//! widened to `0..=512` for `pi` / `ln2` / `ln10` (the three the always-present
//! narrow kernels read at a WORKING scale); BASE / XW / XXW are gated behind
//! `_wide-support` / `x-wide` / `xx-wide`.

pub(crate) mod table;
pub(crate) mod pow10;
pub(crate) mod newton_recip;
pub(crate) use table::*;
pub(crate) use newton_recip::newton_recip_le;
