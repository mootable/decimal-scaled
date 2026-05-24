// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer division algorithm family over little-endian `u64` limb slices.
//!
//! The pure division *engines* — each performs one named algorithm on an
//! already-chosen basis; the divisor-shape *choice* between them lives in
//! [`crate::int::policy::div_rem`]:
//!
//! - [`div_rem`](div_rem::div_rem) — `const fn` single-/double-limb
//!   hardware divide (and the shift-subtract fallback for the rare const
//!   multi-limb case). The const-evaluable `wrapping_div` / `wrapping_rem`
//!   stay on this so they can run at compile time.
//! - [`div_knuth`](div_knuth::div_knuth) — Knuth Algorithm D (TAOCP Vol 2
//!   §4.3.1) at base 2⁶⁴, q̂ estimated with the Möller–Granlund 2-by-1
//!   reciprocal [`Mg2By1`](div_mg::Mg2By1).
//! - [`div_burnikel_ziegler_with_knuth`](div_burnikel_ziegler_with_knuth::div_burnikel_ziegler_with_knuth)
//!   — Burnikel–Ziegler outer chunking that recurses to `div_knuth` as its
//!   base case.
//! - [`div_mg`] — the Möller–Granlund invariant-divisor reciprocal engines
//!   ([`Mg2By1`](div_mg::Mg2By1) / [`Mg3By2`](div_mg::Mg3By2)), the per-q̂
//!   estimators the wider engines build on.
//!
//! [`div_rem_mag_fixed`](div_fixed::div_rem_mag_fixed) /
//! [`div_rem_mag_slice`](div_fixed::div_rem_mag_slice) are the const-`N`
//! fast-arm wrappers the fixed-width `Int<N>` types call. The integer
//! square root fast-arm wrapper `isqrt_mag_fixed` and the Newton kernel
//! `isqrt_newton` live with the [`crate::int::algos::isqrt`] family.

pub(crate) mod div_burnikel_ziegler_with_knuth;
pub(crate) mod div_fixed;
pub(crate) mod div_knuth;
pub(crate) mod div_mg;
pub(crate) mod div_rem;
pub(crate) mod div_rem_schoolbook;

/// Scratch capacity for the runtime u64-limb division engines — 288 u64
/// limbs (18432 bits), covering the widest work integer in the crate
/// (Int<256> used by D1232 cbrt, 256 u64 limbs) with slack.
pub(crate) const SCRATCH_LIMBS: usize = 288;

#[cfg(test)]
mod tests {
    use super::div_burnikel_ziegler_with_knuth::{
        bz_chunk_core, div_burnikel_ziegler_with_knuth,
    };
    use super::div_fixed::div_rem_mag_fixed;
    use super::div_knuth::div_knuth;
    use super::div_mg::{Mg2By1, Mg3By2};
    use super::div_rem::div_rem;
    use crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed;
    use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
    use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

    /// Pack a `[u128; N]` little-endian limb array into `[u64; 2*N]`.
    fn pack(limbs: &[u128]) -> alloc::vec::Vec<u64> {
        let mut out = alloc::vec![0u64; 2 * limbs.len()];
        for (i, &l) in limbs.iter().enumerate() {
            out[2 * i] = l as u64;
            out[2 * i + 1] = (l >> 64) as u64;
        }
        out
    }

    fn corpus() -> alloc::vec::Vec<alloc::vec::Vec<u128>> {
        alloc::vec![
            alloc::vec![0u128, 0, 0, 0],
            alloc::vec![1u128, 0, 0, 0],
            alloc::vec![u128::MAX, 0, 0, 0],
            alloc::vec![u128::MAX, u128::MAX, 0, 0],
            alloc::vec![u128::MAX, u128::MAX, u128::MAX, u128::MAX],
            alloc::vec![123u128, 456, 0, 0],
            alloc::vec![
                0x1234_5678_9abc_def0_fedc_ba98_7654_3210_u128,
                0xa5a5_a5a5_5a5a_5a5a_3c3c_3c3c_c3c3_c3c3,
                0,
                0,
            ],
        ]
    }

