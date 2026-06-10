// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `mul_widen_divide` -- decimal multiplication by the widen-then-divide
//! method, generic over the storage width `N` only.
//!
//! Multiplies `a * b` for two same-`SCALE` decimals stored as `Int<N>`. The
//! logical product is `(a / 10^SCALE) * (b / 10^SCALE)`, whose raw storage
//! is `a * b / 10^SCALE`. The full product spans up to twice the storage
//! width (`2N` limbs), so it is formed in a limb **scratch buffer** rather
//! than a work *type* `Int<2N>` (which stable Rust cannot name from `N`).
//!
//! # Generic over the storage width only -- no `Int<2N>` work type
//!
//! Following the `sqrt`/`cbrt`/`hypot` template, the kernel is generic over
//! the storage limb count `N` alone and does the `2N`-wide work directly in
//! a `ComputeLimbs::double_buffered_u64()` buffer:
//!
//! 1. form the magnitude product `|a| * |b|` (`2N` u64 limbs) via the int
//!    layer's const-`N` policy dispatcher
//!    [`crate::int::policy::mul::dispatch`], which routes even-`N` widths
//!    to the u128-packed `mul_full_limb` kernel for maximum throughput;
//! 2. transcode the product into a u128 magnitude buffer and divide it by
//!    `10^SCALE` in place via the shared MG / Newton magnitude-slice cores
//!    ([`crate::algos::support::mg_divide::div_pow10_mag_u128`] for
//!    `SCALE <= 38`, [`crate::algos::support::newton_reciprocal::dispatch_pow10_mag_u128`]
//!    above) -- the same magic-number / Newton-reciprocal path the typed
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

use crate::int::types::traits::BigInt;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Rebuild a signed `Int<N>` from a quotient magnitude held in u128 limbs
/// `mag` (the low `(N + 1) / 2` of which carry the result) and sign `neg`.
/// Panics in BOTH debug and release if the magnitude exceeds `Int<N>`'s
/// representable range — the decimal default operator never silently wraps a
/// wrong number. (The explicit `wrapping_mul` / `checked_mul` etc. variants
/// take their own `Int<N>` paths and do not reach this kernel.)
#[inline]
fn narrow_mag_to_int<const N: usize>(mag: &[u128], neg: bool, msg: &str) -> Int<N> {
    let u128_limbs = N.div_ceil(2);
    // Any set bit beyond the storage width is overflow.
    let mut overflow = mag.iter().skip(u128_limbs).any(|&l| l != 0);
    // For odd `N` the top counted u128 limb (`u128_limbs - 1`) is only
    // half-used — storage is `N` u64 limbs, so that limb carries one u64 and
    // its HIGH 64 bits sit beyond `Int<N>`. `skip(u128_limbs)` never reaches
    // those bits and the magnitude pack (`from_mag_sign_u128`) truncates them,
    // so a product spilling into them would wrap silently; treat any set bit
    // there as overflow. (Even `N` uses every counted limb fully — no tail.)
    if (N & 1) == 1 {
        if let Some(&top) = mag.get(u128_limbs - 1) {
            overflow |= (top >> 64) != 0;
        }
    }
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
    Int::<N>::from_mag_sign_u128(mag, neg)
}

