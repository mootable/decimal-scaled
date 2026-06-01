// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Möller–Granlund invariant-divisor reciprocal engines.
//!
//! The per-q̂ estimators the wider division engines build on:
//!
//! - [`Mg2By1`] — 2-by-1 invariant divisor (Möller & Granlund 2011,
//!   Algorithm 4); the estimator [`crate::int::algos::div::div_knuth`]
//!   uses.
//! - [`Mg3By2`] — 3-by-2 invariant divisor (Algorithms 5 & 6); a
//!   single-pass exact quotient for a 2-limb divisor.
//! - [`DivLimb`] — the *limb-width* abstraction over the q̂ 2-by-1 estimator,
//!   so ONE generic Knuth Algorithm D kernel
//!   ([`crate::int::algos::div::div_knuth::knuth_d_core`]) runs at base 2⁶⁴
//!   (`L = u64`, [`Mg2By1`]) or base 2¹²⁸ (`L = u128`, [`Mg3By2`]).

use crate::int::types::compute_limbs::Limb;

/// Möller–Granlund 2-by-1 invariant divisor at u64 base.
///
/// Reference: Möller & Granlund (2011), Algorithm 4.
///
/// The u64 base implementation is compact because the doubled type
/// (u128) is *native* — each q̂ step is a single `u128` op rather than a
/// software 256-bit decomposition.
#[derive(Clone, Copy)]
pub(crate) struct Mg2By1 {
    d: u64,
    v: u64,
}

impl Mg2By1 {
    /// `d` must be normalised: `d >> 63 == 1`.
    #[inline]
    pub(crate) const fn new(d: u64) -> Self {
        debug_assert!(d >> 63 == 1, "Mg2By1::new: divisor must be normalised");
        // v = floor((B² - 1 - d·B) / d) where B = 2^64.
        let num = ((!d as u128) << 64) | (u64::MAX as u128);
        let v = (num / (d as u128)) as u64;
        Self { d, v }
    }

    /// Divide `(u1·B + u0)` by `d`. Requires `u1 < d`.
    #[inline]
    pub(crate) const fn div_rem(&self, u1: u64, u0: u64) -> (u64, u64) {
        debug_assert!(u1 < self.d, "Mg2By1::div_rem: high word must be < divisor");
        let q128 = (self.v as u128)
            .wrapping_mul(u1 as u128)
            .wrapping_add(((u1 as u128) << 64) | (u0 as u128));
        let mut q1 = (q128 >> 64) as u64;
        let q0 = q128 as u64;
        q1 = q1.wrapping_add(1);
        let mut r = u0.wrapping_sub(q1.wrapping_mul(self.d));
        if r > q0 {
            q1 = q1.wrapping_sub(1);
            r = r.wrapping_add(self.d);
        }
        if r >= self.d {
            q1 = q1.wrapping_add(1);
            r = r.wrapping_sub(self.d);
        }
        (q1, r)
    }
}

/// Möller–Granlund 3-by-2 invariant divisor at u64 base.
///
/// Divides `(n2·B² + n1·B + n0)` by `(d1·B + d0)` for a normalised 2-limb
/// divisor (`d1`'s top bit set) using *two* limbs of divisor information,
/// returning a quotient that is exactly correct in one pass — no
/// refinement loop is needed in the Knuth Algorithm D caller.
///
/// Reference: Möller & Granlund 2011, Algorithm 5 (the divide) and
/// Algorithm 6 (the reciprocal precompute). [`Mg2By1`] is the 2-by-1
/// cousin used by [`crate::int::algos::div::div_knuth`]'s q̂ estimator.
#[derive(Clone, Copy)]
pub(crate) struct Mg3By2 {
    d1: u64,
    d0: u64,
    /// Reciprocal of the top divisor limb (same formula as Mg2By1::v).
    dinv: u64,
}