    /// Verify the Euclidean identity `num == q·den + r` with
    /// `0 <= r < den` reconstructs across the corpus.
    #[test]
    fn div_rem_satisfies_identity() {
        use crate::int::algos::support::limbs::{add_assign, cmp, is_zero};
        use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
        for num in corpus() {
            for den in corpus() {
                let n64 = pack(&num);
                let d64 = pack(&den);
                if is_zero(&d64) {
                    continue;
                }
                let mut q64 = alloc::vec![0u64; n64.len()];
                let mut r64 = alloc::vec![0u64; n64.len()];
                div_rem(&n64, &d64, &mut q64, &mut r64);

                let mut recon = alloc::vec![0u64; q64.len() + d64.len() + 1];
                mul_schoolbook(&q64, &d64, &mut recon);
                let _ = add_assign(&mut recon, &r64);
                assert_eq!(&recon[..n64.len()], &n64[..], "q·den + r != num");
                assert!(recon[n64.len()..].iter().all(|&x| x == 0), "recon overflow");
                assert!(cmp(&r64, &d64) < 0, "remainder >= divisor");
            }
        }
    }

    /// `div_knuth` agrees with the dispatch path on the corpus.
    #[test]
    fn knuth_matches_dispatch() {
        for num in corpus() {
            for den in corpus() {
                let n64 = pack(&num);
                let d64 = pack(&den);
                let mut dn = d64.len();
                while dn > 0 && d64[dn - 1] == 0 {
                    dn -= 1;
                }
                if dn < 2 {
                    continue;
                }
                let mut q_ref = alloc::vec![0u64; n64.len()];
                let mut r_ref = alloc::vec![0u64; n64.len()];
                div_rem_dispatch(&n64, &d64, &mut q_ref, &mut r_ref);

                let mut q_knuth = alloc::vec![0u64; n64.len()];
                let mut r_knuth = alloc::vec![0u64; n64.len()];
                div_knuth(&n64, &d64, &mut q_knuth, &mut r_knuth);

                assert_eq!(q_knuth, q_ref, "knuth q mismatch");
                assert_eq!(r_knuth, r_ref, "knuth r mismatch");
            }
        }
    }

    /// `Mg3By2` matches the `div_rem` oracle on a representative corpus.
    #[test]
    fn mg3by2_matches_reference() {
        let cases: &[(u64, u64, u64, u64, u64)] = &[
            (0, 0, 1, 1u64 << 63, 0),
            (0, 1, 0, 1u64 << 63, 0),
            ((1u64 << 63) - 1, u64::MAX, u64::MAX, 1u64 << 63, 1),
            (u64::MAX - 1, u64::MAX, u64::MAX, u64::MAX, u64::MAX),
            (0, 0, 1, u64::MAX, 1),
            (
                0xc0ffee,
                0xdead_beef,
                0xface_b00c,
                (1u64 << 63) | 0xc0ffee_u64,
                0xdead_beef_face_b00c,
            ),
            (0, 1, 2, (1u64 << 63) | 1, 2),
        ];
        for &(n2, n1, n0, d1, d0) in cases {
            assert!(d1 >> 63 == 1, "d1 not normalised: {d1:#x}");
            assert!(
                n2 < d1 || (n2 == d1 && n1 < d0),
                "test precondition (n2, n1) < (d1, d0) violated"
            );
            let mg = Mg3By2::new(d1, d0);
            let (q, r1, r0) = mg.div_rem(n2, n1, n0);

            let num = alloc::vec![n0, n1, n2];
            let den = alloc::vec![d0, d1];
            let mut q_ref = alloc::vec![0u64; 3];
            let mut r_ref = alloc::vec![0u64; 3];
            div_rem(&num, &den, &mut q_ref, &mut r_ref);

            assert_eq!(q_ref[0], q, "Mg3By2 q mismatch");
            assert_eq!(q_ref[1], 0, "Mg3By2 q higher limb non-zero");
            assert_eq!(q_ref[2], 0, "Mg3By2 q higher limb non-zero");
            assert_eq!(r_ref[0], r0, "Mg3By2 r0 mismatch");
            assert_eq!(r_ref[1], r1, "Mg3By2 r1 mismatch");
        }
    }

