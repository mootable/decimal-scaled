// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Truncated-low symmetric squaring kernel, generic over the limb type.
//!
//! The [`Limb`]-generic sibling of the const u64 comba square
//! [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`]: it computes the
//! same `(x²) mod 2^(64·N)` but runs base-2^128 for `L = u128` (the operand
//! packed into `⌈N/2⌉` u128 limbs — half the limbs, ≈¼ the partial products,
//! at the cost of a wider 128×128→256 inner step), then unpacks. Bit-identical
//! to the low `N` u64 limbs of `x·x` at either width.
//!
//! Like the multiply pilot [`crate::int::algos::mul::mul_schoolbook::mul_low_limb`],
//! the [`crate::int::types::compute_limbs::LimbSize`] axis (owned by
//! [`crate::int::policy::sqr_low`]) picks `L`; this kernel holds neither the
//! choice nor a per-limb-type copy. The u128 arm requires **even `N`**
//! (`L::packed_len` halves it); the policy gates that via `LimbSize::for_packing`.
//!
//! ## Symmetry
//!
//! A square has symmetric cross terms (`x_i·x_j == x_j·x_i`), so the result is
//! `Σ x_i² · B^(2i)  +  2 · Σ_{i<j} x_i·x_j · B^(i+j)` (`B = 2^bits(L)`). It is
//! formed in three truncated-low passes over a [`Limb`]-typed scratch buffer:
//!   1. accumulate the off-diagonal products `x_i·x_j` (`i<j`) ONCE;
//!   2. double the accumulator (the `2·Σ` factor);
//!   3. add the diagonal squares `x_i²` at position `2i`.
//! Everything above limb `h = packed_len(N)` lies above `2^(64·N)` and is
//! truncated away. This forms ≈ `h²/2` packed products — the symmetry halving
//! that a general packed `x·x` ([`mul_low_limb`]) would NOT get, and the reason
//! the packed square can beat the u64 comba (a general packed square would lose
//! to it).
//!
//! Carry handling mirrors [`mul_low_limb`]: `overflowing_add` for the limb sum,
//! `add_carries` to merge the two boolean carries into the next product's high
//! limb (which is `≤ MAX − 1`, so the merge never overflows), and `Limb::ONE`
//! to materialise a carry bit as a limb value for the doubling / diagonal
//! propagation.
//!
//! Like [`mul_low_limb`], the truncated-low result touches only the low
//! `h = packed_len(N) ≤ N` limbs, so the accumulator is a local `[L; N]` —
//! NOT a `ComputeLimbs`/`double_buffered` scratch (which the wider `2N` *full*
//! product would need). So this kernel takes no `ComputeLimbs` bound, and neither
//! does its [`crate::int::policy::sqr_low`] dispatch.
//!
//! [`mul_low_limb`]: crate::int::algos::mul::mul_schoolbook::mul_low_limb

use crate::int::types::compute_limbs::Limb;

/// `out = (x²) mod 2^(64·N)` — truncated-low symmetric square in limb type `L`.
///
/// Operands and the accumulator are local `[L; N]` arrays (`[L; N]` covers
/// `packed_len(N) ≤ N` for both limb types; stable Rust cannot put `N/2` in an
/// array length, so the high `N − h` slots stay unused). `out` is fully written.
#[inline]
pub(crate) fn sqr_low_limb<const N: usize, L: Limb>(value: &[u64; N], out: &mut [u64; N]) {
    let h = L::packed_len(N);
    let mut value_packed = [L::ZERO; N];
    L::pack(value, &mut value_packed[..h]);

    let mut acc = [L::ZERO; N];

    // Pass 1: off-diagonal products x_i·x_j (i < j), accumulated ONCE into
    // acc[0..h]. Row idiom (carry is always a VALUE = the previous product's
    // high limb), truncated once the column index reaches `h`.
    let mut i = 0;
    while i < h {
        let value_limb = value_packed[i];
        if value_limb != L::ZERO {
            let mut carry = L::ZERO;
            let mut j = i + 1;
            while i + j < h {
                let idx = i + j;
                let (lo, hi) = value_limb.widening_mul(value_packed[j]);
                let (sum1, carry1) = acc[idx].overflowing_add(lo);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                acc[idx] = sum2;
                carry = hi.add_carries(carry1, carry2);
                j += 1;
            }
            // The trailing `carry` would land at column `i + j ≥ h` — above the
            // truncation width — so it is dropped, exactly as in `mul_low_limb`.
        }
        i += 1;
    }

    // Pass 2: acc[0..h] ·= 2 (the `2 · Σ_{i<j}` factor). `2·acc[k]` carries its
    // top bit out; the incoming carry bit is added as a `Limb::ONE` value.
    let mut double_carry = L::ZERO;
    let mut k = 0;
    while k < h {
        let (sum1, carry1) = acc[k].overflowing_add(acc[k]);
        let (sum2, carry2) = sum1.overflowing_add(double_carry);
        acc[k] = sum2;
        // `2·acc[k] == MAX` ⇒ carry1=0, and `+1` cannot wrap ⇒ carry2=0;
        // `acc[k]==MAX` ⇒ carry1=1, sum1=MAX−1, `+double_carry(≤1)` ⇒ carry2=0.
        // So carry1,carry2 are never both set.
        double_carry = if carry1 || carry2 { L::ONE } else { L::ZERO };
        k += 1;
    }

    // Pass 3: add the diagonal squares x_i² at position 2i, truncated at `h`.
    let mut i = 0;
    while 2 * i < h {
        let pos = 2 * i;
        let (lo, hi) = value_packed[i].widening_mul(value_packed[i]);
        let (sum, mut propagate) = acc[pos].overflowing_add(lo);
        acc[pos] = sum;
        let mut carry_pos = pos + 1;
        // Add the high limb `hi` plus the low limb's carry at `pos + 1`, then
        // ripple the carry upward — all truncated at `h`.
        if carry_pos < h {
            let (sum1, carry1) = acc[carry_pos].overflowing_add(hi);
            let (sum2, carry2) =
                sum1.overflowing_add(if propagate { L::ONE } else { L::ZERO });
            acc[carry_pos] = sum2;
            propagate = carry1 || carry2;
            carry_pos += 1;
            while propagate && carry_pos < h {
                let (sum3, carry3) = acc[carry_pos].overflowing_add(L::ONE);
                acc[carry_pos] = sum3;
                propagate = carry3;
                carry_pos += 1;
            }
        }
        i += 1;
    }

    L::unpack(&acc[..h], out);
}

