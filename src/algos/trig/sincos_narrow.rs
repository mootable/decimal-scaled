// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic narrow-`GUARD` `sin_strict` / `cos_strict` /
//! `tan_strict` kernels (the deep-`SCALE` sincos bands that keep the
//! canonical Taylor core but at a narrowed working width).
//!
//! Tang sin/cos at a smaller `M` was confirmed to LOSE at these tiers
//! (see the D57 sibling), so these slots do not attempt a table; the
//! reclaim is purely from narrowing the working width passed to the
//! canonical `sin_fixed` / `cos_fixed` / `sin_cos_fixed` kernels from the
//! tier's over-provisioned `GUARD = 30` down to the band's `GUARD`.
//!
//! ## Layering
//!
//! Algorithm functions (`docs/ARCHITECTURE.md` → "Layering direction"):
//! they compute only through the [`WideTrigCore`] trait surface and the
//! width-free `near_pole_tan` helper; they never call a method on a
//! decimal type. `policy::trig` (the forward family) calls them *down*.
//!
//! Collapses the per-tier D153 70..=82, D307 140..=160, and D462
//! 225..=235 sincos narrow kernels into one set of generics
//! over `C: WideTrigCore`, the `SCALE`, and the band's narrow guard
//! `GUARD` (plus a `NEAR_POLE` flag for `tan`, which D462 omits).

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::support::rounding::RoundingMode;

/// Which component the caller wants out of the shared kernel.
#[derive(Copy, Clone)]
pub(crate) enum Which {
    Sin,
    Cos,
}

/// Shared narrow-`GUARD` directed `sin_strict` / `cos_strict` for a wide
/// tier — generic over `C`, the `SCALE`, and the band's narrow guard.
#[inline]
#[must_use]
fn sin_cos_strict<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    which: Which,
) -> C::Storage {
    if raw == C::storage_zero() {
        return match which {
            Which::Sin => C::storage_zero(),
            Which::Cos => C::storage_one(SCALE),
        };
    }

    // Directed modes decide which side of a storage grid line the true
    // value falls; near a grid line (cos near ±1, sin near a quadrant
    // multiple) the working-scale approximation can land on the wrong
    // side. Route through the shared Ziv escalation; nearest modes narrow
    // once.
    let r = C::round_to_storage_directed(GUARD, SCALE, mode, &mut |guard| {
        let v_w = C::to_work_scaled(raw, guard);
        match which {
            Which::Sin => C::sin_fixed::<SCALE>(v_w, SCALE + guard),
            Which::Cos => C::cos_fixed::<SCALE>(v_w, SCALE + guard),
        }
    });
    // Near an extremum the deviation from ±1 can sit below any reachable
    // working scale, so the kernel rounds to exactly ±10^SCALE and a directed
    // mode lands on the wrong side; sin/cos are strictly interior for raw != 0,
    // so the side is known a priori. See `wide_trig_core::adjust_bounded_extremum`.
    crate::algos::support::wide_trig_core::adjust_bounded_extremum::<C, SCALE>(r, raw, mode)
}

/// Narrow `sin_strict` for a wide tier — generic over `C`, `SCALE`, the
/// band's narrow guard `GUARD`.
#[inline]
#[must_use]
pub(crate) fn sin_narrow_with_taylor<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    sin_cos_strict::<C, SCALE, GUARD>(raw, mode, Which::Sin)
}

/// Narrow `cos_strict` for a wide tier — generic over `C`, `SCALE`, the
/// band's narrow guard `GUARD`.
#[inline]
#[must_use]
pub(crate) fn cos_narrow_with_taylor<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    sin_cos_strict::<C, SCALE, GUARD>(raw, mode, Which::Cos)
}

/// Narrow `tan_strict` for a wide tier — generic over `C`, `SCALE`, the
/// band's narrow guard `GUARD`, and `NEAR_POLE`.
///
/// When `NEAR_POLE` is set, a base-width probe sizes a per-call working
/// lift (`near_pole_tan::tan_extra_digits`) so the `1/cos(r) ≈ |tan|`
/// amplification near an odd multiple of π/2 still lands at ≤ 0.5 ULP at
/// storage (Muller, *Elementary Functions* 3rd ed., §11.1). When clear,
/// the band's guard already covers the worst case, so the kernel divides
/// once at `w = SCALE + GUARD`.
#[inline]
#[must_use]
pub(crate) fn tan_narrow_with_taylor<
    C: WideTrigCore,
    const SCALE: u32,
    const GUARD: u32,
    const NEAR_POLE: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    let w0 = SCALE + GUARD;
    let v0 = C::to_work_scaled(raw, GUARD);
    let (sin0, cos0) = C::sin_cos_fixed::<SCALE>(v0, w0);
    if cos0 == C::zero() {
        panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
    }
    // Near-tie escapes — see `wide_trig_core::tan_series`: clear-of-band
    // residuals keep the single-shot cost; the band escalates through
    // the ratio walker (a deciding digit can sit below the fixed w).
    let tie_walker = |base_guard: u32| -> C::Storage {
        C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
            let w = SCALE + guard;
            let (s, c) = C::sin_cos_fixed::<SCALE>(C::to_work_scaled(raw, guard), w);
            if c == C::zero() {
                panic!("wide-tier tan: cosine is zero (argument is an odd multiple of pi/2)");
            }
            C::div(s, c, w)
        })
    };
    if !NEAR_POLE {
        let r = C::div(sin0, cos0, w0);
        if let Some(st) = crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<
            C::Storage,
            C::W,
        >(r, w0, SCALE, mode, C::storage_max(), C::storage_min())
        {
            return st;
        }
        return tie_walker(GUARD);
    }
    let probe = C::div(sin0, cos0, w0);
    let extra = super::near_pole_tan::tan_extra_digits(C::bit_length(probe), w0);
    if extra == 0 {
        if let Some(st) = crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<
            C::Storage,
            C::W,
        >(probe, w0, SCALE, mode, C::storage_max(), C::storage_min())
        {
            return st;
        }
        return tie_walker(GUARD);
    }
    let w = w0 + extra;
    let v_w = C::to_work_scaled(raw, GUARD + extra);
    let (sin_w, cos_w) = C::sin_cos_fixed::<SCALE>(v_w, w);
    let r = C::div(sin_w, cos_w, w);
    if let Some(st) = crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<
        C::Storage,
        C::W,
    >(r, w, SCALE, mode, C::storage_max(), C::storage_min())
    {
        return st;
    }
    tie_walker(GUARD + extra)
}
