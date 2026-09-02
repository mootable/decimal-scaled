// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p(t) = 2·artanh(t / (2 + t))` — the small-`|t|` kernel, ONE
//! generic algorithm over the storage integer `St` and the work
//! integer `S`.
//!
//! The maths lives in [`exp_generic::log1p_fixed`], the width-generic
//! working-scale kernel already shipped for `acosh` / `atanh`. This file
//! is the storage-facing shell around it: the family domain guard, the
//! lift to the working scale, and the correctly-rounded narrow back to
//! storage through the shared Ziv escalation.
//!
//! # Applicability
//!
//! The artanh series `u + u³/3 + u⁵/5 + …` carries **no range
//! reduction**: its ratio is `u²`, so it converges fast only while
//! `|u| = |t / (2 + t)|` is small, and not at all as `|u| → 1` (that is,
//! as `t → -1` or `t → ∞`). `policy::log1p` therefore routes this kernel
//! over a bounded band around zero and
//! [`log1p_with_ln`](super::log1p_with_ln) everywhere else. Feeding it
//! an out-of-band argument yields a series-cap truncation, not a
//! correctly-rounded result.
//!
//! [`exp_generic::log1p_fixed`]: crate::algos::exp::exp_generic::log1p_fixed

use crate::algos::support::wide_trig_core as wtc;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `log1p(t)` at storage `St`, computed in the work integer `S` and
/// correctly rounded to `SCALE` under `mode`.
///
/// Every width routes here through `policy::log1p`, which supplies the
/// width's work integer `S`, its base guard, and its storage bounds.
/// `recompute(guard_digits)` is re-entered by the Ziv walker at
/// successively deeper guards until the deciding digit resolves, so the
/// result is within 0.5 ULP for every mode.
///
/// `storage_max` / `storage_min` are the tier's storage bounds (`MAX`/`MIN` are
/// inherent consts on `Int<N>`, not on [`BigInt`], so the caller
/// supplies them — the same contract as
/// [`wtc::round_to_storage_directed_g`]).
///
/// # Why the walker is the tail-signed one
///
/// Escalation cannot settle every input here. At `t = ±10^-(S/2)` the whole
/// series vanishes at the working scale, so the kernel returns `2u` and the
/// first digit that could decide the round sits just past `3·S/2` — beyond
/// the walker's `W::BITS/8 − 8` reach at the top scale of the wide tiers
/// (`D462<346>` and `D616<590>` are the cells where it bites). The residual
/// then reads as an exact tie at every guard the walker can reach, and the
/// mode's tie-break decides a tie that is not there: `HalfTowardZero` never
/// bumps and fails at both signs, `HalfToEven` bumps only on an odd kept
/// digit and so fails at whichever sign presents an even one.
///
/// The residual is blind, but the kernel is not: it knows the side its own
/// dropped terms put the truth on, so it hands that back with the value and
/// [`wtc::round_to_storage_tail_signed_g`] rounds to the nearer neighbour
/// outright instead of breaking a tie. Where the kernel cannot prove a side
/// it returns `None` and the walker behaves exactly as it did before.
///
/// # Panics
///
/// Panics if `t <= -1`, or if the result leaves the storage range.
#[inline]
#[must_use]
pub(crate) fn log1p_artanh_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
    raw: St,
    base_guard_digits: u32,
    storage_max: St,
    storage_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    super::guard_domain::<St>(raw, SCALE);
    let rounded = wtc::round_to_storage_tail_signed_g::<St, S>(
        base_guard_digits,
        SCALE,
        mode,
        storage_max,
        storage_min,
        |guard_digits| {
            crate::algos::exp::exp_generic::log1p_fixed_tagged::<S>(
                wtc::to_work_scaled_g::<St, S>(raw, guard_digits),
                SCALE + guard_digits,
                guard_digits,
            )
        },
    );
    super::adjust_near_zero::<St, S, SCALE>(rounded, raw, mode)
}

/// The `_approx` sibling of [`log1p_artanh_g`]: a SINGLE shot at the
/// caller's `working_digits`, with no Ziv escalation — the same
/// precision/latency trade every other `*_approx` transcendental makes.
///
/// # Panics
///
/// Panics if `t <= -1`, or if the result leaves the storage range.
#[inline]
#[must_use]
pub(crate) fn log1p_artanh_approx_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
    raw: St,
    working_digits: u32,
    storage_max: St,
    storage_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    super::guard_domain::<St>(raw, SCALE);
    let working_scale = SCALE + working_digits;
    let working_value = crate::algos::exp::exp_generic::log1p_fixed::<S>(
        wtc::to_work_scaled_g::<St, S>(raw, working_digits),
        working_scale,
    );
    let rounded = wtc::round_to_storage_with_g::<St, S>(
        working_value, working_scale, SCALE, mode, storage_max, storage_min);
    super::adjust_near_zero::<St, S, SCALE>(rounded, raw, mode)
}

/// Tier-generic entry to [`log1p_artanh_g`] — sources the work integer
/// `C::W`, the base guard `C::GUARD` and the storage bounds from the
/// wide tier's `Core`, exactly as
/// [`wide_trig_core::ln_series`](wtc::ln_series) does for `ln`. Saves
/// the policy from repeating five arguments per wide arm.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn log1p_artanh<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    log1p_artanh_g::<C::Storage, C::W, SCALE>(
        raw,
        C::GUARD,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}

/// Tier-generic entry to [`log1p_artanh_approx_g`]. See
/// [`log1p_artanh`].
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn log1p_artanh_approx<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    working_digits: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    log1p_artanh_approx_g::<C::Storage, C::W, SCALE>(
        raw,
        working_digits,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}
