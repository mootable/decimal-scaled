//! Expected result when a true value overflows a subject's cell, per the
//! subject's declared [`Overflow`] policy, as a signed scaled-integer string
//! (the same form the validators compare against).
//!
//! `Overflow::Panic` returns `None` (the tester validates it via `catch_unwind`);
//! `Truncate` / `Saturate` / `Wrap` are computed here from the full-precision
//! golden. `Saturate` and `Wrap` use the 2's-complement storage width
//! (`storage_bits`), so they need the small base-2^32 big-int below.

use crate::rounding::RoundingMode;
use crate::subject::Overflow;
use crate::value::GoldenValue;

/// The expected overflow result at `(width, scale)` for a subject with
/// `storage_bits`-wide 2's-complement storage, as a signed scaled-integer
/// string, or `None` for [`Overflow::Panic`].
pub fn expected_overflow(
    golden: &GoldenValue,
    width: u32,
    scale: u32,
    storage_bits: u32,
    overflow: Overflow,
) -> Option<String> {
    // The true value scaled by 10^scale, truncated toward zero: the signed
    // integer the storage would hold (e.g. "-12345").
    let signed = golden.round_to(scale, RoundingMode::Trunc, false);
    let (neg, mag) = split_sign(&signed);
    match overflow {
        Overflow::Panic => None,
        Overflow::Truncate => Some(with_sign(neg, keep_low_decimal(mag, width as usize))),
        Overflow::Saturate => Some(saturate(neg, storage_bits)),
        Overflow::Wrap => Some(wrap_2c(neg, mag, storage_bits)),
    }
}

fn split_sign(s: &str) -> (bool, &str) {
    match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s),
    }
}

fn with_sign(neg: bool, mag: String) -> String {
    if neg && mag != "0" { format!("-{mag}") } else { mag }
}

/// Keep the low `n` decimal digits of `mag` (decimal truncation of the top),
/// leading zeros stripped.
fn keep_low_decimal(mag: &str, n: usize) -> String {
    let start = mag.len().saturating_sub(n);
    let t = mag[start..].trim_start_matches('0');
    if t.is_empty() { "0".to_string() } else { t.to_string() }
}

/// Saturation target: `+MAX = 2^(b-1) - 1`, `-|MIN| = -2^(b-1)`.
fn saturate(neg: bool, bits: u32) -> String {
    let half = two_pow(bits - 1); // 2^(b-1)
    if neg {
        format!("-{}", to_decimal(&half))
    } else {
        to_decimal(&sub_one(half))
    }
}

/// 2's-complement wrap of the signed scaled value (`±mag`) into `bits` bits.
fn wrap_2c(neg: bool, mag: &str, bits: u32) -> String {
    let modulus = two_pow(bits); // 2^b
    let m = mask_low_bits(from_decimal(mag), bits); // |v| mod 2^b
    // u = v mod 2^b in [0, 2^b)
    let u = if !neg || is_zero(&m) { m } else { sub(&modulus, &m) };
    let half = two_pow(bits - 1); // 2^(b-1)
    if cmp(&u, &half) == std::cmp::Ordering::Less {
        to_decimal(&u) // non-negative
    } else {
        format!("-{}", to_decimal(&sub(&modulus, &u))) // u - 2^b
    }
}

// ── tiny base-2^32 big-int (little-endian limbs) ────────────────────────────

fn normalize(mut v: Vec<u32>) -> Vec<u32> {
    while v.len() > 1 && *v.last().unwrap() == 0 {
        v.pop();
    }
    if v.is_empty() {
        v.push(0);
    }
    v
}

fn is_zero(v: &[u32]) -> bool {
    v.iter().all(|&x| x == 0)
}

fn from_decimal(s: &str) -> Vec<u32> {
    let mut v = vec![0u32];
    for b in s.bytes() {
        if !b.is_ascii_digit() {
            continue;
        }
        mul_small(&mut v, 10);
        add_small(&mut v, (b - b'0') as u32);
    }
    normalize(v)
}

fn mul_small(v: &mut Vec<u32>, m: u32) {
    let mut carry = 0u64;
    for l in v.iter_mut() {
        let t = *l as u64 * m as u64 + carry;
        *l = t as u32;
        carry = t >> 32;
    }
    while carry > 0 {
        v.push(carry as u32);
        carry >>= 32;
    }
}

fn add_small(v: &mut Vec<u32>, a: u32) {
    let mut carry = a as u64;
    let mut i = 0;
    while carry > 0 {
        if i < v.len() {
            let t = v[i] as u64 + carry;
            v[i] = t as u32;
            carry = t >> 32;
        } else {
            v.push(carry as u32);
            carry = 0;
        }
        i += 1;
    }
}

