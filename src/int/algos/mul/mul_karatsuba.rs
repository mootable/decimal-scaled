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
//! limb width wins per (N, SCALE) cell is the policy matcher's job.

use crate::int::algos::support::limbs::{add_assign, sub_assign, MAX_WORK_N};
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::types::compute_limbs::{ComputeLimbs, Limb, Limbs};

// ---- Slice-entry scratch (width-erased build-max) ---------------------------

/// Ceiling on the operand length the WIDTH-ERASED slice entries
/// (`mul_karatsuba` / `mul_karatsuba_forced`) will run Karatsuba for; past it
/// they fall back to schoolbook.
///
/// DERIVED from `MAX_WORK_N`, not frozen. The widest work integer any tier
/// declares is its `Wexp`, which is `8 x storage` at the top of every feature
/// band (`wide` 16 -> `Int<128>` at D307; `x-wide` 32 -> `Int<256>` at D616;
/// `xx-wide` 64 -> `Int<512>` at D1232). Deriving it means adding a tier
/// resizes this automatically instead of leaving two constants to be kept in
/// step by hand -- and it stops a narrow build carrying a frame sized for a
/// width it can never present.
///
/// It bounds ONLY the slice door. The const-`N` door
/// (`int::policy::mul::dispatch` -> `mul_karatsuba_limb`) sources EXACT per-`N`
/// scratch from `ComputeLimbs`, so no `Int<N>` operand of any width -- D1232's
/// `Wexp = Int<512>` included -- can reach this constant. That asymmetry is
/// what makes the hazard tractable: only a runtime-length caller is exposed,
/// and only the entry check in `mul_karatsuba` can bound a runtime length.
///
/// (The previous doc here claimed a wider operand "would route through the
/// concrete-N kernel's exact ComputeLimbs scratch, not this slice path". That
/// is true of the const door and CANNOT be true of the slice door, which has
/// no `N` to route by. What protects the slice door is a property of its call
/// sites, not of routing.)
///
/// Every slice caller (`sqrt_newton`, `cbrt_newton`, `barrett_reciprocal`,
/// `div_widen_scale`, `wide_trig_core`) carves operands from storage-derived
/// `ComputeLimbs` buffers and shrinks them with `sig_len`, so the longest
/// equal-length pair any can present today is about **96** limbs (`cbrt` at
/// D1232) -- below the matcher's engage point, leaving the slice Karatsuba arm
/// unreached in production.
///
/// That 96 is a MEASUREMENT OF TODAY'S CALL SITES, NOT A PROPERTY of the code.
/// A new slice caller, or a work width fed here as a value rather than as a
/// widened storage magnitude, moves it without anything complaining -- which
/// is why the entry fails closed rather than trusting the number.
pub(crate) const KARATSUBA_MAX_WIDTH: usize = 8 * MAX_WORK_N;

/// Build-max stack scratch (in u64 limbs) for the WIDTH-ERASED slice Karatsuba
/// entries mul_karatsuba / mul_karatsuba_forced. Those take &[u64] of RUNTIME
/// length, so they cannot size per-N and use this sanctioned build-max blanket
/// (like the width-erased slice-divide engines). It is sized to the deepest
/// recursion (the threshold floor 4) at [`KARATSUBA_MAX_WIDTH`] via the
/// kernel's own recursion arithmetic -- derived, not a frozen guess, and now
/// feature-scoped through that ceiling rather than pinned to the widest
/// tier the crate can be built with. The concrete-N kernel
/// mul_karatsuba_limb sources its EXACT per-N scratch from ComputeLimbs instead
/// (no build-max on the hot wide-multiply path -- Constitution rule 6).
pub(crate) const KARATSUBA_SCRATCH_LIMBS: usize =
    karatsuba_scratch_needed_th(KARATSUBA_MAX_WIDTH, 4);

// ---- u64 entry points (existing slice-based interface, unchanged) -----------

/// Non-allocating recursive Karatsuba multiplication at u64 base.
/// out.len() >= 2 * lhs.len(), out must be zeroed by the caller.
pub(crate) fn mul_karatsuba(lhs: &[u64], rhs: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(lhs.len(), rhs.len());
    debug_assert!(out.len() >= 2 * lhs.len());
    // FAIL CLOSED. This entry is width-erased, so `lhs.len()` is a RUNTIME
    // length that no const assertion can bound -- the scratch below is a
    // fixed build-max frame. An operand past the ceiling takes the
    // schoolbook path (slower, same product) instead of overrunning the
    // scratch, which `karatsuba_rec` would otherwise turn into a
    // `split_at_mut` panic in release as well as debug.
    //
    // Sizing argument: `KARATSUBA_SCRATCH_LIMBS` covers `KARATSUBA_MAX_WIDTH`
    // at the threshold FLOOR (4), and `karatsuba_scratch_needed_th` is
    // non-decreasing in `limb_count` and non-increasing in `threshold` -- so it
    // covers every `(limb_count <= ceiling, threshold >= 4)` caller, and
    // `lhs.len()` alone is a sufficient test. ONE compare against a const at
    // the entry; the recursion is untouched.
    if lhs.len() > KARATSUBA_MAX_WIDTH {
        mul_schoolbook(lhs, rhs, out);
        return;
    }
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(lhs, rhs, out, &mut scratch, threshold);
}

