// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Karatsuba Square Root over little-endian `u64` limb slices.
//!
//! **Candidate kernel — UNWIRED.** Bit-identical alternative to
//! [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`], written to
//! replace Newton's *full-width* per-iteration division with a recursion
//! whose divide is **half-width** and runs only `O(log n)` times. The
//! square-root policy still routes via the shipped Newton kernel; the
//! coordinator benches this candidate against it and wires it (per `(N)`)
//! only where it wins. See `docs/ARCHITECTURE.md` → "Keeping the
//! alternatives".
//!
//! # Why
//!
//! The shipped [`isqrt_newton`](crate::int::algos::isqrt::isqrt_newton)
//! runs the Heron recurrence `x ← (x + n/x)/2`, whose `n/x` is a **full
//! multi-precision division** routed through the Knuth engine once per
//! iteration (~`log₂(bits)` iterations). Division is the most expensive limb
//! primitive. This kernel does instead `O(log n)` *half-width* divisions —
//! one per recursion level — which is the source of the open `sqrt_D57
//! ~1.3×` headroom.
//!
//! # Algorithm (cleanroom — from the paper, no GPL/LGPL source)
//!
//! Paul Zimmermann, *Karatsuba Square Root*, INRIA Research Report RR-3805
//! (1999), Algorithm 1 (`SqrtRem`). Given `n` in base `B`, written
//! `n = a₃·B³ + a₂·B² + a₁·B + a₀` with the top block normalized
//! (`a₃ ≥ B/4`, i.e. the two most-significant bits of `n` set):
//!
//! ```text
//!   (s', r')  = SqrtRem(a₃·B + a₂)          # recurse on the high half
//!   (q,  u )  = DivRem(r'·B + a₁, 2·s')      # the ONE (half-width) divide
//!   s  = s'·B + q
//!   r  = u·B + a₀ − q²
//!   if r < 0:  r += 2·s − 1;  s −= 1         # at most one correction
//!   return (s, r)                            # s = ⌊√n⌋, r = n − s²
//! ```
//!
//! Correctness (paper, Theorem 1): the returned `s = ⌊√n⌋` exactly, with
//! remainder `r = n − s²` and `0 ≤ r ≤ 2s`. The normalization guarantee that
//! the input's top two bits are set is what bounds the quotient `q < B` and
//! limits the post-correction to a single step. Here the recursion bottoms
//! out (below a small limb threshold) on the shipped exact Newton kernel —
//! the recursion only has to *reduce the width*, so the base case can be any
//! exact `⌊√·⌋`; its remainder is recovered as `n − s²`.
//!
//! This implementation works with a base `B = 2^(64·h)` (`h` a half-limb
//! count), normalizes by an **even** left shift `2·sh` (so
//! `√(n·4^{sh}) = √n · 2^{sh}` recovers the root by a `>> sh`), and is
//! width-agnostic.
//!
//! # Properties
//!
//! - **One half-width divide per recursion level** (`O(log n)` total) — vs
//!   Newton's full-width divide per iteration.
//! - **Generic over N** — width-agnostic `&[u64]` slices; no per-tier copy.
//! - **No dispatch re-entry** — products via [`mul_schoolbook`] directly,
//!   the half-width divide via [`div_rem_dispatch`]; the base case calls the
//!   sibling [`isqrt_newton`] kernel directly, never a re-dispatched method.
//! - **Exact:** result bit-identical to
//!   [`isqrt_newton`](crate::int::algos::isqrt::isqrt_newton::isqrt_newton)
//!   for every input (the `#[cfg(test)]` bit-identity sweep below is its
//!   validity wall).

use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{
    add_assign, bit_len, cmp, shl, shr, sub_assign,
};
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
use crate::int::types::compute_int::MAX_DOUBLE_LIMBS;

/// Scratch capacity — the double-N budget shared with the shipped Newton
/// `isqrt` (radicand ≤ 2N). Sized like the sibling kernel's `SCRATCH_LIMBS`.
const SCRATCH_LIMBS: usize = MAX_DOUBLE_LIMBS;

/// Below this many *significant* limbs, the recursion bottoms out on the
/// shipped exact Newton kernel rather than splitting further. Keeps the
/// recursion shallow and lets the (already-fast, f64-seeded) Newton kernel
/// own the small-width regime where it wins.
const BASE_LIMBS: usize = 2;