impl Mg3By2 {
    /// Setup. `d1` must be normalised (`d1 >> 63 == 1`).
    ///
    /// Reference: Möller & Granlund 2011, Algorithm 6 (the reciprocal
    /// refinement that accounts for `d0`).
    #[inline]
    pub(crate) const fn new(d1: u64, d0: u64) -> Self {
        debug_assert!(
            d1 >> 63 == 1,
            "Mg3By2::new: top divisor limb must be normalised"
        );
        // Step 1: 2-by-1 reciprocal of d1 alone.
        let num = ((!d1 as u128) << 64) | (u64::MAX as u128);
        let mut v = (num / (d1 as u128)) as u64;

        // Step 2: refine for d0. `p = d1·v + d0` (mod B). If the sum
        // overflows, v was over-estimated → decrement.
        let mut p = d1.wrapping_mul(v).wrapping_add(d0);
        if p < d0 {
            v = v.wrapping_sub(1);
            let mask = if p >= d1 { u64::MAX } else { 0 };
            p = p.wrapping_sub(d1);
            v = v.wrapping_add(mask);
            p = p.wrapping_sub(mask & d1);
        }

        // Step 3: account for d0·v.
        let prod = (d0 as u128) * (v as u128);
        let t1 = (prod >> 64) as u64;
        let t0 = prod as u64;
        let (new_p, carry) = p.overflowing_add(t1);
        let _p_final = new_p;
        if carry {
            v = v.wrapping_sub(1);
            if new_p >= d1 && (new_p > d1 || t0 >= d0) {
                v = v.wrapping_sub(1);
            }
        }

        Self { d1, d0, dinv: v }
    }

    /// Divide `(n2·B² + n1·B + n0)` by `(d1·B + d0)`. Requires
    /// `(n2, n1) < (d1, d0)` so the quotient fits a single u64. Returns
    /// `(q, r1, r0)` where the remainder is `r1·B + r0`.
    #[inline]
    pub(crate) const fn div_rem(&self, n2: u64, n1: u64, n0: u64) -> (u64, u64, u64) {
        debug_assert!(
            n2 < self.d1 || (n2 == self.d1 && n1 < self.d0),
            "Mg3By2::div_rem: numerator high pair must be < divisor"
        );

        // Step 1: q estimate from (n2, n1) / d1 via dinv.
        let prod = (n2 as u128)
            .wrapping_mul(self.dinv as u128)
            .wrapping_add(((n2 as u128) << 64) | (n1 as u128));
        let mut q = (prod >> 64) as u64;
        let q_lo = prod as u64;

        // Step 2a: r1 = n1 - q·d1 (mod B).
        let mut r1 = n1.wrapping_sub(q.wrapping_mul(self.d1));

        // Step 2b: (r1, r0) = (r1, n0) - (d1, d0).
        let r256 = (((r1 as u128) << 64) | (n0 as u128))
            .wrapping_sub(((self.d1 as u128) << 64) | (self.d0 as u128));
        r1 = (r256 >> 64) as u64;
        let mut r0 = r256 as u64;

        // Step 2c: (r1, r0) -= d0·q (mod B²).
        let t = (self.d0 as u128).wrapping_mul(q as u128);
        let r256 = (((r1 as u128) << 64) | (r0 as u128)).wrapping_sub(t);
        r1 = (r256 >> 64) as u64;
        r0 = r256 as u64;

        // Step 3: q += 1; provisional.
        q = q.wrapping_add(1);

        // Step 4a: first conditional correction.
        let mask = if r1 >= q_lo { u64::MAX } else { 0 };
        q = q.wrapping_add(mask); // adds u64::MAX = -1.
        let add = ((mask & self.d1) as u128) << 64 | ((mask & self.d0) as u128);
        let r256 = (((r1 as u128) << 64) | (r0 as u128)).wrapping_add(add);
        r1 = (r256 >> 64) as u64;
        r0 = r256 as u64;

        // Step 4b: final correction (rare).
        if r1 > self.d1 || (r1 == self.d1 && r0 >= self.d0) {
            q = q.wrapping_add(1);
            let r256 = (((r1 as u128) << 64) | (r0 as u128))
                .wrapping_sub(((self.d1 as u128) << 64) | (self.d0 as u128));
            r1 = (r256 >> 64) as u64;
            r0 = r256 as u64;
        }

        (q, r1, r0)
    }
}

