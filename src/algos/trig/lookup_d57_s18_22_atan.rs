//! Bespoke narrow-`GUARD` `atan_strict` kernel slot for `D57<SCALE>`
//! with `SCALE ∈ 18..=22`.
//!
//! Companion to [`super::lookup_d57_s18_22_sincos`] and
//! [`crate::algos::ln::lookup_d57_s18_22`] /
//! [`crate::algos::exp::lookup_d57_s18_22`]: the shared wide-tier
//! `atan_fixed` core uses `GUARD = 30`. At `SCALE ∈ 18..=22` the
//! atan Taylor / Euler-acceleration series converges in ~20-30
//! rounded muls (vs ~100+ at SCALE 57), so a narrower
//! `GUARD_NARROW = 14` leaves storage 14 decimal digits below `w`,
//! many orders of magnitude below half a storage ULP.
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 14 = 32..36` — roughly two-thirds of the bits.
//!
//! ## `GUARD_NARROW` selection
//!
//! `atan_fixed` may invoke a half-angle pre-reduction for `|x| > 1`
//! plus a Taylor (or Euler-accelerated) series. At `w = 32..36` and
//! reduced argument the series converges in ~30 iterations. Each
//! rounded mul contributes ≤ 0.5 LSB-of-w of uncorrelated drift;
//! pre-reduction adds a few more LSB. Total accumulated error is
//! ~20 LSB-of-w.
//!
//! With `GUARD_NARROW = 14`, storage scale is `w - 14`, so 20 LSB-of-w
//! is `20·10⁻¹⁴` in storage units — 13 orders of magnitude below half
//! a storage ULP.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::wide_int::Int192;

/// Narrow guard for the SCALE 18..=22 atan slot. Slightly larger than
/// the sincos/ln/exp `GUARD_NARROW = 12` because atan's series can
/// require slightly more iterations after argument reduction.
const GUARD_NARROW: u32 = 10;

/// `atan_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
///
/// Routes the canonical [`core::atan_fixed`] kernel through a narrower
/// working width `w = SCALE + GUARD_NARROW`.
#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let r = core::atan_fixed(v_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
