// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Non-allocating Toom-Cook 3-way (Toom-3) full-product multiplication.
//!
//! # Reference
//!
//! Classical 5-point Toom-Cook split-3 algorithm as described in:
//!   Knuth, D.E., "The Art of Computer Programming, Vol. 2:
//!   Seminumerical Algorithms", 3rd ed., section 4.3.3, Addison-Wesley, 1997.
//! Evaluation/interpolation scheme following:
//!   Bodrato, M., "Towards Optimal Toom-Cook Multiplication for Univariate
//!   and Multivariate Polynomials in Characteristic 2 and 0", WAIFI 2007,
//!   LNCS 4547, pp. 116-133, Springer, 2007.
//! Exact division by 3 by the modular inverse:
//!   Jebelean, T., "An algorithm for exact division", J. Symbolic Computation
//!   15 (1993), 169-180; also Warren, "Hacker's Delight" 2nd ed. §10-17.
//!
//! CLEANROOM: implemented from the published mathematical descriptions only.
//! No GPL/LGPL source (GMP, MPFR, FLINT) was consulted or adapted.
//!
//! # Algorithm
//!
//! Given n-limb operands a and b, let k = ceil(n/3) and split:
//!   a = a0 + a1*B + a2*B^2,  b = b0 + b1*B + b2*B^2,  B = 2^(BITS*k).
//! Evaluate at 5 points {0, +1, -1, +2, inf}:
//!   v0(a) = a0
//!   v1(a) = a0 + a1 + a2
//!   vm(a) = a0 - a1 + a2     (may be negative; sign tracked by flag)
//!   v2(a) = a0 + 2*a1 + 4*a2
//!   vi(a) = a2
//! Multiply pointwise (5 sub-products, each ~n/3 limbs wide), interpolate
//! 5 product-polynomial coefficients using exact integer divisions (by 2
//! and 3; divisibility guaranteed by the Toom-3 identity for integers),
//! then recombine:
//!   product = c0 + c1*B + c2*B^2 + c3*B^3 + c4*B^4.
//!
//! # Architecture conformance
//!
//! - ONE generic kernel: `toom3_rec_limb::<L>` is the single recursion body,
//!   instantiated at `u64` (the value-slice path) or `u128` (the packed path).
//!   No per-tier copies, no per-limb-type duplication (rule 2). The Limb-space
//!   schoolbook base case + the L add/sub are SHARED with Karatsuba
//!   (`mul_karatsuba::{schoolbook_rec_limb, limb_add_assign, limb_sub_assign}`).
//! - Non-allocating: fixed `[L; *_SCRATCH_*]` stack buffer; recursion carves
//!   disjoint windows via `split_at_mut`.
//! - Base-case: below `TOOM3_BASE_THRESHOLD` limbs falls to the schoolbook base.
//! - The limb width (u64 / u128) is the policy-mapper's choice (a `LimbSize`
//!   verdict), exactly like Karatsuba; this kernel just provides both.
//! - NOT wired into any policy: a kept alternative awaiting the policy map.

use crate::int::algos::mul::mul_karatsuba::{
    limb_add_assign, limb_sub_assign, schoolbook_rec_limb,
};
use crate::int::types::compute_limbs::Limb;

// ---- scratch sizing --------------------------------------------------------

/// Fixed stack scratch for the u64 Toom-3 kernel, in u64 limbs.
///
/// Per level on n limbs (k = ceil(n/3)+1):
///   5 a-eval + 5 b-eval buffers of k limbs each = 10k
///   5 product buffers of 2k limbs = 10k
///   3 working temp buffers of 2k limbs = 6k
///   Total per level = 26k.
/// For n=64, threshold=9: level-0 k=23 -> 598; level-1 k<=9 -> base. 1024 is ample.
pub(crate) const TOOM3_SCRATCH_LIMBS: usize = 1024;

/// Minimum operand width (limbs) below which Toom-3 falls to schoolbook.
/// Must be >= 3.
pub(crate) const TOOM3_BASE_THRESHOLD: usize = 9;

