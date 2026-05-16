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
const fn mul_128(a: u128, b: u128) -> (u128, u128) {
    let (a_hi, a_lo) = (a >> 64, a & u64::MAX as u128);
    let (b_hi, b_lo) = (b >> 64, b & u64::MAX as u128);
    let (mid, carry1) = (a_lo * b_hi).overflowing_add(a_hi * b_lo);
    let (low, carry2) = (a_lo * b_lo).overflowing_add(mid << 64);
    let high = a_hi * b_hi + (mid >> 64) + ((carry1 as u128) << 64) + carry2 as u128;
    (high, low)
}

/// `a + b` for 256-bit values; returns `(sum, carry_out)`.
#[inline]
fn add_u256(a: U256, b: U256) -> (U256, bool) {
    let (lo, c0) = a[0].overflowing_add(b[0]);
    let (hi1, c1) = a[1].overflowing_add(b[1]);
    let (hi, c2) = hi1.overflowing_add(u128::from(c0));
    ([lo, hi], c1 || c2)
}

/// `a - b` for 256-bit values; caller guarantees `a >= b`.
#[inline]
fn sub_u256(a: U256, b: U256) -> U256 {
    let (lo, borrow) = a[0].overflowing_sub(b[0]);
    let hi = a[1].wrapping_sub(b[1]).wrapping_sub(u128::from(borrow));
    [lo, hi]
}

/// `a >= b` for 256-bit values.
#[inline]
fn ge_u256(a: U256, b: U256) -> bool {
    a[1] > b[1] || (a[1] == b[1] && a[0] >= b[0])
}

/// `a == 0` for a 256-bit value.
#[inline]
fn is_zero_u256(a: U256) -> bool {
    a[0] == 0 && a[1] == 0
}

/// Full 256x256 -> 512 unsigned product.
pub(crate) fn mul_u256(a: U256, b: U256) -> U512 {
    // a = a0 + a1·B, b = b0 + b1·B, B = 2^128.
    let (p00_hi, p00_lo) = mul_128(a[0], b[0]);
    let (p01_hi, p01_lo) = mul_128(a[0], b[1]);
    let (p10_hi, p10_lo) = mul_128(a[1], b[0]);
    let (p11_hi, p11_lo) = mul_128(a[1], b[1]);

    // limb0 = p00_lo
    let r0 = p00_lo;
    // limb1 = p00_hi + p01_lo + p10_lo
    let (s1, c1a) = p00_hi.overflowing_add(p01_lo);
    let (r1, c1b) = s1.overflowing_add(p10_lo);
    let carry1 = u128::from(c1a) + u128::from(c1b);
    // limb2 = p01_hi + p10_hi + p11_lo + carry1
    let (s2, c2a) = p01_hi.overflowing_add(p10_hi);
    let (s2b, c2b) = s2.overflowing_add(p11_lo);
    let (r2, c2c) = s2b.overflowing_add(carry1);
    let carry2 = u128::from(c2a) + u128::from(c2b) + u128::from(c2c);
    // limb3 = p11_hi + carry2
    let r3 = p11_hi + carry2;
    [r0, r1, r2, r3]
}

/// Bit length of a 512-bit value (`0` for zero, else `floor(log2)+1`).
#[inline]
fn bitlen_u512(n: U512) -> u32 {
    if n[3] != 0 {
        512 - n[3].leading_zeros()
    } else if n[2] != 0 {
        384 - n[2].leading_zeros()
    } else if n[1] != 0 {
        256 - n[1].leading_zeros()
    } else {
        128 - n[0].leading_zeros()
    }
}

/// `n << shift` for a 512-bit value (`shift < 512`).
#[inline]
fn shl_u512(n: U512, shift: u32) -> U512 {
    if shift == 0 {
        return n;
    }
    let limb = (shift / 128) as usize;
    let bit = shift % 128;
    let mut out = [0u128; 4];
    if bit == 0 {
        for i in (limb..4).rev() {
            out[i] = n[i - limb];
        }
    } else {
        for i in (limb..4).rev() {
            let lo = n[i - limb] << bit;
            let carry = if i - limb == 0 {
                0
            } else {
                n[i - limb - 1] >> (128 - bit)
            };
            out[i] = lo | carry;
        }
    }
    out
}

