// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Minimal in-tree wide-integer arithmetic for the correctly-rounded
//! strict transcendentals.
//!
//! The strict `ln` / `exp` / trig family must be accurate to within
//! 0.5 ULP of the exact result (the IEEE-754 round-to-nearest
//! contract). Achieving that for `D38<SCALE>` at the larger scales
//! means evaluating the series in a fixed-point intermediate with
//! *guard digits* beyond `SCALE` — which at `SCALE` near 38 overflows
//! `i128`. This module provides exactly the wide-integer primitives the
//! guard-digit evaluator needs and nothing more:
//!
//! - `U256` (`[u128; 2]`) and `U512` (`[u128; 4]`), little-endian limbs;
//! - full 256x256 -> 512 multiply;
//! - 512 / 256 -> quotient division (binary shift-subtract);
//! - a sign-magnitude `Fixed` value: a signed 256-bit magnitude
//! interpreted at a fixed decimal working scale, with the
//! `(a*b)/10^W` and `(a*10^W)/b` rescaling operations the series
//! evaluator runs on.
//!
//! This is deliberately *not* a general big-integer type. It is the
//! smallest surface that makes the transcendentals correctly rounded,
//! is `no_std`, and is shared by every feature configuration. (the wide integer
//! is only available behind the wide-tier features, so it cannot be
//! used here.)

/// Little-endian 256-bit unsigned integer (`limbs[0]` least significant).
pub(crate) type U256 = [u128; 2];
/// Little-endian 512-bit unsigned integer.
pub(crate) type U512 = [u128; 4];

/// Full 128x128 -> 256 unsigned product, `(high, low)`.
#[inline]
const fn mul_128(lhs: u128, rhs: u128) -> (u128, u128) {
    let (lhs_hi, lhs_lo) = (lhs >> 64, lhs & u64::MAX as u128);
    let (rhs_hi, rhs_lo) = (rhs >> 64, rhs & u64::MAX as u128);
    let (mid, carry1) = (lhs_lo * rhs_hi).overflowing_add(lhs_hi * rhs_lo);
    let (low, carry2) = (lhs_lo * rhs_lo).overflowing_add(mid << 64);
    let high = lhs_hi * rhs_hi + (mid >> 64) + ((carry1 as u128) << 64) + carry2 as u128;
    (high, low)
}

/// `lhs + rhs` for 256-bit values; returns `(sum, carry_out)`.
#[inline]
fn add_u256(lhs: U256, rhs: U256) -> (U256, bool) {
    let (lo, carry0) = lhs[0].overflowing_add(rhs[0]);
    let (hi_partial, carry1) = lhs[1].overflowing_add(rhs[1]);
    let (hi, carry2) = hi_partial.overflowing_add(u128::from(carry0));
    ([lo, hi], carry1 || carry2)
}

/// `lhs - rhs` for 256-bit values; caller guarantees `lhs >= rhs`.
#[inline]
fn sub_u256(lhs: U256, rhs: U256) -> U256 {
    let (lo, borrow) = lhs[0].overflowing_sub(rhs[0]);
    let hi = lhs[1].wrapping_sub(rhs[1]).wrapping_sub(u128::from(borrow));
    [lo, hi]
}

/// `lhs >= rhs` for 256-bit values.
#[inline]
fn ge_u256(lhs: U256, rhs: U256) -> bool {
    lhs[1] > rhs[1] || (lhs[1] == rhs[1] && lhs[0] >= rhs[0])
}

/// `value == 0` for a 256-bit value.
#[inline]
fn is_zero_u256(value: U256) -> bool {
    value[0] == 0 && value[1] == 0
}

/// Full 256x128 -> 384 unsigned product, returned in U512 form
/// (top limb is always 0).
///
/// Specialisation of [`mul_u256`] for the common case where one
/// operand is a 128-bit constant — the four-sub-product schoolbook
/// collapses to two because two of the partial products with the
/// zero high limb are themselves zero.
#[inline]
fn mul_u256_by_u128(value: U256, multiplier: u128) -> U512 {
    let (p0_hi, p0_lo) = mul_128(value[0], multiplier);
    let (p1_hi, p1_lo) = mul_128(value[1], multiplier);
    let limb0 = p0_lo;
    let (limb1, carry1) = p0_hi.overflowing_add(p1_lo);
    let limb2 = p1_hi + u128::from(carry1);
    [limb0, limb1, limb2, 0]
}

/// Full 256x256 -> 512 unsigned product.
pub(crate) fn mul_u256(lhs: U256, rhs: U256) -> U512 {
    // lhs = a0 + a1·B, rhs = b0 + b1·B, B = 2^128.
    let (p00_hi, p00_lo) = mul_128(lhs[0], rhs[0]);
    let (p01_hi, p01_lo) = mul_128(lhs[0], rhs[1]);
    let (p10_hi, p10_lo) = mul_128(lhs[1], rhs[0]);
    let (p11_hi, p11_lo) = mul_128(lhs[1], rhs[1]);

    // limb0 = p00_lo
    let limb0 = p00_lo;
    // limb1 = p00_hi + p01_lo + p10_lo
    let (sum1, carry1a) = p00_hi.overflowing_add(p01_lo);
    let (limb1, carry1b) = sum1.overflowing_add(p10_lo);
    let carry1 = u128::from(carry1a) + u128::from(carry1b);
    // limb2 = p01_hi + p10_hi + p11_lo + carry1
    let (sum2, carry2a) = p01_hi.overflowing_add(p10_hi);
    let (sum2b, carry2b) = sum2.overflowing_add(p11_lo);
    let (limb2, carry2c) = sum2b.overflowing_add(carry1);
    let carry2 = u128::from(carry2a) + u128::from(carry2b) + u128::from(carry2c);
    // limb3 = p11_hi + carry2
    let limb3 = p11_hi + carry2;
    [limb0, limb1, limb2, limb3]
}

/// Quotient `numerator / divisor` for a 512-bit dividend and a divisor
/// that fits in a single 64-bit word.
///
/// Schoolbook long division in base `2^64`: each step divides a
/// 128-bit `(remainder, limb)` pair by the word divisor with one
/// hardware division. Far cheaper than the general bit loop, and it
/// covers every `10^scale` divisor for `scale <= 19` — the common
/// decimal multiply path.
fn div_u512_by_word(numerator: U512, divisor: u64) -> U512 {
    let divisor_u128 = u128::from(divisor);
    let mut limbs = [0u64; 8];
    for i in 0..4 {
        limbs[i << 1] = numerator[i] as u64;
        limbs[(i << 1) | 1] = (numerator[i] >> 64) as u64;
    }
    let mut remainder: u128 = 0;
    let mut i = 8;
    while i > 0 {
        i -= 1;
        let current = (remainder << 64) | u128::from(limbs[i]);
        limbs[i] = (current / divisor_u128) as u64;
        remainder = current % divisor_u128;
    }
    let mut out = [0u128; 4];
    for i in 0..4 {
        out[i] = u128::from(limbs[i << 1]) | (u128::from(limbs[(i << 1) | 1]) << 64);
    }
    out
}

/// Quotient `num / 10^w` for a 512-bit dividend, returned as a 256-bit
/// value (the caller must ensure the true quotient fits — every site
/// in this file does).
///
/// Reuses the Möller-Granlund magic constants and the 2-by-1 kernel
/// from [`crate::algos::support::mg_divide`]: instead of the
/// 256-shift-subtract bit loop the generic `div_u512_by_u256` falls
/// back to once the divisor exceeds `u64::MAX`, we walk the dividend
/// in u128 limbs and apply the MG kernel once per limb. For
/// `w <= 38` this collapses a ~256-iteration bit loop into 4 MG
/// 2-by-1 calls.
///
/// For `w > 38` the divisor itself exceeds a single u128 limb and
/// the simple per-limb MG sweep no longer applies; we fall back to
/// the generic `div_u512_by_u256` bit loop. The fast path covers
/// the `D38<SCALE>` native `Fixed` working scales `SCALE + 30` for
/// `SCALE in 0..=8` — exactly the scales not borrowed to D57. The
/// embedded-constant rescales (`wide_pi`, `wide_ln2`, …) divide by
/// `10^(75 - w)` which is also < 38 for any caller-relevant `w`.
#[inline]
fn div_u512_by_pow10(numerator: U512, working_scale: u32) -> U256 {
    if working_scale == 0 {
        return [numerator[0], numerator[1]];
    }
    if working_scale <= 38 {
        return div_u512_by_pow10_small(numerator, working_scale as usize);
    }
    if working_scale <= 76 {
        // Chained truncating divide: floor(num / 10^w) ==
        // floor(floor(num / 10^38) / 10^(w-38)) for integer w > 38.
        // The first pass shrinks the dividend by ~126 bits, leaving
        // at most ~386 bits — we keep the full 4 u128 limbs across
        // the chain to be safe.
        let first_pass = div_u512_by_pow10_small_full(numerator, 38);
        return div_u512_by_pow10_small(first_pass, (working_scale - 38) as usize);
    }
    // Fallback for w > 76 — not used by any caller in this module.
    let divisor = Fixed::pow10(working_scale);
    let quotient = div_u512_by_u256(numerator, divisor);
    [quotient[0], quotient[1]]
}

