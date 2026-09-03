// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_int_layer` -- decimal remainder via the `Int<N>` layer.

use crate::int::algos::div::div_knuth::div_knuth_into;
use crate::int::algos::div::div_knuth_u128_limb::div_knuth_u128_limb_into;
use crate::int::policy::div_rem::{select_for_limbs, Algorithm};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. The default operator panics on
/// overflow in BOTH debug and release: division by zero always panics, and
/// `MIN % -ONE` panics in both profiles (a fixed-width decimal has no
/// ±∞/NaN, so silently wrapping to `0` is a wrong number with no signal).
/// The explicit `wrapping_rem` / `checked_rem` / `overflowing_rem` variants
/// carry the modular / `None` / flag policies. No rescaling needed --
/// same-SCALE operands share the scale factor.
///
/// A value-gated small-operand fast path runs FIRST: when both operand
/// Two value-gated fast paths run FIRST. (1) When `|dividend| < |divisor|`
/// the truncating remainder is the dividend itself
/// (`dividend % divisor == dividend`), returned after one
/// top-down magnitude compare — no divide, no scratch. This is the dominant
/// decimal-`rem` benchmarked shape (`x % y` with `|x| < |y|`, e.g. `2.0 % 3.5`) and it
/// catches the cases the u128 probe misses (a scaled divisor crossing the
/// 128-bit line while the dividend stays smaller — D76 s38 onward).
/// (2) When both operand magnitudes fit a single 128-bit word it takes a
/// hardware `u128 % u128` (no scratch, no shape classifier, no Knuth setup),
/// a single-word fast path applied generically — the scale-0
/// bare-integer / small-`k` shape where the divmod setup dwarfs the divide.
/// Both are bit-identical to the divmod below, so they only change which path
/// runs, never the result.
///
/// Otherwise it routes on the divide matcher's verdict
/// ([`select_for_limbs`](crate::int::policy::div_rem::select_for_limbs)) and
/// resolves the remainder via the chosen engine with **exact `ComputeLimbs`
/// scratch** (`single_buffered_u64`, `N + 2` per width) instead of the `Rem`
/// operator's build-max `[u64; MAX_SINGLE_LIMBS]` Knuth buffers. The operands'
/// SIGNIFICANT lengths are independent inside one `Int<N>`, so at wide `N` a
/// long dividend over an even mid-width divisor genuinely reaches the u128
/// engine's `num ≥ 2·den` shape (e.g. `N = 64`: a 64-sig-limb dividend `%` a
/// 24-sig-limb even divisor) — every verdict is honored with its own engine
/// ([`div_knuth_into`] routes a single-limb divisor to the hardware path
/// internally). Sizing the
/// normalised `u`/`v` to the operand width drops the build-max memset that
/// dominated the wide-tier remainder (98% of the cost at D57 … 12% at
/// D1232). The bare `Rem` operator must stay build-max (blanket over all `N`,
/// the `exact-scratch` wall); this concrete-`N` decimal kernel carries
/// `Limbs<N>: ComputeLimbs`.
///
/// Reached for `N >= 2` (the decimal `rem` policy routes only `N == 1` to
/// `rem_native`, the one width whose `%` is a hardware `idiv`), so the
/// genuinely-hardware narrow path is untouched; every such `N` is in the
/// `exact-scratch` width list, so the `ComputeLimbs` bound discharges at the
/// concrete `N` and never cascades. At `N == 2` the `while i < N` high-limb
/// probe below is empty and both fast paths are always live: `|a| < |b|`
/// returns the dividend, and otherwise the single-word arm is the whole
/// divide.
///
/// [`div_knuth_into`]: crate::int::algos::div::div_knuth::div_knuth_into
#[inline]
pub(crate) fn rem_int_layer<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    // Divide-by-zero panics, and the `MIN % -ONE` overflow must panic in
    // BOTH debug and release (the default operator never silently wraps to
    // `0`): detect it with cheap comparisons (no divide) and panic before
    // the divide wraps it.
    if dividend == Int::<N>::MIN && divisor == -Int::<N>::ONE {
        panic!("attempt to calculate the remainder with overflow");
    }
    assert!(
        !divisor.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );

    // Truncating-toward-zero: the remainder carries the dividend's sign.
    let remainder_is_negative = dividend.is_negative();
    let dividend_abs = dividend.unsigned_abs();
    let divisor_abs = divisor.unsigned_abs();

    // Dividend-smaller short-circuit: when `|dividend| < |divisor|` the
    // truncating remainder is the dividend itself
    // (`dividend % divisor == dividend`), so return `dividend`
    // unchanged — no divide, no scratch, no shape classifier. This is one
    // top-down `N`-limb magnitude compare (`Uint::cmp`), correct for EVERY
    // `N` and operand value, and it catches the dominant decimal-`rem` shape
    // the u128 fast path below MISSES: a balanced-magnitude `x % y` where the
    // SCALED divisor crosses the 128-bit line (e.g. the benchmarked `2.0 % 3.5` cell
    // at D76 s38: `2·10^38` fits a u128 but `3.5·10^38` is 129 bits, so the
    // u128 probe fails and the operands fall into a full multi-limb Knuth
    // divmod whose `top < n` early-out the compare reaches first, for free).
    // Bit-identical to the divmod (which also yields the dividend here), so it
    // only changes which path runs, never the result.
    if dividend_abs < divisor_abs {
        return dividend;
    }

    // Small-operand fast path (single-word, applied generically across
    // widths): when both magnitudes fit a single 128-bit word, take the
    // hardware `u128 % u128` and re-apply the dividend's sign — bypassing the
    // `select_for_limbs` shape classifier, the `single_buffered_u64` scratch
    // and the Knuth normalise/shift setup that `div_knuth_into` runs even on
    // tiny operands. This is the dominant scale-0 decimal-`rem` shape (a bare
    // integer / a small `k`, e.g. the `2 % 1` benchmarked cell at scale 0), where the
    // full divmod setup dwarfs the divide. Bit-identical to the divmod below
    // (the magnitude check guarantees the `u128` load is lossless), so valid
    // at every `N >= 3`. The MIN%-1 hazard cannot reach here (magnitudes are
    // unsigned, the divisor magnitude is `>= 1`).
    let dividend_limbs = dividend_abs.as_limbs();
    let divisor_limbs = divisor_abs.as_limbs();
    // Probe whether both magnitudes fit one 128-bit word (every limb above
    // index 1 zero). Break on the FIRST set high limb so a full-width operand
    // pays only a couple of comparisons before falling through to the divmod
    // (the wide balanced shape must not regress on the scan cost).
    let mut fits = true;
    let mut i = 2;
    while i < N {
        if dividend_limbs[i] != 0 || divisor_limbs[i] != 0 {
            fits = false;
            break;
        }
        i += 1;
    }
    if fits {
        let dividend_hi = if N >= 2 { dividend_limbs[1] as u128 } else { 0 };
        let divisor_hi = if N >= 2 { divisor_limbs[1] as u128 } else { 0 };
        let dividend_u128 = (dividend_limbs[0] as u128) | (dividend_hi << 64);
        let divisor_u128 = (divisor_limbs[0] as u128) | (divisor_hi << 64);
        let remainder_u128 = dividend_u128 % divisor_u128;
        let mut remainder_limbs = [0u64; N];
        remainder_limbs[0] = remainder_u128 as u64;
        if N >= 2 {
            remainder_limbs[1] = (remainder_u128 >> 64) as u64;
        }
        return Int::<N>::from_mag_limbs(&remainder_limbs, remainder_is_negative);
    }

    divmod_mags::<N>(&dividend_abs, &divisor_abs, remainder_is_negative)
}