/// Upper bound on scratch (in limbs) for toom3_rec_limb(n, threshold).
pub(crate) const fn toom3_scratch_needed(n: usize, threshold: usize) -> usize {
    if n < threshold {
        return 0;
    }
    let k = (n + 2) / 3 + 1;
    26 * k + toom3_scratch_needed(k, threshold)
}

// ---- public entry points ---------------------------------------------------

/// Non-allocating Toom-Cook 3-way multiply: out = a * b (u64 value slices).
/// a and b must have equal length. out.len() >= 2*a.len() and out must be zeroed.
///
/// The u64 path of the ONE generic kernel: `toom3_rec_limb::<u64>`.
pub(crate) fn mul_toom3(a: &[u64], b: &[u64], out: &mut [u64]) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        toom3_scratch_needed(a.len(), TOOM3_BASE_THRESHOLD) <= TOOM3_SCRATCH_LIMBS,
        "Toom-3 scratch overflow: n={} needs {} limbs, have {}",
        a.len(),
        toom3_scratch_needed(a.len(), TOOM3_BASE_THRESHOLD),
        TOOM3_SCRATCH_LIMBS,
    );
    let mut scratch = [0u64; TOOM3_SCRATCH_LIMBS];
    toom3_rec_limb::<u64>(a, b, out, &mut scratch, TOOM3_BASE_THRESHOLD, inv3_of::<u64>());
}

/// Limb-generic Toom-3 full product -- bench-alt / test entry.
///
/// Packs N u64 operand limbs into L limbs (N for L=u64, N/2 for L=u128),
/// runs ONE generic `toom3_rec_limb`, then unpacks the 2*N-u64 product.
/// For L = u64 numerically identical to `mul_toom3`; for L = u128 the identical
/// eval/interpolate/recombine runs in u128 space (half the limb count and
/// carry-chain depth per inner step). out is zeroed by this function.
///
/// threshold is in u64-limb units; converted to packed units internally so the
/// recursion fires consistently for both limb widths.
#[cfg(any(test, feature = "bench-alt"))]
pub(crate) fn mul_toom3_limb<const N: usize, L: Limb>(a: &[u64; N], b: &[u64; N], out: &mut [u64]) {
    let h = L::packed_len(N);
    debug_assert!(h > 0 && h <= N);
    // Pack operands into L-space. [L; N] always covers packed_len(N) <= N.
    let mut ap = [L::ZERO; N];
    let mut bp = [L::ZERO; N];
    L::pack(a, &mut ap[..h]);
    L::pack(b, &mut bp[..h]);

    // Threshold in packed-limb units: u128 packs 2 u64/limb so halve it.
    let ratio: usize = if h < N { 2 } else { 1 };
    let threshold_packed = (TOOM3_BASE_THRESHOLD / ratio).max(3);

    // Product buffer (2h packed slots) + scratch, both in L limbs. The scratch
    // is sized to the u64 worst case (TOOM3_SCRATCH_LIMBS): for L=u64, h=N needs
    // toom3_scratch_needed(N, 9) (598 at N=64); the u128 packing (h=N/2) needs
    // less. One generously-sized buffer covers both limb widths of this
    // bench/test entry (the production u64 path `mul_toom3` is exactly sized).
    let mut prod = [L::ZERO; TOOM3_SCRATCH_LIMBS];
    let mut scratch = [L::ZERO; TOOM3_SCRATCH_LIMBS];
    debug_assert!(2 * h <= prod.len());
    debug_assert!(
        toom3_scratch_needed(h, threshold_packed) <= TOOM3_SCRATCH_LIMBS,
        "Toom-3 limb scratch overflow: h={} threshold={} needs {} > {}",
        h,
        threshold_packed,
        toom3_scratch_needed(h, threshold_packed),
        TOOM3_SCRATCH_LIMBS,
    );

    for v in prod[..2 * h].iter_mut() {
        *v = L::ZERO;
    }
    toom3_rec_limb::<L>(
        &ap[..h],
        &bp[..h],
        &mut prod[..2 * h],
        &mut scratch,
        threshold_packed,
        inv3_of::<L>(),
    );

    L::unpack(&prod[..2 * h], &mut out[..2 * N]);
}

