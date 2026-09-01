// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Bit-by-bit (digit-by-digit) integer cube root.
//!
//! [`icbrt_schoolbook`] is a restoring bitwise integer cube root. It works one
//! bit at a time from the most-significant to the least-significant result bit,
//! maintaining the partial root `p` and its square `p²` incrementally. At each
//! step it tests whether extending the partial root by the current bit keeps
//! the cube ≤ n, using only shifts, adds, subtracts, comparisons, and one
//! schoolbook multiply per iteration.
//!
//! # Algorithm
//!
//! For bit position `k` (MSB-first, 0-indexed), with current partial root `p`
//! and maintained `p_sq = p²`:
//!
//! 1. Let `d = 1 << k` (the candidate next bit).
//! 2. Compute the trial delta:
//!    `delta = d · (3·p_sq + 3·p·d + d²)`
//!    This equals `(p + d)³ - p³` using only multiplication by small
//!    factors — no full cube recomputation.
//! 3. If `p³ + delta <= n` (i.e. `delta <= n - p³`):
//!    accept the bit: `p += d`, update `p_sq = p²`.
//!
//! The `p_sq` update on acceptance:
//!   `p_sq_new = (p + d)² = p² + 2·p·d + d²`
//!   computed by the same schoolbook kernel.
//!
//! The running term `remainder = n - p³` is maintained to avoid recomputing
//! `p³` from scratch each step. On acceptance:
//!   `remainder -= delta`.
//!
//! # Properties
//!
//! - **No dispatch re-entry:** multiplications use [`mul_schoolbook`] directly,
//!   never a method that re-enters this or any other policy.
//! - **Generic over N:** valid for any limb count.
//! - **Exact:** result identical to
//!   [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`].

use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{
    add_assign, bit_len, cmp, shl, sub_assign,
};

/// Scratch capacity — 288 u64 limbs, matching the Newton icbrt budget.
use crate::int::types::compute_limbs::MAX_QUADRUPLE_LIMBS;

