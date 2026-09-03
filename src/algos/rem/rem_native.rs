// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_native` -- decimal remainder via the hardware primitive `%`, for
//! narrow storage widths (`N <= 2`, D18 / D38).
//!
//! Same-`SCALE` decimal remainder needs no rescaling: both operands carry the
//! same `10^SCALE` factor, so the storage-level remainder IS the answer
//! (`(dividend / 10^S) rem (divisor / 10^S) == (dividend rem divisor) /
//! 10^S`). For narrow storage the
//! storage value fits a single hardware integer, so the remainder is a direct
//! primitive `%`:
//!
//! * **`N == 1` (D18):** native `i64 %` -- a genuine hardware `idiv`. ROUTED.
//! * **`N == 2` (D38):** `i128 %` -- **not** an instruction. KEPT, UNROUTED.
//!
//! It bypasses the generic [`rem_int_layer`](crate::algos::rem::rem_int_layer)
//! path, which unpacks both operands to unsigned magnitudes, runs the
//! const-`N` `div_rem` divmod, and rebuilds a signed `Int<N>` with sign
//! reconstruction. At `N == 1` that overhead is worth avoiding; at `N == 2`
//! it is not, and the reason is codegen, not arithmetic.
//!
//! # Why `N == 2` is kept but not routed
//!
//! x86-64 has NO 128-bit divide instruction: `div r/m64` is a 128÷64→64
//! divide that traps when the quotient overflows 64 bits, so a general
//! `i128 % i128` cannot be one instruction. LLVM lowers it to the
//! compiler-builtins soft-call `__modti3` / `__udivmodti4`, whose cost also
//! depends on the operand high words. The crate already routes around the
//! division sibling of exactly this call: [`div_native`] keeps its `N == 1`
//! arm on an `i128 / u64` schoolbook divide expressly to avoid `__divti3`.
//!
//! So at `N == 2` the choice is not "instruction vs generic path" -- both
//! arms reach the same soft-call, and `rem_int_layer` additionally skips the
//! divide whenever `|a| < |b|`. Measured at the dispatch seam over 9 scales
//! x 3 operand classes (`benches/micro/rem_kernel_ab.rs`, group
//! `dec_rem_narrow`), `rem_int_layer` won all 27 cells:
//!
//! | class | `rem_native` | `rem_int_layer` |
//! |---|---:|---:|
//! | `\|a\| < \|b\|` (`2·10^s % 35·10^(s-1)`) | 18.1 ns (s0-s15) / 20.1 ns (s19-s36) | 10.0 ns, flat |
//! | `\|a\| > \|b\|`, small quotient | 18.1 ns / 20.1 ns | 17.0 ns / 18.5 ns |
//! | `\|a\| >> \|b\|`, large quotient | 18.1-18.5 ns | 16.9-17.4 ns |
//!
//! (Timings include a fixed ~9 ns of `#[inline(never)]` + `black_box` harness
//! overhead common to both arms; compare the differences, not the absolutes.
//! The step at `s19` is the soft-call's, and lands exactly where `2·10^s`
//! crosses `2^64` -- the operand-dependence made visible.)
//!
//! At `N == 1` the same sweep splits by operand class rather than favouring
//! one arm outright: `rem_native` wins the `|a| >= |b|` classes by ~0.85 ns
//! at every scale and loses the `|a| < |b|` class by ~2.5 ns (10.8 vs 13.3
//! ns) to `rem_int_layer`'s dividend-smaller short-circuit. `N == 1` stays
//! on `rem_native`; whether a value split earns its place there is open.
//!
//! [`div_native`]: crate::algos::div::div_native::div_native
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
/// Computes `dividend % divisor` on the storage values. Panics on a zero
/// divisor and on the `MIN % -ONE` overflow boundary in BOTH debug and
/// release, matching the generic `rem_int_layer` default-operator contract.
#[inline]
#[must_use]
pub(crate) fn rem_native<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N> {
    assert!(
        !divisor.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    if N == 1 {
        let dividend_raw = dividend.to_i128() as i64;
        let divisor_raw = divisor.to_i128() as i64;
        if dividend_raw == i64::MIN && divisor_raw == -1 {
            panic!("attempt to calculate the remainder with overflow");
        }
        return Int::<N>::from_i128(dividend_raw.wrapping_rem(divisor_raw) as i128);
    }
    // N == 2 (D38): native i128 %.
    let dividend_raw = dividend.to_i128();
    let divisor_raw = divisor.to_i128();
    if dividend_raw == i128::MIN && divisor_raw == -1 {
        panic!("attempt to calculate the remainder with overflow");
    }
    Int::<N>::from_i128(dividend_raw.wrapping_rem(divisor_raw))
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
        for &(dividend, divisor) in cases {
            let got = rem_native::<1>(Int::<1>::from_i64(dividend), Int::<1>::from_i64(divisor));
            assert_eq!(
                got.to_i128() as i64,
                dividend % divisor,
                "rem_native n1 ({dividend}, {divisor})"
            );
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
        for &(dividend, divisor) in cases {
            let got = rem_native::<2>(Int::<2>::from_i128(dividend), Int::<2>::from_i128(divisor));
            assert_eq!(
                got.to_i128(),
                dividend % divisor,
                "rem_native n2 ({dividend}, {divisor})"
            );
        }
    }
}
