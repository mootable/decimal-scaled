// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// ROUTED: `Algorithm::WithExp` — `exp_fixed(v, w) - 10^w` formed at the
// working scale. Selected for `|x| > 1` at every cell.

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
//! The gain is `ln(1 + MAX) - ln(MAX) = ln(1 + 1/MAX)` — precisely the
//! arguments whose `e^x` lands in `(MAX, MAX + 1]`. Doing the `- 10^w` HERE,
//! before `round_to_storage_*`, is what makes that band reachable.
//!
//! **Size it honestly.** The crate caps `MAX_SCALE = N - 1` for `D{N}`
//! (rejected at compile time above that), so every legal scale keeps at least
//! one integer digit and `MAX >= 1` ALWAYS. At each tier's maximum scale
//! `MAX = Storage::MAX / 10^MAX_SCALE` runs from about 17 (D38 at scale 37) to
//! about 92 (D18 at scale 17), so the extra band is only
//! `ln(1 + 1/MAX) ~ 0.011 .. 0.057` wide in `x`, and it narrows further at
//! lower scales. It is a genuine capability `exp` does not have — there ARE
//! arguments this answers and `exp` panics on — but it is a narrow strip
//! at the top of the range, NOT a whole half-domain.
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

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::wide_trig_core as wtc;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `expm1(v)` for a `working_value` at `working_scale`, as
/// `exp_fixed(working_value, working_scale) - 10^w`. `None` propagates
/// `try_exp_fixed`'s out-of-range verdict (the "detect once, the wrapper
/// applies the policy" contract).
pub(crate) fn expm1_with_exp_fixed<S: BigInt>(working_value: S, working_scale: u32) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    if working_value == S::ZERO {
        // expm1(0) = 0 exactly — the only exact case.
        return Some(S::ZERO);
    }
    // Deep-negative arguments arrive here as `try_exp_fixed`'s
    // `Some(1)` (the smallest POSITIVE working value, which it returns rather
    // than 0 so directed rounding keeps the sign). Subtracting `10^w` turns
    // that into `1 - 10^w` — precisely the "one working unit above -1"
    // representative the negative tail requires
    // (`expm1_generic::just_above_minus_one`), so the deep band is correct here
    // for free.
    let exp_value = eg::try_exp_fixed::<S>(working_value, working_scale)?;
    Some(exp_value - eg::one::<S>(working_scale))
}

/// `expm1(x)` at storage `St`, computed in the work integer `S` and correctly
/// rounded to `SCALE` under `mode`. The storage-facing shell around
/// [`expm1_with_exp_fixed`]; see
/// [`expm1_series_g`](super::expm1_series::expm1_series_g) for the shared shape
/// and for why the walker's `never_exact` polarity is `false`.
///
/// # Which work integer to pass
///
/// `policy::expm1` routes this arm the LARGE-argument regime (`|x| > 1`), which
/// is exactly where `e^x`'s internal squaring / `2^k`-reassembly peak grows, so
/// it passes the tier's WIDEST work integer (`C::Wexp`; `WZiv` on the narrow
/// tiers, already the widest they have) rather than `C::W`. `exp_series` reaches
/// the same width by lifting to `C::Wexp` when the peak outgrows the primary —
/// running there directly gives `expm1` at least the reach `exp`
/// has, so it cannot signal out-of-range on an argument `exp` accepts. Choosing
/// the wider integer up front costs speed on the easy cells; that is a
/// deliberate validity-first call, and the cost crossover is un-benched.
///
/// # Panics
///
/// Panics if the result leaves the storage range, or if the argument is so
/// large that even `C::Wexp` cannot host `e^x` — the same wall `exp`
/// carries, reached through `try_exp_fixed`'s `None`.
#[inline]
#[must_use]
pub(crate) fn expm1_with_exp_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
    raw: St,
    base_guard_digits: u32,
    storage_max: St,
    storage_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    let rounded = wtc::round_to_storage_directed_g::<St, S>(
        base_guard_digits,
        SCALE,
        mode,
        storage_max,
        storage_min,
        |guard_digits| {
            super::checked(
                expm1_with_exp_fixed::<S>(
                    wtc::to_work_scaled_g::<St, S>(raw, guard_digits),
                    SCALE + guard_digits),
                "expm1",
                SCALE,
            )
        },
    );
    super::adjust_near_zero::<St>(rounded, raw, mode)
}

/// Tier-generic entry to [`expm1_with_exp_g`] at the tier's widest work integer
/// `C::Wexp` — see [`expm1_with_exp_g`] for why that width and not `C::W`.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn expm1_with_exp<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::Wexp as BigInt>::Scratch: ComputeLimbs,
{
    expm1_with_exp_g::<C::Storage, C::Wexp, SCALE>(
        raw,
        C::GUARD,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}

