// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Const single-/double-limb hardware divide (and the shift-subtract
//! fallback for the rare const multi-limb case).
//!
//! [`div_rem`] is the `const fn` divide the const-evaluable `wrapping_div`
//! / `wrapping_rem` route through so they can run at compile time. The
//! divisor-shape *choice* between the runtime engines lives in
//! [`crate::int::policy::div_rem`].

use crate::int::algos::div::div_mg::Mg2By1;
use crate::int::algos::support::limbs::{bit_len, cmp, fit_one, shl1, sub_assign};

/// `quotient = dividend / divisor`, `remainder = dividend % divisor`, u64
/// limbs. `const fn`.
///
/// Hardware fast paths:
/// - both fit a single u64 → one native `u64 / u64`
/// - divisor fits a single u64 → a Möller–Granlund 2-by-1 reciprocal
///   divide per dividend limb (one reciprocal precompute, then
///   mul/shift/correct per limb — see [`single_limb_div_rem`])
/// - otherwise → bit shift-subtract (only reached when divisor is
///   multi-limb; the dispatcher routes those to Knuth instead)
pub(crate) const fn div_rem(dividend: &[u64], divisor: &[u64], quotient: &mut [u64],
    remainder: &mut [u64]) {
    let mut z = 0;
    while z < quotient.len() {
        quotient[z] = 0;
        z += 1;
    }
    z = 0;
    while z < remainder.len() {
        remainder[z] = 0;
        z += 1;
    }

    let divisor_fits_one_limb = fit_one(divisor);

    // Fast path A: both fit a single u64 → hardware divide.
    if divisor_fits_one_limb && fit_one(dividend) {
        if !quotient.is_empty() {
            quotient[0] = dividend[0] / divisor[0];
        }
        if !remainder.is_empty() {
            remainder[0] = dividend[0] % divisor[0];
        }
        return;
    }

    // Fast path B: divisor fits a single u64 — schoolbook base-2^64 long
    // divide. Each step is a normalised 2-by-1 reciprocal divide (one
    // precompute, then mul/shift/correct per limb) rather than a software
    // `u128 ÷ u64` (`__udivti3`); see [`single_limb_div_rem`].
    if divisor_fits_one_limb {
        single_limb_div_rem(dividend, divisor[0], quotient, remainder);
        return;
    }

    // General path: binary shift-subtract. Only reached for multi-limb
    // divisors when the dispatcher isn't routing to Knuth (i.e. in const
    // contexts where Knuth isn't available).
    let dividend_bits = bit_len(dividend);
    let mut i = dividend_bits;
    while i > 0 {
        i -= 1;
        shl1(remainder);
        let bit = (dividend[(i / 64) as usize] >> (i % 64)) & 1;
        remainder[0] |= bit;
        shl1(quotient);
        if cmp(remainder, divisor) >= 0 {
            sub_assign(remainder, divisor);
            quotient[0] |= 1;
        }
    }
}

/// `quotient = dividend / divisor`, `remainder = dividend % divisor` for a
/// single non-zero u64 `divisor`, little-endian u64 limbs. Computes one
/// quotient limb per dividend limb (high → low) via the Möller–Granlund
/// 2-by-1 invariant-divisor reciprocal ([`Mg2By1`]) instead of a per-limb
/// software `u128 ÷ u64`.
///
/// On x86_64 the obvious `acc / (divisor as u128)` (`acc < divisor·2^64`,
/// quotient fits a u64) does NOT lower to one hardware `DIV r/m64`;
/// LLVM/compiler-builtins emit a full 128÷128 software routine (`__udivti3`).
/// `const fn` rules out inline `asm!` to reach the hardware instruction, so
/// this keeps the divide const-evaluable by replacing the per-limb division
/// with a reciprocal **multiplication**: precompute `divisor`'s reciprocal
/// once (amortised over every dividend limb), then each limb is a 64×64→128
/// multiply, a shift and a small correction — no software `__udivti3`.
///
/// `Mg2By1` requires a *normalised* divisor (top bit set) and a high word
/// strictly below it. So `divisor` is normalised by
/// `shift = divisor.leading_zeros()` into
/// `divisor_normalised = divisor << shift`; the dividend is streamed in the
/// matching left-shifted domain (each window word
/// `(dividend[i] << shift) | (dividend[i-1] >> (64-shift))`), the
/// `running_remainder` stays `< divisor_normalised` (the `Mg2By1`
/// precondition), and the true remainder is recovered as
/// `running_remainder >> shift`. The quotient is unchanged by the common left
/// shift of dividend and divisor.
///
/// Bit-identical to the prior `u128`-division loop for every
/// `(dividend, divisor)`.
const fn single_limb_div_rem(dividend: &[u64], divisor: u64, quotient: &mut [u64],
    remainder: &mut [u64]) {
    // Live dividend extent (skip leading zero limbs).
    let mut dividend_len = dividend.len();
    while dividend_len > 0 && dividend[dividend_len - 1] == 0 {
        dividend_len -= 1;
    }

    // Empty / zero dividend: quotient and remainder are already zeroed.
    if dividend_len == 0 {
        return;
    }

    let shift = divisor.leading_zeros();
    let divisor_normalised = divisor << shift;
    let recip = Mg2By1::new(divisor_normalised);

    // `running_remainder` is the running remainder in the normalised
    // (left-shifted by `shift`) domain; it stays `< divisor_normalised`,
    // satisfying the `Mg2By1::div_rem` high-word precondition. Seed it with the
    // bits the top limb shifts out (0 when `shift == 0`, since `x >> 64` is
    // undefined and there is no overflow word).
    let mut running_remainder: u64 = if shift == 0 {
        0
    } else {
        dividend[dividend_len - 1] >> (64 - shift)
    };

    let mut i = dividend_len;
    while i > 0 {
        i -= 1;
        // The dividend limb at position `i` in the left-shifted domain.
        let window_word = if shift == 0 {
            dividend[i]
        } else {
            let lo_from_below = if i > 0 { dividend[i - 1] >> (64 - shift) } else { 0 };
            (dividend[i] << shift) | lo_from_below
        };
        let (quotient_limb, next_remainder) = recip.div_rem(running_remainder, window_word);
        running_remainder = next_remainder;
        if i < quotient.len() {
            quotient[i] = quotient_limb;
        }
    }

    // De-normalise the remainder back out of the shifted domain.
    if !remainder.is_empty() {
        remainder[0] = running_remainder >> shift;
    }
}
