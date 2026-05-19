//! Bespoke narrow-`GUARD` `ln_strict` kernel slot for `D57<SCALE>`
//! with `SCALE ∈ 18..=22`.
//!
//! Mirrors the [`crate::algos::trig::lookup_d57_s18_22_sincos`] tactic:
//! the shared wide-tier `ln_fixed` core uses `GUARD = 30` so the
//! 0.5-LSB-per-iteration rounded-multiply drift is contained across
//! the worst-case `~p/2` artanh series at `SCALE = 57`. At `SCALE ∈
//! 18..=22` the artanh series with Brent's multi-level `sqrt`
//! pre-reduction converges in ~10-15 rounded muls, so the worst-case
//! drift is ~7-8 LSB-of-w. A narrower `GUARD_NARROW = 12` leaves
//! storage 12 decimal digits below `w`, which is ~12·10⁻¹² in storage
//! units — many orders of magnitude below half a storage ULP for any
//! `SCALE ≤ 22`.
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 12 = 30..34` — roughly two-thirds of the bits, which
//! matches the wall-clock reclaim the same trick produced on the
//! sin/cos kernel.
//!
//! ## `GUARD_NARROW` selection
//!
//! ln_fixed runs Brent's argument reduction with `sqrt_l ≈ √p_bits / 4`
//! pre-sqrt steps, then an artanh series whose iteration count is
//! `~p / (2 + 2·sqrt_l)`. At `w = 30..34`, `p_bits ≈ 100`, so
//! `sqrt_l ≈ 2` and the artanh loop runs ~12 iterations. Each rounded
//! mul contributes ≤ 0.5 LSB-of-w of uncorrelated drift; total
//! accumulated error is ~6 LSB-of-w. Plus 2 sqrt_fixed calls (each
//! ≤ 0.5 LSB-of-w) and 2 div_cached / mul_cached for `t` and `t²`
//! (≤ 1 LSB-of-w each). Total ≤ ~10 LSB-of-w.
//!
//! With `GUARD_NARROW = 12`, storage scale is `w - 12`, so 10 LSB-of-w
//! is `10·10⁻¹²` in storage units — 11 orders of magnitude below half
//! a storage ULP.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::types::widths::wide_trig_d57 as core;
use crate::support::rounding::RoundingMode;
use crate::wide_int::Int192;

/// Narrow guard for the SCALE 18..=22 slot. See module docs for the
/// derivation and headroom.
const GUARD_NARROW: u32 = 12;

/// `ln_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
///
/// Routes the canonical [`core::ln_fixed`] kernel through a narrower
/// working width `w = SCALE + GUARD_NARROW`. The argument lift uses
/// [`core::to_work_w`] so the storage raw is scaled by
/// `10^GUARD_NARROW` (matching the narrower `w`). Panics if `raw <= 0`
/// — matches the wide_kernel contract.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    if raw <= Int192::ZERO {
        panic!("D57::ln: argument must be positive");
    }
    let w = SCALE + GUARD_NARROW;
    let v_w = core::to_work_w(raw, GUARD_NARROW);
    let r = core::ln_fixed(v_w, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}
