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
//! * **`N == 1` (D18):** the product `lhs * rhs` always fits `i128` (two `i64`
//!   magnitudes), and `10^SCALE` (`SCALE <= 18`) always fits `u64`. The
//!   rescale divide is therefore an `i128 / u64` schoolbook divide -- two
//!   hardware `divq` instructions via
//!   [`crate::macros::arithmetic::i128_divrem_by_u64_with_mode`] -- not the
//!   LLVM `__divti3` soft-call an `i128 / i128` would lower to.
//! * **`N == 2` (D38):** the product can exceed `i128` and `10^SCALE` can
//!   exceed `u64` (`SCALE` up to 37), so the rescale is delegated to the
//!   shared `i128` / `256`-bit kernel
//!   [`crate::algos::support::mg_divide::mul_div_pow10_with`].
//!
//! # Overflow contract
//!
//! The default operator panics on an out-of-range result in BOTH debug and
//! release — a fixed-width decimal has no ±∞/NaN, so silently returning a
//! wrapped value is a wrong number with no signal. The explicit
//! `wrapping_mul` / `checked_mul` / `saturating_mul` / `overflowing_mul`
//! variants (in `crate::macros::overflow`) carry the modular / `None` / clamp
//! / flag policies via their own `Int<N>` paths, not this kernel.
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
/// Computes `lhs * rhs / 10^SCALE` rounded under `mode`. Panics on storage
/// overflow in BOTH debug and release per the decimal default-operator
/// contract.
#[inline]
#[must_use]
pub(crate) fn mul_native<const N: usize, const SCALE: u32>(
    lhs: Int<N>,
    rhs: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    if N == 1 {
        // D18: product fits i128, 10^SCALE fits u64 (SCALE <= 18).
        let product = lhs.as_i128() * rhs.as_i128();
        let scaled: i128 = if SCALE == 0 {
            product
        } else {
            let pow10_scale: u64 = 10u64.pow(SCALE);
            crate::macros::arithmetic::i128_divrem_by_u64_with_mode(product, pow10_scale, mode)
        };
        assert!(
            scaled >= i64::MIN as i128 && scaled <= i64::MAX as i128,
            "attempt to multiply with overflow"
        );
        return Int::<N>::from_i128(scaled);
    }

    // N == 2 (D38): the shared i128 / 256-bit kernel.
    let lhs_raw = lhs.as_i128();
    let rhs_raw = rhs.as_i128();
    match mul_div_pow10_with::<SCALE>(lhs_raw, rhs_raw, mode) {
        Some(product) => Int::<N>::from_i128(product),
        None => panic!("attempt to multiply with overflow"),
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
        let pow10_scale = 10i128.pow(S);
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
        for &(lhs, rhs) in cases {
            let want = ((lhs as i128) * (rhs as i128)) / pow10_scale;
            let got = mul_native::<1, S>(Int::<1>::from_i64(lhs), Int::<1>::from_i64(rhs), MODE);
            assert_eq!(got.to_i128(), want, "mul_native n1 ({lhs}, {rhs})");
        }
    }

    #[test]
    fn mul_native_n2_matches_naive() {
        const S: u32 = 12;
        let pow10_scale = 10i128.pow(S);
        // Operands chosen so lhs * rhs is an exact multiple of 10^12 (no tie /
        // rounding ambiguity), letting the truncating reference stand.
        let cases: &[(i128, i128)] = &[
            (0, 0),
            (1_000_000_000_000_i128, 2_000_000_000_000_i128),
            (-1_000_000_000_000_i128, 2_000_000_000_000_i128),
            (5_000_000_000_000_i128, 4_000_000_000_000_i128),
        ];
        for &(lhs, rhs) in cases {
            assert_eq!((lhs * rhs) % pow10_scale, 0,
                "test operands must be exact for ({lhs}, {rhs})");
            let want = (lhs * rhs) / pow10_scale;
            let got = mul_native::<2, S>(Int::<2>::from_i128(lhs), Int::<2>::from_i128(rhs), MODE);
            assert_eq!(got.to_i128(), want, "mul_native n2 ({lhs}, {rhs})");
        }
    }
}
