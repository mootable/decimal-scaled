// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newton integer square root over little-endian `u64` limb slices.
//!
//! [`isqrt_newton`] is the width-agnostic Newton integer square root used
//! by the fixed-width fast-arm dispatch
//! [`crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed`] (`N >= 3`)
//! and by the decimal `sqrt` work-width path. Pure kernel — it takes the
//! operand and writes `floor(sqrt(radicand))`; no algorithm choice.

use crate::algo_x_support::seed::sqrt_seed;
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp, shr};
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

/// Scratch capacity for the Newton isqrt kernel — 288 u64 limbs
/// (18432 bits), covering the widest work integer in the crate
/// (Int<256> used by D1232 cbrt, 256 u64 limbs) with isqrt scratch slack.
use crate::int::algos::support::limbs::max_n_limbs;

const SCRATCH_LIMBS: usize = max_n_limbs(2);

/// `out = floor(sqrt(radicand))`. Newton iteration on top of the runtime
/// divide dispatcher.
///
/// Uses [`div_rem_dispatch`] (not the *const* `div_rem`) per iteration: the
/// const path routes multi-limb divisors through the O(bits²)
/// shift-subtract path, which at Int<64> (n=64 u64 limbs) dominates wall
/// time — Newton converges in ~log₂(b) ≈ 12 iterations, each one a
/// `~65k`-limb-op divmod. The dispatcher gets
/// Knuth-base-2⁶⁴ per iteration (~`~32²` = 1024 limb-ops), worth ~40× on
/// D307 sqrt.
pub(crate) fn isqrt_newton(radicand: &[u64], out: &mut [u64]) {
    for limb in out.iter_mut() {
        *limb = 0;
    }
    let bits = bit_len(radicand);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        out[0] = 1;
        return;
    }
    let work_len = radicand.len() + 1;
    debug_assert!(work_len <= SCRATCH_LIMBS, "isqrt scratch overflow");
    let mut x = [0u64; SCRATCH_LIMBS];

    // Initial guess — delegated to the cross-algorithm seed leaf
    // (`algo_x_support::seed`). Under `std` it bootstraps from the hardware
    // `f64::sqrt` of the top 64 bits of `radicand` (~53 correct bits in one
    // shot, dropping the Newton iteration count by ~half); under `no_std` it
    // uses the classical pure-integer 1-bit seed `2^ceil(bits/2)`. Both are
    // safe over-estimates, so this monotone-downward loop converges to the
    // same floor root either way. The leaf calls nothing in-crate (primitives
    // + std-gated inherent f64) — `num_traits::Float`/libm is never reached.
    sqrt_seed(radicand, bits, &mut x[..work_len]);

    // Newton working buffers hoisted OUT of the loop. The divide engine
    // re-zeros `q`/`r` each pass and `shr` re-zeros `y`, so only the live
    // `[..work_len]` slice is touched per iteration — no per-iteration
    // build-max memset (the previous in-loop `[0u64; SCRATCH_LIMBS]` allocs
    // were the wide-tier tax). `x = y` is likewise a `[..work_len]` copy, not
    // a full array.
    let mut q = [0u64; SCRATCH_LIMBS];
    let mut r = [0u64; SCRATCH_LIMBS];
    let mut y = [0u64; SCRATCH_LIMBS];
    loop {
        div_rem_dispatch(radicand, &x[..work_len], &mut q[..work_len], &mut r[..work_len]);
        add_assign(&mut q[..work_len], &x[..work_len]);
        shr(&q[..work_len], 1, &mut y[..work_len]);
        if cmp(&y[..work_len], &x[..work_len]) >= 0 {
            break;
        }
        x[..work_len].copy_from_slice(&y[..work_len]);
    }
    let copy_len = if out.len() < work_len { out.len() } else { work_len };
    out[..copy_len].copy_from_slice(&x[..copy_len]);
}
