import io

p = 'src/algos/trig/hyper_schoolbook.rs'
s = io.open(p, encoding='utf-8', newline='').read()

start_marker = "/// Schoolbook sinh for a wide tier -- (e^|x| - e^-|x|)/2 (odd)."
end_marker = "// -- Narrow tier -- Int<2> storage, math in the 256-bit Fixed ---------"
start = s.index(start_marker)
end = s.index(end_marker)

new_section = r'''// ── The canonical wide kernels (the `_strict_with` shells' semantics,
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
/// one storage ULP (≤ 1/3 ULP at the band edge, tail included — well
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
) -> Option<C::Storage> {
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
                |av, w| eg::sinh_pos::<C::Wagm>(av, w),
                |av, w| eg::sinh_pos::<C::Wexp>(av, w),
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
                |av, w| eg::cosh_pos::<C::Wagm>(av, w),
                |av, w| eg::cosh_pos::<C::Wexp>(av, w),
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
                |av, w| eg::tanh_pos::<C::Wagm>(av, w),
                |av, w| eg::tanh_pos::<C::Wexp>(av, w),
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

'''

s2 = s[:start] + new_section + s[end:]

# Now replace the RUNG kernels inside the remaining tail (the old _g bodies),
# matching the new tier semantics: pins first, S2 = Wexp (sinh/cosh, never_exact)
# / Wagm (tanh/asinh/acosh/atanh).
s = s2

# sinh_schoolbook_g
old = """pub(crate) fn sinh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_widening_g;
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    // Two-width fall-up: an unresolved-at-rung-cap near-tie reruns the
    // walker at the tier work width with the TIER kernel's closure,
    // verbatim - see `round_to_storage_directed_widening_g`.
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
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
        |guard| {
            let w = SCALE + guard;
            let v = C::to_work_scaled(raw, guard);
            let av = if v < C::zero() { C::zero() - v } else { v };
            let sh = C::sinh_pos_wide::<SCALE>(av, w);
            if neg { C::zero() - sh } else { sh }
        },
    )
}"""
new = """pub(crate) fn sinh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
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
                |av, w| eg::sinh_pos::<C::Wexp>(av, w),
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
}"""
assert s.count(old) == 1, "sinh_g"
s = s.replace(old, new)

# cosh_schoolbook_g
old = """pub(crate) fn cosh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_widening_g;
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    // Two-width fall-up - see [`sinh_schoolbook_g`].
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
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
        |guard| {
            let w = SCALE + guard;
            let v = C::to_work_scaled(raw, guard);
            let av = if v < C::zero() { C::zero() - v } else { v };
            C::cosh_pos_wide::<SCALE>(av, w)
        },
    )
}"""
new = """pub(crate) fn cosh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
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
                |av, w| eg::cosh_pos::<C::Wexp>(av, w),
            )
        },
        |guard| {
            let w = SCALE + guard;
            let v = to_work_scaled_g::<C::Storage, C::Wexp>(raw, guard);
            let av = if v < eg::zero::<C::Wexp>() { eg::zero::<C::Wexp>() - v } else { v };
            eg::cosh_pos::<C::Wexp>(av, w)
        },
    )
}"""
assert s.count(old) == 1, "cosh_g"
s = s.replace(old, new)

# tanh_schoolbook_g
old = """pub(crate) fn tanh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::Wexp as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::round_to_storage_directed_widening_g;
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_scaled(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    // Two-width fall-up - see [`sinh_schoolbook_g`].
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
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
        |guard| {
            let w = SCALE + guard;
            let v = C::to_work_scaled(raw, guard);
            let av = if v < C::zero() { C::zero() - v } else { v };
            let th = C::tanh_pos_wide::<SCALE>(av, w);
            if neg { C::zero() - th } else { th }
        },
    )
}"""
new = """pub(crate) fn tanh_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
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
    // fast path is unreachable here (the policy gate admits |x| < 10,
    // far below the onset) but kept for kernel-level callers.
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
                |av, w| eg::tanh_pos::<Wk>(av, w),
                |av, w| eg::tanh_pos::<C::Wexp>(av, w),
            );
            if neg { eg::zero::<Wk>() - th } else { th }
        },
        |guard| {
            let w = SCALE + guard;
            let th = hyper_probe_g::<C, C::Wagm>(
                raw,
                guard,
                w,
                |av, w| eg::tanh_pos::<C::Wagm>(av, w),
                |av, w| eg::tanh_pos::<C::Wexp>(av, w),
            );
            if neg { eg::zero::<C::Wagm>() - th } else { th }
        },
    )
}"""
assert s.count(old) == 1, "tanh_g"
s = s.replace(old, new)

io.open(p, 'w', encoding='utf-8', newline='').write(s)
print("tier + sinh/cosh/tanh rung ok")
