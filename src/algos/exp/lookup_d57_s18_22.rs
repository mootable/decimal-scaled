//! Bespoke narrow-`GUARD` `exp_strict` kernel slot for `D57<SCALE>`
//! with `SCALE ∈ 18..=22`.
//!
//! Mirrors the [`crate::algos::trig::lookup_d57_s18_22_sincos`] tactic:
//! the shared wide-tier `exp_fixed` core uses `GUARD = 30` so the
//! 0.5-LSB-per-iteration rounded-multiply drift is contained across
//! the worst-case Taylor series at `SCALE = 57` (~40+ terms after
//! Cody-Waite range reduction). At `SCALE ∈ 18..=22` the same
//! reduced-argument Taylor converges in ~25 rounded muls, so the
//! worst-case drift is ~12 LSB-of-w. A narrower `GUARD_NARROW = 12`
//! leaves storage 12 decimal digits below `w`, which is ~12·10⁻¹² in
//! storage units — many orders of magnitude below half a storage ULP
//! for any `SCALE ≤ 22`.
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 12 = 30..34` — roughly two-thirds of the bits.
//!
//! ## `GUARD_NARROW` selection
//!
//! `exp_fixed` Cody-Waite-reduces `v = k·ln 2 + s`, then evaluates
//! `exp(s)` via Taylor on `|s| ≤ ln 2 / 2 ≈ 0.35`, then `2^k` shift.
//! At `w = 30..34` and `|s| ≤ 0.35` the Taylor `δⁿ/n!` term drops
//! below 1 LSB-of-w at `n ≈ 25`. Each rounded mul contributes
//! ≤ 0.5 LSB-of-w of uncorrelated drift; total accumulated error is
//! ~12 LSB-of-w. The `k·ln 2` subtraction and `2^k` shift each cost
//! ≤ 1 LSB-of-w. Total ≤ ~15 LSB-of-w.
//!
//! With `GUARD_NARROW = 12`, storage scale is `w - 12`, so 15 LSB-of-w
//! is `15·10⁻¹²` in storage units — 11 orders of magnitude below half
//! a storage ULP.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::types::widths::wide_trig_d57 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int192;

/// Narrow guard for the SCALE 18..=22 slot. See module docs for the
/// derivation and headroom.
const GUARD_NARROW: u32 = 12;

/// `exp_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
///
/// Routes the canonical [`core::exp_fixed`] kernel through a narrower
/// working width `w = SCALE + GUARD_NARROW`. Short-circuits `raw == 0`
/// to `Dxx<SCALE>::ONE` (`10^SCALE`).
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    if raw == Int192::ZERO {
        let ten: Int192 = crate::wide_int::wide_cast::<u128, Int192>(10);
        return ten.pow(SCALE);
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let r = core::exp_fixed(v_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
