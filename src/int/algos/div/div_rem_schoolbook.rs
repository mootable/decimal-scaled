// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook (binary shift-subtract) long division.
//!
//! [`div_rem_schoolbook`] is the generic naive reference algorithm for
//! unsigned big-integer division, operating over little-endian `u64` limb
//! slices. It uses the classical bit-by-bit shift-subtract method: the
//! invariant is that the running remainder in `rem` is always less than the
//! divisor after each subtraction step.
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
/// Computes `quot = num / den` and `rem = num % den` (unsigned, truncating)
/// over little-endian `u64` limb slices. Both `quot` and `rem` are
/// zeroed before use; their lengths must each be at least as long as `num`.
///
/// The divisor must be non-zero; if `den` is zero the outputs are left as
/// zero (the shift-subtract loop produces no subtractions and no quotient
/// bits, which is consistent with this).
#[allow(dead_code)]
pub(crate) fn div_rem_schoolbook(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    for slot in quot.iter_mut() {
        *slot = 0;
    }
    for slot in rem.iter_mut() {
        *slot = 0;
    }

    let bits = bit_len(num);
    let mut i = bits;
    while i > 0 {
        i -= 1;
        shl1(rem);
        let bit = (num[(i / 64) as usize] >> (i % 64)) & 1;
        rem[0] |= bit;
        shl1(quot);
        if cmp(rem, den) >= 0 {
            sub_assign(rem, den);
            quot[0] |= 1;
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
        let vals: &[u64] = &[
            0, 1, 2, 3, 7, 10, 13, 100, 1_000_000,
            u64::MAX, u64::MAX - 1, 1u64 << 63,
            0xDEAD_BEEF_CAFE_F00D, 0x0102_0304_0506_0708,
        ];
        for &num in vals {
            for &den in vals {
                if den == 0 {
                    continue;
                }
                let mut q = [0u64; 1];
                let mut r = [0u64; 1];
                div_rem_schoolbook(&[num], &[den], &mut q, &mut r);
                assert_eq!(q[0], num / den,
                    "schoolbook quot mismatch: {num} / {den}");
                assert_eq!(r[0], num % den,
                    "schoolbook rem mismatch: {num} % {den}");
            }
        }
    }

    /// Double-limb cases: verify against native u128 arithmetic (external
    /// oracle).
    #[test]
    fn schoolbook_double_limb_oracle() {
        let wide: &[u128] = &[
            0, 1, u128::MAX, u128::MAX - 1,
            1u128 << 64, (1u128 << 64) - 1,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210_u128,
            0xDEAD_BEEF_DEAD_BEEF_CAFE_F00D_CAFE_F00D_u128,
        ];
        let to_limbs = |v: u128| [v as u64, (v >> 64) as u64];
        for &num in wide {
            for &den in wide {
                if den == 0 {
                    continue;
                }
                let n = to_limbs(num);
                let d = to_limbs(den);
                let mut q = [0u64; 2];
                let mut r = [0u64; 2];
                div_rem_schoolbook(&n, &d, &mut q, &mut r);
                let want_q = to_limbs(num / den);
                let want_r = to_limbs(num % den);
                assert_eq!(q, want_q,
                    "schoolbook quot mismatch: {num:#x} / {den:#x}");
                assert_eq!(r, want_r,
                    "schoolbook rem mismatch: {num:#x} % {den:#x}");
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
        for (num, den) in cases {
            if is_zero(den) {
                continue;
            }
            let mut q_ref = [0u64; 8];
            let mut r_ref = [0u64; 8];
            div_rem_dispatch(num, den, &mut q_ref, &mut r_ref);

            let mut q_sb = [0u64; 8];
            let mut r_sb = [0u64; 8];
            div_rem_schoolbook(num, den, &mut q_sb, &mut r_sb);

            assert_eq!(q_sb, q_ref,
                "schoolbook quot differs from dispatch on {:?} / {:?}", num, den);
            assert_eq!(r_sb, r_ref,
                "schoolbook rem differs from dispatch on {:?} / {:?}", num, den);
        }
    }
}
