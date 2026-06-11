// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Hyperbolic + inverse-hyperbolic schoolbook reference kernels.
//!
//! Naive textbook realisations of sinh / cosh / tanh / asinh / acosh /
//! atanh, registered as unrouted `Schoolbook` arms of the hyperbolic and
//! inverse-hyperbolic trig policies. Correctness reference + A/B
//! microbench partner; `select` never routes here. Each is the textbook
//! identity dispatched DOWN to the `Int<N>` work int. NEVER calls a
//! decimal `*_strict_with` on its own value. Identical composition +
//! narrowing as the routed kernel, so it matches bit-exactly.

use crate::algos::exp::exp_series_2limb::exp_fixed;
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, ln_fixed};
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::to_fixed;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Schoolbook sinh for a wide tier -- (e^|x| - e^-|x|)/2 (odd).
#[inline]
#[must_use]
pub(crate) fn sinh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let v = C::to_work_scaled(raw, guard);
        let av = if v < C::zero() { C::zero() - v } else { v };
        let sh = C::sinh_pos_wide::<SCALE>(av, w);
        if neg { C::zero() - sh } else { sh }
    })
}

/// Schoolbook cosh for a wide tier -- (e^|x| + e^-|x|)/2 (even).
#[inline]
#[must_use]
pub(crate) fn cosh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let v = C::to_work_scaled(raw, guard);
        let av = if v < C::zero() { C::zero() - v } else { v };
        C::cosh_pos_wide::<SCALE>(av, w)
    })
}

/// Schoolbook tanh for a wide tier -- (e^|x| - e^-|x|)/(e^|x| + e^-|x|).
#[inline]
#[must_use]
pub(crate) fn tanh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let v = C::to_work_scaled(raw, guard);
        let av = if v < C::zero() { C::zero() - v } else { v };
        let th = C::tanh_pos_wide::<SCALE>(av, w);
        if neg { C::zero() - th } else { th }
    })
}

/// Schoolbook asinh for a wide tier -- ln(x + sqrt(x^2 + 1)) (odd).
#[inline]
#[must_use]
pub(crate) fn asinh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    let neg = raw < C::storage_zero();
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let one_w = C::one(w);
        let v = C::to_work_scaled(raw, guard);
        let ax = if v < C::zero() { C::zero() - v } else { v };
        let inner = if ax >= one_w {
            let inv = C::div(one_w, ax, w);
            let root = C::sqrt_fixed(one_w + C::mul(inv, inv, w), w);
            C::ln_fixed::<SCALE>(ax, w) + C::ln_fixed::<SCALE>(one_w + root, w)
        } else {
            let root = C::sqrt_fixed(C::mul(ax, ax, w) + one_w, w);
            C::ln_fixed::<SCALE>(ax + root, w)
        };
        if neg { C::zero() - inner } else { inner }
    })
}

/// Schoolbook acosh for a wide tier -- ln(x + sqrt(x^2 - 1)), x >= 1.
#[inline]
#[must_use]
pub(crate) fn acosh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    {
        let w0 = SCALE + C::GUARD;
        if C::to_work(raw) < C::one(w0) {
            panic!("schoolbook acosh: argument must be >= 1");
        }
    }
    C::round_to_storage_directed_near_special(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let one_w = C::one(w);
        let v = C::to_work_scaled(raw, guard);
        let two_w = one_w + one_w;
        if v >= two_w {
            let inv = C::div(one_w, v, w);
            let root = C::sqrt_fixed(one_w - C::mul(inv, inv, w), w);
            C::ln_fixed::<SCALE>(v, w) + C::ln_fixed::<SCALE>(one_w + root, w)
        } else {
            let t = v - one_w;
            let root = C::sqrt_fixed(C::mul(t, t + two_w, w), w);
            C::log1p_fixed(t + root, w)
        }
    })
}

/// Schoolbook atanh for a wide tier -- (1/2) ln((1+x)/(1-x)), |x| < 1.
#[inline]
#[must_use]
pub(crate) fn atanh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    {
        let w0 = SCALE + C::GUARD;
        let v0 = C::to_work(raw);
        let ax0 = if v0 < C::zero() { C::zero() - v0 } else { v0 };
        if ax0 >= C::one(w0) {
            panic!("schoolbook atanh: argument out of domain (-1, 1)");
        }
    }
    C::round_to_storage_directed_near_special(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let one_w = C::one(w);
        let v = C::to_work_scaled(raw, guard);
        (C::ln_fixed::<SCALE>(one_w + v, w) - C::ln_fixed::<SCALE>(one_w - v, w)) >> 1
    })
}

