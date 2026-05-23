// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newton integer square root over little-endian `u64` limb slices.
//!
//! [`isqrt_newton`] is the width-agnostic Newton integer square root used
//! by the fixed-width fast-arm dispatch
//! [`crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed`] (`N >= 3`)
//! and by the decimal `sqrt` work-width path. Pure kernel — it takes the
//! operand and writes `floor(sqrt(n))`; no algorithm choice.

use crate::int::algos::limbs::{add_assign, bit_len, cmp, is_zero, shr};
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

// On `no_std` the f64 inherent method (`sqrt`) used by the seed path is
// unavailable; pull it in via `num_traits::Float` (libm-backed). Under
// `std` the inherent method wins, so this import is gated out to avoid an
// unused-import warning and to keep the std float path bit-for-bit
// unchanged.
#[cfg(not(feature = "std"))]
use num_traits::Float as _;

/// Scratch capacity for the Newton isqrt kernel — 288 u64 limbs
/// (18432 bits), covering the widest work integer in the crate
/// (Int<256> used by D1232 cbrt, 256 u64 limbs) with isqrt scratch slack.
const SCRATCH_LIMBS: usize = 288;

/// `out = floor(sqrt(n))`. Newton iteration on top of the runtime divide
/// dispatcher.
///
/// History: this routine previously called the *const* `div_rem` per
/// iteration, which routes multi-limb divisors through the O(bits²)
/// shift-subtract path. At Int<64> (n=64 u64 limbs) that dominates wall
/// time — Newton converges in ~log₂(b) ≈ 12 iterations, each one a
/// `~65k`-limb-op divmod. Switching to [`div_rem_dispatch`] gets
/// Knuth-base-2⁶⁴ per iteration (~`~32²` = 1024 limb-ops), worth ~40× on
/// D307 sqrt.
pub(crate) fn isqrt_newton(n: &[u64], out: &mut [u64]) {
    for o in out.iter_mut() {
        *o = 0;
    }
    let bits = bit_len(n);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        out[0] = 1;
        return;
    }
    let work = n.len() + 1;
    debug_assert!(work <= SCRATCH_LIMBS, "isqrt scratch overflow");
    let mut x = [0u64; SCRATCH_LIMBS];

    // Initial guess. The classical seed is a single bit at position
    // `ceil(bits/2)` — one bit of accuracy, costing one Newton step per
    // doubling of accuracy (≈ `log2(bits/2)` iterations at any width).
    //
    // The hardware-`f64::sqrt` seed below lifts that to ~53 correct bits
    // in one go: extract the top 64 bits of `n` (which fits the f64
    // mantissa with 11 bits of headroom), take the hardware sqrt, and
    // shift the result back to the correct magnitude. For Int<8> (D76
    // sqrt input) this drops the Newton iteration count from ~8 to ~3,
    // with each saved iteration eliminating one full [`div_rem_dispatch`]
    // call (the dominant cost).
    //
    // Hasselgren's trick — see Crandall & Pomerance 2005, "Prime Numbers:
    // A Computational Perspective" §9.2.1 — credits the f64-bootstrap idea
    // to T. Hasselgren in the GMP mailing list archives; the
    // implementation here is a from-first-principles limb-array variant.
    if bits >= 8 {
        // Extract top 64 bits of `n` as a u64, aligned so the leading 1
        // sits at position 63 (or as close as `n` allows).
        let shift = bits - 64.min(bits);
        let limb_idx = (shift / 64) as usize;
        let bit_off = shift % 64;
        let top_u64: u64 = if bit_off == 0 {
            n[limb_idx]
        } else {
            let lo = n[limb_idx] >> bit_off;
            let hi = if limb_idx + 1 < n.len() {
                n[limb_idx + 1].checked_shl(64 - bit_off).unwrap_or(0)
            } else {
                0
            };
            lo | hi
        };
        let seed_f64 = (top_u64 as f64).sqrt();
        let (seed_f64, half_shift) = if (shift & 1) == 1 {
            (seed_f64 * core::f64::consts::SQRT_2, (shift - 1) / 2)
        } else {
            (seed_f64, shift / 2)
        };
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        let seed_int: u128 = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        let seed_limb_idx = (half_shift / 64) as usize;
        let seed_bit_off = half_shift % 64;
        let shifted: u128 = seed_int << seed_bit_off;
        let seed_lo = shifted as u64;
        let seed_hi = (shifted >> 64) as u64;
        if seed_limb_idx < work {
            x[seed_limb_idx] |= seed_lo;
        }
        if seed_limb_idx + 1 < work {
            x[seed_limb_idx + 1] |= seed_hi;
        }
        if is_zero(&x[..work]) {
            x[0] = 1;
        }
    } else {
        // Tiny n: fall back to the classical 1-bit seed.
        let e = bits.div_ceil(2);
        x[(e / 64) as usize] |= 1u64 << (e % 64);
    }

    loop {
        let mut q = [0u64; SCRATCH_LIMBS];
        let mut r = [0u64; SCRATCH_LIMBS];
        div_rem_dispatch(n, &x[..work], &mut q[..work], &mut r[..work]);
        add_assign(&mut q[..work], &x[..work]);
        let mut y = [0u64; SCRATCH_LIMBS];
        shr(&q[..work], 1, &mut y[..work]);
        if cmp(&y[..work], &x[..work]) >= 0 {
            break;
        }
        x = y;
    }
    let copy_len = if out.len() < work { out.len() } else { work };
    out[..copy_len].copy_from_slice(&x[..copy_len]);
}
