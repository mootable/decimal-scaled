// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Hex-float golden parsing + binary rounding — the BINARY grading primitive.
//!
//! A `2:` golden field carries the deep true value stored EXACTLY as
//! `[-]0x<hex-mantissa>p<exp>` (a C99 `%a`-style integer significand × `2^exp`):
//! `<hex>` is the full non-negative significand integer (NO implicit bit, no
//! int/frac split — the exponent carries the magnitude) and `<exp>` the binary
//! exponent. The value is `(-1)^sign · mantissa · 2^exp`. Storing it in hex (not a
//! decimal rendering) lets a binary subject's grader round it in binary with ZERO
//! conversion loss.
//!
//! [`HexFloat::parse`] reads the field exactly (the significand is up to ~4104 bits
//! — well past any primitive integer, so it is held as `u64` limbs).
//! [`HexFloat::round_to_bits`] rounds that exact value to `k` significand bits
//! (round-half-to-even) — the operation that grades a binary subject at its OWN
//! mantissa width (f64 → 53, f32 → 24, a Q128.128 fixed-point → 128). The result is
//! a [`RoundedBinary`]; for the native widths [`RoundedBinary::to_f64`] /
//! [`RoundedBinary::to_f32`] reconstruct the exact float.
//!
//! This module is dependency-free on purpose: the golden harness is library-AGNOSTIC
//! (it must never depend on the core `decimal-scaled` crate, only the competitor
//! crates do, and only in `golden-competitors`), so the wide significand is a small
//! self-contained limb vector rather than a foreign big-integer type.

/// A parsed hex-float golden value: `(-1)^negative · mantissa · 2^exp`, where the
/// non-negative `mantissa` is held as little-endian `u64` limbs (`limbs[0]` is the
/// least-significant 64 bits; an empty `limbs` is the value zero). Width-independent:
/// the significand can be thousands of bits (the deep `2:` golden is ~4104).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexFloat {
    pub negative: bool,
    limbs: Vec<u64>,
    pub exp: i32,
}

/// A deep value rounded to `k` significand bits: `(-1)^negative · mantissa · 2^exp`
/// with `mantissa < 2^k` (and `mantissa` at most `2^128 − 1`, so `k ≤ 128`). This is
/// what a binary subject is graded against — the golden re-rounded to the subject's
/// own width, in binary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoundedBinary {
    pub negative: bool,
    pub mantissa: u128,
    pub exp: i32,
}

