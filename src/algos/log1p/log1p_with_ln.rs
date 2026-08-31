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
//! written as `(1 + t).ln_strict()` — a decimal method on its own
//! operand type re-enters a sibling decimal policy, the layering
//! inversion `docs/ARCHITECTURE.md` forbids.
//!
//! [`exp_generic::ln_fixed`]: crate::algos::exp::exp_generic::ln_fixed

use crate::algos::support::wide_trig_core as wtc;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `1 + t` at working scale `w = SCALE + guard`, exactly, in the work
/// integer `S`.
#[inline]
fn one_plus_t_at_w<St: BigInt, S: BigInt>(raw: St, guard: u32, w: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    crate::algos::exp::exp_generic::one::<S>(w) + wtc::to_work_scaled_g::<St, S>(raw, guard)
}

/// `ln 2` at working scale `w`. Reads the const-scale table on the base
/// guard (the hot path, where `w` is the monomorphisation's own
/// `SCALE + base_guard`) and only falls to the runtime working-scale
/// lookup on a Ziv escalation — the same split
/// [`wtc::ln_series_g`] makes.
#[inline]
fn ln2_at<S: BigInt>(w: u32, base_w: u32) -> S {
    if w == base_w {
        crate::consts::ln2_by_scale::<S>(w, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    } else {
        crate::consts::ln2_by_working_scale::<S>(
            w,
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
    base_guard: u32,
    st_max: St,
    st_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    super::guard_domain::<St>(raw, SCALE);
    let base_w = SCALE + base_guard;
    let r = wtc::round_to_storage_directed_g::<St, S>(
        base_guard,
        SCALE,
        mode,
        st_max,
        st_min,
        |guard| {
            let w = SCALE + guard;
            crate::algos::exp::exp_generic::ln_fixed::<S>(
                one_plus_t_at_w::<St, S>(raw, guard, w),
                w,
                ln2_at::<S>(w, base_w),
            )
        },
    );
    // The same analytic sub-resolution adjust the artanh kernel applies:
    // this composition runs the Series `ln` core directly, which (unlike
    // the Tang path) does not carry `adjust_ln_near_one` of its own.
    super::adjust_near_zero::<St>(r, raw, mode)
}

/// The `_approx` sibling of [`log1p_with_ln_g`]: a SINGLE shot at the
/// caller's `working_digits`, with no Ziv escalation.
///
/// # Panics
///
/// Panics if `t <= -1`, or if the result leaves the storage range.
#[inline]
#[must_use]
pub(crate) fn log1p_with_ln_approx_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
    raw: St,
    working_digits: u32,
    st_max: St,
    st_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    super::guard_domain::<St>(raw, SCALE);
    let w = SCALE + working_digits;
    let r = crate::algos::exp::exp_generic::ln_fixed::<S>(
        one_plus_t_at_w::<St, S>(raw, working_digits, w),
        w,
        ln2_at::<S>(w, w),
    );
    let out = wtc::round_to_storage_with_g::<St, S>(r, w, SCALE, mode, st_max, st_min);
    super::adjust_near_zero::<St>(out, raw, mode)
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

/// Tier-generic entry to [`log1p_with_ln_approx_g`]. See
/// [`log1p_with_ln`].
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn log1p_with_ln_approx<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    working_digits: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    log1p_with_ln_approx_g::<C::Storage, C::W, SCALE>(
        raw,
        working_digits,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}
