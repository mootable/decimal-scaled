// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `mul_widen_divide` -- decimal multiplication by the widen-then-divide
//! method, generic over the storage width `N` only.
//!
//! Multiplies `lhs * rhs` for two same-`SCALE` decimals stored as `Int<N>`.
//! The logical product is `(lhs / 10^SCALE) * (rhs / 10^SCALE)`, whose raw
//! storage is `lhs * rhs / 10^SCALE`. The full product spans up to twice the storage
//! width (`2N` limbs), so it is formed in a limb **scratch buffer** rather
//! than a work *type* `Int<2N>` (which stable Rust cannot name from `N`).
//!
//! # Generic over the storage width only -- no `Int<2N>` work type
//!
//! Following the `sqrt`/`cbrt`/`hypot` template, the kernel is generic over
//! the storage limb count `N` alone and does the `2N`-wide work directly in
//! a `ComputeLimbs::double_buffered_u64()` buffer:
//!
//! 1. form the magnitude product `|lhs| * |rhs|` (`2N` u64 limbs) via the int
//!    layer's const-`N` policy dispatcher
//!    [`crate::int::policy::mul::dispatch`], which routes even-`N` widths
//!    to the u128-packed `mul_full_limb` kernel for maximum throughput;
//! 2. transcode the product into a u128 magnitude buffer and divide it by
//!    `10^SCALE` in place via the shared MG / Barrett magnitude-slice cores
//!    ([`crate::algos::support::mg_divide::div_pow10_mag_u128`] for
//!    `SCALE <= 38`, [`crate::algos::support::barrett_reciprocal::dispatch_pow10_mag_u128`]
//!    above) -- the same magic-number / Barrett-reciprocal path the typed
//!    `div_wide_pow10` wrapper uses, so no Knuth-divide regression;
//! 3. rebuild the signed `Int<N>` result from the quotient magnitude and
//!    the product sign.
//!
//! A leading-zero fast path keeps the narrow case cheap: when the
//! unsigned-magnitude leading-zero count proves `lhs * rhs` fits `Int<N>`, the
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
/// `magnitude` (the low `(N + 1) / 2` of which carry the result) and sign
/// `is_negative`.
/// Panics in BOTH debug and release if the magnitude exceeds `Int<N>`'s
/// representable range — the decimal default operator never silently wraps a
/// wrong number. (The explicit `wrapping_mul` / `checked_mul` etc. variants
/// take their own `Int<N>` paths and do not reach this kernel.)
#[inline]
fn narrow_mag_to_int<const N: usize>(magnitude: &[u128], is_negative: bool, msg: &str) -> Int<N> {
    let u128_limbs = N.div_ceil(2);
    // Any set bit beyond the storage width is overflow.
    let mut overflow = magnitude.iter().skip(u128_limbs).any(|&limb| limb != 0);
    // For odd `N` the top counted u128 limb (`u128_limbs - 1`) is only
    // half-used — storage is `N` u64 limbs, so that limb carries one u64 and
    // its HIGH 64 bits sit beyond `Int<N>`. `skip(u128_limbs)` never reaches
    // those bits and the magnitude pack (`from_mag_sign_u128`) truncates them,
    // so a product spilling into them would wrap silently; treat any set bit
    // there as overflow. (Even `N` uses every counted limb fully — no tail.)
    if (N & 1) == 1 {
        if let Some(&top) = magnitude.get(u128_limbs - 1) {
            overflow |= (top >> 64) != 0;
        }
    }
    if !overflow {
        // Compare the in-range magnitude against |Int<N>::MAX| / |MIN|.
        let limit = if is_negative {
            Int::<N>::MIN.unsigned_abs()
        } else {
            Int::<N>::MAX.unsigned_abs()
        };
        let limit_limbs = *limit.as_limbs();
        // Reconstruct the result magnitude limbs (u64) for the compare.
        let mut magnitude_u64 = [0u64; N];
        let pairs = (N / 2).min(u128_limbs).min(magnitude.len());
        let mut i = 0;
        while i < pairs {
            magnitude_u64[2 * i] = magnitude[i] as u64;
            magnitude_u64[2 * i + 1] = (magnitude[i] >> 64) as u64;
            i += 1;
        }
        if (N & 1) == 1 && i < u128_limbs && i < magnitude.len() {
            magnitude_u64[2 * i] = magnitude[i] as u64;
        }
        // magnitude_u64 > limit ?  (little-endian magnitude compare)
        let mut idx = N;
        while idx > 0 {
            idx -= 1;
            if magnitude_u64[idx] != limit_limbs[idx] {
                overflow = magnitude_u64[idx] > limit_limbs[idx];
                break;
            }
        }
    }
    if overflow {
        panic!("{msg}");
    }
    Int::<N>::from_mag_sign_u128(magnitude, is_negative)
}