impl HexFloat {
    /// Parse a `[-]0x<hex>p<exp>` field. `None` on any malformed shape (missing `0x`
    /// prefix, missing `p`, empty/non-hex significand, non-integer exponent). A bare
    /// decimal value (the untagged corpus, or a `10:` value) does NOT parse here — by
    /// design: the runner uses that to fall back to the decimal grading path.
    pub fn parse(s: &str) -> Option<HexFloat> {
        let (negative, rest) = match s.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, s.strip_prefix('+').unwrap_or(s)),
        };
        let rest = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X"))?;
        let (hex, exp_str) = rest.split_once(['p', 'P'])?;
        if hex.is_empty() {
            return None;
        }
        let exp: i32 = exp_str.parse().ok()?;
        let limbs = hex_to_limbs(hex)?;
        // Normalise a zero significand to a non-negative zero.
        let negative = negative && !limbs.is_empty();
        Some(HexFloat { negative, limbs, exp })
    }

    /// The value is exactly zero.
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    /// The bit length of the significand (0 for zero).
    pub fn bit_len(&self) -> u32 {
        match self.limbs.last() {
            None => 0,
            Some(&top) => (self.limbs.len() as u32 - 1) * 64 + (64 - top.leading_zeros()),
        }
    }

    /// Round the exact value to `k` significand bits, round-half-to-even (the IEEE
    /// binary grid's rounding — what hardware floats produce). `k` in `1..=128`.
    ///
    /// When the significand already fits in `k` bits the value is returned exactly
    /// (its mantissa keeps fewer than `k` bits). Otherwise the low `bits − k` bits are
    /// discarded: the highest discarded bit is the round bit, the OR of the rest is the
    /// sticky bit, and a half-way case rounds to make the kept low bit even. A rounding
    /// carry that grows the mantissa to `k+1` bits shifts it back and bumps `exp`.
    pub fn round_to_bits(&self, k: u32) -> RoundedBinary {
        assert!((1..=128).contains(&k), "mantissa width {k} out of 1..=128");
        if self.is_zero() {
            return RoundedBinary { negative: false, mantissa: 0, exp: 0 };
        }
        let bits = self.bit_len();
        if bits <= k {
            // Fits exactly: the whole value is < 2^k ≤ 2^128, so it is its own mantissa.
            return RoundedBinary {
                negative: self.negative,
                mantissa: self.low_u128(),
                exp: self.exp,
            };
        }
        let drop = bits - k;
        let kept = self.shr_to_u128(drop); // top k bits (k ≤ 128) of the significand
        let round_bit = self.bit(drop - 1);
        let sticky = self.any_bit_below(drop - 1);
        let mut mantissa = kept;
        let mut exp = self.exp + drop as i32;
        let round_up = round_bit && (sticky || (mantissa & 1 == 1));
        if round_up {
            match mantissa.checked_add(1) {
                // Carry grew the mantissa to k+1 bits (reached 2^k): halve, bump exp.
                Some(m) if k < 128 && (m >> k) != 0 => {
                    mantissa = m >> 1;
                    exp += 1;
                }
                Some(m) => mantissa = m,
                // k == 128 and the mantissa was all ones (2^128 − 1): +1 == 2^128.
                None => {
                    mantissa = 1u128 << 127;
                    exp += 1;
                }
            }
        }
        RoundedBinary { negative: self.negative, mantissa, exp }
    }

    /// The low 128 bits of the significand.
    fn low_u128(&self) -> u128 {
        let lo = self.limbs.first().copied().unwrap_or(0) as u128;
        let hi = self.limbs.get(1).copied().unwrap_or(0) as u128;
        lo | (hi << 64)
    }

    /// `(significand >> shift)` truncated to its low 128 bits.
    fn shr_to_u128(&self, shift: u32) -> u128 {
        let limb_shift = (shift / 64) as usize;
        let bit_shift = shift % 64;
        let get = |idx: usize| self.limbs.get(idx).copied().unwrap_or(0);
        let (lo, hi) = if bit_shift == 0 {
            (get(limb_shift), get(limb_shift + 1))
        } else {
            let s = 64 - bit_shift;
            (
                (get(limb_shift) >> bit_shift) | (get(limb_shift + 1) << s),
                (get(limb_shift + 1) >> bit_shift) | (get(limb_shift + 2) << s),
            )
        };
        (lo as u128) | ((hi as u128) << 64)
    }

    /// The bit at index `i` (0 = least significant).
    fn bit(&self, i: u32) -> bool {
        let limb = (i / 64) as usize;
        match self.limbs.get(limb) {
            Some(&w) => (w >> (i % 64)) & 1 == 1,
            None => false,
        }
    }

    /// Any set bit at an index strictly below `i` (the sticky test).
    fn any_bit_below(&self, i: u32) -> bool {
        let full = (i / 64) as usize;
        for l in 0..full.min(self.limbs.len()) {
            if self.limbs[l] != 0 {
                return true;
            }
        }
        let rem = i % 64;
        if rem > 0 {
            if let Some(&w) = self.limbs.get(full) {
                let mask = (1u64 << rem) - 1;
                if w & mask != 0 {
                    return true;
                }
            }
        }
        false
    }
}

impl RoundedBinary {
    /// Reconstruct the exact `f64` for a value rounded to ≤ 53 bits. The mantissa
    /// (≤ 2^53) is exactly representable, and scaling by the power of two is exact in
    /// the normal range, so no rounding occurs here — the float IS the rounded value.
    /// (Subnormal/overflow magnitudes are excluded by f64's representability envelope,
    /// so the subject never produces a finite value to compare against there.)
    pub fn to_f64(self) -> f64 {
        let v = (self.mantissa as f64) * 2f64.powi(self.exp);
        if self.negative { -v } else { v }
    }

    /// Reconstruct the exact `f32` for a value rounded to ≤ 24 bits (see `to_f64`).
    pub fn to_f32(self) -> f32 {
        let v = (self.mantissa as f32) * 2f32.powi(self.exp);
        if self.negative { -v } else { v }
    }
}

