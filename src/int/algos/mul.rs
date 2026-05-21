//! Width-matched limb multiplication primitives.
//!
//! The full-width products (`limbs_mul_u64*`) are re-exported from
//! [`crate::int::limbs`]; see [`super`] for the re-export rationale. The
//! truncated low-`N` product [`limbs_mul_low_u64_fixed`] is new here:
//! the existing `limbs_mul_u64` writes every `out[i + j]` and a
//! cross-row carry tail unconditionally, so it cannot serve a product
//! that keeps only the low `N` output limbs without scribbling past the
//! buffer. This routine writes exactly the low `N` limbs and discards
//! everything that would land at or above `2^(64·N)` — the modular
//! reduction the wrapping operators want.

#[allow(unused_imports)]
pub(crate) use crate::int::limbs::{
    limbs_mul_u64, limbs_mul_u64_fixed, limbs_mul_u64_into,
};

/// `out = (a · b) mod 2^(64·N)` — the low `N` limbs of the schoolbook
/// product, with the high half never formed.
///
/// `out` must be zeroed by the caller. For each operand limb `a[i]`,
/// the inner loop runs only while `i + j < N`; products that would land
/// in limb `N` or above are exactly the bits above the width and are
/// dropped, including the final row carry. Bit-identical to the low `N`
/// limbs of [`limbs_mul_u64_fixed`].
#[inline]
pub(crate) const fn limbs_mul_low_u64_fixed<const N: usize>(
    a: &[u64; N],
    b: &[u64; N],
    out: &mut [u64; N],
) {
    let mut i = 0;
    while i < N {
        let ai = a[i];
        if ai != 0 {
            let mut carry: u64 = 0;
            let mut j = 0;
            // Stop once `i + j` reaches `N`: those partial products lie
            // entirely above `2^(64·N)` and drop out of the result.
            while j < N - i {
                let v = (ai as u128) * (b[j] as u128)
                    + (out[i + j] as u128)
                    + (carry as u128);
                out[i + j] = v as u64;
                carry = (v >> 64) as u64;
                j += 1;
            }
            // The final row carry would land in limb `i + (N - i) = N`,
            // which is above the width — discarded.
        }
        i += 1;
    }
}

/// `out = (x²) mod 2^(64·N)` — dedicated truncated squaring.
///
/// `out` must be zeroed by the caller. A square has symmetric cross
/// terms (`x_i·x_j == x_j·x_i`), so each `i < j` partial product is
/// formed once and added twice (the doubling), while the `i == j`
/// diagonals `x_i²` are added once. Limb-multiply count drops from
/// `N²` (general mul) to `N(N+1)/2`. As with
/// [`limbs_mul_low_u64_fixed`], any product whose column index reaches
/// `N` is above the width and is dropped, so only the low `N` limbs are
/// touched. Bit-identical to the low `N` limbs of `x · x`.
///
/// Each `add_at` folds a `(hi, lo)` pair into `out[col]` / `out[col+1]`
/// and propagates the carry through the remaining low limbs. The
/// cross-term doubling is realised by calling `add_at` twice rather
/// than a separate shift, which keeps the carry handling — the only
/// fiddly part of squaring — identical to the diagonal path.
#[inline]
pub(crate) const fn limbs_sqr_low_u64_fixed<const N: usize>(
    x: &[u64; N],
    out: &mut [u64; N],
) {
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
