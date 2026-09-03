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
/// `Int<N>: ComputeLimbs` context) call `div_knuth_into` directly with their
/// own buffer (`single_buffered_u64` for a value divide, `quad_buffered_u64` for the cbrt
/// radicand divide), skipping the build-max zeroing.
pub(crate) fn div_knuth(dividend: &[u64], divisor: &[u64], quotient: &mut [u64],
    remainder: &mut [u64]) {
    let mut u = max_single_limbs();
    let mut v = max_single_limbs();
    div_knuth_into(dividend, divisor, quotient, remainder, &mut u, &mut v);
}

/// Knuth Algorithm D at base 2^64, in caller-provided normalised `u`/`v`
/// scratch. `u` and `v` must be **zeroed** on entry and at least
/// `dividend.len() + 2` / `divisor.len()` u64 limbs respectively (the divide
/// reads one limb above the live dividend, relying on the zero there).
///
/// Every limb is a u64 and the q̂ estimator uses [`Mg2By1`]. The
/// multiply-subtract pass uses native `u64 × u64 → u128`, which keeps the
/// carry-merge to a single layer.
pub(crate) fn div_knuth_into(
    dividend: &[u64],
    divisor: &[u64],
    quotient: &mut [u64],
    remainder: &mut [u64],
    u: &mut [u64],
    v: &mut [u64],
) {
    for slot in quotient.iter_mut() {
        *slot = 0;
    }
    for slot in remainder.iter_mut() {
        *slot = 0;
    }

    let mut n = divisor.len();
    while n > 0 && divisor[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "div_knuth: divide by zero");

    let mut dividend_len = dividend.len();
    while dividend_len > 0 && dividend[dividend_len - 1] == 0 {
        dividend_len -= 1;
    }
    if dividend_len < n {
        let copy_len = dividend.len().min(remainder.len());
        let mut i = 0;
        while i < copy_len {
            remainder[i] = dividend[i];
            i += 1;
        }
        return;
    }

    let shift = divisor[n - 1].leading_zeros();

    // The length precondition as SLICING, not as the `debug_assert!` it used to
    // be — an assert vanishes in release, a slice constrains release codegen.
    // `u_norm` is the normalisation window (the live dividend plus the one
    // shift-carry limb above it) and `v` is bounded to the `n` significant
    // divisor limbs, so both normalisation loops below run with their length
    // relation already proved. Neither bound can panic where the old code did
    // not: `u[dividend_len]` and `v[n - 1]` are both written unconditionally
    // in every branch below, so `u.len() > dividend_len` and `v.len() >= n`
    // were already hard release requirements.
    let v = &mut v[..n];
    let u_norm = &mut u[..=dividend_len];

    if shift == 0 {
        u_norm[..dividend_len].copy_from_slice(&dividend[..dividend_len]);
        u_norm[dividend_len] = 0;
        v.copy_from_slice(&divisor[..n]);
    } else {
        let mut carry: u64 = 0;
        for i in 0..dividend_len {
            let limb = dividend[i];
            u_norm[i] = (limb << shift) | carry;
            carry = limb >> (64 - shift);
        }
        u_norm[dividend_len] = carry;
        carry = 0;
        for i in 0..n {
            let limb = divisor[i];
            v[i] = (limb << shift) | carry;
            carry = limb >> (64 - shift);
        }
    }

    let m_plus_n = if u_norm[dividend_len] != 0 { dividend_len + 1 } else { dividend_len };
    debug_assert!(m_plus_n >= n);
    let m = m_plus_n - n;

    // Knuth Algorithm D requires a multi-limb divisor. Single-limb
    // divisors have a much faster hardware divide path; route them out
    // here so the hot loop below can assume n >= 2.
    if n == 1 {
        div_rem(dividend, divisor, quotient, remainder);
        return;
    }

    // Knuth D6/D4: emit the `m + 1` quotient digits and reduce `u` in place to
    // the remainder. The base-2⁶⁴ (`L = u64`) monomorphisation of the
    // limb-generic [`knuth_d_core`]; the u64 quotient slice IS `quotient` (no
    // pack/unpack).
    // `u[..=m_plus_n]` is the exact Knuth D window: the `m + n` live dividend
    // limbs plus the zero limb above them that D4 reads as the window top. It
    // is the same shape the u128 engine already hands the core, and it cannot
    // panic where the old code did not — the core indexed `u[m + n]`
    // unconditionally on its first step.
    knuth_d_core::<u64>(&mut u[..=m_plus_n], v, n, m, quotient);

    // The remainder is the low `n` limbs of that same window, read back through
    // a shared view of it — so the denormalisation loop below is bounded too.
    let u_live = &u[..=m_plus_n];

    if shift == 0 {
        let copy_len = n.min(remainder.len());
        remainder[..copy_len].copy_from_slice(&u_live[..copy_len]);
    } else {
        for i in 0..n {
            if i < remainder.len() {
                let lo = u_live[i] >> shift;
                let hi_into_lo = if i + 1 < n {
                    u_live[i + 1] << (64 - shift)
                } else {
                    0
                };
                remainder[i] = lo | hi_into_lo;
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
/// On return `quotient` (little-endian **u64** — the engine's external
/// quotient type) holds the `m + 1` quotient digits (each `L` digit serialised
/// at its u64 limb offset via [`DivLimb::store_quot_digit`], bounds-guarded)
/// and `u[..n]` holds the remainder (still normalised — the caller denormalises
/// by the same shift). The quotient is exact and UNIQUE, so the output is
/// **bit-identical** for any conforming [`DivLimb`].
#[inline]
pub(crate) fn knuth_d_core<L: DivLimb>(u: &mut [L], v: &[L], n: usize, m: usize,
    quotient: &mut [u64]) {
    // Bound `v` to its `n` significant limbs ONCE. Every read below is inside
    // `v[..n]`, so this single check replaces the per-step length re-proof in
    // both inner loops. It cannot panic where the old code did not: `v[n - 1]`
    // is read unconditionally on the very next line, so `v.len() >= n` was
    // already a hard release requirement.
    let v = &v[..n];
    let v_top = v[n - 1]; // normalised: top bit set
    let v_below = v[n - 2];
    // The q̂ 2-by-1 reciprocal of the (constant) top divisor limb, built ONCE.
    let recip = L::new_recip(v_top);

    let mut j_plus_one = m + 1;
    while j_plus_one > 0 {
        j_plus_one -= 1;
        let j = j_plus_one;

        let j_plus_n = j + n;
        // Knuth's own D4 window `u[j..=j+n]`, taken as ONE slice per quotient
        // digit. Its length is `n + 1` by construction, so every access below —
        // `u_win[n]` and `u_win[n - 1]` for the q̂ estimate, `u_win[n - 2]` for
        // the D3 refinement, `u_win[i]` for `i < n` in the D4 and D6 loops — is
        // in range by the slice's own length, with nothing left for the
        // compiler to re-prove per step. That turns the O(m·n) bounds checks
        // the indexed `u[j + i]` form needed into O(m) window checks; not one
        // computed value changes, since `u_win[k]` IS `u[j + k]`.
        let u_win = &mut u[j..=j_plus_n];
        let u_top = u_win[n];
        let u_next = u_win[n - 1];
        debug_assert!(u_top <= v_top, "knuth_d_core: dividend window top exceeds divisor top");

        // D3. q̂ = min(floor((u_top·B + u_next) / v_top), B − 1). The
        // `u_top >= v_top` clamp (only `u_top == v_top` is reachable, per the
        // invariant) caps q̂ at `MAX`; `overflow` records whether the resulting
        // remainder estimate r̂ = u_next + v_top already ran past `B` (a wrapped
        // r̂ ⇒ no D3 refinement is needed).
        let (mut q_hat, mut r_hat, overflow) = if u_top >= v_top {
            let (r, overflowed) = u_next.overflowing_add(v_top);
            (L::MAX, r, overflowed)
        } else {
            let (q, r) = L::est_2by1(&recip, u_top, u_next);
            (q, r, false)
        };

        // D3 refinement against v[n-2]: while q̂·v_below > r̂·B + u[j_plus_n-2],
        // decrement q̂ (and bump r̂ by v_top), until r̂ runs past B.
        if !overflow {
            loop {
                let (p_lo, p_hi) = q_hat.widening_mul(v_below);
                if p_hi < r_hat || (p_hi == r_hat && p_lo <= u_win[n - 2]) {
                    break;
                }
                q_hat = q_hat.overflowing_sub(L::ONE).0;
                let (new_r_hat, overflowed) = r_hat.overflowing_add(v_top);
                if overflowed {
                    break;
                }
                r_hat = new_r_hat;
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
            let (new_limb, new_carry) = L::mul_sub_step(q_hat, v[i], u_win[i], carry);
            u_win[i] = new_limb;
            carry = new_carry;
            i += 1;
        }
        let (final_limb, borrowed) = L::mul_sub_final(u_win[n], carry);
        u_win[n] = final_limb;

        // D5/D6. If the final subtraction borrowed, q̂ was 1 too big: add the
        // divisor back once and decrement q̂.
        if borrowed {
            q_hat = q_hat.overflowing_sub(L::ONE).0;
            let mut carry = L::ZERO;
            let mut i = 0;
            while i < n {
                let (sum1, carry1) = u_win[i].overflowing_add(v[i]);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                u_win[i] = sum2;
                // carry1, carry2 are never both set (`u[j+i]+v[i] ≤ 2B−2 <
                // 2B−1`), so `0 + carry1 + carry2 ∈ {0, 1}` — the schoolbook
                // carry merge.
                carry = L::ZERO.add_carries(carry1, carry2);
                i += 1;
            }
            u_win[n] = u_win[n].overflowing_add(carry).0;
        }

        L::store_quot_digit(quotient, j, q_hat);
    }
}