/// Parse a hex significand into little-endian `u64` limbs (high zero limbs stripped;
/// `"0"` / all-zero → empty == the value zero). `None` if a non-hex byte appears.
fn hex_to_limbs(hex: &str) -> Option<Vec<u64>> {
    if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let mut limbs = Vec::new();
    let mut end = hex.len();
    while end > 0 {
        let start = end.saturating_sub(16);
        limbs.push(u64::from_str_radix(&hex[start..end], 16).ok()?);
        end = start;
    }
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
    Some(limbs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hf(s: &str) -> HexFloat {
        HexFloat::parse(s).unwrap_or_else(|| panic!("parse {s:?}"))
    }

    #[test]
    fn parses_sign_mantissa_exp() {
        let v = hf("0x1p0");
        assert!(!v.negative);
        assert_eq!(v.exp, 0);
        assert_eq!(v.bit_len(), 1);

        let n = hf("-0xffp-4");
        assert!(n.negative);
        assert_eq!(n.exp, -4);
        assert_eq!(n.bit_len(), 8);

        // Uppercase 0X / P and a leading '+' are accepted.
        let u = hf("+0XAp8");
        assert_eq!(u.bit_len(), 4); // 0xA = 1010
        assert_eq!(u.exp, 8);
    }

    #[test]
    fn rejects_malformed() {
        assert!(HexFloat::parse("1.5").is_none()); // bare decimal — falls to decimal path
        assert!(HexFloat::parse("0x1").is_none()); // no exponent
        assert!(HexFloat::parse("0xp0").is_none()); // empty significand
        assert!(HexFloat::parse("0xg1p0").is_none()); // non-hex
        assert!(HexFloat::parse("0x1pz").is_none()); // non-integer exponent
        assert!(HexFloat::parse("1p0").is_none()); // no 0x prefix
    }

    #[test]
    fn zero_normalises() {
        let z = hf("0x0p0");
        assert!(z.is_zero());
        assert!(!z.negative);
        let zn = hf("-0x0p5"); // negative zero collapses to +0
        assert!(zn.is_zero());
        assert!(!zn.negative);
        let r = z.round_to_bits(53);
        assert_eq!(r, RoundedBinary { negative: false, mantissa: 0, exp: 0 });
    }

    #[test]
    fn round_when_already_within_k_bits_is_exact() {
        // 0xA = 1010b (4 bits) rounded to 53 keeps the value verbatim.
        let r = hf("0xap3").round_to_bits(53);
        assert_eq!(r, RoundedBinary { negative: false, mantissa: 0xA, exp: 3 });
        // To 4 bits also exact (fits).
        let r4 = hf("0xap3").round_to_bits(4);
        assert_eq!(r4, RoundedBinary { negative: false, mantissa: 0xA, exp: 3 });
    }

    #[test]
    fn round_down_below_half() {
        // 0b1011_01 (= 45) rounded to 4 bits: keep 1011, dropped = 01 (round bit 0)
        // → round DOWN to 1011b = 11, exp += 2.
        let r = hf("0x2dp0").round_to_bits(4); // 0x2d = 45 = 101101
        assert_eq!(r, RoundedBinary { negative: false, mantissa: 0b1011, exp: 2 });
    }

    #[test]
    fn round_up_above_half() {
        // 0b1011_11 (= 47) rounded to 4 bits: keep 1011, dropped = 11 (round 1, sticky 1)
        // → round UP to 1100b = 12, exp += 2.
        let r = hf("0x2fp0").round_to_bits(4); // 47 = 101111
        assert_eq!(r, RoundedBinary { negative: false, mantissa: 0b1100, exp: 2 });
    }

    #[test]
    fn round_half_to_even_ties() {
        // Exact tie: dropped bits = 100... (round 1, sticky 0). Kept low bit decides.
        // 0b1011_10 (= 46): keep 1011 (low bit 1, ODD) → round to even 1100.
        let up = hf("0x2ep0").round_to_bits(4); // 46 = 101110
        assert_eq!(up.mantissa, 0b1100);
        assert_eq!(up.exp, 2);
        // 0b1010_10 (= 42): keep 1010 (low bit 0, EVEN) → stays 1010.
        let down = hf("0x2ap0").round_to_bits(4); // 42 = 101010
        assert_eq!(down.mantissa, 0b1010);
        assert_eq!(down.exp, 2);
    }

    #[test]
    fn round_carry_grows_mantissa() {
        // 0b1111_1 (= 31) rounded to 4 bits: keep 1111, dropped = 1 (tie, kept odd) →
        // round up → 10000 = 2^4 → renormalise to 1000b (= 8) with exp += 2.
        let r = hf("0x1fp0").round_to_bits(4); // 31 = 11111
        assert_eq!(r, RoundedBinary { negative: false, mantissa: 0b1000, exp: 2 });
    }

    #[test]
    fn round_to_128_bits_boundaries() {
        // A 129-bit value: 1 followed by 128 zeros, plus a low set bit forcing sticky.
        // mantissa = 2^128 + 1 → 129 bits. Round to 128: keep top 128 (= 2^127),
        // dropped = the low bit (round 0) → stays, exp += 1.
        let v = HexFloat { negative: false, limbs: vec![1, 0, 1], exp: 0 }; // 1 + 2^128
        let r = v.round_to_bits(128);
        assert_eq!(r.mantissa, 1u128 << 127);
        assert_eq!(r.exp, 1);
        // All-ones 128-bit value rounded to 128 bits is exact (fits).
        let allones = HexFloat { negative: false, limbs: vec![u64::MAX, u64::MAX], exp: 0 };
        let r2 = allones.round_to_bits(128);
        assert_eq!(r2.mantissa, u128::MAX);
        assert_eq!(r2.exp, 0);
    }

    #[test]
    fn round_to_128_carry_overflow() {
        // 129 bits all set: 2^129 − 1. Round to 128: keep top 128 (all ones), dropped =
        // low bit (round 1, sticky 0, kept odd) → round up → 2^128 → 2^127, exp += 2.
        let v = HexFloat { negative: false, limbs: vec![u64::MAX, u64::MAX, 1], exp: 0 };
        let r = v.round_to_bits(128);
        assert_eq!(r.mantissa, 1u128 << 127);
        assert_eq!(r.exp, 2);
    }

    #[test]
    fn sign_is_preserved() {
        let r = hf("-0x2fp0").round_to_bits(4);
        assert!(r.negative);
        assert_eq!(r.mantissa, 0b1100);
    }

    #[test]
    fn to_f64_reconstructs_known_values() {
        // 0.1 in f64 = 0xccccccccccccd · 2^-55 (verified against the std parser).
        let r = hf("0xccccccccccccdp-55").round_to_bits(53);
        assert_eq!(r.to_f64().to_bits(), 0.1f64.to_bits());
        // Exact powers of two and a simple fraction.
        assert_eq!(hf("0x1p0").round_to_bits(53).to_f64(), 1.0);
        assert_eq!(hf("0x1p1").round_to_bits(53).to_f64(), 2.0);
        assert_eq!(hf("-0x1p-1").round_to_bits(53).to_f64(), -0.5);
        assert_eq!(hf("0x0p0").round_to_bits(53).to_f64(), 0.0);
    }

    #[test]
    fn to_f32_reconstructs_known_values() {
        // 0.5 and 0.25 are exact powers of two; a 24-bit round of 0.1's deep value
        // reconstructs the std f32 parse of "0.1".
        assert_eq!(hf("0x1p-1").round_to_bits(24).to_f32(), 0.5f32);
        assert_eq!(hf("-0x1p-2").round_to_bits(24).to_f32(), -0.25f32);
        // 0.1f32 = 0xcccccd · 2^-27 (the std parser's value).
        let r = hf("0xccccccccccccdp-55").round_to_bits(24);
        assert_eq!(r.to_f32().to_bits(), 0.1f32.to_bits());
    }

    #[test]
    fn rounding_a_deep_value_to_53_then_to_24_matches_native_parse() {
        // The deep significand of 0.1 rounded to 53 bits is f64 0.1; rounded to 24
        // bits is f32 0.1 — the SAME deep value, each subject at its own width.
        let deep = hf("0xcccccccccccccccccccccccccccccccccccdp-147"); // 0.1, 144 bits
        assert_eq!(deep.round_to_bits(53).to_f64().to_bits(), 0.1f64.to_bits());
        assert_eq!(deep.round_to_bits(24).to_f32().to_bits(), 0.1f32.to_bits());
    }

    #[test]
    fn bit_len_across_limb_boundaries() {
        assert_eq!(HexFloat { negative: false, limbs: vec![], exp: 0 }.bit_len(), 0);
        assert_eq!(HexFloat { negative: false, limbs: vec![1], exp: 0 }.bit_len(), 1);
        assert_eq!(HexFloat { negative: false, limbs: vec![u64::MAX], exp: 0 }.bit_len(), 64);
        assert_eq!(HexFloat { negative: false, limbs: vec![0, 1], exp: 0 }.bit_len(), 65);
    }
}
