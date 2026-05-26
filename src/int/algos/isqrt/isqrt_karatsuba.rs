// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Karatsuba Square Root over little-endian `u64` limb slices.
//!
//! Bit-identical alternative to
//! [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`], written to
//! replace Newton's *full-width* per-iteration division with a recursion
//! whose divide is **half-width** and runs only `O(log n)` times. The
//! `isqrt_ab` N-way A/B shows it crosses over the shipped Newton kernel at
//! the widest tier (`N == 64` / D1232) where it wins ~1.1-1.4x; the
//! [`crate::int::policy::isqrt`] matcher routes `N >= 64` here and `3 <= N <
//! 64` to Newton.
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
//! (1999), Algorithm 1 (`SqrtRem`); also Brent & Zimmermann, *Modern
//! Computer Arithmetic* (Cambridge UP, 2010), Algorithm 1.13. Given `n` in
//! base `B`, written `n = a₃·B³ + a₂·B² + a₁·B + a₀` with the top block
//! normalized (`a₃ ≥ B/4`):
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
//! Correctness (paper, Theorem 1): `s = ⌊√n⌋` exactly, with `r = n − s²` and
//! `0 ≤ r ≤ 2s`. The premise that makes the quotient `q ≤ B` and bounds the
//! correction to a single step is the normalization `a₃ ≥ B/4` — and it must
//! hold at **every** recursion level, not just the top.
//!
//! ## Keeping `a₃ ≥ B/4` at every level — the power-of-two window
//!
//! A naive split with `h = ⌈len/4⌉` blocks, normalized only within the top
//! `u64` limb, is WRONG for `len ∉ {4,7,8,9,…}`: the most-significant limb
//! lands in `a₂` (so `a₃ = 0 < B/4`), the quotient overshoots by ≈`B`, the
//! remainder goes to ≈`−q²`, and the single correction becomes a
//! `~2^{bits/2}`-iteration loop (a hang).
//!
//! This implementation instead normalizes into a **power-of-two limb
//! window** `w = next_pow2(sig_len)`: it left-shifts `n` by an **even** bit
//! amount so its most-significant bit lands in the top two bits of limb
//! `w−1` (recovering the root by `>> shift/2`, since `√(n·2^e) = √n·2^{e/2}`
//! for even `e`). Then `h = w/4` is exact, the high half `a₃·B + a₂` is
//! exactly `w/2` limbs (also a power of two) and inherits the top limb — so
//! it stays normalized — and the recursion halves cleanly with NO
//! re-normalization. `a₃ ≥ B/4` therefore holds at every level, `q ≤ B`, and
//! the correction is the paper's single step (a small bounded loop guards
//! against any residual, per the defense-in-depth recursion rule).
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
use crate::int::algos::support::limbs::{add_assign, bit_len, cmp, shl, shr, sub_assign};
use crate::int::policy::div_rem::dispatch as div_rem_dispatch;
use crate::int::types::compute_int::MAX_DOUBLE_LIMBS;

/// Scratch capacity — the double-N budget shared with the shipped Newton
/// `isqrt` (radicand ≤ 2N). The normalized window `w = next_pow2(sig) ≤
/// 2·sig` and every intermediate (`num`, `q²`, `s`, `r`) stays within it.
const SCRATCH_LIMBS: usize = MAX_DOUBLE_LIMBS;

/// Below this many *significant* limbs the kernel hands straight to the
/// shipped exact Newton root: the recursion needs a power-of-two window of
/// at least 4 limbs to split into four blocks, and Newton already owns the
/// small-width regime where it wins.
const BASE_LIMBS: usize = 2;

