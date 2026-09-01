// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Division-free reciprocal-Newton integer cube root over little-endian
//! `u64` limb slices.
//!
//! **Candidate kernel — UNWIRED.** Bit-identical alternative to
//! [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`], written to
//! eliminate the per-iteration multi-precision divide that the shipped
//! Newton cube root pays. The cube-root policy
//! ([`crate::int::policy::icbrt`]) still routes every width to
//! `icbrt_newton`; this candidate is benched against it and wired
//! (per `(N)`) only where it wins. See the kept-alternatives note
//! in `docs/ARCHITECTURE.md`.
//!
//! # Why
//!
//! The shipped [`icbrt_newton`](crate::int::algos::icbrt::icbrt_newton)
//! runs the value-domain recurrence `s ← (2·s + n/s²)/3`, whose `n/s²` is a
//! **full multi-precision division routed through the Knuth divide engine,
//! once per iteration**. This kernel removes it: it converges with
//! **multiplications only**, in a fixed-point *reciprocal* domain, then
//! recovers and exactly corrects the integer floor.
//!
//! # Algorithm (cleanroom — from the published method, no GPL/LGPL source)
//!
//! Newton's method applied to `f(y) = y^{-3} − x` has the division-free
//! reciprocal-root iteration
//!
//! ```text
//!   y_{k+1} = y_k · (4 − x · y_k³) / 3
//! ```
//!
//! which converges quadratically to `y = x^{-1/3}` using only multiplies and
//! a divide-by-the-tiny-constant-3 (a single-word divide, *not* a
//! multi-precision one). Brent & Zimmermann, *Modern Computer Arithmetic*
//! (Cambridge UP, 2010), §2.4 / §4.2.3; the general `k`-th reciprocal-root
//! recurrence `y ← y·((k+1) − x·yᵏ)/k` is also in Knuth, *TAOCP* Vol. 2
//! §4.3.3.
//!
//! Worked in fixed point with scale `2^F` (`Y ≈ 2^F · x^{-1/3}`):
//!
//! ```text
//!   Y_{k+1} = Y · (4·2^{3F} − x · Y³) / (3 · 2^{3F})
//! ```
//!
//! the `2^{3F}` divisions are **bit-shifts** and the `/3` is a single-limb
//! divide. After convergence the cube-root estimate is recovered by
//!
//! ```text
//!   s ≈ (x · Y²) >> (2F)            (a shift, never a divide)
//! ```
//!
//! An **exact integer end-correction** (walk `s` down while `s³ > n`, then up
//! while `(s+1)³ ≤ n`, using only schoolbook multiplies) pins the result to
//! the exact floor `⌊∛n⌋` independently of the fixed-point rounding.
//!
//! ## Intermediate widths (the part the first draft got wrong)
//!
//! With `F = ⌈2·bits/3⌉ + GUARD`, the reciprocal `Y ≈ 2^{F−bits/3}` (≤ `F`
//! bits). The iteration's intermediates do NOT all fit in `~3F` bits as the
//! first draft assumed — the widest is `prod = Y · bracket` where `bracket ≈
//! 4·2^{3F}` (`≈3F` bits), so `prod ≈ 2^{4F−bits/3}` (**≈4F bits**). Each
//! product is therefore formed at its FULL width (`a.len()+b.len()` limbs)
//! via the schoolbook kernel (which requires `out.len() ≥ a.len()+b.len()`
//! and does NOT truncate), and the working scratch is budgeted for the `4F`
//! peak. The first draft passed `~3F`-limb outputs to the full multiply →
//! index-out-of-bounds at wide widths; this version sizes every product
//! exactly.
//!
//! # Properties
//!
//! - **No multi-precision division** in the hot loop — only multiplies,
//!   shifts, and one single-limb `/3`.
//! - **Generic over N** — width-agnostic `&[u64]` slices. No per-tier copy.
//! - **No dispatch re-entry** — multiplies call [`mul_schoolbook`] directly.
//! - **Exact:** result bit-identical to
//!   [`icbrt_newton`](crate::int::algos::icbrt::icbrt_newton::icbrt_newton)
//!   for every input (the `#[cfg(test)]` bit-identity sweep below is its
//!   validity wall).

