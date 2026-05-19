//! Narrow-`GUARD` `sin_strict` + `cos_strict` + `tan_strict` kernel
//! slot for `D153<SCALE>` with `SCALE ∈ 70..=82`.
//!
//! Sibling to [`crate::algos::trig::lookup_d57_s18_22_sincos`]. The
//! shared `wide_trig_d153::sin_fixed` / `cos_fixed` core uses
//! `GUARD = 30`, sized for the worst-case `SCALE = 153` Taylor series.
//! At `SCALE ∈ 70..=82` the storage target only needs ~80-92 working
//! digits to clear 0.5 LSB at storage, so the default
//! `SCALE + 30 = 100..112` width is over-provisioned. Narrowing to
//! `SCALE + 10 = 80..92` roughly two-thirds the bits.
//!
//! Tang sin/cos at smaller M was confirmed to LOSE at narrow tier
//! (see the D57 sibling), so this slot does not attempt a table; the
//! reclaim is purely from narrowing the working width passed to the
//! canonical `sin_fixed` / `cos_fixed` / `sin_cos_fixed` kernels.
//!
//! ## `GUARD_NARROW` selection
//!
//! At `w = SCALE + 10` the Taylor series for `sin_fixed` / `cos_fixed`
//! on `r ∈ [0, π/4]` converges in ~25-35 rounded multiplies; the
//! matching worst-case drift is ~35 × 0.5 = 18 LSB-of-w. With
//! `GUARD_NARROW = 10` that's `18·10⁻¹⁰` in storage units — many
//! orders of magnitude below half a storage ULP for `SCALE ≤ 82`.

#![cfg(any(feature = "d153", feature = "wide"))]

use crate::types::widths::wide_trig_d153 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int512;

const GUARD_NARROW: u32 = 10;

#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int512,
    mode: RoundingMode,
    which: Which,
) -> Int512 {
    if raw == Int512::ZERO {
        return match which {
            Which::Sin => Int512::ZERO,
            Which::Cos => {
                let ten: Int512 = crate::wide_int::wide_cast::<u128, Int512>(10);
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

#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int512, mode: RoundingMode) -> Int512 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int512, mode: RoundingMode) -> Int512 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int512, mode: RoundingMode) -> Int512 {
    if raw == Int512::ZERO {
        return Int512::ZERO;
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let (sin_w, cos_w) = core::sin_cos_fixed(v_w, w);
    if cos_w == core::zero() {
        panic!("D153::tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let r = core::div(sin_w, cos_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
