// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `quantize_pow10` — set a decimal's quantum by scaling its stored
//! integer by `10^|TARGET_SCALE − SCALE|`, at a fixed storage width.
//!
//! One generic kernel over the storage limb count `N`; the direction is
//! decided by the two const scales, so both branches fold away per
//! monomorphisation and each concrete `(N, SCALE, TARGET_SCALE)` compiles
//! to exactly one of the three cases.
//!
//! * `TARGET_SCALE == SCALE` — bit-identity; the stored integer is the
//!   answer.
//! * `TARGET_SCALE > SCALE` — **scale-up**: multiply by `10^shift`, which
//!   is exact. `10^shift` comes from the baked POW10 table
//!   ([`crate::consts::pow10::dispatch_int`]) — a table read, not a
//!   run-time constant generation. `None` reports the overflow; the caller
//!   owns the panic (its message names the tier).
//! * `TARGET_SCALE < SCALE` — **scale-down**: divide by `10^shift` with
//!   `mode` rounding, routed through the crate's `÷10^scale` matcher
//!   ([`crate::algos::support::rescale::dispatch_wide_pow10`]) so the
//!   kernel choice (MG single-chunk / MG chain / baked-reciprocal Newton)
//!   is that matcher's, not this kernel's. The rounding decision inside
//!   those kernels is the shared
//!   [`should_bump`](crate::support::rounding::should_bump) decider.
//!
//! # Why the scale-down does not divide twice
//!
//! The rounding rule needs the truncated quotient AND the remainder. A
//! typed `raw / divisor` followed by `raw % divisor` is two independent
//! full-width divisions for one division's worth of information; the
//! rescale matcher's kernels produce both in one pass over the magnitude
//! limbs and apply `mode` in place.

use crate::consts::pow10;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Quantizes the stored integer `raw` from `SCALE` to `TARGET_SCALE` at
/// storage width `N`, rounding any discarded digits per `mode`.
///
/// Returns `None` **only** when a scale-up overflows `Int<N>`; every
/// scale-down direction is `Some` (the magnitude only shrinks).
#[inline]
pub(crate) fn quantize_pow10<const N: usize, const SCALE: u32, const TARGET_SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>>
where
    Limbs<N>: ComputeLimbs,
{
    if TARGET_SCALE == SCALE {
        return Some(raw);
    }
    if TARGET_SCALE > SCALE {
        // Scale-up is exact. `shift <= MAX_SCALE` (the scale cap the tier
        // aliases enforce), and `10^MAX_SCALE` fits the tier's storage by
        // construction, so the table entry never exceeds `N` limbs; past
        // the baked range `dispatch_int` falls back to the same
        // `TEN.pow(shift)` square-and-multiply this replaced.
        let multiplier = pow10::dispatch_int::<N>(TARGET_SCALE - SCALE);
        return raw.checked_mul(multiplier);
    }
    // Scale-down: one rounded `÷10^shift` through the rescale matcher.
    Some(crate::algos::support::rescale::dispatch_wide_pow10(
        raw,
        SCALE - TARGET_SCALE,
        mode,
    ))
}
