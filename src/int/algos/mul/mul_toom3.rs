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
//!
//! CLEANROOM: implemented from the published mathematical descriptions only.
//! No GPL/LGPL source (GMP, MPFR, FLINT) was consulted or adapted.
//!
//! # Algorithm
//!
//! Given n-limb operands a and b, let k = ceil(n/3) and split:
//!   a = a0 + a1*B + a2*B^2,  b = b0 + b1*B + b2*B^2,  B = 2^(64*k).
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
//! - ONE generic kernel (no per-tier copies, no per-limb-type duplication;
//!   the Limb-generic wrapper is a step-2 concern for the policy-mapper).
//! - Non-allocating: fixed [u64; TOOM3_SCRATCH_LIMBS] stack buffer;
//!   recursion carves disjoint windows via split_at_mut.
//! - Base-case: below TOOM3_BASE_THRESHOLD limbs falls to mul_schoolbook.
//! - NOT wired into any policy: a kept alternative awaiting step-2 mapping.

use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{add_assign, sub_assign, cmp_cross};

// ---- scratch sizing --------------------------------------------------------

/// Fixed stack scratch for the Toom-3 kernel, in u64 limbs.
///
/// Per level on n limbs (k = ceil(n/3)+1):
///   5 a-eval + 5 b-eval buffers of k limbs each = 10k
///   5 product buffers of 2k limbs = 10k
///   3 working temp buffers of 2k limbs = 6k
///   Total per level = 26k.
/// For n=64, threshold=9: level-0 k=23 -> 598; level-1 k<=9 -> base. 1024 is ample.
pub(crate) const TOOM3_SCRATCH_LIMBS: usize = 1024;

/// Minimum operand width (u64 limbs) below which Toom-3 falls to schoolbook.
/// Must be >= 3.
pub(crate) const TOOM3_BASE_THRESHOLD: usize = 9;

/// Upper bound on scratch (u64 limbs) for toom3_rec(n, threshold).
pub(crate) const fn toom3_scratch_needed(n: usize, threshold: usize) -> usize {
    if n < threshold { return 0; }
    let k = (n + 2) / 3 + 1;
    26 * k + toom3_scratch_needed(k, threshold)
}

// ---- public entry points ---------------------------------------------------

/// Non-allocating Toom-Cook 3-way multiply: out = a * b.
/// a and b must have equal length. out.len() >= 2*a.len() and out must be zeroed.
pub(crate) fn mul_toom3(a: &[u64], b: &[u64], out: &mut [u64]) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        toom3_scratch_needed(a.len(), TOOM3_BASE_THRESHOLD) <= TOOM3_SCRATCH_LIMBS,
        "Toom-3 scratch overflow: n={} needs {} limbs, have {}",
        a.len(), toom3_scratch_needed(a.len(), TOOM3_BASE_THRESHOLD), TOOM3_SCRATCH_LIMBS,
    );
    let mut scratch = [0u64; TOOM3_SCRATCH_LIMBS];
    toom3_rec(a, b, out, &mut scratch, TOOM3_BASE_THRESHOLD);
}

/// Test-only entry at a configurable threshold for correctness sweeps.
#[cfg(test)]
pub(crate) fn mul_toom3_with_threshold(
    a: &[u64], b: &[u64], out: &mut [u64], threshold: usize,
) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let need = toom3_scratch_needed(a.len(), threshold);
    let mut scratch = vec![0u64; need.max(1)];
    toom3_rec(a, b, out, &mut scratch, threshold);
}

// ---- core recursion --------------------------------------------------------