/// The exact-scratch Knuth divmod remainder core. Operates on precomputed
/// unsigned magnitudes (`dividend_abs`, `divisor_abs`) and the dividend sign
/// (`remainder_is_negative`), so both [`rem_int_layer`] (fast-path miss) and
/// [`rem_int_layer_divmod`] (fast-path-free, for the microbench) share it.
#[inline]
fn divmod_mags<const N: usize>(
    dividend_abs: &crate::int::types::Uint<N>,
    divisor_abs: &crate::int::types::Uint<N>,
    remainder_is_negative: bool,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    let mut quotient = [0u64; N];
    let mut remainder = [0u64; N];
    // Exact per-`N` Knuth scratch: `single_buffered_u64` is `[u64; N + 2]`, covering
    // the normalised dividend `u` (`num.len() + 2`) and divisor `v`.
    let mut u = Limbs::<N>::single_buffered_u64();
    let mut v = Limbs::<N>::single_buffered_u64();
    // Exhaustive over the verdict (no `_`, so adding an engine forces a
    // decision here). Significant lengths are independent inside one
    // `Int<N>`, so the wide `num ≥ 2·den` u128 shape IS reachable at wide
    // `N` — honor that verdict rather than collapse
    // it onto Knuth.
    match select_for_limbs(dividend_abs.as_limbs(), divisor_abs.as_limbs()) {
        Algorithm::KnuthU128Limb => {
            // Operands are ≤ `N` limbs (one family step below
            // `div_widen_scale`'s `2N` dividend): the engine's minima are
            // `u64buf ≥ num.len()+2` / `u ≥ ⌈(num.len()+2)/2⌉ + 1`, met by
            // the shared `single_buffered_u64` normalisation buffers and
            // `double_u128` / `single_u128` packed scratch.
            let mut u128_u = Limbs::<N>::double_u128();
            let mut u128_v = Limbs::<N>::single_u128();
            div_knuth_u128_limb_into(
                dividend_abs.as_limbs(),
                divisor_abs.as_limbs(),
                &mut quotient,
                &mut remainder,
                u.as_mut(),
                v.as_mut(),
                u128_u.as_mut(),
                u128_v.as_mut(),
            );
        }
        Algorithm::Rem
        | Algorithm::Knuth
        | Algorithm::BurnikelZieglerWithKnuth
        | Algorithm::Schoolbook => div_knuth_into(
            dividend_abs.as_limbs(),
            divisor_abs.as_limbs(),
            &mut quotient,
            &mut remainder,
            u.as_mut(),
            v.as_mut(),
        ),
    }
    Int::<N>::from_mag_limbs(&remainder, remainder_is_negative)
}

