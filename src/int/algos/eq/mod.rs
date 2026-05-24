// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer equality algorithm family.
//!
//! - [`eq_limbwise`](eq_limbwise::eq_limbwise) — the width-agnostic
//!   limb-by-limb two's-complement equality test selected by
//!   [`crate::int::policy::eq`].

pub(crate) mod eq_limbwise;
