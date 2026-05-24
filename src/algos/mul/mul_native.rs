// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `mul_native` -- decimal multiplication via the hardware `i128` path, for
//! narrow storage widths (`N <= 2`, i.e. D18 / D38).
//!
//! When the storage fits a single `i128` (`N == 1` is `i64`-backed but widens
//! losslessly; `N == 2` *is* `i128`), the whole multiply-then-rescale can be
//! done in hardware integers instead of forming a `2N`-limb product in a
//! scratch buffer and routing the magnitude through the MG / Newton divide
//! cores.
//!
//! Two specialised arms, selected on `N` at compile time (the unused arm is
//! dead-code-eliminated per monomorphisation):
//!
//! * **`N == 1` (D18):** the product `a * b` always fits `i128` (two `i64`
//!   magnitudes), and `10^SCALE` (`SCALE <= 18`) always fits `u64`. The
//!   rescale divide is therefore an `i128 / u64` schoolbook divide -- two
//!   hardware `divq` instructions via
//!   [`crate::macros::arithmetic::i128_divrem_by_u64_with_mode`] -- not the
//!   LLVM `__divti3` soft-call an `i128 / i128` would lower to. This mirrors
//!   0.4.4 native D18 mul.
//! * **`N == 2` (D38):** the product can exceed `i128` and `10^SCALE` can
//!   exceed `u64` (`SCALE` up to 37), so the rescale is delegated to the
//!   shared `i128` / `256`-bit kernel
//!   [`crate::algos::support::mg_divide::mul_div_pow10_with`].
//!
//! # Overflow contract
//!
//! Both arms follow the standard integer-overflow contract: debug panics on an
//! out-of-range result, release wraps. The release wrap is re-derived as
//! `a * b / 10^SCALE` in wrapping `Int<N>` arithmetic.
//!
//! # Layering
//!
//! All arithmetic is `i128` / `Int<N>` integer work dispatched DOWN to the
//! shared `mg_divide` leaf and the `i128_divrem_by_u64` helper; this fn never
//! calls a decimal method on its own value. Valid only for `N <= 2`.

use crate::algos::support::mg_divide::mul_div_pow10_with;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Hardware-`i128` decimal multiply kernel for narrow storage (`N <= 2`).
///
/// Computes `a * b / 10^SCALE` rounded under `mode`. Debug-panics /
/// release-wraps on storage overflow per the standard integer contract.
/// `mult` is the pre-computed `10^SCALE` in `Int<N>`, used only on the
/// release-wrap path.
#[inline]
#[must_use]
pub(crate) fn mul_native<const N: usize, const SCALE: u32>(
    a: Int<N>,
    b: Int<N>,
    mult: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    if N == 1 {
        // D18: product fits i128, 10^SCALE fits u64 (SCALE <= 18).
        let n = a.as_i128() * b.as_i128();
        let scaled: i128 = if SCALE == 0 {
            n
        } else {
            let m_mag: u64 = 10u64.pow(SCALE);
            crate::macros::arithmetic::i128_divrem_by_u64_with_mode(n, m_mag, mode)
        };
        if cfg!(debug_assertions) {
            assert!(
                scaled >= i64::MIN as i128 && scaled <= i64::MAX as i128,
                "attempt to multiply with overflow"
            );
        }
        return Int::<N>::from_i128(scaled);
    }

    // N == 2 (D38): the shared i128 / 256-bit kernel.
    let ai = a.as_i128();
    let bi = b.as_i128();
    match mul_div_pow10_with::<SCALE>(ai, bi, mode) {
        Some(q) => Int::<N>::from_i128(q),
        None => {
            if cfg!(debug_assertions) {
                panic!("attempt to multiply with overflow");
            }
            a.wrapping_mul(b).wrapping_div(mult)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mul_native;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const MODE: RoundingMode = RoundingMode::HalfToEven;

    #[test]
    fn mul_native_n1_matches_naive() {
        const S: u32 = 6;
        let m = 10i128.pow(S);
        let cases: &[(i64, i64)] = &[
            (0, 0),
            (1_000_000, 2_000_000),
            (-1_000_000, 2_000_000),
            (1_000_000, -2_000_000),
            (-1_000_000, -2_000_000),
            (1_234_567, 7_654_321),
            (999_999, 999_999),
            (i32::MAX as i64, 1_000_000),
        ];
        for &(a, b) in cases {
            let want = ((a as i128) * (b as i128)) / m;
            let mult = Int::<1>::TEN.pow(S);
            let got = mul_native::<1, S>(Int::<1>::from_i64(a), Int::<1>::from_i64(b), mult, MODE);
            assert_eq!(got.to_i128(), want, "mul_native n1 ({a}, {b})");
        }
    }

    #[test]
    fn mul_native_n2_matches_naive() {
        const S: u32 = 12;
        let m = 10i128.pow(S);
        // Operands chosen so a * b is an exact multiple of 10^12 (no tie /
        // rounding ambiguity), letting the truncating reference stand.
        let cases: &[(i128, i128)] = &[
            (0, 0),
            (1_000_000_000_000_i128, 2_000_000_000_000_i128),
            (-1_000_000_000_000_i128, 2_000_000_000_000_i128),
            (5_000_000_000_000_i128, 4_000_000_000_000_i128),
        ];
        for &(a, b) in cases {
            assert_eq!((a * b) % m, 0, "test operands must be exact for ({a}, {b})");
            let want = (a * b) / m;
            let mult = Int::<2>::TEN.pow(S);
            let got = mul_native::<2, S>(Int::<2>::from_i128(a), Int::<2>::from_i128(b), mult, MODE);
            assert_eq!(got.to_i128(), want, "mul_native n2 ({a}, {b})");
        }
    }
}
