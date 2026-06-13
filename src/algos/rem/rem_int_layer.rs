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
/// Two value-gated fast paths run FIRST. (1) When `|a| < |b|` the truncating
/// remainder is the dividend itself (`a % b == a`), returned after one
/// top-down magnitude compare — no divide, no scratch. This is the dominant
/// decimal-`rem` benchmarked shape (`x % b` with `|x| < |b|`, e.g. `2.0 % 3.5`) and it
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
/// Only reached for `N >= 3` (the decimal `rem` policy routes `N <= 2` to
/// `rem_native`), so the narrow hardware-`%` path is untouched; every such
/// `N` is in the `exact-scratch` width list, so the `ComputeLimbs` bound
/// discharges at the concrete `N` and never cascades.
///
/// [`div_knuth_into`]: crate::int::algos::div::div_knuth::div_knuth_into
#[inline]
pub(crate) fn rem_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    // Divide-by-zero panics, and the `MIN % -ONE` overflow must panic in
    // BOTH debug and release (the default operator never silently wraps to
    // `0`): detect it with cheap comparisons (no divide) and panic before
    // the divide wraps it.
    if a == Int::<N>::MIN && b == -Int::<N>::ONE {
        panic!("attempt to calculate the remainder with overflow");
    }
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );

    // Truncating-toward-zero: the remainder carries the dividend's sign.
    let neg_r = a.is_negative();
    let a_abs = a.unsigned_abs();
    let b_abs = b.unsigned_abs();

    // Dividend-smaller short-circuit: when `|a| < |b|` the truncating
    // remainder is the dividend itself (`a % b == a`), so return `a`
    // unchanged — no divide, no scratch, no shape classifier. This is one
    // top-down `N`-limb magnitude compare (`Uint::cmp`), correct for EVERY
    // `N` and operand value, and it catches the dominant decimal-`rem` shape
    // the u128 fast path below MISSES: a balanced-magnitude `x % b` where the
    // SCALED divisor crosses the 128-bit line (e.g. the benchmarked `2.0 % 3.5` cell
    // at D76 s38: `2·10^38` fits a u128 but `3.5·10^38` is 129 bits, so the
    // u128 probe fails and the operands fall into a full multi-limb Knuth
    // divmod whose `top < n` early-out the compare reaches first, for free).
    // Bit-identical to the divmod (which also yields `rem == a` here), so it
    // only changes which path runs, never the result.
    if a_abs < b_abs {
        return a;
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
    let al = a_abs.as_limbs();
    let bl = b_abs.as_limbs();
    // Probe whether both magnitudes fit one 128-bit word (every limb above
    // index 1 zero). Break on the FIRST set high limb so a full-width operand
    // pays only a couple of comparisons before falling through to the divmod
    // (the wide balanced shape must not regress on the scan cost).
    let mut fits = true;
    let mut i = 2;
    while i < N {
        if al[i] != 0 || bl[i] != 0 {
            fits = false;
            break;
        }
        i += 1;
    }
    if fits {
        let a_hi = if N >= 2 { al[1] as u128 } else { 0 };
        let b_hi = if N >= 2 { bl[1] as u128 } else { 0 };
        let a_u = (al[0] as u128) | (a_hi << 64);
        let b_u = (bl[0] as u128) | (b_hi << 64);
        let r = a_u % b_u;
        let mut rem = [0u64; N];
        rem[0] = r as u64;
        if N >= 2 {
            rem[1] = (r >> 64) as u64;
        }
        return Int::<N>::from_mag_limbs(&rem, neg_r);
    }

    divmod_mags::<N>(&a_abs, &b_abs, neg_r)
}

/// The exact-scratch Knuth divmod remainder core. Operates on precomputed
/// unsigned magnitudes (`a_abs`, `b_abs`) and the dividend sign (`neg_r`),
/// so both [`rem_int_layer`] (fast-path miss) and
/// [`rem_int_layer_divmod`] (fast-path-free, for the microbench) share it.
#[inline]
fn divmod_mags<const N: usize>(
    a_abs: &crate::int::types::Uint<N>,
    b_abs: &crate::int::types::Uint<N>,
    neg_r: bool,
) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    let mut quot = [0u64; N];
    let mut rem = [0u64; N];
    // Exact per-`N` Knuth scratch: `single_buffered_u64` is `[u64; N + 2]`, covering
    // the normalised dividend `u` (`num.len() + 2`) and divisor `v`.
    let mut u = Limbs::<N>::single_buffered_u64();
    let mut v = Limbs::<N>::single_buffered_u64();
    // Exhaustive over the verdict (no `_`, so adding an engine forces a
    // decision here). Significant lengths are independent inside one
    // `Int<N>`, so the wide `num ≥ 2·den` u128 shape IS reachable at wide
    // `N` — honor that verdict rather than collapse
    // it onto Knuth.
    match select_for_limbs(a_abs.as_limbs(), b_abs.as_limbs()) {
        Algorithm::KnuthU128Limb => {
            // Operands are ≤ `N` limbs (one family step below
            // `div_widen_scale`'s `2N` dividend): the engine's minima are
            // `u64buf ≥ num.len()+2` / `u ≥ ⌈(num.len()+2)/2⌉ + 1`, met by
            // the shared `single_buffered_u64` normalisation buffers and
            // `double_u128` / `single_u128` packed scratch.
            let mut u128_u = Limbs::<N>::double_u128();
            let mut u128_v = Limbs::<N>::single_u128();
            div_knuth_u128_limb_into(
                a_abs.as_limbs(),
                b_abs.as_limbs(),
                &mut quot,
                &mut rem,
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
            a_abs.as_limbs(),
            b_abs.as_limbs(),
            &mut quot,
            &mut rem,
            u.as_mut(),
            v.as_mut(),
        ),
    }
    Int::<N>::from_mag_limbs(&rem, neg_r)
}

