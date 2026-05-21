//! Narrow-`GUARD` `sin_strict` + `cos_strict` + `tan_strict` kernel
//! slot for `D307<SCALE>` with `SCALE ∈ 140..=160`.
//!
//! Sibling to [`crate::algos::trig::lookup_d153_s70_82_sincos`]. The
//! shared `wide_trig_d307::sin_fixed` / `cos_fixed` core uses
//! `GUARD = 30`, sized for the worst-case `SCALE = 306` Taylor series.
//! At `SCALE ∈ 140..=160` the storage target only needs ~150-170
//! working digits to clear 0.5 LSB at storage, so the default
//! `SCALE + 30 = 170..190` width is over-provisioned. Narrowing to
//! `SCALE + 8 = 148..168` reclaims a chunk of the Taylor-stage cost.
//!
//! Tang sin/cos at smaller M was confirmed to LOSE at narrow tier
//! (see the D57 sibling), so this slot does not attempt a table; the
//! reclaim is purely from narrowing the working width passed to the
//! canonical `sin_fixed` / `cos_fixed` / `sin_cos_fixed` kernels.
//!
//! ## `GUARD_NARROW` selection
//!
//! At `w = SCALE + 8` the Taylor series for `sin_fixed` / `cos_fixed`
//! on `r ∈ [0, π/4]` converges in ~45-55 rounded multiplies; the
//! matching worst-case drift is ~55 × 0.5 = 28 LSB-of-w. With
//! `GUARD_NARROW = 8` that's `28·10⁻⁸` in storage units — many
//! orders of magnitude below half a storage ULP for `SCALE ≤ 160`.

#![cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]

use crate::types::widths::wide_trig_d307 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int1024;

const GUARD_NARROW: u32 = 8;

#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int1024,
    mode: RoundingMode,
    which: Which,
) -> Int1024 {
    if raw == Int1024::ZERO {
        return match which {
            Which::Sin => Int1024::ZERO,
            Which::Cos => {
                let ten: Int1024 = crate::wide_int::wide_cast::<u128, Int1024>(10);
                ten.pow(SCALE)
            }
        };
    }

    // Directed modes decide which side of a storage grid line the true
    // value falls; near a grid line (e.g. cos near ±1, sin near a quadrant
    // multiple) the working-scale approximation can land on the wrong side.
    // Route through the shared Ziv escalation; nearest modes narrow once.
    core::round_to_storage_directed(GUARD_NARROW, SCALE, mode, |guard| {
        let v_w = core::to_work_w(raw, guard);
        match which {
            Which::Sin => core::sin_fixed(v_w, SCALE + guard),
            Which::Cos => core::cos_fixed(v_w, SCALE + guard),
        }
    })
}

#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int1024, mode: RoundingMode) -> Int1024 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int1024, mode: RoundingMode) -> Int1024 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int1024, mode: RoundingMode) -> Int1024 {
    if raw == Int1024::ZERO {
        return Int1024::ZERO;
    }
    // Near a pole (input close to an odd multiple of π/2) the range-
    // reduced residue folds toward ±π/2 where cos(r) → 0, so the
    // tangent quotient sin(r)/cos(r) grows without bound. Dividing at a
    // fixed working scale then amplifies the working-scale rounding
    // error by `1/cos(r) ≈ |tan|`, the same conditioning class as the
    // exp `2^k` reassembly. Hold 0.5 ULP at storage by lifting the
    // working scale by ~log10(|tan|) guard digits, derived cheaply from
    // a base-width probe quotient, then recompute sin/cos at the lifted
    // scale and divide there.
    //
    // Reference: Muller, *Elementary Functions: Algorithms and
    // Implementation* (3rd ed., 2016), §11.1 — range-reduction error
    // budget for poles of the reduced function.
    let w0 = SCALE + GUARD_NARROW;
    let v0 = core::to_work_w(raw, GUARD_NARROW);
    let (sin0, cos0) = core::sin_cos_fixed(v0, w0);
    if cos0 == core::zero() {
        panic!("D307::tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let probe = core::div(sin0, cos0, w0);
    let extra = super::near_pole_tan::tan_extra_digits(core::bit_length(probe), w0);
    if extra == 0 {
        return core::round_to_storage_with(probe, w0, SCALE, mode);
    }
    let w = w0 + extra;
    let v_w = core::to_work_w(raw, GUARD_NARROW + extra);
    let (sin_w, cos_w) = core::sin_cos_fixed(v_w, w);
    let r = core::div(sin_w, cos_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