/// Bench-only u64 Karatsuba at an arbitrary threshold. out zeroed here.
#[cfg(feature = "bench-alt")]
pub(crate) fn mul_karatsuba_forced(lhs: &[u64], rhs: &[u64], out: &mut [u64],
    threshold: usize) {
    debug_assert_eq!(lhs.len(), rhs.len());
    debug_assert!(out.len() >= 2 * lhs.len());
    for slot in out.iter_mut() { *slot = 0; }
    // Same fail-closed ceiling as `mul_karatsuba` -- see the sizing argument
    // there. `out` is already zeroed above, which is the schoolbook contract.
    if lhs.len() > KARATSUBA_MAX_WIDTH {
        mul_schoolbook(lhs, rhs, out);
        return;
    }
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(lhs, rhs, out, &mut scratch, threshold);
}

/// Test-only entry at an arbitrary threshold (allocates scratch).
#[cfg(test)]
pub(crate) fn mul_karatsuba_with_threshold(
    lhs: &[u64],
    rhs: &[u64],
    out: &mut [u64],
    threshold: usize,
) {
    debug_assert_eq!(lhs.len(), rhs.len());
    debug_assert!(out.len() >= 2 * lhs.len());
    let scratch_needed = karatsuba_scratch_needed_th(lhs.len(), threshold);
    let mut scratch = vec![0u64; scratch_needed];
    karatsuba_rec(lhs, rhs, out, &mut scratch, threshold);
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
    lhs: &[u64; N],
    rhs: &[u64; N],
    out: &mut [u64],
    threshold: usize,
) where
    Limbs<N>: ComputeLimbs,
{
    let h = L::packed_len(N);
    debug_assert!(h > 0 && h <= N);
    // Pack operands into L-space. [L; N] is always >= packed_len(N) <= N.
    let mut lhs_packed = [L::ZERO; N];
    let mut rhs_packed = [L::ZERO; N];
    L::pack(lhs, &mut lhs_packed[..h]);
    L::pack(rhs, &mut rhs_packed[..h]);

    // Convert threshold from u64-limb to packed-limb units.
    // For u128: packed_len = N/2 so ratio=2, threshold_packed = threshold/2.
    // For u64:  packed_len = N  so ratio=1, threshold_packed = threshold.
    // max(., 4) preserves the recursion floor.
    let ratio: usize = if h < N { 2 } else { 1 };
    let threshold_packed = (threshold / ratio).max(4);

    // EXACT per-N Karatsuba work buffer (Constitution rule 6), sourced from
    // ComputeLimbs on Limbs<N>: ONE stack array of
    //   2*h + karatsuba_scratch_needed_th(h, 4)  L-limbs   (h = packed_len(N)),
    // carved into the 2*h-limb product window and the recursion scratch. Sizing
    // the scratch to the threshold FLOOR (4 -- the kernel's recursion base)
    // makes it the exact worst case over every caller `threshold`, so each width
    // carries ONLY its own frame -- no widest-tier blanket on this concrete-N
    // path. (`L::karatsuba` picks the u64 form (2N + scratch over N limbs) or
    // the u128 form (2h + scratch over h = N/2 limbs) for this limb type.)
    let mut work = L::karatsuba::<Limbs<N>>();
    let work = work.as_mut();
    let (prod, scratch) = work.split_at_mut(2 * h);

    debug_assert!(
        scratch.len() >= karatsuba_scratch_needed_th(h, threshold_packed),
        "Karatsuba scratch overflow: h={}, threshold_packed={}, need={}, have={}",
        h,
        threshold_packed,
        karatsuba_scratch_needed_th(h, threshold_packed),
        scratch.len(),
    );

    karatsuba_rec_limb::<L>(&lhs_packed[..h], &rhs_packed[..h], &mut *prod, scratch,
        threshold_packed);

    L::unpack(&prod[..2 * h], &mut out[..2 * N]);
}

// ---- Scratch sizing ---------------------------------------------------------