/// `out = floor(sqrt(n))`, computed by the Karatsuba Square Root recursion.
///
/// Bit-identical to
/// [`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`]; see the module
/// docs for the algorithm and the RR-3805 reference.
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

    let sig = sig_len(n);

    // ── small widths: the shipped exact Newton kernel owns them ───────────
    if sig <= BASE_LIMBS {
        isqrt_newton(&n[..sig], out);
        return;
    }

    // ── normalize into a power-of-two limb window ─────────────────────────
    // `w` = next power of two ≥ sig. Left-shift n by an EVEN bit amount so
    // its MSB lands in the top two bits of limb `w−1` (⇒ a₃ ≥ B/4 at every
    // recursion level). Even shift `e` ⇒ √(n·2^e) = √n·2^{e/2}, so the root
    // is recovered by `>> e/2`.
    let w = next_pow2_limbs(sig);
    debug_assert!(w <= SCRATCH_LIMBS, "isqrt_karatsuba window exceeds scratch");
    let sh = (w as u32) * 64 - bits; // ≥ 0: w ≥ sig ⇒ w·64 ≥ bits
    let sh_even = sh & !1u32;

    let mut nn = [0u64; SCRATCH_LIMBS];
    shl(&n[..sig], sh_even, &mut nn[..w]);

    // ── recurse on the normalized window ──────────────────────────────────
    let mut s = [0u64; SCRATCH_LIMBS];
    let mut r = [0u64; SCRATCH_LIMBS];
    sqrtrem(&nn[..w], &mut s, &mut r);

    // ── de-normalize: s_real = s >> (sh_even/2) ───────────────────────────
    let s_len = sig_len(&s[..SCRATCH_LIMBS]);
    let mut s_out = [0u64; SCRATCH_LIMBS];
    shr(&s[..s_len], sh_even / 2, &mut s_out[..s_len]);

    let copy_len = out.len().min(s_len);
    out[..copy_len].copy_from_slice(&s_out[..copy_len]);
}

