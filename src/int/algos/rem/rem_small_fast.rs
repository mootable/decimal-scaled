// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Small-magnitude hardware-`%` signed remainder for any `Int<N>`.
//!
//! [`rem_small_fast`] is the width-agnostic recovery of v0.4.4's
//! `limbs_divmod` "Fast Path A" (both operands fit one 128-bit word → a
//! single hardware divide, no scratch, no shape classification). It runs at
//! EVERY width `N`, gated on the operand VALUE rather than the const width:
//!
//! * when `|a| < |b|` the truncating remainder is the dividend itself
//!   (`a % b == a`), returned after one top-down `N`-limb magnitude compare —
//!   no divide at all. This catches the balanced-magnitude shape the
//!   single-word probe below misses (a divisor that crosses the 128-bit line
//!   while the dividend stays smaller);
//! * when both operand magnitudes fit a single `u128` (every limb above
//!   index 1 is zero on both inputs) it takes the hardware `u128 % u128`
//!   with the dividend's sign re-applied — bypassing the `[u64; N]` quotient
//!   scratch, the sign-magnitude divmod round trip, and the
//!   `div_rem::dispatch` shape classifier that the general
//!   [`crate::int::algos::rem::rem_via_div_rem`] pays even on tiny operands;
//! * otherwise it falls back to [`rem_via_div_rem`] (the division-policy
//!   path) for the genuinely-wide case.
//!
//! This is the scale-0 (small integer operand) recovery: at wide tiers the
//! decimal integer operands are frequently a small `k` (or `k · 10^scale`
//! that still fits a word) where building and walking `N`-limb scratch is
//! pure overhead. The fast path is correct for ALL `N` — the magnitude check
//! guarantees the `u128` load is lossless — so it is bit-identical to
//! `rem_via_div_rem` everywhere and ELIGIBLE wherever it is faster.
//!
//! Working on the unsigned magnitudes keeps it overflow-free: the
//! `i128::MIN % -1` hazard never arises (the magnitudes are unsigned and the
//! divisor magnitude is `>= 1`).

use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
use crate::int::types::{Int, Uint};

/// `true` and the packed `u128` magnitude when every limb of `mag` above
/// index 1 is zero (i.e. the magnitude fits a single 128-bit word).
#[inline]
fn mag_fits_u128<const N: usize>(mag: &Uint<N>) -> (bool, u128) {
    let l = mag.as_limbs();
    let lo = l[0] as u128;
    let hi = if N >= 2 { l[1] as u128 } else { 0 };
    // Any limb at index >= 2 set means the magnitude exceeds u128.
    let mut fits = true;
    let mut i = 2;
    while i < N {
        if l[i] != 0 {
            fits = false;
            break;
        }
        i += 1;
    }
    (fits, lo | (hi << 64))
}

/// Value-gated hardware-`%` signed remainder for `Int<N>`, any `N`.
///
/// Takes the hardware `u128 % u128` when both magnitudes fit a single word,
/// re-applying the dividend's sign (truncating-toward-zero); otherwise
/// delegates to [`rem_via_div_rem`]. Panics on a zero divisor, matching the
/// `Rem` operator contract. Bit-identical to [`rem_via_div_rem`] at every
/// `N` and every operand value.
#[inline]
pub(crate) fn rem_small_fast<const N: usize>(a: Int<N>, b: Int<N>) -> Int<N> {
    assert!(
        !b.is_zero(),
        "attempt to calculate the remainder with a divisor of zero"
    );
    // Compute both magnitudes ONCE (the sign-magnitude conversion the divmod
    // also needs) and reuse them on whichever branch is taken — neither path
    // re-walks `unsigned_abs`.
    let a_mag = a.unsigned_abs();
    let b_mag = b.unsigned_abs();
    let neg_r = a.is_negative();
    // Dividend-smaller short-circuit: when `|a| < |b|` the truncating
    // remainder is the dividend itself (`a % b == a`), so return `a`
    // unchanged — one top-down `N`-limb magnitude compare (`Uint::cmp`), no
    // hardware divide, no `[u64; N]` quotient scratch, no `div_rem` shape
    // classifier. Correct for EVERY `N` and value, and it catches the
    // balanced-magnitude shape the single-word `u128` probe below MISSES (an
    // operand pair where the divisor crosses the 128-bit line but the
    // dividend is still smaller). Bit-identical to the divmod (which also
    // yields `rem == a` here).
    if a_mag < b_mag {
        return a;
    }
    let (a_fits, a_u) = mag_fits_u128::<N>(&a_mag);
    let (b_fits, b_u) = mag_fits_u128::<N>(&b_mag);
    let mut rem = [0u64; N];
    if a_fits && b_fits {
        let r = a_u % b_u;
        rem[0] = r as u64;
        if N >= 2 {
            rem[1] = (r >> 64) as u64;
        }
    } else {
        let mut quot = [0u64; N];
        div_rem_dispatch(a_mag.as_limbs(), b_mag.as_limbs(), &mut quot, &mut rem);
    }
    Int::<N>::from_mag_limbs(&rem, neg_r)
}

#[cfg(test)]
mod tests {
    use super::rem_small_fast;
    use crate::int::algos::rem::rem_via_div_rem::rem_via_div_rem;
    use crate::int::types::Int;

    /// Bit-identical to the via-div-rem reference at a wide tier for small
    /// operands (the fast-path branch) AND full-width operands (the
    /// fallback branch).
    #[test]
    fn matches_via_div_rem_wide() {
        // small operands: both fit one u128 — fast path.
        let small: &[(i128, i128)] = &[
            (100, 7),
            (-100, 7),
            (100, -7),
            (-100, -7),
            (0, 5),
            (i128::MAX, 3),
            (i128::MIN + 1, 3),
        ];
        for &(a, b) in small {
            let ia = Int::<8>::from_i128(a);
            let ib = Int::<8>::from_i128(b);
            assert_eq!(
                rem_small_fast::<8>(ia, ib),
                rem_via_div_rem::<8>(ia, ib),
                "fast path ({a} % {b})"
            );
        }
        // full-width operands: fallback path. Build magnitudes that span
        // many limbs.
        let mut a_lim = [0u64; 8];
        let mut b_lim = [0u64; 8];
        for i in 0..8 {
            a_lim[i] = 0x9E37_79B9_7F4A_7C15u64.wrapping_mul(i as u64 + 1);
            b_lim[i] = 0xD1B5_4A32_D192_ED03u64.wrapping_mul(i as u64 + 3);
        }
        let ia = Int::<8>::from_mag_limbs(&a_lim, false);
        let ib = Int::<8>::from_mag_limbs(&b_lim, false);
        assert_eq!(
            rem_small_fast::<8>(ia, ib),
            rem_via_div_rem::<8>(ia, ib),
            "fallback path full-width"
        );
    }
}