/// Upper bound on scratch (in typed limbs) for `limb_count`-limb Karatsuba at
/// the given threshold.
pub(crate) const fn karatsuba_scratch_needed_th(limb_count: usize, threshold: usize) -> usize {
    if limb_count < threshold {
        return 0;
    }
    let low_len = limb_count / 2;
    let high_len = limb_count - low_len;
    let level =
        2 * low_len + 2 * high_len + (high_len + 1) + (high_len + 1) + 2 * (high_len + 1);
    level + karatsuba_scratch_needed_th(high_len + 1, threshold)
}

// ---- u64-slice recursion (unchanged) ----------------------------------------

fn karatsuba_rec(lhs: &[u64], rhs: &[u64], out: &mut [u64], scratch: &mut [u64],
    threshold: usize) {
    debug_assert!(threshold >= 4, "Karatsuba threshold must be >= 4 to terminate");
    let limb_count = lhs.len();
    if limb_count < threshold {
        mul_schoolbook(lhs, rhs, out);
        return;
    }
    let low_len = limb_count / 2;
    let high_len = limb_count - low_len;
    let (lhs_lo, lhs_hi) = lhs.split_at(low_len);
    let (rhs_lo, rhs_hi) = rhs.split_at(low_len);

    let (z0, rest) = scratch.split_at_mut(2 * low_len);
    let (z2, rest) = rest.split_at_mut(2 * high_len);
    let (lhs_sum, rest) = rest.split_at_mut(high_len + 1);
    let (rhs_sum, rest) = rest.split_at_mut(high_len + 1);
    let (z1, tail) = rest.split_at_mut(2 * (high_len + 1));

    for slot in z0.iter_mut() { *slot = 0; }
    for slot in z2.iter_mut() { *slot = 0; }
    for slot in z1.iter_mut() { *slot = 0; }

    karatsuba_rec(lhs_lo, rhs_lo, z0, tail, threshold);
    karatsuba_rec_unbalanced(lhs_hi, rhs_hi, z2, tail, threshold);

    for slot in lhs_sum.iter_mut() { *slot = 0; }
    for slot in rhs_sum.iter_mut() { *slot = 0; }
    lhs_sum[..low_len].copy_from_slice(lhs_lo);
    rhs_sum[..low_len].copy_from_slice(rhs_lo);
    let _ = add_assign(lhs_sum, lhs_hi);
    let _ = add_assign(rhs_sum, rhs_hi);

    karatsuba_rec_unbalanced(lhs_sum, rhs_sum, z1, tail, threshold);
    let _ = sub_assign(z1, z0);
    let _ = sub_assign(z1, z2);

    out[..z0.len()].copy_from_slice(z0);
    let _ = add_assign(&mut out[2 * low_len..], z2);
    let _ = add_assign(&mut out[low_len..], z1);
}

