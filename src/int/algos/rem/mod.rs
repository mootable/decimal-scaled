// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer remainder algorithm family.
//!
//! - [`rem_native`](rem_native::rem_native) - hardware `u128 % u128` for
//!   the two narrowest tiers (`N <= 2`), bypassing the division dispatcher.
//! - [`rem_via_div_rem`](rem_via_div_rem::rem_via_div_rem) - the
//!   width-agnostic remainder derived from the division policy, selected by
//!   [`crate::int::policy::rem`] for the wide tiers (`N >= 3`).

pub(crate) mod rem_native;
// candidate (not wired): direct two's-complement i128 `%` for N<=2, skips
// the sign-magnitude round trip the shipped rem_native still pays.
pub(crate) mod rem_native_direct;
pub(crate) mod rem_schoolbook;
pub(crate) mod rem_via_div_rem;
