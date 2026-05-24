// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer comparison algorithm family.
//!
//! - [`cmp_limbwise`](cmp_limbwise::cmp_limbwise) and
//!   [`cmp_limbwise_cross`](cmp_limbwise::cmp_limbwise_cross) — the
//!   width-agnostic sign-first limbwise signed comparison selected by
//!   [`crate::int::policy::cmp`].

pub(crate) mod cmp_limbwise;
