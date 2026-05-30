// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Non-allocating recursive Karatsuba multiplication.
//!
//! Reference: Karatsuba & Ofman 1962, Doklady Akad. Nauk SSSR 145, 293-294.
//! One named algorithm; the schoolbook-vs-Karatsuba choice lives in
//! crate::int::policy::mul.
//!
//! Two entry forms share ONE generic recursion body (karatsuba_rec_limb):
//! - u64 base: operands and scratch in [u64]. Entry via mul_karatsuba /
//!   mul_karatsuba_forced. Unchanged behaviour.
//! - u128 base: packs n u64 input limbs into n/2 u128 limbs, runs the
//!   identical split/recombine in u128 space (half the limb count and
//!   carry-chain depth per step), then unpacks. Entry via
//!   mul_karatsuba_limb (bench-alt only).
//!
//! The ALGORITHM is ONE: karatsuba_rec_limb is the single generic
//! recursion, instantiated at u64 or u128. There is no per-limb-type
//! copy (rule 2 of the architecture constitution). The choice of which
//! limb width wins per (N, SCALE) cell is the policy-mapper s job.

use crate::int::algos::support::limbs::{add_assign, sub_assign};
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::types::compute_limbs::Limb;

// ---- Scratch size constants -------------------------------------------------

/// Stack scratch for the u64 Karatsuba kernel, in u64 limbs.
///
/// K(n) <= 12n + O(log n) per the geometric recursion. For n = 256
/// (Int<256>, the widest tier) that is ~3072; rounded up with headroom.
pub(crate) const KARATSUBA_SCRATCH_LIMBS: usize = 3200;

/// Stack scratch for the u128-packed Karatsuba, in u128 limbs.
///
/// The u128 variant operates on h = n/2 u128 limbs, so scratch is
/// karatsuba_scratch_needed_th(n/2, th) u128 limbs -- half of
/// KARATSUBA_SCRATCH_LIMBS by the same geometric argument.
pub(crate) const KARATSUBA_SCRATCH_U128: usize = KARATSUBA_SCRATCH_LIMBS / 2 + 8;

// ---- u64 entry points (existing slice-based interface, unchanged) -----------

/// Non-allocating recursive Karatsuba multiplication at u64 base.
/// out.len() >= 2 * a.len(), out must be zeroed by the caller.
pub(crate) fn mul_karatsuba(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        karatsuba_scratch_needed_th(a.len(), threshold) <= KARATSUBA_SCRATCH_LIMBS,
        "Karatsuba scratch overflow: n={} needs {} limbs, have {}",
        a.len(),
        karatsuba_scratch_needed_th(a.len(), threshold),
        KARATSUBA_SCRATCH_LIMBS,
    );
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Bench-only u64 Karatsuba at an arbitrary threshold. out zeroed here.
#[cfg(feature = "bench-alt")]
pub(crate) fn mul_karatsuba_forced(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        karatsuba_scratch_needed_th(a.len(), threshold) <= KARATSUBA_SCRATCH_LIMBS,
        "Karatsuba scratch overflow in forced bench entry"
    );
    for o in out.iter_mut() { *o = 0; }
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Test-only entry at an arbitrary threshold (allocates scratch).
#[cfg(test)]
pub(crate) fn mul_karatsuba_with_threshold(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    threshold: usize,
) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let need = karatsuba_scratch_needed_th(a.len(), threshold);
    let mut scratch = vec![0u64; need];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}
// ---- Limb-generic entry point (bench-alt only) ------------------------------

