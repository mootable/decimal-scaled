//! `mul_widen_divide` — decimal multiplication by the widen-then-divide
//! method, generic over the storage width `N` only.
//!
//! Multiplies `a * b` for two same-`SCALE` decimals stored as `Int<N>`. The
//! logical product is `(a / 10^SCALE) * (b / 10^SCALE)`, whose raw storage
//! is `a * b / 10^SCALE`. The full product spans up to twice the storage
//! width (`2N` limbs), so it is formed in a limb **scratch buffer** rather
//! than a work *type* `Int<2N>` (which stable Rust cannot name from `N`).
//!
//! # Generic over the storage width only — no `Int<2N>` work type
//!
//! Following the `sqrt`/`cbrt`/`hypot` template, the kernel is generic over
//! the storage limb count `N` alone and does the `2N`-wide work directly in
//! a `ComputeInt::double_buffered_u64()` buffer:
//!
//! 1. form the magnitude product `|a| * |b|` (`2N` u64 limbs) via the int
//!    layer's width-agnostic slice kernel
//!    [`crate::int::algos::mul::mul_schoolbook::mul_schoolbook`];
//! 2. transcode the product into a u128 magnitude buffer and divide it by
//!    `10^SCALE` in place via the shared MG / Newton magnitude-slice cores
//!    ([`crate::algos::support::mg_divide::div_pow10_mag_u128`] for
//!    `SCALE <= 38`, [`crate::algos::support::newton_reciprocal::dispatch_pow10_mag_u128`]
//!    above) — the same magic-number / Newton-reciprocal path the typed
//!    `div_wide_pow10` wrapper uses, so no Knuth-divide regression;
//! 3. rebuild the signed `Int<N>` result from the quotient magnitude and
//!    the product sign.
//!
//! A leading-zero fast path keeps the narrow case cheap: when the
//! unsigned-magnitude leading-zero count proves `a * b` fits `Int<N>`, the
//! product stays in `Int<N>` and the divide runs over its `(N + 1) / 2`
//! u128 limbs.
//!
//! All integer arithmetic dispatches DOWN to the int layer; this fn never
//! calls a decimal method on its own value.

use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::types::traits::BigInt;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Significant limb length of `a` (index of the highest non-zero limb + 1),
/// clamped to at least 1 so zero has length 1.
#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// Divide the u128 magnitude `mag` (in place) by `10^SCALE`, choosing the
/// MG single-chunk / MG chain / Newton path exactly as the typed
/// `div_wide_pow10` / `dispatch_wide_pow10` wrappers do. `neg` is
/// the result sign (rounding tie-break); `work_bits` is the work width in
/// bits (the Newton cache / threshold key). `SCALE == 0` is a no-op.
#[inline]
fn divide_mag_by_pow10(mag: &mut [u128], scale: u32, neg: bool, work_bits: u32, mode: RoundingMode) {
    if scale == 0 {
        // no rescale
    } else if scale <= 38 {
        crate::algos::support::mg_divide::div_pow10_mag_u128(mag, scale, neg, mode);
    } else {
        crate::algos::support::newton_reciprocal::dispatch_pow10_mag_u128(
            mag, scale, neg, mode, work_bits,
        );
    }
}

