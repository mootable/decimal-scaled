// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! XOR-fold integer equality — CANDIDATE, not wired.
//!
//! Candidate sibling of [`crate::int::algos::eq::eq_limbwise`]. The shipped
//! equality reuses the *comparison* kernel (`cmp_fixed(a, b) == 0`), which does
//! the full sign-then-magnitude `<` / `>` work that ordering needs. Equality
//! needs strictly less: it only has to answer "does ANY limb differ", which is
//! `(a₀ ^ b₀) | (a₁ ^ b₁) | … == 0`.
//!
//! This OR-fold of per-limb XORs:
//!   - drops the two ordering comparisons per limb that `cmp_fixed` performs;
//!   - is branchless (a single reduction), so it has no per-limb branch and the
//!     compiler can auto-vectorise the fold;
//!   - reads every limb (no MSB-first short-circuit), which is the right trade
//!     for equality: unequal values are typically caught by the fold in one
//!     pass, and the loop has no data-dependent control flow to mispredict.
//!
//! It is registered as an UNWIRED, UNROUTED candidate (`#[allow(dead_code)]`)
//! per `docs/ARCHITECTURE.md` → "Keeping the alternatives". The `eq` policy does
//! not route it; only the `#[cfg(test)]` bit-identity check below exercises it.
//! A later benchmark pass compares it against `eq_limbwise` and, if it wins,
//! wires it via an `Algorithm` arm. The expected win is small (equality is
//! already cheap) but it is a genuinely distinct, marginally cheaper algorithm
//! — not an alias of the comparison kernel.

use crate::int::types::Int;

/// Two's-complement equality for `Int<N>` via an OR-fold of per-limb XORs.
///
/// Bit-identical to [`crate::int::algos::eq::eq_limbwise::eq_limbwise`] for
/// every input: two `Int<N>` of identical width are equal iff every limb is
/// equal, i.e. iff the OR of the per-limb XORs is zero. (Two's-complement
/// values of the same width have a unique limb representation, so a limbwise
/// equality is a value equality — no sign special-case is needed.) `const fn`.
#[allow(dead_code)]
#[inline]
pub(crate) const fn eq_xor_fold<const N: usize>(a: Int<N>, b: Int<N>) -> bool {
    let al = a.as_limbs();
    let bl = b.as_limbs();
    let mut diff: u64 = 0;
    let mut i = 0;
    while i < N {
        diff |= al[i] ^ bl[i];
        i += 1;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::eq_xor_fold;
    use crate::int::algos::eq::eq_limbwise::eq_limbwise;
    use crate::int::types::Int;

    /// The candidate MUST agree with the shipped `eq_limbwise` on every pair —
    /// that equality is its whole contract. Differential check over a
    /// deterministic SplitMix64 spread at several widths, exercising equal pairs,
    /// pairs differing only in the top limb, pairs differing only in the bottom
    /// limb, and sign-boundary values.
    fn diff_at<const N: usize>(seeds: &[u64]) {
        for &seed in seeds {
            let mut la = [0u64; N];
            let mut s = seed;
            for limb in la.iter_mut() {
                s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
                let mut z = s;
                z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                *limb = z ^ (z >> 31);
            }
            let a = Int::<N>::from_limbs(la);

            // equal to itself
            assert_eq!(eq_xor_fold::<N>(a, a), eq_limbwise::<N>(a, a));
            assert!(eq_xor_fold::<N>(a, a));

            // differ in the bottom limb
            let mut lb = la;
            lb[0] ^= 1;
            let b = Int::<N>::from_limbs(lb);
            assert_eq!(eq_xor_fold::<N>(a, b), eq_limbwise::<N>(a, b));
            assert!(!eq_xor_fold::<N>(a, b));

            // differ in the top limb
            let mut lc = la;
            lc[N - 1] ^= 1 << 63;
            let c = Int::<N>::from_limbs(lc);
            assert_eq!(eq_xor_fold::<N>(a, c), eq_limbwise::<N>(a, c));
            assert!(!eq_xor_fold::<N>(a, c));
        }
    }

    #[test]
    fn eq_xor_fold_matches_limbwise_all_widths() {
        let seeds: [u64; 6] = [1, 2, 3, 0xDEAD_BEEF, 0xFFFF_FFFF_FFFF_FFFF, 0x1357_9BDF];
        diff_at::<1>(&seeds);
        diff_at::<2>(&seeds);
        diff_at::<3>(&seeds);
        diff_at::<4>(&seeds);
        diff_at::<8>(&seeds);
        diff_at::<16>(&seeds);
    }

    #[test]
    fn eq_xor_fold_sign_boundaries() {
        // MIN vs MIN, MAX vs MAX, MIN vs MAX, 0 vs 0, 0 vs -1.
        let min = Int::<2>::from_i128(i128::MIN);
        let max = Int::<2>::from_i128(i128::MAX);
        let zero = Int::<2>::from_i64(0);
        let neg1 = Int::<2>::from_i64(-1);
        assert!(eq_xor_fold::<2>(min, min));
        assert!(eq_xor_fold::<2>(max, max));
        assert!(!eq_xor_fold::<2>(min, max));
        assert!(eq_xor_fold::<2>(zero, zero));
        assert!(!eq_xor_fold::<2>(zero, neg1));
    }
}