use crate::algo_x_support::seed::cbrt_seed;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp, is_zero, shr, sub_assign};
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
use crate::int::types::compute_limbs::MAX_QUADRUPLE_LIMBS;

/// Scratch capacity for the reciprocal-Newton cube root. The widest
/// intermediate is `prod = Y·bracket ≈ 2^{4F−bits/3}` (≈`4F` bits); with
/// `F ≈ 2·bits/3`, that is `≈8·bits/3` bits ≈ `4·radicand.len()` limbs of
/// headroom over the operand. `2·MAX_QUADRUPLE_LIMBS + 8` u64 limbs covers
/// it for every shipped operand width. Local to this candidate; it sizes no
/// other tier. The wider footprint is the division-free trade's cost,
/// weighed at bench time.
const SCRATCH_LIMBS: usize = 2 * MAX_QUADRUPLE_LIMBS + 8;
/// Guard fractional bits carried by the reciprocal `Y` beyond the root
/// precision; ample for the end-correction to finish in O(1) steps.
const GUARD_BITS: u32 = 32;

/// `out = floor(cbrt(radicand))`, computed division-free via a
/// reciprocal-root Newton iteration plus an exact integer end-correction.
///
/// Bit-identical to
/// [`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`]; see the module
/// docs for the algorithm and references.
#[allow(dead_code)]
pub(crate) fn icbrt_newton_recip(radicand: &[u64], out: &mut [u64]) {
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

    let radicand_len = sig_len(radicand);
    let work_len = radicand_len + 1;

    // ── fixed-point scale + intermediate widths (see module docs) ─────────
    let f: u32 = (2 * bits).div_ceil(3) + GUARD_BITS;
    let f_usize = f as usize;
    let y_len = f_usize / 64 + 2; // Y < 2^F
    // ~3F-bit values: 2^{3F+2}, n·Y³, bracket
    let f3_len = (3 * f_usize) / 64 + 3;
    // ~4F-bit: prod = Y·bracket (the peak)
    let peak_prod_len = (4 * f_usize) / 64 + 4;
    debug_assert!(peak_prod_len <= SCRATCH_LIMBS, "icbrt_recip 4F scratch overflow");
    debug_assert!(radicand_len + f3_len <= SCRATCH_LIMBS,
        "icbrt_recip n·Y³ scratch overflow");

    // ── seed s0 ≥ ⌊∛n⌋ (shared cube-root seed) ────────────────────────────
    let mut s0 = [0u64; SCRATCH_LIMBS];
    cbrt_seed(radicand, bits, &mut s0[..work_len]);

    // ── Y0 = 2^F / s0  (the ONE divide, run ONCE) ─────────────────────────
    let mut two_f = [0u64; SCRATCH_LIMBS];
    two_f[f_usize / 64] = 1u64 << (f % 64);
    let dividend_len = (f_usize / 64 + 2).max(work_len);
    let mut y = [0u64; SCRATCH_LIMBS];
    {
        let mut remainder = [0u64; SCRATCH_LIMBS];
        div_rem_dispatch(&two_f[..dividend_len], &s0[..work_len],
            &mut y[..dividend_len], &mut remainder[..dividend_len]);
    }
    if is_zero(&y[..y_len]) {
        y[0] = 1; // never let the reciprocal collapse to zero
    }

    // ── reciprocal-root Newton (division-free) ────────────────────────────
    let three = [3u64];
    let iters = (32 - f.leading_zeros()) + 4; // ~ log2(F) + 4
    let mut y2 = [0u64; SCRATCH_LIMBS];
    let mut y3 = [0u64; SCRATCH_LIMBS];
    let mut ny3 = [0u64; SCRATCH_LIMBS];
    let mut bracket = [0u64; SCRATCH_LIMBS];
    let mut prod = [0u64; SCRATCH_LIMBS];
    let mut tmp = [0u64; SCRATCH_LIMBS];
    let mut new_y = [0u64; SCRATCH_LIMBS];
    let mut rem3 = [0u64; SCRATCH_LIMBS];
    let mut iter = 0;
    while iter < iters {
        let y_sig_len = sig_len(&y[..(y_len + 2).min(SCRATCH_LIMBS)]);

        // y2 = Y²  (full product width).
        let y2_len = 2 * y_sig_len;
        for limb in y2[..y2_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&y[..y_sig_len], &y[..y_sig_len], &mut y2[..y2_len]);
        let y2_sig_len = sig_len(&y2[..y2_len]);

        // y3 = y2·Y.
        let y3_len = y2_sig_len + y_sig_len;
        for limb in y3[..y3_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&y2[..y2_sig_len], &y[..y_sig_len], &mut y3[..y3_len]);
        let y3_sig_len = sig_len(&y3[..y3_len]);

        // ny3 = n·Y³ ≈ 2^{3F}.
        let ny3_len = radicand_len + y3_sig_len;
        for limb in ny3[..ny3_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&radicand[..radicand_len], &y3[..y3_sig_len], &mut ny3[..ny3_len]);
        let ny3_sig_len = sig_len(&ny3[..ny3_len]);

        // bracket = 4·2^{3F} − n·Y³  (4·2^{3F} = 2^{3F+2}).
        for limb in bracket[..f3_len].iter_mut() {
            *limb = 0;
        }
        {
            let pos = 3 * f_usize + 2;
            bracket[pos / 64] = 1u64 << (pos % 64);
        }
        if cmp(&bracket[..f3_len], &ny3[..ny3_sig_len]) < 0 {
            // over-shot: clamp the bracket to 0 (Y shrinks toward the fixed
            // point); keeps the unsigned subtraction well-defined.
            for limb in bracket[..f3_len].iter_mut() {
                *limb = 0;
            }
        } else {
            sub_assign(&mut bracket[..f3_len], &ny3[..ny3_sig_len]);
        }
        let bracket_sig_len = sig_len(&bracket[..f3_len]);

        // prod = Y·bracket  (the 4F-bit peak — full product width).
        let prod_len = y_sig_len + bracket_sig_len;
        for limb in prod[..prod_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&y[..y_sig_len], &bracket[..bracket_sig_len], &mut prod[..prod_len]);

        // new_y = prod / (3·2^{3F}) = (prod >> 3F) / 3.
        for limb in tmp[..prod_len].iter_mut() {
            *limb = 0;
        }
        shr(&prod[..prod_len], 3 * f, &mut tmp[..prod_len]);
        let tmp_sig_len = sig_len(&tmp[..prod_len]).max(1);
        for limb in new_y[..tmp_sig_len].iter_mut() {
            *limb = 0;
        }
        rem3[0] = 0;
        div_rem_dispatch(&tmp[..tmp_sig_len], &three, &mut new_y[..tmp_sig_len],
            &mut rem3[..1]);

        // Fixed-point reached → stop early (the end-correction does the rest).
        let cmp_len = tmp_sig_len.max(y_len);
        if cmp(&new_y[..cmp_len.min(SCRATCH_LIMBS)], &y[..cmp_len.min(SCRATCH_LIMBS)]) == 0 {
            iter = iters;
        } else {
            iter += 1;
        }
        let copy_len = tmp_sig_len.min(SCRATCH_LIMBS);
        for limb in y[..(y_len + 2).min(SCRATCH_LIMBS)].iter_mut() {
            *limb = 0;
        }
        y[..copy_len].copy_from_slice(&new_y[..copy_len]);
    }

    // ── recover s ≈ (n · Y²) >> 2F ────────────────────────────────────────
    let y_sig_len = sig_len(&y[..(y_len + 2).min(SCRATCH_LIMBS)]);
    let y2_len = 2 * y_sig_len;
    for limb in y2[..y2_len].iter_mut() {
        *limb = 0;
    }
    mul_schoolbook(&y[..y_sig_len], &y[..y_sig_len], &mut y2[..y2_len]);
    let y2_sig_len = sig_len(&y2[..y2_len]);
    let n_y2_len = radicand_len + y2_sig_len;
    for limb in prod[..n_y2_len].iter_mut() {
        *limb = 0;
    }
    mul_schoolbook(&radicand[..radicand_len], &y2[..y2_sig_len], &mut prod[..n_y2_len]);
    let mut s = [0u64; SCRATCH_LIMBS];
    shr(&prod[..n_y2_len], 2 * f, &mut s[..work_len]);

    // If the reciprocal collapsed (s == 0) fall back to the seed over-estimate.
    if is_zero(&s[..work_len]) {
        s[..work_len].copy_from_slice(&s0[..work_len]);
    }

    // ── exact integer end-correction (multiplies only) ────────────────────
    let one = [1u64];
    let mut sq = [0u64; SCRATCH_LIMBS];
    let mut cube = [0u64; SCRATCH_LIMBS];
    let mut s_plus_one = [0u64; SCRATCH_LIMBS];

    // Walk DOWN while s³ > n.
    loop {
        let s_len = sig_len(&s[..work_len]);
        let sq_len = (2 * s_len).min(SCRATCH_LIMBS);
        for limb in sq[..sq_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&s[..s_len], &s[..s_len], &mut sq[..sq_len]);
        let sq_sig_len = sig_len(&sq[..sq_len]);
        let cube_len = (sq_sig_len + s_len).min(SCRATCH_LIMBS);
        for limb in cube[..cube_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&sq[..sq_sig_len], &s[..s_len], &mut cube[..cube_len]);
        if cmp(&cube[..cube_len], radicand) > 0 {
            sub_assign(&mut s[..work_len], &one);
        } else {
            break;
        }
    }

    // Walk UP while (s+1)³ ≤ n.
    loop {
        s_plus_one[..work_len].copy_from_slice(&s[..work_len]);
        add_assign(&mut s_plus_one[..work_len], &one);
        let s_len = sig_len(&s_plus_one[..work_len]);
        let sq_len = (2 * s_len).min(SCRATCH_LIMBS);
        for limb in sq[..sq_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&s_plus_one[..s_len], &s_plus_one[..s_len], &mut sq[..sq_len]);
        let sq_sig_len = sig_len(&sq[..sq_len]);
        let cube_len = (sq_sig_len + s_len).min(SCRATCH_LIMBS);
        for limb in cube[..cube_len].iter_mut() {
            *limb = 0;
        }
        mul_schoolbook(&sq[..sq_sig_len], &s_plus_one[..s_len], &mut cube[..cube_len]);
        if cmp(&cube[..cube_len], radicand) <= 0 {
            s[..work_len].copy_from_slice(&s_plus_one[..work_len]);
        } else {
            break;
        }
    }

    let copy_len = out.len().min(work_len);
    out[..copy_len].copy_from_slice(&s[..copy_len]);
}