/// Test-only entry at a configurable threshold for correctness sweeps (u64).
#[cfg(test)]
pub(crate) fn mul_toom3_with_threshold(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let need = toom3_scratch_needed(a.len(), threshold);
    let mut scratch = vec![0u64; need.max(1)];
    toom3_rec_limb::<u64>(a, b, out, &mut scratch, threshold, inv3_of::<u64>());
}

// ---- core recursion (ONE generic body, u64 or u128) ------------------------

/// One Toom-3 recursion level in L space. out is pre-zeroed; scratch is the
/// live tail. Lifted from u64 to the generic `L: Limb` so the same body serves
/// the value-slice (u64) and packed (u128) paths -- no per-limb-type copy.
fn toom3_rec_limb<L: Limb>(
    a: &[L],
    b: &[L],
    out: &mut [L],
    scratch: &mut [L],
    threshold: usize,
    inv: L,
) {
    let n = a.len();
    debug_assert_eq!(n, b.len());
    debug_assert!(out.len() >= 2 * n);
    debug_assert!(threshold >= 3);

    if n < threshold {
        schoolbook_rec_limb::<L>(a, b, out);
        return;
    }

    // Split: k = ceil(n/3); parts a0,a1 are k limbs, a2 is n-2k limbs.
    let k = (n + 2) / 3;
    let k2 = (2 * k).min(n);
    let a0 = &a[..k];
    let a1 = &a[k..k2];
    let a2 = &a[k2..];
    let b0 = &b[..k];
    let b1 = &b[k..k2];
    let b2 = &b[k2..];

    let ew = k + 1; // eval-buffer width (k+1 for carry headroom)
    let pw = 2 * ew; // product-buffer width

    // Carve level-local scratch (26*(k+1) limbs max):
    //   pa0..pai: 5*ew (a eval points); pb0..pbi: 5*ew (b eval points)
    //   w0..wi: 5*pw (sub-products); t1, t2, tmp: 3*pw (interp + shift)
    let level_need = 10 * ew + 8 * pw;
    let (level_buf, child_scratch) = scratch.split_at_mut(level_need);
    for v in level_buf.iter_mut() {
        *v = L::ZERO;
    } // zero level buf (scratch may be dirty from prior calls)

    let (pa0, r) = level_buf.split_at_mut(ew);
    let (pa1, r) = r.split_at_mut(ew);
    let (pam, r) = r.split_at_mut(ew);
    let (pa2, r) = r.split_at_mut(ew);
    let (pai, r) = r.split_at_mut(ew);
    let (pb0, r) = r.split_at_mut(ew);
    let (pb1, r) = r.split_at_mut(ew);
    let (pbm, r) = r.split_at_mut(ew);
    let (pb2, r) = r.split_at_mut(ew);
    let (pbi, r) = r.split_at_mut(ew);
    let (w0, r) = r.split_at_mut(pw);
    let (w1, r) = r.split_at_mut(pw);
    let (wm, r) = r.split_at_mut(pw);
    let (w2, r) = r.split_at_mut(pw);
    let (wi, r) = r.split_at_mut(pw);
    let (t1, r) = r.split_at_mut(pw);
    let (t2, r) = r.split_at_mut(pw);
    let (tmp, _) = r.split_at_mut(pw);

    // ---- evaluate a at {0, 1, -1, 2, inf} ---------------------------------
    pa0[..a0.len()].copy_from_slice(a0);
    pai[..a2.len()].copy_from_slice(a2);

    // a(1) = a0 + a1 + a2
    pa1[..a0.len()].copy_from_slice(a0);
    let _ = limb_add_assign(pa1, a1);
    let _ = limb_add_assign(pa1, a2);

    // a(-1) = |a0 + a2 - a1|, sign in am_neg
    pam[..a0.len()].copy_from_slice(a0);
    let _ = limb_add_assign(pam, a2);
    let am_neg = signed_sub_limb(pam, a1);

    // a(2) = a0 + 2*a1 + 4*a2
    pa2[..a0.len()].copy_from_slice(a0);
    let _ = limb_add_assign(pa2, a1);
    let _ = limb_add_assign(pa2, a1); // pa2 = a0 + 2*a1
    tmp[..a2.len()].copy_from_slice(a2);
    shl_inplace_limb(&mut tmp[..ew], 2); // 4*a2
    let _ = limb_add_assign(pa2, &tmp[..ew]);

    // ---- evaluate b at {0, 1, -1, 2, inf} ---------------------------------
    pb0[..b0.len()].copy_from_slice(b0);
    pbi[..b2.len()].copy_from_slice(b2);

    pb1[..b0.len()].copy_from_slice(b0);
    let _ = limb_add_assign(pb1, b1);
    let _ = limb_add_assign(pb1, b2);

    pbm[..b0.len()].copy_from_slice(b0);
    let _ = limb_add_assign(pbm, b2);
    let bm_neg = signed_sub_limb(pbm, b1);

    pb2[..b0.len()].copy_from_slice(b0);
    let _ = limb_add_assign(pb2, b1);
    let _ = limb_add_assign(pb2, b1); // pb2 = b0 + 2*b1
    for v in tmp.iter_mut() {
        *v = L::ZERO;
    }
    tmp[..b2.len()].copy_from_slice(b2);
    shl_inplace_limb(&mut tmp[..ew], 2); // 4*b2
    let _ = limb_add_assign(pb2, &tmp[..ew]);

    // ---- 5 pointwise sub-products -----------------------------------------
    // w0..wi were already zeroed by the level_buf zero above and nothing
    // writes them in the eval phase, so they are still zero here -- the child
    // toom3_rec_limb accumulates (+=) into a pre-zeroed `out`.
    toom3_rec_limb::<L>(pa0, pb0, w0, child_scratch, threshold, inv);
    toom3_rec_limb::<L>(pa1, pb1, w1, child_scratch, threshold, inv);
    toom3_rec_limb::<L>(pam, pbm, wm, child_scratch, threshold, inv);
    toom3_rec_limb::<L>(pa2, pb2, w2, child_scratch, threshold, inv);
    toom3_rec_limb::<L>(pai, pbi, wi, child_scratch, threshold, inv);

    let wm_neg = am_neg ^ bm_neg;

    // ---- interpolation (5-point Toom-3; Knuth TAOCP vol.2 sec.4.3.3) --------
    //
    // r0=w0=c0, r1=w1, rm=wm(signed wm_neg), r2=w2, ri=wi=c4.
    //
    // Key identities (p(x)=c0+c1*x+c2*x^2+c3*x^3+c4*x^4):
    //   w1+rm = 2(c0+c2+c4)    w1-rm = 2(c1+c3)
    //   w2 - c0 = 2c1+4c2+8c3+16c4
    // All intermediate values are non-negative for unsigned inputs.
    //
    // Step A: t1 = (w1+rm)/2 - w0 - wi     [= c2]
    //         t2 = (w1-rm)/2               [= c1+c3]
    // Step B: s  = (w2-w0)/2 - 2*c2 - 8*c4 [= c1+4*c3]
    // Step C: c3 = (s - t2) / 3             [= c3; exact: s-t2=3*c3]
    // Step D: c1 = t2 - c3                  [= c1]
    // Recombine: out += c0 + c1*B + c2*B^2 + c3*B^3 + c4*B^4

    // Step A: form t1=(w1+rm)/2-w0-wi and t2=(w1-rm)/2.
    // Both results are >= 0 since ci >= 0.
    let t1_neg;
    let t2_neg;
    if !wm_neg {
        // rm >= 0: t1_before_halve = w1+wm; t2_before_halve = w1-wm (both >= 0)
        t1[..pw].copy_from_slice(w1);
        let _ = limb_add_assign(t1, wm);
        t1_neg = false;
        t2[..pw].copy_from_slice(w1);
        t2_neg = signed_sub_limb(t2, wm); // w1 >= wm since (w1-rm)/2=c1+c3 >= 0
    } else {
        // rm < 0: t1_before_halve = w1-|wm| >= 0; t2_before_halve = w1+|wm| >= 0
        t1[..pw].copy_from_slice(w1);
        t1_neg = signed_sub_limb(t1, wm); // should be false for unsigned inputs
        t2[..pw].copy_from_slice(w1);
        let _ = limb_add_assign(t2, wm);
        t2_neg = false;
    }
    shr1_limb(t1); // t1 = (w1+rm)/2
    shr1_limb(t2); // t2 = (w1-rm)/2
    // t1 -= c0 + c4  =>  t1 = c2
    let t1_neg = signed_sub_signed_limb(t1, t1_neg, w0, false);
    let t1_neg = signed_sub_signed_limb(t1, t1_neg, wi, false);
    // t2 = c1+c3; t1 = c2; both >= 0.
    debug_assert!(!t1_neg, "c2 (t1) negative -- interpolation error (step A)");
    debug_assert!(!t2_neg, "t2=c1+c3 negative -- interpolation error (step A)");

    // Step B: s = (w2 - w0) / 2 - 2*c2 - 8*c4  [= c1+4*c3 >= 0]
    // w2 - w0 = 2c1+4c2+8c3+16c4 >= 0 (always, for unsigned).
    // (w2-w0)/2 = c1+2*c2+4*c3+8*c4. Subtract 2*c2 and 8*c4: c1+4*c3 >= 0.
    // Re-use w2 for s (it is not needed after this).
    let went_neg = signed_sub_limb(w2, w0);
    debug_assert!(!went_neg, "Toom-3: r2 < r0 -- invariant violated for unsigned inputs");
    shr1_limb(w2); // w2 = (w2-w0)/2 = c1+2*c2+4*c3+8*c4
    // subtract 2*c2 (= 2*t1):
    tmp[..pw].copy_from_slice(t1); // t1 = c2
    shl_inplace_limb(tmp, 1); // tmp = 2*c2
    let s_neg = signed_sub_signed_limb(w2, false, tmp, t1_neg);
    // subtract 8*c4 (= 8*wi):
    tmp[..pw].copy_from_slice(wi);
    shl_inplace_limb(tmp, 3); // tmp = 8*wi = 8*c4
    let s_neg = signed_sub_signed_limb(w2, s_neg, tmp, false);
    // w2 = s = c1+4*c3 >= 0
    debug_assert!(!s_neg, "s=c1+4c3 negative -- interpolation error (step B)");

    // Step C: c3 = (s - t2) / 3
    // s - t2 = (c1+4*c3) - (c1+c3) = 3*c3 >= 0 (exact division).
    // Re-use tmp for c3 (avoid aliasing: s is in w2, t2 is in t2 buf).
    tmp[..pw].copy_from_slice(w2); // tmp = s
    let c3_neg = signed_sub_signed_limb(tmp, s_neg, t2, t2_neg); // tmp = s - t2 = 3*c3
    debug_assert!(!c3_neg, "3*c3 negative -- interpolation error (step C)");
    let c3_neg = div3_limb(tmp, c3_neg, inv); // tmp = c3

    // Step D: c1 = t2 - c3  [= c1+c3 - c3 = c1 >= 0]
    // Re-use w2 for c1.
    w2[..pw].copy_from_slice(t2); // w2 = c1+c3
    let c1_neg = signed_sub_signed_limb(w2, t2_neg, tmp, c3_neg); // w2 = c1
    debug_assert!(!c1_neg, "c1 negative -- interpolation error (step D)");

    // Now: w0=c0, w2=c1, t1=c2 (t1_neg), tmp=c3 (c3_neg), wi=c4.
    debug_assert!(!t1_neg, "c2 negative -- final check");
    debug_assert!(!c3_neg, "c3 negative -- final check");

    // ---- recombine: out = c0 + c1*B + c2*B^2 + c3*B^3 + c4*B^4 ----------
    // c0=w0, c1=w2, c2=t1, c3=tmp, c4=wi (all >= 0 per the debug_asserts above).
    add_into_out_limb(out, 0, w0); // c0
    add_into_out_limb(out, k, w2); // c1 (stored in w2 buffer)
    add_into_out_limb(out, 2 * k, t1); // c2
    add_into_out_limb(out, 3 * k, tmp); // c3 (stored in tmp buffer)
    add_into_out_limb(out, 4 * k, wi); // c4
}

