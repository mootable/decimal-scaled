// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook (binary shift-subtract) long division.
//!
//! [`div_rem_schoolbook`] is the generic naive reference algorithm for
//! unsigned big-integer division, operating over little-endian `u64` limb
//! slices. It uses the classical bit-by-bit shift-subtract method: the
//! invariant is that the running remainder in `remainder` is always less than
//! the divisor after each subtraction step.
//!
//! The algorithm: for each bit of the dividend from most-significant to
//! least-significant, shift the running remainder left by one, bring in the
//! current dividend bit, and subtract the divisor from the remainder
//! whenever the remainder is at least as large as the divisor (recording a
//! `1` quotient bit).
//!
//! This is a named reference implementation registered as
//! [`crate::int::policy::div_rem::Algorithm::Schoolbook`]. The production
//! dispatcher (`select`) never returns it; it exists as an unrouted
//! reference arm whose correctness the unit tests exercise directly.

use crate::int::algos::support::limbs::{bit_len, cmp, shl1, sub_assign};

/// Binary shift-subtract long division — schoolbook reference.
///
/// Computes `quotient = dividend / divisor` and
/// `remainder = dividend % divisor` (unsigned, truncating) over little-endian
/// `u64` limb slices. Both `quotient` and `remainder` are zeroed before use;
/// their lengths must each be at least as long as `dividend`.
///
/// The divisor must be non-zero; if `divisor` is zero the outputs are left as
/// zero (the shift-subtract loop produces no subtractions and no quotient
/// bits, which is consistent with this).
#[allow(dead_code)]
pub(crate) fn div_rem_schoolbook(dividend: &[u64], divisor: &[u64], quotient: &mut [u64],
    remainder: &mut [u64]) {
    for slot in quotient.iter_mut() {
        *slot = 0;
    }
    for slot in remainder.iter_mut() {
        *slot = 0;
    }

    let dividend_bits = bit_len(dividend);
    let mut i = dividend_bits;
    while i > 0 {
        i -= 1;
        shl1(remainder);
        let bit = (dividend[(i / 64) as usize] >> (i % 64)) & 1;
        remainder[0] |= bit;
        shl1(quotient);
        if cmp(remainder, divisor) >= 0 {
            sub_assign(remainder, divisor);
            quotient[0] |= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::div_rem_schoolbook;

    /// Single-limb cases: verify quotient and remainder match native u64
    /// arithmetic (external oracle).
    #[test]
    fn schoolbook_single_limb_oracle() {
        let values: &[u64] = &[
            0, 1, 2, 3, 7, 10, 13, 100, 1_000_000,
            u64::MAX, u64::MAX - 1, 1u64 << 63,
            0xDEAD_BEEF_CAFE_F00D, 0x0102_0304_0506_0708,
        ];
        for &dividend in values {
            for &divisor in values {
                if divisor == 0 {
                    continue;
                }
                let mut quotient = [0u64; 1];
                let mut remainder = [0u64; 1];
                div_rem_schoolbook(&[dividend], &[divisor], &mut quotient, &mut remainder);
                assert_eq!(quotient[0], dividend / divisor,
                    "schoolbook quot mismatch: {dividend} / {divisor}");
                assert_eq!(remainder[0], dividend % divisor,
                    "schoolbook rem mismatch: {dividend} % {divisor}");
            }
        }
    }

    /// Double-limb cases: verify against native u128 arithmetic (external
    /// oracle).
    #[test]
    fn schoolbook_double_limb_oracle() {
        let values: &[u128] = &[
            0, 1, u128::MAX, u128::MAX - 1,
            1u128 << 64, (1u128 << 64) - 1,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210_u128,
            0xDEAD_BEEF_DEAD_BEEF_CAFE_F00D_CAFE_F00D_u128,
        ];
        let to_limbs = |value: u128| [value as u64, (value >> 64) as u64];
        for &dividend in values {
            for &divisor in values {
                if divisor == 0 {
                    continue;
                }
                let dividend_limbs = to_limbs(dividend);
                let divisor_limbs = to_limbs(divisor);
                let mut quotient = [0u64; 2];
                let mut remainder = [0u64; 2];
                div_rem_schoolbook(&dividend_limbs, &divisor_limbs, &mut quotient,
                    &mut remainder);
                let expected_quotient = to_limbs(dividend / divisor);
                let expected_remainder = to_limbs(dividend % divisor);
                assert_eq!(quotient, expected_quotient,
                    "schoolbook quot mismatch: {dividend:#x} / {divisor:#x}");
                assert_eq!(remainder, expected_remainder,
                    "schoolbook rem mismatch: {dividend:#x} % {divisor:#x}");
            }
        }
    }

    /// Cross-check: schoolbook agrees with `crate::int::policy::div_rem::dispatch`
    /// on a battery of mixed-width inputs.
    #[test]
    fn schoolbook_matches_dispatch() {
        use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
        use crate::int::algos::support::limbs::is_zero;

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
            if is_zero(divisor) {
                continue;
            }
            let mut quotient_dispatch = [0u64; 8];
            let mut remainder_dispatch = [0u64; 8];
            div_rem_dispatch(dividend, divisor, &mut quotient_dispatch,
                &mut remainder_dispatch);

            let mut quotient_schoolbook = [0u64; 8];
            let mut remainder_schoolbook = [0u64; 8];
            div_rem_schoolbook(dividend, divisor, &mut quotient_schoolbook,
                &mut remainder_schoolbook);

            assert_eq!(quotient_schoolbook, quotient_dispatch,
                "schoolbook quot differs from dispatch on {:?} / {:?}", dividend, divisor);
            assert_eq!(remainder_schoolbook, remainder_dispatch,
                "schoolbook rem differs from dispatch on {:?} / {:?}", dividend, divisor);
        }
    }
}
