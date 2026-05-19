//! Narrow-`GUARD` `atan_strict` kernel slot for `D307<SCALE>` with
//! `SCALE ∈ 140..=160`.
//!
//! Sibling to [`crate::algos::trig::lookup_d153_s70_82_atan`]. The
//! shared core `atan_fixed` uses `GUARD = 30` (sized for SCALE 306).
//! At `SCALE ∈ 140..=160` the atan Euler-accelerated series converges
//! well below that budget, so `GUARD_NARROW = 10` is comfortable —
//! ~10·10⁻¹⁰ in storage units of residual drift, many orders of
//! magnitude below half a storage ULP for `SCALE ≤ 160`.
//!
//! Working width drops from `SCALE + 30 = 170..190` to
//! `SCALE + 10 = 150..170`.

#![cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]

use crate::types::widths::wide_trig_d307 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int1024;

/// Narrow guard for the SCALE 140..=160 atan slot. Slightly larger
/// than the sincos/exp/ln `GUARD_NARROW = 8` because atan's series
/// can require slightly more iterations after argument reduction.
const GUARD_NARROW: u32 = 10;

#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: Int1024, mode: RoundingMode) -> Int1024 {
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let r = core::atan_fixed(v_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
