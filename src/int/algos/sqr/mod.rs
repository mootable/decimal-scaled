// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integer squaring algorithm family.
//!
//! - [`sqr_half_product`] -- the squaring algorithm fn: half-product
//!   squaring over the const [`sqr_low_fixed`] kernel. The per-`N` choice
//!   lives in [`crate::int::policy::sqr`].
//! - [`sqr_low_fixed`] -- the truncated half-product squaring KERNEL the
//!   algorithm (and the cube / pow square-and-multiply loops) compute on.
//! - [`sqr_schoolbook`] -- naive reference squaring via [`sqr_low_fixed`]:
//!   forms all `N*N` partial products (no symmetry exploitation). Bit-identical
//!   to [`sqr_half_product`]; used as the `Schoolbook` reference arm in the policy.
//!
//! [`sqr_half_product`]: crate::int::algos::sqr::sqr_half_product::sqr_half_product
//! [`sqr_low_fixed`]: crate::int::algos::sqr::sqr_low_fixed::sqr_low_fixed
//! [`sqr_schoolbook`]: crate::int::algos::sqr::sqr_schoolbook::sqr_schoolbook

pub(crate) mod sqr_half_product;
pub(crate) mod sqr_low_fixed;
pub(crate) mod sqr_schoolbook;

#[cfg(test)]
mod tests {
    use super::sqr_half_product::sqr_half_product;
    use super::sqr_schoolbook::sqr_schoolbook;
    use crate::int::types::Uint;

    /// `sqr_schoolbook` matches `sqr_half_product` for `Uint<2>` across
    /// zero, one, max, and a random-like value -- bit-exact by construction
    /// (both reduce to the same `mul_low_fixed` product).
    #[test]
    fn sqr_schoolbook_matches_half_product_uint2() {
        let cases: &[Uint<2>] = &[
            Uint::<2>::from_limbs([0, 0]),
            Uint::<2>::from_limbs([1, 0]),
            Uint::<2>::from_limbs([u64::MAX, u64::MAX]),
            Uint::<2>::from_limbs([0xDEAD_BEEF_CAFE_F00D, 0x1234_5678_9ABC_DEF0]),
            Uint::<2>::from_limbs([0xFFFF_FFFF_0000_0000, 0x0000_FFFF_FFFF_0000]),
        ];
        for &x in cases {
            let got = sqr_schoolbook(x);
            let want = sqr_half_product(x);
            assert_eq!(
                got, want,
                "sqr_schoolbook Uint<2> mismatch on {:?}",
                x.as_limbs()
            );
        }
    }

    /// `sqr_schoolbook` matches `sqr_half_product` for `Uint<4>` across
    /// edge cases and a representative random-like corpus.
    #[test]
    fn sqr_schoolbook_matches_half_product_uint4() {
        let cases: &[Uint<4>] = &[
            Uint::<4>::from_limbs([0, 0, 0, 0]),
            Uint::<4>::from_limbs([1, 0, 0, 0]),
            Uint::<4>::from_limbs([u64::MAX, u64::MAX, u64::MAX, u64::MAX]),
            Uint::<4>::from_limbs([
                0xAAAA_AAAA_AAAA_AAAA,
                0x5555_5555_5555_5555,
                0xF0F0_F0F0_F0F0_F0F0,
                0x0F0F_0F0F_0F0F_0F0F,
            ]),
            Uint::<4>::from_limbs([
                0x1234_5678_9ABC_DEF0,
                0xFEDC_BA98_7654_3210,
                0xA5A5_A5A5_5A5A_5A5A,
                0x3C3C_3C3C_C3C3_C3C3,
            ]),
        ];
        for &x in cases {
            let got = sqr_schoolbook(x);
            let want = sqr_half_product(x);
            assert_eq!(
                got, want,
                "sqr_schoolbook Uint<4> mismatch on {:?}",
                x.as_limbs()
            );
        }
    }

    /// `sqr_schoolbook` on `Uint<1>` -- the scalar case: `x^2` truncated
    /// to one u64 limb, i.e. `x*x` wrapping mod 2^64.
    #[test]
    fn sqr_schoolbook_uint1_scalar() {
        let cases: &[(Uint<1>, u64)] = &[
            (Uint::<1>::from_limbs([0]), 0),
            (Uint::<1>::from_limbs([1]), 1),
            (Uint::<1>::from_limbs([2]), 4),
            (Uint::<1>::from_limbs([3]), 9),
            // 2^32 squared = 2^64 = 0 mod 2^64
            (Uint::<1>::from_limbs([1 << 32]), 0),
            // (2^32 - 1)^2 wrapping mod 2^64
            (
                Uint::<1>::from_limbs([0xFFFF_FFFF]),
                (0xFFFF_FFFFu64).wrapping_mul(0xFFFF_FFFF),
            ),
        ];
        for &(x, expected) in cases {
            let got = sqr_schoolbook(x);
            assert_eq!(
                got.as_limbs()[0], expected,
                "sqr_schoolbook Uint<1> mismatch on {:?}",
                x.as_limbs()
            );
        }
    }
}
