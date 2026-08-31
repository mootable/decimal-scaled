// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: reference baseline, exp_fixed(v, w) - 10^w at working scale, not wired

//! `expm1` as `e^v - 1` evaluated at the WORKING scale.
//!
//! The reference baseline every other candidate is measured against, and the
//! widest-domain fallback: it inherits `exp_fixed`'s proven range reduction,
//! guard lift, peak model and out-of-range contract unchanged.
//!
//! # The subtraction is exact — but WHERE it happens is the whole point
//!
//! This crate has no exponent. `1` is exactly `10^w` raw units and the grid is
//! `{ n*10^-SCALE }`, so subtracting one is an exact integer subtraction that
//! maps grid to grid. Rounding commutes with an exact grid translation, hence
//!
//! ```text
//! round_SCALE(e^x) - 1  ==  round_SCALE(e^x - 1)
//! ```
//!
//! whenever both sides are representable: the classic floating-point
//! cancellation argument for a dedicated `expm1` does NOT transfer.
//!
//! What does NOT commute is the RANGE CHECK. Subtracting at working scale, ahead
//! of the storage narrowing, is what buys the domain:
//!
//! | | `exp` | `expm1` |
//! |---|---|---|
//! | result for `x < 0` | `(0, 1)` | `(-1, 0)` |
//! | representable upper arg | `x <= ln(MAX)` | `x <= ln(1 + MAX)` |
//!
//! With `MAX = (10^D - 1)/10^SCALE` on a `D`-digit tier, a MAX-SCALE type
//! (`SCALE = D`, e.g. `D18<18>`) has `MAX < 1`, so `ln(MAX) < 0` and `exp`
//! overflows for EVERY `x >= 0` — while `ln(1 + MAX) ~ ln 2`, so `expm1` covers
//! the whole band `0 < x < 0.693`. Doing the `- 10^w` here, before
//! `round_to_storage_*`, is what makes that band reachable.
//!
//! # Where this candidate is weaker
//!
//! Correct rounding needs `1/2 * 10^guard` WORKING units of accuracy. This route
//! carries `exp_fixed`'s `n = squaring_levels(w) ~ sqrt(3w)` squarings, each of
//! which doubles the accumulated error, so its noise is `~1.5 * 2^n` units and
//! the exact `- 10^w` preserves it. The requirement is
//!
//! ```text
//! guard > 0.301*sqrt(3*w) + 0.5      (~9 digits at w = 300, ~17 at w = 1000)
//! ```
//!
//! whereas the direct series (`expm1_series`) accumulates only a few hundred
//! units and needs `guard >~ 3`. Tiers running the narrow band guard (8) are
//! already inside that margin at high scale, so this candidate is expected to
//! escalate where the direct kernels do not.
//!
//! # Validity
//!
//! Exactly `try_exp_fixed`'s wall, plus the guard condition above.

#![allow(dead_code)]

use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;

/// `expm1(v)` for a working-scale value `v_w` at scale `w`, as
/// `exp_fixed(v_w, w) - 10^w`. `None` propagates `try_exp_fixed`'s
/// out-of-range verdict (the "detect once, the wrapper applies the policy"
/// contract).
pub(crate) fn expm1_via_exp_fixed<S: BigInt>(v_w: S, w: u32) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    if v_w == S::ZERO {
        // expm1(0) = 0 exactly — the only exact case.
        return Some(S::ZERO);
    }
    // Deep-negative arguments arrive here as `try_exp_fixed`'s
    // `Some(1)` (the smallest POSITIVE working value, which it returns rather
    // than 0 so directed rounding keeps the sign). Subtracting `10^w` turns
    // that into `1 - 10^w` — precisely the "one working unit above -1"
    // representative the negative tail requires
    // (`expm1_support::just_above_minus_one`), so the deep band is correct here
    // for free.
    let e = eg::try_exp_fixed::<S>(v_w, w)?;
    Some(e - eg::one::<S>(w))
}
