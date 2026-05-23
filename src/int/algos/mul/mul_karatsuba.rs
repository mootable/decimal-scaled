// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Non-allocating recursive Karatsuba multiplication.
//!
//! Reference: Karatsuba & Ofman 1962, "Multiplication of Multidigit
//! Numbers on Automata" (Doklady Akad. Nauk SSSR 145, 293-294). A *pure*
//! kernel performing one named algorithm; the schoolbook-vs-Karatsuba
//! *choice* (and its crossover threshold) lives in
//! [`crate::int::policy::mul`]. Below the threshold the recursion
//! base-cases to the schoolbook kernel
//! [`crate::int::algos::mul::mul_schoolbook::mul_schoolbook`].

use crate::int::algos::limbs::{add_assign, sub_assign};
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;

/// Stack scratch for the non-allocating Karatsuba kernel, in u64 limbs.
///
/// Each recursion level on an `n`-limb operand carves three product
/// windows (`z0 ≈ 2·⌈n/2⌉`, `z2 ≈ 2·⌈n/2⌉`, `z1 ≈ 2·(⌈n/2⌉+1)`) plus
/// two `(⌈n/2⌉+1)`-limb sum windows off the front of the buffer, then
/// hands the tail to the three (sequential) child calls. That is
/// `S(n) ≤ 6n + O(1)` limbs per level; recursing on the halves gives a
/// geometric total `K(n) = S(n) + S(n/2) + … ≤ 2·S(n) ≤ 12n + O(log n)`.
/// For the widest equal-length multiply the crate performs — `n = 256`
/// limbs (Int<256>) — that bound is `≤ 12·256 ≈ 3072`; rounded up with
/// headroom. ~25 KiB on the stack, recursion depth `log2(256) = 8`.
pub(crate) const KARATSUBA_SCRATCH_LIMBS: usize = 3200;

/// Non-allocating recursive Karatsuba multiplication at u64 base.
///
/// Reference: Karatsuba & Ofman 1962, "Multiplication of Multidigit
/// Numbers on Automata" (Doklady Akad. Nauk SSSR 145, 293-294). Splits
/// both equal-length operands at half: `a = a₁·B + a₀`, `b = b₁·B + b₀`,
/// computes three half-width sub-products `z₀ = a₀·b₀`, `z₂ = a₁·b₁`,
/// `z₁ = (a₀+a₁)·(b₀+b₁) − z₀ − z₂`, then recombines as
/// `z₂·B² + z₁·B + z₀`.
///
/// All temporaries live in a single fixed `[u64; KARATSUBA_SCRATCH_LIMBS]`
/// stack buffer declared once at this public entry point; the recursion
/// carves disjoint windows out of it with `split_at_mut` (so the borrow
/// checker proves non-aliasing — no `unsafe`, no `Vec`, available in
/// `no_std`/no-alloc builds). The three child products run sequentially
/// and share the same scratch tail, which keeps the total at `~2·S(n)`.
///
/// Operands must be equal length. `out.len() >= 2 * a.len()` and `out`
/// must be zeroed by the caller. The crossover threshold against
/// [`mul_schoolbook`] is supplied by [`crate::int::policy::mul`].
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

/// Bench-only entry that drives the production [`karatsuba_rec`] over the
/// real fixed `[u64; KARATSUBA_SCRATCH_LIMBS]` stack scratch at an
/// arbitrary `threshold`, so the crossover sweep can time the kernel at
/// widths below the parked production threshold. `out` is zeroed here.
#[cfg(feature = "bench-alt")]
pub(crate) fn mul_karatsuba_forced(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    debug_assert!(
        karatsuba_scratch_needed_th(a.len(), threshold) <= KARATSUBA_SCRATCH_LIMBS,
        "Karatsuba scratch overflow in forced bench entry"
    );
    for o in out.iter_mut() {
        *o = 0;
    }
    let mut scratch = [0u64; KARATSUBA_SCRATCH_LIMBS];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Test-only entry that drives the production [`karatsuba_rec`] at an
/// arbitrary `threshold`, sizing the scratch for the deeper recursion a
/// small threshold induces. Lets the correctness test exercise the real
/// split/recombine algebra at every width without depending on the
/// shipped threshold.
#[cfg(test)]
pub(crate) fn mul_karatsuba_with_threshold(a: &[u64], b: &[u64], out: &mut [u64], threshold: usize) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(out.len() >= 2 * a.len());
    let need = karatsuba_scratch_needed_th(a.len(), threshold);
    let mut scratch = alloc::vec![0u64; need];
    karatsuba_rec(a, b, out, &mut scratch, threshold);
}

