// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Inverse-trig schoolbook reference kernels -- asin / acos / atan2.
//!
//! Naive textbook realisations of the inverse family, registered as an
//! unrouted `Schoolbook` arm of
//! [`crate::policy::trig::inverse::Algorithm`]. Correctness reference +
//! A/B microbench partner for the routed `Atan` kernels; `select` never
//! routes here. Each is the textbook composition over the leaf atan
//! kernel + the work-int `sqrt_fixed` leaf, dispatched DOWN to the
//! `Int<N>` layer. NEVER calls a decimal `*_strict_with` on its own
//! value (the inversion dec-trig had to avoid). Identical composition +
//! narrowing as the routed kernel, so it matches bit-exactly.

use crate::algos::ln::ln_series_2limb::STRICT_GUARD;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::{atan2_kernel, atan_fixed, to_fixed, wide_half_pi};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[inline]
fn asin_work<C: WideTrigCore, const SCALE: u32>(v: C::W, w: u32) -> C::W {
    let one_w = C::one(w);
    let abs_v = if v < C::zero() { C::zero() - v } else { v };
    let half_w = one_w >> 1;
    if abs_v == one_w {
        let hp = C::half_pi::<SCALE>(w);
        if v < C::zero() { C::zero() - hp } else { hp }
    } else if abs_v <= half_w {
        let denom = C::sqrt_fixed(one_w - C::mul(v, v, w), w);
        C::atan_fixed::<SCALE>(C::div(v, denom, w), w)
    } else {
        let inner = (one_w - abs_v) >> 1;
        let inner_sqrt = C::sqrt_fixed(inner, w);
        let inner_denom = C::sqrt_fixed(one_w - C::mul(inner_sqrt, inner_sqrt, w), w);
        let inner_asin = C::atan_fixed::<SCALE>(C::div(inner_sqrt, inner_denom, w), w);
        let result_abs = C::half_pi::<SCALE>(w) - inner_asin - inner_asin;
        if v < C::zero() { C::zero() - result_abs } else { result_abs }
    }
}

/// Schoolbook asin for a wide tier. Panics if |x| > 1.
#[inline]
#[must_use]
pub(crate) fn asin_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_decided_g, tiny_x_deep_directed_adjust, tiny_x_linear_directed,
    };
    let w0 = SCALE + C::GUARD;
    let one_w0 = C::one(w0);
    let v0 = C::to_work(raw);
    let abs_v0 = if v0 < C::zero() { C::zero() - v0 } else { v0 };
    if abs_v0 > one_w0 {
        panic!("schoolbook asin: argument out of domain [-1, 1]");
    }
    // Analytic tiny-`x` directed decision (relocated from the policy layer) —
    // `asin(x) = x + x³/6 + …` EXPANDS (every Taylor coefficient is positive).
    if let Some(v) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, true) {
        return v;
    }
    // Ziv-escalated narrowing (NOT a single shot): the composition's true
    // value can sit a sub-resolution distance from a rounding boundary
    // while the fixed-w partial lands exactly ON it — asin(3·10⁻⁶⁰) at
    // SCALE 180 has x³/6 = 4.5 ULP EXACT and the deciding +3x⁵/40 tail at
    // fraction depth ~298, beyond any fixed GUARD. The walker's base
    // probe is this same single evaluation (clear-of-band inputs exit
    // there, no cost added); a near-tie escalates the working scale.
    let (r, decided) = round_to_storage_directed_decided_g::<C::Storage, C::W>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| asin_work::<C, SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
    );
    // Deep sub-resolution band (`j* ≥ 5`): `asin` always EXPANDS.
    tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        r,
        decided,
        raw,
        mode,
        false,
        <C::W as crate::int::types::traits::BigInt>::BITS,
    )
}

/// Schoolbook acos for a wide tier -- pi/2 - asin(x). Panics if |x| > 1.
#[inline]
#[must_use]
pub(crate) fn acos_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w0 = SCALE + C::GUARD;
    let one_w0 = C::one(w0);
    let v0 = C::to_work(raw);
    let abs_v0 = if v0 < C::zero() { C::zero() - v0 } else { v0 };
    if abs_v0 > one_w0 {
        panic!("schoolbook acos: argument out of domain [-1, 1]");
    }
    // Ziv-escalated narrowing — see [`asin_schoolbook`].
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        C::half_pi::<SCALE>(w) - asin_work::<C, SCALE>(C::to_work_scaled(raw, guard), w)
    })
}

