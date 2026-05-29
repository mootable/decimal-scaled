// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Knuth Algorithm D division at base 2⁶⁴.
//!
//! [`div_knuth`] — Knuth Algorithm D (TAOCP Vol 2 §4.3.1) at base 2⁶⁴,
//! with the q̂ estimate from the Möller–Granlund 2-by-1 reciprocal
//! [`crate::int::algos::div::div_mg::Mg2By1`]. The divisor-shape *choice*
//! that routes here lives in [`crate::int::policy::div_rem`].

use crate::int::algos::div::div_mg::Mg2By1;
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

    // MG 2-by-1 q̂ estimator (Möller-Granlund 2011 Algorithm 4) + inner
    // refinement against v[n-2]. The 3-by-2 estimator was re-benched post
    // u64 migration: its per-q̂ setup cost (extra multiplies vs the
    // 2-by-1's one) outweighs the refinement loop's near-zero iteration
    // count on decimal divisors, so 2-by-1 + while-loop still wins at the
    // widest tiers.
    let v_top = v[n - 1];
    let v_below = v[n - 2];
    let mg_top = Mg2By1::new(v_top);

    let mut j_plus_one = m + 1;
    while j_plus_one > 0 {
        j_plus_one -= 1;
        let j = j_plus_one;

        let jn = j + n;
        let u_top = u[jn];
        let u_next = u[jn - 1];

        let (mut q_hat, mut r_hat) = if u_top > v_top {
            (u64::MAX, u64::MAX)
        } else if u_top == v_top {
            let (r, of) = u_next.overflowing_add(v_top);
            (u64::MAX, if of { u64::MAX } else { r })
        } else {
            mg_top.div_rem(u_top, u_next)
        };

        // Refinement against v[n-2].
        loop {
            let prod = (q_hat as u128) * (v_below as u128);
            let hi = (prod >> 64) as u64;
            let lo = prod as u64;
            let rhs_lo = u[jn - 2];
            let rhs_hi = r_hat;
            if hi < rhs_hi || (hi == rhs_hi && lo <= rhs_lo) {
                break;
            }
            q_hat = q_hat.wrapping_sub(1);
            let (new_r, of) = r_hat.overflowing_add(v_top);
            if of {
                break;
            }
            r_hat = new_r;
        }

        // D4. u[j..=j+n] -= q̂ · v[0..n]
        // Merged carry: a single u128 `carry` accumulates q̂·v[i] plus the
        // borrow from the previous limb, so each inner step is one 64×64→128
        // multiply, one u128 add and one `overflowing_sub` — one fewer
        // overflowing op than tracking the multiply carry and the borrow as
        // two separate u64s. The O(m·n) inner loop is the engine's hot path
        // at the wide tiers, so the saved op per step compounds with width
        // (benched: ~1.1× faster at D924/D1232; a wash at the narrow tiers).
        // Bound: after `carry += q̂·v[i]`, carry ≤ (2⁶⁴−1)² + 2⁶⁴ < 2¹²⁸, so
        // the accumulate never overflows the u128.
        let mut carry: u128 = 0;
        for i in 0..n {
            carry += (q_hat as u128) * (v[i] as u128);
            let sub_lo = carry as u64;
            let (res, b) = u[j + i].overflowing_sub(sub_lo);
            u[j + i] = res;
            carry = (carry >> 64) + (b as u128);
        }
        let sub_lo = carry as u64;
        let (s2, b1) = u[j + n].overflowing_sub(sub_lo);
        u[j + n] = s2;
        let final_borrow = (b1 as u64) + ((carry >> 64) as u64);

        if final_borrow != 0 {
            q_hat = q_hat.wrapping_sub(1);
            let mut carry: u64 = 0;
            for i in 0..n {
                let (s1, c1) = u[j + i].overflowing_add(v[i]);
                let (s2, c2) = s1.overflowing_add(carry);
                u[j + i] = s2;
                carry = (c1 as u64) + (c2 as u64);
            }
            u[j + n] = u[j + n].wrapping_add(carry);
        }

        if j < quot.len() {
            quot[j] = q_hat;
        }
    }

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