/// Limb-generic Karatsuba full product -- bench-alt entry.
///
/// Packs N u64 operand limbs into L limbs (N for L=u64, N/2 for L=u128),
/// runs ONE generic karatsuba_rec_limb, then unpacks the 2*N-u64 product.
///
/// For L = u64 the result is numerically identical to mul_karatsuba (same
/// algorithm, same values). For L = u128 (requires even N) the identical
/// split/recombine runs in u128 space: half the limb count, half the
/// carry-chain depth at every inner step.
///
/// threshold is in u64-limb units; converted to packed units internally.
/// out is written in full by this function (the unpack overwrites it).
///
/// Production kernel: `int::policy::mul` routes even `N >= KARATSUBA_ENGAGE` to
/// `mul_karatsuba_limb::<N, u128>` — the policy-map showed it beats schoolbook-
/// u128 by ~1.34x (N=128) .. 1.39x (N=256) at recursion threshold 48.
pub(crate) fn mul_karatsuba_limb<const N: usize, L: Limb>(
    a: &[u64; N],
    b: &[u64; N],
    out: &mut [u64],
    threshold: usize,
) {
    let h = L::packed_len(N);
    debug_assert!(h > 0 && h <= N);
    // Pack operands into L-space. [L; N] is always >= packed_len(N) <= N.
    let mut ap = [L::ZERO; N];
    let mut bp = [L::ZERO; N];
    L::pack(a, &mut ap[..h]);
    L::pack(b, &mut bp[..h]);

    // Convert threshold from u64-limb to packed-limb units.
    // For u128: packed_len = N/2 so ratio=2, threshold_packed = threshold/2.
    // For u64:  packed_len = N  so ratio=1, threshold_packed = threshold.
    // max(., 4) preserves the recursion floor.
    let ratio: usize = if h < N { 2 } else { 1 };
    let threshold_packed = (threshold / ratio).max(4);

    // Product buffer: 2*h packed-limb slots.
    // [L; 2*N] covers both limb types (u64: 2h=2N; u128: 2h=N, rest unused).
    // Scratch: KARATSUBA_SCRATCH_U128 slots, enough for u128 at N=256 and
    // also enough for u64 with h <= N/2 (identical scratch geometry).
    // [L; KARATSUBA_SCRATCH_U128] is 1608 limbs, >> 2*h for any benched N (max 2*64=128).
    let mut prod = [L::ZERO; KARATSUBA_SCRATCH_U128];
    let mut scratch = [L::ZERO; KARATSUBA_SCRATCH_U128];

    karatsuba_rec_limb::<L>(
        &ap[..h],
        &bp[..h],
        &mut prod[..2 * h],
        &mut scratch,
        threshold_packed,
    );

    L::unpack(&prod[..2 * h], &mut out[..2 * N]);
}

// ---- Scratch sizing ---------------------------------------------------------

/// Upper bound on scratch (in typed limbs) for n-limb Karatsuba at the
/// given threshold.
pub(crate) const fn karatsuba_scratch_needed_th(n: usize, threshold: usize) -> usize {
    if n < threshold {
        return 0;
    }
    let h = n / 2;
    let hi = n - h;
    let level = 2 * h + 2 * hi + (hi + 1) + (hi + 1) + 2 * (hi + 1);
    level + karatsuba_scratch_needed_th(hi + 1, threshold)
}

// ---- u64-slice recursion (unchanged) ----------------------------------------

