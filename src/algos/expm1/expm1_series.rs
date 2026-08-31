// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: direct leading-term-dropped Taylor, no argument reduction, not wired

//! `expm1` by the bare leading-term-dropped Taylor series.
//!
//! `expm1(x) = x + x^2/2! + x^3/3! + ...` evaluated directly at the working
//! scale, with NO argument reduction, NO `ln 2` constant, NO division to find a
//! range-reduction quotient, and NO reassembly. The whole kernel is the series
//! loop already in `exp_generic::expm1_fixed`; this file supplies the domain
//! gate, the regime classifier and the peak-fit wall around it.
//!
//! # Why this is the accurate kernel near zero
//!
//! Correct rounding needs absolute accuracy `1/2 * 10^-SCALE`, i.e.
//! `1/2 * 10^guard` WORKING units — a bound that does not shrink as the result
//! shrinks. Routing through `exp` instead costs `n = sqrt(3w)` repeated
//! squarings, each of which doubles the accumulated error, giving `~1.5 * 2^n`
//! units of noise; this series accumulates only `~T` units for `T` terms. The
//! guard requirement is therefore
//!
//! * via `exp`:   `guard > 0.301 * sqrt(3w) + 0.5`  (`~9` digits at `w = 300`)
//! * this kernel: `guard >~ 3`
//!
//! — six to eight digits of extra headroom at every scale, which is headroom the
//! Ziv walker would otherwise spend escalating.
//!
//! # Validity
//!
//! * `|x| <= DIRECT_BAND` (= 1). The series alternates for `x < 0`, so the
//!   cancellation loss is `max term / |sum|`; at `x = -1` that is
//!   `1/0.63212`, i.e. 0.66 bits. Past the band the loss grows like `e^|x|`.
//! * `2*w*log2(10) + 2 <= BITS - 2` — the `term * s` product spans `2w` digits
//!   before its `/10^w`, and `mul` returns the low bits.
//!
//! Reference: J.-M. Muller, *Elementary Functions* 3rd ed. (2016), 4.4;
//! N. J. Higham, *Accuracy and Stability of Numerical Algorithms* 2nd ed.
//! (2002), 1.14.1.

use super::expm1_support as sup;
use crate::algos::exp::exp_generic as eg;
use crate::algos::support::wide_trig_core as wtc;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `expm1(v)` for a working-scale value `v_w` at scale `w`, by the direct
/// series.
///
/// `None` means the value cannot be produced in `S` at this `w`: either the
/// argument is outside the series' band (the caller must route to a reducing
/// candidate) or the internal product would wrap the work integer. The
/// `Option` is the `try_exp_fixed` contract — detect once here, let the policy
/// wrapper apply the overflow policy.
pub(crate) fn expm1_series_fixed<S: BigInt>(v_w: S, w: u32) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    expm1_series_fixed_tagged::<S>(v_w, w).0
}

/// [`expm1_series_fixed`] with the sign of the series tail alongside it —
/// the input [`wtc::round_to_storage_tail_signed_g`] needs to settle a
/// residual that reads as an exact zero or an exact tie.
///
/// The value is bit-identical to [`expm1_series_fixed`]'s; that function is
/// this one with the tag dropped, so there is one body and one series loop.
///
/// The tag is `None` for both short-circuit verdicts, because neither is a
/// series sum with a tail: `expm1(0) = 0` is the one exact point, and the
/// deep-negative [`sup::just_above_minus_one`] representative is a
/// construction whose side is already chosen deliberately rather than a
/// computed remainder. `None` makes the walker behave exactly as before, so
/// both keep the treatment they had.
pub(crate) fn expm1_series_fixed_tagged<S: BigInt>(
    v_w: S,
    w: u32,
) -> (Option<S>, Option<eg::TailSign>)
where
    S::Scratch: ComputeLimbs,
{
    if v_w == S::ZERO {
        // expm1(0) = 0 exactly — the ONLY exact case (for algebraic x != 0,
        // e^x is transcendental by Lindemann-Weierstrass, so e^x - 1 never
        // lands on a storage grid line).
        return (Some(S::ZERO), None);
    }
    match sup::regime::<S>(v_w, w) {
        sup::Regime::Overflow => return (None, None),
        sup::Regime::MinusOne => {
            return (Some(sup::just_above_minus_one::<S>(w)), None);
        }
        sup::Regime::Fits => {}
    }
    // Band gate: |v| <= DIRECT_BAND. Tested on the working-scale integer so no
    // division is needed on the hot path.
    if sup::abs::<S>(v_w) > eg::lit::<S>(sup::DIRECT_BAND) * eg::one::<S>(w) {
        return (None, None);
    }
    // Peak: |term| <= 10^w and |s| <= 10^w, so the `term * s` product before
    // the `/10^w` spans at most `2w` digits.
    if !sup::peak_fits::<S>(2 * sup::scale_bits(w)) {
        return (None, None);
    }
    let (v, tail) = eg::expm1_fixed_tagged::<S>(v_w, w);
    (Some(v), tail)
}

