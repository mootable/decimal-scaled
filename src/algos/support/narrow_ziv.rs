// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Narrow-tier (D18 / D38) near-tie Ziv escalation plumbing.
//!
//! The narrow strict kernels evaluate on the 256-bit `Fixed`
//! intermediate at the fixed working scale `w = SCALE + STRICT_GUARD`
//! and historically narrowed in a SINGLE shot (or, for sin/cos, an
//! escalation hard-capped at the 75-digit `Fixed` constant window).
//! That terminal mis-rounds any input whose deciding digit lies below
//! the fixed working scale — the constructible family is an exact
//! rational Taylor partial landing exactly ON a rounding boundary with
//! the transcendental tail below reach (`sin(1e-38)` at D38<38> has its
//! `x³/6` deviation at fraction depth 115; `cosh(1e-19)` lands `x²/2`
//! exactly on the half with the `x⁴/24` tail at depth 77).
//!
//! The fix follows the wide campaign's shape: the fast path keeps the
//! single `Fixed` shot but narrows through
//! [`Fixed::round_to_i128_clear_of_tie`] — residuals clear of the
//! mode's deciding boundary by the near-tie band exit at today's cost —
//! and a near-tie escalates through the SAME generic Ziv walkers the
//! wide tiers run ([`wide_trig_core::round_to_storage_directed_g`] and
//! siblings), recomputing in the wider [`WZiv`] work integer via the
//! width-generic kernels (`trig_generic`, `exp_generic`). `Int<24>`
//! (1536 bits ≈ 462 decimal digits) reaches a probe depth of
//! ~`BITS/8 = 192` digits — comfortably past every constructible
//! narrow-tier deciding depth (≤ `3·38 = 114` for the odd-series
//! deviations, ≤ `2·38 + 2 = 78` for the even-series half families) —
//! so every constructible family resolves before the cap; only the
//! non-constructible Table-Maker's-Dilemma class remains at the cap,
//! the same contract the wide tiers carry at their precision horizon.
//!
//! [`Fixed::round_to_i128_clear_of_tie`]: crate::algos::support::fixed::Fixed::round_to_i128_clear_of_tie
//! [`wide_trig_core::round_to_storage_directed_g`]: crate::algos::support::wide_trig_core::round_to_storage_directed_g

use crate::algos::support::wide_trig_core as wtc;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// The narrow tiers' near-tie escalation work integer — the same
/// `Int<24>` the integer-regime `exp` fallback
/// (`exp_series_2limb::WNarrow`) already runs in on every build.
pub(crate) type WZiv = Int<24>;

/// Lifts a raw `i128` storage value to the working scale: `raw · 10^guard`
/// as a signed [`WZiv`]. Total for every `i128` (the magnitude goes
/// through `from_u128`, so `i128::MIN` does not wrap).
#[inline]
pub(crate) fn lift(raw: i128, guard: u32) -> WZiv {
    let mag = WZiv::from_u128(raw.unsigned_abs())
        * crate::consts::pow10::dispatch::<WZiv>(guard);
    if raw < 0 { -mag } else { mag }
}

/// `π · 10^w`, correctly rounded, at a runtime working scale.
#[inline]
pub(crate) fn pi_w(w: u32) -> WZiv {
    crate::consts::pi_by_working_scale::<WZiv>(w, RoundingMode::HalfToEven)
}

/// `ln 2 · 10^w`, correctly rounded, at a runtime working scale.
#[inline]
pub(crate) fn ln2_w(w: u32) -> WZiv {
    crate::consts::ln2_by_working_scale::<WZiv>(w, RoundingMode::HalfToEven)
}

/// `ln 10 · 10^w`, correctly rounded, at a runtime working scale.
#[inline]
pub(crate) fn ln10_w(w: u32) -> WZiv {
    crate::consts::ln10_by_working_scale::<WZiv>(w, RoundingMode::HalfToEven)
}

/// The plain directed/nearest Ziv walker at the narrow storage —
/// `recompute(guard)` returns the kernel value at working scale
/// `scale + guard` in [`WZiv`]. Result-sign-agnostic at the cap (no
/// never-exact tail assumption): the unresolved endgame snaps to the
/// clean base narrowing, which is the correct answer for an EXACTLY
/// boundary-valued input (`powf(4, 0.5)`, `log_4(8)`).
#[inline]
pub(crate) fn walk(
    base_guard: u32,
    scale: u32,
    mode: RoundingMode,
    recompute: impl FnMut(u32) -> WZiv,
) -> i128 {
    wtc::round_to_storage_directed_g::<Int<2>, WZiv>(
        base_guard,
        scale,
        mode,
        Int::<2>::MAX,
        Int::<2>::MIN,
        recompute,
    )
    .as_i128()
}

/// The `never_exact` walker — for a kernel whose true value is provably
/// NEVER on a storage grid line (`exp`/`exp2`/`cosh` after their exact
/// pins): an unresolved residual at the cap carries a strictly positive
/// sub-resolution tail. Mirrors the wide `exp`/`cosh` shape.
#[inline]
pub(crate) fn walk_never_exact(
    base_guard: u32,
    scale: u32,
    mode: RoundingMode,
    recompute: impl FnMut(u32) -> WZiv,
) -> i128 {
    wtc::round_to_storage_directed_never_exact_g::<Int<2>, WZiv>(
        base_guard,
        scale,
        mode,
        Int::<2>::MAX,
        Int::<2>::MIN,
        recompute,
    )
    .as_i128()
}

/// Option-contract wrapper over [`walk`] for the kernels whose overflow
/// contract is a returned `None` (`ln`/`log`/`powf`/`exp`): the walker's
/// range check PANICS past storage, so a near-tie AT the storage extreme
/// (where the walker's ±1 could leave range) keeps the single-shot
/// verdict `base` instead — `base` is the plain rounding of the same
/// working value, and a tie that deep at the extreme is the
/// Table-Maker's-Dilemma residue either way. `base == None` (out of
/// range) propagates.
#[inline]
pub(crate) fn walk_checked(
    base: Option<i128>,
    base_guard: u32,
    scale: u32,
    mode: RoundingMode,
    recompute: impl FnMut(u32) -> WZiv,
) -> Option<i128> {
    match base {
        None => None,
        Some(b) if b.unsigned_abs() >= (i128::MAX as u128) - 1 => Some(b),
        Some(_) => Some(walk(base_guard, scale, mode, recompute)),
    }
}

/// [`walk_checked`] with the `never_exact` polarity (`exp` / `exp2`).
#[inline]
pub(crate) fn walk_checked_never_exact(
    base: Option<i128>,
    base_guard: u32,
    scale: u32,
    mode: RoundingMode,
    recompute: impl FnMut(u32) -> WZiv,
) -> Option<i128> {
    match base {
        None => None,
        Some(b) if b.unsigned_abs() >= (i128::MAX as u128) - 1 => Some(b),
        Some(_) => Some(walk_never_exact(base_guard, scale, mode, recompute)),
    }
}

/// The near-special-point walker (`acosh` near 1, `atanh` near ±1):
/// forces a confirm recompute even in the nearest modes. Mirrors the
/// wide `acosh`/`atanh` shape.
#[inline]
pub(crate) fn walk_near_special(
    base_guard: u32,
    scale: u32,
    mode: RoundingMode,
    recompute: impl FnMut(u32) -> WZiv,
) -> i128 {
    wtc::round_to_storage_directed_near_special_g::<Int<2>, WZiv>(
        base_guard,
        scale,
        mode,
        Int::<2>::MAX,
        Int::<2>::MIN,
        recompute,
    )
    .as_i128()
}
