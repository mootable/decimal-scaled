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
#[cfg(feature = "_wide-support")]
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::to_fixed;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── The canonical wide kernels (the `_strict_with` shells' semantics,
// hoisted) ────────────────────────────────────────────────────────────
//
// These kernels are the ONE realisation behind BOTH public entries
// (`sinh_strict` via the policy dispatch AND `sinh_strict_with`, which
// now delegates to the same dispatch — the rounding-mode-sibling
// convention). They carry everything the per-tier macro shells used to
// hold shell-side only: the exact-point pins, the analytic
// small-argument odd-cubic bands, tanh's all-nines saturation fast path
// and its k-lift cap, the `never_exact` two-width widening for
// sinh/cosh, the `Wagm` composition width, and the Tang-routed
// working-scale ln for acosh/atanh (Series for asinh — the MAX-scale
// tang pre-residue caveat). The default-mode policy path previously
// lacked the pins/bands, so a deep-cubic directed cell (the
// tanh(1e-168) D462<461> Trunc pin) diverged between the two entries.

/// Analytic small-argument odd-cubic pin — the macro shells' band,
/// hoisted (and EXTENDED to asinh/atanh, which had it on NEITHER path).
///
/// For an odd function `f(x) = x ± x³·c + …` (c = 1/6 or 1/3) and
/// `0 < |raw| ≤ 10^(SCALE − ⌈SCALE/3⌉)`, the cubic correction is below
/// one storage ULP (≤ ~1/3 ULP at the band edge, tail included — well
/// under half a ULP for the nearest modes) yet STRICTLY signed, so the
/// true value sits strictly inside `(raw, raw+1)` (EXPANDING — sinh,
/// atanh: `|f(x)| > |x|`) or `(raw−1, raw)` (compressing — tanh, asinh)
/// in magnitude. No finite-precision kernel can resolve the sub-ULP
/// cubic — the deciding digit sits at fraction depth ~3·|log₁₀ x|,
/// beyond every escalation cap for deep-band inputs — so the rounding
/// is exact integer arithmetic on `raw` for every mode.
#[cfg(feature = "_wide-support")]
#[inline]
fn hyper_tiny_pin<C: WideTrigCore, const SCALE: u32, const EXPANDING: bool>(
    raw: C::Storage,
    mode: RoundingMode,
) -> Option<C::Storage> {
    let zero = C::storage_zero();
    if raw == zero {
        return None;
    }
    let thresh_exp = SCALE - SCALE.div_ceil(3);
    let thresh = crate::consts::pow10::dispatch::<C::Storage>(thresh_exp);
    let a = if raw < zero { zero - raw } else { raw };
    if a > thresh {
        return None;
    }
    let one = <C::Storage as crate::int::types::traits::BigInt>::from_i128(1);
    Some(if EXPANDING {
        crate::support::rounding::tiny_odd_expanding_directed(raw, zero, one, mode)
    } else {
        crate::support::rounding::tiny_odd_compressing_directed(raw, zero, one, mode)
    })
}

/// tanh's capped exp lift (the shell's, hoisted): the integer-digit
/// estimator is a power-of-two UPPER bound on `|x|`; outside saturation
/// `0.86859·|x| ≤ SCALE + GUARD + 3`, so cap the lift at
/// `(SCALE + GUARD)/2 + 2` — an over-lift would push the `e^(−2|x|)`
/// evaluation past the work integer's internal headroom.
#[cfg(feature = "_wide-support")]
#[inline]
fn tanh_k_lift<C: WideTrigCore, const SCALE: u32>(raw: C::Storage) -> u32 {
    C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE)
        .min((SCALE + C::GUARD) / 2 + 2)
}

