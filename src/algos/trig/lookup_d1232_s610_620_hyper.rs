//! Narrow-`GUARD` `sinh_strict` / `cosh_strict` / `tanh_strict` kernel
//! slot for `D1232<SCALE>` with `SCALE ∈ 610..=620`.
//!
//! Sibling to [`crate::algos::trig::lookup_d616_s300_315_hyper`]. The
//! `exp(-v) ≡ 1/exp(v)` identity is already applied at the
//! macro-emitted inherent shells; this slot adds the further win of
//! routing the single remaining `exp_fixed` through the Tang lookup at
//! `GUARD_NARROW = 8`.
//!
//! ## Routing status
//!
//! Carried for completeness alongside the per-width hyperbolic Tang
//! stack and for downstream re-probing. The D616 hyperbolic Tang slot
//! at SCALE 300..=315 was bench-trialled and rejected (break-even
//! against the canonical `*_strict_with`); D1232's deeper Int<256>
//! working width is expected to be at least as flat. The lookup
//! kernels are NOT wired into [`crate::policy::trig`] surface
//! dispatch; the macro-emitted inherent `*_strict_with` shells stay in
//! place.
//!
//! ## Algorithm
//!
//! ```text
//! ex  = tang_exp_fixed(v_w, w)
//! enx = 1 / ex             (exp(-v) identity)
//! sinh = (ex - enx) / 2
//! cosh = (ex + enx) / 2
//! tanh = (ex - enx) / (ex + enx)
//! ```

#![cfg(any(feature = "d1232", feature = "xx-wide"))]
#![allow(dead_code)]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d1232 as core;
use crate::int::types::Int;

/// Narrow guard for the SCALE 610..=620 hyperbolic slot — matches the
/// sibling Tang exp/ln guard so the per-thread `pow10_w` cache slot is
/// shared.
const GUARD_NARROW: u32 = crate::algos::exp::lookup_d1232_s610_620_tang::GUARD_FOR_HYPER;

/// Joint `(ex, enx)` pair shared by sinh / cosh / tanh.
#[inline]
fn ex_enx(v: core::W, w: u32) -> (core::W, core::W) {
    let ex = crate::algos::exp::lookup_d1232_s610_620_tang::tang_exp_fixed(v, w);
    let one_w = core::one(w);
    let enx = core::div(one_w, ex, w);
    (ex, enx)
}

/// `sinh_strict` for `D1232<SCALE>` with `SCALE ∈ 610..=620`.
#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int<64>, mode: RoundingMode) -> Int<64> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex - enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `cosh_strict` for `D1232<SCALE>` with `SCALE ∈ 610..=620`.
#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int<64>, mode: RoundingMode) -> Int<64> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex + enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `tanh_strict` for `D1232<SCALE>` with `SCALE ∈ 610..=620`.
#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int<64>, mode: RoundingMode) -> Int<64> {
    let zero = Int::<64>::from_i128(0);
    if raw != zero {
        // Small-argument linear band: tanh(x) = x − x³/3 + … , the cubic
        // below one ULP yet strictly positive, so the true value sits
        // just inside the grid line `raw`. No finite-precision exp path
        // can resolve the sub-ULP cubic, so the directed result is the
        // analytic decision below (nearest modes return `raw`).
        let thresh_exp = SCALE - (SCALE + 2) / 3;
        let thresh = Int::<64>::from_i128(10).pow(thresh_exp);
        if raw.abs() <= thresh {
            return crate::support::rounding::tiny_odd_compressing_directed(
                raw,
                zero,
                Int::<64>::from_i128(1),
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
