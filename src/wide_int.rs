//! Minimal in-tree wide-integer arithmetic for the correctly-rounded
//! strict transcendentals.
//!
//! The strict `ln` / `exp` / trig family must be accurate to within
//! 0.5 ULP of the exact result (the IEEE-754 round-to-nearest
//! contract). Achieving that for `D128<SCALE>` at the larger scales
//! means evaluating the series in a fixed-point intermediate with
//! *guard digits* beyond `SCALE` — which at `SCALE` near 38 overflows
//! `i128`. This module provides exactly the wide-integer primitives the
//! guard-digit evaluator needs and nothing more:
//!
//! - `U256` (`[u128; 2]`) and `U512` (`[u128; 4]`), little-endian limbs;
//! - full 256x256 -> 512 multiply;
//! - 512 / 256 -> quotient division (binary shift-subtract);
//! - a sign-magnitude `Fixed` value: a signed 256-bit magnitude
//!   interpreted at a fixed decimal working scale, with the
//!   `(a*b)/10^W` and `(a*10^W)/b` rescaling operations the series
//!   evaluator runs on.
//!
//! This is deliberately *not* a general big-integer type. It is the
//! smallest surface that makes the transcendentals correctly rounded,
//! is `no_std`, and is shared by every feature configuration. (`bnum`
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
    let (hi, c2) = hi1.overflowing_add(c0 as u128);
    ([lo, hi], c1 || c2)
}

/// `a - b` for 256-bit values; caller guarantees `a >= b`.
#[inline]
fn sub_u256(a: U256, b: U256) -> U256 {
    let (lo, borrow) = a[0].overflowing_sub(b[0]);
    let hi = a[1].wrapping_sub(b[1]).wrapping_sub(borrow as u128);
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
    let carry1 = c1a as u128 + c1b as u128;
    // limb2 = p01_hi + p10_hi + p11_lo + carry1
    let (s2, c2a) = p01_hi.overflowing_add(p10_hi);
    let (s2b, c2b) = s2.overflowing_add(p11_lo);
    let (r2, c2c) = s2b.overflowing_add(carry1);
    let carry2 = c2a as u128 + c2b as u128 + c2c as u128;
    // limb3 = p11_hi + carry2
    let r3 = p11_hi + carry2;
    [r0, r1, r2, r3]
}

