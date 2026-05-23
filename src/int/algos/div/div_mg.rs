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
