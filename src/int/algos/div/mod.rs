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
//!
//! One routing leaf sits over the engines rather than beside them:
//! [`div_rem_into`](div_rem_into::div_rem_into), the **exact-scratch door**.
//! Each engine's own blanket entry allocates `MAX_SINGLE_LIMBS` normalisation
//! scratch on every call — a size chosen by the build's width features — so a
//! caller that divides in a loop pays that memset per iteration at a width it
//! may never touch. `div_rem_into` takes the matcher's verdict and the
//! CALLER's buffers instead, so its cost tracks the operands.

pub(crate) mod div_burnikel_ziegler_with_knuth;
pub(crate) mod div_fixed;
// candidate (not wired): direct two's-complement i128 div_rem for N<=2,
// skips the sign-magnitude round trip Int::div_rem pays on both operands
// and both outputs. Sibling of int::algos::rem::rem_native_direct.
pub(crate) mod div_native_direct;
pub(crate) mod div_knuth;
// candidate (not wired): Knuth Algorithm D on u128 limbs (base 2^128) — the
// divide side of the LimbSize axis. Parked pending the div_kernel_ab verdict
// (whether the aligned u128 carry-chain beats base-2^64 despite the 4-mult
// q̂·v product). Bit-identical to div_knuth (its #[cfg(test)] differential).
pub(crate) mod div_knuth_u128_limb;
pub(crate) mod div_mg;
pub(crate) mod div_rem;
// the divide's EXACT-SCRATCH door: the matcher's verdict routed into the
// chosen engine's `_into` variant with the caller's normalisation buffers, so
// a caller that divides in a loop stops paying the build-max memset per
// iteration. Not an engine — a routing leaf over the engines above.
pub(crate) mod div_rem_into;
pub(crate) mod div_rem_schoolbook;

#[cfg(test)]
mod tests {
    // The recursive BZ core + public entry are only exercised by the
    // x-wide/xx-wide-gated differentials below (the recursion needs the wide
    // divide scratch), so the import is gated to match — otherwise the narrow
    // default build warns the names are unused.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    use super::div_burnikel_ziegler_with_knuth::{bz_recursive_core, div_burnikel_ziegler_with_knuth};
    use super::div_fixed::div_rem_mag_fixed;
    use super::div_knuth::div_knuth;
    use super::div_mg::{Mg2By1, Mg3By2};
    use super::div_rem::div_rem;
    use crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed;
    use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
    use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

    /// Pack a `[u128; N]` little-endian limb array into `[u64; 2*N]`.
    fn pack(limbs: &[u128]) -> Vec<u64> {
        let mut out = vec![0u64; 2 * limbs.len()];
        for (i, &limb) in limbs.iter().enumerate() {
            out[2 * i] = limb as u64;
            out[2 * i + 1] = (limb >> 64) as u64;
        }
        out
    }

