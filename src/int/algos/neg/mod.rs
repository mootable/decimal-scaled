// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer negation algorithm family.
//!
//! - [`neg_twos_complement`](neg_twos_complement::neg_twos_complement) — the
//!   width-agnostic bitwise-NOT-plus-one two's-complement negation selected
//!   by [`crate::int::policy::neg`].

pub(crate) mod neg_twos_complement;