/// `expm1(x)` at storage `St`, computed in the work integer `S` and correctly
/// rounded to `SCALE` under `mode`.
///
/// The storage-facing shell around [`expm1_series_fixed_tagged`]: lift to the
/// working scale, then run the Ziv escalation with the series' own tail sign
/// threaded in. `policy::expm1` supplies the width's work integer, base guard
/// and storage bounds.
///
/// # Why the tail sign is threaded rather than a fixed polarity asserted
///
/// The `never_exact = true` walker asserts that an exactly-zero working
/// residual means the TRUE magnitude is larger. That holds for `exp`/`cosh`
/// only because they are strictly positive, and it cannot be adapted here: the
/// correct direction varies with the ARGUMENT, not the function, and both
/// directions occur within a single `(width, scale)` cell — at `D1232<1231>`,
/// `x = -3e-240` needs one and `x = -1e-306` the other, because the first
/// surviving term of the tail is the 6th in one case and the 5th in the other.
/// So the sign is computed per call by the kernel that summed the series and
/// handed to [`wtc::round_to_storage_tail_signed_g`], which is the only place
/// that can act on it.
///
/// This subsumes the old near-zero post-adjust, which tested `result == raw`
/// and so reached only the ONE grid point where the value lands on its own
/// linear term. Deeper partial sums land on the grid too whenever the
/// argument's coefficients make `x^j/j!` terminate — `x = -3e-152` reaches the
/// 3rd and `x = -3e-86` the 5th — and no fixed number of such tests covers
/// them, because the run of exactly-representable terms is unbounded for a
/// suitably composite coefficient. The tail sign is exact at every depth, so
/// depth stops mattering.
///
/// The deep-negative `1 - 10^w` representative keeps its own treatment: it is
/// a construction, not a series sum, so the kernel tags it `None` and the
/// walker behaves exactly as it did before.
///
/// # Panics
///
/// Panics if the result leaves the storage range. Within the band
/// `policy::expm1` routes here (`|x| <= 1`) the kernel's own `None` verdicts
/// are unreachable: both regime walls need a large `|x|`, the band gate is the
/// routing condition itself, and the peak wall needs
/// `2·w·log2(10) + 64 >= S::BITS`, which the walker's own probe cap
/// (`w <= S::BITS/8`) precludes for every work integer wider than ~378 bits —
/// every one this crate uses is at least 1024.
#[inline]
#[must_use]
pub(crate) fn expm1_series_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
    raw: St,
    base_guard: u32,
    st_max: St,
    st_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    wtc::round_to_storage_tail_signed_g::<St, S>(
        base_guard,
        SCALE,
        mode,
        st_max,
        st_min,
        |guard| {
            let (v, tail) = expm1_series_fixed_tagged::<S>(
                wtc::to_work_scaled_g::<St, S>(raw, guard),
                SCALE + guard,
            );
            (super::checked(v, "expm1_strict", SCALE), tail)
        },
    )
}

/// The `_approx` sibling of [`expm1_series_g`]: a SINGLE shot at the caller's
/// `working_digits`, no Ziv escalation — the same precision/latency trade every
/// other `*_approx` transcendental makes.
///
/// # Panics
///
/// Panics if the result leaves the storage range, or if `working_digits` is so
/// large that the series' `2·w`-digit product outruns the work integer (the
/// escalation cap that makes this unreachable in [`expm1_series_g`] does not
/// bound a caller-chosen guard).
#[inline]
#[must_use]
pub(crate) fn expm1_series_approx_g<St: BigInt + Copy, S: BigInt, const SCALE: u32>(
    raw: St,
    working_digits: u32,
    st_max: St,
    st_min: St,
    mode: RoundingMode,
) -> St
where
    S::Scratch: ComputeLimbs,
{
    let w = SCALE + working_digits;
    let r = super::checked(
        expm1_series_fixed::<S>(wtc::to_work_scaled_g::<St, S>(raw, working_digits), w),
        "expm1_approx",
        SCALE,
    );
    let out = wtc::round_to_storage_with_g::<St, S>(r, w, SCALE, mode, st_max, st_min);
    super::adjust_near_zero::<St>(out, raw, mode)
}

/// Tier-generic entry to [`expm1_series_g`] — sources the work integer `C::W`,
/// the base guard `C::GUARD` and the storage bounds from the wide tier's
/// `Core`, exactly as `log1p_artanh` does. Saves the policy from repeating five
/// arguments per wide arm.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn expm1_series<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    expm1_series_g::<C::Storage, C::W, SCALE>(
        raw,
        C::GUARD,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}

/// Tier-generic entry to [`expm1_series_approx_g`]. See [`expm1_series`].
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) fn expm1_series_approx<C: wtc::WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    working_digits: u32,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    expm1_series_approx_g::<C::Storage, C::W, SCALE>(
        raw,
        working_digits,
        C::storage_max(),
        C::storage_min(),
        mode,
    )
}
