//! Bespoke narrow-`GUARD` `sinh_strict` / `cosh_strict` /
//! `tanh_strict` kernel slot for `D57<SCALE>` with `SCALE ∈ 18..=22`.
//!
//! sinh / cosh / tanh all share the pair `(eˣ, e⁻ˣ)`. The
//! macro-emitted inherent path runs each `exp_fixed` at
//! `GUARD = 30`. At `SCALE ∈ 18..=22` the same narrow-GUARD trick
//! as [`crate::algos::exp::lookup_d57_s18_22`] applies — the
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
//! enx = exp(-v)
//! sinh = (ex - enx) / 2
//! cosh = (ex + enx) / 2
//! tanh = (ex - enx) / (ex + enx)
//! ```
//!
//! Same arithmetic as the macro-emitted inherent methods, just lifted
//! into this file so it can run at the narrower `w`.
//!
//! ## Correctness
//!
//! Error budget at working scale `w = SCALE + 12` (in LSB-of-w):
//!
//! - Two `exp_fixed` calls: ≤ 24 LSB combined (worst-case Taylor drift).
//! - One add (cosh) or sub (sinh): ≤ 1 LSB.
//! - One divide-by-2 (cosh / sinh): ≤ 0.5 LSB.
//! - Final round-to-storage: ≤ 0.5 LSB.
//!
//! Total ≤ ~26 LSB-of-w = ~26·10⁻¹² in storage units — many orders of
//! magnitude below half a storage ULP for any SCALE ≤ 22.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::types::widths::wide_trig_d57 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int192;

/// Narrow guard for the SCALE 18..=22 hyperbolic slot. Matches the
/// exp / ln narrow guard so a sinh/cosh call shares the same `pow10_w`
/// cache slot with neighboring exp / ln invocations.
const GUARD_NARROW: u32 = 12;

/// `sinh_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let ex = core::exp_fixed(v, w);
    let enx = core::exp_fixed(-v, w);
    let two = core::lit(2);
    let r = (ex - enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `cosh_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let ex = core::exp_fixed(v, w);
    let enx = core::exp_fixed(-v, w);
    let two = core::lit(2);
    let r = (ex + enx) / two;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `tanh_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let ex = core::exp_fixed(v, w);
    let enx = core::exp_fixed(-v, w);
    let r = core::div(ex - enx, ex + enx, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