// ── Rung-generic shells (the SCALE-derived work-rung surface) ─────────
//
// The exp-identity compositions run at an arbitrary work rung `Wk`
// (decoupled from `C::W`), mirroring `wide_trig_core::sin_series_g`.
// Each Ziv probe regime-splits exactly as the per-tier
// `sinh_pos_wide` does: the fast path runs the width-generic
// `exp_generic::{sinh,cosh,tanh}_pos` at the rung when the exp
// squaring-reassembly peak provably fits `Wk`
// (`exp_generic::exp_peak_fits` — the SAME model the tier's
// `hyper_fits_w` gate uses), else the probe lifts to the tier's wide
// `C::Wexp` and narrows the (always-rung-representable) probe VALUE
// back — so a deep escalation probe whose internal peak outgrows the
// rung still computes, value-identical to the tier path at the same
// working scale. The policy gate bounds `|x|` so the everyday region
// stays on the rung's fast path (see `policy::work_rung::EXP_ARG_BUDGET`).

/// Per-probe hyperbolic regime split at the rung — see the module note
/// above. `f_rung` / `f_wide` are the SAME identity at the two widths.
#[cfg(feature = "_wide-support")]
#[inline]
fn hyper_probe_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt>(
    raw: C::Storage,
    guard: u32,
    w: u32,
    f_rung: impl Fn(Wk, u32) -> Wk,
    f_wide: impl Fn(C::Wexp, u32) -> C::Wexp,
) -> Wk
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::to_work_scaled_g;
    let v = to_work_scaled_g::<C::Storage, Wk>(raw, guard);
    let av = if v < eg::zero::<Wk>() { eg::zero::<Wk>() - v } else { v };
    if eg::exp_peak_fits::<Wk>(av, w) {
        f_rung(av, w)
    } else {
        let v_e = to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard);
        let av_e = if v_e < eg::zero::<C::Wexp>() {
            eg::zero::<C::Wexp>() - v_e
        } else {
            v_e
        };
        eg::resize_or_panic::<C::Wexp, Wk>(f_wide(av_e, w))
    }
}

/// Rung-generic [`sinh_schoolbook`] — the `(e^x − e^-x)/2` identity at
/// an arbitrary work rung `Wk`.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn sinh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_g;
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    round_to_storage_directed_g::<C::Storage, Wk>(
        base_guard,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let sh = hyper_probe_g::<C, Wk>(
                raw,
                guard,
                w,
                |av, w| eg::sinh_pos::<Wk>(av, w),
                |av, w| eg::sinh_pos::<C::Wexp>(av, w),
            );
            if neg { eg::zero::<Wk>() - sh } else { sh }
        },
    )
}

/// Rung-generic [`cosh_schoolbook`] — see [`sinh_schoolbook_g`].
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn cosh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_g;
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    round_to_storage_directed_g::<C::Storage, Wk>(
        base_guard,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            hyper_probe_g::<C, Wk>(
                raw,
                guard,
                w,
                |av, w| eg::cosh_pos::<Wk>(av, w),
                |av, w| eg::cosh_pos::<C::Wexp>(av, w),
            )
        },
    )
}

/// Rung-generic [`tanh_schoolbook`] — see [`sinh_schoolbook_g`].
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn tanh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_g;
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    round_to_storage_directed_g::<C::Storage, Wk>(
        base_guard,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let th = hyper_probe_g::<C, Wk>(
                raw,
                guard,
                w,
                |av, w| eg::tanh_pos::<Wk>(av, w),
                |av, w| eg::tanh_pos::<C::Wexp>(av, w),
            );
            if neg { eg::zero::<Wk>() - th } else { th }
        },
    )
}