/// Widen-then-divide decimal multiplication kernel, generic over the
/// storage limb count `N`. Requires `Limbs<N>: ComputeLimbs` for the `2N`-limb
/// product scratch.
///
/// A fast path skips the wide product when `a * b` provably fits `Int<N>`
/// (via leading-zero counts); otherwise the magnitude product is formed in
/// the scratch buffer via [`crate::int::policy::mul::dispatch`] (which routes
/// even-`N` widths to the u128-packed `mul_full_limb` kernel), divided by
/// `10^SCALE` via the MG / Newton magnitude cores, and rebuilt as `Int<N>`
/// (panics on overflow in both debug and release). `SCALE == 0` returns the
/// product unscaled.
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
        crate::algos::support::rescale::dispatch_mag(
            &mut mag[..u128_limbs],
            SCALE,
            neg,
            mode,
            <Int<N>>::BITS,
        );
        return Int::<N>::from_mag_sign_u128(&mag[..u128_limbs], neg);
    }

    // Slow path: form |a| * |b| (2N u64 limbs) in the work scratch via the
    // int-layer const-N policy dispatcher -- routes even-N widths to the
    // u128-packed mul_full_limb kernel (the full-product sibling of
    // mul_low_limb); the dispatcher zeroes its own accumulator and writes
    // 2*N u64 limbs into prod_buf.
    let a_mag = *a.unsigned_abs().as_limbs();
    let b_mag = *b.unsigned_abs().as_limbs();

    let mut prod_buf = Limbs::<N>::double_buffered_u64();
    crate::int::policy::mul::dispatch::<N>(&a_mag, &b_mag, prod_buf.as_mut());
    let prod = prod_buf.as_ref();

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

    // Magnitude-length-aware rescale (mirrors the typed door
    // `rescale::dispatch_wide_pow10`, task 9.24). A *representable* product is
    // far shorter than the full `2N`-limb buffer: the result must fit `Int<N>`,
    // so `|a*b| <= 10^SCALE * |Int::<N>::MAX|` and the high u128 limbs of `mag`
    // are zero. Every rescale kernel's cost scales with the SIGNIFICANT length,
    // not the buffer width, so strip the leading-zero high limbs and size
    // `select` + the baked-Newton apply on the real length — otherwise the
    // wide-tier `÷10^SCALE` Newton runs at the full `2N` width regardless of the
    // operand magnitude (the L6 `mul_D1232` regression). Bit-identical: the
    // quotient `<= ` the numerator, so the trimmed high limbs stay zero and
    // `narrow_mag_to_int` reads the full `mag` unchanged.
    let mut sig = mag.len();
    while sig > 1 && mag[sig - 1] == 0 {
        sig -= 1;
    }
    let sig_bits = (sig as u32).saturating_mul(128).min((2 * N as u32) * 64);
    crate::algos::support::rescale::dispatch_mag(&mut mag[..sig], SCALE, neg, mode, sig_bits);
    narrow_mag_to_int::<N>(&mag, neg, "attempt to multiply with overflow")
}

#[cfg(test)]
mod overflow_tests {
    use super::mul_widen_divide;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    /// `value * 10^k` as an `Int<3>` (the D57 raw storage of `value` at scale `k`).
    fn at_scale(value: i128, k: u32) -> Int<3> {
        let mut p = Int::<3>::from_i128(value);
        let ten = Int::<3>::from_i128(10);
        for _ in 0..k {
            p = p.wrapping_mul(ten);
        }
        p
    }

    /// D57 (`Int<3>`, the only odd-`N` wide tier): an out-of-range product must
    /// PANIC, while an in-range product stays bit-identical. The overflow used to
    /// slip the high 64 bits of the top half-used u128 limb and silently wrap.
    #[test]
    fn mul_widen_divide_d57_overflow_panics_in_range_exact() {
        let mode = RoundingMode::HalfToEven;

        // In-range: 3 * 4 = 12 at scale 56 (MAX ≈ 31.4) — must equal 12·10^56.
        let got = mul_widen_divide::<3, 56>(at_scale(3, 56), at_scale(4, 56), mode);
        assert_eq!(got, at_scale(12, 56), "in-range D57<56> product must be exact");

        // Out-of-range at scale 56: 15 * 13 = 195 > MAX ≈ 31.4 → must panic.
        let a = at_scale(15, 56);
        let b = at_scale(13, 56);
        let r = std::panic::catch_unwind(|| mul_widen_divide::<3, 56>(a, b, mode));
        assert!(r.is_err(), "D57<56> 15*13=195 out of range must panic, not wrap");

        // Out-of-range at scale 0: -2.219...e57 * 3 overflows Int<3> (MAX ≈ 3.14e57).
        let neg_big = Int::<3>::ZERO.wrapping_sub(at_scale(2_219_290_601, 48)); // ≈ -2.22e57, in range
        let three = Int::<3>::from_i128(3);
        let r0 = std::panic::catch_unwind(|| mul_widen_divide::<3, 0>(neg_big, three, mode));
        assert!(r0.is_err(), "D57<0> (-2.2e57)*3 overflow must panic, not wrap");
    }
}
