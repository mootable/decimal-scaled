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
//! — the Tang `tang_exp_fixed::<C, M, INTERNAL_EXTRA>` surface or, at
//! D307, the retained per-tier `exp_tang_16limb_s140_160::tang_exp_fixed`).

use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Joint `(eˣ, e⁻ˣ)` pair at working scale `w`. One `exp` call + one
/// reciprocal divide.
#[inline]
fn ex_enx<C: WideTrigCore>(exp_fixed: fn(C::W, u32) -> C::W, v: C::W, w: u32) -> (C::W, C::W) {
    let ex = exp_fixed(v, w);
    let enx = C::div(C::one(w), ex, w);
    (ex, enx)
}

/// `sinh_strict` for a wide tier — generic over the tier `C`, the band's
/// narrow guard `GUARD`, and its working-scale `exp` kernel.
#[inline]
#[must_use]
pub(crate) fn sinh_exp_identity_with_tang<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    exp_fixed: fn(C::W, u32) -> C::W,
) -> C::Storage {
    let w = SCALE + GUARD;
    let v = C::to_work_w(raw, GUARD);
    let (ex, enx) = ex_enx::<C>(exp_fixed, v, w);
    let r = (ex - enx) / C::lit(2);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// `cosh_strict` for a wide tier — generic over the tier `C`, the band's
/// narrow guard `GUARD`, and its working-scale `exp` kernel.
#[inline]
#[must_use]
pub(crate) fn cosh_exp_identity_with_tang<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    exp_fixed: fn(C::W, u32) -> C::W,
) -> C::Storage {
    let w = SCALE + GUARD;
    let v = C::to_work_w(raw, GUARD);
    let (ex, enx) = ex_enx::<C>(exp_fixed, v, w);
    let r = (ex + enx) / C::lit(2);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// `tanh_strict` for a wide tier — generic over the tier `C`, the band's
/// narrow guard `GUARD`, and its working-scale `exp` kernel.
///
/// Carries the tiny-argument analytic band: for `tanh(x) = x − x³/3 + …`
/// the cubic sits below one storage ULP yet is strictly positive, so the
/// true value lands just inside the grid line `raw`. No finite-precision
/// exp path resolves the sub-ULP cubic, so the directed result is decided
/// analytically (`tiny_odd_compressing_directed`); nearest modes return
/// `raw`.
#[inline]
#[must_use]
pub(crate) fn tanh_exp_identity_with_tang<C: WideTrigCore, const SCALE: u32, const GUARD: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    exp_fixed: fn(C::W, u32) -> C::W,
) -> C::Storage {
    let zero = C::storage_zero();
    if raw != zero {
        let thresh_exp = SCALE - (SCALE + 2) / 3;
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
    let v = C::to_work_w(raw, GUARD);
    let (ex, enx) = ex_enx::<C>(exp_fixed, v, w);
    let r = C::div(ex - enx, ex + enx, w);
    C::round_to_storage_with(r, w, SCALE, mode)
}
