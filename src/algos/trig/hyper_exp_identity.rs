// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic narrow-`GUARD` hyperbolic kernels over the `(eˣ, e⁻ˣ)`
//! identity.
//!
//! sinh / cosh / tanh all share the working-scale pair `(eˣ, e⁻ˣ)`. One
//! Tang `exp` call yields `eˣ`, and `e⁻ˣ = 1/eˣ` follows from a single
//! wide divide — versus a second `exp_fixed` call that costs an order of
//! magnitude more:
//!
//! ```text
//! exp_x     = exp(working_value)
//! exp_neg_x = 1 / exp_x                     (the exp(-x) identity)
//! sinh = (exp_x - exp_neg_x) / 2
//! cosh = (exp_x + exp_neg_x) / 2
//! tanh = (exp_x - exp_neg_x) / (exp_x + exp_neg_x)
//! ```
//!
//! ## Layering
//!
//! These are **algorithm functions** (`docs/ARCHITECTURE.md` →
//! "Layering direction"): they compute only through the [`WideTrigCore`]
//! trait surface and the supplied working-scale `exp` kernel; they never
//! call a method on a decimal type. `policy::trig` (the hyperbolic
//! family) calls them *down*.
//!
//! Collapses the four per-tier hyperbolic kernels (D57 18..=22, D115 57,
//! D153 70..=82, D307 140..=160) into one generic over `C: WideTrigCore`,
//! the band's narrow guard `GUARD`, and the band's working-scale `exp`
//! kernel (a function pointer so each band keeps its own exp realisation
//! — the generic Tang `exp_tang::tang_exp_fixed::<C, M, INTERNAL_EXTRA>`
//! surface, shared by every band including D307 140..=160).

use crate::algos::exp::exp_generic as eg;
use crate::algos::exp::exp_tang::tang_exp_fixed_g;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Joint `(eˣ, e⁻ˣ)` pair at the wide composition work width `C::Wagm`
/// (the two-core split). One Tang `exp` call + one reciprocal divide. `M`
/// is the tier's Tang table size, `IE` its `INTERNAL_EXTRA` flag.
#[inline]
fn ex_enx_agm<C: WideTrigCore, const M: u32, const IE: bool>(
    working_value: C::Wagm,
    working_scale: u32,
) -> (C::Wagm, C::Wagm)
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let exp_x = tang_exp_fixed_g::<C::Wagm, M, IE>(working_value, working_scale, |ln2_scale| {
        crate::consts::ln2_by_working_scale::<C::Wagm>(
            ln2_scale,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    });
    let exp_neg_x = eg::div::<C::Wagm>(eg::one::<C::Wagm>(working_scale), exp_x, working_scale);
    (exp_x, exp_neg_x)
}

/// `sinh_strict` for a wide tier — generic over the tier `C`, the band's
/// narrow guard `GUARD`, and the Tang `exp` config `(M, IE)`. Two-core:
/// runs on the wide `C::Wagm`.
#[inline]
#[must_use]
pub(crate) fn sinh_exp_identity_with_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const GUARD: u32,
    const M: u32,
    const IE: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + GUARD;
    let working_value = C::to_work_scaled_agm(raw, GUARD);
    let (exp_x, exp_neg_x) = ex_enx_agm::<C, M, IE>(working_value, working_scale);
    let sinh_value = (exp_x - exp_neg_x) / eg::lit::<C::Wagm>(2);
    // Near-tie escape — see `wide_trig_core::tan_series` / the asin(3e-60)
    // family: a fixed-working-scale single shot cannot see a deciding digit
    // below the working scale. Clear-of-band residuals keep the single-shot
    // cost; the band falls to the Ziv-escalating generic kernel (rare).
    match crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<
        C::Storage,
        C::Wagm,
    >(sinh_value, working_scale, SCALE, mode, C::storage_max(), C::storage_min())
    {
        Some(rounded) => rounded,
        None => crate::algos::trig::hyper_schoolbook::sinh_schoolbook::<C, SCALE>(raw, mode),
    }
}