/// `ln 2` at working scale `w` in the rung integer `Wk`: const-table
/// keyed on the CONST base working scale on the hot path (`w ==
/// base_w`, const-folds per monomorphisation — the rung sibling of the
/// per-tier `ln2_cf`), the runtime-keyed lookup on the Ziv escalation
/// path. Value-identical either way (same table entry). Mirrors
/// `wide_trig_core::pi_at_rung` / `ln_series_g`'s ln2 threading.
#[cfg(feature = "_wide-support")]
#[inline]
fn ln2_at_rung<Wk: crate::int::types::traits::BigInt>(w: u32, base_w: u32) -> Wk {
    if w == base_w {
        crate::consts::ln2_by_scale::<Wk>(base_w, crate::support::rounding::DEFAULT_ROUNDING_MODE)
    } else {
        crate::consts::ln2_by_working_scale::<Wk>(
            w,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    }
}

/// Rung-generic [`asinh_schoolbook`] — `ln(x + √(x² + 1))` at an
/// arbitrary work rung `Wk` (the ln is the width-generic
/// `exp_generic::ln_fixed`, value-identical to the tier's `C::ln_fixed`
/// — same kernel, `ln2` from the same per-scale table).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn asinh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{round_to_storage_directed_g, to_work_scaled_g};
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    let neg = raw < C::storage_zero();
    round_to_storage_directed_g::<C::Storage, Wk>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let ln2_w = ln2_at_rung::<Wk>(w, SCALE + C::GUARD);
            let one_w = eg::one::<Wk>(w);
            let v = to_work_scaled_g::<C::Storage, Wk>(raw, guard);
            let ax = if v < eg::zero::<Wk>() { eg::zero::<Wk>() - v } else { v };
            let inner = if ax >= one_w {
                let inv = eg::div::<Wk>(one_w, ax, w);
                let root = eg::sqrt_fixed::<Wk>(one_w + eg::mul::<Wk>(inv, inv, w), w);
                eg::ln_fixed::<Wk>(ax, w, ln2_w) + eg::ln_fixed::<Wk>(one_w + root, w, ln2_w)
            } else {
                let root = eg::sqrt_fixed::<Wk>(eg::mul::<Wk>(ax, ax, w) + one_w, w);
                eg::ln_fixed::<Wk>(ax + root, w, ln2_w)
            };
            if neg { eg::zero::<Wk>() - inner } else { inner }
        },
    )
}

/// Rung-generic [`acosh_schoolbook`] — `ln(x + √(x² − 1))`, `x ≥ 1`, at
/// an arbitrary work rung `Wk` (the near-special walker; the policy's
/// rung selector keys on `2·SCALE` so the forced confirm probe stays
/// reachable — see `policy::work_rung::near_special_rung`).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn acosh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_near_special_g, to_work_scaled_g,
    };
    {
        let w0 = SCALE + C::GUARD;
        if to_work_scaled_g::<C::Storage, Wk>(raw, C::GUARD) < eg::one::<Wk>(w0) {
            panic!("schoolbook acosh: argument must be >= 1");
        }
    }
    round_to_storage_directed_near_special_g::<C::Storage, Wk>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let ln2_w = ln2_at_rung::<Wk>(w, SCALE + C::GUARD);
            let one_w = eg::one::<Wk>(w);
            let v = to_work_scaled_g::<C::Storage, Wk>(raw, guard);
            let two_w = one_w + one_w;
            if v >= two_w {
                let inv = eg::div::<Wk>(one_w, v, w);
                let root = eg::sqrt_fixed::<Wk>(one_w - eg::mul::<Wk>(inv, inv, w), w);
                eg::ln_fixed::<Wk>(v, w, ln2_w) + eg::ln_fixed::<Wk>(one_w + root, w, ln2_w)
            } else {
                let t = v - one_w;
                let root = eg::sqrt_fixed::<Wk>(eg::mul::<Wk>(t, t + two_w, w), w);
                eg::log1p_fixed::<Wk>(t + root, w)
            }
        },
    )
}

/// Rung-generic [`atanh_schoolbook`] — `½·ln((1+x)/(1−x))`, `|x| < 1`,
/// at an arbitrary work rung `Wk` (the near-special walker — see
/// [`acosh_schoolbook_g`]). The two logs stay SEPARATE exactly as the
/// tier kernel computes them (the near-±1 ratio overflow analysis in
/// the narrow sibling applies at any width).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn atanh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_near_special_g, to_work_scaled_g,
    };
    {
        let w0 = SCALE + C::GUARD;
        let v0 = to_work_scaled_g::<C::Storage, Wk>(raw, C::GUARD);
        let ax0 = if v0 < eg::zero::<Wk>() { eg::zero::<Wk>() - v0 } else { v0 };
        if ax0 >= eg::one::<Wk>(w0) {
            panic!("schoolbook atanh: argument out of domain (-1, 1)");
        }
    }
    round_to_storage_directed_near_special_g::<C::Storage, Wk>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let ln2_w = ln2_at_rung::<Wk>(w, SCALE + C::GUARD);
            let one_w = eg::one::<Wk>(w);
            let v = to_work_scaled_g::<C::Storage, Wk>(raw, guard);
            (eg::ln_fixed::<Wk>(one_w + v, w, ln2_w) - eg::ln_fixed::<Wk>(one_w - v, w, ln2_w))
                >> 1
        },
    )
}