/// The limb-width abstraction the generic Knuth Algorithm D kernel
/// ([`crate::int::algos::div::div_knuth::knuth_d_core`]) divides over: the q̂
/// 2-by-1 estimator `(hi·B + lo) / v_top → (q̂, r̂)` (where `B = 2^L::BITS`),
/// plus the limb-type `MAX`. Implemented for exactly `u64` (base 2⁶⁴, q̂ via
/// the [`Mg2By1`] reciprocal) and `u128` (base 2¹²⁸, q̂ via two exact
/// [`Mg3By2`] passes — a 256-by-128 divide of the four/two u64 sub-limbs).
///
/// The reciprocal of the divisor's top limb (`v_top`) is **invariant across a
/// whole divide**, so it is built ONCE ([`new_recip`](DivLimb::new_recip)) and
/// threaded into every q̂ ([`est_2by1`](DivLimb::est_2by1)) — never rebuilt per
/// quotient digit.
///
/// Knuth D only needs an estimate within `{q, q + 1}` of the true digit (the
/// D6 add-back corrects a single over-estimate), and the final quotient /
/// remainder of an integer divide are UNIQUE — so any conforming `DivLimb`
/// yields **bit-identical** output regardless of how q̂ is formed.
pub(crate) trait DivLimb: Limb {
    /// The precomputed reciprocal of the top divisor limb.
    type Recip: Copy;
    /// The running carry of the D4 multiply-subtract pass. Width-specific
    /// because the *optimal* accumulation differs (the loop is otherwise one
    /// generic source): `u64` limbs use a **double-width `u128` accumulator**
    /// (`carry + q̂·v[i]` fused as one 128-bit add — a shorter critical path,
    /// benched ~3–4 % faster than splitting the multiply carry from the
    /// borrow), while `u128` limbs have no native double-width type and use a
    /// **single-`u128` high-word carry**.
    type Acc: Copy;
    /// The all-ones limb value (`u64::MAX` / `u128::MAX`) — the q̂ clamp.
    const MAX: Self;
    /// Additive identity of the D4 carry accumulator [`Acc`](DivLimb::Acc).
    const ACC_ZERO: Self::Acc;
    /// Build the 2-by-1 reciprocal of the normalised top divisor limb
    /// `v_top` (its top bit set). Called once per divide.
    fn new_recip(v_top: Self) -> Self::Recip;
    /// The Knuth q̂ 2-by-1 estimate `(hi·B + lo) / v_top → (q̂, r̂)`, requiring
    /// `hi < v_top` (the caller's `hi >= v_top` clamp guarantees it). `B =
    /// 2^Self::BITS`. Returns the floor quotient (`< B`) and its remainder.
    fn est_2by1(recip: &Self::Recip, hi: Self, lo: Self) -> (Self, Self);
    /// One D4 step `u[j+i] -= q̂·v[i] + carry`: returns the reduced dividend
    /// limb and the carry propagated into the next limb. Per-width to keep each
    /// limb type's optimal accumulation (see [`Acc`](DivLimb::Acc)).
    fn mul_sub_step(q_hat: Self, v_i: Self, u_ji: Self, carry: Self::Acc) -> (Self, Self::Acc);
    /// The D4 final window limb `u[j+n] -= carry`: returns the reduced limb and
    /// whether it borrowed (q̂ was 1 too big ⇒ the D6 add-back fires).
    fn mul_sub_final(u_top: Self, carry: Self::Acc) -> (Self, bool);
    /// Serialise one quotient digit (the `L` value at quotient position `j`)
    /// into the little-endian u64 output `quot` — the output is always u64
    /// (the engine's external contract), so the digit is written at its u64
    /// limb offset: `quot[j]` for `u64`, `quot[2j] | quot[2j+1] << 64` for
    /// `u128`. Bounds-guarded (a digit beyond `quot.len()` is dropped, matching
    /// both engines' prior `if … < quot.len()` writes), so the generic core
    /// needs no separate `L`-typed quotient buffer.
    fn store_quot_digit(quot: &mut [u64], j: usize, digit: Self);
}