/// One Toom-3 recursion level. out is pre-zeroed; scratch is the live tail.
fn toom3_rec(
    a: &[u64], b: &[u64], out: &mut [u64], scratch: &mut [u64], threshold: usize,
) {
    let n = a.len();
    debug_assert_eq!(n, b.len());
    debug_assert!(out.len() >= 2 * n);
    debug_assert!(threshold >= 3);

    if n < threshold {
        mul_schoolbook(a, b, out);
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

    let ew = k + 1;   // eval-buffer width (k+1 for carry headroom)
    let pw = 2 * ew;  // product-buffer width

    // Carve level-local scratch (26*(k+1) limbs max):
    //   pa0..pai: 5*ew (a eval points); pb0..pbi: 5*ew (b eval points)
    //   w0..wi: 5*pw (sub-products); t1, t2, tmp: 3*pw (interp + shift)
    let level_need = 10 * ew + 8 * pw;
    let (level_buf, child_scratch) = scratch.split_at_mut(level_need);
    for v in level_buf.iter_mut() { *v = 0; } // zero level buf (scratch may be dirty from prior calls)

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
    let (w0,  r) = r.split_at_mut(pw);
    let (w1,  r) = r.split_at_mut(pw);
    let (wm,  r) = r.split_at_mut(pw);
    let (w2,  r) = r.split_at_mut(pw);
    let (wi,  r) = r.split_at_mut(pw);
    let (t1,  r) = r.split_at_mut(pw);
    let (t2,  r) = r.split_at_mut(pw);
    let (tmp, _) = r.split_at_mut(pw);

    // ---- evaluate a at {0, 1, -1, 2, inf} ---------------------------------
    pa0[..a0.len()].copy_from_slice(a0);
    pai[..a2.len()].copy_from_slice(a2);

    // a(1) = a0 + a1 + a2
    pa1[..a0.len()].copy_from_slice(a0);
    let _ = add_assign(pa1, a1);
    let _ = add_assign(pa1, a2);

    // a(-1) = |a0 + a2 - a1|, sign in am_neg
    pam[..a0.len()].copy_from_slice(a0);
    let _ = add_assign(pam, a2);
    let am_neg = signed_sub(pam, a1);

    // a(2) = a0 + 2*a1 + 4*a2
    pa2[..a0.len()].copy_from_slice(a0);
    let _ = add_assign(pa2, a1);
    let _ = add_assign(pa2, a1); // pa2 = a0 + 2*a1
    tmp[..a2.len()].copy_from_slice(a2);
    shl_inplace(&mut tmp[..ew], 2); // 4*a2
    let _ = add_assign(pa2, &tmp[..ew]);

    // ---- evaluate b at {0, 1, -1, 2, inf} ---------------------------------
    pb0[..b0.len()].copy_from_slice(b0);
    pbi[..b2.len()].copy_from_slice(b2);

    pb1[..b0.len()].copy_from_slice(b0);
    let _ = add_assign(pb1, b1);
    let _ = add_assign(pb1, b2);

    pbm[..b0.len()].copy_from_slice(b0);
    let _ = add_assign(pbm, b2);
    let bm_neg = signed_sub(pbm, b1);

    pb2[..b0.len()].copy_from_slice(b0);
    let _ = add_assign(pb2, b1);
    let _ = add_assign(pb2, b1); // pb2 = b0 + 2*b1
    for v in tmp.iter_mut() { *v = 0; }
    tmp[..b2.len()].copy_from_slice(b2);
    shl_inplace(&mut tmp[..ew], 2); // 4*b2
    let _ = add_assign(pb2, &tmp[..ew]);

    // ---- 5 pointwise sub-products -----------------------------------------
    for v in w0.iter_mut() { *v = 0; }
    for v in w1.iter_mut() { *v = 0; }
    for v in wm.iter_mut() { *v = 0; }
    for v in w2.iter_mut() { *v = 0; }
    for v in wi.iter_mut() { *v = 0; }

    toom3_rec(pa0, pb0, w0, child_scratch, threshold);
    toom3_rec(pa1, pb1, w1, child_scratch, threshold);
    toom3_rec(pam, pbm, wm, child_scratch, threshold);
    toom3_rec(pa2, pb2, w2, child_scratch, threshold);
    toom3_rec(pai, pbi, wi, child_scratch, threshold);

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
        let _ = add_assign(t1, wm);
        t1_neg = false;
        t2[..pw].copy_from_slice(w1);
        t2_neg = signed_sub(t2, wm); // w1 >= wm since (w1-rm)/2=c1+c3 >= 0
    } else {
        // rm < 0: t1_before_halve = w1-|wm| >= 0; t2_before_halve = w1+|wm| >= 0
        t1[..pw].copy_from_slice(w1);
        t1_neg = signed_sub(t1, wm); // should be false for unsigned inputs
        t2[..pw].copy_from_slice(w1);
        let _ = add_assign(t2, wm);
        t2_neg = false;
    }
    shr1(t1); // t1 = (w1+rm)/2
    shr1(t2); // t2 = (w1-rm)/2
    // t1 -= c0 + c4  =>  t1 = c2
    let t1_neg = signed_sub_signed(t1, t1_neg, w0, false);
    let t1_neg = signed_sub_signed(t1, t1_neg, wi, false);
    // t2 = c1+c3; t1 = c2; both >= 0.
    debug_assert!(!t1_neg, "c2 (t1) negative -- interpolation error (step A)");
    debug_assert!(!t2_neg, "t2=c1+c3 negative -- interpolation error (step A)");

    // Step B: s = (w2 - w0) / 2 - 2*c2 - 8*c4  [= c1+4*c3 >= 0]
    // w2 - w0 = 2c1+4c2+8c3+16c4 >= 0 (always, for unsigned).
    // (w2-w0)/2 = c1+2*c2+4*c3+8*c4. Subtract 2*c2 and 8*c4: c1+4*c3 >= 0.
    // Re-use w2 for s (it is not needed after this).
    let went_neg = signed_sub(w2, w0);
    debug_assert!(!went_neg, "Toom-3: r2 < r0 -- invariant violated for unsigned inputs");
    shr1(w2); // w2 = (w2-w0)/2 = c1+2*c2+4*c3+8*c4
    // subtract 2*c2 (= 2*t1):
    tmp[..pw].copy_from_slice(t1); // t1 = c2
    shl_inplace(tmp, 1);           // tmp = 2*c2
    let s_neg = signed_sub_signed(w2, false, tmp, t1_neg);
    // subtract 8*c4 (= 8*wi):
    tmp[..pw].copy_from_slice(wi);
    shl_inplace(tmp, 3);           // tmp = 8*wi = 8*c4
    let s_neg = signed_sub_signed(w2, s_neg, tmp, false);
    // w2 = s = c1+4*c3 >= 0
    debug_assert!(!s_neg, "s=c1+4c3 negative -- interpolation error (step B)");

    // Step C: c3 = (s - t2) / 3
    // s - t2 = (c1+4*c3) - (c1+c3) = 3*c3 >= 0 (exact division).
    // Re-use tmp for c3 (avoid aliasing: s is in w2, t2 is in t2 buf).
    tmp[..pw].copy_from_slice(w2); // tmp = s
    let c3_neg = signed_sub_signed(tmp, s_neg, t2, t2_neg); // tmp = s - t2 = 3*c3
    debug_assert!(!c3_neg, "3*c3 negative -- interpolation error (step C)");
    let c3_neg = div3_signed(tmp, c3_neg); // tmp = c3

    // Step D: c1 = t2 - c3  [= c1+c3 - c3 = c1 >= 0]
    // Re-use w2 for c1.
    w2[..pw].copy_from_slice(t2); // w2 = c1+c3
    let c1_neg = signed_sub_signed(w2, t2_neg, tmp, c3_neg); // w2 = c1
    debug_assert!(!c1_neg, "c1 negative -- interpolation error (step D)");

    // Now: w0=c0, w2=c1, t1=c2 (t1_neg), tmp=c3 (c3_neg), wi=c4.
    // All should be non-negative.
    debug_assert!(!t1_neg, "c2 negative -- final check");
    debug_assert!(!c3_neg, "c3 negative -- final check");

    // ---- recombine: out = c0 + c1*B + c2*B^2 + c3*B^3 + c4*B^4 ----------
    // c0=w0, c1=w2, c2=t1, c3=tmp, c4=wi (all >= 0 per the debug_asserts above).
    add_into_out(out, 0,       w0);  // c0
    add_into_out(out, k,       w2);  // c1 (stored in w2 buffer)
    add_into_out(out, 2 * k,   t1);  // c2
    add_into_out(out, 3 * k,   tmp); // c3 (stored in tmp buffer)
    add_into_out(out, 4 * k,   wi);  // c4
}



