// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Knuth Algorithm D division — the limb-generic core ([`knuth_d_core`]) and
//! the base-2⁶⁴ (u64-limb) value-divide entry points.
//!
//! [`div_knuth`] — Knuth Algorithm D (TAOCP Vol 2 §4.3.1) at base 2⁶⁴, the
//! `L = u64` monomorphisation of [`knuth_d_core`] (q̂ via the Möller–Granlund
//! 2-by-1 reciprocal [`Mg2By1`](crate::int::algos::div::div_mg::Mg2By1)). The
//! same [`knuth_d_core`], monomorphised at `L = u128`, is the base-2¹²⁸ engine
//! [`crate::int::algos::div::div_knuth_u128_limb`] — ONE generic kernel, not a
//! per-limb-width copy ([`DivLimb`] is the limb-width abstraction). The
//! divisor-shape / limb-width *choice* that routes between them lives in
//! [`crate::int::policy::div_rem`].

use crate::int::algos::div::div_mg::DivLimb;
use crate::int::algos::div::div_rem::div_rem;
use crate::int::types::compute_limbs::max_single_limbs;

/// Knuth Algorithm D — build-max-scratch wrapper. Allocates the normalised
/// `u`/`v` working buffers at the build-max width and delegates to
/// [`div_knuth_into`]. Callers that can size the scratch exactly (an
/// `Int<N>: ComputeInt` context) call `div_knuth_into` directly with their
/// own buffer (`single_buffered_u64` for a value divide, `quad_buffered_u64` for the cbrt
/// radicand divide), skipping the build-max zeroing.
pub(crate) fn div_knuth(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let mut u = max_single_limbs();
    let mut v = max_single_limbs();
    div_knuth_into(num, den, quot, rem, &mut u, &mut v);
}

/// Knuth Algorithm D at base 2^64, in caller-provided normalised `u`/`v`
/// scratch. `u` and `v` must be **zeroed** on entry and at least
/// `num.len() + 2` / `den.len()` u64 limbs respectively (the divide reads
/// one limb above the live dividend, relying on the zero there).
///
/// Every limb is a u64 and the q̂ estimator uses [`Mg2By1`]. The
/// multiply-subtract pass uses native `u64 × u64 → u128`, which keeps the
/// carry-merge to a single layer.
pub(crate) fn div_knuth_into(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
    u: &mut [u64],
    v: &mut [u64],
) {
    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }

    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "div_knuth: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }
    if top < n {
        let copy_n = num.len().min(rem.len());
        let mut i = 0;
        while i < copy_n {
            rem[i] = num[i];
            i += 1;
        }
        return;
    }

    let shift = den[n - 1].leading_zeros();
    debug_assert!(top < u.len() && n <= v.len());

    if shift == 0 {
        u[..top].copy_from_slice(&num[..top]);
        u[top] = 0;
        v[..n].copy_from_slice(&den[..n]);
    } else {
        let mut carry: u64 = 0;
        for i in 0..top {
            let val = num[i];
            u[i] = (val << shift) | carry;
            carry = val >> (64 - shift);
        }
        u[top] = carry;
        carry = 0;
        for i in 0..n {
            let val = den[i];
            v[i] = (val << shift) | carry;
            carry = val >> (64 - shift);
        }
    }

    let m_plus_n = if u[top] != 0 { top + 1 } else { top };
    debug_assert!(m_plus_n >= n);
    let m = m_plus_n - n;

    // Knuth Algorithm D requires a multi-limb divisor. Single-limb
    // divisors have a much faster hardware divide path; route them out
    // here so the hot loop below can assume n >= 2.
    if n == 1 {
        div_rem(num, den, quot, rem);
        return;
    }

    // Knuth D6/D4: emit the `m + 1` quotient digits and reduce `u` in place to
    // the remainder. The base-2⁶⁴ (`L = u64`) monomorphisation of the
    // limb-generic [`knuth_d_core`]; the u64 quotient slice IS `quot` (no
    // pack/unpack).
    knuth_d_core::<u64>(u, v, n, m, quot);

    if shift == 0 {
        let copy_n = n.min(rem.len());
        rem[..copy_n].copy_from_slice(&u[..copy_n]);
    } else {
        for i in 0..n {
            if i < rem.len() {
                let lo = u[i] >> shift;
                let hi_into_lo = if i + 1 < n {
                    u[i + 1] << (64 - shift)
                } else {
                    0
                };
                rem[i] = lo | hi_into_lo;
            }
        }
    }
}