/// Same as [`div_u512_by_pow10_small`] but returns all four u128
/// quotient limbs (no narrowing to U256). Used as the first pass of
/// the `w > 38` chain where the intermediate dividend may span more
/// than 256 bits.
#[inline]
fn div_u512_by_pow10_small_full(numerator: U512, scale_idx: usize) -> U512 {
    debug_assert!((1..=38).contains(&scale_idx));
    let pow10_divisor = crate::algos::support::mg_divide::POW10_U128[scale_idx];
    let mut remainder: u128 = 0;
    let (quotient3, remainder3) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[3], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10_small_full: invariant violated");
    remainder = remainder3;
    let (quotient2, remainder2) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[2], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10_small_full: invariant violated");
    remainder = remainder2;
    let (quotient1, remainder1) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[1], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10_small_full: invariant violated");
    remainder = remainder1;
    let (quotient0, _remainder0) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[0], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10_small_full: invariant violated");
    [quotient0, quotient1, quotient2, quotient3]
}

/// `num / 10^scale_idx` where `1 <= scale_idx <= 38`, returning the
/// 256-bit quotient. The divisor fits a single u128 limb, so one MG
/// 2-by-1 step per dividend u128 limb suffices.
#[inline]
fn div_u512_by_pow10_small(numerator: U512, scale_idx: usize) -> U256 {
    debug_assert!((1..=38).contains(&scale_idx));
    let pow10_divisor = crate::algos::support::mg_divide::POW10_U128[scale_idx];
    // Walk dividend top-down (most-significant limb first), tracking a
    // running remainder. Quotient limbs go bottom-up; the high two
    // quotient limbs are discarded (they're always 0 for the working-
    // scale invariants in this module — the radicand fits 256 bits
    // after the divide).
    let mut remainder: u128 = 0;
    // limb 3 (highest)
    let (quotient3, remainder3) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[3], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10: invariant rem < exp violated");
    debug_assert!(
        quotient3 == 0,
        "div_u512_by_pow10: quotient overflows 256 bits — caller invariant violated"
    );
    remainder = remainder3;
    // limb 2
    let (quotient2, remainder2) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[2], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10: invariant rem < exp violated");
    debug_assert!(
        quotient2 == 0,
        "div_u512_by_pow10: quotient overflows 256 bits — caller invariant violated"
    );
    remainder = remainder2;
    // limb 1
    let (out_hi, remainder1) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[1], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10: invariant rem < exp violated");
    remainder = remainder1;
    // limb 0
    let (out_lo, _remainder0) = crate::algos::support::mg_divide::divmod_pow10_2word(
        remainder, numerator[0], pow10_divisor, scale_idx)
        .expect("div_u512_by_pow10: invariant rem < exp violated");
    [out_lo, out_hi]
}

/// Quotient `numerator / divisor` where the numerator is 512-bit and the
/// divisor 256-bit.
///
/// Returned as `U512`; for every use in this crate the true quotient
/// fits in 256 bits, but the wider return type keeps the routine
/// general and the high limbs are simply zero in practice.
///
/// Divisors above `u64::MAX` route through the int layer's
/// divisor-shape matcher
/// ([`crate::algos::support::mg_divide::div_rem_via_int_layer`]) —
/// Knuth Algorithm D at these limb counts — replacing the former
/// shift-subtract bit loop (word-serial O(m·n) limb steps instead of
/// one full-width shift+compare+subtract per dividend bit). This is
/// the divide every `Fixed::div` by a transcendental constant (e.g.
/// `to_degrees`' full-precision pi) and every `isqrt_u512` Newton
/// iteration lands on.
pub(crate) fn div_u512_by_u256(numerator: U512, divisor: U256) -> U512 {
    debug_assert!(!(divisor[0] == 0 && divisor[1] == 0), "division by zero");
    // Fast path: when both the dividend and divisor fit in a single
    // 128-bit word, the hardware divide is exact and far cheaper than
    // any multi-limb engine. This covers the overwhelmingly common case
    // of moderate-magnitude decimal multiply/divide at small scales.
    if numerator[1] == 0 && numerator[2] == 0 && numerator[3] == 0 && divisor[1] == 0 {
        return [numerator[0] / divisor[0], 0, 0, 0];
    }
    // Word-divisor path: a wide dividend divided by a divisor that
    // fits in 64 bits (every `10^scale` for `scale <= 19`).
    if divisor[1] == 0 && divisor[0] <= u128::from(u64::MAX) {
        return div_u512_by_word(numerator, divisor[0] as u64);
    }
    // Wide divisor (`u64::MAX < divisor < 2^256`): the int layer's
    // word-serial engines.
    let num = [
        numerator[0] as u64,
        (numerator[0] >> 64) as u64,
        numerator[1] as u64,
        (numerator[1] >> 64) as u64,
        numerator[2] as u64,
        (numerator[2] >> 64) as u64,
        numerator[3] as u64,
        (numerator[3] >> 64) as u64,
    ];
    let den = [
        divisor[0] as u64,
        (divisor[0] >> 64) as u64,
        divisor[1] as u64,
        (divisor[1] >> 64) as u64,
    ];
    let mut quot = [0u64; 8];
    let mut rem = [0u64; 4];
    crate::algos::support::mg_divide::div_rem_via_int_layer(&num, &den, &mut quot, &mut rem);
    [
        u128::from(quot[0]) | (u128::from(quot[1]) << 64),
        u128::from(quot[2]) | (u128::from(quot[3]) << 64),
        u128::from(quot[4]) | (u128::from(quot[5]) << 64),
        u128::from(quot[6]) | (u128::from(quot[7]) << 64),
    ]
}

/// A signed value held as a 256-bit magnitude interpreted at a fixed
/// decimal working scale `W` — i.e. the logical value is
/// `(if negative { -1 } else { 1 }) * magnitude / 10^W`.
///
/// The working scale is not stored on the value; every operation that
/// depends on it takes `w` explicitly, so a single `Fixed` is only
/// meaningful alongside the `w` it was produced at. The transcendental
/// evaluators pick one `w` for an entire computation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Fixed {
    /// `true` if the logical value is negative. Zero is non-negative.
    pub(crate) negative: bool,
    /// Unsigned magnitude, `value * 10^W`.
    pub(crate) mag: U256,
}

impl Fixed {
    /// The additive identity.
    pub(crate) const ZERO: Fixed = Fixed {
        negative: false,
        mag: [0, 0],
    };

    /// Constructs from a non-negative `u128` magnitude already scaled to
    /// the working scale.
    #[inline]
    pub(crate) fn from_u128_mag(mag: u128, negative: bool) -> Fixed {
        Fixed {
            negative: negative && mag != 0,
            mag: [mag, 0],
        }
    }

    /// `10^exp` as a `Fixed` magnitude (for `exp <= 76`, which always
    /// fits 256 bits). Used to build the working-scale `ONE` and the
    /// embedded constants.
    pub(crate) fn pow10(exp: u32) -> U256 {
        // 10^exp for exp <= 38 fits u128; above that, split.
        if exp <= 38 {
            [10u128.pow(exp), 0]
        } else {
            let lo38 = 10u128.pow(38);
            let rest = 10u128.pow(exp - 38);
            let (hi, lo) = mul_128(lo38, rest);
            [lo, hi]
        }
    }

    /// `true` if the logical value is exactly zero.
    #[inline]
    pub(crate) fn is_zero(self) -> bool {
        is_zero_u256(self.mag)
    }

    /// Builds a non-negative value from a 64-digit decimal constant
    /// supplied as two 32-digit halves: `value = hi * 10^32 + lo`. Used
    /// to embed the bedrock transcendental constants (ln 2, ln 10, e, …)
    /// at a 64-digit working scale.
    pub(crate) fn from_decimal_split(hi: u128, lo: u128) -> Fixed {
        let ten_pow_32 = 10u128.pow(32);
        let (carry, low) = mul_128(hi, ten_pow_32);
        let (mag, _c) = add_u256([low, carry], [lo, 0]);
        Fixed {
            negative: false,
            mag,
        }
    }

    /// Truncating change of working scale from `from_w` down to `to_w`
    /// (`to_w <= from_w`): divides the magnitude by `10^(from_w-to_w)`.
    ///
    /// `from_w >= to_w` is a kernel-boundary invariant: this is the
    /// "down" path. A debug-assert catches callers that accidentally
    /// invert the arguments — without it, `from_w - to_w` wraps as
    /// `u32` and the downstream `Fixed::pow10` overflows with a
    /// confusing message far from the actual site. See `wide_ln2` /
    /// `wide_ln10` / `wide_pi` for the rescale-down call shape.
    pub(crate) fn rescale_down(self, from_w: u32, to_w: u32) -> Fixed {
        debug_assert!(
            from_w >= to_w,
            "Fixed::rescale_down: from_w ({from_w}) must be >= to_w ({to_w}); \
             this is the rescale-DOWN path. Inverted arguments wrap u32 and \
             trip a far-away pow10 overflow."
        );
        if from_w == to_w {
            return self;
        }
        let shift = from_w - to_w;
        let (quotient, _remainder) =
            divmod_u256_by_pow10(self.mag, Fixed::pow10(shift), shift);
        Fixed {
            negative: self.negative && !is_zero_u256(quotient),
            mag: quotient,
        }
    }