    /// `Mg2By1` matches a reference 2-by-1 divide.
    #[test]
    fn mg2by1_matches_reference() {
        let cases: &[(u64, u64, u64)] = &[
            (0, 1, 1u64 << 63),
            (0, u64::MAX, 1u64 << 63),
            ((1u64 << 63) - 1, u64::MAX, 1u64 << 63),
            (0, 1, u64::MAX),
            (u64::MAX - 1, u64::MAX, u64::MAX),
            (12345, 67890, (1u64 << 63) | 0xdead_beef_u64),
            (u64::MAX - 1, 0, u64::MAX),
        ];
        for &(u1, u0, d) in cases {
            assert!(d >> 63 == 1);
            assert!(u1 < d);
            let mg = Mg2By1::new(d);
            let (q, r) = mg.div_rem(u1, u0);
            let num = ((u1 as u128) << 64) | (u0 as u128);
            let exp_q = (num / (d as u128)) as u64;
            let exp_r = (num % (d as u128)) as u64;
            assert_eq!((q, r), (exp_q, exp_r), "Mg2By1 mismatch");
        }
    }

    /// `div_knuth` agrees with the dispatch path on a battery of shapes.
    #[test]
    fn knuth_matches_canonical_divmod() {
        let cases: &[(&[u64], &[u64])] = &[
            (&[42], &[7]),
            (&[u64::MAX, 0], &[2]),
            (&[1, 1, 0, 0], &[3]),
            (&[u64::MAX, u64::MAX, 1, 0], &[5, 9]),
            (&[u64::MAX, u64::MAX, u64::MAX, 0], &[1, 2, 3]),
            (&[100, 0, 0], &[200, 0, 1]),
            (&[0, 0, u64::MAX, u64::MAX], &[1, 2, u64::MAX]),
        ];
        for (num, den) in cases {
            let mut q_canon = [0u64; 8];
            let mut r_canon = [0u64; 8];
            div_rem_dispatch(num, den, &mut q_canon, &mut r_canon);
            let mut q_knuth = [0u64; 8];
            let mut r_knuth = [0u64; 8];
            div_knuth(num, den, &mut q_knuth, &mut r_knuth);
            assert_eq!(q_canon, q_knuth, "quotient mismatch on {:?} / {:?}", num, den);
            assert_eq!(r_canon, r_knuth, "remainder mismatch on {:?} / {:?}", num, den);
        }
    }

