// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// Shared `expm1` family leaves (regime classifier, guard lift, reduction
// depth, peak-fit tests). USED BY THE ROUTED PATH — `expm1_series` imports
// it, as do the two unrouted candidates.

//! Width-generic leaves shared by the `expm1` candidate kernels.
//!
//! Family-local support, in the same role `exp_generic` plays for the `exp`
//! family: no algorithm body lives here, only the classifiers and sizing
//! arithmetic every candidate needs, so the four kernels stay free of
//! duplicated policy arithmetic.
//!
//! Every quantity is derived in closed form from `(|v|, w, S::BITS)` — there
//! are no fitted constants and no per-tier arms.

#![allow(dead_code)]

use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;

/// Widest `|x|` at which the leading-term-dropped Taylor series is evaluated
/// without any argument reduction.
///
/// The series alternates for `x < 0`, so its cancellation loss is
/// `max term / |sum|`. At `x = -1` that is `1 / |expm1(-1)| = 1/0.63212`, i.e.
/// **0.66 bits** — below one bit, so the whole band `|x| <= 1` is safe. The
/// classic `|s| <= ln2/2 ~ 0.3466` band produced by a `k*ln 2` reduction is a
/// strict subset. Stated as a closed-form value bound, continuous in every
/// `(N, SCALE)` cell.
pub(crate) const DIRECT_BAND: i128 = 1;

/// `|v|` (0 for zero).
#[inline]
pub(crate) fn abs<S: BigInt>(v: S) -> S {
    if v < S::ZERO {
        -v
    } else {
        v
    }
}

/// Argument-magnitude regime of `expm1(v)` for a working-scale `v_w` at scale
/// `w` in the work integer `S`, decided from the BIT LENGTH alone — before any
/// division, exactly as `exp_generic::ArgRegime` is.
///
/// The bounds are the `exp` classifier's, re-derived for `e^v - 1`; the two
/// functions differ by exactly `1`, which never moves an overflow threshold.
/// What DOES differ is the negative verdict: `exp` underflows toward **zero**,
/// `expm1` saturates toward **minus one**, so the short-circuit value is a
/// different constant (see [`Regime::MinusOne`]).
pub(crate) enum Regime {
    /// Argument small enough for the kernels' own reduction.
    Fits,
    /// `v > 0` and `(e^v - 1) * 10^w` provably exceeds `S`'s capacity.
    Overflow,
    /// `v < 0` and `e^v` is provably below the working resolution, so
    /// `expm1(v) = -1 + e^v` sits strictly between `-1` and `-1 + 10^-w`.
    MinusOne,
}

/// Classifies `v_w` per [`Regime`].
///
/// Derivation (both bounds SUFFICIENT, never fired by a representable cell),
/// with `bl = bit_length(v_w)` and `|v| >= 2^(bl-1) / 10^w`:
///
/// * **Overflow** (`v > 0`): the result needs `e^v * 10^w < 2^BITS`, i.e.
///   `v < BITS*ln2 - w*ln10`. With
///   `R = floor(BITS*6932/10000) + 1 - floor(w*23025/10000) >= BITS*ln2 - w*ln10`
///   the result provably overflows `S` once
///   `bl >= ceil(w*33220/10000) + bits(R) + 2`.
/// * **MinusOne** (`v < 0`): `e^v < 10^-(w+1)` — strictly below the working
///   resolution — once `|v| >= (w+1)*ln10`. With
///   `U = floor((w+1)*23026/10000) + 1 >= (w+1)*ln10` the same bit-length
///   argument gives `bl >= ceil(w*33220/10000) + bits(U) + 2`.
pub(crate) fn regime<S: BigInt>(v_w: S, w: u32) -> Regime {
    if v_w == S::ZERO {
        return Regime::Fits;
    }
    let bl = eg::bit_length::<S>(v_w) as u64;
    // ceil(w * log2 10), over-approximated (33220/10000 >= log2 10).
    let w_bits = ((w as u64) * 33220).div_ceil(10000);
    // bits(x) = floor(log2 x) + 1, so 2^bits(x) >= x.
    let bits_of = |x: u64| 64 - x.leading_zeros() as u64;
    if v_w > S::ZERO {
        let bits_ln2 = (<S as BigInt>::BITS as u64) * 6932 / 10000 + 1;
        let w_ln10 = (w as u64) * 23025 / 10000;
        let r = bits_ln2.saturating_sub(w_ln10).max(1);
        if bl >= w_bits + bits_of(r) + 2 {
            return Regime::Overflow;
        }
    } else {
        let u = ((w as u64) + 1) * 23026 / 10000 + 1;
        if bl >= w_bits + bits_of(u) + 2 {
            return Regime::MinusOne;
        }
    }
    Regime::Fits
}

