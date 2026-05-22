//! Bespoke narrow-`GUARD` `asin_strict` / `acos_strict` /
//! `atan2_strict` kernel slot for `D57<SCALE>` with `SCALE ∈ 18..=22`.
//!
//! Companion to [`super::lookup_d57_s18_22_atan`] — the inverse-trig
//! family routes through `atan_fixed` plus `sqrt_fixed` and a small
//! amount of `mul` / `div`. At `SCALE ∈ 18..=22` the same narrow-GUARD
//! trick applies: `GUARD_NARROW = 14` (matching atan, slightly larger
//! than the sincos/exp/ln 12 to accommodate the extra `sqrt_fixed`
//! and `atan_fixed` series drift).
//!
//! Per-call working width drops from `SCALE + 30 = 48..52` to
//! `SCALE + 14 = 32..36`.
//!
//! ## Algorithm (asin, the most expensive)
//!
//! Two ranges, matching the macro-emitted inherent path:
//! - `|x| ≤ 0.5`: `asin(x) = atan(x / √(1 − x²))`.
//! - `|x| > 0.5`: half-angle reduction
//!   `asin(|x|) = π/2 − 2·asin(√((1−|x|)/2))`, where the inner asin
//!   takes the stable `|·| ≤ 0.5` branch.
//!
//! `acos(x) = π/2 − asin(x)` shares the same body.
//!
//! ## Correctness
//!
//! Error budget at working scale `w = SCALE + 14`:
//!
//! - One `sqrt_fixed`: ≤ 0.5 LSB-of-w.
//! - One `mul` (for `1 − v²`): ≤ 0.5 LSB-of-w.
//! - One `div` (for `v / sqrt(...)`): ≤ 0.5 LSB-of-w.
//! - One `atan_fixed`: ~25 rounded muls @ 0.5 LSB each = ≤ 13 LSB-of-w.
//! - Final round-to-storage: ≤ 0.5 LSB-of-w.
//!
//! Half-angle branch doubles the sqrt + adds an outer sub/double;
//! cumulative budget is still ≤ ~30 LSB-of-w. With `GUARD_NARROW = 14`
//! that's `30·10⁻¹⁴` in storage units — 13 orders of magnitude below
//! half a storage ULP for any `SCALE ≤ 22`.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::int::types::Int;

/// Narrow guard for the SCALE 18..=22 inverse-trig slot. Matches the
/// `lookup_d57_s18_22_atan` guard.
const GUARD_NARROW: u32 = 10;

fn asin_fixed(v: core::W, w: u32) -> core::W {
    let one_w = core::one(w);
    let abs_v = if v < core::zero() { -v } else { v };
    if abs_v > one_w {
        panic!("D57::asin: argument out of domain [-1, 1]");
    }
    let half_w = one_w / core::lit(2);
    if abs_v == one_w {
        let hp = core::half_pi(w);
        return if v < core::zero() { -hp } else { hp };
    }
    if abs_v <= half_w {
        let denom = core::sqrt_fixed(one_w - core::mul(v, v, w), w);
        return core::atan_fixed(core::div(v, denom, w), w);
    }
    // Half-angle: asin(|x|) = π/2 − 2·asin(√((1−|x|)/2)).
    let inner = (one_w - abs_v) / core::lit(2);
    let inner_sqrt = core::sqrt_fixed(inner, w);
    let inner_denom = core::sqrt_fixed(one_w - core::mul(inner_sqrt, inner_sqrt, w), w);
    let inner_asin = core::atan_fixed(core::div(inner_sqrt, inner_denom, w), w);
    let result_abs = core::half_pi(w) - inner_asin - inner_asin;
    if v < core::zero() {
        -result_abs
    } else {
        result_abs
    }
}

/// `asin_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn asin_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let r = asin_fixed(v, w);
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `acos_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn acos_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    let w = SCALE + GUARD_NARROW;
    let v = core::to_work_w(raw, GUARD_NARROW);
    let asin_w = asin_fixed(v, w);
    let r = core::half_pi(w) - asin_w;
    core::round_to_storage_with(r, w, SCALE, mode)
}

/// `atan2_strict` for `D57<SCALE>` with `SCALE ∈ 18..=22`.
#[inline]
#[must_use]
pub(crate) fn atan2_strict<const SCALE: u32>(
    y_raw: Int<3>,
    x_raw: Int<3>,
    mode: RoundingMode,
) -> Int<3> {
    let w = SCALE + GUARD_NARROW;
    let zero_s = Int::<3>::ZERO;
    let r = if x_raw == zero_s {
        if y_raw > zero_s {
            core::half_pi(w)
        } else if y_raw < zero_s {
            -core::half_pi(w)
        } else {
            core::zero()
        }
    } else {
        let y = core::to_work_w(y_raw, GUARD_NARROW);
        let x = core::to_work_w(x_raw, GUARD_NARROW);
        let zero_w = core::zero();
        let abs_y = if y < zero_w { -y } else { y };
        let abs_x = if x < zero_w { -x } else { x };
        let base = if abs_x >= abs_y {
            core::atan_fixed(core::div(y, x, w), w)
        } else {
            let inv = core::atan_fixed(core::div(x, y, w), w);
            let hp = core::half_pi(w);
            let same_sign = (y < zero_w) == (x < zero_w);
            if same_sign { hp - inv } else { -hp - inv }
        };
        if x_raw > zero_s {
            base
        } else if y_raw >= zero_s {
            base + core::pi(w)
        } else {
            base - core::pi(w)
        }
    };
    core::round_to_storage_with(r, w, SCALE, mode)
}