/// Threshold-parameterised upper bound on the scratch (in u64 limbs) the
/// non-allocating Karatsuba recursion consumes for an `n`-limb
/// equal-length multiply. Mirrors the per-level carve in [`karatsuba_rec`]:
/// `S(n) = 2h + 2hi + 2(hi+1) + 2(hi+1)` with `h = n/2`, `hi = n - h`,
/// plus the geometric tail down the largest-child spine.
pub(crate) const fn karatsuba_scratch_needed_th(n: usize, threshold: usize) -> usize {
    if n < threshold {
        return 0;
    }
    let h = n / 2;
    let hi = n - h;
    let level = 2 * h + 2 * hi + (hi + 1) + (hi + 1) + 2 * (hi + 1);
    // The deepest child is the z1 product on `hi + 1`-limb operands (the
    // sum windows), larger than the z0/z2 children — size for it.
    level + karatsuba_scratch_needed_th(hi + 1, threshold)
}

/// One Karatsuba recursion level. `out` is pre-zeroed by the caller for
/// the `2·n`-limb window; `scratch` is the live tail of the entry buffer.
/// Children below `threshold` base-case to [`mul_schoolbook`].
fn karatsuba_rec(a: &[u64], b: &[u64], out: &mut [u64], scratch: &mut [u64], threshold: usize) {
    debug_assert!(
        threshold >= 4,
        "Karatsuba threshold must be >= 4 to terminate"
    );
    let n = a.len();
    if n < threshold {
        // `out` window pre-zeroed by the caller.
        mul_schoolbook(a, b, out);
        return;
    }
    let h = n / 2;
    let hi = n - h; // hi == h or h + 1 (n odd)
    let (a_lo, a_hi) = a.split_at(h);
    let (b_lo, b_hi) = b.split_at(h);

    // Carve this level's windows off the FRONT of scratch; the TAIL is
    // handed down to the (sequential) child calls. `split_at_mut` proves
    // disjointness — no aliasing, all safe Rust.
    let (z0, rest) = scratch.split_at_mut(2 * h);
    let (z2, rest) = rest.split_at_mut(2 * hi);
    let (sa, rest) = rest.split_at_mut(hi + 1);
    let (sb, rest) = rest.split_at_mut(hi + 1);
    let (z1, tail) = rest.split_at_mut(2 * (hi + 1));

    for v in z0.iter_mut() {
        *v = 0;
    }
    for v in z2.iter_mut() {
        *v = 0;
    }
    for v in z1.iter_mut() {
        *v = 0;
    }

    // z0 = a_lo · b_lo (both h limbs), z2 = a_hi · b_hi (both hi limbs).
    // The children run one at a time, so each may reuse `tail`.
    karatsuba_rec(a_lo, b_lo, z0, tail, threshold);
    karatsuba_rec_unbalanced(a_hi, b_hi, z2, tail, threshold);

    // sa = a_lo + a_hi, sb = b_lo + b_hi (each fits hi + 1 limbs with one
    // limb of carry headroom — sa, sb are pre-zeroed by the carve below).
    for v in sa.iter_mut() {
        *v = 0;
    }
    for v in sb.iter_mut() {
        *v = 0;
    }
    sa[..h].copy_from_slice(a_lo);
    sb[..h].copy_from_slice(b_lo);
    let _ = add_assign(sa, a_hi);
    let _ = add_assign(sb, b_hi);

    // z1 = sa · sb − z0 − z2  (sa, sb both hi + 1 limbs).
    karatsuba_rec_unbalanced(sa, sb, z1, tail, threshold);
    let _ = sub_assign(z1, z0);
    let _ = sub_assign(z1, z2);

    // Recombine into the pre-zeroed out: out = z0 + z2·B² + z1·B
    // (B = 2^(64·h)). z0 lands at offset 0, z2 at 2h, z1 at h — the
    // overlap-adds use carry-propagating add, not copy.
    out[..z0.len()].copy_from_slice(z0);
    let _ = add_assign(&mut out[2 * h..], z2);
    let _ = add_assign(&mut out[h..], z1);
}

/// Karatsuba child dispatch for the equal-length sub-products that may
/// drop below the threshold (or be the `hi + 1`-limb sum product). Both
/// operands are equal length here; route to the recursion above the
/// threshold, else zero the window and base-case to [`mul_schoolbook`].
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
        for v in out.iter_mut() {
            *v = 0;
        }
        mul_schoolbook(a, b, out);
    }
}