/// The fast-path-FREE remainder: identical validation to [`rem_int_layer`]
/// but always the exact-scratch Knuth divmod (no single-word `u128 % u128`
/// short-circuit). Bit-identical to [`rem_int_layer`] at every operand value;
/// exposed only so the microbench can A/B the fast path's contribution
/// against the divmod-only path it guards.
#[inline]
pub(crate) fn rem_int_layer_divmod<const N: usize>(dividend: Int<N>, divisor: Int<N>) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if dividend == Int::<N>::MIN && divisor == -Int::<N>::ONE {
        panic!("attempt to calculate the remainder with overflow");
    }
    assert!(
        !divisor.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let remainder_is_negative = dividend.is_negative();
    let dividend_abs = dividend.unsigned_abs();
    let divisor_abs = divisor.unsigned_abs();
    divmod_mags::<N>(&dividend_abs, &divisor_abs, remainder_is_negative)
}

#[cfg(test)]
mod tests {
    use super::{rem_int_layer, rem_int_layer_divmod};
    use crate::int::types::Int;

    /// The single-word fast path must be bit-identical to the divmod-only
    /// path it guards, at every operand value: the small shape that takes the
    /// fast path, full-width operands that fall through, and all four sign
    /// combinations. The fast path only changes WHICH branch runs, never the
    /// result — this is its validity wall.
    #[test]
    fn fast_path_matches_divmod_only() {
        // Small operands (fit one u128) — the fast-path branch. All sign
        // combinations + the scale-0 benchmarked shape (2 % 1) and zero remainder.
        let small: &[(i128, i128)] = &[
            (2, 1),
            (100, 7),
            (-100, 7),
            (100, -7),
            (-100, -7),
            (0, 5),
            (5, 5),
            (i128::MAX, 3),
            (i128::MIN + 1, 7),
        ];
        for &(dividend, divisor) in small {
            let dividend_int = Int::<3>::from_i128(dividend);
            let divisor_int = Int::<3>::from_i128(divisor);
            assert_eq!(
                rem_int_layer::<3>(dividend_int, divisor_int),
                rem_int_layer_divmod::<3>(dividend_int, divisor_int),
                "fast path ({dividend} % {divisor}) at N=3"
            );
            // also a wide storage width
            let dividend_wide = Int::<16>::from_i128(dividend);
            let divisor_wide = Int::<16>::from_i128(divisor);
            assert_eq!(
                rem_int_layer::<16>(dividend_wide, divisor_wide),
                rem_int_layer_divmod::<16>(dividend_wide, divisor_wide),
                "fast path ({dividend} % {divisor}) at N=16"
            );
        }

        // Full-width operands (span all limbs) — the fall-through branch.
        let mut dividend_limbs = [0u64; 8];
        let mut divisor_limbs = [0u64; 8];
        for i in 0..8 {
            dividend_limbs[i] = 0x9E37_79B9_7F4A_7C15u64.wrapping_mul(i as u64 + 1);
            divisor_limbs[i] = 0xD1B5_4A32_D192_ED03u64.wrapping_mul(i as u64 + 3);
        }
        let dividend_int = Int::<8>::from_mag_limbs(&dividend_limbs, false);
        let divisor_int = Int::<8>::from_mag_limbs(&divisor_limbs, true); // negative divisor
        assert_eq!(
            rem_int_layer::<8>(dividend_int, divisor_int),
            rem_int_layer_divmod::<8>(dividend_int, divisor_int),
            "full-width fall-through"
        );
    }