/// `cosh_strict` for a wide tier — see [`sinh_exp_identity_with_tang`].
/// Two-core: runs on the wide `C::Wagm`.
#[inline]
#[must_use]
pub(crate) fn cosh_exp_identity_with_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const GUARD: u32,
    const M: u32,
    const IE: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: ComputeLimbs,
{
    // Route the narrowing through the shared directed/near-tie Ziv escalation
    // (on the wide `Wagm`), matching `cosh_schoolbook`. `cosh(x) = 1 + x²/2 +
    // x⁴/24 + …`: just above its minimum the deciding `x⁴/24` term sits below
    // the base working scale, so a single narrowing round-to-nearest can misround
    // the half-ULP tie (e.g. `cosh(1e-…)` at the band scales). The escalation
    // confirms the round against a wider guard; non-tie inputs keep the single
    // base narrowing (bit-identical). `cosh(0) = 1` is the only grid-exact point
    // and yields a zero residual, so the directed (not never-exact) narrowing
    // leaves it untouched.
    crate::algos::support::wide_trig_core::round_to_storage_directed_g::<C::Storage, C::Wagm>(
        GUARD,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let working_value = C::to_work_scaled_agm(raw, guard_digits);
            let (exp_x, exp_neg_x) = ex_enx_agm::<C, M, IE>(working_value, working_scale);
            (exp_x + exp_neg_x) / eg::lit::<C::Wagm>(2)
        },
    )
}

/// `tanh_strict` for a wide tier — see [`sinh_exp_identity_with_tang`].
/// Two-core: runs on the wide `C::Wagm`.
///
/// Carries the tiny-argument analytic band: for `tanh(x) = x − x³/3 + …`
/// the cubic sits below one storage ULP yet is strictly positive, so the
/// true value lands just inside the grid line `raw`. No finite-precision
/// exp path resolves the sub-ULP cubic, so the directed result is decided
/// analytically (`tiny_odd_compressing_directed`); nearest modes return
/// `raw`.
#[inline]
#[must_use]
pub(crate) fn tanh_exp_identity_with_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const GUARD: u32,
    const M: u32,
    const IE: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
    <C::Wexp as BigInt>::Scratch: ComputeLimbs,
{
    let zero = C::storage_zero();
    if raw != zero {
        let threshold_exponent = SCALE - SCALE.div_ceil(3);
        let threshold = <C::Storage as BigInt>::TEN.pow(threshold_exponent);
        let abs_raw = if raw < zero { -raw } else { raw };
        if abs_raw <= threshold {
            // `ZeroFiveUp`'s pivot digit; `abs_raw` is already `|raw|`.
            let raw_mod_10 = abs_raw.div_rem(<C::Storage as BigInt>::TEN).1.to_i128() as u8;
            return crate::support::rounding::tiny_odd_compressing_directed(
                raw,
                zero,
                <C::Storage as BigInt>::ONE,
                raw_mod_10,
                mode,
            );
        }
    }
    // General path: outside the tiny band the kernel error is far below
    // half a storage ULP, so a single narrowing is correctly rounded for
    // every mode.
    let working_scale = SCALE + GUARD;
    let working_value = C::to_work_scaled_agm(raw, GUARD);
    let (exp_x, exp_neg_x) = ex_enx_agm::<C, M, IE>(working_value, working_scale);
    let tanh_value = eg::div::<C::Wagm>(exp_x - exp_neg_x, exp_x + exp_neg_x, working_scale);
    // Near-tie escape — see `wide_trig_core::tan_series` / the asin(3e-60)
    // family: a fixed-working-scale single shot cannot see a deciding digit
    // below the working scale. Clear-of-band residuals keep the single-shot
    // cost; the band falls to the Ziv-escalating generic kernel (rare).
    match crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<
        C::Storage,
        C::Wagm,
    >(tanh_value, working_scale, SCALE, mode, C::storage_max(), C::storage_min())
    {
        Some(rounded) => rounded,
        None => crate::algos::trig::hyper_schoolbook::tanh_schoolbook::<C, SCALE>(raw, mode),
    }
}