    /// Multiplies the magnitude by a small unsigned `multiplier`. The
    /// caller guarantees the result stays below `2^256`.
    pub(crate) fn mul_u128(self, multiplier: u128) -> Fixed {
        // self.mag * k: (mag_lo + mag_hi*B) * k.
        let (lo_hi, lo_lo) = mul_128(self.mag[0], multiplier);
        let (_hi_hi, hi_lo) = mul_128(self.mag[1], multiplier);
        let (mag1, _c) = hi_lo.overflowing_add(lo_hi);
        let mag = [lo_lo, mag1];
        Fixed {
            negative: self.negative && !is_zero_u256(mag),
            mag,
        }
    }

    /// `|self| >= |rhs|` — magnitude comparison.
    #[inline]
    pub(crate) fn ge_mag(self, rhs: Fixed) -> bool {
        ge_u256(self.mag, rhs.mag)
    }

    /// `self * 2` (magnitude doubled). Caller guarantees no overflow.
    #[inline]
    pub(crate) fn double(self) -> Fixed {
        let mag = [
            (self.mag[0] << 1),
            (self.mag[1] << 1) | (self.mag[0] >> 127),
        ];
        Fixed {
            negative: self.negative,
            mag,
        }
    }

    /// `self / 2`, truncating (magnitude halved).
    #[inline]
    pub(crate) fn halve(self) -> Fixed {
        Fixed {
            negative: self.negative,
            mag: halve_u256(self.mag),
        }
    }

    /// Bit length of the magnitude (0 for zero, else `floor(log2)+1`).
    #[inline]
    pub(crate) fn bit_length(self) -> u32 {
        if self.mag[1] != 0 {
            256 - self.mag[1].leading_zeros()
        } else {
            128 - self.mag[0].leading_zeros()
        }
    }

    /// `self << shift` (magnitude shifted left). Caller guarantees no
    /// significant bits are lost (`bit_length + shift <= 256`).
    pub(crate) fn shl(self, shift: u32) -> Fixed {
        if shift == 0 {
            return self;
        }
        let mag = if shift >= 128 {
            [0, self.mag[0] << (shift - 128)]
        } else {
            [
                self.mag[0] << shift,
                (self.mag[1] << shift) | (self.mag[0] >> (128 - shift)),
            ]
        };
        Fixed {
            negative: self.negative,
            mag,
        }
    }

    /// `self >> shift` (magnitude shifted right, truncating).
    pub(crate) fn shr(self, shift: u32) -> Fixed {
        if shift == 0 {
            return self;
        }
        // A 256-bit magnitude shifted right by its full width (or more) is zero;
        // this also guards the per-limb shifts below, where `mag[1] >> (n - 128)`
        // would otherwise overflow the u128 once `n >= 256` (e.g. a deep-underflow
        // `exp(-222)` reassembling `2^k` with `k ≈ -320`).
        if shift >= 256 {
            return Fixed {
                negative: false,
                mag: [0, 0],
            };
        }
        let mag = if shift >= 128 {
            [self.mag[1] >> (shift - 128), 0]
        } else {
            [
                (self.mag[0] >> shift) | (self.mag[1] << (128 - shift)),
                self.mag[1] >> shift,
            ]
        };
        Fixed {
            negative: self.negative && !is_zero_u256(mag),
            mag,
        }
    }

    /// Adds two values at the same working scale.
    pub(crate) fn add(self, rhs: Fixed) -> Fixed {
        if self.negative == rhs.negative {
            let (sum, _carry) = add_u256(self.mag, rhs.mag);
            // The transcendental evaluators keep magnitudes well below
            // 2^256, so `_carry` is always false here.
            Fixed {
                negative: self.negative,
                mag: sum,
            }
        } else {
            // Opposite signs: subtract the smaller magnitude.
            match (
                ge_u256(self.mag, rhs.mag),
                is_zero_u256(self.mag) && is_zero_u256(rhs.mag),
            ) {
                (_, true) => Fixed::ZERO,
                (true, _) => {
                    let mag = sub_u256(self.mag, rhs.mag);
                    Fixed {
                        negative: self.negative && !is_zero_u256(mag),
                        mag,
                    }
                }
                (false, _) => {
                    let mag = sub_u256(rhs.mag, self.mag);
                    Fixed {
                        negative: rhs.negative && !is_zero_u256(mag),
                        mag,
                    }
                }
            }
        }
    }

    /// Negates the value.
    #[inline]
    pub(crate) fn neg(self) -> Fixed {
        Fixed {
            negative: !self.negative && !self.is_zero(),
            mag: self.mag,
        }
    }

    /// Subtracts `rhs` from `self` at the same working scale.
    #[inline]
    pub(crate) fn sub(self, rhs: Fixed) -> Fixed {
        self.add(rhs.neg())
    }

    /// Multiplies two working-scale values: `(self * rhs) / 10^w`,
    /// truncating toward zero. Both magnitudes must be below `10^w *
    /// 2^128` so the 512-bit product divides back into 256 bits.
    pub(crate) fn mul(self, rhs: Fixed, working_scale: u32) -> Fixed {
        let product = mul_u256(self.mag, rhs.mag);
        // Specialised `pow10(w)` divisor path. The general
        // `div_u512_by_u256` falls back to a 256-iteration shift /
        // subtract bit loop once the divisor exceeds `u64::MAX`
        // (i.e. `w >= 20`); for power-of-10 divisors we have the
        // Möller-Granlund magic table in `crate::algos::support::mg_divide`,
        // which collapses one 2-limb step into a handful of u128
        // multiplies. Chain it across the 512-bit dividend in u128
        // limbs to avoid the bit loop entirely.
        let quotient_mag = div_u512_by_pow10(product, working_scale);
        Fixed {
            negative: (self.negative ^ rhs.negative)
                && !(quotient_mag[0] == 0 && quotient_mag[1] == 0),
            mag: quotient_mag,
        }
    }

    /// Divides by an unsigned `u128` `divisor`, truncating toward
    /// zero. The divisor must be non-zero.
    pub(crate) fn div_small(self, divisor: u128) -> Fixed {
        debug_assert!(divisor != 0, "division by zero");
        // Fast path: divisor fits a single u64 — schoolbook base-2^64
        // long division costs four hardware u128/u64 divides (one per
        // 64-bit limb) instead of the 256-iteration bit loop below.
        // Every Taylor / artanh series in this crate calls
        // `div_small(2*k+1)` or `div_small((2*k)*(2*k+1))` with
        // k < 400, so the divisor is < ~1.3 million ≪ u64::MAX and
        // this fast path always fires from those sites.
        if divisor <= u64::MAX as u128 {
            let divisor_u64 = divisor as u64;
            let divisor_u128 = divisor; // already u128, avoids reconvert in the loop
            let limbs: [u64; 4] = [
                self.mag[0] as u64,
                (self.mag[0] >> 64) as u64,
                self.mag[1] as u64,
                (self.mag[1] >> 64) as u64,
            ];
            let mut out = [0u64; 4];
            let mut remainder: u128 = 0;
            // Top-down schoolbook divide in base 2^64. Each step:
            //   (rem << 64 | limb) / d  →  64-bit quotient + 64-bit rem
            let current3 = (remainder << 64) | u128::from(limbs[3]);
            out[3] = (current3 / divisor_u128) as u64;
            remainder = current3 - u128::from(out[3]) * divisor_u128;
            let current2 = (remainder << 64) | u128::from(limbs[2]);
            out[2] = (current2 / divisor_u128) as u64;
            remainder = current2 - u128::from(out[2]) * divisor_u128;
            let current1 = (remainder << 64) | u128::from(limbs[1]);
            out[1] = (current1 / divisor_u128) as u64;
            remainder = current1 - u128::from(out[1]) * divisor_u128;
            let current0 = (remainder << 64) | u128::from(limbs[0]);
            out[0] = (current0 / divisor_u128) as u64;
            let _ = divisor_u64;
            let q_lo = u128::from(out[0]) | (u128::from(out[1]) << 64);
            let q_hi = u128::from(out[2]) | (u128::from(out[3]) << 64);
            return Fixed {
                negative: self.negative && !(q_lo == 0 && q_hi == 0),
                mag: [q_lo, q_hi],
            };
        }
        // Wide divisor (`u64::MAX < divisor < 2^128`): the int layer's
        // divisor-shape matcher — Knuth Algorithm D at these limb
        // counts — instead of the former 256-iteration bit loop.
        let num = [
            self.mag[0] as u64,
            (self.mag[0] >> 64) as u64,
            self.mag[1] as u64,
            (self.mag[1] >> 64) as u64,
        ];
        let den = [divisor as u64, (divisor >> 64) as u64];
        let mut quot = [0u64; 4];
        let mut rem = [0u64; 2];
        crate::algos::support::mg_divide::div_rem_via_int_layer(&num, &den, &mut quot, &mut rem);
        let q_lo = u128::from(quot[0]) | (u128::from(quot[1]) << 64);
        let q_hi = u128::from(quot[2]) | (u128::from(quot[3]) << 64);
        Fixed {
            negative: self.negative && !(q_lo == 0 && q_hi == 0),
            mag: [q_lo, q_hi],
        }
    }