/// tanh's all-nines saturation fast path (the shell's, hoisted): once
/// the `1 − tanh(|x|) = 2·e^(−2|x|)·(1 − …)` deficit's leading digit
/// (fractional position `~0.86859·|x|`) passes `SCALE + GUARD`, every
/// digit the narrowing keeps is a `9` — return the all-nines working
/// value directly (its sub-resolution deficit rounds each mode
/// correctly). Integer compare: `|x| > (SCALE + GUARD + 2)/0.86859`.
#[cfg(feature = "_wide-support")]
#[inline]
fn tanh_saturated<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> Option<C::Storage>
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    let zero = C::storage_zero();
    let neg = raw < zero;
    let a = if neg { zero - raw } else { raw };
    let sat_x = ((SCALE as u128 + C::GUARD as u128 + 2) * 100_000 / 86_859) as i128;
    let over = a / crate::consts::pow10::dispatch::<C::Storage>(SCALE)
        > <C::Storage as crate::int::types::traits::BigInt>::from_i128(sat_x);
    if !over {
        return None;
    }
    Some(
        crate::algos::support::wide_trig_core::round_to_storage_directed_g::<C::Storage, C::Wagm>(
            C::GUARD,
            SCALE,
            mode,
            C::storage_max(),
            C::storage_min(),
            |guard| {
                let w = SCALE + guard;
                let sat = eg::one::<C::Wagm>(w)
                    - <C::Wagm as crate::int::types::traits::BigInt>::ONE;
                if neg { eg::zero::<C::Wagm>() - sat } else { sat }
            },
        ),
    )
}

/// Schoolbook sinh for a wide tier -- (e^|x| - e^-|x|)/2 (odd). The
/// canonical kernel behind both public entries (shell semantics: zero
/// pin, the expanding cubic band, `never_exact` two-width widening on
/// the `Wagm` composition width with the `Wexp` near-min retry).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn sinh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_widening_g, to_work_scaled_g,
    };
    // sinh(0) = 0 is the SOLE exact point.
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    if let Some(p) = hyper_tiny_pin::<C, SCALE, true>(raw, mode) {
        return p;
    }
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    // sinh(x) is irrational for rational x != 0 (never on a grid line):
    // never_exact = true, with the Wexp retry covering near-min deciding
    // terms past Wagm's reach.
    round_to_storage_widening_g::<C::Storage, C::Wagm, C::Wexp>(
        base_guard,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let sh = hyper_probe_g::<C, C::Wagm>(
                raw,
                guard,
                w,
                eg::sinh_pos::<C::Wagm>,
                eg::sinh_pos::<C::Wexp>,
            );
            if neg { eg::zero::<C::Wagm>() - sh } else { sh }
        },
        |guard| {
            let w = SCALE + guard;
            let v = to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard);
            let av = if v < eg::zero::<C::Wexp>() { eg::zero::<C::Wexp>() - v } else { v };
            let sh = eg::sinh_pos::<C::Wexp>(av, w);
            if neg { eg::zero::<C::Wexp>() - sh } else { sh }
        },
    )
}

/// Schoolbook cosh for a wide tier -- (e^|x| + e^-|x|)/2 (even). Shell
/// semantics: the cosh(0) = 1 exact pin and `never_exact` two-width
/// widening (cosh(x) > 1 strictly and transcendental for x != 0).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn cosh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_widening_g, to_work_scaled_g,
    };
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    round_to_storage_widening_g::<C::Storage, C::Wagm, C::Wexp>(
        base_guard,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            hyper_probe_g::<C, C::Wagm>(
                raw,
                guard,
                w,
                eg::cosh_pos::<C::Wagm>,
                eg::cosh_pos::<C::Wexp>,
            )
        },
        |guard| {
            let w = SCALE + guard;
            let v = to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard);
            let av = if v < eg::zero::<C::Wexp>() { eg::zero::<C::Wexp>() - v } else { v };
            eg::cosh_pos::<C::Wexp>(av, w)
        },
    )
}