    fn corpus() -> Vec<Vec<u128>> {
        vec![
            vec![0u128, 0, 0, 0],
            vec![1u128, 0, 0, 0],
            vec![u128::MAX, 0, 0, 0],
            vec![u128::MAX, u128::MAX, 0, 0],
            vec![u128::MAX, u128::MAX, u128::MAX, u128::MAX],
            vec![123u128, 456, 0, 0],
            vec![
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
        for dividend in corpus() {
            for divisor in corpus() {
                let dividend_limbs = pack(&dividend);
                let divisor_limbs = pack(&divisor);
                if is_zero(&divisor_limbs) {
                    continue;
                }
                let mut quotient_limbs = vec![0u64; dividend_limbs.len()];
                let mut remainder_limbs = vec![0u64; dividend_limbs.len()];
                div_rem(&dividend_limbs, &divisor_limbs, &mut quotient_limbs,
                    &mut remainder_limbs);

                let mut reconstructed =
                    vec![0u64; quotient_limbs.len() + divisor_limbs.len() + 1];
                mul_schoolbook(&quotient_limbs, &divisor_limbs, &mut reconstructed);
                let _ = add_assign(&mut reconstructed, &remainder_limbs);
                assert_eq!(&reconstructed[..dividend_limbs.len()], &dividend_limbs[..],
                    "q·den + r != num");
                assert!(reconstructed[dividend_limbs.len()..].iter().all(|&limb| limb == 0),
                    "recon overflow");
                assert!(cmp(&remainder_limbs, &divisor_limbs) < 0, "remainder >= divisor");
            }
        }
    }

    /// `div_knuth` agrees with the dispatch path on the corpus.
    #[test]
    fn knuth_matches_dispatch() {
        for dividend in corpus() {
            for divisor in corpus() {
                let dividend_limbs = pack(&dividend);
                let divisor_limbs = pack(&divisor);
                let mut divisor_len = divisor_limbs.len();
                while divisor_len > 0 && divisor_limbs[divisor_len - 1] == 0 {
                    divisor_len -= 1;
                }
                if divisor_len < 2 {
                    continue;
                }
                let mut quotient_reference = vec![0u64; dividend_limbs.len()];
                let mut remainder_reference = vec![0u64; dividend_limbs.len()];
                div_rem_dispatch(&dividend_limbs, &divisor_limbs,
                    &mut quotient_reference, &mut remainder_reference);

                let mut quotient_knuth = vec![0u64; dividend_limbs.len()];
                let mut remainder_knuth = vec![0u64; dividend_limbs.len()];
                div_knuth(&dividend_limbs, &divisor_limbs, &mut quotient_knuth,
                    &mut remainder_knuth);

                assert_eq!(quotient_knuth, quotient_reference, "knuth q mismatch");
                assert_eq!(remainder_knuth, remainder_reference, "knuth r mismatch");
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

            let dividend = vec![n0, n1, n2];
            let divisor = vec![d0, d1];
            let mut quotient_reference = vec![0u64; 3];
            let mut remainder_reference = vec![0u64; 3];
            div_rem(&dividend, &divisor, &mut quotient_reference,
                &mut remainder_reference);

            assert_eq!(quotient_reference[0], q, "Mg3By2 q mismatch");
            assert_eq!(quotient_reference[1], 0, "Mg3By2 q higher limb non-zero");
            assert_eq!(quotient_reference[2], 0, "Mg3By2 q higher limb non-zero");
            assert_eq!(remainder_reference[0], r0, "Mg3By2 r0 mismatch");
            assert_eq!(remainder_reference[1], r1, "Mg3By2 r1 mismatch");
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
            let dividend = ((u1 as u128) << 64) | (u0 as u128);
            let expected_quotient = (dividend / (d as u128)) as u64;
            let expected_remainder = (dividend % (d as u128)) as u64;
            assert_eq!((q, r), (expected_quotient, expected_remainder), "Mg2By1 mismatch");
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
        for (dividend, divisor) in cases {
            let mut quotient_canonical = [0u64; 8];
            let mut remainder_canonical = [0u64; 8];
            div_rem_dispatch(dividend, divisor, &mut quotient_canonical,
                &mut remainder_canonical);
            let mut quotient_knuth = [0u64; 8];
            let mut remainder_knuth = [0u64; 8];
            div_knuth(dividend, divisor, &mut quotient_knuth, &mut remainder_knuth);
            assert_eq!(quotient_canonical, quotient_knuth,
                "quotient mismatch on {:?} / {:?}", dividend, divisor);
            assert_eq!(remainder_canonical, remainder_knuth,
                "remainder mismatch on {:?} / {:?}", dividend, divisor);
        }
    }

    /// `div_burnikel_ziegler_with_knuth` agrees with Knuth on medium-and-
    /// large operands. Recursion engages only above the threshold cutoff.
    // 40-limb dividend / 20-limb divisor exceed the narrow build's div
    // scratch (sized for the compiled decimal tiers); runs at x-wide+.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    #[test]
    fn bz_matches_knuth() {
        let mut dividend = [0u64; 40];
        for (i, slot) in dividend.iter_mut().enumerate() {
            *slot = (i as u64)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(i as u64);
        }
        let mut divisor = [0u64; 20];
        for (i, slot) in divisor.iter_mut().enumerate() {
            *slot = ((i + 1) as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        }
        let mut quotient_canonical = [0u64; 40];
        let mut remainder_canonical = [0u64; 40];
        div_knuth(&dividend, &divisor, &mut quotient_canonical, &mut remainder_canonical);
        let mut quotient_bz = [0u64; 40];
        let mut remainder_bz = [0u64; 40];
        // Drive the recursive core directly (dividend=40 limbs, divisor=20
        // limbs, so dividend_len=40, n=20): this exercises the BZ
        // recursive-division path regardless of the production `BZ_THRESHOLD`
        // engagement value, so the differential survives a threshold that gates
        // the engine off.
        bz_recursive_core(&dividend, &divisor, &mut quotient_bz, &mut remainder_bz, 20, 40);
        assert_eq!(quotient_canonical, quotient_bz, "BZ quotient mismatch");
        assert_eq!(remainder_canonical, remainder_bz, "BZ remainder mismatch");
        // The public engine entry still agrees (whatever it dispatches to).
        let mut quotient_public = [0u64; 40];
        let mut remainder_public = [0u64; 40];
        div_burnikel_ziegler_with_knuth(&dividend, &divisor, &mut quotient_public,
            &mut remainder_public);
        assert_eq!(quotient_canonical, quotient_public, "BZ public-entry quotient mismatch");
        assert_eq!(remainder_canonical, remainder_public,
            "BZ public-entry remainder mismatch");
    }

    /// Knuth's q̂-cap path fires when `u_top >= v_top`.
    #[test]
    fn knuth_q_hat_cap_branch_matches_canonical() {
        let dividend: [u64; 4] = [0, 0, u64::MAX, u64::MAX >> 1];
        let divisor: [u64; 3] = [1, 2, u64::MAX >> 1];
        let mut quotient_canonical = [0u64; 4];
        let mut remainder_canonical = [0u64; 4];
        div_rem_dispatch(&dividend, &divisor, &mut quotient_canonical,
            &mut remainder_canonical);
        let mut quotient_knuth = [0u64; 4];
        let mut remainder_knuth = [0u64; 4];
        div_knuth(&dividend, &divisor, &mut quotient_knuth, &mut remainder_knuth);
        assert_eq!(quotient_canonical, quotient_knuth);
        assert_eq!(remainder_canonical, remainder_knuth);
    }

    /// `div_knuth` matches the independent `div_rem` shift-subtract oracle
    /// limb-for-limb across odd/even limb counts on both operands, the
    /// two-limb (single wide-digit) divisor edge, divisors with a zero top
    /// limb, and exact (zero-remainder) division.
    #[test]
    fn knuth_limb_count_boundaries_match_oracle() {
        let cases: &[(&[u64], &[u64])] = &[
            // even dividend / even divisor
            (&[1, 2, 3, 4], &[5, 6]),
            // odd dividend / even divisor
            (&[1, 2, 3, 4, 5], &[5, 6]),
            // even dividend / odd divisor
            (&[1, 2, 3, 4, 5, 6], &[7, 8, 9]),
            // odd dividend / odd divisor
            (&[1, 2, 3, 4, 5], &[7, 8, 9]),
            // 2-u64-limb divisor (single wide-digit edge)
            (&[u64::MAX, u64::MAX, u64::MAX, 0], &[3, 7]),
            (&[0, 0, 1, 1], &[u64::MAX, 1]),
            // two-limb divisor whose high u64 limb is large
            (&[u64::MAX, u64::MAX, u64::MAX, u64::MAX, 1], &[1, u64::MAX]),
            // 3-u64-limb divisor
            (&[u64::MAX, u64::MAX, u64::MAX, u64::MAX, u64::MAX, 0], &[1, 2, 3]),
            // divisor with a zero top limb (divisor[3] == 0)
            (&[1, 2, 3, 4, 5, 6, 7, 8], &[9, 10, 11, 0]),
            // dividend exactly divisible (zero remainder)
            (&[0, 0, 6, 0], &[0, 3]),
        ];
        for (dividend, divisor) in cases {
            let mut quotient_reference = [0u64; 12];
            let mut remainder_reference = [0u64; 12];
            div_rem(dividend, divisor, &mut quotient_reference, &mut remainder_reference);
            let mut quotient_knuth = [0u64; 12];
            let mut remainder_knuth = [0u64; 12];
            div_knuth(dividend, divisor, &mut quotient_knuth, &mut remainder_knuth);
            assert_eq!(quotient_knuth, quotient_reference,
                "quot mismatch {:?} / {:?}", dividend, divisor);
            assert_eq!(remainder_knuth, remainder_reference,
                "rem mismatch {:?} / {:?}", dividend, divisor);
        }
    }

    /// Randomised differential sweep over varied limb counts (odd and even
    /// for both operands, single- and multi-limb divisors) against the
    /// independent `div_rem` oracle. Catches normalisation / q̂ / carry
    /// regressions the fixed corpus might miss.
    // Operands up to ~10 limbs exceed the narrow build's div scratch.
    #[cfg(feature = "_wide-support")]
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
            let dividend_len = 2 + (next() % 9) as usize; // 2..=10 u64 limbs
            // 2..=dividend_len
            let divisor_len = 2 + (next() % (dividend_len as u64 - 1)) as usize;
            let mut dividend = vec![0u64; dividend_len];
            let mut divisor = vec![0u64; divisor_len];
            for slot in dividend.iter_mut() {
                *slot = next();
            }
            for slot in divisor.iter_mut() {
                *slot = next();
            }
            // Ensure divisor non-zero and has an effective high limb.
            if divisor.iter().all(|&limb| limb == 0) {
                divisor[0] = 1;
            }
            let mut quotient_reference = vec![0u64; dividend_len];
            let mut remainder_reference = vec![0u64; dividend_len];
            div_rem(&dividend, &divisor, &mut quotient_reference,
                &mut remainder_reference);
            let mut quotient_knuth = vec![0u64; dividend_len];
            let mut remainder_knuth = vec![0u64; dividend_len];
            div_knuth(&dividend, &divisor, &mut quotient_knuth, &mut remainder_knuth);
            assert_eq!(quotient_knuth, quotient_reference,
                "quot mismatch num={:?} den={:?}", dividend, divisor);
            assert_eq!(remainder_knuth, remainder_reference,
                "rem mismatch num={:?} den={:?}", dividend, divisor);
        }
    }

    /// BZ with a numerator that has trailing zero limbs strips them off
    /// before deciding whether to recurse.
    // 32-limb dividend / 20-limb divisor — needs the x-wide+ div scratch.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    #[test]
    fn bz_strips_numerator_trailing_zeros() {
        let mut dividend = [0u64; 32];
        for slot in &mut dividend[..16] {
            *slot = 0xCAFE_F00D;
        }
        let mut divisor = [0u64; 20];
        divisor[0] = 7;
        let mut quotient_canonical = [0u64; 32];
        let mut remainder_canonical = [0u64; 32];
        div_knuth(&dividend, &divisor, &mut quotient_canonical, &mut remainder_canonical);
        let mut quotient_bz = [0u64; 32];
        let mut remainder_bz = [0u64; 32];
        // Effective shape after stripping: dividend=16 limbs over divisor=1
        // limb. Drive the recursive core directly so the trailing-zero stripping
        // + single-limb base-case path is tested independent of `BZ_THRESHOLD`.
        bz_recursive_core(&dividend, &divisor, &mut quotient_bz, &mut remainder_bz, 1, 16);
        assert_eq!(quotient_canonical, quotient_bz);
        assert_eq!(remainder_canonical, remainder_bz);
        let mut quotient_public = [0u64; 32];
        let mut remainder_public = [0u64; 32];
        div_burnikel_ziegler_with_knuth(&dividend, &divisor, &mut quotient_public,
            &mut remainder_public);
        assert_eq!(quotient_canonical, quotient_public);
        assert_eq!(remainder_canonical, remainder_public);
    }

    // ── fast-arm wrappers ──────────────────────────────────────────────

    /// The `N == 1` and `N == 2` native fast arms agree limb-for-limb with
    /// the generic dispatch path over the divmod edge cases.
    #[test]
    fn fast_arm_div_rem_matches_generic() {
        let values1: [u64; 8] = [
            0,
            1,
            2,
            7,
            u64::MAX,
            u64::MAX - 1,
            0x8000_0000_0000_0000,
            123_456_789,
        ];
        for &dividend in &values1 {
            for &divisor in &values1 {
                if divisor == 0 {
                    continue;
                }
                let mut fixed_quotient = [0u64; 1];
                let mut fixed_remainder = [0u64; 1];
                div_rem_mag_fixed::<1>(&[dividend], &[divisor], &mut fixed_quotient,
                    &mut fixed_remainder);
                let mut generic_quotient = [0u64; 1];
                let mut generic_remainder = [0u64; 1];
                div_rem_dispatch(&[dividend], &[divisor], &mut generic_quotient,
                    &mut generic_remainder);
                assert_eq!(fixed_quotient, generic_quotient,
                    "N=1 quot mismatch {dividend}/{divisor}");
                assert_eq!(fixed_remainder, generic_remainder,
                    "N=1 rem mismatch {dividend}%{divisor}");
                assert_eq!(fixed_quotient[0], dividend / divisor);
                assert_eq!(fixed_remainder[0], dividend % divisor);
            }
        }

        let values2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            u128::MAX - 1,
            1u128 << 127,
            (1u128 << 64) - 1,
            1u128 << 64,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        let to_limbs = |value: u128| [value as u64, (value >> 64) as u64];
        for &dividend in &values2 {
            for &divisor in &values2 {
                if divisor == 0 {
                    continue;
                }
                let dividend_limbs = to_limbs(dividend);
                let divisor_limbs = to_limbs(divisor);
                let mut fixed_quotient = [0u64; 2];
                let mut fixed_remainder = [0u64; 2];
                div_rem_mag_fixed::<2>(&dividend_limbs, &divisor_limbs,
                    &mut fixed_quotient, &mut fixed_remainder);
                let mut generic_quotient = [0u64; 2];
                let mut generic_remainder = [0u64; 2];
                div_rem_dispatch(&dividend_limbs, &divisor_limbs, &mut generic_quotient,
                    &mut generic_remainder);
                assert_eq!(fixed_quotient, generic_quotient,
                    "N=2 quot mismatch {dividend}/{divisor}");
                assert_eq!(fixed_remainder, generic_remainder,
                    "N=2 rem mismatch {dividend}%{divisor}");
                assert_eq!(fixed_quotient, to_limbs(dividend / divisor));
                assert_eq!(fixed_remainder, to_limbs(dividend % divisor));
            }
        }
    }

    /// The native isqrt fast arms match the generic limb isqrt.
    #[test]
    fn fast_arm_isqrt_matches_generic() {
        let values1: [u64; 9] = [
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
        for &value in &values1 {
            let mut fixed_sqrt = [0u64; 1];
            isqrt_mag_fixed::<1>(&[value], &mut fixed_sqrt);
            let mut generic_sqrt = [0u64; 1];
            isqrt_newton(&[value], &mut generic_sqrt);
            assert_eq!(fixed_sqrt, generic_sqrt, "N=1 isqrt mismatch sqrt({value})");
            assert_eq!(fixed_sqrt[0], value.isqrt());
        }

        let values2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            (1u128 << 64) - 1,
            1u128 << 64,
            1u128 << 126,
            (u64::MAX as u128) * (u64::MAX as u128),
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        for &value in &values2 {
            let value_limbs = [value as u64, (value >> 64) as u64];
            let mut fixed_sqrt = [0u64; 2];
            isqrt_mag_fixed::<2>(&value_limbs, &mut fixed_sqrt);
            let mut generic_sqrt = [0u64; 2];
            isqrt_newton(&value_limbs, &mut generic_sqrt);
            assert_eq!(fixed_sqrt, generic_sqrt, "N=2 isqrt mismatch sqrt({value})");
            let expected_sqrt = value.isqrt();
            assert_eq!(fixed_sqrt, [expected_sqrt as u64, (expected_sqrt >> 64) as u64]);
        }
    }
}
