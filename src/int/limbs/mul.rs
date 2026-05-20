//! Width-matched limb multiplication primitives.
//!
//! The full-width products (`limbs_mul_u64*`) are re-exported from
//! [`crate::wide_int`]; see [`super`] for the re-export rationale. The
//! truncated low-`N` product [`limbs_mul_low_u64_fixed`] is new here:
//! the existing `limbs_mul_u64` writes every `out[i + j]` and a
//! cross-row carry tail unconditionally, so it cannot serve a product
//! that keeps only the low `N` output limbs without scribbling past the
//! buffer. This routine writes exactly the low `N` limbs and discards
//! everything that would land at or above `2^(64·N)` — the modular
//! reduction the wrapping operators want.

#[allow(unused_imports)]
pub(crate) use crate::wide_int::{
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