/// `out = floor(cbrt(radicand))`. Bit-by-bit integer cube root.
///
/// Determines each bit of the result from MSB to LSB. At each step tests
/// whether the next candidate bit can be set by checking
/// `(p + bit)³ <= radicand` via an incremental delta, using schoolbook
/// multiply kernels directly. No division, no floating-point seed, no Newton
/// iteration.
///
/// Result is identical to
/// [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`].
pub(crate) fn icbrt_schoolbook(radicand: &[u64], out: &mut [u64]) {
    for limb in out.iter_mut() {
        *limb = 0;
    }
    let bits = bit_len(radicand);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        out[0] = 1;
        return;
    }

    let work_len = radicand.len() + 1;
    // sq_len: p_sq needs at most 2*(work_len/3)+2 limbs, but we use
    // work_len+1 as an upper bound (the result has at most ceil(bits/3) bits,
    // so p_sq has at most ceil(2*bits/3) bits, which is < work_len limbs).
    let sq_len = (work_len * 2).min(MAX_QUADRUPLE_LIMBS);
    debug_assert!(work_len <= MAX_QUADRUPLE_LIMBS, "icbrt_schoolbook scratch overflow");

    // `p`: partial root (result bits accumulated so far).
    // `p_sq`: p², maintained incrementally.
    // `remainder`: n - p³, the running remainder.
    let mut p = [0u64; MAX_QUADRUPLE_LIMBS];
    let mut p_sq = [0u64; MAX_QUADRUPLE_LIMBS];
    let mut remainder = [0u64; MAX_QUADRUPLE_LIMBS];
    // Initialise remainder = n.
    remainder[..radicand.len()].copy_from_slice(radicand);

    // result_bits = ceil(bits / 3): the number of bits in floor(cbrt(n)).
    let result_bits = bits.div_ceil(3);

    // Process one bit at a time from result_bits-1 down to 0.
    let mut k = result_bits as i64 - 1;
    while k >= 0 {
        // `d` = 1 << k  (a single-bit value in a limb array).
        let bit_pos = k as u32;
        let d_limb_index = (bit_pos / 64) as usize;
        let d_bit_offset = bit_pos % 64;
        let mut d = [0u64; MAX_QUADRUPLE_LIMBS];
        if d_limb_index < MAX_QUADRUPLE_LIMBS {
            d[d_limb_index] = 1u64 << d_bit_offset;
        }

        // Compute delta = d * (3*p_sq + 3*p*d + d*d)
        //               = (p + d)^3 - p^3
        //
        // Step 1: three_p_sq = 3 * p_sq  (shift p_sq left by 1 and add once)
        // 3*p_sq = p_sq + p_sq + p_sq = p_sq*2 + p_sq = (p_sq << 1) + p_sq
        let mut three_p_sq = [0u64; MAX_QUADRUPLE_LIMBS];
        {
            let mut shifted = [0u64; MAX_QUADRUPLE_LIMBS];
            shl(&p_sq[..sq_len], 1, &mut shifted[..sq_len]);
            three_p_sq[..sq_len].copy_from_slice(&shifted[..sq_len]);
            add_assign(&mut three_p_sq[..sq_len], &p_sq[..sq_len]);
        }

        // Step 2: p_d = p * d  (using mul_schoolbook, result in 2*work_len
        // area)
        let p_d_len = (work_len + d_limb_index + 1).min(MAX_QUADRUPLE_LIMBS);
        let mut p_d = [0u64; MAX_QUADRUPLE_LIMBS];
        mul_schoolbook(&p[..work_len], &d[..d_limb_index + 1], &mut p_d[..p_d_len]);

        // Step 3: three_p_d = 3 * p_d = (p_d << 1) + p_d
        let mut three_p_d = [0u64; MAX_QUADRUPLE_LIMBS];
        {
            let mut shifted = [0u64; MAX_QUADRUPLE_LIMBS];
            shl(&p_d[..p_d_len], 1, &mut shifted[..p_d_len]);
            three_p_d[..p_d_len].copy_from_slice(&shifted[..p_d_len]);
            add_assign(&mut three_p_d[..p_d_len], &p_d[..p_d_len]);
        }

        // Step 4: d_sq = d * d = d << k (since d = 2^k, d^2 = 2^(2k))
        // d^2 has bit at position 2k.
        let d_sq_pos = (k as u32) * 2;
        let d_sq_limb_index = (d_sq_pos / 64) as usize;
        let d_sq_bit_offset = d_sq_pos % 64;
        let mut d_sq = [0u64; MAX_QUADRUPLE_LIMBS];
        if d_sq_limb_index < MAX_QUADRUPLE_LIMBS {
            d_sq[d_sq_limb_index] = 1u64 << d_sq_bit_offset;
        }
        // Handle overflow into the next limb if d_sq_bit_offset == 63 and a
        // carry would appear — but since d = 2^k and d^2 = 2^(2k), there is
        // exactly one bit set at position 2k, no carry needed.

        // Step 5: inner = three_p_sq + three_p_d + d_sq = 3p^2 + 3pd + d^2
        let inner_len = sq_len.max(p_d_len).max(d_sq_limb_index + 1) + 1;
        let inner_len = inner_len.min(MAX_QUADRUPLE_LIMBS);
        let mut inner = [0u64; MAX_QUADRUPLE_LIMBS];
        inner[..sq_len].copy_from_slice(&three_p_sq[..sq_len]);
        add_assign(&mut inner[..inner_len], &three_p_d[..p_d_len.min(inner_len)]);
        add_assign(&mut inner[..inner_len], &d_sq[..(d_sq_limb_index + 1).min(inner_len)]);
        // inner = 3*p^2 + 3*p*d + d^2

        // Step 6: delta = d * inner
        let delta_len = (d_limb_index + 1 + inner_len).min(MAX_QUADRUPLE_LIMBS);
        let mut delta = [0u64; MAX_QUADRUPLE_LIMBS];
        mul_schoolbook(&d[..d_limb_index + 1], &inner[..inner_len],
            &mut delta[..delta_len]);
        // delta = (p + d)^3 - p^3

        // Step 7: If remainder >= delta, accept the bit.
        //   remainder -= delta; p += d; p_sq = p^2 (recomputed).
        if cmp(&remainder[..work_len], &delta[..delta_len.min(work_len)]) >= 0 {
            sub_assign(&mut remainder[..work_len], &delta[..delta_len.min(work_len)]);
            add_assign(&mut p[..work_len], &d[..d_limb_index + 1]);
            // Recompute p_sq = p * p
            let mut new_p_sq = [0u64; MAX_QUADRUPLE_LIMBS];
            mul_schoolbook(&p[..work_len], &p[..work_len], &mut new_p_sq[..sq_len]);
            p_sq[..sq_len].copy_from_slice(&new_p_sq[..sq_len]);
        }

        k -= 1;
    }

    let copy_len = out.len().min(work_len);
    out[..copy_len].copy_from_slice(&p[..copy_len]);
}


#[cfg(test)]
mod tests {
    use super::icbrt_schoolbook;
    use crate::int::algos::icbrt::icbrt_newton::icbrt_newton;

    /// Helper: run icbrt_schoolbook on a u64 value using 1-limb buffers.
    fn schoolbook_u64(radicand: u64) -> u64 {
        let input = [radicand];
        let mut out = [0u64];
        icbrt_schoolbook(&input, &mut out);
        out[0]
    }