fn karatsuba_rec(a: &[u64], b: &[u64], out: &mut [u64], scratch: &mut [u64], threshold: usize) {
    debug_assert!(threshold >= 4, "Karatsuba threshold must be >= 4 to terminate");
    let n = a.len();
    if n < threshold {
        mul_schoolbook(a, b, out);
        return;
    }
    let h = n / 2;
    let hi = n - h;
    let (a_lo, a_hi) = a.split_at(h);
    let (b_lo, b_hi) = b.split_at(h);

    let (z0, rest) = scratch.split_at_mut(2 * h);
    let (z2, rest) = rest.split_at_mut(2 * hi);
    let (sa, rest) = rest.split_at_mut(hi + 1);
    let (sb, rest) = rest.split_at_mut(hi + 1);
    let (z1, tail) = rest.split_at_mut(2 * (hi + 1));

    for v in z0.iter_mut() { *v = 0; }
    for v in z2.iter_mut() { *v = 0; }
    for v in z1.iter_mut() { *v = 0; }

    karatsuba_rec(a_lo, b_lo, z0, tail, threshold);
    karatsuba_rec_unbalanced(a_hi, b_hi, z2, tail, threshold);

    for v in sa.iter_mut() { *v = 0; }
    for v in sb.iter_mut() { *v = 0; }
    sa[..h].copy_from_slice(a_lo);
    sb[..h].copy_from_slice(b_lo);
    let _ = add_assign(sa, a_hi);
    let _ = add_assign(sb, b_hi);

    karatsuba_rec_unbalanced(sa, sb, z1, tail, threshold);
    let _ = sub_assign(z1, z0);
    let _ = sub_assign(z1, z2);

    out[..z0.len()].copy_from_slice(z0);
    let _ = add_assign(&mut out[2 * h..], z2);
    let _ = add_assign(&mut out[h..], z1);
}

fn karatsuba_rec_unbalanced(
    a: &[u64],
    b: &[u64],
    out: &mut [u64],
    scratch: &mut [u64],
    threshold: usize,
) {
    debug_assert_eq!(a.len(), b.len());
    if a.len() >= threshold {
        karatsuba_rec(a, b, out, scratch, threshold);
    } else {
        for v in out.iter_mut() { *v = 0; }
        mul_schoolbook(a, b, out);
    }
}
// ---- Limb-generic recursion: ONE kernel body for both u64 and u128 ----------

/// Limb-generic schoolbook base case. out must be pre-zeroed by the caller.
/// Same outer-product algorithm as mul_schoolbook, lifted to L space via
/// Limb::widening_mul / overflowing_add / add_carries primitives.
/// ONE function body, no per-limb-type copy.
///
/// `pub(crate)` because the Toom-3 kernel (`mul_toom3`) reuses it as its own
/// Limb-generic base case — ONE shared L-space schoolbook, no duplicate
/// (Constitution rule 2). (A future tidy could relocate this + the two
/// `limb_{add,sub}_assign` helpers to `mul_schoolbook.rs` beside `mul_low_limb`.)
#[inline]
pub(crate) fn schoolbook_rec_limb<L: Limb>(a: &[L], b: &[L], out: &mut [L]) {
    let na = a.len();
    let nb = b.len();
    let mut i = 0;
    while i < na {
        let ai = a[i];
        if ai != L::ZERO {
            let mut carry = L::ZERO;
            let mut j = 0;
            while j < nb {
                let (lo, hi) = ai.widening_mul(b[j]);
                let idx = i + j;
                let (s1, c1) = out[idx].overflowing_add(lo);
                let (s2, c2) = s1.overflowing_add(carry);
                out[idx] = s2;
                carry = hi.add_carries(c1, c2);
                j += 1;
            }
            let mut idx = i + nb;
            while carry != L::ZERO && idx < out.len() {
                let (s, c) = out[idx].overflowing_add(carry);
                out[idx] = s;
                carry = if c { L::ONE } else { L::ZERO };
                idx += 1;
            }
        }
        i += 1;
    }
}

/// Limb-generic add-assign: a += b, returns carry. a.len() >= b.len().
/// Used for sum-formation and recombine in L space. `pub(crate)`: shared
/// with the Toom-3 kernel (one L-space add, no duplicate — rule 2).
#[inline]
pub(crate) fn limb_add_assign<L: Limb>(a: &mut [L], b: &[L]) -> bool {
    let mut carry = false;
    let mut i = 0;
    while i < a.len() {
        let bv = if i < b.len() { b[i] } else { L::ZERO };
        let (s1, c1) = a[i].overflowing_add(bv);
        let (s2, c2) = s1.overflowing_add(if carry { L::ONE } else { L::ZERO });
        a[i] = s2;
        carry = c1 | c2;
        i += 1;
    }
    carry
}