// -- Narrow tier -- Int<2> storage, math in the 256-bit Fixed ---------

#[inline]
fn one_fixed(w: u32) -> Fixed {
    Fixed { negative: false, mag: Fixed::pow10(w) }
}

#[inline]
#[must_use]
fn sinh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let v = to_fixed(raw);
    let neg = raw < 0;
    let av = Fixed { negative: false, mag: v.mag };
    let ex = exp_fixed(av, w);
    let one_w = one_fixed(w);
    let enx = one_w.div(ex, w);
    let sh = ex.sub(enx).halve();
    let sh = if neg { sh.neg() } else { sh };
    sh.round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("sinh", SCALE))
}

#[inline]
#[must_use]
fn cosh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    let v = to_fixed(raw);
    let av = Fixed { negative: false, mag: v.mag };
    let ex = exp_fixed(av, w);
    let one_w = one_fixed(w);
    let enx = one_w.div(ex, w);
    ex.add(enx)
        .halve()
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("cosh", SCALE))
}

#[inline]
#[must_use]
fn tanh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let neg = raw < 0;
    // Large |x| via the NEGATIVE-exponent identity tanh(|x|) = (1 − m)/(1 + m),
    // m = e^(−2|x|) = (e^|x| − e^−|x|)/(e^|x| + e^−|x|) — exact. Forming e^(+|x|)
    // directly overflows the 256-bit `Fixed` once |x| ≳ (256·ln2 − w·ln10) (≈ 44
    // at w = 58), BELOW the all-nines saturation onset |x| ≳ 1.1513·w (`thr_x`),
    // leaving a panic GAP. The identity sidesteps it: m is TINY for large |x|,
    // formed by `exp_fixed` on the NEGATIVE argument −2|x| whose `2^k` reassembly
    // shifts DOWN, never the overflowing up-shift. Mirrors the routed
    // `tanh_with_raw` / wide `exp_generic::tanh_pos`, bit-for-bit.
    let thr_x = (w as i128) * 1152 / 1000 + 2;
    // Largest working value below 1 (value 1 − 10^−w): the all-nines saturation.
    let saturated = one_w.sub(Fixed::from_u128_mag(1, false));
    let th = if raw.abs() / 10_i128.pow(SCALE) > thr_x {
        saturated
    } else {
        let v = to_fixed(raw);
        let av = Fixed { negative: false, mag: v.mag };
        let m = exp_fixed(av.double().neg(), w);
        if m.is_zero() {
            // |x| just under `thr_x`: m underflowed; tanh is all-nines too.
            saturated
        } else {
            one_w.sub(m).div(one_w.add(m), w)
        }
    };
    let th = if neg { th.neg() } else { th };
    th.round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("tanh", SCALE))
}

#[inline]
#[must_use]
fn asinh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let v = to_fixed(raw);
    let ax = Fixed { negative: false, mag: v.mag };
    let inner = if ax.ge_mag(one_w) {
        let inv = one_w.div(ax, w);
        let root = one_w.add(inv.mul(inv, w)).sqrt(w);
        ln_fixed(ax, w).add(ln_fixed(one_w.add(root), w))
    } else {
        let root = ax.mul(ax, w).add(one_w).sqrt(w);
        ln_fixed(ax.add(root), w)
    };
    let signed = if raw < 0 { inner.neg() } else { inner };
    signed
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("asinh", SCALE))
}

#[inline]
#[must_use]
fn acosh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let v = to_fixed(raw);
    assert!(!v.negative && v.ge_mag(one_w), "acosh: argument must be >= 1");
    let two_w = one_w.double();
    let inner = if v.ge_mag(two_w) {
        let inv = one_w.div(v, w);
        let root = one_w.sub(inv.mul(inv, w)).sqrt(w);
        ln_fixed(v, w).add(ln_fixed(one_w.add(root), w))
    } else {
        let root = v.mul(v, w).sub(one_w).sqrt(w);
        ln_fixed(v.add(root), w)
    };
    inner
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("acosh", SCALE))
}