/// Significant limb count of `limbs` (highest non-zero index + 1), minimum 1.
#[inline]
fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 0 {
        if limbs[len - 1] != 0 {
            return len;
        }
        len -= 1;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::icbrt_newton_recip;
    use crate::int::algos::icbrt::icbrt_newton::icbrt_newton;

    fn recip(radicand: &[u64], limbs: usize) -> Vec<u64> {
        let mut out = vec![0u64; limbs];
        icbrt_newton_recip(radicand, &mut out);
        out
    }
    fn newton(radicand: &[u64], limbs: usize) -> Vec<u64> {
        let mut out = vec![0u64; limbs];
        icbrt_newton(radicand, &mut out);
        out
    }
    fn recip_u64(radicand: u64) -> u64 {
        recip(&[radicand], 1)[0]
    }
    fn newton_u64(radicand: u64) -> u64 {
        newton(&[radicand], 1)[0]
    }
    fn recip_u128(radicand: u128) -> u128 {
        let root = recip(&[radicand as u64, (radicand >> 64) as u64], 2);
        (root[0] as u128) | ((root[1] as u128) << 64)
    }
    fn newton_u128(radicand: u128) -> u128 {
        let root = newton(&[radicand as u64, (radicand >> 64) as u64], 2);
        (root[0] as u128) | ((root[1] as u128) << 64)
    }

    #[test]
    fn recip_known_values_u64() {
        let cases: &[(u64, u64)] = &[
            (0, 0),
            (1, 1),
            (2, 1),
            (7, 1),
            (8, 2),
            (9, 2),
            (26, 2),
            (27, 3),
            (28, 3),
            (63, 3),
            (64, 4),
            (124, 4),
            (125, 5),
            (126, 5),
            (999, 9),
            (1_000, 10),
            (1_001, 10),
            (2_u64.pow(63), 2_097_152),
            (u64::MAX, 2_642_245),
        ];
        for &(n, expected) in cases {
            assert_eq!(recip_u64(n), expected, "icbrt_recip({n})");
        }
    }

    #[test]
    fn recip_matches_newton_u64_dense() {
        for n in 0u64..=4096 {
            assert_eq!(recip_u64(n), newton_u64(n), "dense mismatch n={n}");
        }
    }

    #[test]
    fn recip_matches_newton_u64_perfect_cubes_and_edges() {
        let mut k: u64 = 1;
        while let Some(cube) = k.checked_mul(k).and_then(|s| s.checked_mul(k)) {
            for &n in &[cube.wrapping_sub(1), cube, cube + 1] {
                assert_eq!(recip_u64(n), newton_u64(n), "edge mismatch n={n} (k={k})");
            }
            k += 1;
            if k > 2_700_000 {
                break;
            }
            if k > 4096 {
                k += 9_999;
            }
        }
        for &n in &[u64::MAX, u64::MAX - 1, 1u64 << 32, 1u64 << 62, (1u64 << 21).pow(1)] {
            assert_eq!(recip_u64(n), newton_u64(n), "boundary mismatch n={n}");
        }
    }

    #[test]
    fn recip_matches_newton_u128() {
        for n in 0u128..=512 {
            assert_eq!(recip_u128(n), newton_u128(n), "u128 dense mismatch n={n}");
        }
        for k in [
            2u128, 3, 5, 10, 100, 1_000, 10_000, 100_000, 1_000_000,
            1_000_000_000, 1_000_000_000_000u128, 5_541_191_377_756u128,
        ] {
            let cube = k * k * k;
            for &n in &[cube - 1, cube, cube + 1] {
                assert_eq!(recip_u128(n), newton_u128(n), "u128 cube-edge mismatch n={n}");
            }
        }
        for &n in &[
            u128::MAX,
            u128::MAX - 1,
            1u128 << 64,
            1u128 << 96,
            1u128 << 127,
            (2u128.pow(64)) + 1,
        ] {
            assert_eq!(recip_u128(n), newton_u128(n), "u128 boundary mismatch n={n}");
        }
    }

    // Multi-limb operands carry the wide-tier scratch budget
    // (`SCRATCH_LIMBS` is build-max-sized); the narrow default build sizes it
    // for the ≤2-limb tiers where this kernel is never engaged. Gate the wide
    // sweep to a wide build accordingly.
    #[cfg(feature = "wide")]
    #[test]
    fn recip_matches_newton_wide_widths() {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for &limbs in &[3usize, 4, 6, 8, 16] {
            for _ in 0..40 {
                let mut n = vec![0u64; limbs];
                let fill_len = 1 + (next() as usize % limbs);
                for limb in n.iter_mut().take(fill_len) {
                    *limb = next();
                }
                if n[fill_len - 1] == 0 {
                    n[fill_len - 1] = 1;
                }
                assert_eq!(
                    recip(&n, limbs),
                    newton(&n, limbs),
                    "wide mismatch limbs={limbs} n={n:?}"
                );
            }
            for _ in 0..10 {
                let mut base = vec![0u64; limbs];
                let base_fill_len = 1 + (next() as usize % limbs.div_ceil(3).max(1));
                for limb in base.iter_mut().take(base_fill_len) {
                    *limb = next();
                }
                if base[base_fill_len - 1] == 0 {
                    base[base_fill_len - 1] = 1;
                }
                let mut sq = vec![0u64; limbs * 3];
                crate::int::algos::mul::mul_schoolbook::mul_schoolbook(&base, &base, &mut sq);
                let mut cube = vec![0u64; limbs * 3];
                crate::int::algos::mul::mul_schoolbook::mul_schoolbook(&sq, &base, &mut cube);
                let mut n = vec![0u64; limbs];
                n.copy_from_slice(&cube[..limbs]);
                assert_eq!(
                    recip(&n, limbs),
                    newton(&n, limbs),
                    "wide cube mismatch limbs={limbs}"
                );
            }
        }
    }
}
