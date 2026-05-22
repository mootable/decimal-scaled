//! Narrow-`GUARD` `sinh_strict` / `cosh_strict` / `tanh_strict` kernel
//! slot for `D616<SCALE>` with `SCALE ∈ 300..=315`.
//!
//! Sibling to [`crate::algos::trig::lookup_d153_s70_82_hyper`]. sinh /
//! cosh / tanh all share the pair `(eˣ, e⁻ˣ)`. The cross-cutting
//! `exp(-v) ≡ 1/exp(v)` identity is already applied at the
//! macro-emitted inherent shells; this slot adds the further win of
//! routing the single remaining `exp_fixed` through the Tang lookup at
//! `GUARD_NARROW = 10`.
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
//!
//! ## Correctness
//!
//! Error budget at working scale `w = SCALE + 10`:
//!
//! - One `tang_exp_fixed` call: ≤ ~36 LSB-of-w (Tang reduction +
//!   Taylor on δ at D616's working width).
//! - One `1/ex` divide (rounded half-to-even): ≤ 0.5 LSB.
//! - One add / sub: ≤ 1 LSB.
//! - One divide-by-2 (cosh / sinh) or one final divide (tanh): ≤ 0.5 LSB.
//! - Final round-to-storage: ≤ 0.5 LSB.
//!
//! Total ≤ ~40 LSB-of-w ≈ 40·10⁻¹⁰ in storage units — many orders
//! of magnitude below half a storage ULP for any `SCALE ≤ 315`.

#![cfg(any(feature = "d616", feature = "x-wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d616 as core;
use crate::wide_int::Int2048;

/// Narrow guard for the SCALE 300..=315 hyperbolic slot — matches the
/// sibling Tang exp/ln guard so the per-thread `pow10_w` cache slot is
/// shared.
const GUARD_NARROW: u32 = crate::algos::exp::lookup_d616_s300_315_tang::GUARD_FOR_HYPER;

/// Joint `(ex, enx)` pair shared by sinh / cosh / tanh.
#[inline]
fn ex_enx(v: core::W, w: u32) -> (core::W, core::W) {
    let ex = crate::algos::exp::lookup_d616_s300_315_tang::tang_exp_fixed(v, w);
    let one_w = core::one(w);
    let enx = core::div(one_w, ex, w);
    (ex, enx)
}

/// `sinh_strict` for `D616<SCALE>` with `SCALE ∈ 300..=315`.
#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int2048, mode: RoundingMode) -> Int2048 {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex - enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `cosh_strict` for `D616<SCALE>` with `SCALE ∈ 300..=315`.
#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int2048, mode: RoundingMode) -> Int2048 {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex + enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `tanh_strict` for `D616<SCALE>` with `SCALE ∈ 300..=315`.
#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int2048, mode: RoundingMode) -> Int2048 {
    let zero = Int2048::from_i128(0);
    if raw != zero {
        // Small-argument linear band: tanh(x) = x − x³/3 + … , the cubic
        // below one ULP yet strictly positive, so the true value sits
        // just inside the grid line `raw`. No finite-precision exp path
        // can resolve the sub-ULP cubic, so the directed result is the
        // analytic decision below (nearest modes return `raw`).
        let thresh_exp = SCALE - (SCALE + 2) / 3;
        let thresh = Int2048::from_i128(10).pow(thresh_exp);
        if raw.abs() <= thresh {
            return crate::support::rounding::tiny_odd_compressing_directed(
                raw,
                zero,
                Int2048::from_i128(1),
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