/// Schoolbook tanh for a wide tier -- (e^|x| - e^-|x|)/(e^|x| + e^-|x|).
/// Shell semantics: the compressing cubic band, the all-nines
/// saturation fast path, the capped exp lift, directed walker on `Wagm`.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn tanh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_g;
    if let Some(p) = hyper_tiny_pin::<C, SCALE, false>(raw, mode) {
        return p;
    }
    if let Some(p) = tanh_saturated::<C, SCALE>(raw, mode) {
        return p;
    }
    let neg = raw < C::storage_zero();
    let base_guard = C::GUARD + tanh_k_lift::<C, SCALE>(raw);
    round_to_storage_directed_g::<C::Storage, C::Wagm>(
        base_guard,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let th = hyper_probe_g::<C, C::Wagm>(
                raw,
                guard,
                w,
                // The shell's fits-branch form, exactly: the DIRECT ratio
                // (the k-lifted base guard covers the e^|x| amplification;
                // the m = e^(-2|x|) identity loses the deficit's deciding
                // digits in this regime).
                |av, w| {
                    let ex = eg::exp_fixed::<C::Wagm>(av, w);
                    let enx = eg::div::<C::Wagm>(eg::one::<C::Wagm>(w), ex, w);
                    eg::div::<C::Wagm>(ex - enx, ex + enx, w)
                },
                eg::tanh_pos::<C::Wexp>,
            );
            if neg { eg::zero::<C::Wagm>() - th } else { th }
        },
    )
}

/// Schoolbook asinh for a wide tier -- ln(x + sqrt(x^2 + 1)) (odd).
/// Shell semantics plus the (new) compressing cubic band; the ln stays
/// SERIES (`eg::ln_fixed`) — the MAX-scale tang pre-residue caveat
/// (memory project_050_asinh_max_tang_residue) keeps Tang off this
/// composition until ln_fixed_routed gains a PRE_RESIDUE flag.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn asinh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_g, to_work_scaled_g,
    };
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    // asinh(x) = x − x³/6 + … : compressing.
    if let Some(p) = hyper_tiny_pin::<C, SCALE, false>(raw, mode) {
        return p;
    }
    let neg = raw < C::storage_zero();
    round_to_storage_directed_g::<C::Storage, C::Wagm>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let ln2_w = ln2_at_rung::<C::Wagm>(w, SCALE + C::GUARD);
            let one_w = eg::one::<C::Wagm>(w);
            let v = to_work_scaled_g::<C::Storage, C::Wagm>(raw, guard);
            let ax = if v < eg::zero::<C::Wagm>() { eg::zero::<C::Wagm>() - v } else { v };
            let inner = if ax >= one_w {
                let inv = eg::div::<C::Wagm>(one_w, ax, w);
                let root = eg::sqrt_fixed::<C::Wagm>(one_w + eg::mul::<C::Wagm>(inv, inv, w), w);
                eg::ln_fixed::<C::Wagm>(ax, w, ln2_w) + eg::ln_fixed::<C::Wagm>(one_w + root, w, ln2_w)
            } else {
                let root = eg::sqrt_fixed::<C::Wagm>(eg::mul::<C::Wagm>(ax, ax, w) + one_w, w);
                eg::ln_fixed::<C::Wagm>(ax + root, w, ln2_w)
            };
            if neg { eg::zero::<C::Wagm>() - inner } else { inner }
        },
    )
}

/// Schoolbook acosh for a wide tier -- ln(x + sqrt(x^2 - 1)), x >= 1.
/// Shell semantics: near-special walker on `Wagm` with the Tang-ROUTED
/// working-scale ln (`C::ln_fixed_routed_agm`) and the near-1 log1p
/// form.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn acosh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_near_special_g, to_work_scaled_g,
    };
    {
        let w0 = SCALE + C::GUARD;
        if to_work_scaled_g::<C::Storage, C::Wagm>(raw, C::GUARD) < eg::one::<C::Wagm>(w0) {
            panic!("schoolbook acosh: argument must be >= 1");
        }
    }
    round_to_storage_directed_near_special_g::<C::Storage, C::Wagm>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let one_w = eg::one::<C::Wagm>(w);
            let v = to_work_scaled_g::<C::Storage, C::Wagm>(raw, guard);
            let two_w = one_w + one_w;
            if v >= two_w {
                let inv = eg::div::<C::Wagm>(one_w, v, w);
                let root = eg::sqrt_fixed::<C::Wagm>(one_w - eg::mul::<C::Wagm>(inv, inv, w), w);
                C::ln_fixed_routed_agm::<SCALE>(v, w) + C::ln_fixed_routed_agm::<SCALE>(one_w + root, w)
            } else {
                let t = v - one_w;
                let root = eg::sqrt_fixed::<C::Wagm>(eg::mul::<C::Wagm>(t, t + two_w, w), w);
                eg::log1p_fixed::<C::Wagm>(t + root, w)
            }
        },
    )
}

