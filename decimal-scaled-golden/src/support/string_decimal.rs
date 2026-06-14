// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::cmp::Ordering;

/// Strip leading zeros; "" or all-zeros => "0".
pub fn strip_lead(s: &str) -> &str {
    let t = s.trim_start_matches('0');
    if t.is_empty() { "0" } else { t }
}

/// Compare two non-negative integer digit strings by magnitude.
pub fn cmp_mag(a: &str, b: &str) -> Ordering {
    let (a, b) = (strip_lead(a), strip_lead(b));
    a.len().cmp(&b.len()).then_with(|| a.as_bytes().cmp(b.as_bytes()))
}

/// Sum of two non-negative integer digit strings.
pub fn add_mag(a: &str, b: &str) -> String {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let (mut i, mut j) = (a.len(), b.len());
    let mut carry = 0u8;
    let mut out = Vec::new();
    while i > 0 || j > 0 || carry > 0 {
        let da = if i > 0 { i -= 1; a[i] - b'0' } else { 0 };
        let db = if j > 0 { j -= 1; b[j] - b'0' } else { 0 };
        let s = da + db + carry;
        out.push(b'0' + s % 10);
        carry = s / 10;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

/// Difference of two non-negative integer digit strings. PRECONDITION: a >= b.
pub fn sub_mag(a: &str, b: &str) -> String {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let (mut i, mut j) = (a.len(), b.len());
    let mut borrow = 0i16;
    let mut out = Vec::new();
    while i > 0 {
        i -= 1;
        let da = (a[i] - b'0') as i16;
        let db = if j > 0 { j -= 1; (b[j] - b'0') as i16 } else { 0 };
        let mut d = da - db - borrow;
        if d < 0 { d += 10; borrow = 1; } else { borrow = 0; }
        out.push(b'0' + d as u8);
    }
    out.reverse();
    strip_lead(&String::from_utf8(out).unwrap()).to_string()
}

fn split_sign(s: &str) -> (bool, &str) {
    match s.strip_prefix('-') { Some(r) => (true, r), None => (false, s) }
}

/// |a - b| of two SIGNED integer digit strings, as a non-negative digit string.
pub fn abs_diff(a: &str, b: &str) -> String {
    let (na, ma) = split_sign(a);
    let (nb, mb) = split_sign(b);
    if na == nb {
        match cmp_mag(ma, mb) {
            Ordering::Less => sub_mag(mb, ma),
            _ => sub_mag(ma, mb),
        }
    } else {
        add_mag(ma, mb)
    }
}

/// Split a signed decimal string into `(negative, int_digits, frac_digits)`.
fn split_decimal(s: &str) -> (bool, &str, &str) {
    let (neg, body) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let (int, frac) = body.split_once('.').unwrap_or((body, ""));
    (neg, int, frac)
}

fn is_zero(int: &str, frac: &str) -> bool {
    strip_lead(int) == "0" && frac.bytes().all(|b| b == b'0')
}

/// Compare two fraction-digit strings, right-padding the shorter with zeros.
fn cmp_frac(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let n = a.len().max(b.len());
    for i in 0..n {
        let ca = a.get(i).copied().unwrap_or(b'0');
        let cb = b.get(i).copied().unwrap_or(b'0');
        match ca.cmp(&cb) {
            Ordering::Equal => continue,
            o => return o,
        }
    }
    Ordering::Equal
}

/// Compare two signed decimal strings (`[-]int[.frac]`) by value. `-0 == 0`.
pub fn cmp_decimal(a: &str, b: &str) -> Ordering {
    let (na, ia, fa) = split_decimal(a);
    let (nb, ib, fb) = split_decimal(b);
    let (za, zb) = (is_zero(ia, fa), is_zero(ib, fb));
    if za && zb {
        return Ordering::Equal;
    }
    let na = na && !za;
    let nb = nb && !zb;
    match (na, nb) {
        (false, true) => return Ordering::Greater,
        (true, false) => return Ordering::Less,
        _ => {}
    }
    let mag = cmp_mag(ia, ib).then_with(|| cmp_frac(fa, fb));
    if na { mag.reverse() } else { mag }
}

/// True if `value` lies within `[min, max]` (inclusive); a `None` bound is open.
pub fn within(value: &str, min: Option<&str>, max: Option<&str>) -> bool {
    if let Some(lo) = min {
        if cmp_decimal(value, lo) == Ordering::Less {
            return false;
        }
    }
    if let Some(hi) = max {
        if cmp_decimal(value, hi) == Ordering::Greater {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_and_sub() {
        assert_eq!(add_mag("999", "1"), "1000");
        assert_eq!(sub_mag("1000", "1"), "999");
        assert_eq!(cmp_mag("007", "7"), Ordering::Equal);
    }
    #[test]
    fn signed_abs_diff() {
        assert_eq!(abs_diff("14142", "14140"), "2");
        assert_eq!(abs_diff("14140", "14142"), "2");
        assert_eq!(abs_diff("-3", "2"), "5");
        assert_eq!(abs_diff("-3", "-7"), "4");
        assert_eq!(abs_diff("0", "0"), "0");
    }
}
