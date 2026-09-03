// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! candidate: `Fixed::div_small` routed through the int layer's divide
//! matcher (Möller–Granlund single-limb engine for a one-word divisor),
//! not wired.
//!
//! ## Why
//!
//! `Fixed::div_small`'s `divisor <= u64::MAX` arm (`fixed.rs`, the
//! "hardware divide" fast path) performs four `u128 / u128` divisions of
//! the form `((remainder << 64) | limb) / divisor`. That is exactly the
//! shape `int/algos/div/div_rem.rs` documents as NOT lowering to one
//! hardware `DIV r/m64` — LLVM/compiler-builtins emit the full software
//! 128÷128 routine (`__udivti3`) because the quotient-fits-a-word
//! precondition (`remainder < divisor`) is a loop invariant the compiler
//! cannot prove. The int layer already migrated its own identical loop to
//! the Möller–Granlund 2-by-1 reciprocal for this reason
//! (`single_limb_div_rem`), with bit-identity to the u128-division loop
//! proven by its tests. The narrow-tier `Fixed` engine never received the
//! same migration: its WIDE-divisor arm routes to the int layer
//! (`div_rem_via_int_layer`) while its "fast" arm keeps the libcalls.
//!
//! `div_small` runs once per Taylor/artanh term in every narrow series
//! kernel (`exp_series_2limb.rs:220`, `ln_series_2limb.rs:145`,
//! `trig_series_2limb.rs:244,285,418`, `angle_schoolbook.rs:87`,
//! `trig_series_2limb.rs:1944`), so its cost recurs tens of times per
//! narrow strict transcendental call.
//!
//! ## What this candidate does
//!
//! Collapses BOTH arms of `div_small` onto the existing
//! `mg_divide::div_rem_via_int_layer` bridge (the one the wide arm
//! already uses). A one-limb divisor takes the matcher's `Rem` verdict
//! into `div_rem`'s Möller–Granlund single-limb engine — one reciprocal
//! precompute, then a multiply/shift/correct per limb instead of a
//! `__udivti3` per limb. A two-limb divisor takes the same Knuth door the
//! current wide arm takes, unchanged.
//!
//! Bit-identity argument: both the current loop and the MG engine compute
//! the unique base-2^64 schoolbook quotient digits of the same dividend
//! by the same divisor (`single_limb_div_rem` is documented and tested
//! bit-identical to the prior u128-division loop it replaced); the sign
//! rule below reproduces `div_small`'s. The test module asserts identity
//! against `Fixed::div_small` over dense/sparse magnitudes and the series
//! divisor family.
//!
//! ## Proposed wiring (coordinator)
//!
//! Replace the body of `Fixed::div_small` with a call to
//! [`div_small_int_layer`]; delete nothing (the current body remains in
//! `fixed.rs` history / tests as the reference). Micro-bench first at a
//! narrow series seam (e.g. sin at D38 high scale), per
//! `feedback_micro_bench_first`.

use crate::algos::support::fixed::Fixed;
use crate::algos::support::mg_divide::div_rem_via_int_layer;

/// `value / divisor`, truncating toward zero — the int-layer-routed
/// sibling of [`Fixed::div_small`]. `divisor` must be non-zero.
#[allow(dead_code)]
pub(crate) fn div_small_int_layer(value: Fixed, divisor: u128) -> Fixed {
    debug_assert!(divisor != 0, "division by zero");
    let num = [
        value.mag[0] as u64,
        (value.mag[0] >> 64) as u64,
        value.mag[1] as u64,
        (value.mag[1] >> 64) as u64,
    ];
    let mut quot = [0u64; 4];
    let mut rem = [0u64; 2];
    if divisor <= u64::MAX as u128 {
        // One-limb divisor: the matcher's `Rem` verdict — the
        // Möller–Granlund single-limb engine, no normalisation scratch.
        div_rem_via_int_layer(&num, &[divisor as u64], &mut quot, &mut rem);
    } else {
        // Two-limb divisor: identical to `div_small`'s current wide arm.
        div_rem_via_int_layer(
            &num,
            &[divisor as u64, (divisor >> 64) as u64],
            &mut quot,
            &mut rem,
        );
    }
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

    /// SplitMix64 — deterministic pattern stream (same generator as the
    /// `fixed.rs` bit-identity sweeps).
    fn mix(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn mix_u128(state: &mut u64) -> u128 {
        (u128::from(mix(state)) << 64) | u128::from(mix(state))
    }

    /// Bit-identity with `Fixed::div_small` across dense and sparse
    /// magnitudes, both signs, and the divisor families the series
    /// kernels actually pass: small odd `2k+1`, `(2k)(2k+1)`, `180`,
    /// powers of ten, near-u64-boundary values, and above-u64 values
    /// (the wide arm).
    #[test]
    fn div_small_int_layer_matches_div_small() {
        let mut state = 0x00D1_1500_u64;
        let mut divisors: [u128; 24] = [0; 24];
        let mut i = 0;
        // Series family: 2k+1 and (2k)(2k+1) for k in 1..=6.
        for k in 1u128..=6 {
            divisors[i] = 2 * k + 1;
            divisors[i + 1] = (2 * k) * (2 * k + 1);
            i += 2;
        }
        divisors[12] = 1;
        divisors[13] = 2;
        divisors[14] = 3;
        divisors[15] = 180;
        divisors[16] = 10u128.pow(19);
        divisors[17] = u64::MAX as u128 - 1;
        divisors[18] = u64::MAX as u128;
        divisors[19] = u64::MAX as u128 + 1;
        divisors[20] = u64::MAX as u128 + 3;
        divisors[21] = 10u128.pow(38);
        divisors[22] = u128::MAX / 2;
        divisors[23] = u128::MAX;
        for &divisor in &divisors {
            for case in 0..200u32 {
                // Cycle magnitude shapes: dense 256-bit, low-only,
                // high-only, small.
                let mag = match case % 4 {
                    0 => [mix_u128(&mut state), mix_u128(&mut state)],
                    1 => [mix_u128(&mut state), 0],
                    2 => [0, mix_u128(&mut state)],
                    _ => [u128::from(mix(&mut state)), 0],
                };
                let negative = case % 3 == 0;
                let value = Fixed { negative, mag };
                let expected = value.div_small(divisor);
                let actual = div_small_int_layer(value, divisor);
                assert_eq!(
                    actual, expected,
                    "div_small_int_layer mismatch: mag={mag:?} neg={negative} d={divisor}"
                );
            }
        }
    }
}
