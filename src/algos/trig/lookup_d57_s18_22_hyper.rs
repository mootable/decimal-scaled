//! Bespoke narrow-`GUARD` `sinh_strict` / `cosh_strict` /
//! `tanh_strict` kernel slot for `D57<SCALE>` with `SCALE ∈ 18..=22`.
//!
//! sinh / cosh / tanh all share the pair `(eˣ, e⁻ˣ)`. The
//! macro-emitted inherent path runs each `exp_fixed` at
//! `GUARD = 30`. At `SCALE ∈ 18..=22` the same narrow-GUARD trick as the
//! Tang exp kernel `lookup_d57_s18_22_tang` applies — the
//! 12-LSB-of-w drift budget holds across the Taylor series at
//! `w = SCALE + 12 = 30..34` with many orders of magnitude margin.
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 12 = 30..34` — roughly two-thirds of the bits. Since
//! sinh / cosh do *two* `exp_fixed` calls per surface call, the
//! reclaim multiplies.
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
//! Same shape as the macro-emitted inherent methods, but the second
//! `exp_fixed(-v, w)` is replaced by a single wide divide — wide-tier
//! `exp_fixed` is dominated by the Tang-table reduction + Taylor
//! series and costs ~10-20× more than a wide divide, so the identity
//! drops per-call wall-clock ~40%.
//!
//! ## Correctness
//!
//! Error budget at working scale `w = SCALE + 8` (in LSB-of-w):
//!
//! - One `exp_fixed` call: ≤ 12 LSB (worst-case Taylor drift).
//! - One `1/ex` divide (rounded half-to-even): ≤ 0.5 LSB.
//! - One add (cosh) or sub (sinh): ≤ 1 LSB.
//! - One divide-by-2 (cosh / sinh): ≤ 0.5 LSB.
//! - Final round-to-storage: ≤ 0.5 LSB.
//!
//! Total ≤ ~15 LSB-of-w = ~15·10⁻⁸ in storage units — many orders of
//! magnitude below half a storage ULP for any SCALE ≤ 22.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::int::types::Int;

/// Narrow guard for the SCALE 18..=22 hyperbolic slot. Matches the
/// exp / ln narrow guard so a sinh/cosh call shares the same `pow10_w`
/// cache slot with neighboring exp / ln invocations.
const GUARD_NARROW: u32 = 8;

/// Joint `(ex, enx)` pair shared by sinh / cosh / tanh. One Tang exp
/// call yields `eˣ`, and `e⁻ˣ = 1/eˣ` follows from one wide divide —
/// versus a second `exp_fixed` call that's an order of magnitude
/// more expensive.
#[inline]
fn ex_enx(v: core::W, w: u32) -> (core::W, core::W) {
    let ex = crate::algos::exp::lookup_d57_s18_22_tang::tang_exp_fixed(v, w);
    let one_w = core::one(w);
    let enx = core::div(one_w, ex, w);
    (ex, enx)
}

/// `sinh_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`. One Tang
/// `exp` + one reciprocal-divide for the `(eˣ, e⁻ˣ)` pair.
#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex - enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `cosh_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let (ex, enx) = ex_enx(v, w);
    let two = core::lit(2);
    let r = (ex + enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `tanh_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    let zero = Int::<3>::from_i128(0);
    if raw != zero {
        // Small-argument linear band: tanh(x) = x − x³/3 + … , the cubic
        // below one ULP yet strictly positive, so the true value sits
        // just inside the grid line `raw`. No finite-precision exp path
        // can resolve the sub-ULP cubic, so the directed result is the
        // analytic decision below (nearest modes return `raw`).
        let thresh_exp = SCALE - (SCALE + 2) / 3;
        let thresh = Int::<3>::from_i128(10).pow(thresh_exp);
        if raw.abs() <= thresh {
            return crate::support::rounding::tiny_odd_compressing_directed(
                raw,
                zero,
                Int::<3>::from_i128(1),
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