/// The limb-generic Knuth Algorithm D inner engine (TAOCP Vol 2 §4.3.1, steps
/// D2–D7), at base `2^L::BITS`. Runs the SAME source at base 2⁶⁴ (`L = u64`,
/// the [`div_knuth`] path) and base 2¹²⁸ (`L = u128`, the
/// [`div_knuth_u128_limb`](crate::int::algos::div::div_knuth_u128_limb) path),
/// so the limb-width axis is ONE kernel, not a per-width copy (rule 2). The
/// width is delivered by the [`DivLimb`] type parameter; the q̂ estimator and
/// the limb `MAX` come from it.
///
/// Preconditions (the caller normalises + packs into `L` limbs):
/// - `v[..n]` is the normalised divisor, `n >= 2` (the divisor's top limb
///   `v[n-1]` has its top bit set);
/// - `u` is the normalised running dividend with `u[m + n]` a zeroed window
///   limb above the live dividend, and `u.len() > m + n`;
/// - at each step `u[j+n] <= v[n-1]` (the Knuth normalisation invariant — the
///   leading dividend limb never exceeds the leading divisor limb).
///
/// On return `quot` (little-endian **u64** — the engine's external quotient
/// type) holds the `m + 1` quotient digits (each `L` digit serialised at its
/// u64 limb offset via [`DivLimb::store_quot_digit`], bounds-guarded) and
/// `u[..n]` holds the remainder (still normalised — the caller denormalises by
/// the same shift). The quotient is exact and UNIQUE, so the output is
/// **bit-identical** for any conforming [`DivLimb`].
#[inline]
pub(crate) fn knuth_d_core<L: DivLimb>(u: &mut [L], v: &[L], n: usize, m: usize, quot: &mut [u64]) {
    let v_top = v[n - 1]; // normalised: top bit set
    let v_below = v[n - 2];
    // The q̂ 2-by-1 reciprocal of the (constant) top divisor limb, built ONCE.
    let recip = L::new_recip(v_top);

    let mut j_plus_one = m + 1;
    while j_plus_one > 0 {
        j_plus_one -= 1;
        let j = j_plus_one;

        let jn = j + n;
        let u_top = u[jn];
        let u_next = u[jn - 1];
        debug_assert!(u_top <= v_top, "knuth_d_core: dividend window top exceeds divisor top");

        // D3. q̂ = min(floor((u_top·B + u_next) / v_top), B − 1). The
        // `u_top >= v_top` clamp (only `u_top == v_top` is reachable, per the
        // invariant) caps q̂ at `MAX`; `overflow` records whether the resulting
        // remainder estimate r̂ = u_next + v_top already ran past `B` (a wrapped
        // r̂ ⇒ no D3 refinement is needed).
        let (mut q_hat, mut r_hat, overflow) = if u_top >= v_top {
            let (r, of) = u_next.overflowing_add(v_top);
            (L::MAX, r, of)
        } else {
            let (q, r) = L::est_2by1(&recip, u_top, u_next);
            (q, r, false)
        };

        // D3 refinement against v[n-2]: while q̂·v_below > r̂·B + u[jn-2],
        // decrement q̂ (and bump r̂ by v_top), until r̂ runs past B.
        if !overflow {
            loop {
                let (p_lo, p_hi) = q_hat.widening_mul(v_below);
                if p_hi < r_hat || (p_hi == r_hat && p_lo <= u[jn - 2]) {
                    break;
                }
                q_hat = q_hat.overflowing_sub(L::ONE).0;
                let (new_r, of) = r_hat.overflowing_add(v_top);
                if of {
                    break;
                }
                r_hat = new_r;
            }
        }

        // D4. u[j..=j+n] -= q̂ · v[0..n]. The O(m·n) inner loop — the engine's
        // hot path at the wide tiers, so its per-step critical path dominates.
        // The accumulation form is the limb type's optimal one
        // ([`DivLimb::mul_sub_step`], [`DivLimb::Acc`]): a double-width `u128`
        // accumulator for `u64` limbs (the fused `carry + q̂·v[i]` 128-bit add —
        // benched faster than splitting the multiply carry), a single-`u128`
        // high-word carry for `u128` limbs (no native double-width type). Both
        // keep the carry within one accumulator word.
        let mut carry = L::ACC_ZERO;
        let mut i = 0;
        while i < n {
            let (res, c) = L::mul_sub_step(q_hat, v[i], u[j + i], carry);
            u[j + i] = res;
            carry = c;
            i += 1;
        }
        let (s2, b1) = L::mul_sub_final(u[jn], carry);
        u[jn] = s2;

        // D5/D6. If the final subtraction borrowed, q̂ was 1 too big: add the
        // divisor back once and decrement q̂.
        if b1 {
            q_hat = q_hat.overflowing_sub(L::ONE).0;
            let mut carry = L::ZERO;
            let mut i = 0;
            while i < n {
                let (s1, c1) = u[j + i].overflowing_add(v[i]);
                let (s2, c2) = s1.overflowing_add(carry);
                u[j + i] = s2;
                // c1, c2 are never both set (`u[j+i]+v[i] ≤ 2B−2 < 2B−1`), so
                // `0 + c1 + c2 ∈ {0, 1}` — the schoolbook carry merge.
                carry = L::ZERO.add_carries(c1, c2);
                i += 1;
            }
            u[jn] = u[jn].overflowing_add(carry).0;
        }

        L::store_quot_digit(quot, j, q_hat);
    }
}