/// Widen-then-divide decimal multiplication kernel, generic over the
/// storage limb count `N`. Requires `Limbs<N>: ComputeLimbs` for the `2N`-limb
/// product scratch.
///
/// A fast path skips the wide product when `lhs * rhs` provably fits `Int<N>`
/// (via leading-zero counts); otherwise the magnitude product is formed in
/// the scratch buffer via [`crate::int::policy::mul::dispatch`] (which routes
/// even-`N` widths to the u128-packed `mul_full_limb` kernel), divided by
/// `10^SCALE` via the MG / Barrett magnitude cores, and rebuilt as `Int<N>`
/// (panics on overflow in both debug and release). `SCALE == 0` returns the
/// product unscaled.
#[inline]
pub(crate) fn mul_widen_divide<const N: usize, const SCALE: u32>(
    lhs: Int<N>,
    rhs: Int<N>,
    mode: RoundingMode,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    let is_negative = lhs.is_negative() != rhs.is_negative();
    let lhs_leading_zeros = lhs.unsigned_abs().leading_zeros();
    let rhs_leading_zeros = rhs.unsigned_abs().leading_zeros();

    if lhs_leading_zeros + rhs_leading_zeros > <Int<N>>::BITS {
        // Fast path: |lhs * rhs| fits `Int<N>`. Divide its `(N + 1) / 2` u128
        // limbs in place; the result certainly fits, so build directly.
        let product: Int<N> = lhs.wrapping_mul(rhs);
        if SCALE == 0 {
            return product;
        }
        let u128_limbs = N.div_ceil(2);
        let mut magnitude = [0u128; N];
        let _ = product.mag_into_u128(&mut magnitude[..u128_limbs]);
        crate::algos::support::rescale::dispatch_mag(
            &mut magnitude[..u128_limbs],
            SCALE,
            is_negative,
            mode,
            <Int<N>>::BITS,
        );
        return Int::<N>::from_mag_sign_u128(&magnitude[..u128_limbs], is_negative);
    }

    // Slow path: form |lhs| * |rhs| (2N u64 limbs) in the work scratch via the
    // int-layer const-N policy dispatcher -- routes even-N widths to the
    // u128-packed mul_full_limb kernel (the full-product sibling of
    // mul_low_limb); the dispatcher zeroes its own accumulator and writes
    // 2*N u64 limbs into product_buf.
    let lhs_mag = *lhs.unsigned_abs().as_limbs();
    let rhs_mag = *rhs.unsigned_abs().as_limbs();

    let mut product_buf = Limbs::<N>::double_buffered_u64();
    crate::int::policy::mul::dispatch::<N>(&lhs_mag, &rhs_mag, product_buf.as_mut());
    let product = product_buf.as_ref();

    // Transcode the 2N-u64 product into N u128 limbs (2N u64 == N u128).
    let mut magnitude = [0u128; N];
    for i in 0..N {
        let lo = product[2 * i] as u128;
        let hi = *product.get(2 * i + 1).unwrap_or(&0) as u128;
        magnitude[i] = lo | (hi << 64);
    }

    if SCALE == 0 {
        return narrow_mag_to_int::<N>(&magnitude, is_negative, "attempt to multiply with overflow");
    }

    // Magnitude-length-aware rescale (mirrors the typed door
    // `rescale::dispatch_wide_pow10`, task 9.24). A *representable* product is
    // far shorter than the full `2N`-limb buffer: the result must fit `Int<N>`,
    // so `|lhs*rhs| <= 10^SCALE * |Int::<N>::MAX|` and the high u128 limbs of
    // `magnitude` are zero. Every rescale kernel's cost scales with the
    // SIGNIFICANT length, not the buffer width, so strip the leading-zero
    // high limbs and size
    // `select` + the baked-Barrett apply on the real length — otherwise the
    // wide-tier `÷10^SCALE` Barrett runs at the full `2N` width regardless of the
    // operand magnitude. Bit-identical: the
    // quotient `<= ` the numerator, so the trimmed high limbs stay zero and
    // `narrow_mag_to_int` reads the full `magnitude` unchanged.
    let mut significant_limbs = magnitude.len();
    while significant_limbs > 1 && magnitude[significant_limbs - 1] == 0 {
        significant_limbs -= 1;
    }
    let significant_bits =
        (significant_limbs as u32).saturating_mul(128).min((2 * N as u32) * 64);
    crate::algos::support::rescale::dispatch_mag(
        &mut magnitude[..significant_limbs], SCALE, is_negative, mode, significant_bits);
    narrow_mag_to_int::<N>(&magnitude, is_negative, "attempt to multiply with overflow")
}