/// The working-scale value the [`Regime::MinusOne`] band must return:
/// `1 - 10^w`, i.e. `-(10^w - 1)` — ONE WORKING UNIT ABOVE `-1`, never `-10^w`.
///
/// `expm1(v) = -1 + e^v` with `e^v > 0`, so the true value is strictly ABOVE
/// `-1`: its magnitude is strictly BELOW one. The Ziv walkers'
/// `never_exact` rule (`wide_trig_core::near_min_resolve_g`) reads an exactly
/// zero residual as "the true value lies FURTHER from zero than the computed
/// grid line" and bumps the MAGNITUDE — which for a negative result is what
/// `Floor` does. Returning a bare `-10^w` therefore has `Floor` deliver
/// `-1 - 1 ULP`: the wrong side. That value IS representable (the crate caps
/// `MAX_SCALE = N - 1`, so `MAX >= 1` at every legal scale), which makes this a
/// silently wrong result rather than a loud one.
///
/// Returning `-(10^w - 1)` instead leaves a real non-zero residual
/// (`10^guard - 1`), so the walker decides on the true residual: `Floor` gives
/// `-1.0` and `Ceiling` gives `-0.999...9`. Both correct.
///
/// This is the exact structural twin of `try_exp_fixed`'s deep-underflow
/// `Some(1)` (the smallest POSITIVE working value rather than `0`, so directed
/// rounding keeps the sign) — the same rule, reflected about `-1`.
#[inline]
pub(crate) fn just_above_minus_one<S: BigInt>(w: u32) -> S {
    eg::lit::<S>(1) - eg::one::<S>(w)
}

/// `ceil(|v|)` as a `u128`, for a working-scale `v_w` at scale `w`.
///
/// Bounded by [`Regime::Fits`] to order `S::BITS * ln 2`, far inside `u128`.
pub(crate) fn ceil_abs_int<S: BigInt>(v_w: S, w: u32) -> u128
where
    S::Scratch: ComputeLimbs,
{
    let (q, r) = eg::div_rem_exact(abs::<S>(v_w), eg::pow10::<S>(w));
    let n = <S as BigInt>::to_i128(q).unsigned_abs();
    if r == S::ZERO {
        n
    } else {
        n.saturating_add(1)
    }
}

/// Decimal integer-digit count of `e^|v|`, from `ceil(|v|)`:
/// `ceil(|v| * log10 e)` with `log10 e ~ 0.43430` over-approximated.
///
/// This is the guard the reduction error analysis demands: the reassembly
/// multiplies the kernel's absolute error by `2^k = e^|v|`, so the lift must
/// absorb the result's integer-digit count (identical in magnitude to
/// `exp_fixed`'s `ceil(|k| * 0.30103)`, reached without needing `k`).
#[inline]
pub(crate) fn result_int_digits(ceil_abs: u128) -> u32 {
    ceil_abs
        .saturating_mul(43_430)
        .div_ceil(100_000)
        .min(u32::MAX as u128) as u32
}

/// Bit-width of `e^|v|`, from `ceil(|v|)`: `ceil(|v| * log2 e)` with
/// `log2 e ~ 1.44270` over-approximated. The peak models use it where the
/// growth is carried through the recurrence rather than applied once.
#[inline]
pub(crate) fn result_bits(ceil_abs: u128) -> u64 {
    ceil_abs
        .saturating_mul(14_427)
        .div_ceil(10_000)
        .min(u64::MAX as u128) as u64
}

/// Guard-digit lift for a result carrying `int_digits` integer digits — the
/// `exp_fixed` slack shape (`d + 12 + d/4`), capped at `BITS/4` so the lift can
/// never on its own outrun the work integer.
#[inline]
pub(crate) fn extra_digits<S: BigInt>(int_digits: u32) -> u32 {
    if int_digits == 0 {
        return 0;
    }
    let capped = int_digits.min(<S as BigInt>::BITS / 4);
    capped + 12 + (capped >> 2)
}

/// Argument-reduction depth for the halving/doubling core at working scale
/// `w_ext`: the largest `n >= 1` with `(n+1)^2 <= 3*w_ext + 1` (so
/// `n ~ sqrt(3*w_ext)`) — the same balance point `exp_fixed`'s
/// `squaring_levels` strikes between reduction depth and Taylor term count.
#[inline]
pub(crate) fn halving_levels(w_ext: u32) -> u32 {
    let p_bits = w_ext.saturating_mul(3).saturating_add(1);
    let mut n: u32 = 1;
    while (n + 1) * (n + 1) <= p_bits {
        n += 1;
    }
    n
}

/// Extra halvings needed to bring `|v|` into the [`DIRECT_BAND`]:
/// `bits(ceil(|v|))`, which over-estimates `ceil(log2 |v|)` by at most one
/// level (one surplus doubling, never a correctness cost).
#[inline]
pub(crate) fn band_levels(ceil_abs: u128) -> u32 {
    if ceil_abs <= DIRECT_BAND as u128 {
        0
    } else {
        128 - ceil_abs.leading_zeros()
    }
}

/// `w * log2 10`, over-approximated (3322/1000 >= log2 10).
#[inline]
pub(crate) fn scale_bits(w: u32) -> u64 {
    (w as u64) * 3322 / 1000
}

/// Whether a modelled internal peak of `peak` bits fits the work integer `S`
/// without wrapping. The kernels' products are low-half
/// (`wrapping_mul_low_u128`), so an overrun TRUNCATES silently — the model must
/// be an upper bound and is padded by one u64 limb, exactly as
/// `exp_generic::exp_peak_bits_model` pads its own.
#[inline]
pub(crate) fn peak_fits<S: BigInt>(peak: u64) -> bool {
    peak.saturating_add(64) < <S as BigInt>::BITS as u64
}