// ---- L-generic helpers ------------------------------------------------------

/// Compare two little-endian L-limb magnitudes (lengths may differ): returns
/// `1` if `a > b`, `-1` if `a < b`, `0` if equal.
#[inline]
fn cmp_cross_limb<L: Limb>(a: &[L], b: &[L]) -> i32 {
    let la = a.len();
    let lb = b.len();
    let top = la.max(lb);
    let mut i = top;
    while i > 0 {
        i -= 1;
        let av = if i < la { a[i] } else { L::ZERO };
        let bv = if i < lb { b[i] } else { L::ZERO };
        if av > bv {
            return 1;
        }
        if av < bv {
            return -1;
        }
    }
    0
}

/// `dst = src - dst` (reverse subtract, little-endian). Caller guarantees
/// `src >= dst` as magnitudes. Avoids a separate two's-complement negate.
#[inline]
fn rsub_assign_limb<L: Limb>(dst: &mut [L], src: &[L]) {
    let mut borrow = false;
    let mut i = 0;
    while i < dst.len() {
        let sv = if i < src.len() { src[i] } else { L::ZERO };
        let (d1, b1) = sv.overflowing_sub(dst[i]);
        let (d2, b2) = d1.overflowing_sub(if borrow { L::ONE } else { L::ZERO });
        dst[i] = d2;
        borrow = b1 | b2;
        i += 1;
    }
}

