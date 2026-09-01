// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `mul_schoolbook` -- naive schoolbook decimal multiplication reference,
//! generic over the storage width `N` only.
//!
//! Computes `lhs * rhs` for two same-`SCALE` decimals stored as `Int<N>`.
//! The logical product is `(lhs / 10^SCALE) * (rhs / 10^SCALE)`, whose raw
//! storage value is `lhs * rhs / 10^SCALE`.
//!
//! This is the naive reference algorithm — no leading-zero fast path:
//!
//! 1. Form the full magnitude product `|lhs| * |rhs|` (`2N` u64 limbs) in a
//!    [`ComputeLimbs::double_buffered_u64`] buffer via the int layer's slice
//!    [`crate::int::algos::mul::mul_schoolbook::mul_schoolbook`].
//! 2. Build `10^SCALE` in the same limb domain and divide the product by
//!    it using the int layer's width-agnostic divide
//!    ([`crate::int::algos::div::div_fixed::div_rem_mag_slice`]),
//!    rounding under `mode`.
//! 3. Rebuild the signed `Int<N>` quotient.
//!
//! Unlike [`mul_widen_divide`](super::mul_widen_divide::mul_widen_divide),
//! this kernel has no leading-zero fast path and does not use the
//! MG-divide or Newton-reciprocal acceleration: it divides via the plain
//! int-layer `div_rem`, making it the unambiguous schoolbook reference and
//! a real benchmarkable seam.
//!
//! All integer arithmetic dispatches DOWN to the int layer; this fn never
//! calls a decimal method on its own value.

use crate::int::algos::div::div_fixed::div_rem_mag_slice;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook as mul_slice;
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

/// Naive schoolbook decimal multiplication, generic over `N`. Requires
/// `Limbs<N>: ComputeLimbs` for the `2N`-limb product scratch.
///
/// Forms the full magnitude product in the scratch buffer, then divides by
/// `10^SCALE` using the plain int-layer `div_rem`, rounding under `mode`.
/// No MG-divide, no Newton-reciprocal, no leading-zero fast path.
/// `SCALE == 0` returns the narrowed product unscaled.
#[inline]
pub(crate) fn mul_schoolbook<const N: usize, const SCALE: u32>(
    lhs: Int<N>,
    rhs: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    let is_negative = lhs.is_negative() != rhs.is_negative();
    let lhs_mag = *lhs.unsigned_abs().as_limbs();
    let rhs_mag = *rhs.unsigned_abs().as_limbs();
    let lhs_len = sig_len(&lhs_mag);
    let rhs_len = sig_len(&rhs_mag);

    // Full magnitude product in the work scratch (2N u64 limbs).
    let mut product_buf = Limbs::<N>::double_buffered_u64();
    let product = product_buf.as_mut();
    let product_len = (lhs_len + rhs_len).min(product.len());
    for slot in product[..product_len].iter_mut() {
        *slot = 0;
    }
    mul_slice(&lhs_mag[..lhs_len], &rhs_mag[..rhs_len], &mut product[..product_len]);

    if SCALE == 0 {
        let mut product_limbs = [0u64; N];
        product_limbs.copy_from_slice(&product[..N]);
        return apply_sign::<N>(product_limbs, is_negative, "attempt to multiply with overflow");
    }

    // Build 10^SCALE in a u64 limb buffer (iterative *10).
    let mut divisor_buf = Limbs::<N>::double_buffered_u64();
    let divisor = divisor_buf.as_mut();
    divisor[0] = 1;
    let mut divisor_len = 1usize;
    for _ in 0..SCALE {
        let mut carry: u64 = 0;
        for limb in divisor[..divisor_len].iter_mut() {
            let scaled_limb = (*limb as u128) * 10u128 + carry as u128;
            *limb = scaled_limb as u64;
            carry = (scaled_limb >> 64) as u64;
        }
        if carry != 0 {
            divisor[divisor_len] = carry;
            divisor_len += 1;
        }
    }

    // quotient = product / divisor, remainder = product % divisor
    // (magnitudes, via int layer).
    let product_sig_len = sig_len(&product[..product_len]);
    let mut quotient_buf = Limbs::<N>::double_buffered_u64();
    let quotient = quotient_buf.as_mut();
    let mut remainder_buf = Limbs::<N>::double_buffered_u64();
    let remainder = remainder_buf.as_mut();
    for slot in quotient[..product_sig_len].iter_mut() {
        *slot = 0;
    }
    for slot in remainder[..divisor_len].iter_mut() {
        *slot = 0;
    }
    div_rem_mag_slice(&product[..product_sig_len], &divisor[..divisor_len],
        &mut quotient[..product_sig_len], &mut remainder[..divisor_len]);

    // Round: compare remainder against divisor - remainder.
    let remainder_len = sig_len(&remainder[..divisor_len]);
    let remainder_nonzero = !(remainder_len == 1 && remainder[0] == 0);
    if remainder_nonzero {
        // remainder_cmp = remainder.cmp(divisor - remainder), via comparing
        // 2*remainder to divisor.
        let remainder_cmp = cmp_double_vs::<N>(&remainder[..divisor_len], &divisor[..divisor_len]);
        let quotient_is_odd = (quotient[0] & 1) != 0;
        if should_bump(mode, remainder_cmp, quotient_is_odd, !is_negative) {
            // quotient += 1
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
    apply_sign::<N>(quotient_limbs, is_negative, "attempt to multiply with overflow")
}

/// Compare `2*remainder` against `divisor` (both little-endian magnitudes),
/// returning the ordering of `remainder` vs `divisor - remainder`.
#[inline]
fn cmp_double_vs<const N: usize>(remainder: &[u64], divisor: &[u64]) -> core::cmp::Ordering
where
    Limbs<N>: ComputeLimbs,
{
    // `2·remainder` spans at most `remainder.len() + 1` limbs, and
    // `remainder < divisor`, whose
    // length is `≤ N + 1` (the `10^SCALE` divisor); the `single_buffered_u64`
    // buffer (`N + 2`) holds it exactly per-`N`.
    let mut double_remainder_buf = Limbs::<N>::single_buffered_u64();
    let double_remainder = double_remainder_buf.as_mut();
    let mut carry: u64 = 0;
    for (i, &limb) in remainder.iter().enumerate() {
        let doubled = (limb as u128) << 1 | carry as u128;
        double_remainder[i] = doubled as u64;
        carry = (doubled >> 64) as u64;
    }
    let mut len = remainder.len();
    if carry != 0 {
        double_remainder[len] = carry;
        len += 1;
    }
    // Compare double_remainder[..len] vs divisor (little-endian).
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
    // `from_limbs` reinterprets bits as two's complement; if the top bit is
    // set the magnitude exceeds the signed range. The sole representable case
    // is exactly Int<N>::MIN with is_negative.
    if magnitude.is_negative() && !(is_negative && magnitude == Int::<N>::MIN) {
        panic!("{msg}");
    }
    if is_negative {
        magnitude.wrapping_neg()
    } else {
        magnitude
    }
}