/// The fast-path-FREE remainder: identical validation to [`rem_int_layer`]
/// but always the exact-scratch Knuth divmod (no single-word `u128 % u128`
/// short-circuit). Bit-identical to [`rem_int_layer`] at every operand value;
/// exposed only so the microbench can A/B the fast path's contribution
/// against the divmod-only path it guards.
#[inline]
pub(crate) fn rem_int_layer_divmod<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if a == Int::<N>::MIN && b == -Int::<N>::ONE {
        panic!("attempt to calculate the remainder with overflow");
    }
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    let neg_r = a.is_negative();
    let a_abs = a.unsigned_abs();
    let b_abs = b.unsigned_abs();
    divmod_mags::<N>(&a_abs, &b_abs, neg_r)
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
        for &(a, b) in small {
            let ia = Int::<3>::from_i128(a);
            let ib = Int::<3>::from_i128(b);
            assert_eq!(
                rem_int_layer::<3>(ia, ib),
                rem_int_layer_divmod::<3>(ia, ib),
                "fast path ({a} % {b}) at N=3"
            );
            // also a wide storage width
            let ja = Int::<16>::from_i128(a);
            let jb = Int::<16>::from_i128(b);
            assert_eq!(
                rem_int_layer::<16>(ja, jb),
                rem_int_layer_divmod::<16>(ja, jb),
                "fast path ({a} % {b}) at N=16"
            );
        }

        // Full-width operands (span all limbs) — the fall-through branch.
        let mut a_lim = [0u64; 8];
        let mut b_lim = [0u64; 8];
        for i in 0..8 {
            a_lim[i] = 0x9E37_79B9_7F4A_7C15u64.wrapping_mul(i as u64 + 1);
            b_lim[i] = 0xD1B5_4A32_D192_ED03u64.wrapping_mul(i as u64 + 3);
        }
        let ia = Int::<8>::from_mag_limbs(&a_lim, false);
        let ib = Int::<8>::from_mag_limbs(&b_lim, true); // negative divisor
        assert_eq!(
            rem_int_layer::<8>(ia, ib),
            rem_int_layer_divmod::<8>(ia, ib),
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
                let mut a_lim = [0u64; N];
                let mut b_lim = [0u64; N];
                for x in a_lim[..num_n].iter_mut() {
                    *x = next();
                }
                for x in b_lim[..den_n].iter_mut() {
                    *x = next();
                }
                // Keep the dividend magnitude in signed-positive range when
                // it spans all N limbs, and pin both top limbs nonzero so
                // the significant lengths are exactly (num_n, den_n).
                if num_n == N {
                    a_lim[N - 1] &= !(1u64 << 63);
                }
                a_lim[num_n - 1] |= 1;
                b_lim[den_n - 1] |= 1;

                // The pair must provably route to the u128 engine — this is
                // what makes the test exercise the new arm, not Knuth.
                assert!(
                    select_for_limbs(&a_lim, &b_lim) == Algorithm::KnuthU128Limb,
                    "case {case}: ({num_n},{den_n}) sig limbs must route to KnuthU128Limb"
                );

                // Reference remainder: the base-2^64 Knuth engine on the
                // same magnitudes (zeroed u/v, >= num.len()+2 / den.len()).
                let mut quot = [0u64; N];
                let mut rem = [0u64; N];
                let mut u = [0u64; N + 2];
                let mut v = [0u64; N + 2];
                div_knuth_into(&a_lim, &b_lim, &mut quot, &mut rem, &mut u, &mut v);

                // All four sign combinations; the remainder carries the
                // dividend's sign.
                let a_neg = round & 1 == 1;
                let b_neg = round & 2 == 2;
                let ia = Int::<N>::from_mag_limbs(&a_lim, a_neg);
                let ib = Int::<N>::from_mag_limbs(&b_lim, b_neg);
                let expected = Int::<N>::from_mag_limbs(&rem, a_neg);
                assert_eq!(
                    rem_int_layer::<N>(ia, ib),
                    expected,
                    "case {case} round {round}: ({num_n},{den_n}) a_neg={a_neg} b_neg={b_neg}"
                );
            }
        }
    }
}
