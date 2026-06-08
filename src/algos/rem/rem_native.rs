// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_native` -- decimal remainder via the hardware primitive `%`, for
//! narrow storage widths (`N <= 2`, D18 / D38).
//!
//! Same-`SCALE` decimal remainder needs no rescaling: both operands carry the
//! same `10^SCALE` factor, so the storage-level remainder IS the answer
//! (`(a / 10^S) rem (b / 10^S) == (a rem b) / 10^S`). For narrow storage the
//! storage value fits a single hardware integer, so the remainder is a direct
//! primitive `%`:
//!
//! * **`N == 1` (D18):** native `i64 %` on the `i64` storage values.
//! * **`N == 2` (D38):** native `i128 %` on the `i128` storage values.
//!
//! This is exactly 0.4.4 native D18 / D38 remainder (`self.0 % rhs.0`). It
//! bypasses the generic [`rem_int_layer`](crate::algos::rem::rem_int_layer)
//! path, which unpacks both operands to unsigned magnitudes, runs the
//! const-`N` `div_rem` divmod, and rebuilds a signed `Int<N>` with sign
//! reconstruction -- overhead the single hardware `%` instruction avoids.
//!
//! # Overflow / divide-by-zero contract
//!
//! The default operator panics on overflow in BOTH debug and release: a zero
//! divisor panics, and `MIN % -ONE` panics in both profiles (a fixed-width
//! decimal has no ±∞/NaN, so silently wrapping to `0` is a wrong number with
//! no signal). The explicit `wrapping_rem` / `checked_rem` / `overflowing_rem`
//! variants carry the modular / `None` / flag policies.
//!
//! # Layering
//!
//! Pure primitive `i64` / `i128` arithmetic on the storage value dispatched
//! DOWN through `Int<N>`'s lossless `to_i64` / `to_i128`; never calls a
//! decimal method on its own value. Valid only for `N <= 2`.

use crate::int::types::Int;

/// Hardware-`%` decimal remainder for narrow storage (`N <= 2`).
///
/// Computes `a % b` on the storage values. Panics on a zero divisor and on
/// the `MIN % -ONE` overflow boundary in BOTH debug and release, matching the
/// generic `rem_int_layer` default-operator contract.
#[inline]
#[must_use]
pub(crate) fn rem_native<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    if N == 1 {
        let ai = a.to_i128() as i64;
        let bi = b.to_i128() as i64;
        if ai == i64::MIN && bi == -1 {
            panic!("attempt to calculate the remainder with overflow");
        }
        return Int::<N>::from_i128(ai.wrapping_rem(bi) as i128);
    }
    // N == 2 (D38): native i128 %.
    let ai = a.to_i128();
    let bi = b.to_i128();
    if ai == i128::MIN && bi == -1 {
        panic!("attempt to calculate the remainder with overflow");
    }
    Int::<N>::from_i128(ai.wrapping_rem(bi))
}

#[cfg(test)]
mod tests {
    use super::rem_native;
    use crate::int::types::Int;

    #[test]
    fn rem_native_n1_matches_primitive() {
        let cases: &[(i64, i64)] = &[
            (10, 3),
            (-10, 3),
            (10, -3),
            (-10, -3),
            (0, 7),
            (7, 7),
            (100, 13),
            (i64::MAX, 2),
            (i64::MIN + 1, 2),
            (i64::MIN, 7),
        ];
        for &(a, b) in cases {
            let got = rem_native::<1>(Int::<1>::from_i64(a), Int::<1>::from_i64(b));
            assert_eq!(got.to_i128() as i64, a % b, "rem_native n1 ({a}, {b})");
        }
    }

    #[test]
    fn rem_native_n2_matches_primitive() {
        let cases: &[(i128, i128)] = &[
            (100, 7),
            (-100, 7),
            (100, -7),
            (-100, -7),
            (i128::MAX, 3),
            (i128::MIN + 1, 3),
            (1_000_000_000_000_i128, 999_999_937),
        ];
        for &(a, b) in cases {
            let got = rem_native::<2>(Int::<2>::from_i128(a), Int::<2>::from_i128(b));
            assert_eq!(got.to_i128(), a % b, "rem_native n2 ({a}, {b})");
        }
    }
}