/// Schoolbook atanh for a wide tier -- (1/2) ln((1+x)/(1-x)), |x| < 1.
/// Shell semantics plus the (new) expanding cubic band; near-special
/// walker on `Wagm` with the Tang-routed ln on the exact gap form.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn atanh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_near_special_g, to_work_scaled_g,
    };
    {
        let w0 = SCALE + C::GUARD;
        let v0 = to_work_scaled_g::<C::Storage, C::Wagm>(raw, C::GUARD);
        let ax0 = if v0 < eg::zero::<C::Wagm>() { eg::zero::<C::Wagm>() - v0 } else { v0 };
        if ax0 >= eg::one::<C::Wagm>(w0) {
            panic!("schoolbook atanh: argument out of domain (-1, 1)");
        }
    }
    // atanh(x) = x + x³/3 + … : expanding.
    if let Some(p) = hyper_tiny_pin::<C, SCALE, true>(raw, mode) {
        return p;
    }
    round_to_storage_directed_near_special_g::<C::Storage, C::Wagm>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let one_w = eg::one::<C::Wagm>(w);
            let v = to_work_scaled_g::<C::Storage, C::Wagm>(raw, guard);
            (C::ln_fixed_routed_agm::<SCALE>(one_w + v, w)
                - C::ln_fixed_routed_agm::<SCALE>(one_w - v, w))
                >> 1
        },
    )
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
    use crate::algos::support::wide_trig_core::{
        round_to_storage_widening_g, to_work_scaled_g,
    };
    // The canonical pins — identical to the tier kernel.
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    if let Some(p) = hyper_tiny_pin::<C, SCALE, true>(raw, mode) {
        return p;
    }
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    // never_exact two-width widening, rung-first: a near-tie unresolved
    // at the rung's cap retries at the SAME `Wexp` the tier kernel
    // retries at — never weaker than the tier conclusion.
    round_to_storage_widening_g::<C::Storage, Wk, C::Wexp>(
        base_guard,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let sh = hyper_probe_g::<C, Wk>(
                raw,
                guard,
                w,
                |av, w| eg::sinh_pos::<Wk>(av, w),
                eg::sinh_pos::<C::Wexp>,
            );
            if neg { eg::zero::<Wk>() - sh } else { sh }
        },
        |guard| {
            let w = SCALE + guard;
            let v = to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard);
            let av = if v < eg::zero::<C::Wexp>() { eg::zero::<C::Wexp>() - v } else { v };
            let sh = eg::sinh_pos::<C::Wexp>(av, w);
            if neg { eg::zero::<C::Wexp>() - sh } else { sh }
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
    use crate::algos::support::wide_trig_core::{
        round_to_storage_widening_g, to_work_scaled_g,
    };
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    // never_exact two-width widening, rung-first - see [`sinh_schoolbook_g`].
    round_to_storage_widening_g::<C::Storage, Wk, C::Wexp>(
        base_guard,
        SCALE,
        mode,
        true,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            hyper_probe_g::<C, Wk>(
                raw,
                guard,
                w,
                |av, w| eg::cosh_pos::<Wk>(av, w),
                eg::cosh_pos::<C::Wexp>,
            )
        },
        |guard| {
            let w = SCALE + guard;
            let v = to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard);
            let av = if v < eg::zero::<C::Wexp>() { eg::zero::<C::Wexp>() - v } else { v };
            eg::cosh_pos::<C::Wexp>(av, w)
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
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_widening_g;
    // The canonical pins — identical to the tier kernel. The saturation
    // fast path is unreachable at the rung (the policy gate admits
    // |x| < 10, far below the onset) but kept for kernel-level callers.
    if let Some(p) = hyper_tiny_pin::<C, SCALE, false>(raw, mode) {
        return p;
    }
    if let Some(p) = tanh_saturated::<C, SCALE>(raw, mode) {
        return p;
    }
    let neg = raw < C::storage_zero();
    let base_guard = C::GUARD + tanh_k_lift::<C, SCALE>(raw);
    // Two-width fall-up to the tier walker width `Wagm`.
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::Wagm>(
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
                // Direct ratio — see the tier kernel.
                |av, w| {
                    let ex = eg::exp_fixed::<Wk>(av, w);
                    let enx = eg::div::<Wk>(eg::one::<Wk>(w), ex, w);
                    eg::div::<Wk>(ex - enx, ex + enx, w)
                },
                eg::tanh_pos::<C::Wexp>,
            );
            if neg { eg::zero::<Wk>() - th } else { th }
        },
        |guard| {
            let w = SCALE + guard;
            let th = hyper_probe_g::<C, C::Wagm>(
                raw,
                guard,
                w,
                // The shell's fits-branch form, exactly: the DIRECT ratio
                // (the k-lifted base guard covers the e^|x| amplification;
                // the m = e^(-2|x|) identity loses the deficit's deciding
                // digits in this regime).
                |av, w| {
                    let ex = eg::exp_fixed::<C::Wagm>(av, w);
                    let enx = eg::div::<C::Wagm>(eg::one::<C::Wagm>(w), ex, w);
                    eg::div::<C::Wagm>(ex - enx, ex + enx, w)
                },
                eg::tanh_pos::<C::Wexp>,
            );
            if neg { eg::zero::<C::Wagm>() - th } else { th }
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
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_widening_g, to_work_scaled_g,
    };
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    // The canonical pins — identical to the tier kernel (compressing:
    // asinh(x) = x − x³/6 + …).
    if let Some(p) = hyper_tiny_pin::<C, SCALE, false>(raw, mode) {
        return p;
    }
    let neg = raw < C::storage_zero();
    // Two-width fall-up to the tier walker width `Wagm` - the second
    // closure is the tier kernel's, verbatim.
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::Wagm>(
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
        |guard| {
            let w = SCALE + guard;
            let ln2_w = ln2_at_rung::<C::Wagm>(w, SCALE + C::GUARD);
            let one_w = eg::one::<C::Wagm>(w);
            let v = to_work_scaled_g::<C::Storage, C::Wagm>(raw, guard);
            let ax = if v < eg::zero::<C::Wagm>() { eg::zero::<C::Wagm>() - v } else { v };
            let inner = if ax >= one_w {
                let inv = eg::div::<C::Wagm>(one_w, ax, w);
                let root = eg::sqrt_fixed::<C::Wagm>(one_w + eg::mul::<C::Wagm>(inv, inv, w), w);
                eg::ln_fixed::<C::Wagm>(ax, w, ln2_w) + eg::ln_fixed::<C::Wagm>(one_w + root, w, ln2_w)
            } else {
                let root = eg::sqrt_fixed::<C::Wagm>(eg::mul::<C::Wagm>(ax, ax, w) + one_w, w);
                eg::ln_fixed::<C::Wagm>(ax + root, w, ln2_w)
            };
            if neg { eg::zero::<C::Wagm>() - inner } else { inner }
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
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_near_special_widening_g, to_work_scaled_g,
    };
    {
        let w0 = SCALE + C::GUARD;
        if to_work_scaled_g::<C::Storage, Wk>(raw, C::GUARD) < eg::one::<Wk>(w0) {
            panic!("schoolbook acosh: argument must be >= 1");
        }
    }
    // Two-width fall-up (near-special form) to the tier walker width
    // `Wagm` - the second closure is the tier kernel's, verbatim.
    round_to_storage_directed_near_special_widening_g::<C::Storage, Wk, C::Wagm>(
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
        |guard| {
            let w = SCALE + guard;
            let one_w = eg::one::<C::Wagm>(w);
            let v = to_work_scaled_g::<C::Storage, C::Wagm>(raw, guard);
            let two_w = one_w + one_w;
            if v >= two_w {
                let inv = eg::div::<C::Wagm>(one_w, v, w);
                let root = eg::sqrt_fixed::<C::Wagm>(one_w - eg::mul::<C::Wagm>(inv, inv, w), w);
                C::ln_fixed_routed_agm::<SCALE>(v, w) + C::ln_fixed_routed_agm::<SCALE>(one_w + root, w)
            } else {
                let t = v - one_w;
                let root = eg::sqrt_fixed::<C::Wagm>(eg::mul::<C::Wagm>(t, t + two_w, w), w);
                eg::log1p_fixed::<C::Wagm>(t + root, w)
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
    <C::Wagm as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_near_special_widening_g, to_work_scaled_g,
    };
    {
        let w0 = SCALE + C::GUARD;
        let v0 = to_work_scaled_g::<C::Storage, Wk>(raw, C::GUARD);
        let ax0 = if v0 < eg::zero::<Wk>() { eg::zero::<Wk>() - v0 } else { v0 };
        if ax0 >= eg::one::<Wk>(w0) {
            panic!("schoolbook atanh: argument out of domain (-1, 1)");
        }
    }
    // The canonical pins — identical to the tier kernel (expanding:
    // atanh(x) = x + x³/3 + …).
    if let Some(p) = hyper_tiny_pin::<C, SCALE, true>(raw, mode) {
        return p;
    }
    // Two-width fall-up (near-special form) to the tier walker width
    // `Wagm` - see [`acosh_schoolbook_g`].
    round_to_storage_directed_near_special_widening_g::<C::Storage, Wk, C::Wagm>(
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
        |guard| {
            let w = SCALE + guard;
            let one_w = eg::one::<C::Wagm>(w);
            let v = to_work_scaled_g::<C::Storage, C::Wagm>(raw, guard);
            (C::ln_fixed_routed_agm::<SCALE>(one_w + v, w)
                - C::ln_fixed_routed_agm::<SCALE>(one_w - v, w))
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

    // Defect-B regression (2026-06-12): large-|x| hyperbolic and
    // deep-negative exp/exp2 cells whose results are IN RANGE for the tier
    // (D115 raw max = 2^383 − 1 ≈ 1.97e115) PANICKED on baked-table-less
    // builds (single-tier `dNN` / `wide`-umbrella): the rescale matcher's
    // Newton arm fell back to a per-call Knuth divide whose dividend —
    // `even(width_limbs + w_ext/19 + 3) + 1` u64 limbs — outran the
    // build-max divide blanket (67 > 66 limbs at `width_limbs = 42`,
    // `w_ext = 407`, `MAX_WORK_N = 16`). Fixed in `rescale::select` (the
    // Newton arm is gated to the table-baking `x-wide`/`xx-wide` builds)
    // and `NewtonReciprocal::precompute` (the fallback now sizes its own
    // Knuth scratch). Inputs are the golden d115 panic rows
    // (sinh.golden:3296/8150, cosh.golden:3322/3350/3352/7270, exp/exp2
    // deep negatives).
    #[cfg(any(feature = "d115", feature = "wide"))]
    mod wide_d115_defect_b {
        use super::*;

        #[test]
        fn sinh_cosh_large_arg_in_range_d115_s0() {
            // All in range: cosh(257) ≈ 1.2e111 .. sinh(266) ≈ 1.66e115 <
            // 2^383 − 1 ≈ 1.97e115. sinh/cosh(|x|) ≥ e^|x|/2 > 10^110 for
            // |x| ≥ 257 (257·log10(e) ≈ 111.6).
            let floor_mag = Int::<6>::TEN.pow(110);
            for &x in &[257i128, 259, 263, 264, 265, 266] {
                let pos = Int::<6>::from_i128(x);
                let neg = Int::<6>::from_i128(-x);
                for &mode in &MODES {
                    let c = D::<Int<6>, 0>(pos).cosh_strict_with(mode).0;
                    let s = D::<Int<6>, 0>(pos).sinh_strict_with(mode).0;
                    assert!(c > floor_mag, "cosh({x}) too small, mode {mode:?}");
                    assert!(s > floor_mag, "sinh({x}) too small, mode {mode:?}");
                    assert!(c >= s, "cosh({x}) < sinh({x}), mode {mode:?}");
                    // Even / odd symmetry against the negative-argument rows.
                    // cosh is even: same value, same mode, same result. sinh
                    // is odd, so directed modes flip across the negation:
                    // round_Floor(-v) = -round_Ceiling(v); the nearest modes
                    // and Trunc are sign-symmetric.
                    assert_eq!(
                        D::<Int<6>, 0>(neg).cosh_strict_with(mode).0,
                        c,
                        "cosh(-{x}) != cosh({x}), mode {mode:?}"
                    );
                    let flipped = match mode {
                        RoundingMode::Floor => RoundingMode::Ceiling,
                        RoundingMode::Ceiling => RoundingMode::Floor,
                        m => m,
                    };
                    let s_flipped = D::<Int<6>, 0>(pos).sinh_strict_with(flipped).0;
                    assert_eq!(
                        D::<Int<6>, 0>(neg).sinh_strict_with(mode).0,
                        Int::<6>::ZERO - s_flipped,
                        "sinh(-{x}) != -sinh({x}) under the flipped mode, mode {mode:?}"
                    );
                }
            }
        }

        #[test]
        fn cosh_large_arg_in_range_d115_s57() {
            // cosh.golden:7270 — cosh(133.6131707362966849971232805) at
            // D115<57>: ≈ e^133.61/2 ≈ 5.3e57, raw ≈ 5.3e114 < 2^383 − 1.
            // raw = 133.6131707362966849971232805 · 10^57 (28 significant
            // digits, 25 of them fractional → ·10^32 to reach scale 57).
            let raw = Int::<6>::from_i128(1_336_131_707_362_966_849_971_232_805)
                * Int::<6>::TEN.pow(32);
            let floor_mag = Int::<6>::TEN.pow(114);
            for &mode in &MODES {
                let c = D::<Int<6>, 57>(raw).cosh_strict_with(mode).0;
                assert!(c > floor_mag, "cosh(133.61..) too small, mode {mode:?}");
                assert_eq!(
                    D::<Int<6>, 57>(Int::<6>::ZERO - raw).cosh_strict_with(mode).0,
                    c,
                    "cosh(-133.61..) != cosh(133.61..), mode {mode:?}"
                );
            }
        }

        #[test]
        fn exp_exp2_deep_negative_in_range_d115() {
            // 0 < e^x, 2^x < 10^-SCALE for these arguments, so every mode
            // rounds to 0 except Ceiling, which gives exactly 1 ulp.
            let one_ulp = Int::<6>::from_i128(1);
            for &x in &[-357i128, -391, -436, -1013, -1089] {
                for &mode in &MODES {
                    let expect = if mode == RoundingMode::Ceiling {
                        one_ulp
                    } else {
                        Int::<6>::ZERO
                    };
                    // Scale 0.
                    let r0 = Int::<6>::from_i128(x);
                    assert_eq!(
                        D::<Int<6>, 0>(r0).exp_strict_with(mode).0,
                        expect,
                        "exp({x}) at s0, mode {mode:?}"
                    );
                    assert_eq!(
                        D::<Int<6>, 0>(r0).exp2_strict_with(mode).0,
                        expect,
                        "exp2({x}) at s0, mode {mode:?}"
                    );
                    // Scale 50 (the deep-escalation band).
                    let r50 = Int::<6>::from_i128(x) * Int::<6>::TEN.pow(50);
                    assert_eq!(
                        D::<Int<6>, 50>(r50).exp_strict_with(mode).0,
                        expect,
                        "exp({x}) at s50, mode {mode:?}"
                    );
                    assert_eq!(
                        D::<Int<6>, 50>(r50).exp2_strict_with(mode).0,
                        expect,
                        "exp2({x}) at s50, mode {mode:?}"
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
