// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: N<=2 signed div_rem via direct two's-complement i128 `/` and
// `%` (skips the unsigned_abs / from_mag_limbs sign-magnitude round trip
// the shipped Int::div_rem pays on BOTH operands and BOTH outputs), not
// wired.
//
//! Direct-`i128` signed `div_rem` candidate for narrow `Int<N>` (`N <= 2`).
//!
//! The shipped [`crate::int::types::Int::div_rem`] (which the `Int<N>`
//! `Div` and `Rem` operators route through) computes on UNSIGNED
//! magnitudes: it calls `unsigned_abs()` (a `neg_dispatch` round trip on a
//! negative operand) on BOTH `self` and `rhs`, runs `div_rem_mag_fixed`
//! (whose `N <= 2` arm is already a native `u64`/`u128` idiv), then rebuilds
//! BOTH the quotient and the remainder via `from_mag_limbs` with the
//! reconstructed signs. That is two `unsigned_abs` + two `from_mag_limbs`
//! sign-magnitude conversions wrapping a single hardware divide.
//!
//! This is the SAME overhead the rem-only sibling
//! [`crate::int::algos::rem::rem_native_direct`] removes, and the SAME
//! overhead `Int::as_i128` avoids. For `N <= 2` the limbs
//! ARE the i64/i128 two's-complement value (see `Int::as_i128`'s `N <= 2`
//! fast path), and hardware signed `/` and `%` already implement
//! truncating-toward-zero with the exact sign rules `div_rem`
//! reconstructs, so the magnitudes never need to be materialised.
//!
//! The only hazard signed `/`/`%` carry is the `i128::MIN / -1` (and
//! `i128::MIN % -1`) overflow trap; we guard it explicitly (quotient
//! `MIN`, remainder `0` — the `wrapping_div`/`wrapping_rem` results, which
//! match the magnitude path: |MIN|/1 = |MIN| re-signed wraps to MIN, and
//! |MIN| % 1 = 0). Bit-identical to the magnitude path for every operand
//! relationship.
//!
//! Both `as_i128` (the read) and `from_i128` (the write-back) const-fold to
//! plain limb loads/stores for `N <= 2`, so this stays a single generic
//! kernel — no per-tier type, no macro duplication. Valid only for
//! `N <= 2`; intended for the `Int<N>` `Div`/`Rem` operator fast arm at
//! `N == 1` / `N == 2`.

use crate::int::types::Int;

/// Direct two's-complement signed `(quotient, remainder)` for `Int<N>`,
/// `N <= 2`.
///
/// Reads both operands as native `i128` (the `N <= 2` fast path is a plain
/// limb load), takes hardware signed `/` and `%`, and stores back. Panics
/// on a zero divisor, matching `Int::div_rem`. Guards the `i128::MIN / -1`
/// overflow boundary (quotient `MIN`, remainder `0`, as the wrapping
/// primitives do), so it is bit-identical to the shipped magnitude path.
#[inline]
#[allow(dead_code)]
pub(crate) fn div_rem_native_direct<const N: usize>(dividend: Int<N>, divisor: Int<N>)
    -> (Int<N>, Int<N>) {
    assert!(!divisor.is_zero(), "attempt to divide by zero");
    let dividend_i128 = dividend.to_i128();
    let divisor_i128 = divisor.to_i128();
    // i128::MIN / -1 (and the matching `%`) is the sole overflow trap of
    // signed `/`/`%`; the magnitude path can't hit it (unsigned operands).
    // wrapping semantics: quotient wraps to MIN, remainder is 0.
    let (quotient, remainder) = if divisor_i128 == -1 {
        (dividend_i128.wrapping_neg(), 0)
    } else {
        (dividend_i128 / divisor_i128, dividend_i128 % divisor_i128)
    };
    (Int::<N>::from_i128(quotient), Int::<N>::from_i128(remainder))
}

#[cfg(test)]
mod tests {
    // NOTE: candidate test — bit-identity vs the shipped magnitude path
    // (`Int::div_rem`). Not run here; kept for validation before wiring.
    use super::div_rem_native_direct;
    use crate::int::types::Int;

    #[test]
    fn n1_matches_shipped_div_rem() {
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
            (i64::MIN, 1),
        ];
        for &(dividend, divisor) in cases {
            let dividend_int = Int::<1>::from_i64(dividend);
            let divisor_int = Int::<1>::from_i64(divisor);
            assert_eq!(
                div_rem_native_direct::<1>(dividend_int, divisor_int),
                dividend_int.div_rem(divisor_int),
                "n1 ({dividend} / {divisor})"
            );
        }
    }

    #[test]
    fn n2_matches_shipped_div_rem() {
        let cases: &[(i128, i128)] = &[
            (100, 7),
            (-100, 7),
            (100, -7),
            (-100, -7),
            (i128::MAX, 3),
            (i128::MIN + 1, 3),
            (i128::MIN, 7),
            (i128::MIN, -1),
            (i128::MIN, 1),
        ];
        for &(dividend, divisor) in cases {
            let dividend_int = Int::<2>::from_i128(dividend);
            let divisor_int = Int::<2>::from_i128(divisor);
            assert_eq!(
                div_rem_native_direct::<2>(dividend_int, divisor_int),
                dividend_int.div_rem(divisor_int),
                "n2 ({dividend} / {divisor})"
            );
        }
    }
}