/// Schoolbook atan2 for a wide tier -- quadrant-resolved atan(y/x).
#[inline]
#[must_use]
pub(crate) fn atan2_schoolbook<C: WideTrigCore, const SCALE: u32>(
    y_raw: C::Storage,
    x_raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_decided_g, tiny_x_deep_directed_adjust, tiny_x_linear_directed,
    };
    let zero = C::storage_zero();
    let abs_y = if y_raw < zero { zero - y_raw } else { y_raw };
    // Tiny-result directed decision. atan2(y, x) reduces to atan(y/x) near 0 —
    // a ±π-offset-free result that can be tiny — ONLY for x > 0 and |y/x| < 1.
    if !crate::support::rounding::is_nearest_mode(mode) && x_raw > zero && abs_y < x_raw {
        // Probe Floor and Ceiling. They AGREE (== G) iff the result is EXACTLY
        // on the storage grid: the RESOLVED Taylor terms terminate, the deciding
        // odd term is sub-resolution, and the directed walker is mode-blind — the
        // genuine analytic tie. They DIFFER iff the result is OFF-grid: a
        // RESOLVED but non-terminating Taylor term (atan's z⁷/7 once 7k ≤ SCALE,
        // i.e. j* ≥ 9) hands the walker a REACHABLE deciding residual it already
        // rounds directed correctly, so trust it.
        //
        // This Floor==Ceiling test is the robust discriminator: the walker's own
        // `decided` flag is NOT (a sub-resolution linear tie can false-positive
        // to `decided == true` via paired exact-zero probes — d307 s120 — yet
        // still needs the analytic step; a genuinely-resolved off-grid cell —
        // d924 s923 — is also `decided == true` but must NOT be stepped).
        let probe = |m: RoundingMode| {
            round_to_storage_directed_decided_g::<C::Storage, C::W>(
                C::GUARD,
                SCALE,
                m,
                C::storage_max(),
                C::storage_min(),
                |guard| atan2_work::<C, SCALE>(y_raw, x_raw, guard),
            )
            .0
        };
        let r_f = probe(RoundingMode::Floor);
        let r_c = probe(RoundingMode::Ceiling);
        if r_f == r_c {
            // On grid: G = r_f. SINGLE analytic step from G — `atan z = z − z³/3
            // + z⁵/5 − …` COMPRESSES (cubic −) and ALTERNATES, so the linear
            // (j* = 3) and deep (j* ≥ 5, alternating) helpers place the directed
            // neighbour exactly one ULP from G.
            let g = r_f;
            if let Some(v) = tiny_x_linear_directed::<C::Storage, SCALE>(g, mode, false) {
                return v;
            }
            let stepped = tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
                g,
                false,
                g,
                mode,
                true,
                <C::W as crate::int::types::traits::BigInt>::BITS,
            );
            if stepped != g {
                return stepped;
            }
            // On grid but not in the tiny band (the helpers no-op): G is exact.
            return g;
        }
        // Off grid: the walker resolved the directed rounding. Return its result
        // for `mode` (Trunc is toward zero — x > 0 so the result sign is y's).
        return match mode {
            RoundingMode::Ceiling => r_c,
            RoundingMode::Floor => r_f,
            RoundingMode::Trunc => {
                if y_raw >= zero {
                    r_f
                } else {
                    r_c
                }
            }
            _ => unreachable!("directed mode"),
        };
    }
    // Nearest modes, x ≤ 0, or |y| ≥ |x| (non-tiny |result|): ordinary narrowing.
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        atan2_work::<C, SCALE>(y_raw, x_raw, guard)
    })
}

/// One working-scale atan2 evaluation at `w = SCALE + guard` — the
/// quadrant-resolved composition body shared by the walker probes.
#[inline]
fn atan2_work<C: WideTrigCore, const SCALE: u32>(
    y_raw: C::Storage,
    x_raw: C::Storage,
    guard: u32,
) -> C::W {
    let w = SCALE + guard;
    let z = C::storage_zero();
    if x_raw == z {
        return if y_raw > z {
            C::half_pi::<SCALE>(w)
        } else if y_raw < z {
            C::zero() - C::half_pi::<SCALE>(w)
        } else {
            C::zero()
        };
    }
    let y = C::to_work_scaled(y_raw, guard);
    let x = C::to_work_scaled(x_raw, guard);
    let zero_w = C::zero();
    let abs_y = if y < zero_w { zero_w - y } else { y };
    let abs_x = if x < zero_w { zero_w - x } else { x };
    let base = if abs_x >= abs_y {
        C::atan_fixed::<SCALE>(C::div(y, x, w), w)
    } else {
        let inv = C::atan_fixed::<SCALE>(C::div(x, y, w), w);
        let hp = C::half_pi::<SCALE>(w);
        let same_sign = (y < zero_w) == (x < zero_w);
        if same_sign { hp - inv } else { (zero_w - hp) - inv }
    };
    if x_raw > z {
        base
    } else if y_raw >= z {
        base + C::pi::<SCALE>(w)
    } else {
        base - C::pi::<SCALE>(w)
    }
}

// ── Rung-generic shells (the SCALE-derived work-rung surface) ─────────
//
// The same compositions run at an arbitrary work rung `Wk` (decoupled
// from `C::W`), so the policy can run them at the minimal valid work
// width (mirrors `wide_trig_core::sin_series_g` / `atan_series_g`;
// the `C::W`-bound kernels above stay the tier-width realisation,
// value-identical — every leaf is the identical width-agnostic
// `exp_generic` integer op the per-tier core forwards to, and `π` comes
// from the same per-scale constant table).

/// Rung-generic [`asin_work`] — `π` supplied at the working scale
/// (only its `π/2` half is consumed).
#[cfg(feature = "_wide-support")]
#[inline]
fn asin_work_g<Wk: crate::int::types::traits::BigInt>(v: Wk, w: u32, pi_w: Wk) -> Wk
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    let one_w = eg::one::<Wk>(w);
    let zero = eg::zero::<Wk>();
    let abs_v = if v < zero { zero - v } else { v };
    let half_w = one_w >> 1;
    if abs_v == one_w {
        let hp = pi_w >> 1;
        if v < zero { zero - hp } else { hp }
    } else if abs_v <= half_w {
        let denom = eg::sqrt_fixed::<Wk>(one_w - eg::mul::<Wk>(v, v, w), w);
        crate::algos::trig::trig_generic::atan_fixed::<Wk>(eg::div::<Wk>(v, denom, w), w, pi_w)
    } else {
        let inner = (one_w - abs_v) >> 1;
        let inner_sqrt = eg::sqrt_fixed::<Wk>(inner, w);
        let inner_denom =
            eg::sqrt_fixed::<Wk>(one_w - eg::mul::<Wk>(inner_sqrt, inner_sqrt, w), w);
        let inner_asin = crate::algos::trig::trig_generic::atan_fixed::<Wk>(
            eg::div::<Wk>(inner_sqrt, inner_denom, w),
            w,
            pi_w,
        );
        let result_abs = (pi_w >> 1) - inner_asin - inner_asin;
        if v < zero { zero - result_abs } else { result_abs }
    }
}