    /// Square root at working scale `w`: returns `√self` at scale `w`,
    /// truncating toward zero. `self` must be non-negative; a negative
    /// value's magnitude is used (callers guard the sign themselves).
    ///
    /// `√(mag/10^w) · 10^w = √(mag · 10^w)` — the radicand is formed as
    /// a 512-bit value and its integer square root taken exactly. The
    /// caller's working values keep `mag · 10^w < 2^512`.
    pub(crate) fn sqrt(self, working_scale: u32) -> Fixed {
        // For w <= 38 the multiplier fits a single u128; the
        // collapsed 256x128 multiply skips the two zero sub-products
        // of the general 256x256 schoolbook.
        let radicand = if working_scale <= 38 {
            mul_u256_by_u128(
                self.mag,
                crate::algos::support::mg_divide::POW10_U128[working_scale as usize])
        } else {
            mul_u256(self.mag, Fixed::pow10(working_scale))
        };
        Fixed {
            negative: false,
            mag: isqrt_u512(radicand),
        }
    }

    /// Divides by another working-scale value: `(self * 10^w) / rhs`,
    /// truncating toward zero. `rhs` must be non-zero. `self * 10^w`
    /// must fit 512 bits (it always does for the evaluators' inputs).
    pub(crate) fn div(self, rhs: Fixed, working_scale: u32) -> Fixed {
        // Build the numerator `self.mag * 10^w` as a 512-bit value.
        // The single-u128-limb multiplier specialisation collapses
        // half the sub-products when `w <= 38`; outside that band
        // we go through the general 256x256 schoolbook.
        let scaled = if working_scale <= 38 {
            mul_u256_by_u128(
                self.mag,
                crate::algos::support::mg_divide::POW10_U128[working_scale as usize])
        } else {
            mul_u256(self.mag, Fixed::pow10(working_scale))
        };
        let quotient = div_u512_by_u256(scaled, rhs.mag);
        Fixed {
            negative: (self.negative ^ rhs.negative)
                && !(quotient[0] == 0 && quotient[1] == 0),
            mag: [quotient[0], quotient[1]],
        }
    }