#[cfg(test)]
mod overflow_tests {
    use super::mul_widen_divide;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    /// `value * 10^scale` as an `Int<3>` (the D57 raw storage of `value` at
    /// scale `scale`).
    fn at_scale(value: i128, scale: u32) -> Int<3> {
        let mut scaled = Int::<3>::from_i128(value);
        let ten = Int::<3>::from_i128(10);
        for _ in 0..scale {
            scaled = scaled.wrapping_mul(ten);
        }
        scaled
    }

    /// D57 (`Int<3>`, the only odd-`N` wide tier): an out-of-range product must
    /// PANIC, while an in-range product stays bit-identical. Without the panic, the
    /// overflow would slip the high 64 bits of the top half-used u128 limb and silently wrap.
    #[test]
    fn mul_widen_divide_d57_overflow_panics_in_range_exact() {
        let mode = RoundingMode::HalfToEven;

        // In-range: 3 * 4 = 12 at scale 56 (MAX ≈ 31.4) — must equal 12·10^56.
        let got = mul_widen_divide::<3, 56>(at_scale(3, 56), at_scale(4, 56), mode);
        assert_eq!(got, at_scale(12, 56), "in-range D57<56> product must be exact");

        // Out-of-range at scale 56: 15 * 13 = 195 > MAX ≈ 31.4 → must panic.
        let lhs = at_scale(15, 56);
        let rhs = at_scale(13, 56);
        let caught = std::panic::catch_unwind(|| mul_widen_divide::<3, 56>(lhs, rhs, mode));
        assert!(caught.is_err(), "D57<56> 15*13=195 out of range must panic, not wrap");

        // Out-of-range at scale 0: -2.219...e57 * 3 overflows Int<3> (MAX ≈ 3.14e57).
        let negative_big = Int::<3>::ZERO.wrapping_sub(at_scale(2_219_290_601, 48)); // ≈ -2.22e57, in range
        let three = Int::<3>::from_i128(3);
        let caught_scale0 =
            std::panic::catch_unwind(|| mul_widen_divide::<3, 0>(negative_big, three, mode));
        assert!(caught_scale0.is_err(), "D57<0> (-2.2e57)*3 overflow must panic, not wrap");
    }
}