/// `out = floor(sqrt(n))`, computed by the Karatsuba Square Root recursion.
///
/// Bit-identical to
/// [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`]; see the module
/// docs for the algorithm and the RR-3805 reference.
#[allow(dead_code)]
pub(crate) fn isqrt_karatsuba(n: &[u64], out: &mut [u64]) {
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

    // ── normalize: even left shift so the top two bits of the top limb are
    // set. Shifting by an even amount `2·sh` scales n by 4^sh, so
    // √(n·4^sh) = √n · 2^sh and the root is recovered by `>> sh`. ──────────
    let sig = sig_len(n);
    let top = n[sig - 1];
    let lz = top.leading_zeros(); // 0..=63
    // We want the top limb's top two bits set: shift left by `lz` aligns the
    // leading 1 to bit 63; one more bit may be needed but `lz` (rounded to
    // even, and ensuring ≥ 2 leading bits) suffices for the paper's bound.
    // Use the largest even shift ≤ lz that leaves the top two bits set:
    // shifting by `lz` sets the top bit; the paper needs the top *two* bits,
    // so we shift by `lz & !1` when that keeps ≥1 spare, else handle via the
    // recursion's own correction. To stay safe + exact we normalize to the
    // top bit (shift = lz rounded DOWN to even) and rely on `sqrtrem`'s
    // internal guards (it does not actually require the 2nd bit for
    // correctness — only for the tighter single-correction bound, and we
    // loop the correction).
    let sh_bits = lz & !1u32; // even
    let norm_limbs = (sig + 2).min(SCRATCH_LIMBS); // room for the shift
    let mut nn = [0u64; SCRATCH_LIMBS];
    shl(&n[..sig], sh_bits, &mut nn[..norm_limbs]);
    let nn_len = sig_len(&nn[..norm_limbs]);

    // ── recurse ───────────────────────────────────────────────────────────
    let mut s = [0u64; SCRATCH_LIMBS];
    let mut r = [0u64; SCRATCH_LIMBS];
    sqrtrem(&nn[..nn_len], &mut s, &mut r);

    // ── de-normalize the root: s_real = s >> (sh_bits/2) ───────────────────
    let mut s_out = [0u64; SCRATCH_LIMBS];
    let s_len = sig_len(&s[..SCRATCH_LIMBS]).max(1);
    shr(&s[..s_len], sh_bits / 2, &mut s_out[..s_len]);

    let copy_len = out.len().min(s_len);
    out[..copy_len].copy_from_slice(&s_out[..copy_len]);
}

/// Significant limb count of `a` (index of the highest non-zero limb + 1),
/// minimum 1.
#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut i = a.len();
    while i > 0 {
        if a[i - 1] != 0 {
            return i;
        }
        i -= 1;
    }
    1
}

