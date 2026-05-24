// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer multiplication algorithm family.
//!
//! - [`mul_schoolbook`] — base-2⁶⁴ schoolbook outer product, plus its
//!   fixed-width (`mul_schoolbook_fixed`), single-word
//!   (`mul_schoolbook_into`) and truncated-low (`mul_low_fixed`)
//!   specialisations.
//! - [`mul_karatsuba`] — non-allocating recursive Karatsuba.
//!
//! The schoolbook-vs-Karatsuba *choice* (and its crossover threshold)
//! lives in [`crate::int::policy::mul`]; these kernels stay pure.
//!
//! [`mul_schoolbook`]: crate::int::algos::mul::mul_schoolbook
//! [`mul_karatsuba`]: crate::int::algos::mul::mul_karatsuba

pub(crate) mod mul_karatsuba;
pub(crate) mod mul_schoolbook;

#[cfg(test)]
mod tests {
    use super::mul_karatsuba::{
        karatsuba_scratch_needed_th, mul_karatsuba, mul_karatsuba_with_threshold,
        KARATSUBA_SCRATCH_LIMBS,
    };
    use super::mul_schoolbook::{
        mul_low_fixed, mul_low_fixed_u128, mul_schoolbook, mul_schoolbook_fixed,
        mul_schoolbook_into,
    };

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

