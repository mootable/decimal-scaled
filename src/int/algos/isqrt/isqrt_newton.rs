// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newton integer square root over little-endian `u64` limb slices.
//!
//! [`isqrt_newton`] is the width-agnostic Newton integer square root used
//! by the fixed-width fast-arm dispatch
//! [`crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed`] (`N >= 3`)
//! and by the decimal `sqrt` work-width path. Pure kernel — it takes the
//! operand and writes `floor(sqrt(n))`; no algorithm choice.

use crate::algo_x_support::seed::sqrt_seed;
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp, shr};
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

/// Scratch capacity for the Newton isqrt kernel — 288 u64 limbs
/// (18432 bits), covering the widest work integer in the crate
/// (Int<256> used by D1232 cbrt, 256 u64 limbs) with isqrt scratch slack.
use crate::int::algos::support::limbs::max_n_limbs;

const SCRATCH_LIMBS: usize = max_n_limbs(2);

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

    // Initial guess — delegated to the cross-algorithm seed leaf
    // (`algo_x_support::seed`). Under `std` it bootstraps from the hardware
    // `f64::sqrt` of the top 64 bits of `n` (~53 correct bits in one shot,
    // dropping the Newton iteration count by ~half); under `no_std` it uses
    // the classical pure-integer 1-bit seed `2^ceil(bits/2)`. Both are safe
    // over-estimates, so this monotone-downward loop converges to the same
    // floor root either way. The leaf calls nothing in-crate (primitives +
    // std-gated inherent f64) — `num_traits::Float`/libm is never reached.
    sqrt_seed(n, bits, &mut x[..work]);

    // Newton working buffers hoisted OUT of the loop. The divide engine
    // re-zeros `q`/`r` each pass and `shr` re-zeros `y`, so only the live
    // `[..work]` slice is touched per iteration — no per-iteration build-max
    // memset (the previous in-loop `[0u64; SCRATCH_LIMBS]` allocs were the
    // wide-tier tax). `x = y` is likewise a `[..work]` copy, not a full array.
    let mut q = [0u64; SCRATCH_LIMBS];
    let mut r = [0u64; SCRATCH_LIMBS];
    let mut y = [0u64; SCRATCH_LIMBS];
    loop {
        div_rem_dispatch(n, &x[..work], &mut q[..work], &mut r[..work]);
        add_assign(&mut q[..work], &x[..work]);
        shr(&q[..work], 1, &mut y[..work]);
        if cmp(&y[..work], &x[..work]) >= 0 {
            break;
        }
        x[..work].copy_from_slice(&y[..work]);
    }
    let copy_len = if out.len() < work { out.len() } else { work };
    out[..copy_len].copy_from_slice(&x[..copy_len]);
}