    /// Rounds the working-scale magnitude to a narrower decimal scale
    /// `target` (`target <= w`) using the supplied [`RoundingMode`] and
    /// returns the result as a signed `i128` raw storage value. Used to
    /// land a guard-digit computation back on the caller's `D38<SCALE>`.
    ///
    /// For the crate-default rounding mode, pass
    /// [`crate::support::rounding::DEFAULT_ROUNDING_MODE`].
    ///
    /// Returns `None` if the rounded magnitude does not fit `i128`.
    ///
    /// `#[inline]` so callers that thread a const mode (the strict
    /// path's `DEFAULT_ROUNDING_MODE`) get the `should_bump` match
    /// folded at the call site rather than dispatching at runtime.
    ///
    /// [`RoundingMode`]: crate::support::rounding::RoundingMode
    #[inline]
    pub(crate) fn round_to_i128_with(
        self,
        working_scale: u32,
        target: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Option<i128> {
        self.round_to_i128_with_exact(working_scale, target, mode)
            .map(|(value, _exact)| value)
    }

    /// Like [`round_to_i128_with`](Self::round_to_i128_with) but also reports
    /// whether the sub-target working residual was exactly zero — i.e. whether
    /// the value sits *exactly* on the storage grid line at this working scale
    /// `w`. (The strict narrow terminals now decide grid/half proximity through
    /// [`round_to_i128_clear_of_tie`](Self::round_to_i128_clear_of_tie) + the
    /// Ziv walkers; the exactness flag remains for callers that only need the
    /// on-grid signal.)
    #[inline]
    pub(crate) fn round_to_i128_with_exact(
        self,
        working_scale: u32,
        target: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Option<(i128, bool)> {
        let shift = working_scale - target;
        if shift == 0 {
            // No rounding; just narrow. An all-zero shift carries no residual,
            // so the value is exactly on grid by construction.
            if self.mag[1] != 0 {
                return None;
            }
            let magnitude = self.mag[0];
            let value = if self.negative {
                if magnitude > 1u128 << 127 {
                    return None;
                }
                (magnitude as i128).wrapping_neg()
            } else if magnitude > i128::MAX as u128 {
                return None;
            } else {
                magnitude as i128
            };
            return Some((value, true));
        }
        let divisor = Fixed::pow10(shift);
        let (quotient, remainder) = divmod_u256_by_pow10(self.mag, divisor, shift);
        let exact = is_zero_u256(remainder);
        self.finish_round_to_i128(quotient, remainder, divisor, mode)
            .map(|value| (value, exact))
    }

    /// Shared rounding tail of [`round_to_i128_with_exact`] /
    /// [`round_to_i128_clear_of_tie`]: folds the split
    /// `(quotient, remainder)` of the magnitude at `divisor = 10^shift` to
    /// the signed `i128` storage value under `mode`. `None` = does not fit
    /// `i128`.
    ///
    /// [`round_to_i128_with_exact`]: Self::round_to_i128_with_exact
    /// [`round_to_i128_clear_of_tie`]: Self::round_to_i128_clear_of_tie
    #[inline]
    fn finish_round_to_i128(
        self,
        quotient: U256,
        remainder: U256,
        divisor: U256,
        mode: crate::support::rounding::RoundingMode,
    ) -> Option<i128> {
        let rounded = if is_zero_u256(remainder) {
            quotient
        } else {
            // |r| is r (already a magnitude); complement = divisor - r.
            let complement = sub_u256(divisor, remainder);
            let remainder_cmp = cmp_u256(remainder, complement);
            // `quotient` is a u128-limb magnitude (`U256`), so the last decimal
            // digit needs both limbs, not just the low one.
            let q_mod_10 = crate::support::rounding::limbs_u128_mod_10(&quotient);
            let result_is_positive = !self.negative;
            if crate::support::rounding::should_bump(
                mode, remainder_cmp, q_mod_10, result_is_positive)
            {
                add_u256(quotient, [1, 0]).0
            } else {
                quotient
            }
        };
        if rounded[1] != 0 {
            return None;
        }
        let magnitude = rounded[0];
        let value = if self.negative {
            if magnitude > 1u128 << 127 {
                return None;
            }
            (magnitude as i128).wrapping_neg()
        } else if magnitude > i128::MAX as u128 {
            return None;
        } else {
            magnitude as i128
        };
        Some(value)
    }

    /// Single-shot narrowing with a NEAR-TIE escape hatch — the `Fixed`
    /// sibling of `wide_trig_core::round_to_storage_clear_of_tie_g`.
    /// Rounds exactly as [`round_to_i128_with`](Self::round_to_i128_with)
    /// PROVIDED the sub-storage residual is clear of the mode's deciding
    /// boundary (the half-ULP line for the nearest modes, the grid line
    /// for the directed ones) by more than the near-tie band
    /// (`divisor/1000`, the shared Ziv escalate trigger). Returns the
    /// outer `None` when the residual sits inside the band: the value's
    /// TRUE deciding digit may then lie below `w`'s resolution (an exact
    /// rational Taylor partial landing exactly ON a boundary with the
    /// transcendental tail below the fixed working scale — the narrow
    /// analogue of the wide `asin(3e-60)` family), and the caller must
    /// escalate through the Ziv walker instead of concluding from this
    /// single shot. The inner `Option` is the usual does-not-fit-`i128`
    /// signal. One `div_rem` — the clear path costs what the plain
    /// narrowing cost.
    #[inline]
    pub(crate) fn round_to_i128_clear_of_tie(
        self,
        working_scale: u32,
        target: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> Option<Option<i128>> {
        let shift = working_scale - target;
        if shift < 3 {
            // Degenerate guard: no band to measure — round directly.
            return Some(self.round_to_i128_with(working_scale, target, mode));
        }
        let divisor = Fixed::pow10(shift);
        let (quotient, remainder) = divmod_u256_by_pow10(self.mag, divisor, shift);
        let band = Fixed::pow10(shift - 3);
        let distance = if crate::support::rounding::is_nearest_mode(mode) {
            // Distance to the half-ULP boundary (divisor is even, the
            // halve is exact).
            let half = halve_u256(divisor);
            if ge_u256(half, remainder) {
                sub_u256(half, remainder)
            } else {
                sub_u256(remainder, half)
            }
        } else {
            // Distance to the grid line (zero or divisor side).
            let complement = sub_u256(divisor, remainder);
            if ge_u256(complement, remainder) { remainder } else { complement }
        };
        if !matches!(cmp_u256(distance, band), core::cmp::Ordering::Greater) {
            return None;
        }
        Some(self.finish_round_to_i128(quotient, remainder, divisor, mode))
    }
}

impl Fixed {
    /// Rounds the working-scale value to the nearest integer (ties away
    /// from zero) and returns it as `i128`. Used to find the `k` in the
    /// `exp` range reduction `v = k·ln(2) + s`; `|k|` is small there, so
    /// the result always fits.
    pub(crate) fn round_to_nearest_int(self, working_scale: u32) -> i128 {
        let divisor = Fixed::pow10(working_scale);
        let (quotient, remainder) =
            divmod_u256_by_pow10(self.mag, divisor, working_scale);
        let integer_magnitude = if ge_u256(remainder, halve_u256(divisor)) {
            add_u256(quotient, [1, 0]).0
        } else {
            quotient
        };
        let magnitude = integer_magnitude[0] as i128;
        if self.negative { -magnitude } else { magnitude }
    }
}

/// `floor(sqrt(n))` for an unsigned 512-bit value, via Newton's method.
///
/// The result fits `U256`. Callers in this crate keep the radicand below
/// `2^452` (a working-scale `mag · 10^w` with `w <= 68`), so the
/// initial overestimate and every iterate stay below `2^256`.
fn isqrt_u512(radicand: U512) -> U256 {
    if radicand == [0, 0, 0, 0] {
        return [0, 0];
    }
    // Bit length of the radicand.
    let bits = if radicand[3] != 0 {
        512 - radicand[3].leading_zeros()
    } else if radicand[2] != 0 {
        384 - radicand[2].leading_zeros()
    } else if radicand[1] != 0 {
        256 - radicand[1].leading_zeros()
    } else {
        128 - radicand[0].leading_zeros()
    };
    // Initial over-estimate from the shared seed library (the same
    // pattern as `mg_divide::isqrt_256` / `icbrt_384`): under `std` the
    // hardware `f64::sqrt` of the top 64 significant bits (~53 correct
    // bits → ~1-2 Newton iterations, each one a `div_u512_by_u256`),
    // under `no_std` the classical `2^ceil(bits/2)` — the seed this
    // function formerly hand-rolled on every build, costing ~bits/2
    // halvings' worth of extra Newton divides. Both bodies are
    // guaranteed over-estimates `>= sqrt(n)` (the load-bearing
    // invariant: the downward-monotone loop never under-runs, and every
    // `radicand / estimate` quotient stays within U256), converging to
    // the identical floor either way. The radicand stays below `2^452`
    // (fn doc), so the seed is below `2^227` and reads back from the
    // low four u64 limbs.
    let n_limbs = [
        radicand[0] as u64,
        (radicand[0] >> 64) as u64,
        radicand[1] as u64,
        (radicand[1] >> 64) as u64,
        radicand[2] as u64,
        (radicand[2] >> 64) as u64,
        radicand[3] as u64,
        (radicand[3] >> 64) as u64,
    ];
    let mut seed_limbs = [0u64; 8];
    crate::algo_x_support::seed::sqrt_seed(&n_limbs, bits, &mut seed_limbs);
    debug_assert!(
        seed_limbs[4] == 0 && seed_limbs[5] == 0 && seed_limbs[6] == 0 && seed_limbs[7] == 0,
        "isqrt_u512: seed exceeds U256 — radicand outside the < 2^452 contract"
    );
    let mut estimate: U256 = [
        u128::from(seed_limbs[0]) | (u128::from(seed_limbs[1]) << 64),
        u128::from(seed_limbs[2]) | (u128::from(seed_limbs[3]) << 64),
    ];
    loop {
        // quotient = radicand / estimate (fits U256 because estimate >= sqrt).
        let quotient = div_u512_by_u256(radicand, estimate);
        let quotient = [quotient[0], quotient[1]];
        // next_estimate = (estimate + quotient) / 2.
        let (sum, _carry) = add_u256(estimate, quotient);
        let next_estimate = halve_u256(sum);
        if ge_u256(next_estimate, estimate) {
            return estimate;
        }
        estimate = next_estimate;
    }
}

/// Bit length of a 256-bit value (`0` for zero, else `floor(log2)+1`).
#[inline]
fn bitlen_u256(value: U256) -> u32 {
    if value[1] != 0 {
        256 - value[1].leading_zeros()
    } else {
        128 - value[0].leading_zeros()
    }
}

/// `value << shift` for a 256-bit value (`shift < 256`).
#[inline]
fn shl_u256(value: U256, shift: u32) -> U256 {
    if shift == 0 {
        value
    } else if shift >= 128 {
        [0, value[0] << (shift - 128)]
    } else {
        [value[0] << shift, (value[1] << shift) | (value[0] >> (128 - shift))]
    }
}

/// `numerator / 10^w` and `numerator % 10^w` for a 256-bit dividend and a
/// `working_scale` in `1..=76`.
///
/// Uses the Möller-Granlund 2-by-1 magic kernel from
/// [`crate::algos::support::mg_divide`] when `w <= 38` (the divisor fits a
/// single u128 magic-table entry), collapsing the generic
/// `divmod_u256` ~256-iteration shift / subtract bit loop into two
/// MG calls. Falls back to the generic path for `w > 38` (divisor
/// exceeds u128, outside the MG magic table).
///
/// The fast path matches the divisor `[divisor]` the caller passes
/// in; `working_scale` and `divisor` must agree
/// (`divisor == Fixed::pow10(working_scale)`).
#[inline]
fn divmod_u256_by_pow10(numerator: U256, divisor: U256, working_scale: u32) -> (U256, U256) {
    if (1..=38).contains(&working_scale) {
        let pow10_divisor =
            crate::algos::support::mg_divide::POW10_U128[working_scale as usize];
        // Walk dividend top-down (limb 1, then limb 0).
        let (q_hi, remainder1) = crate::algos::support::mg_divide::divmod_pow10_2word(
            0, numerator[1], pow10_divisor, working_scale as usize)
            .expect("divmod_u256_by_pow10: invariant violated");
        let (q_lo, remainder0) = crate::algos::support::mg_divide::divmod_pow10_2word(
            remainder1, numerator[0], pow10_divisor, working_scale as usize)
            .expect("divmod_u256_by_pow10: invariant violated");
        // The remainder is `remainder0` (< exp ≤ u128); the high limb is 0.
        return ([q_lo, q_hi], [remainder0, 0]);
    }
    divmod_u256(numerator, divisor)
}

/// `numerator / divisor` and `numerator % divisor` for 256-bit values.
///
/// Binary shift-subtract long division, bounded by the dividend's
/// actual bit length rather than a fixed 256 iterations.
fn divmod_u256(numerator: U256, divisor: U256) -> (U256, U256) {
    debug_assert!(!is_zero_u256(divisor), "division by zero");
    // Fast path: both operands fit in a single 128-bit word.
    if numerator[1] == 0 && divisor[1] == 0 {
        return ([numerator[0] / divisor[0], 0], [numerator[0] % divisor[0], 0]);
    }
    let bits = bitlen_u256(numerator);
    if bits == 0 {
        return ([0, 0], [0, 0]);
    }
    let mut quotient: U256 = [0, 0];
    let mut remainder: U256 = [0, 0];
    let mut numerator = shl_u256(numerator, 256 - bits);
    let mut i = bits;
    while i > 0 {
        i -= 1;
        let bit = (numerator[1] >> 127) & 1;
        numerator[1] = (numerator[1] << 1) | (numerator[0] >> 127);
        numerator[0] <<= 1;
        remainder[1] = (remainder[1] << 1) | (remainder[0] >> 127);
        remainder[0] = (remainder[0] << 1) | bit;
        quotient[1] = (quotient[1] << 1) | (quotient[0] >> 127);
        quotient[0] <<= 1;
        if ge_u256(remainder, divisor) {
            remainder = sub_u256(remainder, divisor);
            quotient[0] |= 1;
        }
    }
    (quotient, remainder)
}

/// `value / 2` for a 256-bit value.
#[inline]
fn halve_u256(value: U256) -> U256 {
    [(value[0] >> 1) | (value[1] << 127), value[1] >> 1]
}

/// Three-way comparison of 256-bit values.
#[inline]
fn cmp_u256(lhs: U256, rhs: U256) -> core::cmp::Ordering {
    match lhs[1].cmp(&rhs[1]) {
        core::cmp::Ordering::Equal => lhs[0].cmp(&rhs[0]),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Bit-identity: int-layer-routed divides vs the former bit loops ──
    //
    // `div_u512_by_u256`'s wide-divisor tail and `div_small`'s
    // above-`u64::MAX` fallback were rewired to the int layer's engines
    // (`mg_divide::div_rem_via_int_layer`), and `isqrt_u512`'s seed
    // moved from a hand-rolled `2^ceil(bits/2)` to the shared seed
    // library. The references below are verbatim copies of the replaced
    // code; the sweeps prove bit-identity. Every rounding decision
    // upstream (all 8 modes) consumes only the quotient/remainder pair
    // these leaves produce, so leaf identity covers all 8 modes by
    // construction.

    /// SplitMix64 — deterministic pattern stream for the sweeps.
    fn mix(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn mix_u128(state: &mut u64) -> u128 {
        (u128::from(mix(state)) << 64) | u128::from(mix(state))
    }

    /// Verbatim copy of the deleted `bitlen_u512` (reference-loop helper).
    fn ref_bitlen_u512(value: U512) -> u32 {
        if value[3] != 0 {
            512 - value[3].leading_zeros()
        } else if value[2] != 0 {
            384 - value[2].leading_zeros()
        } else if value[1] != 0 {
            256 - value[1].leading_zeros()
        } else {
            128 - value[0].leading_zeros()
        }
    }

    /// Verbatim copy of the deleted `shl_u512` (reference-loop helper).
    fn ref_shl_u512(value: U512, shift: u32) -> U512 {
        if shift == 0 {
            return value;
        }
        let limb_offset = (shift / 128) as usize;
        let bit_offset = shift % 128;
        let mut out = [0u128; 4];
        if bit_offset == 0 {
            for i in (limb_offset..4).rev() {
                out[i] = value[i - limb_offset];
            }
        } else {
            for i in (limb_offset..4).rev() {
                let shifted_low = value[i - limb_offset] << bit_offset;
                let carry = if i - limb_offset == 0 {
                    0
                } else {
                    value[i - limb_offset - 1] >> (128 - bit_offset)
                };
                out[i] = shifted_low | carry;
            }
        }
        out
    }

    /// Verbatim copy of the replaced `div_u512_by_u256` (bit-loop tail,
    /// fast paths included so the whole domain is compared).
    fn div_u512_by_u256_reference(numerator: U512, divisor: U256) -> U512 {
        if numerator[1] == 0 && numerator[2] == 0 && numerator[3] == 0 && divisor[1] == 0 {
            return [numerator[0] / divisor[0], 0, 0, 0];
        }
        if divisor[1] == 0 && divisor[0] <= u128::from(u64::MAX) {
            return div_u512_by_word(numerator, divisor[0] as u64);
        }
        let bits = ref_bitlen_u512(numerator);
        if bits == 0 {
            return [0; 4];
        }
        let mut numerator = ref_shl_u512(numerator, 512 - bits);
        let mut quotient: U512 = [0; 4];
        let mut remainder: U256 = [0, 0];
        let mut i = bits;
        while i > 0 {
            i -= 1;
            let bit = (numerator[3] >> 127) & 1;
            numerator[3] = (numerator[3] << 1) | (numerator[2] >> 127);
            numerator[2] = (numerator[2] << 1) | (numerator[1] >> 127);
            numerator[1] = (numerator[1] << 1) | (numerator[0] >> 127);
            numerator[0] <<= 1;
            remainder[1] = (remainder[1] << 1) | (remainder[0] >> 127);
            remainder[0] = (remainder[0] << 1) | bit;
            quotient[3] = (quotient[3] << 1) | (quotient[2] >> 127);
            quotient[2] = (quotient[2] << 1) | (quotient[1] >> 127);
            quotient[1] = (quotient[1] << 1) | (quotient[0] >> 127);
            quotient[0] <<= 1;
            if ge_u256(remainder, divisor) {
                remainder = sub_u256(remainder, divisor);
                quotient[0] |= 1;
            }
        }
        quotient
    }

    /// Verbatim copy of `div_small`'s replaced above-`u64::MAX` bit loop.
    fn div_small_wide_reference(mag: U256, divisor: u128) -> U256 {
        let mut remainder: u128 = 0;
        let mut hi = mag[1];
        let mut lo = mag[0];
        let mut q_hi: u128 = 0;
        let mut q_lo: u128 = 0;
        let mut bit = 256;
        while bit > 0 {
            bit -= 1;
            let top = (hi >> 127) & 1;
            hi = (hi << 1) | (lo >> 127);
            lo <<= 1;
            remainder = (remainder << 1) | top;
            q_hi = (q_hi << 1) | (q_lo >> 127);
            q_lo <<= 1;
            if remainder >= divisor {
                remainder -= divisor;
                q_lo |= 1;
            }
        }
        [q_lo, q_hi]
    }

    /// Verbatim copy of the replaced `isqrt_u512` (hand-rolled 1-bit
    /// seed + the identical Newton loop).
    fn isqrt_u512_reference(radicand: U512) -> U256 {
        if radicand == [0, 0, 0, 0] {
            return [0, 0];
        }
        let bits = ref_bitlen_u512(radicand);
        let half_bits = bits.div_ceil(2);
        let mut estimate: U256 = if half_bits >= 128 {
            [0, 1u128 << (half_bits - 128)]
        } else {
            [1u128 << half_bits, 0]
        };
        loop {
            let quotient = div_u512_by_u256_reference(radicand, estimate);
            let quotient = [quotient[0], quotient[1]];
            let (sum, _carry) = add_u256(estimate, quotient);
            let next_estimate = halve_u256(sum);
            if ge_u256(next_estimate, estimate) {
                return estimate;
            }
            estimate = next_estimate;
        }
    }

    /// A pattern `U512` with roughly `limb_count` active u64 limbs.
    fn pattern_u512(state: &mut u64, limb_count: u32) -> U512 {
        let mut out = [0u128; 4];
        for half in 0..limb_count.min(8) {
            let limb = u128::from(mix(state));
            out[(half / 2) as usize] |= limb << (64 * (half % 2));
        }
        out
    }

    /// `div_u512_by_u256` (int-layer route above `u64::MAX`) is
    /// bit-identical to the replaced bit loop across dividend sizes
    /// (1-8 u64 limbs) x divisor sizes (1-4 u64 limbs).
    #[test]
    fn div_u512_by_u256_matches_bit_loop_reference() {
        let mut state = 0x0512_0256_u64;
        for num_limbs in 1..=8u32 {
            for den_limbs in 1..=4u32 {
                for _ in 0..60 {
                    let numerator = pattern_u512(&mut state, num_limbs);
                    let d512 = pattern_u512(&mut state, den_limbs);
                    let mut divisor: U256 = [d512[0], d512[1]];
                    if divisor == [0, 0] {
                        divisor[0] = 1;
                    }
                    // Reference equality only where the old loop is
                    // itself correct: its 256-bit remainder register
                    // overflows on `remainder << 1` once the divisor
                    // exceeds 2^255 (top bit of divisor[1] set), a
                    // domain no production caller reaches (`Fixed`
                    // magnitudes and `10^w`, `w <= 76`, stay below
                    // 2^255). The reconstruction sweep below proves the
                    // int-layer route correct there.
                    if divisor[1] >> 127 == 0 {
                        assert_eq!(
                            div_u512_by_u256(numerator, divisor),
                            div_u512_by_u256_reference(numerator, divisor),
                            "div_u512_by_u256 mismatch: n={numerator:?} d={divisor:?}"
                        );
                    }
                }
            }
        }
        // Ground truth by reconstruction over the wide band, top-bit
        // divisors included: n = d*q + r with r < d ⇒ result must be q.
        for wide_top in [false, true] {
            for _ in 0..200 {
                let quotient_in: U256 = [mix_u128(&mut state), 0];
                let divisor: U256 = if wide_top {
                    // divisor in (2^255, 2^256): the old loop's overflow
                    // domain.
                    [mix_u128(&mut state), mix_u128(&mut state) | (1 << 127)]
                } else {
                    [mix_u128(&mut state) | (1 << 127), mix_u128(&mut state) >> 64]
                };
                // r < d via r[1] < d[1] (d[1] != 0 in both variants
                // above ... the narrow variant needs d[1] >= 1).
                let divisor = [divisor[0], if divisor[1] == 0 { 1 } else { divisor[1] }];
                let remainder_in: U256 =
                    [mix_u128(&mut state), mix_u128(&mut state) % divisor[1]];
                let mut product = mul_u256(divisor, quotient_in);
                let (sum0, carry0) = product[0].overflowing_add(remainder_in[0]);
                product[0] = sum0;
                let (sum1, carry1a) = product[1].overflowing_add(remainder_in[1]);
                let (sum1, carry1b) = sum1.overflowing_add(u128::from(carry0));
                product[1] = sum1;
                let (sum2, carry2) =
                    product[2].overflowing_add(u128::from(carry1a) + u128::from(carry1b));
                product[2] = sum2;
                product[3] += u128::from(carry2);
                let result = div_u512_by_u256(product, divisor);
                assert_eq!(
                    [result[0], result[1]],
                    quotient_in,
                    "div_u512_by_u256 reconstruction: d={divisor:?}"
                );
                assert_eq!(result[2], 0);
                assert_eq!(result[3], 0);
            }
        }
    }

    /// `div_small`'s wide-divisor branch (int-layer route) is
    /// bit-identical to the replaced 256-iteration bit loop, magnitude
    /// and sign.
    #[test]
    fn div_small_wide_divisor_matches_bit_loop_reference() {
        let mut state = 0x0d15_0256_u64;
        // Reference equality on the old loop's correct half (divisor
        // <= 2^127): its u128 remainder register overflows on
        // `remainder << 1` above that, a domain no production caller
        // reaches (the series call `div_small` with divisors below
        // ~10^6). The reconstruction sweep below proves the int-layer
        // route correct there.
        let edge_divisors: [u128; 3] = [
            u128::from(u64::MAX) + 1,
            (1u128 << 96) + 12345,
            1u128 << 127,
        ];
        for divisor in edge_divisors {
            for _ in 0..40 {
                let mag: U256 = [mix_u128(&mut state), mix_u128(&mut state)];
                for negative in [false, true] {
                    let value = Fixed { negative, mag };
                    let expected_mag = div_small_wide_reference(mag, divisor);
                    let actual = value.div_small(divisor);
                    assert_eq!(
                        actual.mag, expected_mag,
                        "div_small mismatch: mag={mag:?} d={divisor}"
                    );
                    let expect_negative =
                        negative && !(expected_mag[0] == 0 && expected_mag[1] == 0);
                    assert_eq!(actual.negative, expect_negative);
                }
            }
        }
        // Pattern sweep across the whole wide band: reference equality
        // below 2^127, ground truth by reconstruction (n = d*q + r,
        // r < d ⇒ mag must be exactly q) over the whole band including
        // the (2^127, 2^128) divisors the old loop got wrong.
        for _ in 0..400 {
            let divisor = u128::from(u64::MAX) + 1 + (mix_u128(&mut state) >> 1);
            let mag: U256 = [mix_u128(&mut state), mix_u128(&mut state)];
            let value = Fixed { negative: false, mag };
            if divisor <= 1u128 << 127 {
                assert_eq!(
                    value.div_small(divisor).mag,
                    div_small_wide_reference(mag, divisor),
                    "div_small wide mismatch: mag={mag:?} d={divisor}"
                );
            }
            let quotient_in = mix_u128(&mut state) >> 1; // < 2^127: n fits U256
            let remainder_in = mix_u128(&mut state) % divisor;
            let (prod_hi, prod_lo) = mul_128(divisor, quotient_in);
            let (n_lo, carry) = prod_lo.overflowing_add(remainder_in);
            let n_hi = prod_hi + u128::from(carry);
            let built = Fixed { negative: false, mag: [n_lo, n_hi] };
            assert_eq!(
                built.div_small(divisor).mag,
                [quotient_in, 0],
                "div_small reconstruction: d={divisor}"
            );
        }
    }

    /// `isqrt_u512` with the shared-library seed lands on the identical
    /// floor root as the replaced hand-rolled-seed version, across
    /// magnitudes and the perfect-square neighbourhood (the seed
    /// library's worst case).
    #[test]
    fn isqrt_u512_matches_hand_seed_reference() {
        let mut state = 0x1512_0451_u64;
        // Magnitude sweep: radicands of 1..=7 active u64 limbs (bit
        // length <= 448 < 452, inside the documented contract).
        for limb_count in 1..=7u32 {
            for _ in 0..40 {
                let radicand = pattern_u512(&mut state, limb_count);
                assert_eq!(
                    isqrt_u512(radicand),
                    isqrt_u512_reference(radicand),
                    "isqrt_u512 mismatch: n={radicand:?}"
                );
            }
        }
        // Perfect squares and their neighbours: r = x², x² - 1, x² + 1
        // for roots x up to 2^224 (radicand < 2^448).
        for _ in 0..120 {
            let root: U256 = [mix_u128(&mut state), mix_u128(&mut state) >> 32];
            let square = mul_u256(root, root);
            for delta in [0i32, -1, 1] {
                let mut radicand = square;
                if delta == -1 {
                    if radicand == [0, 0, 0, 0] {
                        continue;
                    }
                    // subtract 1 with borrow
                    let mut i = 0;
                    loop {
                        let (limb, borrow) = radicand[i].overflowing_sub(1);
                        radicand[i] = limb;
                        if !borrow {
                            break;
                        }
                        i += 1;
                    }
                } else if delta == 1 {
                    let mut i = 0;
                    loop {
                        let (limb, carry) = radicand[i].overflowing_add(1);
                        radicand[i] = limb;
                        if !carry {
                            break;
                        }
                        i += 1;
                    }
                }
                assert_eq!(
                    isqrt_u512(radicand),
                    isqrt_u512_reference(radicand),
                    "isqrt_u512 square-neighbour mismatch: n={radicand:?} delta={delta}"
                );
            }
        }
        // Exactness spot-check: floor(sqrt(x²)) == x, floor(sqrt(x²-1)) == x-1.
        let root: U256 = [0x1234_5678_9abc_def0_u128, 0];
        let square = mul_u256(root, root);
        assert_eq!(isqrt_u512(square), root);
        let square_minus_1 = [square[0] - 1, square[1], square[2], square[3]];
        assert_eq!(isqrt_u512(square_minus_1), [root[0] - 1, 0]);
    }

    #[test]
    fn mul_u256_small() {
        // 7 * 11 = 77.
        assert_eq!(mul_u256([7, 0], [11, 0]), [77, 0, 0, 0]);
        // (2^128) * (2^128) = 2^256.
        assert_eq!(mul_u256([0, 1], [0, 1]), [0, 0, 1, 0]);
        // (2^128 - 1)^2.
        let max_low_limb = [u128::MAX, 0];
        let product = mul_u256(max_low_limb, max_low_limb);
        // (2^128-1)^2 = 2^256 - 2^129 + 1.
        assert_eq!(product, [1, u128::MAX - 1, 0, 0]);
    }

    #[test]
    fn div_u512_round_trip() {
        // (a * b) / b == a for assorted a, b.
        for &(lhs, rhs) in &[
            ([123u128, 0], [456u128, 0]),
            ([u128::MAX, 7], [3, 0]),
            ([0, 1], [0, 1]),
            ([99, 99], [1234567, 0]),
        ] {
            let product = mul_u256(lhs, rhs);
            let quotient = div_u512_by_u256(product, rhs);
            assert_eq!([quotient[0], quotient[1]], lhs, "({lhs:?} * {rhs:?}) / {rhs:?}");
            assert_eq!(quotient[2], 0);
            assert_eq!(quotient[3], 0);
        }
    }

    #[test]
    fn fixed_add_sub_signs() {
        let working_scale = 6;
        let three = Fixed::from_u128_mag(3_000_000, false); // 3.0
        let two = Fixed::from_u128_mag(2_000_000, false); // 2.0
        assert_eq!(three.add(two), Fixed::from_u128_mag(5_000_000, false));
        assert_eq!(three.sub(two), Fixed::from_u128_mag(1_000_000, false));
        assert_eq!(two.sub(three), Fixed::from_u128_mag(1_000_000, true));
        assert_eq!(three.add(two.neg()), Fixed::from_u128_mag(1_000_000, false));
        assert!(three.sub(three).is_zero());
        let _ = working_scale;
    }

    #[test]
    fn fixed_mul_div() {
        let working_scale = 12;
        let one = Fixed {
            negative: false,
            mag: Fixed::pow10(working_scale),
        };
        let two = Fixed::from_u128_mag(2 * 10u128.pow(working_scale), false);
        let three = Fixed::from_u128_mag(3 * 10u128.pow(working_scale), false);
        // 2 * 3 == 6
        assert_eq!(
            two.mul(three, working_scale),
            Fixed::from_u128_mag(6 * 10u128.pow(working_scale), false)
        );
        // 6 / 2 == 3
        let six = Fixed::from_u128_mag(6 * 10u128.pow(working_scale), false);
        assert_eq!(six.div(two, working_scale), three);
        // x * 1 == x
        assert_eq!(three.mul(one, working_scale), three);
        // x / 3 (small) — 6 / 3 == 2
        assert_eq!(
            six.div_small(3),
            Fixed::from_u128_mag(2 * 10u128.pow(working_scale), false)
        );
        // sign of a negative product
        assert_eq!(two.neg().mul(three, working_scale).negative, true);
        assert_eq!(two.neg().mul(three.neg(), working_scale).negative, false);
    }

    #[test]
    fn fixed_sqrt_basic() {
        let working_scale = 12;
        let one = 10u128.pow(working_scale);
        // sqrt(4) == 2
        assert_eq!(
            Fixed::from_u128_mag(4 * one, false).sqrt(working_scale),
            Fixed::from_u128_mag(2 * one, false)
        );
        // sqrt(2) ≈ 1.414213562373 (truncated at scale 12)
        let sqrt_two = Fixed::from_u128_mag(2 * one, false).sqrt(working_scale);
        assert_eq!(sqrt_two.mag[0], 1_414_213_562_373);
        assert_eq!(sqrt_two.mag[1], 0);
        // sqrt(1) == 1, sqrt(0) == 0
        assert_eq!(
            Fixed::from_u128_mag(one, false).sqrt(working_scale),
            Fixed::from_u128_mag(one, false)
        );
        assert!(Fixed::ZERO.sqrt(working_scale).is_zero());
    }

    // ── Wide shifts ─────────────────────────────────────────────────
    //
    // `shl` and `shr` have a fast path for `n < 128` (within a limb) and
    // a slow path for `n >= 128` (cross-limb). The fast path is hit by
    // every guard-digit op; the slow path needs an explicit test.

    #[test]
    fn fixed_shl_crosses_limb_boundary() {
        // 1 << 130 = 4 in the high limb.
        let one = Fixed::from_u128_mag(1, false);
        let shifted = one.shl(130);
        assert_eq!(shifted.mag, [0, 4]);
        // shl(0) is identity.
        let value = Fixed::from_u128_mag(7, false);
        assert_eq!(value.shl(0).mag, [7, 0]);
    }

    #[test]
    fn fixed_shr_crosses_limb_boundary() {
        // A value with bits only in the high limb shifted right by 130
        // ends up in the low limb.
        let value = Fixed {
            negative: false,
            mag: [0, 4],
        };
        let shifted = value.shr(130);
        assert_eq!(shifted.mag, [1, 0]);
        // Negative magnitude shifted to zero loses its sign.
        let negative_value = Fixed {
            negative: true,
            mag: [0, 1],
        };
        let shifted = negative_value.shr(200);
        assert!(shifted.is_zero());
        // shr(0) is identity.
        let value = Fixed::from_u128_mag(7, false);
        assert_eq!(value.shr(0).mag, [7, 0]);
    }

    // ── Opposite-sign add with both zero ────────────────────────────
    //
    // `Fixed::add` of two ZEROs takes a distinct branch from the regular
    // same-sign or opposite-sign-but-non-zero adds.

    #[test]
    fn fixed_add_both_zero_opposite_signs() {
        let pos_zero = Fixed {
            negative: false,
            mag: [0, 0],
        };
        let neg_zero = Fixed {
            negative: true,
            mag: [0, 0],
        };
        let result_value = pos_zero.add(neg_zero);
        assert!(result_value.is_zero());
    }

    // ── div_small exercises the bit-loop body ──────────────────────
    //
    // `div_small` divides a 256-bit `Fixed` magnitude by a `u128`. The
    // loop body advances 256 bits, propagating remainder and quotient
    // limbs. Using a value that needs the high limb stresses the body.

    #[test]
    fn fixed_div_small_uses_full_256_bits() {
        // (2^130) / 4 = 2^128.
        let big = Fixed {
            negative: false,
            mag: [0, 4],
        };
        let result_value = big.div_small(4);
        assert_eq!(result_value.mag, [0, 1]);
        // (3 · 10^36) / 6 = 5 · 10^35 (fits one limb).
        let three_e36 = Fixed::from_u128_mag(3 * 10u128.pow(36), false);
        let result_value = three_e36.div_small(6);
        assert_eq!(result_value.mag, [5 * 10u128.pow(35), 0]);
        // Negative magnitude carries sign correctly.
        let negative_value = Fixed {
            negative: true,
            mag: [0, 4],
        };
        let result_value = negative_value.div_small(4);
        assert_eq!(result_value.mag, [0, 1]);
        assert!(result_value.negative);
    }

    // ── round_to_i128 overflow paths ───────────────────────────────

    #[test]
    fn round_to_i128_shift_zero_overflow_returns_none() {
        // shift=0 path: if the magnitude doesn't fit i128, return None.
        // Magnitude > i128::MAX requires the high limb to be set or the
        // low limb to exceed 1<<127.
        use crate::support::rounding::RoundingMode;
        let hte = RoundingMode::HalfToEven;
        // High limb non-zero — instant overflow.
        let value = Fixed {
            negative: false,
            mag: [0, 1],
        };
        assert_eq!(value.round_to_i128_with(0, 0, hte), None);
        // Low limb just above i128::MAX (positive).
        let value = Fixed {
            negative: false,
            mag: [(i128::MAX as u128) + 1, 0],
        };
        assert_eq!(value.round_to_i128_with(0, 0, hte), None);
        // Negative magnitude just past i128::MIN's absolute value.
        let value = Fixed {
            negative: true,
            mag: [(i128::MAX as u128) + 2, 0],
        };
        assert_eq!(value.round_to_i128_with(0, 0, hte), None);
        // i128::MIN itself round-trips exactly.
        let value = Fixed {
            negative: true,
            mag: [1u128 << 127, 0],
        };
        assert_eq!(value.round_to_i128_with(0, 0, hte), Some(i128::MIN));
    }

    #[test]
    fn round_to_i128_post_shift_overflow_returns_none() {
        // Shift > 0 path: a value that rounds to a magnitude wider than
        // i128 must return None. At working scale 1, dividing 2^128 by
        // 10 yields a magnitude that fits a single limb but still
        // exceeds i128::MAX for sufficiently large inputs; here we use
        // the full 256-bit max so the high-limb-nonzero post-rounding
        // overflow branch fires.
        use crate::support::rounding::RoundingMode;
        let hte = RoundingMode::HalfToEven;
        // 2^128 / 10 = ~3.4e37, fits low limb; not an overflow.
        let two_to_128 = Fixed {
            negative: false,
            mag: [0, 1],
        };
        let result_value = two_to_128.round_to_i128_with(1, 0, hte);
        // 2^128 / 10 ≈ 3.4e37, still > i128::MAX (1.7e38? No, 1.7e38; 3.4e37 < 1.7e38).
        // So the result actually fits i128. Sanity:
        assert!(result_value.is_some(), "2^128 / 10 fits i128");
        // The full-MAX value definitely overflows after rounding.
        let value = Fixed {
            negative: false,
            mag: [u128::MAX, u128::MAX],
        };
        assert_eq!(value.round_to_i128_with(0, 0, hte), None);
        // A value just above 10 · i128::MAX at working scale 1 overflows
        // after the /10 round.
        let huge = Fixed {
            negative: false,
            mag: [u128::MAX, 9u128],
        };
        assert_eq!(huge.round_to_i128_with(1, 0, hte), None);
    }

    // ── Large-radicand isqrt ────────────────────────────────────────
    //
    // The `Fixed::sqrt` path forms `mag · 10^working_scale` as a 512-bit value. With
    // `mag` near 2^128 and `working_scale` large, the radicand needs the top 512-bit
    // limb (`n[3]`) — exercising the high-limb branch of `isqrt_u512`.

    #[test]
    fn fixed_sqrt_at_large_working_scale() {
        // At `working_scale = 30`, the radicand `mag · 10^working_scale` for `mag = 10^30` is
        // `10^60` which lives in the 512-bit value's third limb,
        // exercising the high-limb branch of `isqrt_u512`.
        let working_scale = 30;
        let one_w = Fixed {
            negative: false,
            mag: Fixed::pow10(working_scale),
        };
        assert_eq!(one_w.sqrt(working_scale), one_w);
        // sqrt(4 at working_scale=30) ought to be 2 at working_scale=30.
        let four_w = Fixed {
            negative: false,
            mag: [4 * 10u128.pow(working_scale), 0],
        };
        let result_value = four_w.sqrt(working_scale);
        assert_eq!(result_value.mag, [2 * 10u128.pow(working_scale), 0]);
    }

    #[test]
    fn fixed_round_to_i128_half_to_even() {
        use crate::support::rounding::RoundingMode;
        // Working scale 6, round to scale 0. Pin the mode so this
        // test asserts HalfToEven specifically regardless of the
        // active `rounding-*` feature.
        let working_scale = 6;
        let hte = RoundingMode::HalfToEven;
        // 2.5 -> 2 (tie to even)
        let value = Fixed::from_u128_mag(2_500_000, false);
        assert_eq!(value.round_to_i128_with(working_scale, 0, hte), Some(2));
        // 3.5 -> 4 (tie to even)
        let value = Fixed::from_u128_mag(3_500_000, false);
        assert_eq!(value.round_to_i128_with(working_scale, 0, hte), Some(4));
        // 2.4 -> 2
        let value = Fixed::from_u128_mag(2_400_000, false);
        assert_eq!(value.round_to_i128_with(working_scale, 0, hte), Some(2));
        // 2.6 -> 3
        let value = Fixed::from_u128_mag(2_600_000, false);
        assert_eq!(value.round_to_i128_with(working_scale, 0, hte), Some(3));
        // negative: -2.5 -> -2
        let value = Fixed::from_u128_mag(2_500_000, true);
        assert_eq!(value.round_to_i128_with(working_scale, 0, hte), Some(-2));
        // same-scale narrowing (no rounding needed)
        let value = Fixed::from_u128_mag(123_456, false);
        assert_eq!(value.round_to_i128_with(working_scale, working_scale, hte), Some(123_456));
    }
}
