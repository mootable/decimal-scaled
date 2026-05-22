//! Narrow-`GUARD` `atan_strict` kernel slot for `D153<SCALE>` with
//! `SCALE ∈ 70..=82`.
//!
//! Sibling to [`crate::algos::trig::lookup_d57_s18_22_atan`]. The
//! shared core `atan_fixed` uses `GUARD = 30` (sized for SCALE 153).
//! At `SCALE ∈ 70..=82` the atan Euler-accelerated series converges
//! well below that budget, so `GUARD_NARROW = 12` is comfortable —
//! ~12·10⁻¹² in storage units of residual drift, many orders of
//! magnitude below half a storage ULP for `SCALE ≤ 82`.
//!
//! Working width drops from `SCALE + 30 = 100..112` to
//! `SCALE + 12 = 82..94`.

#![cfg(any(feature = "d153", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d153 as core;
use crate::int::types::Int;

/// Narrow guard for the SCALE 70..=82 atan slot. Slightly larger than
/// the sincos/exp/ln `GUARD_NARROW = 10` because atan's series can
/// require slightly more iterations after argument reduction.
const GUARD_NARROW: u32 = 12;

#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: Int<8>, mode: RoundingMode) -> Int<8> {
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let r = core::atan_fixed(v_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
