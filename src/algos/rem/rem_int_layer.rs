// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rem_int_layer` -- decimal remainder via the `Int<N>` layer.

use crate::int::algos::div::div_knuth::div_knuth_into;
use crate::int::policy::div_rem::{select_for_limbs, Algorithm};
use crate::int::types::compute_int::ComputeInt;
use crate::int::types::Int;

/// Decimal remainder via the `Int<N>` layer. Applies Rust's standard
/// integer-overflow contract: division by zero always panics; `MIN % -ONE`
/// panics in debug (with "overflow") and wraps in release (matching
/// `i128::wrapping_rem`). No rescaling needed -- same-SCALE operands share
/// the scale factor.
///
/// A value-gated small-operand fast path runs FIRST: when both operand
/// magnitudes fit a single 128-bit word it takes a hardware `u128 % u128`
/// (no scratch, no shape classifier, no Knuth setup), recovering v0.4.4's
/// `limbs_divmod` "Fast Path A" generically. This is the dominant scale-0
/// decimal-`rem` shape (a bare integer / small `k`), where the divmod setup
/// dwarfs the divide. It is bit-identical to the divmod below, so it only
/// changes which path runs, never the result.
///
/// Otherwise it routes on the divide matcher's verdict
/// ([`select_for_limbs`](crate::int::policy::div_rem::select_for_limbs)) and
/// resolves the remainder via the chosen engine with **exact `ComputeInt`
/// scratch** (`single_buffered_u64`, `N + 2` per width) instead of the `Rem`
/// operator's build-max `[u64; MAX_SINGLE_LIMBS]` Knuth buffers. The balanced
/// `a % b` shape never presents the wide `num ≥ 2·den` form the u128 /
/// Burnikel–Ziegler engines require, so every verdict resolves to a correct
/// Knuth divide ([`div_knuth_into`] routes a single-limb divisor to the
/// hardware path internally) — but consulting the matcher (rather than
/// hardcoding the engine) means a future engine the matcher picks for this
/// shape reaches the kernel instead of being silently bypassed. Sizing the
/// normalised `u`/`v` to the operand width drops the build-max memset that
/// dominated the wide-tier remainder (98% of the cost at D57 … 12% at
/// D1232). The bare `Rem` operator must stay build-max (blanket over all `N`,
/// the `exact-scratch` wall); this concrete-`N` decimal kernel carries
/// `Int<N>: ComputeInt`.
///
/// Only reached for `N >= 3` (the decimal `rem` policy routes `N <= 2` to
/// `rem_native`), so the narrow hardware-`%` path is untouched; every such
/// `N` is in the `exact-scratch` width list, so the `ComputeInt` bound
/// discharges at the concrete `N` and never cascades.
///
/// [`div_knuth_into`]: crate::int::algos::div::div_knuth::div_knuth_into
#[inline]
pub(crate) fn rem_int_layer<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N>
where
    Int<N>: ComputeInt,
{
    // Divide-by-zero panics in both modes. In debug, the `MIN % -ONE`
    // overflow must also panic, matching the primitive contract and the
    // prior `checked_rem` path; detect it with cheap comparisons (no divide)
    // and panic before the divide wraps it to zero.
    if cfg!(debug_assertions) && a == Int::<N>::MIN && b == -Int::<N>::ONE {
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

    // Small-operand fast path (recovers v0.4.4's `limbs_divmod` "Fast Path A"
    // generically): when both magnitudes fit a single 128-bit word, take the
    // hardware `u128 % u128` and re-apply the dividend's sign — bypassing the
    // `select_for_limbs` shape classifier, the `single_buffered_u64` scratch
    // and the Knuth normalise/shift setup that `div_knuth_into` runs even on
    // tiny operands. This is the dominant scale-0 decimal-`rem` shape (a bare
    // integer / a small `k`, e.g. the `2 % 1` bbc cell at scale 0), where the
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
    Int<N>: ComputeInt,
{
    let mut quot = [0u64; N];
    let mut rem = [0u64; N];
    // Exact per-`N` Knuth scratch: `single_buffered_u64` is `[u64; N + 2]`, covering
    // the normalised dividend `u` (`num.len() + 2`) and divisor `v`.
    let mut u = Int::<N>::single_buffered_u64();
    let mut v = Int::<N>::single_buffered_u64();
    // Exhaustive over the verdict: the balanced shape only ever yields `Rem`
    // or `Knuth`, both correct via `div_knuth_into`; the wide-only engines are
    // unreachable here but listed so adding an engine forces a decision.
    match select_for_limbs(a_abs.as_limbs(), b_abs.as_limbs()) {
        Algorithm::Rem
        | Algorithm::Knuth
        | Algorithm::KnuthU128Limb
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
    Int<N>: ComputeInt,
{
    if cfg!(debug_assertions) && a == Int::<N>::MIN && b == -Int::<N>::ONE {
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
        // combinations + the scale-0 bbc shape (2 % 1) and zero remainder.
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
}
