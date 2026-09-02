// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Hardware-`%` signed remainder for narrow `Int<N>` (`N <= 2`).
//!
//! [`rem_native`] is the fast remainder for storage widths whose magnitude
//! fits a single `u128`: it loads both operand magnitudes into native
//! `u128`s, takes the hardware `u128 % u128`, and re-applies the dividend
//! sign (truncating-toward-zero semantics). This bypasses the runtime
//! division dispatcher (`crate::int::policy::div_rem::dispatch`) entirely:
//! no leading-zero shape classification, no `[u64; 288]` Knuth scratch, no
//! quotient buffer, for the two narrowest tiers (`Int<1>` = D18,
//! `Int<2>` = D38), where the dispatcher setup cost dwarfs the divide
//! itself.
//!
//! Working on the unsigned magnitudes keeps it overflow-free: the
//! `i128::MIN % -1` hazard (which a signed `to_i128() % to_i128()` would
//! hit) never arises because the magnitudes are unsigned and the divisor
//! magnitude is `>= 1`.
//!
//! Valid only for `N <= 2`: a wider magnitude does not fit `u128`. The
//! remainder policy routes only `N == 1` / `N == 2` here.

use crate::int::types::Int;

/// Load the (<=128-bit) magnitude of `value` into a `u128`. Correct only for
/// `N <= 2`; this fn is only routed to for `N <= 2`.
#[inline]
fn mag_u128<const N: usize>(value: &Int<N>) -> u128 {
    let magnitude = value.unsigned_abs();
    let limbs = magnitude.as_limbs();
    let lo = limbs[0] as u128;
    let hi = if N >= 2 { limbs[1] as u128 } else { 0 };
    lo | (hi << 64)
}

/// Hardware-`%` signed remainder for `Int<N>`, `N <= 2`.
///
/// Loads both magnitudes into `u128`, computes `|dividend| % |divisor|` with
/// the native hardware remainder, and re-applies the dividend sign
/// (truncating-toward-zero). Panics on a zero divisor, matching the `Rem`
/// operator contract.
#[inline]
pub(crate) fn rem_native<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N> {
    assert!(
        !divisor.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let remainder_is_negative = dividend.is_negative();
    let remainder_u128 = mag_u128(&dividend) % mag_u128(&divisor);
    let mut remainder = [0u64; N];
    remainder[0] = remainder_u128 as u64;
    if N >= 2 {
        remainder[1] = (remainder_u128 >> 64) as u64;
    }
    Int::<N>::from_mag_limbs(&remainder, remainder_is_negative)
}

#[cfg(test)]
mod tests {
    use super::rem_native;
    use crate::int::types::Int;

    #[test]
    fn rem_native_known_values_n1() {
        let cases: &[(i64, i64, i64)] = &[
            (10, 3, 1),
            (-10, 3, -1),
            (10, -3, 1),
            (-10, -3, -1),
            (0, 7, 0),
            (7, 7, 0),
            (8, 7, 1),
            (100, 13, 9),
            (i64::MAX, 2, 1),
            (i64::MIN + 1, 2, -1),
        ];
        for &(dividend, divisor, expected) in cases {
            let actual = rem_native::<1>(Int::<1>::from_i64(dividend),
                Int::<1>::from_i64(divisor));
            assert_eq!(actual.to_i64(), expected, "rem_native({dividend}, {divisor})");
        }
    }

    #[test]
    fn rem_native_matches_rem_operator_n2() {
        let cases: &[(i128, i128)] = &[
            (0, 1),
            (1, 1),
            (-1, 1),
            (100, 7),
            (-100, 7),
            (100, -7),
            (-100, -7),
            (i128::MAX, 3),
            (i128::MIN + 1, 3),
            (i128::MAX, i128::MAX - 1),
            (1_000_000_000_000_i128, 999_999_937),
            (0x0001_0000_0000_0000_0000_i128, 0xFFFF),
            (
                170_141_183_460_469_231_731_687_303_715_884_105_727_i128,
                12345678901234567,
            ),
        ];
        for &(dividend, divisor) in cases {
            let actual = rem_native::<2>(Int::<2>::from_i128(dividend),
                Int::<2>::from_i128(divisor));
            assert_eq!(actual.to_i128(), dividend % divisor,
                "rem_native({dividend}, {divisor})");
        }
    }
}