    /// `mul_karatsuba` matches `mul_schoolbook` on equal-length operands
    /// across the carry-stressing corpus at a forced low threshold (so the
    /// split/recombine algebra is exercised even at narrow widths).
    #[test]
    fn karatsuba_matches_schoolbook() {
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                let n = a64.len().min(b64.len());
                let mut a_buf = alloc::vec![0u64; n];
                let mut b_buf = alloc::vec![0u64; n];
                a_buf.copy_from_slice(&a64[..n]);
                b_buf.copy_from_slice(&b64[..n]);
                let mut out_school = alloc::vec![0u64; 2 * n];
                let mut out_kara = alloc::vec![0u64; 2 * n];
                mul_schoolbook(&a_buf, &b_buf, &mut out_school);
                mul_karatsuba_with_threshold(&a_buf, &b_buf, &mut out_kara, 4);
                assert_eq!(out_kara, out_school, "Karatsuba mismatch at n={n}");
            }
        }
    }

    /// Non-allocating Karatsuba is bit-exact against the schoolbook oracle
    /// [`mul_schoolbook`] over a large seeded corpus across every width the
    /// crate multiplies, including odd, threshold-boundary, and the
    /// 256-limb maximum. The recursion is driven at small thresholds so
    /// the full split/recombine algebra is exercised even at the narrow
    /// widths. Commutativity (`a·b == b·a`) is asserted in the same pass.
    #[test]
    fn nonalloc_karatsuba_bit_exact_vs_schoolbook() {
        // SplitMix64 — Vigna 2014, public-domain reference algorithm.
        let mut state: u64 = 0x5EED_1234_ABCD_0F0F;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };

        const WIDTHS: &[usize] = &[
            1, 2, 4, 7, 8, 15, 16, 17, 31, 32, 33, 48, 64, 96, 128, 255, 256,
        ];
        const THRESHOLDS: &[usize] = &[4, 8, 16, 24, 256];

        let edge_fill = |buf: &mut [u64], kind: usize, next: &mut dyn FnMut() -> u64| match kind {
            0 => buf.iter_mut().for_each(|x| *x = 0),
            1 => buf.iter_mut().for_each(|x| *x = u64::MAX),
            2 => {
                buf.iter_mut().for_each(|x| *x = 0);
                if let Some(last) = buf.last_mut() {
                    *last = u64::MAX;
                }
            }
            3 => {
                buf.iter_mut().for_each(|x| *x = 0);
                buf[0] = u64::MAX;
            }
            _ => buf.iter_mut().for_each(|x| *x = next()),
        };

        for &n in WIDTHS {
            let random_pairs = if n <= 16 {
                400
            } else if n <= 64 {
                120
            } else {
                30
            };

            let mut pairs: alloc::vec::Vec<(alloc::vec::Vec<u64>, alloc::vec::Vec<u64>)> =
                alloc::vec::Vec::new();
            for ka in 0..5 {
                for kb in 0..5 {
                    let mut a = alloc::vec![0u64; n];
                    let mut b = alloc::vec![0u64; n];
                    edge_fill(&mut a, ka, &mut next);
                    edge_fill(&mut b, kb, &mut next);
                    pairs.push((a, b));
                }
            }
            for _ in 0..random_pairs {
                let mut a = alloc::vec![0u64; n];
                let mut b = alloc::vec![0u64; n];
                for x in a.iter_mut() {
                    *x = next();
                }
                for x in b.iter_mut() {
                    *x = next();
                }
                pairs.push((a, b));
            }

            for (a, b) in &pairs {
                let mut oracle = alloc::vec![0u64; 2 * n];
                mul_schoolbook(a, b, &mut oracle);

                for &th in THRESHOLDS {
                    let mut got = alloc::vec![0u64; 2 * n];
                    mul_karatsuba_with_threshold(a, b, &mut got, th);
                    assert_eq!(
                        got, oracle,
                        "non-alloc Karatsuba mismatch at n={n}, threshold={th}\na={a:?}\nb={b:?}"
                    );

                    let mut got_swapped = alloc::vec![0u64; 2 * n];
                    mul_karatsuba_with_threshold(b, a, &mut got_swapped, th);
                    assert_eq!(
                        got_swapped, oracle,
                        "non-alloc Karatsuba not commutative at n={n}, threshold={th}"
                    );
                }
            }
        }
    }

    /// The widest equal-length multiply (256 limbs, Int<256>) routes
    /// through the production [`mul_karatsuba`] entry — which declares the
    /// fixed `[u64; KARATSUBA_SCRATCH_LIMBS]` stack buffer — without
    /// tripping the scratch-overflow `debug_assert` and matches schoolbook.
    /// Guards the scratch sizing against future threshold drops that deepen
    /// the recursion.
    #[test]
    fn nonalloc_karatsuba_max_width_fits_fixed_scratch() {
        let mut state: u64 = 0xC0FF_EE00_1357_9BDF;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        assert!(
            karatsuba_scratch_needed_th(256, 8) <= KARATSUBA_SCRATCH_LIMBS,
            "fixed scratch too small for n=256 at a threshold of 8"
        );

        let n = 256;
        let mut a = alloc::vec![0u64; n];
        let mut b = alloc::vec![0u64; n];
        for x in a.iter_mut() {
            *x = next();
        }
        for x in b.iter_mut() {
            *x = next();
        }
        let mut oracle = alloc::vec![0u64; 2 * n];
        let mut got = alloc::vec![0u64; 2 * n];
        mul_schoolbook(&a, &b, &mut oracle);
        // Production entry: real fixed stack scratch, production threshold.
        mul_karatsuba(&a, &b, &mut got, crate::int::policy::mul::karatsuba_threshold());
        assert_eq!(got, oracle, "max-width Karatsuba mismatch via fixed scratch");
    }

    /// `mul_schoolbook_fixed::<L, D>` matches `mul_schoolbook` at a
    /// representative set of compile-time `L` values covering every wide
    /// tier (D38..D1232).
    #[test]
    fn mul_schoolbook_fixed_matches_slice() {
        macro_rules! check {
            ($L:expr, $D:expr) => {{
                for a in corpus() {
                    for b in corpus() {
                        let a64 = pack(&a);
                        let b64 = pack(&b);
                        if a64.len() < $L || b64.len() < $L {
                            continue;
                        }
                        let mut a_arr = [0u64; $L];
                        let mut b_arr = [0u64; $L];
                        a_arr.copy_from_slice(&a64[..$L]);
                        b_arr.copy_from_slice(&b64[..$L]);
                        let mut out_slice = alloc::vec![0u64; $D];
                        let mut out_fixed = [0u64; $D];
                        mul_schoolbook(&a_arr, &b_arr, &mut out_slice);
                        mul_schoolbook_fixed::<$L, $D>(&a_arr, &b_arr, &mut out_fixed);
                        assert_eq!(
                            &out_slice[..],
                            &out_fixed[..],
                            "mul_schoolbook_fixed::<{}, {}> mismatch",
                            $L,
                            $D
                        );
                    }
                }
            }};
        }
        check!(2, 4);
        check!(4, 8);
        check!(8, 16);
        check!(16, 32);
        check!(24, 48);
        check!(32, 64);
        check!(48, 96);
        check!(64, 128);
    }

    /// `mul_schoolbook_into::<L, L+1>` matches `mul_schoolbook_fixed::<L, 2·L>`
    /// when the wider operand is `[n, 0, ..., 0]`, across L covering every
    /// wide tier from D38 (L=2) to D307 (L=16).
    #[test]
    fn mul_schoolbook_into_matches_fixed() {
        // SplitMix64 — Vigna 2014, public-domain reference algorithm.
        let mut state: u64 = 0xDEAD_BEEF_CAFE_F00D;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };

        macro_rules! check_into {
            ($L:expr, $LP1:expr, $D:expr) => {{
                for _ in 0..1000 {
                    let mut a = [0u64; $L];
                    for slot in a.iter_mut() {
                        *slot = next();
                    }
                    let n = next();

                    let mut out_into = [0u64; $LP1];
                    mul_schoolbook_into::<$L, $LP1>(&a, n, &mut out_into);

                    let mut b = [0u64; $L];
                    b[0] = n;
                    let mut out_fixed = [0u64; $D];
                    mul_schoolbook_fixed::<$L, $D>(&a, &b, &mut out_fixed);

                    assert_eq!(
                        &out_into[..],
                        &out_fixed[..$LP1],
                        "mul_schoolbook_into::<{}, {}> low limbs mismatch (a={:?}, n={:#x})",
                        $L,
                        $LP1,
                        a,
                        n
                    );
                    for (k, &limb) in out_fixed[$LP1..].iter().enumerate() {
                        assert_eq!(
                            limb,
                            0,
                            "mul_schoolbook_fixed high limb {} not zero",
                            $LP1 + k
                        );
                    }
                }
            }};
        }
        check_into!(2, 3, 4);
        check_into!(3, 4, 6);
        check_into!(4, 5, 8);
        check_into!(6, 7, 12);
        check_into!(8, 9, 16);
        check_into!(16, 17, 32);
    }

    /// `mul_low_fixed` matches the low `N` limbs of the full product.
    #[test]
    fn mul_low_matches_full_product_low_half() {
        const N: usize = 4;
        const D: usize = 8;
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                let mut a_arr = [0u64; N];
                let mut b_arr = [0u64; N];
                a_arr.copy_from_slice(&a64[..N]);
                b_arr.copy_from_slice(&b64[..N]);
                let mut full = [0u64; D];
                mul_schoolbook_fixed::<N, D>(&a_arr, &b_arr, &mut full);
                let mut low = [0u64; N];
                mul_low_fixed::<N>(&a_arr, &b_arr, &mut low);
                assert_eq!(&full[..N], &low[..], "mul_low_fixed mismatch");
            }
        }
    }

    /// `mul_low_fixed_u128::<N>` (the u128-packed candidate) produces the
    /// bit-identical low `N` limbs of `mul_low_fixed::<N>` across the
    /// carry-stressing corpus (at N = 4) and a wide spread of random
    /// inputs at the even wide-tier work widths (N = 128 / 192 / 256, the
    /// D616 / D924 / D1232 exp Taylor work integers). This is the
    /// correctness gate the u128-multiply pilot rides on.
    #[test]
    fn mul_low_fixed_u128_matches_u64() {
        // Edge corpus at N = 4 (all-ones / single-limb / mixed).
        const N4: usize = 4;
        for a in corpus() {
            for b in corpus() {
                let a64 = pack(&a);
                let b64 = pack(&b);
                let mut a_arr = [0u64; N4];
                let mut b_arr = [0u64; N4];
                a_arr.copy_from_slice(&a64[..N4]);
                b_arr.copy_from_slice(&b64[..N4]);
                let mut lo_ref = [0u64; N4];
                let mut lo_u128 = [0u64; N4];
                mul_low_fixed::<N4>(&a_arr, &b_arr, &mut lo_ref);
                mul_low_fixed_u128::<N4>(&a_arr, &b_arr, &mut lo_u128);
                assert_eq!(lo_ref, lo_u128, "u128 low-mul mismatch (corpus N=4)");
            }
        }

        // Random spread at the wide work widths.
        let mut state: u64 = 0xF00D_FACE_1357_9BDF;
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        macro_rules! check_rand {
            ($n:literal, $rounds:literal) => {{
                const N: usize = $n;
                for _ in 0..$rounds {
                    let mut a = [0u64; N];
                    let mut b = [0u64; N];
                    for x in a.iter_mut() {
                        *x = next();
                    }
                    for x in b.iter_mut() {
                        *x = next();
                    }
                    let mut lo_ref = [0u64; N];
                    let mut lo_u128 = [0u64; N];
                    mul_low_fixed::<N>(&a, &b, &mut lo_ref);
                    mul_low_fixed_u128::<N>(&a, &b, &mut lo_u128);
                    assert_eq!(
                        lo_ref, lo_u128,
                        "u128 low-mul mismatch at N = {}",
                        N
                    );
                }
            }};
        }
        // All-ones worst case at each width (maximal carries).
        macro_rules! check_ones {
            ($n:literal) => {{
                const N: usize = $n;
                let a = [u64::MAX; N];
                let b = [u64::MAX; N];
                let mut lo_ref = [0u64; N];
                let mut lo_u128 = [0u64; N];
                mul_low_fixed::<N>(&a, &b, &mut lo_ref);
                mul_low_fixed_u128::<N>(&a, &b, &mut lo_u128);
                assert_eq!(lo_ref, lo_u128, "u128 low-mul mismatch (all-ones N={})", N);
            }};
        }
        check_rand!(128, 32);
        check_rand!(192, 24);
        check_rand!(256, 16);
        check_ones!(128);
        check_ones!(192);
        check_ones!(256);
    }
}
