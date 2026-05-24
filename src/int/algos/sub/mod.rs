// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer subtraction algorithm family.
//!
//! - [`sub_ripple_borrow`](sub_ripple_borrow::sub_ripple_borrow) — the
//!   width-agnostic ripple-borrow accumulator selected by
//!   [`crate::int::policy::sub`].

pub(crate) mod sub_ripple_borrow;
