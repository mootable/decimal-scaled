// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p(t) = ln(1 + t)` by composition — the general-argument kernel,
//! ONE generic algorithm over the storage integer `St` and the work
//! integer `S`.
//!
//! `1 + t` is formed **at the working scale**, where it is EXACT: `t`
//! lifts to `t · 10^guard` and `1` is `10^w`, both exact integers, so
//! the sum carries every significant digit of `t`. The binary
//! floating-point cancellation that motivates a separate `log1p` cannot
//! occur in fixed point, which is why this composition is a legitimate
//! `log1p` kernel and not a precision compromise.
//!
//! The `ln` core is [`exp_generic::ln_fixed`] — the same width-generic
//! kernel [`wide_trig_core::ln_series_g`](wtc::ln_series_g) runs, with
//! its own multi-level sqrt range reduction. That reduction is what
//! makes this the right kernel away from zero, where
//! [`log1p_artanh`](super::log1p_artanh)'s unreduced series slows down
//! and finally stops converging (`|t / (2 + t)| → 1`).
//!
//! # Layering
//!
//! This calls the `ln` KERNEL directly and downward. It must never be
//! written as `(1 + t).ln()` — a decimal method on its own
//! operand type re-enters a sibling decimal policy, the layering
//! inversion `docs/ARCHITECTURE.md` forbids.
//!
//! [`exp_generic::ln_fixed`]: crate::algos::exp::exp_generic::ln_fixed

use crate::algos::support::wide_trig_core as wtc;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `1 + t` at `working_scale = SCALE + guard_digits`, exactly, in the
/// work integer `S`.
#[inline]
fn one_plus_t_at_w<St: BigInt, S: BigInt>(raw: St, guard_digits: u32, working_scale: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    crate::algos::exp::exp_generic::one::<S>(working_scale)
        + wtc::to_work_scaled_g::<St, S>(raw, guard_digits)
}

/// `ln 2` at `working_scale`. Reads the const-scale table on the base
/// guard (the hot path, where `working_scale` is the monomorphisation's
/// own `SCALE + base_guard_digits`) and only falls to the runtime
/// working-scale lookup on a Ziv escalation — the same split
/// [`wtc::ln_series_g`] makes.
#[inline]
fn ln2_at<S: BigInt>(working_scale: u32, base_working_scale: u32) -> S {
    if working_scale == base_working_scale {
        crate::consts::ln2_by_scale::<S>(
            working_scale, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    } else {
        crate::consts::ln2_by_working_scale::<S>(
            working_scale,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }
}

/// `log1p(t)` at storage `St`, computed in the work integer `S` and
/// correctly rounded to `SCALE` under `mode`.
///
/// Same contract and same caller-supplied bounds as
/// [`log1p_artanh_g`](super::log1p_artanh::log1p_artanh_g); only the
/// working-scale core differs.
///
/// # Panics
///
/// Panics if `t <= -1`, or if the result leaves the storage range.
#[inline]
#[must_use]
pub(crate) fn log1p_with_ln_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
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
    let base_working_scale = SCALE + base_guard_digits;
    let rounded = wtc::round_to_storage_directed_g::<St, S>(
        base_guard_digits,
        SCALE,
        mode,
        storage_max,
        storage_min,
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            crate::algos::exp::exp_generic::ln_fixed::<S>(
                one_plus_t_at_w::<St, S>(raw, guard_digits, working_scale),
                working_scale,
                ln2_at::<S>(working_scale, base_working_scale),
            )
        },
    );
    // The same analytic sub-resolution adjust the artanh kernel applies:
    // this composition runs the Series `ln` core directly, which (unlike
    // the Tang path) does not carry `adjust_ln_near_one` of its own.
    super::adjust_near_zero::<St, S, SCALE>(rounded, raw, mode)
}

/// Tier-generic entry to [`log1p_with_ln_g`] — sources the work integer
/// `C::W`, the base guard `C::GUARD` and the storage bounds from the
/// wide tier's `Core`, mirroring
/// [`log1p_artanh`](super::log1p_artanh::log1p_artanh).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn log1p_with_ln<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    log1p_with_ln_g::<C::Storage, C::W, SCALE>(
        raw,
        C::GUARD,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}

