// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Truncated half-product squaring kernel.
//!
//! The dedicated low-`N` squaring kernel that
//! [`crate::int::algos::sqr::sqr_half_product`] (and, via the
//! square-and-multiply loop, [`crate::int::algos::pow`] /
//! [`crate::int::algos::cube`]) compute on. It is the *kernel* the
//! squaring algorithm uses; the per-`N` choice lives in
//! [`crate::int::policy::sqr`].

/// `out = (x²) mod 2^(64·N)` — dedicated truncated squaring.
///
/// Product-scanning (comba) squaring: the output is built one limb at a
/// time, lowest first, by summing every partial product that lands in that
/// column into a running accumulator. A square has symmetric cross terms
/// (`x_i·x_j == x_j·x_i`), so for column `col` only the pairs `i ≤ j` with
/// `i + j == col` are formed: each `i < j` cross term once and doubled, the
/// `i == j` diagonal `x_i²` once. That is ≈ `N²/4` limb-multiplies — half of
/// the `N²/2` a general `x·x` ([`mul_low_fixed`]) forms — and the running
/// accumulator threads the carry one column at a time, so there is no
/// per-product carry walk. Any column index reaching `N` is above the width
/// and never visited, so only the low `N` limbs are touched. Bit-identical
/// to the low `N` limbs of `x · x`. `out` is fully overwritten (it need not
/// be pre-zeroed, though callers may pass a zeroed buffer).
///
/// The accumulator holds `acc + (hi << 128)`: `acc` is the live 128-bit
/// sum, `hi` the small overflow above bit 128 (at most a few units per
/// column, since each add carries ≤ 1 and there are `O(N)` adds). After a
/// column is emitted, the accumulator is shifted down 64 bits — the carry
/// into the next column — and `hi` folds back into the top.
#[inline]
pub(crate) const fn sqr_low_fixed<const N: usize>(x: &[u64; N], out: &mut [u64; N]) {
    let mut acc: u128 = 0;
    let mut hi: u64 = 0;
    let mut col = 0;
    while col < N {
        // Pairs (i, j) with i ≤ j and i + j == col. j = col - i < N holds
        // for every col < N, so no explicit width guard is needed here.
        let mut i = 0;
        while 2 * i <= col {
            let j = col - i;
            let p = (x[i] as u128) * (x[j] as u128);
            // Diagonal once; off-diagonal twice (the symmetry doubling).
            let reps = if i == j { 1 } else { 2 };
            let mut r = 0;
            while r < reps {
                let (s, c) = acc.overflowing_add(p);
                acc = s;
                hi += c as u64;
                r += 1;
            }
            i += 1;
        }
        out[col] = acc as u64;
        // Shift the accumulator down one limb: the surviving high bits are
        // the carry into the next column. `hi << 64` cannot overflow u128
        // (hi is a tiny per-column overflow count) and occupies bits ≥ 64,
        // disjoint from `acc >> 64` (bits < 64).
        acc = (acc >> 64) + ((hi as u128) << 64);
        hi = 0;
        col += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::sqr_low_fixed;
    use crate::int::algos::mul::mul_schoolbook::mul_low_fixed;

    /// `sqr_low_fixed` must be bit-identical to the low `N` limbs of the
    /// general `x · x` ([`mul_low_fixed`]) — that equality is the kernel's
    /// whole contract, and golden does not exercise wide int squaring
    /// directly. Differential check over a deterministic SplitMix64 spread
    /// at every storage width, including the all-ones worst case for carry
    /// propagation.
    fn diff_at<const N: usize>(seeds: &[u64]) {
        for &seed in seeds {
            let mut x = [0u64; N];
            let mut s = seed;
            for limb in x.iter_mut() {
                // SplitMix64 step.
                s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let mut z = s;
                z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                *limb = z ^ (z >> 31);
            }
            let mut got = [0u64; N];
            sqr_low_fixed::<N>(&x, &mut got);
            let mut want = [0u64; N];
            mul_low_fixed::<N>(&x, &x, &mut want);
            assert_eq!(got, want, "N={N} seed={seed:#x} x={x:?}");
        }
    }

    /// All-ones operand: every column saturates, the worst case for the
    /// running-accumulator carry.
    fn all_ones_at<const N: usize>() {
        let x = [u64::MAX; N];
        let mut got = [0u64; N];
        sqr_low_fixed::<N>(&x, &mut got);
        let mut want = [0u64; N];
        mul_low_fixed::<N>(&x, &x, &mut want);
        assert_eq!(got, want, "all-ones N={N}");
    }

    #[test]
    fn sqr_matches_mul_low_fixed_all_widths() {
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
        all_ones_at::<4>();
        all_ones_at::<8>();
        all_ones_at::<16>();
    }

    #[test]
    fn sqr_small_exact_values() {
        // Hand-checked low limbs.
        let mut out = [0u64; 2];
        sqr_low_fixed::<2>(&[3, 0], &mut out);
        assert_eq!(out, [9, 0]); // 3² = 9
        sqr_low_fixed::<2>(&[0, 1], &mut out);
        assert_eq!(out, [0, 0]); // (2^64)² ≡ 0 mod 2^128
        sqr_low_fixed::<2>(&[u64::MAX, 0], &mut out);
        // (2^64-1)² = 2^128 - 2^65 + 1 → low 128 = -2^65 + 1
        assert_eq!(out, [1, 0xFFFF_FFFF_FFFF_FFFE]);
    }
}