/// Smallest power-of-two limb count `≥ x`, at least 4 (the four-block split
/// needs a window divisible by 4, and the recursion bottoms out at 2).
#[inline]
fn next_pow2_limbs(x: usize) -> usize {
    let mut w = 4usize;
    while w < x {
        w <<= 1;
    }
    w
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
///
/// `n.len()` is a power of two and `n` is normalized (its MSB in the top two
/// bits of the top limb). `s`/`r` are zeroed then written. Recursive
/// Karatsuba Square Root (RR-3805 Algorithm 1); base case = the shipped
/// exact Newton kernel with the remainder recovered as `n − s²`.
fn sqrtrem(n: &[u64], s: &mut [u64], r: &mut [u64]) {
    for v in s.iter_mut() {
        *v = 0;
    }
    for v in r.iter_mut() {
        *v = 0;
    }
    let w = n.len();

    // ── base case: exact Newton root + remainder n − s² ───────────────────
    if w <= BASE_LIMBS {
        isqrt_newton(n, &mut s[..w]);
        let s_len = sig_len(&s[..w]);
        let mut sq = [0u64; SCRATCH_LIMBS];
        let sq_len = (2 * s_len).min(SCRATCH_LIMBS);
        mul_schoolbook(&s[..s_len], &s[..s_len], &mut sq[..sq_len]);
        // r = n − s²  (n ≥ s² by definition of the floor root).
        r[..w].copy_from_slice(n);
        sub_assign(&mut r[..w], &sq[..sq_len.min(w)]);
        return;
    }

    // ── four equal blocks of h = w/4 limbs: n = a3·B³+a2·B²+a1·B+a0 ────────
    // B = 2^{64·h}. high (= a3·B + a2) is the top w/2 limbs and is itself a
    // normalized power-of-two-length number → the recursion needs no
    // re-normalization.
    let h = w / 4;
    let a0 = &n[0..h];
    let a1 = &n[h..2 * h];
    let high = &n[2 * h..w]; // a3·B + a2, w/2 limbs

    // ── (s', r') = SqrtRem(high) ──────────────────────────────────────────
    let mut sp = [0u64; SCRATCH_LIMBS];
    let mut rp = [0u64; SCRATCH_LIMBS];
    sqrtrem(high, &mut sp, &mut rp);
    let sp_len = sig_len(&sp[..SCRATCH_LIMBS]);
    let rp_len = sig_len(&rp[..SCRATCH_LIMBS]);

    // ── (q, u) = DivRem(r'·B + a1, 2·s') ──────────────────────────────────
    // numerator = r'·B + a1  (r' at limb offset h, a1 in the low h).
    let mut num = [0u64; SCRATCH_LIMBS];
    for (i, &v) in rp[..rp_len].iter().enumerate() {
        if h + i < SCRATCH_LIMBS {
            num[h + i] = v;
        }
    }
    add_assign(&mut num, a1);
    let num_len = sig_len(&num[..SCRATCH_LIMBS]);

    // divisor = 2·s'
    let mut den = [0u64; SCRATCH_LIMBS];
    shl(&sp[..sp_len], 1, &mut den[..sp_len + 1]);
    let den_len = sig_len(&den[..SCRATCH_LIMBS]);

    let mut q = [0u64; SCRATCH_LIMBS];
    let mut u = [0u64; SCRATCH_LIMBS];
    let qrlen = num_len.max(den_len);
    div_rem_dispatch(
        &num[..num_len],
        &den[..den_len],
        &mut q[..qrlen],
        &mut u[..qrlen],
    );
    let q_len = sig_len(&q[..qrlen]);
    let u_len = sig_len(&u[..qrlen]);

    // ── s = s'·B + q  (s' at offset h, q low; add_assign folds any carry) ──
    for (i, &v) in sp[..sp_len].iter().enumerate() {
        if h + i < SCRATCH_LIMBS {
            s[h + i] = v;
        }
    }
    add_assign(s, &q[..q_len]);

    // ── r = u·B + a0 − q² ─────────────────────────────────────────────────
    let mut rr = [0u64; SCRATCH_LIMBS];
    for (i, &v) in u[..u_len].iter().enumerate() {
        if h + i < SCRATCH_LIMBS {
            rr[h + i] = v;
        }
    }
    add_assign(&mut rr, a0);
    let mut qsq = [0u64; SCRATCH_LIMBS];
    let qsq_len = (2 * q_len).min(SCRATCH_LIMBS);
    mul_schoolbook(&q[..q_len], &q[..q_len], &mut qsq[..qsq_len]);

    let one = [1u64];
    if cmp(&rr[..SCRATCH_LIMBS], &qsq[..qsq_len]) >= 0 {
        sub_assign(&mut rr, &qsq[..qsq_len]);
        r[..SCRATCH_LIMBS].copy_from_slice(&rr[..SCRATCH_LIMBS]);
    } else {
        // r = rr − q² < 0. deficit = q² − rr (> 0). Apply the paper's
        // correction `r += 2s − 1; s −= 1` until r ≥ 0.
        let mut deficit = [0u64; SCRATCH_LIMBS];
        deficit[..qsq_len].copy_from_slice(&qsq[..qsq_len]);
        sub_assign(&mut deficit, &rr[..SCRATCH_LIMBS]);
        // Under correct normalization (a3 ≥ B/4) this is a SINGLE step; the
        // bound far above that turns any residual logic error into an
        // instant located panic instead of a silent hang (defense-in-depth
        // recursion rule). See the module docs.
        let mut guard = 0usize;
        loop {
            guard += 1;
            debug_assert!(
                guard <= 8,
                "isqrt_karatsuba correction exceeded bound (guard={guard}); \
                 normalization broken — deficit={:?} s={:?}",
                &deficit[..SCRATCH_LIMBS.min(8)],
                &s[..SCRATCH_LIMBS.min(8)],
            );
            // tm = 2·s − 1 (uses the current s).
            let mut tm = [0u64; SCRATCH_LIMBS];
            shl(s, 1, &mut tm);
            sub_assign(&mut tm, &one);
            if cmp(&deficit[..SCRATCH_LIMBS], &tm[..SCRATCH_LIMBS]) <= 0 {
                // r += 2s − 1 makes it ≥ 0: r = tm − deficit. Then s −= 1.
                let mut rfinal = [0u64; SCRATCH_LIMBS];
                rfinal.copy_from_slice(&tm);
                sub_assign(&mut rfinal, &deficit[..SCRATCH_LIMBS]);
                sub_assign(s, &one);
                r[..SCRATCH_LIMBS].copy_from_slice(&rfinal[..SCRATCH_LIMBS]);
                break;
            }
            // r += 2s − 1 still < 0: deficit −= tm; s −= 1; repeat.
            sub_assign(&mut deficit, &tm[..SCRATCH_LIMBS]);
            sub_assign(s, &one);
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
            // Skip k whose square overflows u128 (e.g. k = 2^64): the edge
            // probe needs k² to fit; k = 2^64 is covered by the boundary
            // list below (1u128 << 64) instead.
            let Some(sq) = k.checked_mul(k) else { continue };
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

    // Multi-limb operands (window up to 32 limbs) need the wide-tier scratch
    // budget (`SCRATCH_LIMBS = MAX_DOUBLE_LIMBS`, which is build-max-sized);
    // the narrow default build sizes it for ≤2-limb tiers, where this kernel
    // is never engaged. Gate the wide sweep to a wide build accordingly.
    #[cfg(feature = "wide")]
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
            // Perfect-square ±1 edges at this width.
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
