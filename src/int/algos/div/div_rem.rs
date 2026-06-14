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

/// `quot = num / den`, `rem = num % den`, u64 limbs. `const fn`.
///
/// Hardware fast paths:
/// - both fit a single u64 → one native `u64 / u64`
/// - divisor fits a single u64 → a Möller–Granlund 2-by-1 reciprocal
///   divide per dividend limb (one reciprocal precompute, then
///   mul/shift/correct per limb — see [`single_limb_div_rem`])
/// - otherwise → bit shift-subtract (only reached when divisor is
///   multi-limb; the dispatcher routes those to Knuth instead)
pub(crate) const fn div_rem(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let mut z = 0;
    while z < quot.len() {
        quot[z] = 0;
        z += 1;
    }
    z = 0;
    while z < rem.len() {
        rem[z] = 0;
        z += 1;
    }

    let den_one_limb = fit_one(den);

    // Fast path A: both fit a single u64 → hardware divide.
    if den_one_limb && fit_one(num) {
        if !quot.is_empty() {
            quot[0] = num[0] / den[0];
        }
        if !rem.is_empty() {
            rem[0] = num[0] % den[0];
        }
        return;
    }

    // Fast path B: divisor fits a single u64 — schoolbook base-2^64 long
    // divide. Each step is a normalised 2-by-1 reciprocal divide (one
    // precompute, then mul/shift/correct per limb) rather than a software
    // `u128 ÷ u64` (`__udivti3`); see [`single_limb_div_rem`].
    if den_one_limb {
        single_limb_div_rem(num, den[0], quot, rem);
        return;
    }

    // General path: binary shift-subtract. Only reached for multi-limb
    // divisors when the dispatcher isn't routing to Knuth (i.e. in const
    // contexts where Knuth isn't available).
    let bits = bit_len(num);
    let mut i = bits;
    while i > 0 {
        i -= 1;
        shl1(rem);
        let bit = (num[(i / 64) as usize] >> (i % 64)) & 1;
        rem[0] |= bit;
        shl1(quot);
        if cmp(rem, den) >= 0 {
            sub_assign(rem, den);
            quot[0] |= 1;
        }
    }
}

/// `quot = num / d`, `rem = num % d` for a single non-zero u64 divisor `d`,
/// little-endian u64 limbs. Computes one quotient limb per dividend limb
/// (high → low) via the Möller–Granlund 2-by-1 invariant-divisor reciprocal
/// ([`Mg2By1`]) instead of a per-limb software `u128 ÷ u64`.
///
/// On x86_64 the obvious `acc / (d as u128)` (`acc < d·2^64`, quotient fits
/// a u64) does NOT lower to one hardware `DIV r/m64`; LLVM/compiler-builtins
/// emit a full 128÷128 software routine (`__udivti3`). `const fn` rules out
/// inline `asm!` to reach the hardware instruction, so this keeps the divide
/// const-evaluable by replacing the per-limb division with a reciprocal
/// **multiplication**: precompute `d`'s reciprocal once (amortised over every
/// dividend limb), then each limb is a 64×64→128 multiply, a shift and a small
/// correction — no software `__udivti3`.
///
/// `Mg2By1` requires a *normalised* divisor (top bit set) and a high word
/// strictly below it. So `d` is normalised by `s = d.leading_zeros()` into
/// `dn = d << s`; the dividend is streamed in the matching left-shifted
/// domain (each window word `(num[i] << s) | (num[i-1] >> (64-s))`), the
/// running remainder `r` stays `< dn` (the `Mg2By1` precondition), and the
/// true remainder is recovered as `r >> s`. The quotient is unchanged by the
/// common left shift of dividend and divisor.
///
/// Bit-identical to the prior `u128`-division loop for every `(num, d)`.
const fn single_limb_div_rem(num: &[u64], d: u64, quot: &mut [u64], rem: &mut [u64]) {
    // Live dividend extent (skip leading zero limbs).
    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }

    // Empty / zero dividend: quotient and remainder are already zeroed.
    if top == 0 {
        return;
    }

    let s = d.leading_zeros();
    let dn = d << s;
    let recip = Mg2By1::new(dn);

    // `r` is the running remainder in the normalised (left-shifted by `s`)
    // domain; it stays `< dn`, satisfying the `Mg2By1::div_rem` high-word
    // precondition. Seed it with the bits the top limb shifts out (0 when
    // `s == 0`, since `x >> 64` is undefined and there is no overflow word).
    let mut r: u64 = if s == 0 { 0 } else { num[top - 1] >> (64 - s) };

    let mut i = top;
    while i > 0 {
        i -= 1;
        // The dividend limb at position `i` in the left-shifted domain.
        let w = if s == 0 {
            num[i]
        } else {
            let lo_from_below = if i > 0 { num[i - 1] >> (64 - s) } else { 0 };
            (num[i] << s) | lo_from_below
        };
        let (q, r_next) = recip.div_rem(r, w);
        r = r_next;
        if i < quot.len() {
            quot[i] = q;
        }
    }

    // De-normalise the remainder back out of the shifted domain.
    if !rem.is_empty() {
        rem[0] = r >> s;
    }
}