    /// Helper: run icbrt_schoolbook on a u128 value using 2-limb buffers.
    fn schoolbook_u128(radicand: u128) -> u128 {
        let input = [radicand as u64, (radicand >> 64) as u64];
        let mut out = [0u64, 0u64];
        icbrt_schoolbook(&input, &mut out);
        (out[0] as u128) | ((out[1] as u128) << 64)
    }

    /// Helper: run icbrt_newton on a u64 value (cross-check oracle).
    fn newton_u64(radicand: u64) -> u64 {
        let input = [radicand];
        let mut out = [0u64];
        icbrt_newton(&input, &mut out);
        out[0]
    }

    /// Helper: run icbrt_newton on a u128 value (cross-check oracle).
    fn newton_u128(radicand: u128) -> u128 {
        let input = [radicand as u64, (radicand >> 64) as u64];
        let mut out = [0u64, 0u64];
        icbrt_newton(&input, &mut out);
        (out[0] as u128) | ((out[1] as u128) << 64)
    }

    // ── Fixed known values (externally-computed, Python integer Newton) ──

    #[test]
    fn icbrt_schoolbook_known_values_u64() {
        // (input, expected floor cube root) — verified via Python integer Newton.
        let cases: &[(u64, u64)] = &[
            (0, 0),
            (1, 1),
            (2, 1),
            (7, 1),
            (8, 2),          // 2^3
            (9, 2),
            (26, 2),
            (27, 3),         // 3^3
            (28, 3),
            (63, 3),
            (64, 4),         // 4^3
            (65, 4),
            (125, 5),        // 5^3
            (126, 5),
            (999, 9),
            (1_000, 10),     // 10^3
            (1_001, 10),
            // u64 boundary cases — Python integer Newton.
            (2_u64.pow(63), 2_097_152),              // icbrt(2**63) = 2**21 = 2097152
            (u64::MAX, 2_642_245),                    // icbrt(2**64-1)
        ];
        for &(n, expected) in cases {
            let got = schoolbook_u64(n);
            assert_eq!(got, expected,
                "icbrt_schoolbook({n}) = {got}, expected {expected}");
        }
    }

    #[test]
    fn icbrt_schoolbook_known_values_u128() {
        // (input, expected) — Python integer Newton.
        let cases: &[(u128, u128)] = &[
            (0, 0),
            (1, 1),
            (7, 1),
            (8, 2),
            (27, 3),
            (64, 4),
            (125, 5),
            (2_u128.pow(64), 2_642_245),             // icbrt(2**64)
            (2_u128.pow(127), 5_541_191_377_756),    // icbrt(2**127)
            // u128::MAX = 2**128-1; icbrt = 2**42 + ... check via cube root
            // Perfect cubes.
            (1_000_000_000_u128, 1_000),             // 10^9 = 1000^3
        ];
        for &(n, expected) in cases {
            let got = schoolbook_u128(n);
            assert_eq!(got, expected,
                "icbrt_schoolbook({n}) = {got}, expected {expected}");
        }
    }

    // ── Cross-check: schoolbook matches Newton over a range ──

    #[test]
    fn icbrt_schoolbook_matches_newton_u64_range() {
        // Dense range near 0.
        for n in 0u64..=512 {
            let sb = schoolbook_u64(n);
            let nt = newton_u64(n);
            assert_eq!(sb, nt, "mismatch at n={n}: schoolbook={sb}, newton={nt}");
        }
        // Boundary sweep.
        for n in [u64::MAX, u64::MAX - 1, 2_u64.pow(63),
                  2_u64.pow(63) - 1, 2_u64.pow(32), 2_u64.pow(21).pow(3)] {
            let sb = schoolbook_u64(n);
            let nt = newton_u64(n);
            assert_eq!(sb, nt, "mismatch at n={n}: schoolbook={sb}, newton={nt}");
        }
    }

    #[test]
    fn icbrt_schoolbook_matches_newton_u128_range() {
        // Dense range.
        for n in 0u128..=256 {
            let sb = schoolbook_u128(n);
            let nt = newton_u128(n);
            assert_eq!(sb, nt, "mismatch at n={n}");
        }
        // Perfect cubes: floor root equals cube base.
        for k in [2u128, 3, 5, 10, 100, 1_000, 10_000] {
            let n = k * k * k;
            let sb = schoolbook_u128(n);
            let nt = newton_u128(n);
            assert_eq!(sb, k, "icbrt({n}) schoolbook = {sb}, expected {k}");
            assert_eq!(nt, k, "icbrt({n}) newton = {nt}, expected {k}");
            // One above and below perfect cube.
            if n > 0 {
                let sb_below = schoolbook_u128(n - 1);
                assert_eq!(sb_below, k - 1,
                    "icbrt({}) schoolbook should be {}", n - 1, k - 1);
            }
            let sb_above = schoolbook_u128(n + 1);
            assert_eq!(sb_above, k,
                "icbrt({}) schoolbook should be {}", n + 1, k);
        }
    }
}