#[cfg(test)]
mod tests {
    use super::sqr_low_limb;
    use crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed;

    /// `sqr_low_limb::<N, u128>` and `::<N, u64>` must each be bit-identical to
    /// the const u64 comba [`sqr_low_fixed`] — that equality is the kernel's
    /// whole contract (golden does not exercise wide int squaring directly).
    /// Differential check over a deterministic SplitMix64 spread at the even
    /// widths the u128 arm targets, including the all-ones carry worst case.
    fn diff_at<const N: usize>(seeds: &[u64]) {
        for &seed in seeds {
            let mut value = [0u64; N];
            let mut state = seed;
            for limb in value.iter_mut() {
                state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let mut z = state;
                z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                *limb = z ^ (z >> 31);
            }
            let mut expected = [0u64; N];
            sqr_low_fixed::<N>(&value, &mut expected);
            let mut actual_u64 = [0u64; N];
            sqr_low_limb::<N, u64>(&value, &mut actual_u64);
            assert_eq!(actual_u64, expected, "u64 N={N} seed={seed:#x}");
            let mut actual_u128 = [0u64; N];
            sqr_low_limb::<N, u128>(&value, &mut actual_u128);
            assert_eq!(actual_u128, expected, "u128 N={N} seed={seed:#x}");
        }
    }

    fn all_ones_at<const N: usize>() {
        let value = [u64::MAX; N];
        let mut expected = [0u64; N];
        sqr_low_fixed::<N>(&value, &mut expected);
        let mut actual_u128 = [0u64; N];
        sqr_low_limb::<N, u128>(&value, &mut actual_u128);
        assert_eq!(actual_u128, expected, "u128 all-ones N={N}");
        let mut actual_u64 = [0u64; N];
        sqr_low_limb::<N, u64>(&value, &mut actual_u64);
        assert_eq!(actual_u64, expected, "u64 all-ones N={N}");
    }

    #[test]
    fn sqr_low_limb_matches_sqr_low_fixed_even_widths() {
        let seeds: [u64; 8] = [0, 1, 2, 3, 0xDEAD_BEEF, 0xFFFF_FFFF_FFFF_FFFF, 7, 0x1357_9BDF];
        // Even widths (the u128 packing requirement). Covers the storage and
        // work integers the wide-exp Smith square runs on.
        diff_at::<2>(&seeds);
        diff_at::<4>(&seeds);
        diff_at::<6>(&seeds);
        diff_at::<8>(&seeds);
        diff_at::<16>(&seeds);
        diff_at::<32>(&seeds);
        // N=64 is a routed `U128` cell (D307 exp work / D1232 storage); pin its
        // u128/u64 bit-identity to the comba reference.
        diff_at::<64>(&seeds);
        all_ones_at::<2>();
        all_ones_at::<4>();
        all_ones_at::<8>();
        all_ones_at::<16>();
        all_ones_at::<32>();
        all_ones_at::<64>();
    }
}
