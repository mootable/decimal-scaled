//! Bespoke narrow-`GUARD` `sin_strict` + `cos_strict` kernel slot for
//! `D57<SCALE>` with `SCALE ∈ 18..=22`.
//!
//! The shared wide-tier core sizes `GUARD = 30` for the worst case
//! `SCALE = 57` (~200 rounded Taylor multiplies at a 87-digit working
//! width). At narrow scales the storage target only needs ~22-26
//! working digits to clear 0.5 LSB at storage, so running the
//! `sin_fixed` / `cos_fixed` kernels at the full `SCALE + 30 = 48..52`
//! width is over-provisioned. Halving the working width roughly halves
//! the per-iteration `Int192` `mul` / `div` cost.
//!
//! Unlike the SCALE 44..=56 sibling
//! ([`super::lookup_d57_s44_56_sincos`]), this slot does NOT carry an
//! argument-reduction table. The reduction-table win only pays back
//! when the unreduced Taylor series exceeds ~30 terms (which happens
//! above ~SCALE 30 with the default `GUARD`); at SCALE 18..=22 the
//! generic Taylor evaluator already converges in ~15-20 terms. The
//! reclaim here is purely from narrowing the working width passed to
//! the canonical `sin_fixed` / `cos_fixed` kernels.
//!
//! ## `GUARD_NARROW` selection
//!
//! The shared core's `GUARD = 30` is documented (in
//! `crate::macros::wide_transcendental`) as supporting `~200 ×
//! 0.5 = 100 LSB-of-w` accumulated drift across the longest series
//! the wide tiers run. At SCALE 18..=22 the Taylor series for
//! `sin_fixed` / `cos_fixed` on `r ∈ [0, π/4]` converges in ~20-30
//! rounded multiplies; the matching worst-case drift is ~30 × 0.5 =
//! 15 LSB-of-w. A `GUARD_NARROW = 12` leaves storage 12 decimal
//! digits below `w`, so 15 LSB-of-w is ~15·10⁻¹² in storage units —
//! many orders of magnitude below half a storage ULP for any
//! `SCALE ≤ 22`. The probe set this empirically: every value tried
//! in the 8..=14 band held the wide-tier baseline within 1 LSB of
//! storage on the existing trig regression suite; `12` is the
//! comfortable middle.
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 12 = 30..34` — roughly two-thirds of the bits, which
//! matches the ~25-30% wall-clock reclaim observed in the probe.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::core_type::wide_trig_d57 as core;
use crate::rounding::RoundingMode;
use crate::wide_int::Int192;

/// Narrow guard for the SCALE 18..=22 slot. See module docs for the
/// derivation and headroom.
const GUARD_NARROW: u32 = 12;

/// Sin/cos selector. Both share every stage of the reduction; the
/// selector only picks which output to return.
#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

/// Shared narrow-`GUARD` `sin_strict` / `cos_strict` kernel for
/// `D57<SCALE>` with `SCALE ∈ 18..=22`.
///
/// Reroutes the canonical [`core::sin_fixed`] / [`core::cos_fixed`]
/// kernels through a narrower working width
/// `w = SCALE + GUARD_NARROW`. The argument lift uses
/// [`core::to_work_w`] so the storage raw is scaled by `10^GUARD_NARROW`
/// (matching the narrower `w`).
#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int192,
    mode: RoundingMode,
    which: Which,
) -> Int192 {
    // sin(0) = 0, cos(0) = 1 short-circuit — match `wide_kernel`.
    if raw == Int192::ZERO {
        return match which {
            Which::Sin => Int192::ZERO,
            // D57::<SCALE>::ONE raw is 10^SCALE in storage units.
            Which::Cos => {
                let ten: Int192 = crate::wide_int::wide_cast::<u128, Int192>(10);
                ten.pow(SCALE)
            }
        };
    }

    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let result = match which {
        Which::Sin => core::sin_fixed(v_w, w),
        Which::Cos => core::cos_fixed(v_w, w),
    };
    core::round_to_storage_with(result, w, SCALE, mode)
}

/// Thin entry shim — `sin_strict` for `D57<SCALE>` with
/// `SCALE ∈ 18..=22`. See [`sin_cos_strict`] for the algorithm.
#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

/// Thin entry shim — `cos_strict` for `D57<SCALE>` with
/// `SCALE ∈ 18..=22`. See [`sin_cos_strict`] for the algorithm.
#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}
