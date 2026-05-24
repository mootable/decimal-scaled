// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer remainder algorithm family.
//!
//! - [`rem_via_div_rem`](rem_via_div_rem::rem_via_div_rem) — the
//!   width-agnostic remainder derived from the division policy, selected by
//!   [`crate::int::policy::rem`].

pub(crate) mod rem_schoolbook;
pub(crate) mod rem_via_div_rem;