    /// The wide `num ≥ 2·den` shape — a long dividend over an even
    /// mid-width divisor inside ONE `Int<N>` — genuinely reaches the
    /// matcher's `KnuthU128Limb` verdict (significant lengths are
    /// independent of `N`), so the divmod must honor it with the u128
    /// engine. Bit-identity wall: for operand pairs that PROVABLY route to
    /// `KnuthU128Limb` (asserted per pair), the remainder via
    /// [`rem_int_layer`] equals the [`div_knuth_into`] reference on the
    /// same magnitudes. Int-layer only (`Int<64>` + exact `ComputeLimbs`
    /// scratch — in the exact-scratch width list regardless of decimal
    /// tiers); gated on `exact-scratch` because the build-max blanket of a
    /// narrow no-default-features build undersizes 64-limb scratch.
    #[cfg(feature = "exact-scratch")]
    #[test]
    fn u128_verdict_shape_matches_knuth_reference() {
        use crate::int::algos::div::div_knuth::div_knuth_into;
        use crate::int::policy::div_rem::{select_for_limbs, Algorithm};

        const N: usize = 64;
        let mut state: u64 = 0xA076_1D64_78BD_642F;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        // Significant-limb shapes meeting the matcher's u128 gate:
        // den_n even, den_n >= U128_DIV_THRESHOLD (24), num_m >= 2*den_n.
        let shapes: &[(usize, usize)] = &[(64, 24), (64, 32), (48, 24), (56, 28), (50, 24)];
        for (case, &(num_n, den_n)) in shapes.iter().enumerate() {
            for round in 0..8 {
                let mut dividend_limbs = [0u64; N];
                let mut divisor_limbs = [0u64; N];
                for limb in dividend_limbs[..num_n].iter_mut() {
                    *limb = next();
                }
                for limb in divisor_limbs[..den_n].iter_mut() {
                    *limb = next();
                }
                // Keep the dividend magnitude in signed-positive range when
                // it spans all N limbs, and pin both top limbs nonzero so
                // the significant lengths are exactly (num_n, den_n).
                if num_n == N {
                    dividend_limbs[N - 1] &= !(1u64 << 63);
                }
                dividend_limbs[num_n - 1] |= 1;
                divisor_limbs[den_n - 1] |= 1;

                // The pair must provably route to the u128 engine — this is
                // what makes the test exercise the new arm, not Knuth.
                assert!(
                    select_for_limbs(&dividend_limbs, &divisor_limbs)
                        == Algorithm::KnuthU128Limb,
                    "case {case}: ({num_n},{den_n}) sig limbs must route to KnuthU128Limb"
                );

                // Reference remainder: the base-2^64 Knuth engine on the
                // same magnitudes (zeroed u/v, >= num.len()+2 / den.len()).
                let mut quotient = [0u64; N];
                let mut remainder = [0u64; N];
                let mut u = [0u64; N + 2];
                let mut v = [0u64; N + 2];
                div_knuth_into(
                    &dividend_limbs, &divisor_limbs,
                    &mut quotient, &mut remainder, &mut u, &mut v);

                // All four sign combinations; the remainder carries the
                // dividend's sign.
                let dividend_is_negative = round & 1 == 1;
                let divisor_is_negative = round & 2 == 2;
                let dividend_int = Int::<N>::from_mag_limbs(&dividend_limbs, dividend_is_negative);
                let divisor_int = Int::<N>::from_mag_limbs(&divisor_limbs, divisor_is_negative);
                let expected = Int::<N>::from_mag_limbs(&remainder, dividend_is_negative);
                assert_eq!(
                    rem_int_layer::<N>(dividend_int, divisor_int),
                    expected,
                    "case {case} round {round}: ({num_n},{den_n}) a_neg={dividend_is_negative} b_neg={divisor_is_negative}"
                );
            }
        }
    }
}
