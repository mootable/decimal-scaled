// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Const single-/double-limb hardware divide (and the shift-subtract
//! fallback for the rare const multi-limb case).
//!
//! [`div_rem`] is the `const fn` divide the const-evaluable `wrapping_div`
//! / `wrapping_rem` route through so they can run at compile time. The
//! divisor-shape *choice* between the runtime engines lives in
//! [`crate::int::policy::div_rem`].

use crate::int::algos::support::limbs::{bit_len, cmp, fit_one, shl1, sub_assign};

/// `quot = num / den`, `rem = num % den`, u64 limbs. `const fn`.
///
/// Hardware fast paths:
/// - both fit a single u64 → one native `u64 / u64`
/// - divisor fits a single u64 → native `u128 / u64` per dividend limb
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
    // divide using the native u128/u64 hardware divide.
    if den_one_limb {
        let d = den[0];
        let mut r: u64 = 0;
        let mut top = num.len();
        while top > 0 && num[top - 1] == 0 {
            top -= 1;
        }
        let mut i = top;
        while i > 0 {
            i -= 1;
            let acc = ((r as u128) << 64) | (num[i] as u128);
            let q = (acc / (d as u128)) as u64;
            r = (acc % (d as u128)) as u64;
            if i < quot.len() {
                quot[i] = q;
            }
        }
        if !rem.is_empty() {
            rem[0] = r;
        }
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
