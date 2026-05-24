// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook integer remainder for `Int<N>`.
//!
//! [`rem_schoolbook`] is the naive reference algorithm: it performs
//! binary shift-subtract long division on the unsigned magnitudes and
//! takes the remainder output, re-applying the dividend's sign
//! (truncating-toward-zero semantics). It calls the underlying
//! [`div_rem_schoolbook`](crate::int::algos::div::div_rem_schoolbook::div_rem_schoolbook)
//! kernel directly — never an operator or dispatched method that
//! re-enters a policy.

use crate::int::algos::div::div_rem_schoolbook::div_rem_schoolbook;
use crate::int::types::Int;

/// Schoolbook signed remainder for `Int<N>`.
///
/// Strips the operand signs, calls
/// [`div_rem_schoolbook`] (the naive binary shift-subtract long division)
/// on the unsigned magnitudes, and re-applies the dividend's sign to the
/// remainder (truncating-toward-zero semantics).
///
/// This is a named reference algorithm wired as a registered (but
/// unrouted) [`crate::int::policy::rem::Algorithm::Schoolbook`] arm.
/// `select` never returns it; it exists so the reference baseline is
/// reachable by unit tests and future routing experiments without being
/// called in production.
///
/// Panics on a zero divisor, matching the `Rem` operator contract.
#[allow(dead_code)]
pub(crate) fn rem_schoolbook<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let neg_r = a.is_negative();
    let mut quot = [0u64; N];
    let mut rem = [0u64; N];
    div_rem_schoolbook(
        a.unsigned_abs().as_limbs(),
        b.unsigned_abs().as_limbs(),
        &mut quot,
        &mut rem,
    );
    Int::<N>::from_mag_limbs(&rem, neg_r)
}

#[cfg(test)]
mod tests {
    use super::rem_schoolbook;
    use crate::int::types::Int;

    /// Known-value table: (a, b, expected_remainder) — all fit in i64
    /// so we can verify by construction.
    #[test]
    fn rem_schoolbook_known_values_n1() {
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
        for &(a, b, want) in cases {
            let got = rem_schoolbook::<1>(Int::<1>::from_i64(a), Int::<1>::from_i64(b));
            assert_eq!(
                got.to_i64(),
                want,
                "rem_schoolbook({a}, {b}) = {:?}, want {want}",
                got.to_i64()
            );
        }
    }

    /// For a range of values across N=2 (128-bit), check that
    /// rem_schoolbook agrees with the standard `%` operator result
    /// (both truncate toward zero).
    #[test]
    fn rem_schoolbook_matches_rem_operator_n2() {
        // Representative i128 pairs — chosen to exercise multi-limb paths.
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
        for &(a, b) in cases {
            let ia = Int::<2>::from_i128(a);
            let ib = Int::<2>::from_i128(b);
            let got = rem_schoolbook::<2>(ia, ib);
            let want = a % b;
            assert_eq!(
                got.to_i128(),
                want,
                "rem_schoolbook({a}, {b}) = {:?}, want {want}",
                got.to_i128()
            );
        }
    }
}
