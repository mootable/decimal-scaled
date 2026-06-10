// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Bespoke narrow-`GUARD` `tan_strict` kernel slot for `D57<SCALE>`
//! with `SCALE ∈ 18..=22`.
//!
//! The shared wide-tier core sizes `GUARD = 30` for the worst case
//! `SCALE = 57` (~200 rounded Taylor multiplies at a 87-digit working
//! width). At narrow scales the storage target only needs ~22-26
//! working digits to clear 0.5 LSB at storage, so running the
//! `sin_cos_fixed` kernel at the full `SCALE + 30 = 48..52` width is
//! over-provisioned. Halving the working width roughly halves the
//! per-iteration `Int<3>` `mul` / `div` cost.
//!
//! Unlike the SCALE 44..=56 sibling (the generic Tang sincos kernel
//! `super::sincos_tang`), this slot does NOT carry an
//! argument-reduction table. The reduction-table win only pays back
//! when the unreduced Taylor series exceeds ~30 terms (which happens
//! above ~SCALE 30 with the default `GUARD`); at SCALE 18..=22 the
//! generic Taylor evaluator already converges in ~15-20 terms. The
//! reclaim here is purely from narrowing the working width passed to
//! the canonical `sin_cos_fixed` kernel.
//!
//! The band's `sin_strict` / `cos_strict` route through the shared
//! directed-aware generic `super::sincos_narrow` (the same narrow-GUARD
//! reclaim, plus the Ziv escalation and bounded-extremum adjust the
//! directed modes need) — see `policy::trig`'s D57 forward arms. Only
//! `tan` keeps this bespoke slot, for its shared single
//! `sin_cos_fixed` evaluation.
//!
//! ## `GUARD_NARROW` selection
//!
//! The shared core's `GUARD = 30` is documented (in
//! `crate::macros::wide_transcendental`) as supporting `~200 ×
//! 0.5 = 100 LSB-of-w` accumulated drift across the longest series
//! the wide tiers run. At SCALE 18..=22 the Taylor series on
//! `r ∈ [0, π/4]` converges in ~20-30 rounded multiplies; the matching
//! worst-case drift is ~30 × 0.5 = 15 LSB-of-w, many orders of
//! magnitude below half a storage ULP for any `SCALE ≤ 22` at the
//! band's guard. The probe set the value empirically: every value
//! tried in the 8..=14 band held the wide-tier baseline within 1 LSB
//! of storage on the existing trig regression suite.
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 8 = 26..30` — roughly two-thirds of the bits, which
//! matches the ~25-30% wall-clock reclaim observed in the probe.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::int::types::Int;

/// Narrow guard for the SCALE 18..=22 slot. See module docs for the
/// derivation and headroom.
const GUARD_NARROW: u32 = 8;

/// `tan_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22` — shares one
/// `sin_cos_fixed` between numerator and denominator, narrowed to
/// `GUARD_NARROW`. Panics if `cos(self) == 0` (odd multiples of π/2).
#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    if raw == Int::<3>::ZERO {
        return Int::<3>::ZERO;
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_scaled(raw, GUARD_NARROW);
    let (sin_w, cos_w) = core::sin_cos_fixed::<SCALE>(v_w, w);
    if cos_w == core::zero() {
        panic!("D57::tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let r = core::div(sin_w, cos_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