impl DivLimb for u64 {
    type Recip = Mg2By1;
    type Acc = u128;
    const MAX: Self = u64::MAX;
    const ACC_ZERO: Self::Acc = 0;
    #[inline]
    fn new_recip(v_top: Self) -> Self::Recip {
        Mg2By1::new(v_top)
    }
    #[inline]
    fn est_2by1(recip: &Self::Recip, hi: Self, lo: Self) -> (Self, Self) {
        recip.div_rem(hi, lo)
    }
    #[inline(always)]
    fn mul_sub_step(q_hat: Self, v_i: Self, u_ji: Self, carry: u128) -> (Self, u128) {
        // Double-width accumulator: fuse `carry + q̂·v[i]` into one 128-bit add
        // (`carry ≤ 2⁶⁴`, product ≤ (2⁶⁴−1)², sum < 2¹²⁸), take the low word as
        // the amount to subtract, propagate the high word plus the borrow.
        // `inline(always)`: this is the O(m·n) hot step — it MUST fold into the
        // generic loop body so the monomorphisation matches the hand-inlined
        // base-2⁶⁴ loop with no call/abstraction overhead.
        let acc = carry + (q_hat as u128) * (v_i as u128);
        let (res, b) = u_ji.overflowing_sub(acc as u64);
        (res, (acc >> 64) + (b as u128))
    }
    #[inline(always)]
    fn mul_sub_final(u_top: Self, carry: u128) -> (Self, bool) {
        let (s, b1) = u_top.overflowing_sub(carry as u64);
        // The borrow is the subtraction borrow OR a non-zero high carry word.
        (s, b1 || (carry >> 64) != 0)
    }
    #[inline]
    fn store_quot_digit(quot: &mut [u64], j: usize, digit: Self) {
        if j < quot.len() {
            quot[j] = digit;
        }
    }
}

impl DivLimb for u128 {
    type Recip = Mg3By2;
    type Acc = u128;
    const MAX: Self = u128::MAX;
    const ACC_ZERO: Self::Acc = 0;
    #[inline]
    fn new_recip(v_top: Self) -> Self::Recip {
        // The two u64 sub-limbs of the normalised top u128 divisor limb.
        Mg3By2::new((v_top >> 64) as u64, v_top as u64)
    }
    #[inline]
    fn est_2by1(recip: &Self::Recip, hi: Self, lo: Self) -> (Self, Self) {
        // 256-by-128 → 128: a base-2⁶⁴ n=2 Knuth divide of the four u64 limbs
        // (a3,a2,a1,a0) by the two u64 limbs of v_top, two exact Mg3By2 passes
        // (no software u256 reciprocal). `hi < v_top` guarantees each pass's
        // `(n2,n1) < (d1,d0)` precondition and a 128-bit quotient.
        let a3 = (hi >> 64) as u64;
        let a2 = hi as u64;
        let a1 = (lo >> 64) as u64;
        let a0 = lo as u64;
        let (q1, r1, r0) = recip.div_rem(a3, a2, a1);
        let (q0, s1, s0) = recip.div_rem(r1, r0, a0);
        let q = ((q1 as u128) << 64) | (q0 as u128);
        let r = ((s1 as u128) << 64) | (s0 as u128);
        (q, r)
    }
    #[inline(always)]
    fn mul_sub_step(q_hat: Self, v_i: Self, u_ji: Self, carry: u128) -> (Self, u128) {
        // No native double-width type: single-`u128` high-word carry. `q̂·v[i]`
        // is a 256-bit (p_lo, p_hi); add the incoming carry into p_lo, subtract
        // p_lo from u[j+i], propagate p_hi + the add-overflow + the borrow
        // (which fits one u128 by the same bound as the u64 double-width form).
        // `inline(always)`: the O(m·n) hot step — see the u64 sibling.
        let (p_lo, p_hi) = <u128 as Limb>::widening_mul(q_hat, v_i);
        let (sub_lo, c0) = p_lo.overflowing_add(carry);
        let (res, b) = u_ji.overflowing_sub(sub_lo);
        (res, p_hi + (c0 as u128) + (b as u128))
    }
    #[inline(always)]
    fn mul_sub_final(u_top: Self, carry: u128) -> (Self, bool) {
        u_top.overflowing_sub(carry)
    }
    #[inline]
    fn store_quot_digit(quot: &mut [u64], j: usize, digit: Self) {
        // The u128 digit is two little-endian u64 limbs at offset 2j / 2j+1.
        if 2 * j < quot.len() {
            quot[2 * j] = digit as u64;
        }
        if 2 * j + 1 < quot.len() {
            quot[2 * j + 1] = (digit >> 64) as u64;
        }
    }
}
