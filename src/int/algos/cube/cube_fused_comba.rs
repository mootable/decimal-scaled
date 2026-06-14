// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Fused product-scanning (comba) integer cubing.
//!
//! Candidate sibling of [`crate::int::algos::cube::cube_schoolbook`]. Where the
//! shipped algorithm computes `x³` as `x²·x` — materialising the full `x²` in a
//! scratch buffer (one ≈N²/2 half-product pass plus its carry sweep) and then a
//! second ≈N² truncated multiply — this kernel forms `x³` in a SINGLE
//! product-scanning pass, the cube analogue of the symmetric comba square
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`].
//!
//! The `int_cube_eq_ab` N-way A/B benched it against `cube_schoolbook`: it
//! wins at the narrow `N == 2` tier (~1.11x, where the `x²` materialisation
//! dominates) and loses by a widening margin from `N >= 3` (its `~N³/6`
//! triple-product count). [`crate::int::policy::cube`] therefore routes
//! `N == 2` here via the [`Algorithm::Comba`] arm and every other width to
//! `cube_schoolbook`. The remaining widths keep it as a kept alternative per
//! `docs/ARCHITECTURE.md` → "Keeping the alternatives".
//!
//! ## The symmetry
//!
//! `x³ = (Σ_i x_i B^i)³ = Σ_{i,j,k} x_i x_j x_k B^{i+j+k}` (`B = 2^64`). The
//! triple is symmetric in `(i, j, k)`, so each *unordered* multiset `{i ≤ j ≤ k}`
//! is formed ONCE and weighted by its number of orderings:
//!   - all three distinct (`i < j < k`) → 6,
//!   - exactly two equal (`i == j < k` or `i < j == k`) → 3,
//!   - all equal (`i == j == k`) → 1.
//! This forms ≈ the truncated tetrahedral count of triple-products, versus the
//! `x²·x` path's `N²/2` (square) + `N²` (multiply) limb-multiplies plus the
//! intermediate `x²` write/reload. `cube` is truncated-low (the result wraps
//! mod `2^(64·N)`), so any column index `≥ N` is above the width and skipped.
//!
//! ## The accumulator
//!
//! A single `u64·u64·u64` partial product spans up to 192 bits
//! (`(2^64−1)³ < 2^192`). A column accumulates `O(N²)` such products with small
//! ordering multipliers, so the running column sum can exceed 192 bits — a
//! 256-bit accumulator (`acc_lo: u128` + `acc_hi: u128`) holds it with ample
//! headroom (`N ≤ 64` ⇒ at most a few thousand 192-bit terms ⇒ < 2^210). After a
//! column emits its low 64 bits, the accumulator shifts down 64 bits to become
//! the carry into the next column. All arithmetic is plain `u64`/`u128`, so the
//! kernel is `const fn` — it can therefore be wired on the `const` `cube`
//! dispatch path WITHOUT de-const-ing the public API (unlike the u128-`LimbSize`
//! cube the policy doc rules out).

use crate::int::types::Uint;

/// `x³ mod 2^(64·N)` via a single fused product-scanning pass.
///
/// Bit-identical to [`crate::int::algos::cube::cube_schoolbook::cube_schoolbook`]
/// (and to the low `N` limbs of the exact `x·x·x`) for every input. `const fn`.
/// Wired by [`crate::int::policy::cube`] at the `N == 2` tier.
#[inline]
pub(crate) const fn cube_fused_comba<const N: usize>(x: Uint<N>) -> Uint<N> {
    let limbs = x.as_limbs();
    let mut out = [0u64; N];

    // 256-bit running accumulator: acc_lo holds bits 0..128, acc_hi bits
    // 128..256. A triple product (≤ 192 bits) and the per-column sum fit
    // comfortably (N ≤ 64).
    let mut acc_lo: u128 = 0;
    let mut acc_hi: u128 = 0;

    let mut col = 0;
    while col < N {
        // Unordered triples {i ≤ j ≤ k} with i + j + k == col, each i,j,k < N.
        // Since col < N, j = ... and k = col - i - j are automatically < N.
        let mut i = 0;
        while 3 * i <= col {
            let xi = limbs[i] as u128;
            let mut j = i;
            // j ranges so that k = col - i - j >= j (i.e. i + 2j <= col).
            while i + 2 * j <= col {
                let k = col - i - j;
                let xj = limbs[j] as u128;
                let xk = limbs[k] as u128;

                // p = x_i · x_j · x_k, up to 192 bits, held as (p_lo:u128, p_hi:u64).
                let ab = xi * xj; // ≤ (2^64−1)² < 2^128
                let ab_lo = ab as u64 as u128;
                let ab_hi = ab >> 64;
                // (ab_lo + ab_hi·2^64) · xk
                let t0 = ab_lo * xk; // ≤ 2^128
                let t1 = ab_hi * xk; // ≤ 2^128
                // p = t0 + (t1 << 64)
                let t1_lo = (t1 as u64 as u128) << 64; // low 64 of t1, shifted to bits 64..128
                let t1_hi = t1 >> 64; // bits 128..192 contribution
                let (p_lo, c0) = t0.overflowing_add(t1_lo);
                let p_hi: u128 = t1_hi + (c0 as u128); // ≤ 2^64

                // Ordering multiplier: distinct → 6, two equal → 3, all equal → 1.
                let reps = if i == j && j == k {
                    1
                } else if i == j || j == k {
                    3
                } else {
                    6
                };

                // acc += reps · (p_lo + p_hi·2^128), added one copy at a time
                // (reps ≤ 6 — the same overflowing-add idiom as `sqr_low_fixed`,
                // no 256-bit multiply needed).
                let mut r = 0;
                while r < reps {
                    let (s_lo, cc) = acc_lo.overflowing_add(p_lo);
                    acc_lo = s_lo;
                    acc_hi = acc_hi.wrapping_add(p_hi).wrapping_add(cc as u128);
                    r += 1;
                }

                j += 1;
            }
            i += 1;
        }

        out[col] = acc_lo as u64;
        // Shift the 256-bit accumulator down 64 bits.
        acc_lo = (acc_lo >> 64) | ((acc_hi & ((1u128 << 64) - 1)) << 64);
        acc_hi >>= 64;
        col += 1;
    }

    Uint::<N>::from_limbs(out)
}

#[cfg(test)]
mod tests {
    use super::cube_fused_comba;
    use crate::int::algos::cube::cube_schoolbook::cube_schoolbook;
    use crate::int::types::Uint;

    /// The candidate MUST be bit-identical to the shipped `cube_schoolbook`
    /// (`x²·x`) at every input — that equality is its whole contract. Differential
    /// check over a deterministic SplitMix64 spread at every storage width,
    /// including the all-ones carry worst case.
    fn diff_at<const N: usize>(seeds: &[u64]) {
        for &seed in seeds {
            let mut limbs = [0u64; N];
            let mut s = seed;
            for limb in limbs.iter_mut() {
                s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let mut z = s;
                z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                *limb = z ^ (z >> 31);
            }
            let x = Uint::<N>::from_limbs(limbs);
            let got = cube_fused_comba::<N>(x);
            let want = cube_schoolbook::<N>(x);
            assert_eq!(got.as_limbs(), want.as_limbs(), "N={N} seed={seed:#x}");
        }
    }

    fn all_ones_at<const N: usize>() {
        let x = Uint::<N>::from_limbs([u64::MAX; N]);
        let got = cube_fused_comba::<N>(x);
        let want = cube_schoolbook::<N>(x);
        assert_eq!(got.as_limbs(), want.as_limbs(), "all-ones N={N}");
    }

    #[test]
    fn cube_fused_matches_schoolbook_all_widths() {
        let seeds: [u64; 8] = [0, 1, 2, 3, 0xDEAD_BEEF, 0xFFFF_FFFF_FFFF_FFFF, 7, 0x1357_9BDF];
        diff_at::<1>(&seeds);
        diff_at::<2>(&seeds);
        diff_at::<3>(&seeds);
        diff_at::<4>(&seeds);
        diff_at::<6>(&seeds);
        diff_at::<8>(&seeds);
        diff_at::<16>(&seeds);
        all_ones_at::<1>();
        all_ones_at::<2>();
        all_ones_at::<3>();
        all_ones_at::<4>();
        all_ones_at::<8>();
        all_ones_at::<16>();
    }

    #[test]
    fn cube_small_exact_values() {
        // 3³ = 27.
        let got = cube_fused_comba::<2>(Uint::<2>::from_limbs([3, 0]));
        assert_eq!(got.as_limbs(), &[27, 0]);
        // 2³ = 8.
        let got = cube_fused_comba::<1>(Uint::<1>::from_limbs([2]));
        assert_eq!(got.as_limbs(), &[8]);
    }
}