    /// `div_burnikel_ziegler_with_knuth` agrees with Knuth on medium-and-
    /// large operands. Recursion engages only above the threshold cutoff.
    #[test]
    fn bz_matches_knuth() {
        let mut num = [0u64; 40];
        for (i, slot) in num.iter_mut().enumerate() {
            *slot = (i as u64)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(i as u64);
        }
        let mut den = [0u64; 20];
        for (i, slot) in den.iter_mut().enumerate() {
            *slot = ((i + 1) as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        }
        let mut q_canon = [0u64; 40];
        let mut r_canon = [0u64; 40];
        div_knuth(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_bz = [0u64; 40];
        let mut r_bz = [0u64; 40];
        // Drive the chunking core directly (num=40 limbs, den=20 limbs, so
        // top=40, n=20): this exercises the BZ block-division path
        // regardless of the production `BZ_THRESHOLD` engagement value, so
        // the differential survives a threshold that gates the engine off.
        bz_chunk_core(&num, &den, &mut q_bz, &mut r_bz, 20, 40);
        assert_eq!(q_canon, q_bz, "BZ quotient mismatch");
        assert_eq!(r_canon, r_bz, "BZ remainder mismatch");
        // The public engine entry still agrees (whatever it dispatches to).
        let mut q_pub = [0u64; 40];
        let mut r_pub = [0u64; 40];
        div_burnikel_ziegler_with_knuth(&num, &den, &mut q_pub, &mut r_pub);
        assert_eq!(q_canon, q_pub, "BZ public-entry quotient mismatch");
        assert_eq!(r_canon, r_pub, "BZ public-entry remainder mismatch");
    }

    /// Knuth's q̂-cap path fires when `u_top >= v_top`.
    #[test]
    fn knuth_q_hat_cap_branch_matches_canonical() {
        let num: [u64; 4] = [0, 0, u64::MAX, u64::MAX >> 1];
        let den: [u64; 3] = [1, 2, u64::MAX >> 1];
        let mut q_canon = [0u64; 4];
        let mut r_canon = [0u64; 4];
        div_rem_dispatch(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_knuth = [0u64; 4];
        let mut r_knuth = [0u64; 4];
        div_knuth(&num, &den, &mut q_knuth, &mut r_knuth);
        assert_eq!(q_canon, q_knuth);
        assert_eq!(r_canon, r_knuth);
    }

    /// `div_knuth` matches the independent `div_rem` shift-subtract oracle
    /// limb-for-limb across odd/even limb counts on both operands, the
    /// two-limb (single wide-digit) divisor edge, divisors with a zero top
    /// limb, and exact (zero-remainder) division.
    #[test]
    fn knuth_limb_count_boundaries_match_oracle() {
        let cases: &[(&[u64], &[u64])] = &[
            // even num / even den
            (&[1, 2, 3, 4], &[5, 6]),
            // odd num / even den
            (&[1, 2, 3, 4, 5], &[5, 6]),
            // even num / odd den
            (&[1, 2, 3, 4, 5, 6], &[7, 8, 9]),
            // odd num / odd den
            (&[1, 2, 3, 4, 5], &[7, 8, 9]),
            // 2-u64-limb divisor (single wide-digit edge)
            (&[u64::MAX, u64::MAX, u64::MAX, 0], &[3, 7]),
            (&[0, 0, 1, 1], &[u64::MAX, 1]),
            // two-limb divisor whose high u64 limb is large
            (&[u64::MAX, u64::MAX, u64::MAX, u64::MAX, 1], &[1, u64::MAX]),
            // 3-u64-limb divisor
            (&[u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, 0], &[1, 2, 3]),
            // divisor with a zero top limb (den[3] == 0)
            (&[1, 2, 3, 4, 5, 6, 7, 8], &[9, 10, 11, 0]),
            // num exactly divisible (zero remainder)
            (&[0, 0, 6, 0], &[0, 3]),
        ];
        for (num, den) in cases {
            let mut q_ref = [0u64; 12];
            let mut r_ref = [0u64; 12];
            div_rem(num, den, &mut q_ref, &mut r_ref);
            let mut q_k = [0u64; 12];
            let mut r_k = [0u64; 12];
            div_knuth(num, den, &mut q_k, &mut r_k);
            assert_eq!(q_k, q_ref, "quot mismatch {:?} / {:?}", num, den);
            assert_eq!(r_k, r_ref, "rem mismatch {:?} / {:?}", num, den);
        }
    }

    /// Randomised differential sweep over varied limb counts (odd and even
    /// for both operands, single- and multi-limb divisors) against the
    /// independent `div_rem` oracle. Catches normalisation / q̂ / carry
    /// regressions the fixed corpus might miss.
    #[test]
    fn knuth_random_differential_match_oracle() {
        // Deterministic xorshift so the sweep is reproducible.
        let mut state: u64 = 0x243F_6A88_85A3_08D3;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..3000 {
            let num_len = 2 + (next() % 9) as usize; // 2..=10 u64 limbs
            let den_len = 2 + (next() % (num_len as u64 - 1)) as usize; // 2..=num_len
            let mut num = alloc::vec![0u64; num_len];
            let mut den = alloc::vec![0u64; den_len];
            for x in num.iter_mut() {
                *x = next();
            }
            for x in den.iter_mut() {
                *x = next();
            }
            // Ensure divisor non-zero and has an effective high limb.
            if den.iter().all(|&x| x == 0) {
                den[0] = 1;
            }
            let mut q_ref = alloc::vec![0u64; num_len];
            let mut r_ref = alloc::vec![0u64; num_len];
            div_rem(&num, &den, &mut q_ref, &mut r_ref);
            let mut q_k = alloc::vec![0u64; num_len];
            let mut r_k = alloc::vec![0u64; num_len];
            div_knuth(&num, &den, &mut q_k, &mut r_k);
            assert_eq!(q_k, q_ref, "quot mismatch num={:?} den={:?}", num, den);
            assert_eq!(r_k, r_ref, "rem mismatch num={:?} den={:?}", num, den);
        }
    }

    /// BZ with a numerator that has trailing zero limbs strips them off
    /// before deciding whether to recurse.
    #[test]
    fn bz_strips_numerator_trailing_zeros() {
        let mut num = [0u64; 32];
        for slot in &mut num[..16] {
            *slot = 0xCAFE_F00D;
        }
        let mut den = [0u64; 20];
        den[0] = 7;
        let mut q_canon = [0u64; 32];
        let mut r_canon = [0u64; 32];
        div_knuth(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_bz = [0u64; 32];
        let mut r_bz = [0u64; 32];
        // Effective shape after stripping: num=16 limbs over den=20 limbs.
        // Drive the core directly so the trailing-zero stripping + single
        // sub-divisor chunk path is tested independent of `BZ_THRESHOLD`.
        bz_chunk_core(&num, &den, &mut q_bz, &mut r_bz, 1, 16);
        assert_eq!(q_canon, q_bz);
        assert_eq!(r_canon, r_bz);
        let mut q_pub = [0u64; 32];
        let mut r_pub = [0u64; 32];
        div_burnikel_ziegler_with_knuth(&num, &den, &mut q_pub, &mut r_pub);
        assert_eq!(q_canon, q_pub);
        assert_eq!(r_canon, r_pub);
    }

    // ── fast-arm wrappers ──────────────────────────────────────────────

    /// The `N == 1` and `N == 2` native fast arms agree limb-for-limb with
    /// the generic dispatch path over the divmod edge cases.
    #[test]
    fn fast_arm_div_rem_matches_generic() {
        let vals1: [u64; 8] = [
            0,
            1,
            2,
            7,
            u64::MAX,
            u64::MAX - 1,
            0x8000_0000_0000_0000,
            123_456_789,
        ];
        for &num in &vals1 {
            for &den in &vals1 {
                if den == 0 {
                    continue;
                }
                let mut fq = [0u64; 1];
                let mut fr = [0u64; 1];
                div_rem_mag_fixed::<1>(&[num], &[den], &mut fq, &mut fr);
                let mut gq = [0u64; 1];
                let mut gr = [0u64; 1];
                div_rem_dispatch(&[num], &[den], &mut gq, &mut gr);
                assert_eq!(fq, gq, "N=1 quot mismatch {num}/{den}");
                assert_eq!(fr, gr, "N=1 rem mismatch {num}%{den}");
                assert_eq!(fq[0], num / den);
                assert_eq!(fr[0], num % den);
            }
        }

        let vals2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            u128::MAX - 1,
            1u128 << 127,
            (1u128 << 64) - 1,
            1u128 << 64,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        let to_limbs = |v: u128| [v as u64, (v >> 64) as u64];
        for &num in &vals2 {
            for &den in &vals2 {
                if den == 0 {
                    continue;
                }
                let n = to_limbs(num);
                let d = to_limbs(den);
                let mut fq = [0u64; 2];
                let mut fr = [0u64; 2];
                div_rem_mag_fixed::<2>(&n, &d, &mut fq, &mut fr);
                let mut gq = [0u64; 2];
                let mut gr = [0u64; 2];
                div_rem_dispatch(&n, &d, &mut gq, &mut gr);
                assert_eq!(fq, gq, "N=2 quot mismatch {num}/{den}");
                assert_eq!(fr, gr, "N=2 rem mismatch {num}%{den}");
                assert_eq!(fq, to_limbs(num / den));
                assert_eq!(fr, to_limbs(num % den));
            }
        }
    }

    /// The native isqrt fast arms match the generic limb isqrt.
    #[test]
    fn fast_arm_isqrt_matches_generic() {
        let vals1: [u64; 9] = [
            0,
            1,
            2,
            3,
            4,
            15,
            16,
            u64::MAX,
            (u32::MAX as u64) * (u32::MAX as u64),
        ];
        for &v in &vals1 {
            let mut f = [0u64; 1];
            isqrt_mag_fixed::<1>(&[v], &mut f);
            let mut g = [0u64; 1];
            isqrt_newton(&[v], &mut g);
            assert_eq!(f, g, "N=1 isqrt mismatch sqrt({v})");
            assert_eq!(f[0], v.isqrt());
        }

        let vals2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            (1u128 << 64) - 1,
            1u128 << 64,
            1u128 << 126,
            (u64::MAX as u128) * (u64::MAX as u128),
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        for &v in &vals2 {
            let n = [v as u64, (v >> 64) as u64];
            let mut f = [0u64; 2];
            isqrt_mag_fixed::<2>(&n, &mut f);
            let mut g = [0u64; 2];
            isqrt_newton(&n, &mut g);
            assert_eq!(f, g, "N=2 isqrt mismatch sqrt({v})");
            let r = v.isqrt();
            assert_eq!(f, [r as u64, (r >> 64) as u64]);
        }
    }
}
