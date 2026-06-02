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
//! ex  = exp(v)
//! enx = 1 / ex              (exp(-v) identity)
//! sinh = (ex - enx) / 2
//! cosh = (ex + enx) / 2
//! tanh = (ex - enx) / (ex + enx)
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
/// is the tier's Tang table size, `IE` its `INTERNAL_EXTRA` flag — the same
/// pair the per-tier `policy::trig` arm previously baked into the passed
/// `tang_exp_fixed` fn pointer.
#[inline]
fn ex_enx_agm<C: WideTrigCore, const M: u32, const IE: bool>(
    v: C::Wagm,
    w: u32,
) -> (C::Wagm, C::Wagm)
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let ex = tang_exp_fixed_g::<C::Wagm, M, IE>(v, w, |ww| {
        crate::consts::ln2_by_working_scale::<C::Wagm>(
            ww,
            crate::support::rounding::DEFAULT_ROUNDING_MODE,
        )
    });
    let enx = eg::div::<C::Wagm>(eg::one::<C::Wagm>(w), ex, w);
    (ex, enx)
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
{
    let w = SCALE + GUARD;
    let v = C::to_work_scaled_agm(raw, GUARD);
    let (ex, enx) = ex_enx_agm::<C, M, IE>(v, w);
    let r = (ex - enx) / eg::lit::<C::Wagm>(2);
    C::round_to_storage_with_agm(r, w, SCALE, mode)
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
{
    let w = SCALE + GUARD;
    let v = C::to_work_scaled_agm(raw, GUARD);
    let (ex, enx) = ex_enx_agm::<C, M, IE>(v, w);
    let r = (ex + enx) / eg::lit::<C::Wagm>(2);
    C::round_to_storage_with_agm(r, w, SCALE, mode)
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
{
    let zero = C::storage_zero();
    if raw != zero {
        let thresh_exp = SCALE - SCALE.div_ceil(3);
        let thresh = <C::Storage as BigInt>::TEN.pow(thresh_exp);
        let abs_raw = if raw < zero { -raw } else { raw };
        if abs_raw <= thresh {
            return crate::support::rounding::tiny_odd_compressing_directed(
                raw,
                zero,
                <C::Storage as BigInt>::ONE,
                mode,
            );
        }
    }
    // General path: outside the tiny band the kernel error is far below
    // half a storage ULP, so a single narrowing is correctly rounded for
    // every mode.
    let w = SCALE + GUARD;
    let v = C::to_work_scaled_agm(raw, GUARD);
    let (ex, enx) = ex_enx_agm::<C, M, IE>(v, w);
    let r = eg::div::<C::Wagm>(ex - enx, ex + enx, w);
    C::round_to_storage_with_agm(r, w, SCALE, mode)
}
