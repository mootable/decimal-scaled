// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `div_widen_scale` — decimal division by the widen-then-divide method,
//! generic over the storage width `N` only.
//!
//! Divides `dividend / divisor` for two same-`SCALE` decimals stored as
//! `Int<N>`. The logical quotient is
//! `(dividend / 10^SCALE) / (divisor / 10^SCALE) = dividend / divisor`, but to
//! keep `SCALE` fractional digits the numerator is first scaled up by
//! `10^SCALE` (`dividend * 10^SCALE`). Scaling can overflow `Int<N>`, so the
//! scaled numerator spans up to `2N` limbs and is formed in a limb
//! **scratch buffer** rather than a work *type* `Int<2N>`.
//!
//! # Generic over the storage width only — no `Int<2N>` work type
//!
//! Following the `sqrt`/`cbrt`/`hypot` template, the kernel is generic over
//! `N` alone:
//!
//! 1. form `|dividend| * 10^SCALE` (`2N` u64 limbs) in a [`ComputeLimbs::double_buffered_u64`]
//!    buffer via the int slice multiply;
//! 2. divide it by `|divisor|` via the int layer's width-agnostic divide
//!    ([`crate::int::algos::div::div_fixed::div_rem_mag_slice`], which
//!    fronts the divisor-shape policy — Knuth / single-limb fast paths),
//!    rounding under `mode`;
//! 3. rebuild the signed `Int<N>` quotient (panics on overflow in both
//!    debug and release).
//!
//! The divisor here is the runtime operand, not `10^SCALE`, so the MG
//! magic-divide does not apply — the int-layer `div_rem` (with its own
//! hardware fast paths) is the right engine, exactly as the prior
//! `Int<W>::div_rem` path used.
//!
//! All integer arithmetic dispatches DOWN to the int layer; this fn never
//! calls a decimal method on its own value.

use crate::int::algos::div::div_knuth::div_knuth_into;
use crate::int::algos::div::div_knuth_u128_limb::div_knuth_u128_limb_into;
use crate::int::policy::mul::dispatch_slice as mul_slice;
use crate::int::policy::div_rem::{select_for_limbs, Algorithm};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::{should_bump, RoundingMode};

