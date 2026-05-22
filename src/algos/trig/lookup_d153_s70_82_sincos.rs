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

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d153 as core;
use crate::int::types::Int;

const GUARD_NARROW: u32 = 10;

#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int<8>,
    mode: RoundingMode,
    which: Which,
) -> Int<8> {
    if raw == Int::<8>::ZERO {
        return match which {
            Which::Sin => Int::<8>::ZERO,
            Which::Cos => {
                let ten: Int<8> = crate::int::types::traits::wide_cast::<u128, Int<8>>(10);
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
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int<8>, mode: RoundingMode) -> Int<8> {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int<8>, mode: RoundingMode) -> Int<8> {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int<8>, mode: RoundingMode) -> Int<8> {
    if raw == Int::<8>::ZERO {
        return Int::<8>::ZERO;
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
        panic!("D153::tan: cosine is zero (argument is an odd multiple of pi/2)");
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
