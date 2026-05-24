// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer addition algorithm family.
//!
//! - [`add_ripple_carry`](add_ripple_carry::add_ripple_carry) — the
//!   width-agnostic ripple-carry accumulator selected by
//!   [`crate::int::policy::add`].

pub(crate) mod add_ripple_carry;
