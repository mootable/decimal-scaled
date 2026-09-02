// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: N<=2 signed remainder via direct two's-complement i128 `%`
// (skips the unsigned_abs / from_mag_limbs sign-magnitude round trip the
// shipped rem_native still pays), not wired.
//
//! Direct-`i128` signed remainder candidate for narrow `Int<N>` (`N <= 2`).
//!
//! The shipped [`crate::int::algos::rem::rem_native::rem_native`] computes
//! the remainder on the UNSIGNED magnitudes — it calls `unsigned_abs()`
//! (a `neg_dispatch` round trip on a negative operand) on both inputs,
//! takes `u128 % u128`, then rebuilds the signed result via
//! `from_mag_limbs`. That is correct but adds sign-magnitude
//! conversion overhead: a single hardware `i64 %` / `i128 %`
//! directly on the two's-complement storage primitive needs no magnitude trip.
//!
//! For `N <= 2` the `Int<N>` limbs ARE the i64/i128 two's-complement value
//! (see [`crate::int::types::Int::as_i128`]'s `N <= 2` fast path),
//! so the remainder can be the hardware signed `%` on that value
//! directly. The only
//! hazard a signed `%` carries is the `i128::MIN % -1` overflow trap; we
//! guard it explicitly and return zero (the mathematically-correct
//! remainder, matching `wrapping_rem`), so the result is bit-identical to
//! the magnitude path for every operand relationship.
//!
//! Both `as_i128` (the read) and `from_i128` (the write-back) const-fold to
//! straight limb loads/stores for `N <= 2`, so this stays a single generic
//! kernel — no per-tier type, no macro duplication. Valid only for
//! `N <= 2`; the rem policy routes only `N == 1` / `N == 2` here.

use crate::int::types::Int;

/// Direct two's-complement signed remainder for `Int<N>`, `N <= 2`.
///
/// Reads both operands as native `i128` (the `N <= 2` fast path is a plain
/// limb load), takes the hardware signed `%`, and stores back. Panics on a
/// zero divisor, matching the `Rem` operator contract. Guards the
/// `i128::MIN % -1` overflow boundary (returns zero, as `wrapping_rem`
/// does), so it is bit-identical to the shipped magnitude path.
#[inline]
#[allow(dead_code)]
pub(crate) fn rem_native_direct<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N> {
    assert!(
        !divisor.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let dividend_i128 = dividend.to_i128();
    let divisor_i128 = divisor.to_i128();
    // i128::MIN % -1 is the sole overflow trap of signed `%`; the magnitude
    // path can't hit it because its operands are unsigned. The true
    // remainder there is 0 (MIN is exactly divisible by -1), which is what
    // wrapping_rem yields — match it without invoking the trapping `%`.
    let remainder = if divisor_i128 == -1 { 0 } else { dividend_i128 % divisor_i128 };
    Int::<N>::from_i128(remainder)
}

#[cfg(test)]
mod tests {
    // NOTE: candidate test — bit-identity vs the shipped magnitude path.
    // Not run here; kept for validation before wiring.
    use super::rem_native_direct;
    use crate::int::algos::rem::rem_native::rem_native;
    use crate::int::types::Int;

    #[test]
    fn n1_matches_shipped_magnitude_path() {
        let cases: &[(i64, i64)] = &[
            (10, 3),
            (-10, 3),
            (10, -3),
            (-10, -3),
            (0, 7),
            (7, 7),
            (i64::MAX, 2),
            (i64::MIN + 1, 2),
            (i64::MIN, 7),
            (i64::MIN, -1),
        ];
        for &(dividend, divisor) in cases {
            let dividend_int = Int::<1>::from_i64(dividend);
            let divisor_int = Int::<1>::from_i64(divisor);
            assert_eq!(
                rem_native_direct::<1>(dividend_int, divisor_int),
                rem_native::<1>(dividend_int, divisor_int),
                "n1 ({dividend} % {divisor})"
            );
        }
    }

    #[test]
    fn n2_matches_shipped_magnitude_path() {
        let cases: &[(i128, i128)] = &[
            (100, 7),
            (-100, 7),
            (100, -7),
            (-100, -7),
            (i128::MAX, 3),
            (i128::MIN + 1, 3),
            (i128::MIN, 7),
            (i128::MIN, -1),
        ];
        for &(dividend, divisor) in cases {
            let dividend_int = Int::<2>::from_i128(dividend);
            let divisor_int = Int::<2>::from_i128(divisor);
            assert_eq!(
                rem_native_direct::<2>(dividend_int, divisor_int),
                rem_native::<2>(dividend_int, divisor_int),
                "n2 ({dividend} % {divisor})"
            );
        }
    }
}