/// Rung-generic [`asin_schoolbook`] — the atan-of-ratio composition at
/// an arbitrary work rung `Wk`. Panics if |x| > 1 (the policy gate only
/// admits magnitudes whose lift provably fits the rung, so the domain
/// check fires at the rung exactly as at the tier width).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn asin_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        pi_at_rung, round_to_storage_directed_widening_decided_g, tiny_x_deep_directed_adjust,
        tiny_x_linear_directed, to_work_scaled_g,
    };
    let w0 = SCALE + C::GUARD;
    let one_w0 = eg::one::<Wk>(w0);
    let zero = eg::zero::<Wk>();
    let v0 = to_work_scaled_g::<C::Storage, Wk>(raw, C::GUARD);
    let abs_v0 = if v0 < zero { zero - v0 } else { v0 };
    if abs_v0 > one_w0 {
        panic!("schoolbook asin: argument out of domain [-1, 1]");
    }
    // Analytic tiny-`x` directed decision — the SAME pre-empt the tier
    // [`asin_schoolbook`] carries (relocated from the policy layer).
    if let Some(v) = tiny_x_linear_directed::<C::Storage, SCALE>(raw, mode, true) {
        return v;
    }
    // Ziv-escalated two-width narrowing — see the tier [`asin_schoolbook`]
    // (the asin(3e-60) partial-sum family): rung probes first, an
    // unresolved-at-rung-cap walk falls up to the tier width.
    let (r, decided) = round_to_storage_directed_widening_decided_g::<C::Storage, Wk, C::W>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            asin_work_g::<Wk>(
                to_work_scaled_g::<C::Storage, Wk>(raw, guard),
                w,
                pi_at_rung::<Wk>(w, w0),
            )
        },
        |guard| asin_work::<C, SCALE>(C::to_work_scaled(raw, guard), SCALE + guard),
    );
    // Deep sub-resolution band (`j* ≥ 5`): `asin` always EXPANDS.
    tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
        r,
        decided,
        raw,
        mode,
        false,
        <C::W as crate::int::types::traits::BigInt>::BITS,
    )
}

/// Rung-generic [`acos_schoolbook`] — `π/2 − asin(x)` at an arbitrary
/// work rung `Wk`. Panics if |x| > 1.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn acos_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{
        pi_at_rung, round_to_storage_directed_widening_g, to_work_scaled_g,
    };
    let w0 = SCALE + C::GUARD;
    let one_w0 = eg::one::<Wk>(w0);
    let zero = eg::zero::<Wk>();
    let v0 = to_work_scaled_g::<C::Storage, Wk>(raw, C::GUARD);
    let abs_v0 = if v0 < zero { zero - v0 } else { v0 };
    if abs_v0 > one_w0 {
        panic!("schoolbook acos: argument out of domain [-1, 1]");
    }
    // Ziv-escalated two-width narrowing — see [`asin_schoolbook_g`].
    round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
        C::GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard| {
            let w = SCALE + guard;
            let pi_w = pi_at_rung::<Wk>(w, w0);
            (pi_w >> 1)
                - asin_work_g::<Wk>(to_work_scaled_g::<C::Storage, Wk>(raw, guard), w, pi_w)
        },
        |guard| {
            let w = SCALE + guard;
            C::half_pi::<SCALE>(w) - asin_work::<C, SCALE>(C::to_work_scaled(raw, guard), w)
        },
    )
}

/// Rung-generic [`atan2_schoolbook`] — quadrant-resolved `atan(y/x)` at
/// an arbitrary work rung `Wk` (both operands gated by the policy).
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn atan2_schoolbook_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    y_raw: C::Storage,
    x_raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
    <C::W as crate::int::types::traits::BigInt>::Scratch:
        crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_widening_g, tiny_x_deep_directed_adjust, tiny_x_linear_directed,
    };
    let w0 = SCALE + C::GUARD;
    let zero = C::storage_zero();
    let abs_y = if y_raw < zero { zero - y_raw } else { y_raw };
    // Two-width directed narrowing (rung-first, fall-up to the tier width C::W).
    let walk = |m: RoundingMode| {
        round_to_storage_directed_widening_g::<C::Storage, Wk, C::W>(
            C::GUARD,
            SCALE,
            m,
            C::storage_max(),
            C::storage_min(),
            |guard| atan2_work_g::<C, Wk, SCALE>(y_raw, x_raw, guard, w0),
            |guard| atan2_work::<C, SCALE>(y_raw, x_raw, guard),
        )
    };
    // Tiny-result directed decision — see the tier [`atan2_schoolbook`]: probe
    // Floor/Ceiling; AGREE ⟺ exactly on grid (mode-blind, deciding odd term
    // sub-resolution → SINGLE analytic step from G = r_f); DIFFER ⟺ off-grid (a
    // resolved non-terminating term, j* ≥ 9 → the walker already rounds it
    // correctly). atan compresses + alternates.
    if !crate::support::rounding::is_nearest_mode(mode) && x_raw > zero && abs_y < x_raw {
        let r_f = walk(RoundingMode::Floor);
        let r_c = walk(RoundingMode::Ceiling);
        if r_f == r_c {
            let g = r_f;
            if let Some(v) = tiny_x_linear_directed::<C::Storage, SCALE>(g, mode, false) {
                return v;
            }
            let stepped = tiny_x_deep_directed_adjust::<C::Storage, SCALE>(
                g,
                false,
                g,
                mode,
                true,
                <C::W as crate::int::types::traits::BigInt>::BITS,
            );
            if stepped != g {
                return stepped;
            }
            return g;
        }
        return match mode {
            RoundingMode::Ceiling => r_c,
            RoundingMode::Floor => r_f,
            RoundingMode::Trunc => {
                if y_raw >= zero {
                    r_f
                } else {
                    r_c
                }
            }
            _ => unreachable!("directed mode"),
        };
    }
    walk(mode)
}