#[inline]
#[must_use]
fn atanh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let v = to_fixed(raw);
    let ax = Fixed { negative: false, mag: v.mag };
    assert!(!ax.ge_mag(one_w), "atanh: argument out of domain (-1, 1)");
    // atanh(x) = ½·ln((1+x)/(1-x)) = ½·(ln(1+x) − ln(1-x)). Computing the two
    // logs SEPARATELY (not ln of the ratio) is essential near |x| = 1: the
    // ratio (1+x)/(1-x) reaches ~10^(2·digits) there and, scaled to the
    // working scale `w`, overflows the 256-bit `Fixed` — e.g. atanh(1−10⁻²⁸)
    // at D38 s28 has ratio ≈ 2·10²⁸ which at w = SCALE+30 = 58 is a raw ~10⁸⁶,
    // far past 2²⁵⁶. `1+x` and `1-x` each fit, and `ln_fixed` handles arguments
    // below 1 (the x-near-−1 case already relies on it).
    let ln_num = ln_fixed(one_w.add(v), w);
    let ln_den = ln_fixed(one_w.sub(v), w);
    ln_num
        .sub(ln_den)
        .halve()
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atanh", SCALE))
}

/// Narrow schoolbook sinh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn sinh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(sinh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook cosh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn cosh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cosh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook tanh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn tanh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(tanh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook asinh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn asinh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(asinh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook acosh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn acosh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(acosh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook atanh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn atanh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(atanh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

