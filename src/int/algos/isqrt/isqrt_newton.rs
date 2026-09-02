// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newton integer square root over little-endian `u64` limb slices.
//!
//! Two doors onto one kernel. [`isqrt_newton_into`] is the implementation and
//! takes the caller's scratch; [`isqrt_newton`] is the width-agnostic wrapper
//! that sizes that scratch from the build-max, for callers with no `N` — the
//! fixed-width fast-arm dispatch
//! [`crate::int::algos::isqrt::isqrt_mag_fixed::isqrt_mag_fixed`] (`N >= 3`),
//! `hypot`, `isqrt_karatsuba` and the bench seam. The decimal `sqrt`
//! work-width path holds a concrete `N` and goes through the `_into` door.
//! Pure kernel either way — it takes the operand and writes
//! `floor(sqrt(radicand))`; no algorithm choice.

use crate::algo_x_support::seed::sqrt_seed;
use crate::int::algos::div::div_rem_into::div_rem_into;
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp, shr};

/// Scratch capacity for the width-agnostic [`isqrt_newton`] door — the
/// build-max `2·MAX_WORK_N + ⌈MAX_WORK_N/2⌉`, the only sizing available to a
/// caller with no `N`. Callers that DO hold a concrete `N` must use
/// [`isqrt_newton_into`] instead: this constant is selected by the build's
/// WIDTH FEATURES (40 limbs at `wide`, 160 at `xx-wide`), so a kernel sized
/// from it makes every narrow call pay for the widest enabled tier — the
/// feature-coupling the exact-scratch mechanism exists to remove.
use crate::int::algos::support::limbs::max_n_limbs;
use crate::int::types::compute_limbs::MAX_SINGLE_LIMBS;

const SCRATCH_LIMBS: usize = max_n_limbs(2);

/// Build-max Knuth normalisation scratch for [`isqrt_newton`] — the same
/// `MAX_SINGLE_LIMBS` budget [`div_knuth`](crate::int::algos::div::div_knuth)
/// allocates at its own blanket door, hoisted here so it is zeroed ONCE per
/// root instead of once per Newton iteration.
const DIV_SCRATCH_LIMBS: usize = MAX_SINGLE_LIMBS;

/// Build-max base-2¹²⁸ packed scratch — `⌈MAX_SINGLE_LIMBS/2⌉ + 2`, matching
/// `div_knuth_u128_limb`'s own blanket sizing (the one-above window limb plus
/// an even-rounding limb).
const DIV_SCRATCH_LIMBS_128: usize = MAX_SINGLE_LIMBS / 2 + 2;

/// `out = floor(sqrt(radicand))` — the **build-max** door, for callers with no
/// `N` to size scratch from (the `Uint<N>` fast-arm dispatch, `hypot`, the
/// bench seam). Allocates the three Newton working buffers plus the divide's
/// normalisation scratch at the build-max width and delegates to
/// [`isqrt_newton_into`]. Hoisting the divide's scratch up here is a win even
/// on this path: it is now zeroed once per root rather than once per Newton
/// iteration, which is where `div_knuth`'s blanket door was zeroing it.
///
/// A caller holding a concrete `N` (`Limbs<N>: ComputeLimbs` — the decimal
/// `sqrt` kernel) calls [`isqrt_newton_into`] with its own exactly-sized
/// buffers instead, skipping the build-max zeroing entirely.
pub(crate) fn isqrt_newton(radicand: &[u64], out: &mut [u64]) {
    debug_assert!(radicand.len() + 1 <= SCRATCH_LIMBS, "isqrt scratch overflow");
    let mut x = [0u64; SCRATCH_LIMBS];
    let mut q = [0u64; SCRATCH_LIMBS];
    let mut y = [0u64; SCRATCH_LIMBS];
    let mut u = [0u64; DIV_SCRATCH_LIMBS];
    let mut v = [0u64; DIV_SCRATCH_LIMBS];
    let mut u128_u = [0u128; DIV_SCRATCH_LIMBS_128];
    let mut u128_v = [0u128; DIV_SCRATCH_LIMBS_128];
    isqrt_newton_into(
        radicand, out, &mut x, &mut q, &mut y, &mut u, &mut v, &mut u128_u, &mut u128_v,
    );
}