/// Subtract `src` from `dst` (unsigned magnitudes). Returns true when
/// `src > dst` (result is the magnitude `|dst - src|`, i.e. negative).
#[inline]
fn signed_sub_limb<L: Limb>(dst: &mut [L], src: &[L]) -> bool {
    if cmp_cross_limb(dst, src) >= 0 {
        let _ = limb_sub_assign(dst, src);
        false
    } else {
        rsub_assign_limb(dst, src); // dst = src - dst
        true
    }
}

/// Signed subtraction: `(a, a_neg) -= (b, b_neg)`. Returns the new sign.
#[inline]
fn signed_sub_signed_limb<L: Limb>(a: &mut [L], a_neg: bool, b: &[L], b_neg: bool) -> bool {
    signed_add_signed_limb(a, a_neg, b, !b_neg)
}

/// Signed addition: `(a, a_neg) += (b, b_neg)`. Returns the new sign.
#[inline]
fn signed_add_signed_limb<L: Limb>(a: &mut [L], a_neg: bool, b: &[L], b_neg: bool) -> bool {
    if a_neg == b_neg {
        let _ = limb_add_assign(a, b);
        a_neg
    } else if cmp_cross_limb(a, b) >= 0 {
        let _ = limb_sub_assign(a, b);
        a_neg
    } else {
        rsub_assign_limb(a, b); // a = b - a
        b_neg
    }
}