/// One rung-width atan2 evaluation at `w = SCALE + guard` — the
/// quadrant-resolved composition body shared by the rung walker probes
/// (`base_w0` keys the hot-path const-fold of `π`).
#[cfg(feature = "_wide-support")]
#[inline]
fn atan2_work_g<C: WideTrigCore, Wk: crate::int::types::traits::BigInt, const SCALE: u32>(
    y_raw: C::Storage,
    x_raw: C::Storage,
    guard: u32,
    base_w0: u32,
) -> Wk
where
    Wk::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::exp::exp_generic as eg;
    use crate::algos::support::wide_trig_core::{pi_at_rung, to_work_scaled_g};
    use crate::algos::trig::trig_generic::atan_fixed;
    let w = SCALE + guard;
    let z = C::storage_zero();
    let pi_w = pi_at_rung::<Wk>(w, base_w0);
    if x_raw == z {
        return if y_raw > z {
            pi_w >> 1
        } else if y_raw < z {
            eg::zero::<Wk>() - (pi_w >> 1)
        } else {
            eg::zero::<Wk>()
        };
    }
    let y = to_work_scaled_g::<C::Storage, Wk>(y_raw, guard);
    let x = to_work_scaled_g::<C::Storage, Wk>(x_raw, guard);
    let zero_w = eg::zero::<Wk>();
    let abs_y = if y < zero_w { zero_w - y } else { y };
    let abs_x = if x < zero_w { zero_w - x } else { x };
    let base = if abs_x >= abs_y {
        atan_fixed::<Wk>(eg::div::<Wk>(y, x, w), w, pi_w)
    } else {
        let inv = atan_fixed::<Wk>(eg::div::<Wk>(x, y, w), w, pi_w);
        let hp = pi_w >> 1;
        let same_sign = (y < zero_w) == (x < zero_w);
        if same_sign { hp - inv } else { (zero_w - hp) - inv }
    };
    if x_raw > z {
        base
    } else if y_raw >= z {
        base + pi_w
    } else {
        base - pi_w
    }
}

#[inline]
fn asin_work_narrow(v: Fixed, w: u32) -> Fixed {
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let abs_v = Fixed { negative: false, mag: v.mag };
    if abs_v == one_w {
        let hp = wide_half_pi(w);
        return if v.negative { hp.neg() } else { hp };
    }
    let half_w = one_w.halve();
    if !abs_v.ge_mag(half_w) {
        let denom = one_w.sub(v.mul(v, w)).sqrt(w);
        atan_fixed(v.div(denom, w), w)
    } else {
        let inner = one_w.sub(abs_v).halve();
        let inner_sqrt = inner.sqrt(w);
        let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
        let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
        if v.negative { result_abs.neg() } else { result_abs }
    }
}

#[inline]
#[must_use]
fn asin_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed(raw);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "asin: argument out of domain [-1, 1]"
    );
    // Near-tie protected terminal, mirroring the routed
    // `trig_series_2limb::asin_strict_raw` (the schoolbook is the
    // bit-exact reference, so it must decide ties the same way).
    match asin_work_narrow(v, w).round_to_i128_clear_of_tie(w, SCALE, mode) {
        Some(v) => v.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE)
        }),
        None => crate::algos::support::narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |g| {
            crate::algos::trig::trig_series_2limb::asin_ziv(raw, SCALE, g)
        }),
    }
}

#[inline]
#[must_use]
fn acos_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    use crate::types::consts::DecimalConstants;
    // Endpoint pins, matching the routed `acos_strict_raw`: acos(1) = 0
    // is EXACT (pinned for every mode); acos(0) = pi/2 and acos(-1) = pi
    // are IRRATIONAL, so their half-even-baked constants hold for the
    // NEAREST modes only — directed modes fall through to the series +
    // mode-aware rounding.
    if raw == 0 && crate::support::rounding::is_nearest_mode(mode) {
        return <crate::D<Int<2>, SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits && crate::support::rounding::is_nearest_mode(mode) {
        return <crate::D<Int<2>, SCALE> as DecimalConstants>::pi().0.as_i128();
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed(raw);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "acos: argument out of domain [-1, 1]"
    );
    // Near-tie protected terminal — see `asin_schoolbook_raw`.
    match wide_half_pi(w)
        .sub(asin_work_narrow(v, w))
        .round_to_i128_clear_of_tie(w, SCALE, mode)
    {
        Some(v) => v.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("acos", SCALE)
        }),
        None => crate::algos::support::narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |g| {
            crate::algos::trig::trig_series_2limb::acos_ziv(raw, SCALE, g)
        }),
    }
}

#[inline]
#[must_use]
fn atan2_schoolbook_raw<const SCALE: u32>(y_raw: i128, x_raw: i128, mode: RoundingMode) -> i128 {
    let w = SCALE + STRICT_GUARD;
    // Near-tie protected terminal — see `asin_schoolbook_raw`.
    match atan2_kernel(to_fixed(y_raw), to_fixed(x_raw), y_raw, w)
        .round_to_i128_clear_of_tie(w, SCALE, mode)
    {
        Some(v) => v.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("atan2", SCALE)
        }),
        None => crate::algos::support::narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |g| {
            crate::algos::trig::trig_series_2limb::atan2_ziv(y_raw, x_raw, SCALE, g)
        }),
    }
}

/// Narrow schoolbook asin for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn asin_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(asin_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook acos for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn acos_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(acos_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook atan2 for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn atan2_schoolbook_narrow<const SCALE: u32>(
    y_raw: Int<2>,
    x_raw: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    Int::<2>::from_i128(atan2_schoolbook_raw::<SCALE>(y_raw.as_i128(), x_raw.as_i128(), mode))
}

