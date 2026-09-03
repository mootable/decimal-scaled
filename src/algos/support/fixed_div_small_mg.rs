// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: `Fixed::div_small` u64-divisor fast path routed through the int
// layer's Möller–Granlund single-limb engine, not wired.
//
//! The current fast path in [`Fixed::div_small`] performs its four base-2^64
//! long-division steps as `u128 ÷ u128` expressions. Release-mode assembly
//! (x86-64) shows three of the four lower to `__udivti3` software-division
//! calls (only the top step, whose incoming remainder is provably zero, is
//! salvaged into a hardware `divq`/`divl`), each with the Windows-ABI
//! by-pointer `u128` argument marshalling around it — the same lowering
//! failure `int::algos::div::div_rem` documents and already solved with the
//! Möller–Granlund 2-by-1 reciprocal ([`Mg2By1`]): one reciprocal precompute
//! per divisor, then one multiply/shift/correct step per dividend limb, no
//! software division in the loop.
//!
//! This candidate routes the u64-divisor path through that existing engine
//! ([`div_rem`] dispatches a single-limb divisor to its
//! `single_limb_div_rem`), whose header documents bit-identity with the
//! per-limb `u128`-division formulation for every `(dividend, divisor)`.
//! Divisors above `u64::MAX` delegate to [`Fixed::div_small`] unchanged.
//!
//! [`Mg2By1`]: crate::int::algos::div::div_mg::Mg2By1
//! [`div_rem`]: crate::int::algos::div::div_rem::div_rem

use crate::algos::support::fixed::Fixed;

/// Divides by an unsigned non-zero `u128` `divisor`, truncating toward
/// zero — same contract and same bits as [`Fixed::div_small`].
///
/// For `divisor <= u64::MAX` the four-limb magnitude is divided by the int
/// layer's single-limb Möller–Granlund engine; wider divisors take
/// [`Fixed::div_small`]'s existing wide path unchanged.
#[allow(dead_code)]
pub(crate) fn div_small_mg(value: Fixed, divisor: u128) -> Fixed {
    debug_assert!(divisor != 0, "division by zero");
    if divisor > u64::MAX as u128 {
        return value.div_small(divisor);
    }
    let num = [
        value.mag[0] as u64,
        (value.mag[0] >> 64) as u64,
        value.mag[1] as u64,
        (value.mag[1] >> 64) as u64,
    ];
    let den = [divisor as u64];
    let mut quot = [0u64; 4];
    let mut rem = [0u64; 1];
    crate::int::algos::div::div_rem::div_rem(&num, &den, &mut quot, &mut rem);
    let q_lo = u128::from(quot[0]) | (u128::from(quot[1]) << 64);
    let q_hi = u128::from(quot[2]) | (u128::from(quot[3]) << 64);
    Fixed {
        negative: value.negative && !(q_lo == 0 && q_hi == 0),
        mag: [q_lo, q_hi],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic pattern stream (xorshift64*) for the sweep below.
    fn mix(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        state.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn mix_u128(state: &mut u64) -> u128 {
        (u128::from(mix(state)) << 64) | u128::from(mix(state))
    }

    /// Bit-identity with `Fixed::div_small` across the u64-divisor fast
    /// path: series-shaped small divisors, u64-boundary divisors, and
    /// pattern magnitudes, both signs.
    #[test]
    fn div_small_mg_matches_div_small() {
        let mut state = 0x5eed_0451_d1f5_0a11_u64;
        let divisors: [u128; 8] = [
            1,
            3,
            7,
            999_983,
            1_299_709,
            u128::from(u32::MAX),
            u128::from(u64::MAX) - 1,
            u128::from(u64::MAX),
        ];
        let mut mags: [[u128; 2]; 6] = [
            [0, 0],
            [1, 0],
            [u128::MAX, 0],
            [0, u128::MAX],
            [u128::MAX, u128::MAX],
            [0, 0],
        ];
        for _ in 0..64 {
            mags[5] = [mix_u128(&mut state), mix_u128(&mut state)];
            for &divisor in &divisors {
                for &mag in &mags {
                    for negative in [false, true] {
                        let value = Fixed { negative, mag };
                        let expected = value.div_small(divisor);
                        let actual = div_small_mg(value, divisor);
                        assert_eq!(
                            actual.mag, expected.mag,
                            "div_small_mg mag mismatch: mag={mag:?} d={divisor}"
                        );
                        assert_eq!(
                            actual.negative, expected.negative,
                            "div_small_mg sign mismatch: mag={mag:?} d={divisor}"
                        );
                    }
                }
            }
        }
    }
}