/// Quotient `num / d` for a 512-bit dividend and a divisor that fits
/// in a single 64-bit word.
///
/// Schoolbook long division in base `2^64`: each step divides a
/// 128-bit `(remainder, limb)` pair by the word divisor with one
/// hardware division. Far cheaper than the general bit loop, and it
/// covers every `10^scale` divisor for `scale <= 19` — the common
/// decimal multiply path.
fn div_u512_by_word(num: U512, d: u64) -> U512 {
    let dd = u128::from(d);
    let mut limbs = [0u64; 8];
    for i in 0..4 {
        limbs[i << 1] = num[i] as u64;
        limbs[(i << 1) | 1] = (num[i] >> 64) as u64;
    }
    let mut rem: u128 = 0;
    let mut i = 8;
    while i > 0 {
        i -= 1;
        let cur = (rem << 64) | u128::from(limbs[i]);
        limbs[i] = (cur / dd) as u64;
        rem = cur % dd;
    }
    let mut out = [0u128; 4];
    for i in 0..4 {
        out[i] = u128::from(limbs[i << 1]) | (u128::from(limbs[(i << 1) | 1]) << 64);
    }
    out
}

/// Quotient `num / d` where `num` is 512-bit and `d` is 256-bit.
///
/// Returned as `U512`; for every use in this crate the true quotient
/// fits in 256 bits, but the wider return type keeps the routine
/// general and the high limbs are simply zero in practice.
///
/// Binary shift-subtract long division. The loop is bounded by the
/// numerator's actual bit length, not a fixed 512 — for the typical
/// operands in this crate (products of moderate-magnitude decimals)
/// that is a multiple-times reduction in iteration count.
pub(crate) fn div_u512_by_u256(num: U512, d: U256) -> U512 {
    debug_assert!(!(d[0] == 0 && d[1] == 0), "division by zero");
    // Fast path: when both the dividend and divisor fit in a single
    // 128-bit word, the hardware divide is exact and far cheaper than
    // any bit loop. This covers the overwhelmingly common case of
    // moderate-magnitude decimal multiply/divide at small scales.
    if num[1] == 0 && num[2] == 0 && num[3] == 0 && d[1] == 0 {
        return [num[0] / d[0], 0, 0, 0];
    }
    // Word-divisor path: a wide dividend divided by a divisor that
    // fits in 64 bits (every `10^scale` for `scale <= 19`).
    if d[1] == 0 && d[0] <= u128::from(u64::MAX) {
        return div_u512_by_word(num, d[0] as u64);
    }
    let bits = bitlen_u512(num);
    if bits == 0 {
        return [0; 4];
    }
    // Pre-shift so the most-significant set bit sits at position 511,
    // then only `bits` shift-subtract steps are needed (the leading
    // `512 - bits` iterations of the naive loop are provably no-ops).
    let mut num = shl_u512(num, 512 - bits);
    let mut q: U512 = [0; 4];
    let mut rem: U256 = [0, 0];
    let mut i = bits;
    while i > 0 {
        i -= 1;
        // Shift (rem, num) left by 1; the top bit of num enters rem.
        let bit = (num[3] >> 127) & 1;
        num[3] = (num[3] << 1) | (num[2] >> 127);
        num[2] = (num[2] << 1) | (num[1] >> 127);
        num[1] = (num[1] << 1) | (num[0] >> 127);
        num[0] <<= 1;
        rem[1] = (rem[1] << 1) | (rem[0] >> 127);
        rem[0] = (rem[0] << 1) | bit;
        q[3] = (q[3] << 1) | (q[2] >> 127);
        q[2] = (q[2] << 1) | (q[1] >> 127);
        q[1] = (q[1] << 1) | (q[0] >> 127);
        q[0] <<= 1;
        if ge_u256(rem, d) {
            rem = sub_u256(rem, d);
            q[0] |= 1;
        }
    }
    q
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
    pub(crate) const ZERO: Fixed = Fixed { negative: false, mag: [0, 0] };

    /// Constructs from a non-negative `u128` magnitude already scaled to
    /// the working scale.
    #[inline]
    pub(crate) fn from_u128_mag(mag: u128, negative: bool) -> Fixed {
        Fixed { negative: negative && mag != 0, mag: [mag, 0] }
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
        Fixed { negative: false, mag }
    }

    /// Truncating change of working scale from `from_w` down to `to_w`
    /// (`to_w <= from_w`): divides the magnitude by `10^(from_w-to_w)`.
    pub(crate) fn rescale_down(self, from_w: u32, to_w: u32) -> Fixed {
        if from_w == to_w {
            return self;
        }
        let (q, _r) = divmod_u256(self.mag, Fixed::pow10(from_w - to_w));
        Fixed { negative: self.negative && !is_zero_u256(q), mag: q }
    }

    /// Multiplies the magnitude by a small unsigned integer `k`. The
    /// caller guarantees the result stays below `2^256`.
    pub(crate) fn mul_u128(self, k: u128) -> Fixed {
        // self.mag * k: (mag_lo + mag_hi*B) * k.
        let (lo_hi, lo_lo) = mul_128(self.mag[0], k);
        let (_hi_hi, hi_lo) = mul_128(self.mag[1], k);
        let (mag1, _c) = hi_lo.overflowing_add(lo_hi);
        let mag = [lo_lo, mag1];
        Fixed { negative: self.negative && !is_zero_u256(mag), mag }
    }

    /// `|self| >= |rhs|` — magnitude comparison.
    #[inline]
    pub(crate) fn ge_mag(self, rhs: Fixed) -> bool {
        ge_u256(self.mag, rhs.mag)
    }

    /// `self * 2` (magnitude doubled). Caller guarantees no overflow.
    #[inline]
    pub(crate) fn double(self) -> Fixed {
        let mag = [(self.mag[0] << 1), (self.mag[1] << 1) | (self.mag[0] >> 127)];
        Fixed { negative: self.negative, mag }
    }

    /// `self / 2`, truncating (magnitude halved).
    #[inline]
    pub(crate) fn halve(self) -> Fixed {
        Fixed { negative: self.negative, mag: halve_u256(self.mag) }
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

    /// `self << n` (magnitude shifted left). Caller guarantees no
    /// significant bits are lost (`bit_length + n <= 256`).
    pub(crate) fn shl(self, n: u32) -> Fixed {
        if n == 0 {
            return self;
        }
        let mag = if n >= 128 {
            [0, self.mag[0] << (n - 128)]
        } else {
            [self.mag[0] << n, (self.mag[1] << n) | (self.mag[0] >> (128 - n))]
        };
        Fixed { negative: self.negative, mag }
    }

    /// `self >> n` (magnitude shifted right, truncating).
    pub(crate) fn shr(self, n: u32) -> Fixed {
        if n == 0 {
            return self;
        }
        let mag = if n >= 128 {
            [self.mag[1] >> (n - 128), 0]
        } else {
            [(self.mag[0] >> n) | (self.mag[1] << (128 - n)), self.mag[1] >> n]
        };
        Fixed { negative: self.negative && !is_zero_u256(mag), mag }
    }

    /// Adds two values at the same working scale.
    pub(crate) fn add(self, rhs: Fixed) -> Fixed {
        if self.negative == rhs.negative {
            let (sum, _carry) = add_u256(self.mag, rhs.mag);
            // The transcendental evaluators keep magnitudes well below
            // 2^256, so `_carry` is always false here.
            Fixed { negative: self.negative, mag: sum }
        } else {
            // Opposite signs: subtract the smaller magnitude.
            match (ge_u256(self.mag, rhs.mag), is_zero_u256(self.mag) && is_zero_u256(rhs.mag)) {
                (_, true) => Fixed::ZERO,
                (true, _) => {
                    let mag = sub_u256(self.mag, rhs.mag);
                    Fixed { negative: self.negative && !is_zero_u256(mag), mag }
                }
                (false, _) => {
                    let mag = sub_u256(rhs.mag, self.mag);
                    Fixed { negative: rhs.negative && !is_zero_u256(mag), mag }
                }
            }
        }
    }

    /// Negates the value.
    #[inline]
    pub(crate) fn neg(self) -> Fixed {
        Fixed { negative: !self.negative && !self.is_zero(), mag: self.mag }
    }

    /// Subtracts `rhs` from `self` at the same working scale.
    #[inline]
    pub(crate) fn sub(self, rhs: Fixed) -> Fixed {
        self.add(rhs.neg())
    }

    /// Multiplies two working-scale values: `(self * rhs) / 10^w`,
    /// truncating toward zero. Both magnitudes must be below `10^w *
    /// 2^128` so the 512-bit product divides back into 256 bits.
    pub(crate) fn mul(self, rhs: Fixed, w: u32) -> Fixed {
        let prod = mul_u256(self.mag, rhs.mag);
        let scale = Fixed::pow10(w);
        let q = div_u512_by_u256(prod, scale);
        Fixed {
            negative: (self.negative ^ rhs.negative) && !(q[0] == 0 && q[1] == 0),
            mag: [q[0], q[1]],
        }
    }

    /// Divides by an unsigned `u128` quotient `n`, truncating toward
    /// zero. `n` must be non-zero.
    pub(crate) fn div_small(self, n: u128) -> Fixed {
        debug_assert!(n != 0, "division by zero");
        // 256-bit / 128-bit long division.
        let mut rem: u128 = 0;
        let mut hi = self.mag[1];
        let mut lo = self.mag[0];
        let mut q_hi: u128 = 0;
        let mut q_lo: u128 = 0;
        let mut bit = 256;
        while bit > 0 {
            bit -= 1;
            let top = (hi >> 127) & 1;
            hi = (hi << 1) | (lo >> 127);
            lo <<= 1;
            rem = (rem << 1) | top;
            q_hi = (q_hi << 1) | (q_lo >> 127);
            q_lo <<= 1;
            if rem >= n {
                rem -= n;
                q_lo |= 1;
            }
        }
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
    pub(crate) fn sqrt(self, w: u32) -> Fixed {
        let radicand = mul_u256(self.mag, Fixed::pow10(w));
        Fixed { negative: false, mag: isqrt_u512(radicand) }
    }

    /// Divides by another working-scale value: `(self * 10^w) / rhs`,
    /// truncating toward zero. `rhs` must be non-zero. `self * 10^w`
    /// must fit 512 bits (it always does for the evaluators' inputs).
    pub(crate) fn div(self, rhs: Fixed, w: u32) -> Fixed {
        let scale = Fixed::pow10(w);
        let scaled = mul_u256(self.mag, scale);
        let q = div_u512_by_u256(scaled, rhs.mag);
        Fixed {
            negative: (self.negative ^ rhs.negative) && !(q[0] == 0 && q[1] == 0),
            mag: [q[0], q[1]],
        }
    }

    /// Rounds the working-scale magnitude to a narrower decimal scale
    /// `target` (`target <= w`) using the crate-default
    /// [`RoundingMode`] and returns the result as a signed `i128`
    /// raw storage value. Used to land a guard-digit computation back
    /// on the caller's `D38<SCALE>`.
    ///
    /// Returns `None` if the rounded magnitude does not fit `i128`.
    ///
    /// [`RoundingMode`]: crate::rounding::RoundingMode
    pub(crate) fn round_to_i128(self, w: u32, target: u32) -> Option<i128> {
        self.round_to_i128_with(w, target, crate::rounding::DEFAULT_ROUNDING_MODE)
    }

    /// Mode-aware variant of [`Self::round_to_i128`]. Mode dispatch
    /// runs through [`crate::rounding::should_bump`], matching the
    /// rest of the crate's rounding sites.
    pub(crate) fn round_to_i128_with(
        self,
        w: u32,
        target: u32,
        mode: crate::rounding::RoundingMode,
    ) -> Option<i128> {
        let shift = w - target;
        if shift == 0 {
            // No rounding; just narrow.
            if self.mag[1] != 0 {
                return None;
            }
            let m = self.mag[0];
            return if self.negative {
                if m > 1u128 << 127 { None } else { Some((m as i128).wrapping_neg()) }
            } else if m > i128::MAX as u128 {
                None
            } else {
                Some(m as i128)
            };
        }
        let divisor = Fixed::pow10(shift);
        let (q, r) = divmod_u256(self.mag, divisor);
        let rounded = if is_zero_u256(r) {
            q
        } else {
            // |r| is r (already a magnitude); comp = divisor - r.
            let comp = sub_u256(divisor, r);
            let cmp_r = cmp_u256(r, comp);
            let q_is_odd = (q[0] & 1) == 1;
            let result_positive = !self.negative;
            if crate::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
                add_u256(q, [1, 0]).0
            } else {
                q
            }
        };
        if rounded[1] != 0 {
            return None;
        }
        let m = rounded[0];
        if self.negative {
            if m > 1u128 << 127 { None } else { Some((m as i128).wrapping_neg()) }
        } else if m > i128::MAX as u128 {
            None
        } else {
            Some(m as i128)
        }
    }
}

impl Fixed {
    /// Rounds the working-scale value to the nearest integer (ties away
    /// from zero) and returns it as `i128`. Used to find the `k` in the
    /// `exp` range reduction `v = k·ln(2) + s`; `|k|` is small there, so
    /// the result always fits.
    pub(crate) fn round_to_nearest_int(self, w: u32) -> i128 {
        let scale = Fixed::pow10(w);
        let (q, r) = divmod_u256(self.mag, scale);
        let int_mag = if ge_u256(r, halve_u256(scale)) {
            add_u256(q, [1, 0]).0
        } else {
            q
        };
        let m = int_mag[0] as i128;
        if self.negative {
            -m
        } else {
            m
        }
    }
}

/// `floor(sqrt(n))` for an unsigned 512-bit value, via Newton's method.
///
/// The result fits `U256`. Callers in this crate keep `n < 2^452`
/// (a working-scale radicand `mag · 10^w` with `w <= 68`), so the
/// initial overestimate and every iterate stay below `2^256`.
fn isqrt_u512(n: U512) -> U256 {
    if n == [0, 0, 0, 0] {
        return [0, 0];
    }
    // Bit length of n.
    let bits = if n[3] != 0 {
        512 - n[3].leading_zeros()
    } else if n[2] != 0 {
        384 - n[2].leading_zeros()
    } else if n[1] != 0 {
        256 - n[1].leading_zeros()
    } else {
        128 - n[0].leading_zeros()
    };
    // Initial overestimate y0 = 2^ceil(bits/2) >= sqrt(n); for n < 2^452
    // this is at most 2^226, comfortably inside U256.
    let half_bits = bits.div_ceil(2);
    let mut y: U256 = if half_bits >= 128 {
        [0, 1u128 << (half_bits - 128)]
    } else {
        [1u128 << half_bits, 0]
    };
    loop {
        // nq = n / y (fits U256 because y >= sqrt(n)).
        let nq = div_u512_by_u256(n, y);
        let nq = [nq[0], nq[1]];
        // y_next = (y + nq) / 2.
        let (sum, _carry) = add_u256(y, nq);
        let y_next = halve_u256(sum);
        if ge_u256(y_next, y) {
            return y;
        }
        y = y_next;
    }
}

/// Bit length of a 256-bit value (`0` for zero, else `floor(log2)+1`).
#[inline]
fn bitlen_u256(n: U256) -> u32 {
    if n[1] != 0 {
        256 - n[1].leading_zeros()
    } else {
        128 - n[0].leading_zeros()
    }
}

/// `n << shift` for a 256-bit value (`shift < 256`).
#[inline]
fn shl_u256(n: U256, shift: u32) -> U256 {
    if shift == 0 {
        n
    } else if shift >= 128 {
        [0, n[0] << (shift - 128)]
    } else {
        [n[0] << shift, (n[1] << shift) | (n[0] >> (128 - shift))]
    }
}

/// `a / b` and `a % b` for 256-bit values.
///
/// Binary shift-subtract long division, bounded by the dividend's
/// actual bit length rather than a fixed 256 iterations.
fn divmod_u256(a: U256, b: U256) -> (U256, U256) {
    debug_assert!(!is_zero_u256(b), "division by zero");
    // Fast path: both operands fit in a single 128-bit word.
    if a[1] == 0 && b[1] == 0 {
        return ([a[0] / b[0], 0], [a[0] % b[0], 0]);
    }
    let bits = bitlen_u256(a);
    if bits == 0 {
        return ([0, 0], [0, 0]);
    }
    let mut q: U256 = [0, 0];
    let mut rem: U256 = [0, 0];
    let mut a = shl_u256(a, 256 - bits);
    let mut i = bits;
    while i > 0 {
        i -= 1;
        let bit = (a[1] >> 127) & 1;
        a[1] = (a[1] << 1) | (a[0] >> 127);
        a[0] <<= 1;
        rem[1] = (rem[1] << 1) | (rem[0] >> 127);
        rem[0] = (rem[0] << 1) | bit;
        q[1] = (q[1] << 1) | (q[0] >> 127);
        q[0] <<= 1;
        if ge_u256(rem, b) {
            rem = sub_u256(rem, b);
            q[0] |= 1;
        }
    }
    (q, rem)
}

/// `a / 2` for a 256-bit value.
#[inline]
fn halve_u256(a: U256) -> U256 {
    [(a[0] >> 1) | (a[1] << 127), a[1] >> 1]
}

/// Three-way comparison of 256-bit values.
#[inline]
fn cmp_u256(a: U256, b: U256) -> core::cmp::Ordering {
    match a[1].cmp(&b[1]) {
        core::cmp::Ordering::Equal => a[0].cmp(&b[0]),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_u256_small() {
        // 7 * 11 = 77.
        assert_eq!(mul_u256([7, 0], [11, 0]), [77, 0, 0, 0]);
        // (2^128) * (2^128) = 2^256.
        assert_eq!(mul_u256([0, 1], [0, 1]), [0, 0, 1, 0]);
        // (2^128 - 1)^2.
        let m = [u128::MAX, 0];
        let p = mul_u256(m, m);
        // (2^128-1)^2 = 2^256 - 2^129 + 1.
        assert_eq!(p, [1, u128::MAX - 1, 0, 0]);
    }

    #[test]
    fn div_u512_round_trip() {
        // (a * b) / b == a for assorted a, b.
        for &(a, b) in &[
            ([123u128, 0], [456u128, 0]),
            ([u128::MAX, 7], [3, 0]),
            ([0, 1], [0, 1]),
            ([99, 99], [1234567, 0]),
        ] {
            let prod = mul_u256(a, b);
            let q = div_u512_by_u256(prod, b);
            assert_eq!([q[0], q[1]], a, "({a:?} * {b:?}) / {b:?}");
            assert_eq!(q[2], 0);
            assert_eq!(q[3], 0);
        }
    }

    #[test]
    fn fixed_add_sub_signs() {
        let w = 6;
        let three = Fixed::from_u128_mag(3_000_000, false); // 3.0
        let two = Fixed::from_u128_mag(2_000_000, false); // 2.0
        assert_eq!(three.add(two), Fixed::from_u128_mag(5_000_000, false));
        assert_eq!(three.sub(two), Fixed::from_u128_mag(1_000_000, false));
        assert_eq!(two.sub(three), Fixed::from_u128_mag(1_000_000, true));
        assert_eq!(three.add(two.neg()), Fixed::from_u128_mag(1_000_000, false));
        assert!(three.sub(three).is_zero());
        let _ = w;
    }

    #[test]
    fn fixed_mul_div() {
        let w = 12;
        let one = Fixed { negative: false, mag: Fixed::pow10(w) };
        let two = Fixed::from_u128_mag(2 * 10u128.pow(w), false);
        let three = Fixed::from_u128_mag(3 * 10u128.pow(w), false);
        // 2 * 3 == 6
        assert_eq!(two.mul(three, w), Fixed::from_u128_mag(6 * 10u128.pow(w), false));
        // 6 / 2 == 3
        let six = Fixed::from_u128_mag(6 * 10u128.pow(w), false);
        assert_eq!(six.div(two, w), three);
        // x * 1 == x
        assert_eq!(three.mul(one, w), three);
        // x / 3 (small) — 6 / 3 == 2
        assert_eq!(six.div_small(3), Fixed::from_u128_mag(2 * 10u128.pow(w), false));
        // sign of a negative product
        assert_eq!(two.neg().mul(three, w).negative, true);
        assert_eq!(two.neg().mul(three.neg(), w).negative, false);
    }

    #[test]
    fn fixed_sqrt_basic() {
        let w = 12;
        let one = 10u128.pow(w);
        // sqrt(4) == 2
        assert_eq!(
            Fixed::from_u128_mag(4 * one, false).sqrt(w),
            Fixed::from_u128_mag(2 * one, false)
        );
        // sqrt(2) ≈ 1.414213562373 (truncated at scale 12)
        let s2 = Fixed::from_u128_mag(2 * one, false).sqrt(w);
        assert_eq!(s2.mag[0], 1_414_213_562_373);
        assert_eq!(s2.mag[1], 0);
        // sqrt(1) == 1, sqrt(0) == 0
        assert_eq!(Fixed::from_u128_mag(one, false).sqrt(w), Fixed::from_u128_mag(one, false));
        assert!(Fixed::ZERO.sqrt(w).is_zero());
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
        let v = Fixed::from_u128_mag(7, false);
        assert_eq!(v.shl(0).mag, [7, 0]);
    }

    #[test]
    fn fixed_shr_crosses_limb_boundary() {
        // A value with bits only in the high limb shifted right by 130
        // ends up in the low limb.
        let v = Fixed { negative: false, mag: [0, 4] };
        let shifted = v.shr(130);
        assert_eq!(shifted.mag, [1, 0]);
        // Negative magnitude shifted to zero loses its sign.
        let neg = Fixed { negative: true, mag: [0, 1] };
        let shifted = neg.shr(200);
        assert!(shifted.is_zero());
        // shr(0) is identity.
        let v = Fixed::from_u128_mag(7, false);
        assert_eq!(v.shr(0).mag, [7, 0]);
    }

    // ── Opposite-sign add with both zero ────────────────────────────
    //
    // `Fixed::add` of two ZEROs takes a distinct branch from the regular
    // same-sign or opposite-sign-but-non-zero adds.

    #[test]
    fn fixed_add_both_zero_opposite_signs() {
        let pos_zero = Fixed { negative: false, mag: [0, 0] };
        let neg_zero = Fixed { negative: true, mag: [0, 0] };
        let r = pos_zero.add(neg_zero);
        assert!(r.is_zero());
    }

    // ── div_small exercises the bit-loop body ──────────────────────
    //
    // `div_small` divides a 256-bit `Fixed` magnitude by a `u128`. The
    // loop body advances 256 bits, propagating remainder and quotient
    // limbs. Using a value that needs the high limb stresses the body.

    #[test]
    fn fixed_div_small_uses_full_256_bits() {
        // (2^130) / 4 = 2^128.
        let big = Fixed { negative: false, mag: [0, 4] };
        let r = big.div_small(4);
        assert_eq!(r.mag, [0, 1]);
        // (3 · 10^36) / 6 = 5 · 10^35 (fits one limb).
        let three_e36 = Fixed::from_u128_mag(3 * 10u128.pow(36), false);
        let r = three_e36.div_small(6);
        assert_eq!(r.mag, [5 * 10u128.pow(35), 0]);
        // Negative magnitude carries sign correctly.
        let neg = Fixed { negative: true, mag: [0, 4] };
        let r = neg.div_small(4);
        assert_eq!(r.mag, [0, 1]);
        assert!(r.negative);
    }

    // ── round_to_i128 overflow paths ───────────────────────────────

    #[test]
    fn round_to_i128_shift_zero_overflow_returns_none() {
        // shift=0 path: if the magnitude doesn't fit i128, return None.
        // Magnitude > i128::MAX requires the high limb to be set or the
        // low limb to exceed 1<<127.
        use crate::rounding::RoundingMode;
        let hte = RoundingMode::HalfToEven;
        // High limb non-zero — instant overflow.
        let v = Fixed { negative: false, mag: [0, 1] };
        assert_eq!(v.round_to_i128_with(0, 0, hte), None);
        // Low limb just above i128::MAX (positive).
        let v = Fixed { negative: false, mag: [(i128::MAX as u128) + 1, 0] };
        assert_eq!(v.round_to_i128_with(0, 0, hte), None);
        // Negative magnitude just past i128::MIN's absolute value.
        let v = Fixed { negative: true, mag: [(i128::MAX as u128) + 2, 0] };
        assert_eq!(v.round_to_i128_with(0, 0, hte), None);
        // i128::MIN itself round-trips exactly.
        let v = Fixed { negative: true, mag: [1u128 << 127, 0] };
        assert_eq!(v.round_to_i128_with(0, 0, hte), Some(i128::MIN));
    }

    #[test]
    fn round_to_i128_post_shift_overflow_returns_none() {
        // Shift > 0 path: a value that rounds to a magnitude wider than
        // i128 must return None. At working scale 1, dividing 2^128 by
        // 10 yields a magnitude that fits a single limb but still
        // exceeds i128::MAX for sufficiently large inputs; here we use
        // the full 256-bit max so the high-limb-nonzero post-rounding
        // overflow branch fires.
        use crate::rounding::RoundingMode;
        let hte = RoundingMode::HalfToEven;
        // 2^128 / 10 = ~3.4e37, fits low limb; not an overflow.
        let two_to_128 = Fixed { negative: false, mag: [0, 1] };
        let r = two_to_128.round_to_i128_with(1, 0, hte);
        // 2^128 / 10 ≈ 3.4e37, still > i128::MAX (1.7e38? No, 1.7e38; 3.4e37 < 1.7e38).
        // So the result actually fits i128. Sanity:
        assert!(r.is_some(), "2^128 / 10 fits i128");
        // The full-MAX value definitely overflows after rounding.
        let v = Fixed { negative: false, mag: [u128::MAX, u128::MAX] };
        assert_eq!(v.round_to_i128_with(0, 0, hte), None);
        // A value just above 10 · i128::MAX at working scale 1 overflows
        // after the /10 round.
        let huge = Fixed { negative: false, mag: [u128::MAX, 9u128] };
        assert_eq!(huge.round_to_i128_with(1, 0, hte), None);
    }

    // ── Large-radicand isqrt ────────────────────────────────────────
    //
    // The `Fixed::sqrt` path forms `mag · 10^w` as a 512-bit value. With
    // `mag` near 2^128 and `w` large, the radicand needs the top 512-bit
    // limb (`n[3]`) — exercising the high-limb branch of `isqrt_u512`.

    #[test]
    fn fixed_sqrt_at_large_working_scale() {
        // At `w = 30`, the radicand `mag · 10^w` for `mag = 10^30` is
        // `10^60` which lives in the 512-bit value's third limb,
        // exercising the high-limb branch of `isqrt_u512`.
        let w = 30;
        let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
        assert_eq!(one_w.sqrt(w), one_w);
        // sqrt(4 at w=30) ought to be 2 at w=30.
        let four_w = Fixed { negative: false, mag: [4 * 10u128.pow(w), 0] };
        let r = four_w.sqrt(w);
        assert_eq!(r.mag, [2 * 10u128.pow(w), 0]);
    }

    #[test]
    fn fixed_round_to_i128_half_to_even() {
        use crate::rounding::RoundingMode;
        // Working scale 6, round to scale 0. Pin the mode so this
        // test asserts HalfToEven specifically regardless of the
        // active `rounding-*` feature.
        let w = 6;
        let hte = RoundingMode::HalfToEven;
        // 2.5 -> 2 (tie to even)
        let v = Fixed::from_u128_mag(2_500_000, false);
        assert_eq!(v.round_to_i128_with(w, 0, hte), Some(2));
        // 3.5 -> 4 (tie to even)
        let v = Fixed::from_u128_mag(3_500_000, false);
        assert_eq!(v.round_to_i128_with(w, 0, hte), Some(4));
        // 2.4 -> 2
        let v = Fixed::from_u128_mag(2_400_000, false);
        assert_eq!(v.round_to_i128_with(w, 0, hte), Some(2));
        // 2.6 -> 3
        let v = Fixed::from_u128_mag(2_600_000, false);
        assert_eq!(v.round_to_i128_with(w, 0, hte), Some(3));
        // negative: -2.5 -> -2
        let v = Fixed::from_u128_mag(2_500_000, true);
        assert_eq!(v.round_to_i128_with(w, 0, hte), Some(-2));
        // same-scale narrowing (no rounding needed)
        let v = Fixed::from_u128_mag(123_456, false);
        assert_eq!(v.round_to_i128_with(w, w, hte), Some(123_456));
    }
}