// -- Unit tests: each schoolbook is bit-exact against the routed kernel.
//
// The schoolbook is the correctness reference (skill 7): it MUST produce
// the SAME storage raw as the golden-validated routed kernel at every
// input, scale, tier and mode. We assert delta == 0 over a range that
// covers the half-angle branch (|x| > 1/2), the |x| -> 1 boundary, the
// negative-argument fold, and the atan2 quadrants. A mismatch is a hard
// failure, never weakened.
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
    // domain [-1, 1] at scale 12: 0, 0.25, 0.5, 0.6 (>1/2 branch), 0.9,
    // 1.0 (endpoint), and negatives.
    const ASIN_INPUTS: [i128; 11] = [
        0,
        250_000_000_000,
        500_000_000_000,
        600_000_000_000,
        900_000_000_000,
        1_000_000_000_000,
        -250_000_000_000,
        -500_000_000_000,
        -600_000_000_000,
        -900_000_000_000,
        -1_000_000_000_000,
    ];

    #[test]
    fn asin_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &ASIN_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    asin_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).asin_strict_with(mode).0,
                    "asin schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn acos_schoolbook_narrow_matches_routed_kernel() {
        // Exclude the exact endpoints acos(0)=pi/2, acos(+-1)=0/pi: under
        // the wide features the D38 routed path borrows D57 and rounds
        // those 1 ULP high in the directed modes (the documented
        // borrow-D57 endpoint quirk, see trig_schoolbook::atan(1)). The
        // schoolbook pins the correctly-rounded constant there, so it is
        // the valid reference -- the endpoints are asserted separately
        // against the external (DecimalConstants / mpmath) value below.
        let one_bits: i128 = 10_i128.pow(S38);
        for &raw in &ASIN_INPUTS {
            if raw == 0 || raw == one_bits || raw == -one_bits {
                continue;
            }
            for &mode in &MODES {
                assert_eq!(
                    acos_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).acos_strict_with(mode).0,
                    "acos schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn acos_schoolbook_narrow_endpoints_are_correctly_rounded() {
        use crate::types::consts::DecimalConstants;
        // External oracle = the mpmath-pinned DecimalConstants for the
        // NEAREST modes (acos(0) = pi/2, acos(-1) = pi are irrational, so
        // the half-even-baked constant is only the nearest-modes answer);
        // acos(1) = 0 is exact in every mode. Directed modes must agree
        // with the routed kernel's mode-aware computation bit-exactly.
        let one_bits: i128 = 10_i128.pow(S38);
        let half_pi = <D<Int<2>, S38> as DecimalConstants>::half_pi().0;
        let pi = <D<Int<2>, S38> as DecimalConstants>::pi().0;
        for mode in [RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero] {
            assert_eq!(acos_schoolbook_narrow::<S38>(d38(0).0, mode), half_pi);
            assert_eq!(acos_schoolbook_narrow::<S38>(d38(-one_bits).0, mode), pi);
        }
        for &mode in &MODES {
            assert_eq!(acos_schoolbook_narrow::<S38>(d38(one_bits).0, mode), Int::<2>::from_i128(0));
            assert_eq!(
                acos_schoolbook_narrow::<S38>(d38(0).0, mode),
                d38(0).acos_strict_with(mode).0,
                "acos(0) schoolbook != routed at mode={mode:?}"
            );
            assert_eq!(
                acos_schoolbook_narrow::<S38>(d38(-one_bits).0, mode),
                d38(-one_bits).acos_strict_with(mode).0,
                "acos(-1) schoolbook != routed at mode={mode:?}"
            );
        }
    }

    #[test]
    fn atan2_schoolbook_narrow_matches_routed_kernel() {
        // (y, x) across all four quadrants + axes.
        const PTS: [(i128, i128); 9] = [
            (1_000_000_000_000, 1_000_000_000_000),
            (1_000_000_000_000, -1_000_000_000_000),
            (-1_000_000_000_000, 1_000_000_000_000),
            (-1_000_000_000_000, -1_000_000_000_000),
            (1_000_000_000_000, 0),
            (-1_000_000_000_000, 0),
            (0, 1_000_000_000_000),
            (500_000_000_000, 2_000_000_000_000),
            (2_000_000_000_000, 500_000_000_000),
        ];
        for &(y, x) in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    atan2_schoolbook_narrow::<S38>(d38(y).0, d38(x).0, mode),
                    d38(y).atan2_strict_with(d38(x), mode).0,
                    "atan2 schoolbook != routed at y={y} x={x} mode={mode:?}"
                );
            }
        }
    }

    // The asin near-half deciding-tail cell the regenerated golden gate
    // caught (D462<180>, HalfToEven, x = 3e-60): asin(x) = x + x^3/6 +
    // 3x^5/40 + ...; x is exact, so x^3/6 = 27e-180/6 = 4.5e-180 EXACTLY
    // 4.5 storage ULPs, and the 3x^5/40 term (~1.8e-298, positive) puts
    // the true value STRICTLY above the half - HalfToEven must round UP
    // to raw + 5. A single-shot narrowing at w = SCALE + GUARD = 210
    // sees exactly the half (the x^5 term underflows) and ties-to-even
    // DOWN to raw + 4 - the pre-existing defect the Ziv escalation
    // fixes. Oracle: the exact rational partial sum above (mpmath
    // confirms via the regenerated asin.golden row).
    #[cfg(feature = "d462")]
    mod near_half_tail_d462 {
        use super::*;
        use crate::int::types::Int;

        #[test]
        fn asin_3e60_d462_s180_rounds_above_the_half() {
            type Core = crate::types::widths::wide_trig_d462::Core;
            let raw = Int::<24>::from_i128(3) * Int::<24>::from_i128(10).pow(120);
            let expect_nearest = raw + Int::<24>::from_i128(5); // x + 4.5+ ULP, above half
            let expect_floor = raw + Int::<24>::from_i128(4);
            // Tier kernel.
            assert_eq!(
                asin_schoolbook::<Core, 180>(raw, RoundingMode::HalfToEven),
                expect_nearest,
                "tier asin HalfToEven"
            );
            assert_eq!(
                asin_schoolbook::<Core, 180>(raw, RoundingMode::Floor),
                expect_floor,
                "tier asin Floor"
            );
            assert_eq!(
                asin_schoolbook::<Core, 180>(raw, RoundingMode::Ceiling),
                expect_nearest,
                "tier asin Ceiling"
            );
            // Public path (policy -> rung walker -> tier fall-up).
            let x = crate::D::<Int<24>, 180>(raw);
            assert_eq!(
                x.asin_strict_with(RoundingMode::HalfToEven).0,
                expect_nearest,
                "public asin HalfToEven"
            );
            for mode in MODES {
                assert_eq!(
                    x.asin_strict_with(mode).0,
                    asin_schoolbook::<Core, 180>(raw, mode),
                    "public == tier at mode {mode:?}"
                );
            }
        }
    }

    // The tiny-RESULT directed cells the six-mode golden-comprehensive gate
    // caught for atan2: atan2(y, 1) with x>0 and tiny y reduces to atan(y),
    // whose sub-resolution odd Taylor term decides the directed side.
    // atan(z) = z - z^3/3 + z^5/5 - ...: the kernel recovers the NEAREST grid
    // value G and takes a SINGLE analytic step from it (compress for j*=3, then
    // alternating: expand for j*=5, compress for j*=7, ...). Validated across
    // the LINEAR -> DEEP band transition at MANY scales (not one cell), all six
    // modes, with a single-step invariant (|directed - G| <= 1 ULP, never 2 —
    // a double-step would return G∓2).
    //
    // Generic over the tier `C` and the const scale `S`; `coeff * 10^-big_k` is
    // the tiny argument (x = 1). The expected step is derived from the analytic
    // band: j* = first ODD j with j*k > S; atan alternates, so the deciding term
    // EXPANDS iff j* % 4 == 1.
    // On-grid analytic cell (j* <= 7: the first non-terminating Taylor term
    // z^7/7 stays sub-resolution, so the resolved partial is on the grid).
    // `coeff` may be NEGATIVE (the result sign flips). Valid for both the
    // mode-blind cells (analytic step) and the walker-resolved j*<=7 cells
    // (the walker agrees, deciding on the SAME z^{j*} sign).
    #[cfg(any(feature = "d462", feature = "d307", feature = "d616"))]
    fn assert_atan2_tiny_single_step<C: WideTrigCore, const S: u32>(coeff: i128, big_k: u32)
    where
        <C::W as crate::int::types::traits::BigInt>::Scratch:
            crate::int::types::compute_limbs::ComputeLimbs,
    {
        use crate::int::types::traits::BigInt;
        let p = |n: u32| crate::consts::pow10::dispatch::<C::Storage>(n);
        let one = <C::Storage as BigInt>::from_i128(1);
        let y = <C::Storage as BigInt>::from_i128(coeff) * p(S - big_k);
        let x = p(S); // 1.0
        let g = atan2_schoolbook::<C, S>(y, x, RoundingMode::HalfToEven);
        assert_eq!(
            atan2_schoolbook::<C, S>(y, x, RoundingMode::HalfAwayFromZero),
            g,
            "nearest modes agree S={S} k={big_k} coeff={coeff}"
        );
        let j_min = S / big_k + 1;
        let j_star = if j_min % 2 == 1 { j_min } else { j_min + 1 };
        let expanding = j_star % 4 == 1;
        let positive = coeff > 0;
        // atan COMPRESSES (cubic −), then alternates. The directed neighbour per
        // (deciding-term direction, result sign) — mirrors
        // `tiny_odd_{compressing,expanding}_directed`.
        let (floor_e, trunc_e, ceil_e) = match (expanding, positive) {
            (true, true) => (g, g, g + one),            // expanding +: Ceil up
            (true, false) => (g - one, g, g),           // expanding −: Floor down
            (false, true) => (g - one, g - one, g),     // compress +: Floor/Trunc down
            (false, false) => (g, g + one, g + one),    // compress −: Trunc/Ceil up
        };
        assert_eq!(
            atan2_schoolbook::<C, S>(y, x, RoundingMode::Floor),
            floor_e,
            "Floor S={S} k={big_k} coeff={coeff} j*={j_star} exp={expanding}"
        );
        assert_eq!(
            atan2_schoolbook::<C, S>(y, x, RoundingMode::Trunc),
            trunc_e,
            "Trunc S={S} k={big_k} coeff={coeff} j*={j_star} exp={expanding}"
        );
        assert_eq!(
            atan2_schoolbook::<C, S>(y, x, RoundingMode::Ceiling),
            ceil_e,
            "Ceiling S={S} k={big_k} coeff={coeff} j*={j_star} exp={expanding}"
        );
        // Single-step invariant: every directed result within ONE ULP of G.
        for m in [RoundingMode::Floor, RoundingMode::Trunc, RoundingMode::Ceiling] {
            let r = atan2_schoolbook::<C, S>(y, x, m);
            let d = if r < g { g - r } else { r - g };
            assert!(d <= one, "single-step |r-G|<=1 S={S} k={big_k} coeff={coeff} mode={m:?}");
        }
    }

    // Off-grid deep cell (j* >= 9: the first non-terminating Taylor term z^7/7
    // is RESOLVED, putting the partial OFF the grid). The kernel must return the
    // directed WALKER's result (Floor != Ceiling), NOT an analytic step from the
    // mode-blind G — the residual decider is z^7/7, not z^{j*}. A forced-analytic
    // kernel would disagree with the walker, so this
    // pins the off-grid routing. `coeff` may be NEGATIVE.
    #[cfg(any(feature = "d924", feature = "d1232"))]
    fn assert_atan2_offgrid_uses_walker<C: WideTrigCore, const S: u32>(coeff: i128, big_k: u32)
    where
        <C::W as crate::int::types::traits::BigInt>::Scratch:
            crate::int::types::compute_limbs::ComputeLimbs,
    {
        use crate::algos::support::wide_trig_core::round_to_storage_directed_decided_g;
        use crate::int::types::traits::BigInt;
        let p = |n: u32| crate::consts::pow10::dispatch::<C::Storage>(n);
        let one = <C::Storage as BigInt>::from_i128(1);
        let y = <C::Storage as BigInt>::from_i128(coeff) * p(S - big_k);
        let x = p(S);
        let g = atan2_schoolbook::<C, S>(y, x, RoundingMode::HalfToEven);
        let walker = |m: RoundingMode| {
            round_to_storage_directed_decided_g::<C::Storage, C::W>(
                C::GUARD,
                S,
                m,
                C::storage_max(),
                C::storage_min(),
                |guard| atan2_work::<C, S>(y, x, guard),
            )
            .0
        };
        // Genuinely off-grid: the walker brackets the value.
        assert_ne!(
            walker(RoundingMode::Floor),
            walker(RoundingMode::Ceiling),
            "off-grid Floor != Ceiling S={S} k={big_k} coeff={coeff}"
        );
        for m in [RoundingMode::Floor, RoundingMode::Trunc, RoundingMode::Ceiling] {
            assert_eq!(
                atan2_schoolbook::<C, S>(y, x, m),
                walker(m),
                "off-grid kernel==walker S={S} k={big_k} coeff={coeff} mode={m:?}"
            );
            let r = atan2_schoolbook::<C, S>(y, x, m);
            let d = if r < g { g - r } else { r - g };
            assert!(d <= one, "off-grid single-step S={S} k={big_k} coeff={coeff} mode={m:?}");
        }
    }

    #[cfg(feature = "d462")]
    mod tiny_directed_d462 {
        use super::*;
        use crate::int::types::Int;
        type Core = crate::types::widths::wide_trig_d462::Core;

        // atan2(3e-117, 1) sweeping the LINEAR band (j*=3, S where 3*117=351 > S)
        // into the DEEP band (j*=5, 351 <= S < 585), plus the atan2(1e-38, 1)
        // family (the new d153 break). Every scale, all six modes, single-step.
        #[test]
        fn atan2_tiny_linear_to_deep_sweep() {
            // 3e-117: linear up to s350, deep (j*=5) from s351.
            assert_atan2_tiny_single_step::<Core, 120>(3, 117);
            assert_atan2_tiny_single_step::<Core, 180>(3, 117);
            assert_atan2_tiny_single_step::<Core, 231>(3, 117);
            assert_atan2_tiny_single_step::<Core, 290>(3, 117);
            assert_atan2_tiny_single_step::<Core, 346>(3, 117); // linear (351>346)
            assert_atan2_tiny_single_step::<Core, 400>(3, 117); // deep j*=5
            assert_atan2_tiny_single_step::<Core, 461>(3, 117); // deep j*=5
            // NEGATIVE argument (result < 0): the directed sides mirror.
            assert_atan2_tiny_single_step::<Core, 120>(-3, 117); // linear
            assert_atan2_tiny_single_step::<Core, 461>(-3, 117); // deep j*=5
            // 1e-38 (the new d153 break): coeff=1's z^3/3 = 10^-3k/3 is NON-
            // integer, so the resolved partial is on-grid ONLY in the LINEAR
            // band (z^3 sub-resolution, j*=3); the deep band is off-grid and the
            // walker decides it (not the analytic single step). Test linear only.
            assert_atan2_tiny_single_step::<Core, 76>(1, 38); // linear (114>76)
            assert_atan2_tiny_single_step::<Core, 113>(1, 38); // linear (114>113)
            assert_atan2_tiny_single_step::<Core, 76>(-1, 38); // linear, negative
        }

        // Public path (policy -> rung -> tier) == tier kernel, one linear + one
        // deep cell across all six modes (the rung==tier invariant).
        #[test]
        fn atan2_public_eq_tier() {
            check::<180>(3, 117); // linear
            check::<461>(3, 117); // deep
            fn check<const S: u32>(coeff: i128, big_k: u32) {
                let p = |n: u32| Int::<24>::from_i128(10).pow(n);
                let y = Int::<24>::from_i128(coeff) * p(S - big_k);
                let x = p(S);
                let yd = crate::D::<Int<24>, S>(y);
                let xd = crate::D::<Int<24>, S>(x);
                for mode in MODES {
                    assert_eq!(
                        yd.atan2_strict_with(xd, mode).0,
                        atan2_schoolbook::<Core, S>(y, x, mode),
                        "public==tier S={S} k={big_k} mode={mode:?}"
                    );
                }
            }
        }

        // Non-tiny sanity: atan2(1, 1) = pi/4 (~0.785, NOT tiny). The pre-empt's
        // |G| check no-ops, so the directed modes are the ordinary rounding -
        // ordered and within 1 ULP, never a spurious analytic step.
        #[test]
        fn atan2_nontiny_no_spurious_step() {
            let one_val = Int::<24>::from_i128(10).pow(461);
            let one = Int::<24>::from_i128(1);
            let f = atan2_schoolbook::<Core, 461>(one_val, one_val, RoundingMode::Floor);
            let n = atan2_schoolbook::<Core, 461>(one_val, one_val, RoundingMode::HalfToEven);
            let c = atan2_schoolbook::<Core, 461>(one_val, one_val, RoundingMode::Ceiling);
            assert!(f <= n && n <= c, "non-tiny atan2(1,1) directed ordered");
            assert!(c - f <= one, "non-tiny atan2(1,1) Ceiling-Floor <= 1 ULP (no spurious step)");
            let d = crate::D::<Int<24>, 461>(one_val);
            for mode in MODES {
                assert_eq!(
                    d.atan2_strict_with(d, mode).0,
                    atan2_schoolbook::<Core, 461>(one_val, one_val, mode),
                    "non-tiny public==tier mode={mode:?}"
                );
            }
        }
    }

    #[cfg(feature = "d307")]
    mod tiny_directed_d307 {
        use super::*;
        type Core = crate::types::widths::wide_trig_d307::Core;

        // atan2(3e-117, 1) is LINEAR (j*=3) across D307's whole scale range
        // (351 > every supported scale) - the exact cells the gate flagged
        // (s120, s153, s230, s290, s306). 3e-117 needs scale >= 117 to be
        // representable. All six modes, single-step.
        #[test]
        fn atan2_3e117_linear_sweep() {
            assert_atan2_tiny_single_step::<Core, 120>(3, 117);
            assert_atan2_tiny_single_step::<Core, 153>(3, 117);
            assert_atan2_tiny_single_step::<Core, 230>(3, 117);
            assert_atan2_tiny_single_step::<Core, 290>(3, 117);
            assert_atan2_tiny_single_step::<Core, 306>(3, 117);
        }

        // A DEEP cell on D307 via a less-tiny input: atan2(3e-70, 1) at s290 has
        // j*=5 (3*70=210 resolvable, 5*70=350 sub-resolution); s200 is still
        // linear (210>200) - the linear->deep transition within one tier.
        #[test]
        fn atan2_3e70_linear_to_deep() {
            assert_atan2_tiny_single_step::<Core, 200>(3, 70); // linear j*=3
            assert_atan2_tiny_single_step::<Core, 250>(3, 70); // deep j*=5
            assert_atan2_tiny_single_step::<Core, 290>(3, 70); // deep j*=5
        }
    }

    // j*=7 on-grid cell (z^7/7 still sub-resolution: 7*117=819 > S): the analytic
    // single step (compressing, 7 % 4 == 3) holds, both signs.
    #[cfg(feature = "d616")]
    mod tiny_directed_d616 {
        use super::*;
        type Core = crate::types::widths::wide_trig_d616::Core;

        #[test]
        fn atan2_3e117_j7() {
            assert_atan2_tiny_single_step::<Core, 615>(3, 117); // j*=7 (585<=615<819)
            assert_atan2_tiny_single_step::<Core, 600>(3, 117); // j*=7
            assert_atan2_tiny_single_step::<Core, 615>(-3, 117); // j*=7, negative
        }
    }

    // OFF-GRID deep cells: j* >= 9, the first non-terminating term z^7/7 is
    // RESOLVED (7*117=819 <= S). The directed walker rounds from the reachable
    // z^7/7 residual and the kernel must return THAT (Floor != Ceiling), not an
    // analytic step from G. The proven residual: D924 s923 Ceiling.
    #[cfg(feature = "d924")]
    mod offgrid_d924 {
        use super::*;

        #[test]
        fn atan2_3e117_s923_j9_uses_walker() {
            type Core = crate::types::widths::wide_trig_d924::Core;
            assert_atan2_offgrid_uses_walker::<Core, 923>(3, 117); // j*=9 (the proven cell)
            assert_atan2_offgrid_uses_walker::<Core, 923>(-3, 117); // j*=9, negative
            assert_atan2_offgrid_uses_walker::<Core, 900>(3, 117); // j*=9 (819<=900<1053)

            // Public path == tier kernel across all six modes (rung == tier).
            use crate::int::types::Int;
            let p = |n: u32| Int::<48>::from_i128(10).pow(n);
            let y = Int::<48>::from_i128(3) * p(923 - 117);
            let x = p(923);
            let yd = crate::D::<Int<48>, 923>(y);
            let xd = crate::D::<Int<48>, 923>(x);
            for mode in MODES {
                assert_eq!(
                    yd.atan2_strict_with(xd, mode).0,
                    atan2_schoolbook::<Core, 923>(y, x, mode),
                    "d924 s923 public==tier mode={mode:?}"
                );
            }
        }
    }

    // The deepest off-grid band: j* >= 11 (9*117=1053 <= S). Confirms the kernel
    // keeps deferring to the walker (the analytic z^{j*} sign would now be WRONG
    // - compressing at j*=11 vs the walker's actual z^7/7 verdict).
    #[cfg(feature = "d1232")]
    mod offgrid_d1232 {
        use super::*;
        type Core = crate::types::widths::wide_trig_d1232::Core;

        #[test]
        fn atan2_3e117_deep_j11_uses_walker() {
            assert_atan2_offgrid_uses_walker::<Core, 1231>(3, 117); // j*=11
            assert_atan2_offgrid_uses_walker::<Core, 1231>(-3, 117); // j*=11, negative
            assert_atan2_offgrid_uses_walker::<Core, 1100>(3, 117); // j*=11 (1053<=1100<1287)
        }
    }

    // wide tier: D57, scale 19.
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }
        // scale-9 micro-values in [-1, 1].
        const INPUTS9: [i128; 9] = [
            0,
            250_000_000,
            500_000_000,
            600_000_000,
            900_000_000,
            1_000_000_000,
            -500_000_000,
            -900_000_000,
            -1_000_000_000,
        ];

        #[test]
        fn asin_acos_atan2_schoolbook_match_routed() {
            for &u in &INPUTS9 {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        asin_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).asin_strict_with(mode).0,
                        "D57 asin schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        acos_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).acos_strict_with(mode).0,
                        "D57 acos schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
            // atan2 quadrants at scale 9.
            const PTS: [(i128, i128); 5] = [
                (1_000_000_000, 1_000_000_000),
                (1_000_000_000, -1_000_000_000),
                (-1_000_000_000, 1_000_000_000),
                (-1_000_000_000, -1_000_000_000),
                (500_000_000, 2_000_000_000),
            ];
            for &(y, x) in &PTS {
                let yr = raw9(y);
                let xr = raw9(x);
                for &mode in &MODES {
                    assert_eq!(
                        atan2_schoolbook::<Core, S>(yr, xr, mode),
                        D::<Int<3>, S>(yr).atan2_strict_with(D::<Int<3>, S>(xr), mode).0,
                        "D57 atan2 schoolbook != routed at y={y} x={x} mode={mode:?}"
                    );
                }
            }
        }
    }
}
