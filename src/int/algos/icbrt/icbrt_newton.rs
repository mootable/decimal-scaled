// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Newton integer cube root over little-endian `u64` limb slices.
//!
//! [`icbrt_newton`] is the width-agnostic Brent–Zimmermann integer Newton
//! cube root used by the fixed-width fast-arm dispatch in
//! [`crate::int::policy::icbrt`] (`N >= 3`). Pure kernel — it takes the
//! operand and writes `floor(cbrt(n))`; no algorithm choice.

use crate::algo_x_support::seed::cbrt_seed;
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp};
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;

/// Scratch capacity for the Newton icbrt kernel — 288 u64 limbs
/// (18432 bits), covering the widest work integer in the crate
/// (Int<256> used by D1232 cbrt, 256 u64 limbs) with scratch slack.
const SCRATCH_LIMBS: usize = 288;

/// `out = floor(cbrt(n))`. Newton iteration for the integer cube root.
///
/// Implements the Brent–Zimmermann integer Newton iteration for cube root
/// (Modern Computer Arithmetic §1.5.2): starting from a safe over-estimate
/// of the root, each step applies
///
///   `s_new = (2·s + n / s²) / 3`
///
/// which converges monotonically downward to `floor(n^(1/3))`. Convergence
/// is quadratic once the error is small, so the total iteration count is
/// `O(log₂(bits))`.
///
/// The seed is delegated to the cross-algorithm seed leaf
/// ([`crate::algo_x_support::seed::cbrt_seed`]): under `std` it is derived
/// from the hardware `f64::cbrt` of the top 64 bits of `n`, scaled back and
/// rounded up to a safe over-estimate; under `no_std` it is the classical
/// pure-integer 1-bit seed `2^ceil(bits/3)`. No `libm` / `num_traits::Float`
/// is reached either way.
///
/// All arithmetic uses fixed-size `SCRATCH_LIMBS` scratch buffers —
/// no heap allocation, `core`/no_std-safe.
///
/// Hasselgren seed strategy: see Crandall & Pomerance 2005, "Prime Numbers:
/// A Computational Perspective" §9.2.1.
pub(crate) fn icbrt_newton(n: &[u64], out: &mut [u64]) {
    for o in out.iter_mut() {
        *o = 0;
    }
    let bits = bit_len(n);
    if bits == 0 {
        return;
    }
    if bits <= 1 {
        // n == 0 already handled; n == 1 → root is 1.
        out[0] = 1;
        return;
    }
    // The cube root has at most `ceil(bits / 3)` bits.
    // The intermediate s² has at most `2·ceil(bits/3)` bits.
    // We need scratch wide enough for s² — the same SCRATCH_LIMBS budget
    // as isqrt covers this (SCRATCH_LIMBS ≥ 288 limbs = 18 432 bits; the
    // widest shipped work-int is Int<256> = 16 384 bits).
    let work = n.len() + 1;
    debug_assert!(work <= SCRATCH_LIMBS, "icbrt scratch overflow");

    // ── seed ──────────────────────────────────────────────────────────
    // Delegated to the cross-algorithm seed leaf
    // (`algo_x_support::seed::cbrt_seed`): under `std` it is the hardware
    // `f64::cbrt` of the top 64 bits of `n` scaled back and rounded up to a
    // strict over-estimate; under `no_std` it is the classical pure-integer
    // 1-bit seed `2^ceil(bits/3)`. Both over-estimate, so the
    // monotone-downward Newton loop below converges to the same floor root.
    // The leaf calls nothing in-crate — `num_traits::Float`/libm is never
    // reached.
    let mut x = [0u64; SCRATCH_LIMBS];
    cbrt_seed(n, bits, &mut x[..work]);

    // ── Newton loop ───────────────────────────────────────────────────
    // Invariant: x ≥ floor(cbrt(n)) at entry of each iteration.
    // The iteration s_new = (2*s + n/s²) / 3 is monotone-non-increasing
    // and halts when s_new ≥ s (i.e. s is the floor root).
    loop {
        // t = s²  (2 * work limbs, but only work+1 matter)
        let sq_work = (work * 2).min(SCRATCH_LIMBS);
        let mut sq = [0u64; SCRATCH_LIMBS];
        mul_schoolbook(&x[..work], &x[..work], &mut sq[..sq_work]);

        // q = n / s²
        let mut q = [0u64; SCRATCH_LIMBS];
        let mut r = [0u64; SCRATCH_LIMBS];
        // Ensure the divisor has the right effective length (sq_work may
        // over-estimate — div_rem_dispatch handles leading zeros).
        div_rem_dispatch(n, &sq[..sq_work], &mut q[..work], &mut r[..sq_work]);

        // t = 2*s + q: add 2*x into q.
        // 2*s = s << 1: add s twice (no overflow into extra limbs because
        // the result fits in work+1 limbs by the cube-root bound).
        add_assign(&mut q[..work], &x[..work]);
        add_assign(&mut q[..work], &x[..work]);

        // y = t / 3
        let three = [3u64];
        let mut y = [0u64; SCRATCH_LIMBS];
        let mut rem3_buf = [0u64; SCRATCH_LIMBS];
        div_rem_dispatch(&q[..work], &three, &mut y[..work], &mut rem3_buf[..1]);

        if cmp(&y[..work], &x[..work]) >= 0 {
            break;
        }
        x = y;
    }
    let copy_len = if out.len() < work { out.len() } else { work };
    out[..copy_len].copy_from_slice(&x[..copy_len]);
}
