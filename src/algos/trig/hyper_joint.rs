// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Joint `(sinh, cosh)` over ONE shared `exp` evaluation.
//!
//! The pair kernel behind `sinh_cosh` / `sinh_cosh_with`.
//! One `exp(x)` plus the `e⁻ˣ = 1/eˣ` reciprocal gives both terms of the
//! identity, so the pair costs one `exp` and one wide divide rather than
//! two `exp`s — worth roughly 40% at the wide tiers, where `exp_fixed` is
//! 10-20x a divide.
//!
//! ```text
//! exp_x     = exp(working_value)
//! exp_neg_x = 1 / exp_x                     (the exp(-x) identity)
//! sinh = (exp_x - exp_neg_x) / 2
//! cosh = (exp_x + exp_neg_x) / 2
//! ```
//!
//! The `exp` here is the SERIES engine on the composition integer
//! (`C::exp_fixed_series_agm`), NOT the Tang-routed one that the
//! per-band single-function kernels in [`super::hyper_exp_identity`] use;
//! that is why the two live in separate files rather than as arms of one.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` → "Layering
//! direction"): it computes only through the [`WideTrigCore`] trait
//! surface and never calls a method on a decimal type. The near-tie
//! escapes are taken as plain function arguments rather than named here —
//! the same function-pointer parameterisation
//! [`super::hyper_exp_identity`] uses for its band `exp` kernel — so the
//! choice of what a near-tie falls back to stays a ROUTING decision owned
//! by `policy::trig::sinh_cosh_dispatch`.

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::wide_trig_core::{round_to_storage_clear_of_tie_g, WideTrigCore};
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Joint hyperbolic sine and cosine of `raw`, as `(sinh, cosh)`, from one
/// shared `exp` evaluation at the tier's fixed working scale
/// `SCALE + C::GUARD`. Two-core: the composition runs on the wide
/// `C::Wagm` work int.
///
/// Each component is narrowed independently and takes the near-tie escape
/// on its own: `sinh(x) = x + x³/6 + …` lands exact rational partials on
/// rounding boundaries, and a single shot at a FIXED working scale cannot
/// see a deciding digit below that scale, so a residual inside the tie
/// band falls to `sinh_escape` / `cosh_escape` — the analytically-pinned
/// / Ziv-escalated single-function path — while a clear-of-band residual
/// keeps the joint kernel's cost.
///
/// `sinh_escape` / `cosh_escape` are passed in rather than named:
/// `policy::trig::sinh_cosh_dispatch` supplies the matcher's own `sinh` /
/// `cosh` verdicts for the cell, so an escape always lands on whichever
/// engine that cell routes to and this kernel never pins one.
#[inline]
#[must_use]
pub(crate) fn sinh_cosh_exp_reciprocal<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    sinh_escape: fn(C::Storage, RoundingMode) -> C::Storage,
    cosh_escape: fn(C::Storage, RoundingMode) -> C::Storage,
) -> (C::Storage, C::Storage)
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    // One shared exp evaluation; each component takes the near-tie escape
    // (sinh(x) = x + x^3/6 + ... lands exact rational partials on
    // rounding boundaries), falling to the analytically-pinned /
    // Ziv-escalated single-function path when inside the band.
    let working_scale = SCALE + C::GUARD;
    // Two-core: composition runs on the wide `Wagm` work int.
    let working_value = C::to_work_scaled_agm(raw, C::GUARD);
    let exp_x = C::exp_fixed_series_agm(working_value, working_scale);
    let exp_neg_x = eg::div::<C::Wagm>(
        eg::pow10::<C::Wagm>(working_scale),
        exp_x,
        working_scale,
    );
    let sinh_value = (exp_x - exp_neg_x) >> 1;
    let cosh_value = (exp_x + exp_neg_x) >> 1;
    let sinh_bits = match round_to_storage_clear_of_tie_g::<C::Storage, C::Wagm>(
        sinh_value,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    ) {
        Some(narrowed) => narrowed,
        None => sinh_escape(raw, mode),
    };
    let cosh_bits = match round_to_storage_clear_of_tie_g::<C::Storage, C::Wagm>(
        cosh_value,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    ) {
        Some(narrowed) => narrowed,
        None => cosh_escape(raw, mode),
    };
    (sinh_bits, cosh_bits)
}
