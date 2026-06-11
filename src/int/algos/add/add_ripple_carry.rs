// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Ripple-carry integer addition over little-endian `u64` limb arrays.
//!
//! [`add_ripple_carry`] is the width-agnostic addition algorithm selected by
//! the addition policy [`crate::int::policy::add::dispatch`]. Pure kernel —
//! it adds the two operands' limbs with carry propagation and wraps modulo
//! `2^BITS`; no algorithm choice.

use crate::int::algos::support::limbs::add_assign_fixed;
use crate::int::types::Int;

/// Ripple-carry integer addition for `Int<N>`. Wraps
/// [`add_assign_fixed`], discarding the carry-out so the result wraps
/// modulo `2^BITS` (two's-complement wrapping semantics).
#[inline]
pub(crate) const fn add_ripple_carry<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    let mut limbs = *a.as_limbs();
    add_assign_fixed(&mut limbs, b.as_limbs());
    Int::<N>::from_limbs(limbs)
}

/// Fused checked form of [`add_ripple_carry`]: the same ripple pass with
/// the two's-complement overflow verdict read off the operands' and
/// result's top bits in the SAME traversal, returning the `Option`
/// directly. Same algorithm and the same rule as `Int::checked_add`
/// ("both operands share a sign and the result's differs") — one pass.
/// The layered shape (`wrapping_add` → three `is_negative` reads →
/// `Option` rewrap) measured ≈2× the bare loop at 24 limbs (the bbc
/// `add D462` chain), almost all of it by-value array moves between the
/// layers; fusing removes them.
#[inline]
pub(crate) const fn add_ripple_carry_checked<const N: usize>(
    a: Int<N>,
    b: Int<N>,
) -> Option<Int<N>> {
    let av = a.as_limbs();
    let bv = b.as_limbs();
    let mut out = [0u64; N];
    let mut carry = 0u64;
    let mut i = 0;
    while i < N {
        let (s1, c1) = av[i].overflowing_add(bv[i]);
        let (s2, c2) = s1.overflowing_add(carry);
        out[i] = s2;
        carry = (c1 as u64) + (c2 as u64);
        i += 1;
    }
    let sa = av[N - 1] >> 63;
    let sb = bv[N - 1] >> 63;
    let sr = out[N - 1] >> 63;
    if sa == sb && sr != sa {
        None
    } else {
        Some(Int::<N>::from_limbs(out))
    }
}

#[cfg(test)]
mod tests {
    use super::add_ripple_carry;
    use crate::int::types::Int;

    /// 1 + 1 = 2 in a single-limb Int<1>.
    #[test]
    fn add_simple_single_limb() {
        let a = Int::<1>::from_i64(1);
        let b = Int::<1>::from_i64(1);
        let got = add_ripple_carry(a, b);
        assert_eq!(got.as_i128(), 2);
    }

    /// Carry propagates across the limb boundary: u64::MAX + 1 in Int<2>
    /// must produce 2^64 exactly (limb[0] = 0, limb[1] = 1).
    #[test]
    fn add_carry_across_limb_boundary() {
        let a = Int::<2>::from_u128(u64::MAX as u128);
        let b = Int::<2>::from_i64(1);
        let got = add_ripple_carry(a, b);
        assert_eq!(got.as_i128(), 1_i128 << 64);
    }

    /// Wrapping: MAX<1> + 1 wraps to MIN<1> (i64::MAX + 1 = i64::MIN).
    #[test]
    fn add_wraps_at_max() {
        let a = Int::<1>::from_i64(i64::MAX);
        let b = Int::<1>::from_i64(1);
        let got = add_ripple_carry(a, b);
        assert_eq!(got.as_i128(), i64::MIN as i128);
    }

    /// Adding zero is identity (both operand positions).
    #[test]
    fn add_zero_identity() {
        let v = Int::<2>::from_i128(i128::MIN);
        let zero = Int::<2>::from_i64(0);
        assert_eq!(add_ripple_carry(v, zero).as_i128(), i128::MIN);
        assert_eq!(add_ripple_carry(zero, v).as_i128(), i128::MIN);
    }

    /// Negative + positive cancel to zero across widths.
    #[test]
    fn add_negative_positive_cancel() {
        let pos = Int::<3>::from_i64(999_999);
        let neg = Int::<3>::from_i64(-999_999);
        let got = add_ripple_carry(pos, neg);
        assert_eq!(got.as_i128(), 0);
    }

    /// Fused checked kernel agrees with the layered rule at the signed
    /// boundaries: in-range sums are `Some(exact)`, positive and negative
    /// overflow are `None`, mixed signs never overflow.
    #[test]
    fn add_checked_fused_boundaries() {
        use super::add_ripple_carry_checked;
        // Normal sum.
        let got = add_ripple_carry_checked(Int::<2>::from_i128(7), Int::<2>::from_i128(35));
        assert_eq!(got.map(|v| v.as_i128()), Some(42));
        // MAX + 1 overflows positive.
        assert!(add_ripple_carry_checked(Int::<2>::from_i128(i128::MAX), Int::<2>::from_i128(1)).is_none());
        // MIN + (-1) overflows negative.
        assert!(add_ripple_carry_checked(Int::<2>::from_i128(i128::MIN), Int::<2>::from_i128(-1)).is_none());
        // MAX + MIN = -1 (mixed signs can't overflow).
        let got = add_ripple_carry_checked(Int::<2>::from_i128(i128::MAX), Int::<2>::from_i128(i128::MIN));
        assert_eq!(got.map(|v| v.as_i128()), Some(-1));
        // Multi-limb carry into the top limb without overflow (Int<3>).
        let a = Int::<3>::from_i128(i128::MAX);
        let got = add_ripple_carry_checked(a, a);
        assert!(got.is_some(), "2*(i128::MAX) fits 192 bits");
    }
}