/// In-place left shift by `shift` bits (0 < shift < L::BITS). Carry out dropped.
/// Carry-merge via disjoint-bit addition (the low `shift` bits of `x << shift`
/// are zero, so OR-ing the carry == adding it).
#[inline]
fn shl_inplace_limb<L: Limb>(a: &mut [L], shift: u32) {
    if shift == 0 {
        return;
    }
    let rshift = L::BITS - shift;
    let mut carry = L::ZERO;
    for limb in a.iter_mut() {
        let out_carry = limb.wrapping_shr(rshift);
        *limb = limb.wrapping_shl(shift).overflowing_add(carry).0;
        carry = out_carry;
    }
}

/// In-place right shift by 1 (value must be even for correct interpolation).
/// The bit leaving limb i+1's LSB enters limb i's MSB; disjoint-bit add merges.
#[inline]
fn shr1_limb<L: Limb>(a: &mut [L]) {
    let mut carry = L::ZERO; // pre-positioned at the MSB
    for limb in a.iter_mut().rev() {
        let new_carry = limb.wrapping_shl(L::BITS - 1); // bit0 -> MSB
        *limb = limb.wrapping_shr(1).overflowing_add(carry).0;
        carry = new_carry;
    }
    debug_assert!(carry == L::ZERO, "shr1: odd value -- must be even for Toom-3");
}

