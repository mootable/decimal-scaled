// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer exponentiation algorithm family.
//!
//! - [`pow_square_and_multiply`] -- binary exponentiation by squaring over
//!   the const [`crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed`] and
//!   [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`] kernels. The per-`N`
//!   choice lives in [`crate::int::policy::pow`].
//! - [`pow_schoolbook`] -- naive repeated-multiply reference: `exp - 1`
//!   sequential multiplications via [`crate::int::algos::mul::mul_schoolbook::mul_low_fixed`].
//!   Unrouted reference arm for the `Schoolbook` policy variant.
//!
//! [`pow_square_and_multiply`]: crate::int::algos::pow::pow_square_and_multiply::pow_square_and_multiply
//! [`pow_schoolbook`]: crate::int::algos::pow::pow_schoolbook::pow_schoolbook

pub(crate) mod pow_schoolbook;
pub(crate) mod pow_square_and_multiply;

#[cfg(test)]
mod tests {
    use super::pow_schoolbook::pow_schoolbook;
    use super::pow_square_and_multiply::pow_square_and_multiply;
    use crate::int::types::Uint;

    /// `pow_schoolbook` matches `pow_square_and_multiply` for `Uint<2>`
    /// across zero-exponent, exponent-one, small exponents, and a larger
    /// exponent, for several bases.
    #[test]
    fn pow_schoolbook_matches_square_and_multiply_uint2() {
        let bases: &[Uint<2>] = &[
            Uint::<2>::from_limbs([0, 0]),
            Uint::<2>::from_limbs([1, 0]),
            Uint::<2>::from_limbs([2, 0]),
            Uint::<2>::from_limbs([3, 0]),
            Uint::<2>::from_limbs([0xDEAD_BEEF_CAFE_F00D, 0x1234_5678_9ABC_DEF0]),
        ];
        let exps: &[u32] = &[0, 1, 2, 3, 5, 7, 10];
        for &base in bases {
            for &exp in exps {
                let got = pow_schoolbook(base, exp);
                let want = pow_square_and_multiply(base, exp);
                assert_eq!(
                    got, want,
                    "pow_schoolbook Uint<2> mismatch: base={:?} exp={}",
                    base.as_limbs(), exp
                );
            }
        }
    }

    /// `pow_schoolbook` matches `pow_square_and_multiply` for `Uint<1>` --
    /// the scalar case -- verifying exact numeric values for base^exp
    /// wrapping mod 2^64.
    #[test]
    fn pow_schoolbook_uint1_known_values() {
        // Known-good values: 2^0..2^10, 3^0..3^5, and wrapping boundary.
        let cases: &[(u64, u32, u64)] = &[
            (2, 0, 1),
            (2, 1, 2),
            (2, 2, 4),
            (2, 3, 8),
            (2, 10, 1024),
            (3, 0, 1),
            (3, 1, 3),
            (3, 2, 9),
            (3, 3, 27),
            (3, 5, 243),
            // 0^k = 0 for k > 0
            (0, 1, 0),
            (0, 5, 0),
            // 1^k = 1 for all k
            (1, 0, 1),
            (1, 100, 1),
            // u64::MAX squared mod 2^64 = 1 (since -1 * -1 = 1 mod 2^64),
            // so MAX^2 = 1, MAX^3 = MAX, etc.
            (u64::MAX, 2, 1),
            (u64::MAX, 3, u64::MAX),
        ];
        for &(base_raw, exp, expected) in cases {
            let base = Uint::<1>::from_limbs([base_raw]);
            let got = pow_schoolbook(base, exp);
            assert_eq!(
                got.as_limbs()[0], expected,
                "pow_schoolbook Uint<1> mismatch: {}^{} expected {}",
                base_raw, exp, expected
            );
        }
    }

    /// `pow_schoolbook` on `Uint<4>` -- a wider type -- matches
    /// `pow_square_and_multiply` for a selection of bases and exponents.
    #[test]
    fn pow_schoolbook_matches_square_and_multiply_uint4() {
        let bases: &[Uint<4>] = &[
            Uint::<4>::from_limbs([0, 0, 0, 0]),
            Uint::<4>::from_limbs([1, 0, 0, 0]),
            Uint::<4>::from_limbs([7, 0, 0, 0]),
            Uint::<4>::from_limbs([0xFFFF_FFFF, 0, 0, 0]),
            Uint::<4>::from_limbs([0xAAAA_AAAA_AAAA_AAAA, 0x5555_5555_5555_5555, 0, 0]),
        ];
        let exps: &[u32] = &[0, 1, 2, 3, 4];
        for &base in bases {
            for &exp in exps {
                let got = pow_schoolbook(base, exp);
                let want = pow_square_and_multiply(base, exp);
                assert_eq!(
                    got, want,
                    "pow_schoolbook Uint<4> mismatch: base={:?} exp={}",
                    base.as_limbs(), exp
                );
            }
        }
    }
}
