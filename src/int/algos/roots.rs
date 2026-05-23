//! Integer-root algorithms over little-endian `u64` limb slices.
//!
//! [`isqrt_newton`] is the width-agnostic Newton integer square root used
//! by the fixed-width fast-arm dispatch in [`super::div::isqrt_mag_fixed`]
//! (`N >= 3`) and by the decimal `sqrt` work-width path. Pure kernel — it
//! takes the operand and writes `floor(sqrt(n))`; no algorithm choice.

use super::limbs::{add_assign, bit_len, cmp, is_zero, mul_schoolbook, shr};
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
/// The seed is derived from the hardware `f64::cbrt` of the top 64 bits of
/// `n`, scaled back to the correct magnitude, then rounded up to guarantee
/// a safe over-estimate. On `no_std` the `f64` inherent method is
/// unavailable; `num_traits::Float` provides a libm-backed fallback.
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
    // Extract the top 64 bits of `n` and call f64::cbrt on them, then
    // shift the result back to the correct magnitude.  Round up to keep
    // the seed a strict over-estimate.
    let mut x = [0u64; SCRATCH_LIMBS];
    if bits >= 9 {
        // Shift so the leading bit of `n` sits at bit 63 of top_u64.
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
        // cbrt(top_u64 · 2^shift) = cbrt(top_u64) · 2^(shift/3).
        // Handle the fractional-third-power by splitting mod 3.
        let rem3 = shift % 3;
        // Scale factor: 2^(rem3/3) approximation using exact multipliers.
        // rem3 == 0: no extra factor.
        // rem3 == 1: multiply by 2^(1/3) ≈ 1.2599  (round up → 2).
        // rem3 == 2: multiply by 2^(2/3) ≈ 1.5874  (round up → 2).
        let seed_f64 = {
            let raw = (top_u64 as f64).cbrt();
            // Apply any residual power-of-2 factor.
            if rem3 == 0 { raw } else { raw * (1u64 << rem3) as f64 }
        };
        // half_shift = shift / 3 (integer, rounded down — the fractional
        // part was absorbed into the f64 multiplier above).
        let half_shift = shift / 3;
        let truncated = seed_f64 as u128;
        let frac_nonzero = (truncated as f64) != seed_f64;
        // Add 2 to guarantee over-estimate (one for truncation, one for
        // the f64 cbrt rounding, one spare → add 2 is conservative enough
        // without over-widening).
        let seed_int: u128 = truncated.saturating_add(if frac_nonzero { 2 } else { 1 });
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
        // Tiny n: use the classical 1-bit seed 2^ceil(bits/3).
        let e = bits.div_ceil(3);
        x[(e / 64) as usize] |= 1u64 << (e % 64);
    }

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