// ---- helpers ---------------------------------------------------------------

/// Subtract src from dst (unsigned magnitudes).
/// Returns true when src > dst; stores src-dst in dst (result is negative).
fn signed_sub(dst: &mut [u64], src: &[u64]) -> bool {
    if cmp_cross(dst, src) >= 0 {
        let _ = sub_assign(dst, src);
        false
    } else {
        negate(dst);
        let _ = add_assign(dst, src);
        true
    }
}

/// Signed subtraction: (a, a_neg) -= (b, b_neg). Returns new sign.
fn signed_sub_signed(a: &mut [u64], a_neg: bool, b: &[u64], b_neg: bool) -> bool {
    signed_add_signed(a, a_neg, b, !b_neg)
}

/// Signed addition: (a, a_neg) += (b, b_neg). Returns new sign.
fn signed_add_signed(a: &mut [u64], a_neg: bool, b: &[u64], b_neg: bool) -> bool {
    if a_neg == b_neg {
        let _ = add_assign(a, b);
        a_neg
    } else if cmp_cross(a, b) >= 0 {
        let _ = sub_assign(a, b);
        a_neg
    } else {
        negate(a);
        let _ = add_assign(a, b);
        b_neg
    }
}

/// Two's complement negation in place (little-endian u64).
#[inline]
fn negate(a: &mut [u64]) {
    let mut carry: u64 = 1;
    for limb in a.iter_mut() {
        let (v, c) = (!*limb).overflowing_add(carry);
        *limb = v;
        carry = c as u64;
    }
}

