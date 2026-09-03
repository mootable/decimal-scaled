// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Shared operand builders for the scale-change family benches
//! (`quantize_surface`, `quantize_family_ab`, `quantize_dyn_ab`).
//!
//! # Why two input classes
//!
//! `quantize_with` returns EARLY when the scale-down remainder is zero,
//! skipping the whole rounding block. So the operand decides which path
//! is measured, and a single operand would silently bench only one of
//! them. These builders produce the two classes deliberately:
//!
//! - [`exact_str`] — remainder always zero, the early return fires;
//! - [`dense_str`] — remainder never zero, the full divide-and-round
//!   path runs.
//!
//! Both are parsed OUTSIDE the timed closure by every caller; the parse
//! cost is never inside `bn.iter`.
//!
//! This module is `#![allow(dead_code)]` because each bench uses only
//! the builders it needs.

#![allow(dead_code)]

/// An operand whose low-order digits are ALL zero at any scale.
///
/// Parsed at `SCALE`, `2` stores as `2 * 10^SCALE`, so dividing by
/// `10^k` for any `k <= SCALE` leaves `remainder == 0` and
/// `quantize_with` takes its early return without entering the rounding
/// block. This is the SHORT path.
pub fn exact_str(_scale: u32) -> String {
    "2".to_string()
}

/// An operand whose residue is non-zero for EVERY `k >= 1`.
///
/// The fractional digits cycle `1..=9` and so never contain a zero, let
/// alone a zero run — therefore the low `k` digits are never all zero
/// and `remainder != 0` for every truncation, forcing the FULL
/// divide-and-round path.
///
/// One integer digit is what keeps the operand legal across the whole
/// sweep: a tier's `MAX_SCALE` is `digits - 1`, so `1` + `MAX_SCALE`
/// fractional digits exactly fills the width, and every scale-UP from a
/// lower source scale also stays inside the tier.
pub fn dense_str(scale: u32) -> String {
    if scale == 0 {
        // No fractional room at scale 0; a bare digit is still a valid
        // operand and every scale-up from 0 stays inside the tier.
        return "7".to_string();
    }
    let mut s = String::with_capacity(scale as usize + 2);
    s.push_str("1.");
    for i in 0..scale as usize {
        // '1'..='9', cycling — never a '0'.
        s.push(char::from(b'1' + (i % 9) as u8));
    }
    s
}
