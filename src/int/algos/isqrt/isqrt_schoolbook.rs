// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Two-bits-at-a-time bitwise (digit-by-digit) integer square root.
//!
//! The algorithm processes input two bits at a time from most-significant
//! pair to least-significant, maintaining partial root p and remainder r
//! using only shifts, adds, subtracts, and compares.
//!
//! # Algorithm (Knuth TAOCP Vol. 2, Sec. 4.3.2, Exercise 17)
//!
//! b = LOW bit of current 2-bit group [b+1, b] of n:
//!   r = (r << 2) | (n >> b) & 3; t = (p << 2) | 1;
//!   if r >= t: r -= t, p = (p << 1) | 1; else p = p << 1.
//!
//! Invariant: p^2 + r = n. At completion p = floor(sqrt(n)).
//!
//! No division, no float seed. Generic over N. No policy re-entry.
//! Result identical to isqrt_newton.

use crate::int::algos::support::limbs::{bit_len, cmp, shl, sub_assign};

use crate::int::types::compute_int::MAX_DOUBLE_LIMBS;

/// floor(sqrt(n)) via two-bits-at-a-time bitwise algorithm.
pub(crate) fn isqrt_schoolbook(n: &[u64], out: &mut [u64]) {
    for o in out.iter_mut() { *o = 0; }
    let bits = bit_len(n);
    if bits == 0 { return; }
    if bits <= 1 { out[0] = 1; return; }
    let work = n.len() + 1;
    debug_assert!(work <= MAX_DOUBLE_LIMBS, "isqrt_schoolbook scratch overflow");
    let mut p = [0u64; MAX_DOUBLE_LIMBS];
    let mut r = [0u64; MAX_DOUBLE_LIMBS];
    let mut tmp = [0u64; MAX_DOUBLE_LIMBS];
    // b = LOW bit of top 2-bit group.
    // bits even: start = bits-2; bits odd: start = bits-1.
    let start: i64 = if bits % 2 == 1 { (bits - 1) as i64 } else { (bits - 2) as i64 };
    let mut b = start;
    while b >= 0 {
        // Step 1: r = (r << 2) | 2-bit group at [b+1, b].
        shl(&r[..work], 2, &mut tmp[..work]);
        r[..work].copy_from_slice(&tmp[..work]);
        let b_u32 = b as u32;
        let li = (b_u32 / 64) as usize;
        let bo = b_u32 % 64;
        let group: u64 = if bo == 63 {
            let lo = if li < n.len() { (n[li] >> 63) & 1 } else { 0 };
            let hi = if li + 1 < n.len() { n[li + 1] & 1 } else { 0 };
            (hi << 1) | lo
        } else if li < n.len() {
            (n[li] >> bo) & 3
        } else { 0 };
        r[0] |= group;
        // Step 2: t = (p << 2) | 1 = 4p + 1.
        shl(&p[..work], 2, &mut tmp[..work]);
        let mut t = [0u64; MAX_DOUBLE_LIMBS];
        t[..work].copy_from_slice(&tmp[..work]);
        t[0] |= 1;
        // Step 3: accept or reject next bit.
        if cmp(&r[..work], &t[..work]) >= 0 {
            sub_assign(&mut r[..work], &t[..work]);
            shl(&p[..work], 1, &mut tmp[..work]);
            p[..work].copy_from_slice(&tmp[..work]);
            p[0] |= 1;
        } else {
            shl(&p[..work], 1, &mut tmp[..work]);
            p[..work].copy_from_slice(&tmp[..work]);
        }
        b -= 2;
    }
    let copy_len = out.len().min(work);
    out[..copy_len].copy_from_slice(&p[..copy_len]);
}

#[cfg(test)]
mod tests {
    use super::isqrt_schoolbook;
    use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
    fn sb64(n: u64) -> u64 {
        let i = [n]; let mut o = [0u64]; isqrt_schoolbook(&i, &mut o); o[0]
    }
    fn sb128(n: u128) -> u128 {
        let i = [n as u64, (n>>64) as u64]; let mut o = [0u64, 0u64];
        isqrt_schoolbook(&i, &mut o); (o[0] as u128) | ((o[1] as u128) << 64)
    }
    fn nt64(n: u64) -> u64 {
        let i = [n]; let mut o = [0u64]; isqrt_newton(&i, &mut o); o[0]
    }
    fn nt128(n: u128) -> u128 {
        let i = [n as u64, (n>>64) as u64]; let mut o = [0u64, 0u64];
        isqrt_newton(&i, &mut o); (o[0] as u128) | ((o[1] as u128) << 64)
    }
    /// Fixed known values -- externally verified via Python math.isqrt.
    #[test]
    fn isqrt_schoolbook_known_values_u64() {
        let cases: &[(u64,u64)] = &[
            (0,0),(1,1),(2,1),(3,1),(4,2),(5,2),(8,2),(9,3),
            (15,3),(16,4),(17,4),(24,4),(25,5),(35,5),(36,6),
            (99,9),(100,10),(10_000,100),(999_999,999),(1_000_000,1_000),
            (2_u64.pow(32)-1,65_535),(2_u64.pow(32),65_536),
            (2_u64.pow(32)+1,65_536),
            (2_u64.pow(63),3_037_000_499),(u64::MAX,4_294_967_295),
        ];
        for &(n,e) in cases { let g=sb64(n); assert_eq!(g,e,"n={n}: got {g} expected {e}"); }
    }
    /// Fixed known values -- externally verified via Python math.isqrt.
    #[test]
    fn isqrt_schoolbook_known_values_u128() {
        let cases: &[(u128,u128)] = &[
            (0,0),(1,1),(4,2),(9,3),
            (2_u128.pow(64), 2_u128.pow(32)),
            (2_u128.pow(64)-1, u32::MAX as u128),
            (2_u128.pow(127), 13_043_817_825_332_782_212_u128),
            (u128::MAX, u64::MAX as u128),
        ];
        for &(n,e) in cases { let g=sb128(n); assert_eq!(g,e,"n={n}: got {g} expected {e}"); }
    }
    /// Cross-check: matches Newton for u64 inputs 0..=512 and boundary values.
    #[test]
    fn isqrt_schoolbook_matches_newton_u64_range() {
        for n in 0u64..=512 { let sb=sb64(n); let nt=nt64(n);
            assert_eq!(sb,nt,"n={n}: sb={sb} nt={nt}"); }
        for n in [u64::MAX,u64::MAX-1,u64::MAX-2,
                  2_u64.pow(63),2_u64.pow(63)-1,2_u64.pow(32),2_u64.pow(32)-1] {
            let sb=sb64(n); let nt=nt64(n);
            assert_eq!(sb,nt,"n={n}: sb={sb} nt={nt}");
        }
    }
    /// Cross-check u128 including perfect-square boundaries.
    #[test]
    fn isqrt_schoolbook_matches_newton_u128_range() {
        for n in 0u128..=256 { let sb=sb128(n); let nt=nt128(n);
            assert_eq!(sb,nt,"n={n}"); }
        for k in [2u128,3,5,10,100,1_000,65_536,1_000_000] {
            let n=k*k;
            assert_eq!(sb128(n),k,"isqrt({k}^2)");
            assert_eq!(nt128(n),k,"newton isqrt({k}^2)");
            if n>0 { assert_eq!(sb128(n-1),k-1,"isqrt({k}^2-1)"); }
            assert_eq!(sb128(n+1),k,"isqrt({k}^2+1)");
        }
    }
}
