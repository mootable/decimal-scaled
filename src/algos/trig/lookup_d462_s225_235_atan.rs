//! Narrow-`GUARD` `atan_strict` kernel slot for `D462<SCALE>` with
//! `SCALE ∈ 225..=235`.
//!
//! Sibling to [`crate::algos::trig::lookup_d153_s70_82_atan`]. The
//! shared core `atan_fixed` uses `GUARD = 30` (sized for the
//! worst-case `SCALE = MAX_SCALE = 461`). At `SCALE ∈ 225..=235` the
//! atan Euler-accelerated series converges well below that budget, so
//! `GUARD_NARROW = 12` is comfortable — ~12·10⁻¹² in storage units of
//! residual drift, many orders of magnitude below half a storage ULP.
//!
//! Working width drops from `SCALE + 30 = 255..265` to
//! `SCALE + 12 = 237..247`.

#![cfg(any(feature = "d462", feature = "x-wide"))]

use crate::types::widths::wide_trig_d462 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int1536;

/// Narrow guard for the SCALE 225..=235 atan slot. Slightly larger
/// than the sincos / exp / ln `GUARD_NARROW = 10` because atan's
/// series can require a touch more headroom after argument reduction.
const GUARD_NARROW: u32 = 12;

#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: Int1536, mode: RoundingMode) -> Int1536 {
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let r = core::atan_fixed(v_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
