// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Two's-complement integer negation over little-endian `u64` limb arrays.
//!
//! [`neg_twos_complement`] is the width-agnostic negation algorithm selected
//! by the negation policy [`crate::int::policy::neg::dispatch`]. Pure kernel
//! — bitwise-NOT plus carry-propagating `+1`, wrapping modulo `2^BITS`; no
//! algorithm choice.

use crate::int::types::Int;

/// Two's-complement negation for `Int<N>`: bitwise-NOT plus a
/// carry-propagating `+1`, wrapping modulo `2^BITS`. `MIN` maps to
/// itself, matching the primitive signed integer `wrapping_neg`
/// contract.
///
/// Limb-0 split shape — the routed kernel after the wide-tier
/// `neg_kernel_ab` A/B (see `benches/micro/neg_kernel_ab.rs`):
///
/// 1. Compute `out[0] = !a[0] + 1`, capturing the carry `c0`.
/// 2. If `c0 == false` (the overwhelmingly common path — `a[0] != MAX`),
///    limbs `1..N` reduce to plain independent `!a[i]` writes: no
///    cross-limb dependency chain, so the compiler can keep them
///    register-resident / vectorise the NOT loop.
/// 3. If `c0 == true` (`a[0] == MAX`), fall back to a dependent
///    carry-prop chain through limbs `1..N`.
///
/// The previous two-pass shape (NOT loop into `out[N]`, then a
/// full-width `add_assign_fixed(out, [1, 0, …, 0])`) paid a serialised
/// carry chain across all N limbs on every call AND wrote a second
/// stack array. The new shape collapses to one NOT loop with `+1` on
/// limb 0 for typical inputs — at D462/D616/D924/D1232 the
/// `neg_kernel_ab` ranking shows 1.40x-1.83x speed-ups across realistic
/// (tiny, half-wide, mid, high) input mixes. Generic over `N`,
/// const-fn so it stays available in const contexts (`abs`,
/// `wrapping_div`, `wrapping_rem`, `from_mag_limbs`).
#[inline]
pub(crate) const fn neg_twos_complement<const N: usize>(a: Int<N>) -> Int<N> {
    let mut out = [0u64; N];
    if N == 0 {
        return Int::<N>::from_limbs(out);
    }
    let limbs = a.as_limbs();
    let (s0, c0) = (!limbs[0]).overflowing_add(1);
    out[0] = s0;
    if c0 {
        // limb 0 was MAX — carry continues through the dependent chain.
        let mut carry: u64 = 1;
        let mut i = 1;
        while i < N {
            let (s, c) = (!limbs[i]).overflowing_add(carry);
            out[i] = s;
            carry = c as u64;
            i += 1;
        }
        // `carry` discarded — that is the modulo-2^BITS wrap (MIN → MIN).
        let _ = carry;
    } else {
        // Common path: just NOT the remaining limbs — independent
        // writes, no cross-limb dependency chain.
        let mut i = 1;
        while i < N {
            out[i] = !limbs[i];
            i += 1;
        }
    }
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