/// `(s, r) = SqrtRem(n)`: `s = ⌊√n⌋`, `r = n − s²` with `0 ≤ r ≤ 2s`.
/// `n` must be non-zero with its top limb non-zero. `s`/`r` are zeroed,
/// then written.
///
/// Recursive Karatsuba Square Root (RR-3805 Algorithm 1). Base case: the
/// shipped exact Newton kernel, with the remainder recovered as `n − s²`.
fn sqrtrem(n: &[u64], s: &mut [u64], r: &mut [u64]) {
    for v in s.iter_mut() {
        *v = 0;
    }
    for v in r.iter_mut() {
        *v = 0;
    }
    let len = sig_len(n);

    // ── base case: exact Newton root + remainder n − s² ─────────────────
    if len <= BASE_LIMBS {
        isqrt_newton(&n[..len], &mut s[..len]);
        let s_len = sig_len(&s[..len]);
        // r = n − s²
        let mut sq = [0u64; SCRATCH_LIMBS];
        let sq_len = (2 * s_len).min(SCRATCH_LIMBS);
        mul_schoolbook(&s[..s_len], &s[..s_len], &mut sq[..sq_len]);
        // r = n − sq  (n ≥ sq by definition of floor root)
        let rl = len.max(sq_len);
        r[..len].copy_from_slice(&n[..len]);
        sub_assign(&mut r[..rl], &sq[..sq_len.min(rl)]);
        return;
    }

    // ── split n into four blocks of `h` limbs: n = a3·B³+a2·B²+a1·B+a0 ───
    // B = 2^(64·h). Choose h = ceil(len/4) so the high half (a3·B+a2) has
    // ≤ 2h limbs and is strictly narrower than n → the recursion shrinks.
    let h = len.div_ceil(4);
    let block = |idx: usize| -> &[u64] {
        let lo = idx * h;
        if lo >= len {
            &n[len..len] // empty
        } else {
            let hi = (lo + h).min(len);
            &n[lo..hi]
        }
    };
    let a0 = block(0);
    let a1 = block(1);
    let a3a2_lo = 2 * h; // start of the high half (a2 then a3)
    let high = if a3a2_lo >= len { &n[len..len] } else { &n[a3a2_lo..len] };

    // ── (s', r') = SqrtRem(high) ─────────────────────────────────────────
    let mut sp = [0u64; SCRATCH_LIMBS];
    let mut rp = [0u64; SCRATCH_LIMBS];
    sqrtrem(high, &mut sp, &mut rp);
    let sp_len = sig_len(&sp[..SCRATCH_LIMBS]);
    let rp_len = sig_len(&rp[..SCRATCH_LIMBS]);

    // ── (q, u) = DivRem(r'·B + a1, 2·s') ─────────────────────────────────
    // numerator = r'·B + a1  (r' shifted up by h limbs, a1 in the low h)
    let mut num = [0u64; SCRATCH_LIMBS];
    let num_len = (rp_len + h + 1).min(SCRATCH_LIMBS);
    // place r' at limb offset h
    for (i, &v) in rp[..rp_len].iter().enumerate() {
        if h + i < SCRATCH_LIMBS {
            num[h + i] = v;
        }
    }
    // add a1 into the low h limbs
    add_assign(&mut num[..num_len], a1);

    // divisor = 2·s'
    let mut den = [0u64; SCRATCH_LIMBS];
    let den_len = (sp_len + 1).min(SCRATCH_LIMBS);
    shl(&sp[..sp_len], 1, &mut den[..den_len]);
    let den_sig = sig_len(&den[..den_len]);

    let mut q = [0u64; SCRATCH_LIMBS];
    let mut u = [0u64; SCRATCH_LIMBS];
    let qrlen = num_len.max(den_sig);
    div_rem_dispatch(&num[..num_len], &den[..den_sig], &mut q[..qrlen], &mut u[..qrlen]);
    let q_len = sig_len(&q[..qrlen]);
    let u_len = sig_len(&u[..qrlen]);

    // ── s = s'·B + q ─────────────────────────────────────────────────────
    // s' at limb offset h, q in the low limbs.
    for (i, &v) in sp[..sp_len].iter().enumerate() {
        if h + i < SCRATCH_LIMBS {
            s[h + i] = v;
        }
    }
    add_assign(s, &q[..q_len]);

    // ── r = u·B + a0 − q² ────────────────────────────────────────────────
    // ub_a0 = u·B + a0
    let mut rr = [0u64; SCRATCH_LIMBS];
    for (i, &v) in u[..u_len].iter().enumerate() {
        if h + i < SCRATCH_LIMBS {
            rr[h + i] = v;
        }
    }
    add_assign(&mut rr, a0);
    // q² (subtract)
    let mut qsq = [0u64; SCRATCH_LIMBS];
    let qsq_len = (2 * q_len).min(SCRATCH_LIMBS);
    mul_schoolbook(&q[..q_len], &q[..q_len], &mut qsq[..qsq_len]);

    // r = rr − q²; if it would go negative, apply the correction loop:
    //   while r < 0:  r += 2·s − 1;  s −= 1
    // (paper: at most one step under full normalization; we loop to stay
    // correct without requiring the 2nd normalized bit.)
    let one = [1u64];
    if cmp(&rr[..SCRATCH_LIMBS], &qsq[..qsq_len]) >= 0 {
        sub_assign(&mut rr, &qsq[..qsq_len]);
        r[..SCRATCH_LIMBS].copy_from_slice(&rr[..SCRATCH_LIMBS]);
    } else {
        // deficit = q² − rr  (positive). Then repeatedly add (2·s − 1).
        let mut deficit = [0u64; SCRATCH_LIMBS];
        deficit[..qsq_len].copy_from_slice(&qsq[..qsq_len]);
        sub_assign(&mut deficit, &rr[..SCRATCH_LIMBS]);
        // r currently conceptually = −deficit; bring it ≥ 0.
        loop {
            // twos_minus_1 = 2·s − 1
            let mut tm = [0u64; SCRATCH_LIMBS];
            shl(s, 1, &mut tm); // 2·s  (s has < SCRATCH_LIMBS-1 sig limbs)
            sub_assign(&mut tm, &one);
            // s −= 1
            sub_assign(s, &one);
            // If deficit ≤ tm, then r = tm − deficit ≥ 0 → done.
            if cmp(&deficit[..SCRATCH_LIMBS], &tm[..SCRATCH_LIMBS]) <= 0 {
                let mut rfinal = [0u64; SCRATCH_LIMBS];
                rfinal[..SCRATCH_LIMBS].copy_from_slice(&tm[..SCRATCH_LIMBS]);
                sub_assign(&mut rfinal, &deficit[..SCRATCH_LIMBS]);
                r[..SCRATCH_LIMBS].copy_from_slice(&rfinal[..SCRATCH_LIMBS]);
                break;
            }
            // else deficit > tm: r still negative; deficit −= tm, repeat.
            sub_assign(&mut deficit, &tm[..SCRATCH_LIMBS]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::isqrt_karatsuba;
    use crate::int::algos::isqrt::isqrt_newton::isqrt_newton;

    fn kara(n: &[u64], limbs: usize) -> Vec<u64> {
        let mut out = vec![0u64; limbs];
        isqrt_karatsuba(n, &mut out);
        out
    }
    fn newton(n: &[u64], limbs: usize) -> Vec<u64> {
        let mut out = vec![0u64; limbs];
        isqrt_newton(n, &mut out);
        out
    }
    fn kara_u64(n: u64) -> u64 {
        kara(&[n], 1)[0]
    }
    fn newton_u64(n: u64) -> u64 {
        newton(&[n], 1)[0]
    }
    fn kara_u128(n: u128) -> u128 {
        let v = kara(&[n as u64, (n >> 64) as u64], 2);
        (v[0] as u128) | ((v[1] as u128) << 64)
    }
    fn newton_u128(n: u128) -> u128 {
        let v = newton(&[n as u64, (n >> 64) as u64], 2);
        (v[0] as u128) | ((v[1] as u128) << 64)
    }

    #[test]
    fn kara_known_values_u64() {
        let cases: &[(u64, u64)] = &[
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (8, 2),
            (9, 3),
            (15, 3),
            (16, 4),
            (24, 4),
            (25, 5),
            (99, 9),
            (100, 10),
            (101, 10),
            (1u64 << 62, 1u64 << 31),
            (u64::MAX, 4_294_967_295),
        ];
        for &(n, expected) in cases {
            assert_eq!(kara_u64(n), expected, "isqrt_karatsuba({n})");
        }
    }

    #[test]
    fn kara_matches_newton_u64_dense() {
        for n in 0u64..=8192 {
            assert_eq!(kara_u64(n), newton_u64(n), "dense mismatch n={n}");
        }
    }

    #[test]
    fn kara_matches_newton_u64_perfect_squares_and_edges() {
        let mut k: u64 = 1;
        while let Some(sq) = k.checked_mul(k) {
            for &n in &[sq - 1, sq, sq + 1] {
                assert_eq!(kara_u64(n), newton_u64(n), "edge mismatch n={n} (k={k})");
            }
            k += 1;
            if k > 4_294_967_295 {
                break;
            }
            if k > 8192 {
                k += 1_000_003;
            }
        }
        for &n in &[u64::MAX, u64::MAX - 1, 1u64 << 32, 1u64 << 63, (1u64 << 31) * (1u64 << 31)] {
            assert_eq!(kara_u64(n), newton_u64(n), "boundary mismatch n={n}");
        }
    }

    #[test]
    fn kara_matches_newton_u128() {
        for n in 0u128..=1024 {
            assert_eq!(kara_u128(n), newton_u128(n), "u128 dense mismatch n={n}");
        }
        for k in [
            2u128, 3, 5, 10, 100, 1_000, 1_000_000, 1_000_000_000,
            1_000_000_000_000u128, 4_294_967_296u128, 18_446_744_073_709_551_616u128,
        ] {
            let sq = k * k;
            for &n in &[sq - 1, sq, sq + 1] {
                assert_eq!(kara_u128(n), newton_u128(n), "u128 sq-edge mismatch n={n}");
            }
        }
        for &n in &[
            u128::MAX,
            u128::MAX - 1,
            1u128 << 64,
            1u128 << 100,
            1u128 << 126,
            (1u128 << 64) + 1,
        ] {
            assert_eq!(kara_u128(n), newton_u128(n), "u128 boundary mismatch n={n}");
        }
    }

    #[test]
    fn kara_matches_newton_wide_widths() {
        let mut state: u64 = 0xD1B5_4A32_D192_ED03;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for &limbs in &[3usize, 4, 5, 6, 8, 16, 24] {
            for _ in 0..40 {
                let mut n = vec![0u64; limbs];
                let top = 1 + (next() as usize % limbs);
                for l in n.iter_mut().take(top) {
                    *l = next();
                }
                if n[top - 1] == 0 {
                    n[top - 1] = 1;
                }
                assert_eq!(
                    kara(&n, limbs),
                    newton(&n, limbs),
                    "wide mismatch limbs={limbs} n={n:?}"
                );
            }
            // Perfect-square ±1 edges at this width: square a random base
            // (truncated into `limbs`), compare both kernels on the same
            // truncated operand.
            for _ in 0..10 {
                let mut b = vec![0u64; limbs];
                let bt = 1 + (next() as usize % limbs.div_ceil(2).max(1));
                for l in b.iter_mut().take(bt) {
                    *l = next();
                }
                if b[bt - 1] == 0 {
                    b[bt - 1] = 1;
                }
                let mut sq = vec![0u64; limbs * 2 + 1];
                crate::int::algos::mul::mul_schoolbook::mul_schoolbook(&b, &b, &mut sq);
                let mut n = vec![0u64; limbs];
                n.copy_from_slice(&sq[..limbs]);
                assert_eq!(
                    kara(&n, limbs),
                    newton(&n, limbs),
                    "wide square mismatch limbs={limbs}"
                );
            }
        }
    }
}