/// In-place left shift by shift bits (0 <= shift < 64). Carry out dropped.
#[inline]
fn shl_inplace(a: &mut [u64], shift: u32) {
    if shift == 0 { return; }
    let rshift = 64 - shift;
    let mut carry: u64 = 0;
    for limb in a.iter_mut() {
        let out_carry = *limb >> rshift;
        *limb = (*limb << shift) | carry;
        carry = out_carry;
    }
}

/// In-place right shift by 1 (value must be even for correct interpolation).
#[inline]
fn shr1(a: &mut [u64]) {
    let mut carry: u64 = 0;
    for limb in a.iter_mut().rev() {
        let new_carry = (*limb & 1) << 63;
        *limb = (*limb >> 1) | carry;
        carry = new_carry;
    }
    debug_assert_eq!(carry, 0, "shr1: odd value -- must be even for Toom-3");
}

/// Exact division by 3 (little-endian unsigned). Caller guarantees divisibility.
fn div3_signed(a: &mut [u64], neg: bool) -> bool {
    let mut rem: u64 = 0;
    for limb in a.iter_mut().rev() {
        let cur = ((rem as u128) << 64) | (*limb as u128);
        *limb = (cur / 3) as u64;
        rem = (cur % 3) as u64;
    }
    debug_assert_eq!(rem, 0, "div3: not divisible by 3 -- Toom-3 interpolation error");
    neg
}

/// Add src into out[offset..] with carry propagation.
#[inline]
fn add_into_out(out: &mut [u64], offset: usize, src: &[u64]) {
    if offset < out.len() {
        let _ = add_assign(&mut out[offset..], src);
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

    /// Bit-identity against schoolbook: N in 3..=64, thresholds {3,6,9},
    /// edge operands and random operands. PASS = correctness wall satisfied.
    #[test]
    fn toom3_bit_identical_to_schoolbook() {
        const WIDTHS: &[usize] = &[
            3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 18,
            21, 24, 27, 32, 33, 36, 48, 51, 60, 63, 64,
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
                        // commutativity
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