/// `out = floor(sqrt(radicand))` in **caller-provided scratch** — the real
/// implementation, and the exact-scratch sibling of [`isqrt_newton`].
///
/// Newton's downward-monotone recurrence `x ← (x + radicand/x) / 2` from a
/// guaranteed over-estimate seed, one divide per iteration. The divide is the
/// hot term, so it goes through the divide matcher's verdict
/// ([`select_for_limbs`]) into the chosen engine's `_into` door with the
/// caller's buffers — rather than the build-max
/// [`dispatch`](crate::int::policy::div_rem::dispatch), whose `div_knuth`
/// wrapper allocates and zeroes two `MAX_SINGLE_LIMBS` arrays **per
/// iteration** (2 × 258 limbs at `xx-wide`, 2 × 66 at `wide` — for the same
/// work). Every verdict is honored with its own engine: the `radicand / x`
/// shape has `den_n ≈ num_m / 2`, so the wide even-divisor `num_m ≥ 2·den_n`
/// u128-limb arm is genuinely reachable from D462 up and must not be
/// collapsed onto base-2⁶⁴ Knuth.
///
/// Required scratch lengths, with `L = radicand.len()` and `W = L + 1`:
///
/// | buffer | minimum | role |
/// |---|---|---|
/// | `x`, `q`, `y` | `W` | estimate, quotient, next estimate |
/// | `u` | `L + 2` | Knuth normalised dividend (reads one limb above the live dividend) |
/// | `v` | `W` | Knuth normalised divisor |
/// | `u128_u` | `⌈(L + 2) / 2⌉ + 1` | base-2¹²⁸ packed dividend |
/// | `u128_v` | `⌈W / 2⌉` | base-2¹²⁸ packed divisor |
///
/// The buffers may be **dirty** on entry and are reusable across calls: `x` is
/// zeroed here before the seed ORs into it, `shr` re-zeros `y`, and both
/// divide engines re-zero their own outputs and normalisation scratch. Only
/// the live `[..W]` prefixes are touched, so the cost tracks the operand, not
/// the build.
#[allow(clippy::too_many_arguments)]
pub(crate) fn isqrt_newton_into(
    radicand: &[u64],
    out: &mut [u64],
    x: &mut [u64],
    q: &mut [u64],
    y: &mut [u64],
    u: &mut [u64],
    v: &mut [u64],
    u128_u: &mut [u128],
    u128_v: &mut [u128],
) {
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

    // Initial guess — delegated to the cross-algorithm seed leaf
    // (`algo_x_support::seed`). Under `std` it bootstraps from the hardware
    // `f64::sqrt` of the top 64 bits of `radicand` (~53 correct bits in one
    // shot, dropping the Newton iteration count by ~half); under `no_std` it
    // uses the classical pure-integer 1-bit seed `2^ceil(bits/2)`. Both are
    // safe over-estimates, so this monotone-downward loop converges to the
    // same floor root either way. The leaf calls nothing in-crate (primitives
    // + std-gated inherent f64) — `num_traits::Float`/libm is never reached.
    //
    // The seed ORs into its destination, so the live prefix is zeroed first
    // (the caller's buffer may carry a previous call's limbs).
    for limb in x[..work_len].iter_mut() {
        *limb = 0;
    }
    sqrt_seed(radicand, bits, &mut x[..work_len]);

    // Newton loop. `y` doubles as the divide's remainder sink — the remainder
    // is never read here, and `shr` fully re-zeros `y` from `q` immediately
    // after, so one buffer covers both roles. Per pass only the live
    // `[..work_len]` slices are touched; `x = y` is likewise a prefix copy.
    loop {
        div_rem_into(radicand, &x[..work_len], &mut q[..work_len], &mut y[..work_len],
            u, v, u128_u, u128_v);
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