/// Quotient `num / d` where `num` is 512-bit and `d` is 256-bit.
///
/// Returned as `U512`; for every use in this crate the true quotient
/// fits in 256 bits, but the wider return type keeps the routine
/// general and the high limbs are simply zero in practice.
pub(crate) fn div_u512_by_u256(num: U512, d: U256) -> U512 {
    debug_assert!(!(d[0] == 0 && d[1] == 0), "division by zero");
    let mut q: U512 = [0; 4];
    let mut rem: U256 = [0, 0];
    let mut num = num;
    let mut i = 512;
    while i > 0 {
        i -= 1;
        // Shift (rem, num) left by 1; the top bit of num enters rem.
        let bit = (num[3] >> 127) & 1;
        // num <<= 1
        num[3] = (num[3] << 1) | (num[2] >> 127);
        num[2] = (num[2] << 1) | (num[1] >> 127);
        num[1] = (num[1] << 1) | (num[0] >> 127);
        num[0] <<= 1;
        // rem = (rem << 1) | bit
        rem[1] = (rem[1] << 1) | (rem[0] >> 127);
        rem[0] = (rem[0] << 1) | bit;
        // q <<= 1
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
        let mag = [self.mag[0] << 1 | 0, (self.mag[1] << 1) | (self.mag[0] >> 127)];
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
    /// `target` (`target <= w`), half-to-even, and returns the result
    /// as a signed `i128` raw storage value. Used to land a guard-digit
    /// computation back on the caller's `D128<SCALE>`.
    ///
    /// Returns `None` if the rounded magnitude does not fit `i128`.
    pub(crate) fn round_to_i128(self, w: u32, target: u32) -> Option<i128> {
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
        // quotient + remainder of mag / 10^shift via the small-or-wide path.
        let (q, r) = divmod_u256(self.mag, divisor);
        let half = halve_u256(divisor);
        // round half-to-even on the magnitude.
        let round_up = match cmp_u256(r, half) {
            core::cmp::Ordering::Less => false,
            core::cmp::Ordering::Greater => true,
            core::cmp::Ordering::Equal => q[0] & 1 == 1,
        };
        let rounded = if round_up {
            let (s, _c) = add_u256(q, [1, 0]);
            s
        } else {
            q
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
    let half_bits = (bits + 1) / 2;
    let mut y: U256 = if half_bits >= 128 {
        [0, 1u128 << (half_bits - 128)]
    } else {
        [1u128 << half_bits, 0]
    };
    loop {
        // nq = n / y  (fits U256 because y >= sqrt(n)).
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

/// `a / b` and `a % b` for 256-bit values.
fn divmod_u256(a: U256, b: U256) -> (U256, U256) {
    debug_assert!(!is_zero_u256(b), "division by zero");
    let mut q: U256 = [0, 0];
    let mut rem: U256 = [0, 0];
    let mut a = a;
    let mut i = 256;
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

    #[test]
    fn fixed_round_to_i128_half_to_even() {
        // Working scale 6, round to scale 0.
        let w = 6;
        // 2.5 -> 2 (tie to even)
        let v = Fixed::from_u128_mag(2_500_000, false);
        assert_eq!(v.round_to_i128(w, 0), Some(2));
        // 3.5 -> 4 (tie to even)
        let v = Fixed::from_u128_mag(3_500_000, false);
        assert_eq!(v.round_to_i128(w, 0), Some(4));
        // 2.4 -> 2
        let v = Fixed::from_u128_mag(2_400_000, false);
        assert_eq!(v.round_to_i128(w, 0), Some(2));
        // 2.6 -> 3
        let v = Fixed::from_u128_mag(2_600_000, false);
        assert_eq!(v.round_to_i128(w, 0), Some(3));
        // negative: -2.5 -> -2
        let v = Fixed::from_u128_mag(2_500_000, true);
        assert_eq!(v.round_to_i128(w, 0), Some(-2));
        // same-scale narrowing
        let v = Fixed::from_u128_mag(123_456, false);
        assert_eq!(v.round_to_i128(w, w), Some(123_456));
    }
}

// ─────────────────────────────────────────────────────────────────────
// HInt256 — a hand-rolled signed 256-bit integer.
//
// This is the storage backend for the hand-rolled wide-decimal type
// `D256H` (see `src/hand_decimal.rs`), kept *alongside* the
// `bnum`-backed `D256` so the two can be benchmarked head to head.
//
// Representation: two's-complement, little-endian `[u128; 2]`
// (`limbs[0]` least significant, the sign is bit 127 of `limbs[1]`).
// It reuses the unsigned 256/512-bit primitives above
// (`mul_u256`, `div_u512_by_u256`, `divmod_u256`) for the
// multiply/divide magnitude work.
// ─────────────────────────────────────────────────────────────────────

/// Hand-rolled signed 256-bit integer (two's complement).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct HInt256(pub(crate) [u128; 2]);

impl HInt256 {
    /// Zero.
    pub(crate) const ZERO: HInt256 = HInt256([0, 0]);
    /// The largest representable value, `2^255 − 1`.
    pub(crate) const MAX: HInt256 = HInt256([u128::MAX, i128::MAX as u128]);
    /// The smallest representable value, `−2^255`.
    pub(crate) const MIN: HInt256 = HInt256([0, 1u128 << 127]);

    /// `true` if the value is negative (sign bit set).
    #[inline]
    pub(crate) fn is_negative(self) -> bool {
        self.0[1] >> 127 == 1
    }

    /// `true` if the value is strictly positive.
    #[inline]
    pub(crate) fn is_positive(self) -> bool {
        !self.is_negative() && !self.is_zero()
    }

    /// `true` if the value is exactly zero.
    #[inline]
    pub(crate) fn is_zero(self) -> bool {
        self.0[0] == 0 && self.0[1] == 0
    }

    /// Two's-complement negation. `MIN.negate() == MIN` (wraps, matching
    /// primitive signed integers).
    #[inline]
    pub(crate) fn negate(self) -> HInt256 {
        let (lo, c) = (!self.0[0]).overflowing_add(1);
        let hi = (!self.0[1]).wrapping_add(c as u128);
        HInt256([lo, hi])
    }

    /// Wrapping addition.
    #[inline]
    pub(crate) fn wrapping_add(self, rhs: HInt256) -> HInt256 {
        let (lo, c) = self.0[0].overflowing_add(rhs.0[0]);
        let hi = self.0[1]
            .wrapping_add(rhs.0[1])
            .wrapping_add(c as u128);
        HInt256([lo, hi])
    }

    /// Wrapping subtraction.
    #[inline]
    pub(crate) fn wrapping_sub(self, rhs: HInt256) -> HInt256 {
        self.wrapping_add(rhs.negate())
    }

    /// Checked addition — `None` on signed overflow.
    #[inline]
    pub(crate) fn checked_add(self, rhs: HInt256) -> Option<HInt256> {
        let r = self.wrapping_add(rhs);
        // Overflow iff the operands share a sign and the result's sign
        // differs from it.
        if self.is_negative() == rhs.is_negative() && r.is_negative() != self.is_negative() {
            None
        } else {
            Some(r)
        }
    }

    /// Checked subtraction — `None` on signed overflow.
    #[inline]
    pub(crate) fn checked_sub(self, rhs: HInt256) -> Option<HInt256> {
        let r = self.wrapping_sub(rhs);
        if self.is_negative() != rhs.is_negative() && r.is_negative() != self.is_negative() {
            None
        } else {
            Some(r)
        }
    }

    /// Checked negation — `None` only for `MIN`.
    #[inline]
    pub(crate) fn checked_neg(self) -> Option<HInt256> {
        if self == HInt256::MIN {
            None
        } else {
            Some(self.negate())
        }
    }

    /// Unsigned magnitude `|self|` as a `U256`. For `MIN` this is
    /// exactly `2^255`, which is the true magnitude (it does not fit a
    /// signed `HInt256`, but the unsigned `U256` holds it).
    #[inline]
    fn magnitude(self) -> U256 {
        if self.is_negative() {
            self.negate().0
        } else {
            self.0
        }
    }

    /// Applies a sign to a non-negative magnitude.
    #[inline]
    fn with_sign(mag: U256, negative: bool) -> HInt256 {
        let v = HInt256(mag);
        if negative && !v.is_zero() {
            v.negate()
        } else {
            v
        }
    }

    /// Decimal multiply at scale `scale`: `(self · rhs) / 10^scale`,
    /// truncating toward zero. The 256×256→512 product is exact; the
    /// divide-back is the only rounding step. The caller's values keep
    /// the result inside the signed 256-bit range.
    #[inline]
    pub(crate) fn dec_mul(self, rhs: HInt256, scale: u32) -> HInt256 {
        let negative = self.is_negative() ^ rhs.is_negative();
        let prod = mul_u256(self.magnitude(), rhs.magnitude());
        let q = div_u512_by_u256(prod, Fixed::pow10(scale));
        HInt256::with_sign([q[0], q[1]], negative)
    }

    /// Decimal divide at scale `scale`: `(self · 10^scale) / rhs`,
    /// truncating toward zero. `rhs` must be non-zero.
    #[inline]
    pub(crate) fn dec_div(self, rhs: HInt256, scale: u32) -> HInt256 {
        let negative = self.is_negative() ^ rhs.is_negative();
        let scaled = mul_u256(self.magnitude(), Fixed::pow10(scale));
        let q = div_u512_by_u256(scaled, rhs.magnitude());
        HInt256::with_sign([q[0], q[1]], negative)
    }

    /// Truncated remainder `self % rhs` (result carries the sign of
    /// `self`, matching primitive signed `%`). `rhs` must be non-zero.
    #[inline]
    pub(crate) fn rem(self, rhs: HInt256) -> HInt256 {
        let (_q, r) = divmod_u256(self.magnitude(), rhs.magnitude());
        HInt256::with_sign(r, self.is_negative())
    }

    /// `10^exp` as an `HInt256` (`exp <= 76`, always non-negative and
    /// in range).
    #[inline]
    pub(crate) fn pow10(exp: u32) -> HInt256 {
        HInt256(Fixed::pow10(exp))
    }

    /// Splits `|self|` into `(integer_part, fractional_part)` modulo
    /// `10^exp` — the magnitude divided by, and remaindered against,
    /// `10^exp`. Used by the hand-rolled decimal `Display`.
    #[inline]
    pub(crate) fn magnitude_divmod_pow10(self, exp: u32) -> (U256, U256) {
        divmod_u256(self.magnitude(), Fixed::pow10(exp))
    }
}

/// Renders an unsigned 256-bit value as its decimal digit string.
///
/// Repeatedly splits off 38-digit chunks (each `< 10^38`, so it fits a
/// `u128`) from the least-significant end, then concatenates them
/// most-significant first with zero-padding between chunks.
#[cfg(feature = "alloc")]
pub(crate) fn u256_decimal_string(mut v: U256) -> alloc::string::String {
    use alloc::string::String;
    if is_zero_u256(v) {
        return String::from("0");
    }
    let ten_38 = Fixed::pow10(38);
    let mut chunks: alloc::vec::Vec<u128> = alloc::vec::Vec::new();
    while !is_zero_u256(v) {
        let (q, r) = divmod_u256(v, ten_38);
        chunks.push(r[0]); // r < 10^38, so it fits limb 0.
        v = q;
    }
    let mut s = String::new();
    let last = chunks.len() - 1;
    // Most-significant chunk has no leading zeros.
    s.push_str(&alloc::format!("{}", chunks[last]));
    // Remaining chunks are zero-padded to a full 38 digits.
    for i in (0..last).rev() {
        s.push_str(&alloc::format!("{:0>38}", chunks[i]));
    }
    s
}

impl core::cmp::Ord for HInt256 {
    #[inline]
    fn cmp(&self, other: &HInt256) -> core::cmp::Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => core::cmp::Ordering::Less,
            (false, true) => core::cmp::Ordering::Greater,
            // Same sign: the unsigned limb ordering agrees with the
            // signed ordering (two's complement is monotonic within a
            // sign class).
            _ => cmp_u256(self.0, other.0),
        }
    }
}

impl core::cmp::PartialOrd for HInt256 {
    #[inline]
    fn partial_cmp(&self, other: &HInt256) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod hint256_tests {
    use super::HInt256;

    fn from_i128(n: i128) -> HInt256 {
        if n < 0 {
            HInt256([n as u128, u128::MAX])
        } else {
            HInt256([n as u128, 0])
        }
    }

    #[test]
    fn add_sub_neg() {
        let three = from_i128(3);
        let five = from_i128(5);
        assert_eq!(three.wrapping_add(five), from_i128(8));
        assert_eq!(five.wrapping_sub(three), from_i128(2));
        assert_eq!(three.wrapping_sub(five), from_i128(-2));
        assert_eq!(three.negate(), from_i128(-3));
        assert_eq!(from_i128(-7).negate(), from_i128(7));
        assert_eq!(HInt256::ZERO.negate(), HInt256::ZERO);
    }

    #[test]
    fn checked_overflow() {
        assert_eq!(HInt256::MAX.checked_add(from_i128(1)), None);
        assert_eq!(HInt256::MIN.checked_sub(from_i128(1)), None);
        assert_eq!(HInt256::MIN.checked_neg(), None);
        assert_eq!(HInt256::MAX.checked_neg(), Some(HInt256::MIN.wrapping_add(from_i128(1))));
        assert_eq!(from_i128(2).checked_add(from_i128(3)), Some(from_i128(5)));
    }

    #[test]
    fn ordering_is_signed() {
        assert!(from_i128(-1) < from_i128(0));
        assert!(from_i128(-1) < from_i128(1));
        assert!(HInt256::MIN < HInt256::MAX);
        assert!(HInt256::MAX > HInt256::ZERO);
        assert!(from_i128(5) > from_i128(3));
        assert!(from_i128(-5) < from_i128(-3));
    }

    #[test]
    fn dec_mul_div_rem() {
        // Scale 6: 2.0 * 3.0 == 6.0
        let s = 6;
        let one = HInt256::pow10(s);
        let two = HInt256([2 * 10u128.pow(s), 0]);
        let three = HInt256([3 * 10u128.pow(s), 0]);
        let six = HInt256([6 * 10u128.pow(s), 0]);
        assert_eq!(two.dec_mul(three, s), six);
        assert_eq!(six.dec_div(two, s), three);
        assert_eq!(three.dec_mul(one, s), three);
        // Signed: (-2) * 3 == -6
        assert_eq!(two.negate().dec_mul(three, s), six.negate());
        // rem: 7 % 3 == 1 at scale 0
        assert_eq!(from_i128(7).rem(from_i128(3)), from_i128(1));
        assert_eq!(from_i128(-7).rem(from_i128(3)), from_i128(-1));
    }
}