fn to_decimal(v: &[u32]) -> String {
    if is_zero(v) {
        return "0".to_string();
    }
    let mut limbs = v.to_vec();
    let mut groups: Vec<u32> = Vec::new(); // base-1e9 groups, little-endian
    while !is_zero(&limbs) {
        let rem = divmod_small(&mut limbs, 1_000_000_000);
        limbs = normalize(limbs);
        groups.push(rem);
    }
    let mut s = String::new();
    for (i, g) in groups.iter().rev().enumerate() {
        if i == 0 {
            s.push_str(&g.to_string());
        } else {
            s.push_str(&format!("{g:09}"));
        }
    }
    s
}

fn divmod_small(v: &mut [u32], d: u32) -> u32 {
    let mut rem = 0u64;
    for i in (0..v.len()).rev() {
        let acc = (rem << 32) | v[i] as u64;
        v[i] = (acc / d as u64) as u32;
        rem = acc % d as u64;
    }
    rem as u32
}

fn two_pow(b: u32) -> Vec<u32> {
    let idx = (b / 32) as usize;
    let bit = b % 32;
    let mut v = vec![0u32; idx + 1];
    v[idx] = 1u32 << bit;
    v
}

fn mask_low_bits(mut v: Vec<u32>, b: u32) -> Vec<u32> {
    let full = (b / 32) as usize;
    let rem = b % 32;
    let keep = if rem > 0 { full + 1 } else { full };
    if v.len() > keep {
        v.truncate(keep);
    }
    if rem > 0 && full < v.len() {
        v[full] &= (1u32 << rem) - 1;
    }
    normalize(v)
}

fn strip(v: &[u32]) -> &[u32] {
    let mut len = v.len();
    while len > 1 && v[len - 1] == 0 {
        len -= 1;
    }
    &v[..len]
}

fn cmp(a: &[u32], b: &[u32]) -> std::cmp::Ordering {
    let (a, b) = (strip(a), strip(b));
    a.len().cmp(&b.len()).then_with(|| {
        for i in (0..a.len()).rev() {
            match a[i].cmp(&b[i]) {
                std::cmp::Ordering::Equal => {}
                o => return o,
            }
        }
        std::cmp::Ordering::Equal
    })
}

/// `a - b`, precondition `a >= b`.
fn sub(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0i64;
    for i in 0..a.len() {
        let av = a[i] as i64;
        let bv = if i < b.len() { b[i] as i64 } else { 0 };
        let mut d = av - bv - borrow;
        if d < 0 {
            d += 1i64 << 32;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(d as u32);
    }
    normalize(out)
}

/// `v - 1`, precondition `v >= 1`.
fn sub_one(mut v: Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    loop {
        if v[i] > 0 {
            v[i] -= 1;
            break;
        }
        v[i] = u32::MAX;
        i += 1;
    }
    normalize(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gv(s: &str) -> GoldenValue {
        GoldenValue::parse(s).unwrap()
    }

    #[test]
    fn truncate_keeps_low_digits() {
        assert_eq!(
            expected_overflow(&gv("12345"), 3, 0, 128, Overflow::Truncate).unwrap(),
            "345"
        );
    }

    #[test]
    fn saturate_i128_max_min() {
        // huge positive -> i128::MAX, huge negative -> i128::MIN
        assert_eq!(
            expected_overflow(&gv("999999999999999999999999999999999999999"), 38, 0, 128, Overflow::Saturate).unwrap(),
            "170141183460469231731687303715884105727"
        );
        assert_eq!(
            expected_overflow(&gv("-999999999999999999999999999999999999999"), 38, 0, 128, Overflow::Saturate).unwrap(),
            "-170141183460469231731687303715884105728"
        );
    }

    #[test]
    fn wrap_2pow127_is_i128_min() {
        // 2^127 (i128::MAX + 1) wraps to i128::MIN = -2^127
        assert_eq!(
            expected_overflow(&gv("170141183460469231731687303715884105728"), 38, 0, 128, Overflow::Wrap).unwrap(),
            "-170141183460469231731687303715884105728"
        );
    }

    #[test]
    fn wrap_small_in_range_is_identity() {
        assert_eq!(
            expected_overflow(&gv("5"), 38, 0, 128, Overflow::Wrap).unwrap(),
            "5"
        );
    }

    #[test]
    fn panic_has_no_expected_value() {
        assert_eq!(expected_overflow(&gv("123"), 2, 0, 128, Overflow::Panic), None);
    }
}