/// 3^{-1} mod 2^BITS via Newton's iteration (a fixed point of
/// `x <- x*(2 - 3x)`). Seed 3 is correct mod 8; each step doubles the correct
/// bits, so 6 steps cover up to 192 bits. Computed ONCE per top-level multiply
/// and threaded into `div3_limb` -- the recursion calls div3 at every internal
/// node, so recomputing the inverse per node was a fixed per-call cost that
/// regressed the small-N path (+23% at N=24) since LLVM did not const-fold the
/// loop. inv depends only on L, so one computation serves the whole tree.
#[inline]
fn inv3_of<L: Limb>() -> L {
    let three = L::ONE.overflowing_add(L::ONE).0.overflowing_add(L::ONE).0;
    let two = L::ONE.overflowing_add(L::ONE).0;
    let mut inv = three; // x0 = 3 (3*3 = 9 == 1 mod 8)
    let mut step = 0;
    while step < 6 {
        let t = two.overflowing_sub(three.widening_mul(inv).0).0;
        inv = inv.widening_mul(t).0;
        step += 1;
    }
    inv
}

/// Exact division by 3 (little-endian unsigned). Caller guarantees divisibility
/// and passes the precomputed modular inverse `inv = 3^{-1} mod 2^BITS`.
///
/// Jebelean exact division: each quotient limb is `(limb - borrow) * inv`
/// (mod 2^BITS), with next borrow = `mulhi(q, 3) + underflow`. ONE multiply +
/// a cheap `mulhi` per limb -- no per-limb 128/64 hardware division (the prior
/// `/3`+`%3` was the kernel's #1 self-time hot spot per the samply probe).
#[inline]
fn div3_limb<L: Limb>(a: &mut [L], neg: bool, inv: L) -> bool {
    let three = L::ONE.overflowing_add(L::ONE).0.overflowing_add(L::ONE).0;
    let mut borrow = L::ZERO;
    for limb in a.iter_mut() {
        let (s, under) = limb.overflowing_sub(borrow);
        let q = s.widening_mul(inv).0; // q = s * 3^{-1} mod 2^BITS
        *limb = q;
        // next borrow = mulhi(q, 3) + underflow
        let hi = q.widening_mul(three).1;
        borrow = hi.overflowing_add(if under { L::ONE } else { L::ZERO }).0;
    }
    debug_assert!(borrow == L::ZERO, "div3: not divisible by 3 -- Toom-3 interpolation error");
    neg
}

/// Add `src` into `out[offset..]` with carry propagation (L-generic).
#[inline]
fn add_into_out_limb<L: Limb>(out: &mut [L], offset: usize, src: &[L]) {
    if offset < out.len() {
        let _ = limb_add_assign(&mut out[offset..], src);
    }
}