fn karatsuba_rec_unbalanced(
    lhs: &[u64],
    rhs: &[u64],
    out: &mut [u64],
    scratch: &mut [u64],
    threshold: usize,
) {
    debug_assert_eq!(lhs.len(), rhs.len());
    if lhs.len() >= threshold {
        karatsuba_rec(lhs, rhs, out, scratch, threshold);
    } else {
        for slot in out.iter_mut() { *slot = 0; }
        mul_schoolbook(lhs, rhs, out);
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
pub(crate) fn schoolbook_rec_limb<L: Limb>(lhs: &[L], rhs: &[L], out: &mut [L]) {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let mut i = 0;
    while i < lhs_len {
        let lhs_limb = lhs[i];
        if lhs_limb != L::ZERO {
            let mut carry = L::ZERO;
            let mut j = 0;
            while j < rhs_len {
                let (lo, hi) = lhs_limb.widening_mul(rhs[j]);
                let idx = i + j;
                let (sum1, carry1) = out[idx].overflowing_add(lo);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                out[idx] = sum2;
                carry = hi.add_carries(carry1, carry2);
                j += 1;
            }
            let mut idx = i + rhs_len;
            while carry != L::ZERO && idx < out.len() {
                let (sum, carried) = out[idx].overflowing_add(carry);
                out[idx] = sum;
                carry = if carried { L::ONE } else { L::ZERO };
                idx += 1;
            }
        }
        i += 1;
    }
}

/// Limb-generic add-assign: lhs += rhs, returns carry. lhs.len() >= rhs.len().
/// Used for sum-formation and recombine in L space. `pub(crate)`: shared
/// with the Toom-3 kernel (one L-space add, no duplicate — rule 2).
#[inline]
pub(crate) fn limb_add_assign<L: Limb>(lhs: &mut [L], rhs: &[L]) -> bool {
    let mut carry = false;
    let mut i = 0;
    while i < lhs.len() {
        let rhs_limb = if i < rhs.len() { rhs[i] } else { L::ZERO };
        let (sum1, carry1) = lhs[i].overflowing_add(rhs_limb);
        let (sum2, carry2) = sum1.overflowing_add(if carry { L::ONE } else { L::ZERO });
        lhs[i] = sum2;
        carry = carry1 | carry2;
        i += 1;
    }
    carry
}

/// Limb-generic sub-assign: lhs -= rhs, returns borrow. lhs.len() >= rhs.len().
/// Used for z1 formation (z1 -= z0; z1 -= z2) in L space. `pub(crate)`:
/// shared with the Toom-3 kernel (one L-space sub, no duplicate — rule 2).
#[inline]
pub(crate) fn limb_sub_assign<L: Limb>(lhs: &mut [L], rhs: &[L]) -> bool {
    let mut borrow = false;
    let mut i = 0;
    while i < lhs.len() {
        let rhs_limb = if i < rhs.len() { rhs[i] } else { L::ZERO };
        let (diff1, borrow1) = lhs[i].overflowing_sub(rhs_limb);
        let (diff2, borrow2) =
            diff1.overflowing_sub(if borrow { L::ONE } else { L::ZERO });
        lhs[i] = diff2;
        borrow = borrow1 | borrow2;
        i += 1;
    }
    borrow
}

/// Limb-generic child dispatch: routes to karatsuba_rec_limb above the
/// threshold or schoolbook_rec_limb below.
fn karatsuba_rec_limb_unbalanced<L: Limb>(
    lhs: &[L],
    rhs: &[L],
    out: &mut [L],
    scratch: &mut [L],
    threshold: usize,
) {
    debug_assert_eq!(lhs.len(), rhs.len());
    if lhs.len() >= threshold {
        karatsuba_rec_limb::<L>(lhs, rhs, out, scratch, threshold);
    } else {
        for slot in out.iter_mut() { *slot = L::ZERO; }
        schoolbook_rec_limb::<L>(lhs, rhs, out);
    }
}

/// ONE generic Karatsuba recursion level in L space.
///
/// Identical split/recombine algebra as karatsuba_rec, lifted to the
/// generic L: Limb type. For L = u64 numerically identical to
/// karatsuba_rec; for L = u128 runs in n/2 u128 limbs, halving the
/// carry-chain depth per inner step. ONE body, no per-limb-type copy.
///
/// out must be pre-zeroed for the 2*limb_count-limb window.
fn karatsuba_rec_limb<L: Limb>(
    lhs: &[L],
    rhs: &[L],
    out: &mut [L],
    scratch: &mut [L],
    threshold: usize,
) {
    debug_assert!(threshold >= 4);
    let limb_count = lhs.len();
    if limb_count < threshold {
        schoolbook_rec_limb::<L>(lhs, rhs, out);
        return;
    }
    let low_len = limb_count / 2;
    let high_len = limb_count - low_len;

    let (lhs_lo, lhs_hi) = lhs.split_at(low_len);
    let (rhs_lo, rhs_hi) = rhs.split_at(low_len);

    let (z0, rest) = scratch.split_at_mut(2 * low_len);
    let (z2, rest) = rest.split_at_mut(2 * high_len);
    let (lhs_sum, rest) = rest.split_at_mut(high_len + 1);
    let (rhs_sum, rest) = rest.split_at_mut(high_len + 1);
    let (z1, tail) = rest.split_at_mut(2 * (high_len + 1));

    for slot in z0.iter_mut() { *slot = L::ZERO; }
    for slot in z2.iter_mut() { *slot = L::ZERO; }
    for slot in z1.iter_mut() { *slot = L::ZERO; }

    karatsuba_rec_limb::<L>(lhs_lo, rhs_lo, z0, tail, threshold);
    karatsuba_rec_limb_unbalanced::<L>(lhs_hi, rhs_hi, z2, tail, threshold);

    for slot in lhs_sum.iter_mut() { *slot = L::ZERO; }
    for slot in rhs_sum.iter_mut() { *slot = L::ZERO; }
    lhs_sum[..low_len].copy_from_slice(lhs_lo);
    rhs_sum[..low_len].copy_from_slice(rhs_lo);
    let _ = limb_add_assign::<L>(lhs_sum, lhs_hi);
    let _ = limb_add_assign::<L>(rhs_sum, rhs_hi);

    karatsuba_rec_limb_unbalanced::<L>(lhs_sum, rhs_sum, z1, tail, threshold);
    let _ = limb_sub_assign::<L>(z1, z0);
    let _ = limb_sub_assign::<L>(z1, z2);

    out[..z0.len()].copy_from_slice(z0);
    let _ = limb_add_assign::<L>(&mut out[2 * low_len..], z2);
    let _ = limb_add_assign::<L>(&mut out[low_len..], z1);
}