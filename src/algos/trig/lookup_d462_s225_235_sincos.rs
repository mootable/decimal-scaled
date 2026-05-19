//! Narrow-`GUARD` `sin_strict` + `cos_strict` + `tan_strict` kernel
//! slot for `D462<SCALE>` with `SCALE ∈ 225..=235`.
//!
//! Sibling to [`crate::algos::trig::lookup_d153_s70_82_sincos`]. The
//! shared `wide_trig_d462::sin_fixed` / `cos_fixed` core uses
//! `GUARD = 30`, sized for the worst-case `SCALE = MAX_SCALE = 461`
//! Taylor series. At `SCALE ∈ 225..=235` the storage target only
//! needs `~SCALE + 10 = 235..245` working digits to clear half a
//! storage ULP, so the default `SCALE + 30 = 255..265` width is
//! over-provisioned. Narrowing reclaims the surplus by passing
//! `GUARD_NARROW = 10` to the same shared `sin_fixed` / `cos_fixed` /
//! `sin_cos_fixed` core.
//!
//! Tang sin/cos at small `M` was confirmed to LOSE at the narrow tier
//! (see the D57 sibling), so this slot does not attempt a table; the
//! reclaim is purely from narrowing the working-width passed in.

#![cfg(any(feature = "d462", feature = "x-wide"))]

use crate::types::widths::wide_trig_d462 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int1536;

const GUARD_NARROW: u32 = 10;

#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

#[inline]
#[must_use]
pub(crate) fn sin_cos_strict<const SCALE: u32>(
    raw: Int1536,
    mode: RoundingMode,
    which: Which,
) -> Int1536 {
    if raw == Int1536::ZERO {
        return match which {
            Which::Sin => Int1536::ZERO,
            Which::Cos => {
                let ten: Int1536 = crate::wide_int::wide_cast::<u128, Int1536>(10);
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
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int1536, mode: RoundingMode) -> Int1536 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Sin)
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int1536, mode: RoundingMode) -> Int1536 {
    sin_cos_strict::<SCALE>(raw, mode, Which::Cos)
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int1536, mode: RoundingMode) -> Int1536 {
    if raw == Int1536::ZERO {
        return Int1536::ZERO;
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let (sin_w, cos_w) = core::sin_cos_fixed(v_w, w);
    if cos_w == core::zero() {
        panic!("D462::tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    let r = core::div(sin_w, cos_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