/// Limb-generic sub-assign: a -= b, returns borrow. a.len() >= b.len().
/// Used for z1 formation (z1 -= z0; z1 -= z2) in L space. `pub(crate)`:
/// shared with the Toom-3 kernel (one L-space sub, no duplicate — rule 2).
#[inline]
pub(crate) fn limb_sub_assign<L: Limb>(a: &mut [L], b: &[L]) -> bool {
    let mut borrow = false;
    let mut i = 0;
    while i < a.len() {
        let bv = if i < b.len() { b[i] } else { L::ZERO };
        let (d1, b1) = a[i].overflowing_sub(bv);
        let (d2, b2) = d1.overflowing_sub(if borrow { L::ONE } else { L::ZERO });
        a[i] = d2;
        borrow = b1 | b2;
        i += 1;
    }
    borrow
}

/// Limb-generic child dispatch: routes to karatsuba_rec_limb above the
/// threshold or schoolbook_rec_limb below.
fn karatsuba_rec_limb_unbalanced<L: Limb>(
    a: &[L],
    b: &[L],
    out: &mut [L],
    scratch: &mut [L],
    threshold: usize,
) {
    debug_assert_eq!(a.len(), b.len());
    if a.len() >= threshold {
        karatsuba_rec_limb::<L>(a, b, out, scratch, threshold);
    } else {
        for v in out.iter_mut() { *v = L::ZERO; }
        schoolbook_rec_limb::<L>(a, b, out);
    }
}

/// ONE generic Karatsuba recursion level in L space.
///
/// Identical split/recombine algebra as karatsuba_rec, lifted to the
/// generic L: Limb type. For L = u64 numerically identical to
/// karatsuba_rec; for L = u128 runs in n/2 u128 limbs, halving the
/// carry-chain depth per inner step. ONE body, no per-limb-type copy.
///
/// out must be pre-zeroed for the 2*n-limb window.
fn karatsuba_rec_limb<L: Limb>(
    a: &[L],
    b: &[L],
    out: &mut [L],
    scratch: &mut [L],
    threshold: usize,
) {
    debug_assert!(threshold >= 4);
    let n = a.len();
    if n < threshold {
        schoolbook_rec_limb::<L>(a, b, out);
        return;
    }
    let h = n / 2;
    let hi = n - h;

    let (a_lo, a_hi) = a.split_at(h);
    let (b_lo, b_hi) = b.split_at(h);

    let (z0, rest) = scratch.split_at_mut(2 * h);
    let (z2, rest) = rest.split_at_mut(2 * hi);
    let (sa, rest) = rest.split_at_mut(hi + 1);
    let (sb, rest) = rest.split_at_mut(hi + 1);
    let (z1, tail) = rest.split_at_mut(2 * (hi + 1));

    for v in z0.iter_mut() { *v = L::ZERO; }
    for v in z2.iter_mut() { *v = L::ZERO; }
    for v in z1.iter_mut() { *v = L::ZERO; }

    karatsuba_rec_limb::<L>(a_lo, b_lo, z0, tail, threshold);
    karatsuba_rec_limb_unbalanced::<L>(a_hi, b_hi, z2, tail, threshold);

    for v in sa.iter_mut() { *v = L::ZERO; }
    for v in sb.iter_mut() { *v = L::ZERO; }
    sa[..h].copy_from_slice(a_lo);
    sb[..h].copy_from_slice(b_lo);
    let _ = limb_add_assign::<L>(sa, a_hi);
    let _ = limb_add_assign::<L>(sb, b_hi);

    karatsuba_rec_limb_unbalanced::<L>(sa, sb, z1, tail, threshold);
    let _ = limb_sub_assign::<L>(z1, z0);
    let _ = limb_sub_assign::<L>(z1, z2);

    out[..z0.len()].copy_from_slice(z0);
    let _ = limb_add_assign::<L>(&mut out[2 * h..], z2);
    let _ = limb_add_assign::<L>(&mut out[h..], z1);
}