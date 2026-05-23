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
/// `out` must be zeroed by the caller. A square has symmetric cross terms
/// (`x_i·x_j == x_j·x_i`), so each `i < j` partial product is formed once
/// and added twice (the doubling), while the `i == j` diagonals `x_i²`
/// are added once. Limb-multiply count drops from `N²` (general mul) to
/// `N(N+1)/2`. As with `mul_low_fixed`, any product whose column index
/// reaches `N` is above the width and is dropped, so only the low `N`
/// limbs are touched. Bit-identical to the low `N` limbs of `x · x`.
///
/// Each `add_at` folds a `(hi, lo)` pair into `out[col]` / `out[col+1]`
/// and propagates the carry through the remaining low limbs. The
/// cross-term doubling is realised by calling `add_at` twice rather than
/// a separate shift, which keeps the carry handling — the only fiddly
/// part of squaring — identical to the diagonal path.
#[inline]
pub(crate) const fn sqr_low_fixed<const N: usize>(x: &[u64; N], out: &mut [u64; N]) {
    // Fold `value` (a u128 partial product) into the low limbs starting
    // at `col`, propagating carry until exhausted or past the width.
    #[inline(always)]
    const fn add_at<const N: usize>(out: &mut [u64; N], col: usize, value: u128) {
        if col >= N {
            return;
        }
        let mut idx = col;
        let mut carry = value;
        while carry != 0 && idx < N {
            let v = (out[idx] as u128) + (carry & 0xFFFF_FFFF_FFFF_FFFF);
            out[idx] = v as u64;
            // Surviving carry = high 64 of this column's sum plus the
            // high 64 of the incoming value that has not been consumed.
            carry = (v >> 64) + (carry >> 64);
            idx += 1;
        }
    }

    let mut i = 0;
    while i < N {
        let xi = x[i] as u128;
        if xi != 0 {
            // Diagonal square at column 2i (added once).
            add_at::<N>(out, i + i, xi * xi);
            // Doubled cross terms x_i·x_j for j > i at column i+j.
            let mut j = i + 1;
            while i + j < N {
                let prod = xi * (x[j] as u128);
                add_at::<N>(out, i + j, prod);
                add_at::<N>(out, i + j, prod);
                j += 1;
            }
        }
        i += 1;
    }
}