// -- Unit tests: each schoolbook is bit-exact against the routed kernel.
//
// Reference correctness (skill 7): the schoolbook MUST produce the SAME
// storage raw as the routed kernel at every input, scale, tier and mode
// (delta == 0). A mismatch is a hard failure, never weakened.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::D;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    const S38: u32 = 12;
    fn d38(raw: i128) -> D<Int<2>, S38> {
        D(Int::<2>::from_i128(raw))
    }
    const HYP_INPUTS: [i128; 9] = [
        0,
        1_000_000_000,
        500_000_000_000,
        1_000_000_000_000,
        2_500_000_000_000,
        -1_000_000_000,
        -500_000_000_000,
        -1_000_000_000_000,
        -2_500_000_000_000,
    ];

    #[test]
    fn sinh_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &HYP_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    sinh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).sinh_strict_with(mode).0,
                    "sinh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn cosh_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &HYP_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    cosh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).cosh_strict_with(mode).0,
                    "cosh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn tanh_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &HYP_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    tanh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).tanh_strict_with(mode).0,
                    "tanh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    // Large |x| (well past the saturation onset |x| ≳ 1.1513·w): tanh is
    // BOUNDED in (−1, 1), so these must SATURATE to ±1 (or ±(1−ulp) under the
    // directed modes), never panic by forming e^|x|. Schoolbook and routed
    // must still agree bit-for-bit. Dedicated array (not the shared
    // HYP_INPUTS) because sinh/cosh genuinely overflow the tier at this scale.
    #[test]
    fn tanh_schoolbook_narrow_saturates_large_x_matches_routed() {
        const LARGE: [i128; 4] = [
            100_000_000_000_000,    // x = 100
            5_000_000_000_000_000,  // x = 5000
            -100_000_000_000_000,   // x = -100
            -5_000_000_000_000_000, // x = -5000
        ];
        for &raw in &LARGE {
            for &mode in &MODES {
                assert_eq!(
                    tanh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).tanh_strict_with(mode).0,
                    "tanh saturation schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    // Large |x| at a HIGH scale (28) — the band the old `(e^|x| ± e^-|x|)` form
    // could not reach. At w = SCALE + 30 = 58 the dominant e^(+|x|) overflows the
    // 256-bit `Fixed` once |x| ≳ 44, yet the saturation onset sits at |x| ≳ 1.1513·w
    // ≈ 67, so |x| in [44, 67] used to PANIC (the 18 D38<28> defect cells). The
    // negative-exponent form tanh = (1 − e^-2|x|)/(1 + e^-2|x|) never forms e^(+|x|),
    // so these compute (no panic) AND still agree with the routed kernel bit-for-bit.
    #[test]
    fn tanh_schoolbook_narrow_gap_band_high_scale_matches_routed() {
        const S: u32 = 28;
        const UNIT: i128 = 10_i128.pow(28);
        // |x| spanning the overflow gap [44, 66] plus the saturation region above.
        const XS: [i128; 9] = [44, 48, 50, 55, 60, 66, 67, 70, 100];
        for &x in &XS {
            for &raw in &[x * UNIT, -x * UNIT] {
                for &mode in &MODES {
                    assert_eq!(
                        tanh_schoolbook_narrow::<S>(Int::<2>::from_i128(raw), mode),
                        D::<Int<2>, S>(Int::<2>::from_i128(raw)).tanh_strict_with(mode).0,
                        "tanh gap-band schoolbook != routed at raw={raw} mode={mode:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn asinh_schoolbook_narrow_matches_routed_kernel() {
        const PTS: [i128; 9] = [
            0,
            500_000_000_000,
            1_000_000_000_000,
            2_500_000_000_000,
            5_000_000_000_000,
            -500_000_000_000,
            -1_000_000_000_000,
            -2_500_000_000_000,
            -5_000_000_000_000,
        ];
        for &raw in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    asinh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).asinh_strict_with(mode).0,
                    "asinh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn acosh_schoolbook_narrow_matches_routed_kernel() {
        const PTS: [i128; 5] = [
            1_000_000_000_000,
            1_200_000_000_000,
            2_000_000_000_000,
            3_000_000_000_000,
            5_000_000_000_000,
        ];
        for &raw in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    acosh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).acosh_strict_with(mode).0,
                    "acosh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn atanh_schoolbook_narrow_matches_routed_kernel() {
        const PTS: [i128; 7] = [
            0,
            250_000_000_000,
            500_000_000_000,
            900_000_000_000,
            -250_000_000_000,
            -500_000_000_000,
            -900_000_000_000,
        ];
        for &raw in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    atanh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).atanh_strict_with(mode).0,
                    "atanh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    // atanh near |x| = 1 at a HIGH scale (28) — the band where the old ratio
    // form `ln((1+x)/(1-x))` overflowed the 256-bit `Fixed`. The ratio reaches
    // ~2·10^SCALE there, and the working-scale divide widens it by 10^(SCALE+30),
    // so it exceeds U256 once SCALE >= ~24 (hence the scale-12 cases above could
    // never catch this). The two-log form keeps both operands in (0, 2); it must
    // agree with the routed kernel here. This is exactly the cell that
    // `golden atanh_d38_s28` exposes.
    #[test]
    fn atanh_schoolbook_narrow_matches_routed_kernel_near_one() {
        const S: u32 = 28;
        const ONE: i128 = 10_i128.pow(28);
        // x = 1 − 10^{-k} for shrinking 10^{-k} (raw = ONE − 10^{28-k}), both signs.
        const NEAR_ONE: [i128; 8] = [
            ONE - 1,
            ONE - 10,
            ONE - 100,
            ONE - 10_000,
            ONE - 1_000_000,
            ONE - 100_000_000,
            ONE - 10_000_000_000,
            ONE / 2,
        ];
        for &mag in &NEAR_ONE {
            for &raw in &[mag, -mag] {
                for &mode in &MODES {
                    assert_eq!(
                        atanh_schoolbook_narrow::<S>(Int::<2>::from_i128(raw), mode),
                        D::<Int<2>, S>(Int::<2>::from_i128(raw)).atanh_strict_with(mode).0,
                        "atanh near-1 schoolbook != routed at raw={raw} mode={mode:?}"
                    );
                }
            }
        }
    }

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }

        #[test]
        fn forward_hyper_schoolbook_match_routed() {
            const INPUTS9: [i128; 7] = [
                0,
                1_000_000,
                500_000_000,
                1_000_000_000,
                2_500_000_000,
                -1_000_000_000,
                -2_500_000_000,
            ];
            for &u in &INPUTS9 {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        sinh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).sinh_strict_with(mode).0,
                        "D57 sinh schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        cosh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).cosh_strict_with(mode).0,
                        "D57 cosh schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        tanh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).tanh_strict_with(mode).0,
                        "D57 tanh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }

        #[test]
        fn inverse_hyper_schoolbook_match_routed() {
            const SINPUTS: [i128; 7] = [
                0,
                500_000_000,
                1_000_000_000,
                2_500_000_000,
                -500_000_000,
                -1_000_000_000,
                -2_500_000_000,
            ];
            for &u in &SINPUTS {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        asinh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).asinh_strict_with(mode).0,
                        "D57 asinh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
            const TINPUTS: [i128; 5] = [
                0,
                250_000_000,
                500_000_000,
                900_000_000,
                -500_000_000,
            ];
            for &u in &TINPUTS {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        atanh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).atanh_strict_with(mode).0,
                        "D57 atanh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
            const AINPUTS: [i128; 4] = [
                1_000_000_000,
                1_200_000_000,
                2_000_000_000,
                3_000_000_000,
            ];
            for &u in &AINPUTS {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        acosh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).acosh_strict_with(mode).0,
                        "D57 acosh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }

}
