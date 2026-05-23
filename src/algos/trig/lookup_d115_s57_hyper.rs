//! Bespoke narrow-`GUARD` `sinh_strict` / `cosh_strict` /
//! `tanh_strict` kernel slot for `D115<SCALE>` with `SCALE = 57`.
//!
//! Direct port of [`crate::algos::trig::lookup_d57_s18_22_hyper`].
//! Shares the Tang-style exp from
//! [`crate::algos::exp::lookup_d115_s57_tang`] and avoids the second
//! `exp_fixed` call via the `1/eˣ` reciprocal identity.
//!
//! ## Algorithm
//!
//! ```text
//! ex  = exp(v)
//! enx = 1 / ex          (exp(-v) identity)
//! sinh = (ex - enx) / 2
//! cosh = (ex + enx) / 2
//! tanh = (ex - enx) / (ex + enx)
//! ```
//!
//! ## Correctness
//!
//! Per-op error at working scale `w = SCALE + 8 = 65` is bounded by
//! ~15 LSB-of-w (one Tang exp ≤ ~12 LSB + reciprocal + add/sub +
//! /2 + final round). At w=65 that's `~1.5·10⁻⁶⁴`; storage half-ULP
//! at SCALE=57 is `5·10⁻⁵⁸`, so the budget closes with ~6 orders to
//! spare.

#![cfg(any(feature = "d115", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d115 as core;
use crate::int::types::Int;

/// Narrow guard for the SCALE = 57 hyperbolic slot. Matches the Tang
/// exp narrow guard so the `pow10_cached(w)` slot is shared.
const GUARD_NARROW: u32 = 8;

/// Joint `(ex, enx)` pair via one Tang exp + one reciprocal divide.
#[inline]
fn ex_enx(v: core::W, w: u32) -> (core::W, core::W) {
    let ex = crate::algos::exp::exp_tang::tang_exp_fixed::<
        crate::types::widths::wide_trig_d115::Core,
        128,
        false,
    >(v, w);
    let one_w = core::one(w);
    let enx = core::div(one_w, ex, w);
    (ex, enx)
}

/// `sinh_strict` for `D115<SCALE>` with `SCALE = 57`.
#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int<6>, mode: RoundingMode) -> Int<6> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex - enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `cosh_strict` for `D115<SCALE>` with `SCALE = 57`.
#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int<6>, mode: RoundingMode) -> Int<6> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex + enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `tanh_strict` for `D115<SCALE>` with `SCALE = 57`.
#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int<6>, mode: RoundingMode) -> Int<6> {
    let zero = Int::<6>::from_i128(0);
    if raw != zero {
        // Small-argument linear band: tanh(x) = x − x³/3 + … , the cubic
        // below one ULP yet strictly positive, so the true value sits
        // just inside the grid line `raw`. No finite-precision exp path
        // can resolve the sub-ULP cubic, so the directed result is the
        // analytic decision below (nearest modes return `raw`).
        let thresh_exp = SCALE - (SCALE + 2) / 3;
        let thresh = Int::<6>::from_i128(10).pow(thresh_exp);
        if raw.abs() <= thresh {
            return crate::support::rounding::tiny_odd_compressing_directed(
                raw,
                zero,
                Int::<6>::from_i128(1),
                mode,
            );
        }
    }
    // General path: outside the tiny band the kernel error is far below
    // half a storage ULP, so a single narrowing is correctly rounded for
    // every mode.
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let r = core::div(ex - enx, ex + enx, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
