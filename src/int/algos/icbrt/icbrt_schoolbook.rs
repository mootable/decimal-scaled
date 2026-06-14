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
//! The running term `p_cube_rem = n - p³` (the remainder) is maintained to
//! avoid recomputing `p³` from scratch each step. On acceptance:
//!   `p_cube_rem -= delta`.
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

/// `out = floor(cbrt(n))`. Bit-by-bit integer cube root.
///
/// Determines each bit of the result from MSB to LSB. At each step tests
/// whether the next candidate bit can be set by checking `(p + bit)³ <= n`
/// via an incremental delta, using schoolbook multiply kernels directly.
/// No division, no floating-point seed, no Newton iteration.
///
/// Result is identical to
/// [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`].
pub(crate) fn icbrt_schoolbook(n: &[u64], out: &mut [u64]) {
    for o in out.iter_mut() {
        *o = 0;
    }
    let bits = bit_len(n);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        out[0] = 1;
        return;
    }

    let work = n.len() + 1;
    // sq_work: p_sq needs at most 2*(work/3)+2 limbs, but we use work+1 as
    // an upper bound (the result has at most ceil(bits/3) bits, so p_sq has
    // at most ceil(2*bits/3) bits, which is < work limbs).
    let sq_work = (work * 2).min(MAX_QUADRUPLE_LIMBS);
    debug_assert!(work <= MAX_QUADRUPLE_LIMBS, "icbrt_schoolbook scratch overflow");

    // `p`: partial root (result bits accumulated so far).
    // `p_sq`: p², maintained incrementally.
    // `rem`: n - p³, the running remainder.
    let mut p = [0u64; MAX_QUADRUPLE_LIMBS];
    let mut p_sq = [0u64; MAX_QUADRUPLE_LIMBS];
    let mut rem = [0u64; MAX_QUADRUPLE_LIMBS];
    // Initialise rem = n.
    rem[..n.len()].copy_from_slice(n);

    // result_bits = ceil(bits / 3): the number of bits in floor(cbrt(n)).
    let result_bits = bits.div_ceil(3);

    // Process one bit at a time from result_bits-1 down to 0.
    let mut k = result_bits as i64 - 1;
    while k >= 0 {
        // `d` = 1 << k  (a single-bit value in a limb array).
        let bit_pos = k as u32;
        let d_limb = (bit_pos / 64) as usize;
        let d_off = bit_pos % 64;
        let mut d = [0u64; MAX_QUADRUPLE_LIMBS];
        if d_limb < MAX_QUADRUPLE_LIMBS {
            d[d_limb] = 1u64 << d_off;
        }

        // Compute delta = d * (3*p_sq + 3*p*d + d*d)
        //               = (p + d)^3 - p^3
        //
        // Step 1: t1 = 3 * p_sq  (just shift p_sq left by 1 and add once)
        // 3*p_sq = p_sq + p_sq + p_sq = p_sq*2 + p_sq = (p_sq << 1) + p_sq
        let mut t1 = [0u64; MAX_QUADRUPLE_LIMBS];
        {
            let mut shifted = [0u64; MAX_QUADRUPLE_LIMBS];
            shl(&p_sq[..sq_work], 1, &mut shifted[..sq_work]);
            t1[..sq_work].copy_from_slice(&shifted[..sq_work]);
            add_assign(&mut t1[..sq_work], &p_sq[..sq_work]);
        }
        // t1 = 3 * p_sq

        // Step 2: p_d = p * d  (using mul_schoolbook, result in 2*work area)
        let pd_work = (work + d_limb + 1).min(MAX_QUADRUPLE_LIMBS);
        let mut p_d = [0u64; MAX_QUADRUPLE_LIMBS];
        mul_schoolbook(&p[..work], &d[..d_limb + 1], &mut p_d[..pd_work]);

        // Step 3: t2 = 3 * p_d = (p_d << 1) + p_d
        let mut t2 = [0u64; MAX_QUADRUPLE_LIMBS];
        {
            let mut shifted = [0u64; MAX_QUADRUPLE_LIMBS];
            shl(&p_d[..pd_work], 1, &mut shifted[..pd_work]);
            t2[..pd_work].copy_from_slice(&shifted[..pd_work]);
            add_assign(&mut t2[..pd_work], &p_d[..pd_work]);
        }
        // t2 = 3 * p * d

        // Step 4: d_sq = d * d = d << k (since d = 2^k, d^2 = 2^(2k))
        // d^2 has bit at position 2k.
        let d_sq_pos = (k as u32) * 2;
        let d_sq_limb = (d_sq_pos / 64) as usize;
        let d_sq_off = d_sq_pos % 64;
        let mut d_sq = [0u64; MAX_QUADRUPLE_LIMBS];
        if d_sq_limb < MAX_QUADRUPLE_LIMBS {
            d_sq[d_sq_limb] = 1u64 << d_sq_off;
        }
        // Handle overflow into the next limb if d_sq_off == 63 and a carry
        // would appear — but since d = 2^k and d^2 = 2^(2k), there is
        // exactly one bit set at position 2k, no carry needed.

        // Step 5: inner = t1 + t2 + d_sq = 3p^2 + 3pd + d^2
        let inner_work = sq_work.max(pd_work).max(d_sq_limb + 1) + 1;
        let inner_work = inner_work.min(MAX_QUADRUPLE_LIMBS);
        let mut inner = [0u64; MAX_QUADRUPLE_LIMBS];
        inner[..sq_work].copy_from_slice(&t1[..sq_work]);
        add_assign(&mut inner[..inner_work], &t2[..pd_work.min(inner_work)]);
        add_assign(&mut inner[..inner_work], &d_sq[..(d_sq_limb + 1).min(inner_work)]);
        // inner = 3*p^2 + 3*p*d + d^2

        // Step 6: delta = d * inner
        let delta_work = (d_limb + 1 + inner_work).min(MAX_QUADRUPLE_LIMBS);
        let mut delta = [0u64; MAX_QUADRUPLE_LIMBS];
        mul_schoolbook(&d[..d_limb + 1], &inner[..inner_work], &mut delta[..delta_work]);
        // delta = (p + d)^3 - p^3

        // Step 7: If rem >= delta, accept the bit.
        //   rem -= delta; p += d; p_sq = p^2 (recomputed).
        if cmp(&rem[..work], &delta[..delta_work.min(work)]) >= 0 {
            sub_assign(&mut rem[..work], &delta[..delta_work.min(work)]);
            add_assign(&mut p[..work], &d[..d_limb + 1]);
            // Recompute p_sq = p * p
            let mut new_p_sq = [0u64; MAX_QUADRUPLE_LIMBS];
            mul_schoolbook(&p[..work], &p[..work], &mut new_p_sq[..sq_work]);
            p_sq[..sq_work].copy_from_slice(&new_p_sq[..sq_work]);
        }

        k -= 1;
    }

    let copy_len = out.len().min(work);
    out[..copy_len].copy_from_slice(&p[..copy_len]);
}


#[cfg(test)]
mod tests {
    use super::icbrt_schoolbook;
    use crate::int::algos::icbrt::icbrt_newton::icbrt_newton;

    /// Helper: run icbrt_schoolbook on a u64 value using 1-limb buffers.
    fn schoolbook_u64(n: u64) -> u64 {
        let input = [n];
        let mut out = [0u64];
        icbrt_schoolbook(&input, &mut out);
        out[0]
    }

    /// Helper: run icbrt_schoolbook on a u128 value using 2-limb buffers.
    fn schoolbook_u128(n: u128) -> u128 {
        let input = [n as u64, (n >> 64) as u64];
        let mut out = [0u64, 0u64];
        icbrt_schoolbook(&input, &mut out);
        (out[0] as u128) | ((out[1] as u128) << 64)
    }

    /// Helper: run icbrt_newton on a u64 value (cross-check oracle).
    fn newton_u64(n: u64) -> u64 {
        let input = [n];
        let mut out = [0u64];
        icbrt_newton(&input, &mut out);
        out[0]
    }

    /// Helper: run icbrt_newton on a u128 value (cross-check oracle).
    fn newton_u128(n: u128) -> u128 {
        let input = [n as u64, (n >> 64) as u64];
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
