// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Joint `(sin, cos)` over ONE shared Taylor evaluation.
//!
//! The pair kernel behind `sin_cos` / `sin_cos_with`.
//! `C::sin_cos_fixed` evaluates only `|sin|` and recovers
//! `|cos| = √(1 − sin²)` from the Pythagorean identity, so the joint
//! result costs about one `sin` plus one wide `sqrt` — roughly half of
//! two independent evaluations.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` → "Layering
//! direction"): it computes only through the [`WideTrigCore`] trait
//! surface and never calls a method on a decimal type. The near-tie
//! escapes are taken as plain function arguments rather than named here —
//! the SAME function-pointer parameterisation
//! [`super::hyper_exp_identity`] uses for its band `exp` kernel — so the
//! choice of what a near-tie falls back to stays a ROUTING decision owned
//! by `policy::trig::sin_cos_dispatch`, and this file stays one
//! algorithm.
//!
//! Generic over the tier core `C`, so one kernel serves every wide width;
//! there is no scale precondition, hence no band suffix on the name.

use crate::algos::support::wide_trig_core::{round_to_storage_clear_of_tie_g, WideTrigCore};
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Joint sine and cosine of `raw` (radians), as `(sin, cos)`, from one
/// shared Taylor evaluation at the tier's fixed working scale
/// `SCALE + C::GUARD`.
///
/// Each component is narrowed independently, and each takes the near-tie
/// escape on its own: a single shot at a FIXED working scale cannot see a
/// deciding digit below that scale (the `asin(3e-60)` family), so a
/// residual inside the tie band falls to `sin_escape` / `cos_escape` —
/// the Ziv-escalating single-function path — while a clear-of-band
/// residual keeps the joint kernel's cost. Because the two components
/// escape separately, one may take the joint value while the other
/// escalates.
///
/// `sin_escape` / `cos_escape` are the single-function fallbacks, passed
/// in rather than named: `policy::trig::sin_cos_dispatch` supplies the
/// matcher's own `sin` / `cos` verdicts for the cell, so the escape
/// always lands on whichever engine that cell routes to and this kernel
/// never pins one.
#[inline]
#[must_use]
pub(crate) fn sin_cos_shared_taylor<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
    sin_escape: fn(C::Storage, RoundingMode) -> C::Storage,
    cos_escape: fn(C::Storage, RoundingMode) -> C::Storage,
) -> (C::Storage, C::Storage)
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    // One shared kernel evaluation; each component takes the near-tie
    // escape (a deciding digit can sit below the fixed w - the
    // asin(3e-60) family), falling to the Ziv-escalated single-function
    // path when inside the band.
    let working_scale = SCALE + C::GUARD;
    let (sin_w, cos_w) = C::sin_cos_fixed::<SCALE>(C::to_work(raw), working_scale);
    let sin_bits = match round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        sin_w,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    ) {
        Some(narrowed) => narrowed,
        None => sin_escape(raw, mode),
    };
    let cos_bits = match round_to_storage_clear_of_tie_g::<C::Storage, C::W>(
        cos_w,
        working_scale,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
    ) {
        Some(narrowed) => narrowed,
        None => cos_escape(raw, mode),
    };
    (sin_bits, cos_bits)
}
