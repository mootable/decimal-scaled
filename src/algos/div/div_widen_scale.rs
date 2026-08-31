// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `div_widen_scale` — decimal division by the widen-then-divide method,
//! generic over the storage width `N` only.
//!
//! Divides `a / b` for two same-`SCALE` decimals stored as `Int<N>`. The
//! logical quotient is `(a / 10^SCALE) / (b / 10^SCALE) = a / b`, but to
//! keep `SCALE` fractional digits the numerator is first scaled up by
//! `10^SCALE` (`a * 10^SCALE`). Scaling can overflow `Int<N>`, so the
//! scaled numerator spans up to `2N` limbs and is formed in a limb
//! **scratch buffer** rather than a work *type* `Int<2N>`.
//!
//! # Generic over the storage width only — no `Int<2N>` work type
//!
//! Following the `sqrt`/`cbrt`/`hypot` template, the kernel is generic over
//! `N` alone:
//!
//! 1. form `|a| * 10^SCALE` (`2N` u64 limbs) in a [`ComputeLimbs::double_buffered_u64`]
//!    buffer via the int slice multiply;
//! 2. divide it by `|b|` via the int layer's width-agnostic divide
//!    ([`crate::int::algos::div::div_fixed::div_rem_mag_slice`], which
//!    fronts the divisor-shape policy — Knuth / single-limb fast paths),
//!    rounding under `mode`;
//! 3. rebuild the signed `Int<N>` quotient (panics on overflow in both
//!    debug and release).
//!
//! The divisor here is the runtime operand `b`, not `10^SCALE`, so the MG
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
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// Compare `2*rem` against `divisor` (little-endian magnitudes), returning
/// the ordering of `rem` vs `divisor - rem` (the rounding half-comparison).
#[inline]
fn cmp_double_vs<const N: usize>(rem: &[u64], divisor: &[u64]) -> core::cmp::Ordering
where
    Limbs<N>: ComputeLimbs,
{
    // `2·rem` spans at most `rem.len() + 1` limbs, and `rem < divisor`, whose
    // length is `≤ N`; the `single_buffered_u64` buffer (`N + 2`) holds it
    // exactly per-`N`.
    let mut two_r_buf = Limbs::<N>::single_buffered_u64();
    let two_r = two_r_buf.as_mut();
    let mut carry: u64 = 0;
    for (i, &r) in rem.iter().enumerate() {
        let v = ((r as u128) << 1) | carry as u128;
        two_r[i] = v as u64;
        carry = (v >> 64) as u64;
    }
    let mut len = rem.len();
    if carry != 0 {
        two_r[len] = carry;
        len += 1;
    }
    let dl = divisor.len();
    let maxl = len.max(dl);
    let mut k = maxl;
    while k > 0 {
        k -= 1;
        let lhs = if k < len { two_r[k] } else { 0 };
        let rhs = if k < dl { divisor[k] } else { 0 };
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

/// Rebuild a signed `Int<N>` from magnitude limbs `out` and sign `neg`,
/// panicking on overflow in BOTH debug and release (the decimal default
/// operator never silently wraps a wrong number).
#[inline]
fn apply_sign<const N: usize>(out: [u64; N], neg: bool, msg: &str) -> Int<N> {
    let mag = Int::<N>::from_limbs(out);
    if mag.is_negative() && !(neg && mag == Int::<N>::MIN) {
        panic!("{msg}");
    }
    if neg {
        mag.wrapping_neg()
    } else {
        mag
    }
}

/// Widen-then-divide decimal division kernel, generic over `N`. Requires
/// `Limbs<N>: ComputeLimbs` for the `2N`-limb scaled-numerator scratch.
///
/// `mult` is the pre-computed `10^SCALE` multiplier in `Int<N>` storage
/// (the policy evaluates the per-type `multiplier()` const so it folds at
/// compile time). Forms `|a| * mult` in scratch, divides by `|b|` via the
/// int layer, rounds under `mode`, and rebuilds the signed quotient.
///
/// Panics on a zero divisor.
#[inline]
pub(crate) fn div_widen_scale<const N: usize>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if b == Int::<N>::ZERO {
        panic!("attempt to divide by zero");
    }
    let neg = a.is_negative() != b.is_negative();
    let a_mag = *a.unsigned_abs().as_limbs();
    let m_mag = *mult.as_limbs(); // mult >= 0
    let b_mag = *b.unsigned_abs().as_limbs();
    let al = sig_len(&a_mag);
    let ml = sig_len(&m_mag);
    let bl = sig_len(&b_mag);

    // ── Fast path: the scaled numerator |a|·10^SCALE fits Int<N> ─────────
    // When `lz(|a|) + lz(10^SCALE) > Int<N>::BITS` the product fits Int<N>, so
    // divide in N limbs and skip the 2N widen machinery (the ×10^SCALE into a
    // double-buffered scratch, the 2N-sized divide setup). Mirrors
    // `mul_widen_divide`'s fits-Int<N> arm and recovers v0.4.4's `div_with`
    // fast path — the narrow balanced `div@low-scale` recovery (at SCALE==0,
    // `mult == 1`, so it engages for any operand with ≥2 leading zero bits).
    // Bit-identical: the same `round(|a|·10^SCALE / |b|)`, an N-limb Knuth
    // divide instead of 2N. Hardcoding Knuth is the matcher's choice for this
    // shape: the dividend fits N limbs (≤ the divisor width `n`), so the u128
    // engine's `dividend ≥ 2n` precondition is false and `select_for_limbs`
    // always returns Knuth here. Class-G caveat: this direct engine call is
    // sound ONLY while the matcher's verdict for this shape IS Knuth; MUST
    // be re-verified whenever an Algorithm arm joins `int::policy::div_rem`
    // (a new engine winning for small-`n` dividends would void this fast path).
    let lz_a = a.unsigned_abs().leading_zeros();
    let lz_m = mult.unsigned_abs().leading_zeros();
    if lz_a + lz_m > <Int<N>>::BITS {
        let num_mag = *a.wrapping_mul(mult).unsigned_abs().as_limbs();
        let nl_fast = sig_len(&num_mag);
        let mut quot = [0u64; N];
        let mut rem = [0u64; N];
        let mut u_buf = Limbs::<N>::single_buffered_u64();
        let mut v_buf = Limbs::<N>::single_buffered_u64();
        div_knuth_into(
            &num_mag[..nl_fast],
            &b_mag[..bl],
            &mut quot,
            &mut rem,
            u_buf.as_mut(),
            v_buf.as_mut(),
        );
        let rl = sig_len(&rem[..bl.max(1)]);
        let rem_nonzero = !(rl == 1 && rem[0] == 0);
        if rem_nonzero {
            let cmp_r = cmp_double_vs::<N>(&rem[..bl.max(1)], &b_mag[..bl]);
            let q_is_odd = (quot[0] & 1) != 0;
            if should_bump(mode, cmp_r, q_is_odd, !neg) {
                let mut carry: u64 = 1;
                for limb in quot.iter_mut() {
                    let (s, c) = limb.overflowing_add(carry);
                    *limb = s;
                    if !c {
                        carry = 0;
                        break;
                    }
                }
                let _ = carry;
            }
        }
        return apply_sign::<N>(quot, neg, "attempt to divide with overflow");
    }

    // Scaled numerator |a| * 10^SCALE (up to 2N u64 limbs) in scratch.
    let mut num_buf = Limbs::<N>::double_buffered_u64();
    let num = num_buf.as_mut();
    let nlen = (al + ml).min(num.len());
    for slot in num[..nlen].iter_mut() {
        *slot = 0;
    }
    mul_slice(&a_mag[..al], &m_mag[..ml], &mut num[..nlen]);
    let ntop = sig_len(&num[..nlen]);

    // q = num / b, r = num % b (magnitudes, via the int layer).
    let mut quot_buf = Limbs::<N>::double_buffered_u64();
    let quot = quot_buf.as_mut();
    let mut rem_buf = Limbs::<N>::double_buffered_u64();
    let rem = rem_buf.as_mut();
    let qlen = ntop.max(1);
    for slot in quot[..qlen].iter_mut() {
        *slot = 0;
    }
    for slot in rem[..bl.max(1)].iter_mut() {
        *slot = 0;
    }
    // Route on the divide matcher's verdict, with exact `ComputeLimbs` scratch
    // per engine. The scaled-numerator shape (`2N`-limb dividend over an
    // `N`-limb divisor) is exactly where the u128-limb engine wins — for an
    // even divisor of ≥ 24 limbs whose dividend is ≥ 2·n — so it picks up the
    // wide-tier `/` win; every other shape takes Knuth (a single-limb divisor
    // is handled inside `div_knuth_into`). Burnikel–Ziegler can't engage (the
    // divisor `b` is `N ≤ 64 < 65` limbs) and `Schoolbook` is never returned,
    // but both are matched (no `_`) so a new engine forces a decision here.
    let num_s = &num[..ntop];
    let den_s = &b_mag[..bl];
    let q = &mut quot[..qlen];
    let r = &mut rem[..bl.max(1)];
    match select_for_limbs(num_s, den_s) {
        Algorithm::KnuthU128Limb => {
            // `u` = `2N`-value normalised dividend in u128 (`double_buffered`);
            // `v` = `N`-value divisor in u128 (`single`); the u64 buffers hold
            // the base-2⁶⁴ normalisation before packing.
            let mut u64buf = Limbs::<N>::double_buffered_u64();
            let mut v64buf = Limbs::<N>::single_buffered_u64();
            let mut u128_u = Limbs::<N>::double_buffered_u128();
            let mut u128_v = Limbs::<N>::single_u128();
            div_knuth_u128_limb_into(
                num_s,
                den_s,
                q,
                r,
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
            // `u` needs `double_buffered_u64` (`≥ 2N + 2`); the divisor `b` is
            // `N`-wide, so `v` needs `single_buffered_u64` (`N + 2`).
            let mut u_buf = Limbs::<N>::double_buffered_u64();
            let mut v_buf = Limbs::<N>::single_buffered_u64();
            div_knuth_into(num_s, den_s, q, r, u_buf.as_mut(), v_buf.as_mut());
        }
    }

    // Round per `mode`: compare remainder against b - remainder.
    let rl = sig_len(&rem[..bl.max(1)]);
    let rem_nonzero = !(rl == 1 && rem[0] == 0);
    if rem_nonzero {
        let cmp_r = cmp_double_vs::<N>(&rem[..bl.max(1)], &b_mag[..bl]);
        let q_is_odd = (quot[0] & 1) != 0;
        if should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u64 = 1;
            for limb in quot.iter_mut() {
                let (s, c) = limb.overflowing_add(carry);
                *limb = s;
                if !c {
                    carry = 0;
                    break;
                }
            }
            let _ = carry;
        }
    }

    let mut out = [0u64; N];
    out.copy_from_slice(&quot[..N]);
    apply_sign::<N>(out, neg, "attempt to divide with overflow")
}