// ---- tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;

    /// SplitMix64 -- Vigna 2014, public-domain reference algorithm.
    fn fill(n: usize, seed: u64) -> Vec<u64> {
        let mut out = vec![0u64; n];
        let mut state = seed;
        for x in out.iter_mut() {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            *x = z ^ (z >> 31);
        }
        out
    }

    fn schoolbook_ref(a: &[u64], b: &[u64]) -> Vec<u64> {
        let n = a.len().max(b.len());
        let mut a2 = vec![0u64; n];
        let mut b2 = vec![0u64; n];
        a2[..a.len()].copy_from_slice(a);
        b2[..b.len()].copy_from_slice(b);
        let mut out = vec![0u64; 2 * n];
        mul_schoolbook(&a2, &b2, &mut out);
        out
    }

    fn edge_cases(n: usize) -> Vec<Vec<u64>> {
        let mut v = vec![vec![0u64; n], vec![u64::MAX; n]];
        let mut lo = vec![0u64; n];
        lo[0] = u64::MAX;
        v.push(lo);
        if n > 1 {
            let mut hi = vec![0u64; n];
            hi[n - 1] = 1;
            v.push(hi);
        }
        if n > 2 {
            let mut mid = vec![0u64; n];
            mid[n / 2] = u64::MAX;
            v.push(mid);
        }
        v
    }

    /// Bit-identity against schoolbook (u64 path): N in 3..=64, thresholds
    /// {3,6,9}, edge operands and random operands, commutativity.
    #[test]
    fn toom3_bit_identical_to_schoolbook() {
        const WIDTHS: &[usize] = &[
            3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 18, 21, 24, 27, 32, 33, 36, 48, 51, 60, 63, 64,
        ];
        const THRESHOLDS: &[usize] = &[3, 6, 9];

        for &n in WIDTHS {
            for &th in THRESHOLDS {
                for a in edge_cases(n) {
                    for b in edge_cases(n) {
                        let expected = schoolbook_ref(&a, &b);
                        let mut got = vec![0u64; 2 * n];
                        mul_toom3_with_threshold(&a, &b, &mut got, th);
                        assert_eq!(got, expected, "mismatch (edge) n={n} th={th}");
                        let mut got2 = vec![0u64; 2 * n];
                        mul_toom3_with_threshold(&b, &a, &mut got2, th);
                        assert_eq!(got2, expected, "not commutative n={n} th={th}");
                    }
                }
                for seed in [1u64, 3, 7, 13, 42, 1337, 0xDEAD_BEEF, 0xCAFE_F00D] {
                    let a = fill(n, seed);
                    let b = fill(n, seed.wrapping_add(0x1234_5678));
                    let expected = schoolbook_ref(&a, &b);
                    let mut got = vec![0u64; 2 * n];
                    mul_toom3_with_threshold(&a, &b, &mut got, th);
                    assert_eq!(got, expected, "mismatch (random) n={n} th={th} seed={seed}");
                }
            }
        }
    }

    /// The packed u128 path (`mul_toom3_limb::<N, u128>`) is bit-identical to
    /// the u64 path / schoolbook across the even wide widths it covers.
    /// The u64 instantiation of the same generic entry is checked too, so the
    /// ONE kernel is verified at BOTH limb widths.
    #[test]
    fn toom3_limb_u128_bit_identical() {
        macro_rules! check {
            ($N:literal) => {{
                const N: usize = $N;
                // edge operands
                let mut ops: Vec<([u64; N], [u64; N])> = Vec::new();
                ops.push(([0u64; N], [u64::MAX; N]));
                ops.push(([u64::MAX; N], [u64::MAX; N]));
                let mut lo = [0u64; N];
                lo[0] = u64::MAX;
                ops.push((lo, [u64::MAX; N]));
                // random operands
                for seed in [1u64, 7, 42, 0xDEAD_BEEF, 0xCAFE_F00D, 1009] {
                    let a = fill(N, seed);
                    let b = fill(N, seed.wrapping_add(0x9999));
                    let mut aa = [0u64; N];
                    let mut bb = [0u64; N];
                    aa.copy_from_slice(&a);
                    bb.copy_from_slice(&b);
                    ops.push((aa, bb));
                }
                for (a, b) in ops {
                    let expected = schoolbook_ref(&a, &b);
                    let mut g_u64 = vec![0u64; 2 * N];
                    super::mul_toom3_limb::<N, u64>(&a, &b, &mut g_u64);
                    assert_eq!(g_u64, expected, "toom3_limb u64 mismatch N={}", N);
                    let mut g_u128 = vec![0u64; 2 * N];
                    super::mul_toom3_limb::<N, u128>(&a, &b, &mut g_u128);
                    assert_eq!(g_u128, expected, "toom3_limb u128 mismatch N={}", N);
                }
            }};
        }
        // even N >= 2*threshold so the recursion engages at both widths
        check!(18);
        check!(24);
        check!(32);
        check!(48);
        check!(64);
    }

    /// Fixed-scratch entry handles n=64 without overflow.
    #[test]
    fn toom3_max_width_fixed_scratch() {
        assert!(
            toom3_scratch_needed(64, TOOM3_BASE_THRESHOLD) <= TOOM3_SCRATCH_LIMBS,
            "scratch too small for n=64 threshold={}",
            TOOM3_BASE_THRESHOLD,
        );
        let a = fill(64, 0xABCD_EF01_2345_6789);
        let b = fill(64, 0xFEDC_BA98_7654_3210);
        let expected = schoolbook_ref(&a, &b);
        let mut got = vec![0u64; 128];
        mul_toom3(&a, &b, &mut got);
        assert_eq!(got, expected, "n=64 fixed-scratch mismatch");
    }
}