/// Significant limb length (highest non-zero limb index + 1, min 1).
#[inline]
fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 1 && limbs[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Compare `2*remainder` against `divisor` (little-endian magnitudes),
/// returning the ordering of `remainder` vs `divisor - remainder` (the
/// rounding half-comparison).
#[inline]
fn cmp_double_vs<const N: usize>(remainder: &[u64], divisor: &[u64]) -> core::cmp::Ordering
where
    Limbs<N>: ComputeLimbs,
{
    // `2·remainder` spans at most `remainder.len() + 1` limbs, and
    // `remainder < divisor`, whose
    // length is `≤ N`; the `single_buffered_u64` buffer (`N + 2`) holds it
    // exactly per-`N`.
    let mut double_remainder_buf = Limbs::<N>::single_buffered_u64();
    let double_remainder = double_remainder_buf.as_mut();
    let mut carry: u64 = 0;
    for (i, &limb) in remainder.iter().enumerate() {
        let doubled = ((limb as u128) << 1) | carry as u128;
        double_remainder[i] = doubled as u64;
        carry = (doubled >> 64) as u64;
    }
    let mut len = remainder.len();
    if carry != 0 {
        double_remainder[len] = carry;
        len += 1;
    }
    let divisor_len = divisor.len();
    let max_len = len.max(divisor_len);
    let mut idx = max_len;
    while idx > 0 {
        idx -= 1;
        let lhs = if idx < len { double_remainder[idx] } else { 0 };
        let rhs = if idx < divisor_len { divisor[idx] } else { 0 };
        if lhs != rhs {
            return if lhs > rhs {
                core::cmp::Ordering::Greater
            } else {
                core::cmp::Ordering::Less
            };
        }
    }
    core::cmp::Ordering::Equal
}

/// Rebuild a signed `Int<N>` from `magnitude_limbs` and sign `is_negative`,
/// panicking on overflow in BOTH debug and release (the decimal default
/// operator never silently wraps a wrong number).
#[inline]
fn apply_sign<const N: usize>(magnitude_limbs: [u64; N], is_negative: bool, msg: &str) -> Int<N> {
    let magnitude = Int::<N>::from_limbs(magnitude_limbs);
    if magnitude.is_negative() && !(is_negative && magnitude == Int::<N>::MIN) {
        panic!("{msg}");
    }
    if is_negative {
        magnitude.wrapping_neg()
    } else {
        magnitude
    }
}

/// Widen-then-divide decimal division kernel, generic over `N`. Requires
/// `Limbs<N>: ComputeLimbs` for the `2N`-limb scaled-numerator scratch.
///
/// `multiplier` is the pre-computed `10^SCALE` multiplier in `Int<N>` storage
/// (the policy evaluates the per-type `multiplier()` const so it folds at
/// compile time). Forms `|dividend| * multiplier` in scratch, divides by
/// `|divisor|` via the
/// int layer, rounds under `mode`, and rebuilds the signed quotient.
///
/// Panics on a zero divisor.
#[inline]
pub(crate) fn div_widen_scale<const N: usize>(
    dividend: Int<N>,
    divisor: Int<N>,
    multiplier: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if divisor == Int::<N>::ZERO {
        panic!("attempt to divide by zero");
    }
    let is_negative = dividend.is_negative() != divisor.is_negative();
    let dividend_mag = *dividend.unsigned_abs().as_limbs();
    let multiplier_mag = *multiplier.as_limbs(); // multiplier >= 0
    let divisor_mag = *divisor.unsigned_abs().as_limbs();
    let dividend_len = sig_len(&dividend_mag);
    let multiplier_len = sig_len(&multiplier_mag);
    let divisor_len = sig_len(&divisor_mag);

    // ── Fast path: the scaled numerator |dividend|·10^SCALE fits Int<N> ──
    // When `lz(|dividend|) + lz(10^SCALE) > Int<N>::BITS` the product fits Int<N>, so
    // divide in N limbs and skip the 2N widen machinery (the ×10^SCALE into a
    // double-buffered scratch, the 2N-sized divide setup). Mirrors
    // `mul_widen_divide`'s fits-Int<N> arm (at SCALE==0,
    // `multiplier == 1`, so it engages for any operand with ≥2 leading zero
    // bits).
    // Bit-identical: the same `round(|dividend|·10^SCALE / |divisor|)`, an N-limb Knuth
    // divide instead of 2N. Hardcoding Knuth is the matcher's choice for this
    // shape: the dividend fits N limbs (≤ the divisor width `n`), so the u128
    // engine's `dividend ≥ 2n` precondition is false and `select_for_limbs`
    // always returns Knuth here. Class-G caveat: this direct engine call is
    // sound ONLY while the matcher's verdict for this shape IS Knuth; MUST
    // be re-verified whenever an Algorithm arm joins `int::policy::div_rem`
    // (a new engine winning for small-`n` dividends would void this fast path).
    let dividend_leading_zeros = dividend.unsigned_abs().leading_zeros();
    let multiplier_leading_zeros = multiplier.unsigned_abs().leading_zeros();
    if dividend_leading_zeros + multiplier_leading_zeros > <Int<N>>::BITS {
        let numerator_mag = *dividend.wrapping_mul(multiplier).unsigned_abs().as_limbs();
        let numerator_len = sig_len(&numerator_mag);
        let mut quotient = [0u64; N];
        let mut remainder = [0u64; N];
        let mut u_buf = Limbs::<N>::single_buffered_u64();
        let mut v_buf = Limbs::<N>::single_buffered_u64();
        div_knuth_into(
            &numerator_mag[..numerator_len],
            &divisor_mag[..divisor_len],
            &mut quotient,
            &mut remainder,
            u_buf.as_mut(),
            v_buf.as_mut(),
        );
        let remainder_len = sig_len(&remainder[..divisor_len.max(1)]);
        let remainder_nonzero = !(remainder_len == 1 && remainder[0] == 0);
        if remainder_nonzero {
            let remainder_cmp = cmp_double_vs::<N>(
                &remainder[..divisor_len.max(1)], &divisor_mag[..divisor_len]);
            let quotient_is_odd = (quotient[0] & 1) != 0;
            if should_bump(mode, remainder_cmp, quotient_is_odd, !is_negative) {
                let mut carry: u64 = 1;
                for limb in quotient.iter_mut() {
                    let (sum, overflowed) = limb.overflowing_add(carry);
                    *limb = sum;
                    if !overflowed {
                        carry = 0;
                        break;
                    }
                }
                let _ = carry;
            }
        }
        return apply_sign::<N>(quotient, is_negative, "attempt to divide with overflow");
    }

    // Scaled numerator |dividend| * 10^SCALE (up to 2N u64 limbs) in scratch.
    let mut numerator_buf = Limbs::<N>::double_buffered_u64();
    let numerator = numerator_buf.as_mut();
    let numerator_len = (dividend_len + multiplier_len).min(numerator.len());
    for slot in numerator[..numerator_len].iter_mut() {
        *slot = 0;
    }
    mul_slice(&dividend_mag[..dividend_len], &multiplier_mag[..multiplier_len],
        &mut numerator[..numerator_len]);
    let numerator_sig_len = sig_len(&numerator[..numerator_len]);

    // quotient = numerator / divisor, remainder = numerator % divisor
    // (magnitudes, via the int layer).
    let mut quotient_buf = Limbs::<N>::double_buffered_u64();
    let quotient = quotient_buf.as_mut();
    let mut remainder_buf = Limbs::<N>::double_buffered_u64();
    let remainder = remainder_buf.as_mut();
    let quotient_len = numerator_sig_len.max(1);
    for slot in quotient[..quotient_len].iter_mut() {
        *slot = 0;
    }
    for slot in remainder[..divisor_len.max(1)].iter_mut() {
        *slot = 0;
    }
    // Route on the divide matcher's verdict, with exact `ComputeLimbs` scratch
    // per engine. The scaled-numerator shape (`2N`-limb dividend over an
    // `N`-limb divisor) is exactly where the u128-limb engine wins — for an
    // even divisor of ≥ 24 limbs whose dividend is ≥ 2·n — so it picks up the
    // wide-tier `/` win; every other shape takes Knuth (a single-limb divisor
    // is handled inside `div_knuth_into`). Burnikel–Ziegler can't engage (the
    // divisor is `N ≤ 64 < 65` limbs) and `Schoolbook` is never returned,
    // but both are matched (no `_`) so a new engine forces a decision here.
    let numerator_slice = &numerator[..numerator_sig_len];
    let divisor_slice = &divisor_mag[..divisor_len];
    let quotient_slice = &mut quotient[..quotient_len];
    let remainder_slice = &mut remainder[..divisor_len.max(1)];
    match select_for_limbs(numerator_slice, divisor_slice) {
        Algorithm::KnuthU128Limb => {
            // `u` = `2N`-value normalised dividend in u128 (`double_buffered`);
            // `v` = `N`-value divisor in u128 (`single`); the u64 buffers hold
            // the base-2⁶⁴ normalisation before packing.
            let mut u64buf = Limbs::<N>::double_buffered_u64();
            let mut v64buf = Limbs::<N>::single_buffered_u64();
            let mut u128_u = Limbs::<N>::double_buffered_u128();
            let mut u128_v = Limbs::<N>::single_u128();
            div_knuth_u128_limb_into(
                numerator_slice,
                divisor_slice,
                quotient_slice,
                remainder_slice,
                u64buf.as_mut(),
                v64buf.as_mut(),
                u128_u.as_mut(),
                u128_v.as_mut(),
            );
        }
        Algorithm::Rem
        | Algorithm::Knuth
        | Algorithm::BurnikelZieglerWithKnuth
        | Algorithm::Schoolbook => {
            // The scaled numerator spans up to `2N` limbs, so its normalised
            // `u` needs `double_buffered_u64` (`≥ 2N + 2`); the divisor is
            // `N`-wide, so `v` needs `single_buffered_u64` (`N + 2`).
            let mut u_buf = Limbs::<N>::double_buffered_u64();
            let mut v_buf = Limbs::<N>::single_buffered_u64();
            div_knuth_into(numerator_slice, divisor_slice, quotient_slice, remainder_slice,
                u_buf.as_mut(), v_buf.as_mut());
        }
    }

    // Round per `mode`: compare remainder against divisor - remainder.
    let remainder_len = sig_len(&remainder[..divisor_len.max(1)]);
    let remainder_nonzero = !(remainder_len == 1 && remainder[0] == 0);
    if remainder_nonzero {
        let remainder_cmp = cmp_double_vs::<N>(
            &remainder[..divisor_len.max(1)], &divisor_mag[..divisor_len]);
        let quotient_is_odd = (quotient[0] & 1) != 0;
        if should_bump(mode, remainder_cmp, quotient_is_odd, !is_negative) {
            let mut carry: u64 = 1;
            for limb in quotient.iter_mut() {
                let (sum, overflowed) = limb.overflowing_add(carry);
                *limb = sum;
                if !overflowed {
                    carry = 0;
                    break;
                }
            }
            let _ = carry;
        }
    }

    let mut quotient_limbs = [0u64; N];
    quotient_limbs.copy_from_slice(&quotient[..N]);
    apply_sign::<N>(quotient_limbs, is_negative, "attempt to divide with overflow")
}
