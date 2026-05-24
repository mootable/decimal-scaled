// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Two's-complement integer negation over little-endian `u64` limb arrays.
//!
//! [`neg_twos_complement`] is the width-agnostic negation algorithm selected
//! by the negation policy [`crate::int::policy::neg::dispatch`]. Pure kernel
//! — bitwise-NOT then carry-propagating increment, wrapping modulo `2^BITS`;
//! no algorithm choice.

use crate::int::algos::support::limbs::add_assign_fixed;
use crate::int::types::Int;

/// Two's-complement negation for `Int<N>`: bitwise-NOT then increment
/// by one, wrapping modulo `2^BITS`. `MIN` maps to itself, matching
/// the primitive signed integer `wrapping_neg` contract.
///
/// Uses [`add_assign_fixed`] for the increment step so the carry
/// propagation is the same const-safe kernel the add policy uses.
#[inline]
pub(crate) const fn neg_twos_complement<const N: usize>(a: Int<N>) -> Int<N> {
    let mut out = [0u64; N];
    let mut i = 0;
    while i < N {
        out[i] = !a.as_limbs()[i];
        i += 1;
    }
    let mut one = [0u64; N];
    if N > 0 {
        one[0] = 1;
    }
    add_assign_fixed(&mut out, &one);
    Int::<N>::from_limbs(out)
}

#[cfg(test)]
mod tests {
    use super::neg_twos_complement;
    use crate::int::types::Int;

    /// neg(0) = 0.
    #[test]
    fn neg_zero_is_zero() {
        let z = Int::<1>::from_i64(0);
        assert_eq!(neg_twos_complement(z).as_i128(), 0);
    }

    /// neg(1) = -1 in single-limb Int<1>.
    #[test]
    fn neg_one_single_limb() {
        let a = Int::<1>::from_i64(1);
        assert_eq!(neg_twos_complement(a).as_i128(), -1);
    }

    /// neg(-1) = 1.
    #[test]
    fn neg_minus_one() {
        let a = Int::<1>::from_i64(-1);
        assert_eq!(neg_twos_complement(a).as_i128(), 1);
    }

    /// neg(MIN) = MIN (wrapping: two's-complement MIN is its own negation).
    #[test]
    fn neg_min_wraps_to_min() {
        let m = Int::<1>::from_i64(i64::MIN);
        assert_eq!(neg_twos_complement(m).as_i128(), i64::MIN as i128);
    }

    /// Double negation is identity across a multi-limb width.
    #[test]
    fn neg_double_is_identity() {
        let v = Int::<3>::from_i128(i128::MAX);
        let once = neg_twos_complement(v);
        let twice = neg_twos_complement(once);
        assert_eq!(twice.as_i128(), i128::MAX);
    }

    /// Carry propagates correctly: neg of 2^64 in Int<2> = -(2^64).
    #[test]
    fn neg_carry_across_limb_boundary() {
        let a = Int::<2>::from_u128(1_u128 << 64);
        let got = neg_twos_complement(a);
        // -(2^64) as i128
        assert_eq!(got.as_i128(), -(1_i128 << 64));
    }
}