/// Rebuild a signed `Int<N>` from a quotient magnitude held in u128 limbs
/// `mag` (the low `(N + 1) / 2` of which carry the result) and sign `neg`.
/// In debug, panics if the magnitude exceeds `Int<N>`'s representable range
/// (matching the old `narrow_or_panic!`); in release it wraps via
/// `from_mag_sign_u128`.
#[inline]
fn narrow_mag_to_int<const N: usize>(mag: &[u128], neg: bool, msg: &str) -> Int<N> {
    if cfg!(debug_assertions) {
        let u128_limbs = N.div_ceil(2);
        // Any set bit beyond the storage width is overflow.
        let mut overflow = mag.iter().skip(u128_limbs).any(|&l| l != 0);
        if !overflow {
            // Compare the in-range magnitude against |Int<N>::MAX| / |MIN|.
            let limit = if neg {
                Int::<N>::MIN.unsigned_abs()
            } else {
                Int::<N>::MAX.unsigned_abs()
            };
            let limit_limbs = *limit.as_limbs();
            // Reconstruct the result magnitude limbs (u64) for the compare.
            let mut got = [0u64; N];
            let pairs = (N / 2).min(u128_limbs).min(mag.len());
            let mut i = 0;
            while i < pairs {
                got[2 * i] = mag[i] as u64;
                got[2 * i + 1] = (mag[i] >> 64) as u64;
                i += 1;
            }
            if (N & 1) == 1 && i < u128_limbs && i < mag.len() {
                got[2 * i] = mag[i] as u64;
            }
            // got > limit ?  (little-endian magnitude compare)
            let mut k = N;
            while k > 0 {
                k -= 1;
                if got[k] != limit_limbs[k] {
                    overflow = got[k] > limit_limbs[k];
                    break;
                }
            }
        }
        if overflow {
            panic!("{msg}");
        }
    }
    Int::<N>::from_mag_sign_u128(mag, neg)
}

/// Widen-then-divide decimal multiplication kernel, generic over the
/// storage limb count `N`. Requires `Limbs<N>: ComputeLimbs` for the `2N`-limb
/// product scratch.
///
/// A fast path skips the wide product when `a * b` provably fits `Int<N>`
/// (via leading-zero counts); otherwise the magnitude product is formed in
/// the scratch buffer, divided by `10^SCALE` via the MG / Newton magnitude
/// cores, and rebuilt as `Int<N>` (debug panic / release wrap on overflow).
/// `SCALE == 0` returns the product unscaled.
#[inline]
pub(crate) fn mul_widen_divide<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    let neg = a.is_negative() != b.is_negative();
    let lz_a = a.unsigned_abs().leading_zeros();
    let lz_b = b.unsigned_abs().leading_zeros();

    if lz_a + lz_b > <Int<N>>::BITS {
        // Fast path: |a * b| fits `Int<N>`. Divide its `(N + 1) / 2` u128
        // limbs in place; the result certainly fits, so build directly.
        let prod: Int<N> = a.wrapping_mul(b);
        if SCALE == 0 {
            return prod;
        }
        let u128_limbs = N.div_ceil(2);
        let mut mag = [0u128; N];
        let _ = prod.mag_into_u128(&mut mag[..u128_limbs]);
        divide_mag_by_pow10(&mut mag[..u128_limbs], SCALE, neg, <Int<N>>::BITS, mode);
        return Int::<N>::from_mag_sign_u128(&mag[..u128_limbs], neg);
    }

    // Slow path: form |a| * |b| (2N u64 limbs) in the work scratch via the
    // int slice kernel, then divide the u128 view by 10^SCALE.
    let a_mag = *a.unsigned_abs().as_limbs();
    let b_mag = *b.unsigned_abs().as_limbs();
    let al = sig_len(&a_mag);
    let bl = sig_len(&b_mag);

    let mut prod_buf = Limbs::<N>::double_buffered_u64();
    let prod = prod_buf.as_mut();
    let plen = (al + bl).min(prod.len());
    for slot in prod[..plen].iter_mut() {
        *slot = 0;
    }
    mul_schoolbook(&a_mag[..al], &b_mag[..bl], &mut prod[..plen]);

    // Transcode the 2N-u64 product into N u128 limbs (2N u64 == N u128).
    let mut mag = [0u128; N];
    for i in 0..N {
        let lo = prod[2 * i] as u128;
        let hi = *prod.get(2 * i + 1).unwrap_or(&0) as u128;
        mag[i] = lo | (hi << 64);
    }

    if SCALE == 0 {
        return narrow_mag_to_int::<N>(&mag, neg, "attempt to multiply with overflow");
    }

    // Work width is the 2N-limb product: 2 * N * 64 bits.
    let work_bits = (2 * N as u32) * 64;
    divide_mag_by_pow10(&mut mag, SCALE, neg, work_bits, mode);
    narrow_mag_to_int::<N>(&mag, neg, "attempt to multiply with overflow")
}
