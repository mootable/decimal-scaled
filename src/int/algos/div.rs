//! Integer division engines over little-endian `u64` limb slices.
//!
//! The pure division *engines* — each performs one named algorithm on an
//! already-chosen basis; the divisor-shape *choice* between them lives in
//! [`crate::int::policy::div`]:
//!
//! - [`div_rem`] — `const fn` single-/double-limb hardware divide (and the
//!   shift-subtract fallback for the rare const multi-limb case). The
//!   const-evaluable `wrapping_div` / `wrapping_rem` stay on this so they
//!   can run at compile time.
//! - [`div_knuth`] — Knuth Algorithm D (TAOCP Vol 2 §4.3.1) at base 2⁶⁴,
//!   q̂ estimated with the Möller–Granlund 2-by-1 reciprocal [`Mg2By1`].
//! - [`div_burnikel_ziegler_with_knuth`] — Burnikel–Ziegler outer chunking
//!   that recurses to [`div_knuth`] as its base case (hence the `_with_`
//!   hybrid name).
//! - [`div_mg`] — the Möller–Granlund invariant-divisor reciprocal engines
//!   ([`Mg2By1`] / [`Mg3By2`]), the per-q̂ estimators the wider engines
//!   build on.
//!
//! [`div_rem_mag_fixed`] / [`isqrt_mag_fixed`] are the const-`N` fast-arm
//! wrappers the fixed-width `Int<N>` types call.

use super::limbs::{bit_len, cmp, fit_one, shl1, sub_assign};
use crate::int::policy::div::div_rem_dispatch;
use crate::int::algos::roots::isqrt_newton;

/// Scratch capacity for the runtime u64-limb division engines — 288 u64
/// limbs (18432 bits), covering the widest work integer in the crate
/// (Int<256> used by D1232 cbrt, 256 u64 limbs) with slack.
pub(crate) const SCRATCH_LIMBS: usize = 288;

/// `quot = num / den`, `rem = num % den`, u64 limbs. `const fn`.
///
/// Hardware fast paths:
/// - both fit a single u64 → one native `u64 / u64`
/// - divisor fits a single u64 → native `u128 / u64` per dividend limb
/// - otherwise → bit shift-subtract (only reached when divisor is
///   multi-limb; the dispatcher routes those to Knuth instead)
pub(crate) const fn div_rem(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let mut z = 0;
    while z < quot.len() {
        quot[z] = 0;
        z += 1;
    }
    z = 0;
    while z < rem.len() {
        rem[z] = 0;
        z += 1;
    }

    let den_one_limb = fit_one(den);

    // Fast path A: both fit a single u64 → hardware divide.
    if den_one_limb && fit_one(num) {
        if !quot.is_empty() {
            quot[0] = num[0] / den[0];
        }
        if !rem.is_empty() {
            rem[0] = num[0] % den[0];
        }
        return;
    }

    // Fast path B: divisor fits a single u64 — schoolbook base-2^64 long
    // divide using the native u128/u64 hardware divide.
    if den_one_limb {
        let d = den[0];
        let mut r: u64 = 0;
        let mut top = num.len();
        while top > 0 && num[top - 1] == 0 {
            top -= 1;
        }
        let mut i = top;
        while i > 0 {
            i -= 1;
            let acc = ((r as u128) << 64) | (num[i] as u128);
            let q = (acc / (d as u128)) as u64;
            r = (acc % (d as u128)) as u64;
            if i < quot.len() {
                quot[i] = q;
            }
        }
        if !rem.is_empty() {
            rem[0] = r;
        }
        return;
    }

    // General path: binary shift-subtract. Only reached for multi-limb
    // divisors when the dispatcher isn't routing to Knuth (i.e. in const
    // contexts where Knuth isn't available).
    let bits = bit_len(num);
    let mut i = bits;
    while i > 0 {
        i -= 1;
        shl1(rem);
        let bit = (num[(i / 64) as usize] >> (i % 64)) & 1;
        rem[0] |= bit;
        shl1(quot);
        if cmp(rem, den) >= 0 {
            sub_assign(rem, den);
            quot[0] |= 1;
        }
    }
}

// ── Möller–Granlund invariant-divisor reciprocal engines (`div_mg`) ────

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
/// cousin used by [`div_knuth`]'s q̂ estimator.
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

/// Knuth Algorithm D at base 2^64.
///
/// Every limb is a u64 and the q̂ estimator uses [`Mg2By1`]. The
/// multiply-subtract pass uses native `u64 × u64 → u128`, which keeps the
/// carry-merge to a single layer.
pub(crate) fn div_knuth(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
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
    let mut u = [0u64; SCRATCH_LIMBS];
    let mut v = [0u64; SCRATCH_LIMBS];
    debug_assert!(top < SCRATCH_LIMBS && n <= SCRATCH_LIMBS);

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
        let mut mul_carry: u64 = 0;
        let mut borrow: u64 = 0;
        for i in 0..n {
            let prod = (q_hat as u128) * (v[i] as u128);
            let prod_lo = prod as u64;
            let prod_hi = (prod >> 64) as u64;
            let (s_prod, c1) = prod_lo.overflowing_add(mul_carry);
            let new_mul_carry = prod_hi + (c1 as u64);
            let (s1, b1) = u[j + i].overflowing_sub(s_prod);
            let (s2, b2) = s1.overflowing_sub(borrow);
            u[j + i] = s2;
            borrow = (b1 as u64) + (b2 as u64);
            mul_carry = new_mul_carry;
        }
        let (s1, b1) = u[j + n].overflowing_sub(mul_carry);
        let (s2, b2) = s1.overflowing_sub(borrow);
        u[j + n] = s2;
        let final_borrow = (b1 as u64) + (b2 as u64);

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

/// Burnikel–Ziegler outer chunking, u64 base, recursing to [`div_knuth`]
/// as the base case.
pub(crate) fn div_burnikel_ziegler_with_knuth(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
) {
    let mut n = den.len();
    while n > 0 && den[n - 1] == 0 {
        n -= 1;
    }
    assert!(n > 0, "div_burnikel_ziegler_with_knuth: divide by zero");

    let mut top = num.len();
    while top > 0 && num[top - 1] == 0 {
        top -= 1;
    }

    if n < crate::int::policy::div::BZ_THRESHOLD || top < 2 * n {
        div_knuth(num, den, quot, rem);
        return;
    }

    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }

    let chunks = top.div_ceil(n);
    let mut carry = [0u64; SCRATCH_LIMBS];
    let mut buf = [0u64; SCRATCH_LIMBS];
    let mut q_chunk = [0u64; SCRATCH_LIMBS];
    let mut r_chunk = [0u64; SCRATCH_LIMBS];

    let mut idx = chunks;
    while idx > 0 {
        idx -= 1;
        let lo = idx * n;
        let hi = ((idx + 1) * n).min(top);
        buf.fill(0);
        let chunk_len = hi - lo;
        buf[..chunk_len].copy_from_slice(&num[lo..lo + chunk_len]);
        buf[chunk_len..chunk_len + n].copy_from_slice(&carry[..n]);
        let buf_len = chunk_len + n;
        div_knuth(
            &buf[..buf_len],
            &den[..n],
            &mut q_chunk[..buf_len],
            &mut r_chunk[..n],
        );
        let store_end = (lo + n).min(quot.len());
        let store_len = store_end.saturating_sub(lo);
        quot[lo..lo + store_len].copy_from_slice(&q_chunk[..store_len]);
        carry[..n].copy_from_slice(&r_chunk[..n]);
    }
    let rem_n = n.min(rem.len());
    rem[..rem_n].copy_from_slice(&carry[..rem_n]);
}

// ── Const-`N` fast-arm wrappers for the fixed-width `Int<N>` types ──────

/// Const-`N` fast-arm divmod over little-endian u64 magnitude limbs.
///
/// `num`, `den`, `quot`, `rem` are all `N`-limb magnitudes (sign handling
/// is the caller's; this is an unsigned division of the magnitudes). The
/// quotient and remainder are written into `quot` / `rem`.
///
/// Because `N` is a compile-time constant, the `if N == …` ladder
/// const-folds per monomorphisation:
///
/// * `N == 1` lowers to a single native `u64` `/` + `%` (the hardware
///   `idiv`).
/// * `N == 2` widens to native `u128` `/` + `%`.
/// * `N >= 3` falls through to the shared [`div_rem_dispatch`] (Knuth-D /
///   Burnikel–Ziegler).
///
/// All three arms are behaviour-identical: truncating (Euclidean on
/// non-negative magnitudes) division. The divisor must be non-zero (the
/// caller guards this before delegating).
#[inline]
pub(crate) fn div_rem_mag_fixed<const N: usize>(
    num: &[u64; N],
    den: &[u64; N],
    quot: &mut [u64; N],
    rem: &mut [u64; N],
) {
    if N == 1 {
        let n0 = num[0];
        let d0 = den[0];
        quot[0] = n0 / d0;
        rem[0] = n0 % d0;
    } else if N == 2 {
        let n = (num[0] as u128) | ((num[1] as u128) << 64);
        let d = (den[0] as u128) | ((den[1] as u128) << 64);
        let q = n / d;
        let r = n % d;
        quot[0] = q as u64;
        quot[1] = (q >> 64) as u64;
        rem[0] = r as u64;
        rem[1] = (r >> 64) as u64;
    } else {
        div_rem_dispatch(num, den, quot, rem);
    }
}

/// Const-`N` fast-arm integer square root over little-endian u64
/// magnitude limbs. Writes `floor(sqrt(n))` into `out`.
///
/// Mirrors [`div_rem_mag_fixed`]: `N == 1` uses the native `u64::isqrt`,
/// `N == 2` uses `u128::isqrt`, and `N >= 3` falls through to the shared
/// [`isqrt_newton`] (Newton with a hardware-`f64::sqrt` seed). All arms
/// return the identical floor square root.
#[inline]
pub(crate) fn isqrt_mag_fixed<const N: usize>(n: &[u64; N], out: &mut [u64; N]) {
    if N == 1 {
        out[0] = n[0].isqrt();
    } else if N == 2 {
        let v = (n[0] as u128) | ((n[1] as u128) << 64);
        let r = v.isqrt();
        out[0] = r as u64;
        out[1] = (r >> 64) as u64;
    } else {
        isqrt_newton(n, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::policy::div::div_rem_dispatch;

    /// Pack a `[u128; N]` little-endian limb array into `[u64; 2*N]`.
    fn pack(limbs: &[u128]) -> alloc::vec::Vec<u64> {
        let mut out = alloc::vec![0u64; 2 * limbs.len()];
        for (i, &l) in limbs.iter().enumerate() {
            out[2 * i] = l as u64;
            out[2 * i + 1] = (l >> 64) as u64;
        }
        out
    }

    fn corpus() -> alloc::vec::Vec<alloc::vec::Vec<u128>> {
        alloc::vec![
            alloc::vec![0u128, 0, 0, 0],
            alloc::vec![1u128, 0, 0, 0],
            alloc::vec![u128::MAX, 0, 0, 0],
            alloc::vec![u128::MAX, u128::MAX, 0, 0],
            alloc::vec![u128::MAX, u128::MAX, u128::MAX, u128::MAX],
            alloc::vec![123u128, 456, 0, 0],
            alloc::vec![
                0x1234_5678_9abc_def0_fedc_ba98_7654_3210_u128,
                0xa5a5_a5a5_5a5a_5a5a_3c3c_3c3c_c3c3_c3c3,
                0,
                0,
            ],
        ]
    }

    /// Verify the Euclidean identity `num == q·den + r` with
    /// `0 <= r < den` reconstructs across the corpus.
    #[test]
    fn div_rem_satisfies_identity() {
        use super::super::limbs::{add_assign, cmp, is_zero, mul_schoolbook};
        for num in corpus() {
            for den in corpus() {
                let n64 = pack(&num);
                let d64 = pack(&den);
                if is_zero(&d64) {
                    continue;
                }
                let mut q64 = alloc::vec![0u64; n64.len()];
                let mut r64 = alloc::vec![0u64; n64.len()];
                div_rem(&n64, &d64, &mut q64, &mut r64);

                let mut recon = alloc::vec![0u64; q64.len() + d64.len() + 1];
                mul_schoolbook(&q64, &d64, &mut recon);
                let _ = add_assign(&mut recon, &r64);
                assert_eq!(&recon[..n64.len()], &n64[..], "q·den + r != num");
                assert!(recon[n64.len()..].iter().all(|&x| x == 0), "recon overflow");
                assert!(cmp(&r64, &d64) < 0, "remainder >= divisor");
            }
        }
    }

    /// `div_knuth` agrees with the dispatch path on the corpus.
    #[test]
    fn knuth_matches_dispatch() {
        for num in corpus() {
            for den in corpus() {
                let n64 = pack(&num);
                let d64 = pack(&den);
                let mut dn = d64.len();
                while dn > 0 && d64[dn - 1] == 0 {
                    dn -= 1;
                }
                if dn < 2 {
                    continue;
                }
                let mut q_ref = alloc::vec![0u64; n64.len()];
                let mut r_ref = alloc::vec![0u64; n64.len()];
                div_rem_dispatch(&n64, &d64, &mut q_ref, &mut r_ref);

                let mut q_knuth = alloc::vec![0u64; n64.len()];
                let mut r_knuth = alloc::vec![0u64; n64.len()];
                div_knuth(&n64, &d64, &mut q_knuth, &mut r_knuth);

                assert_eq!(q_knuth, q_ref, "knuth q mismatch");
                assert_eq!(r_knuth, r_ref, "knuth r mismatch");
            }
        }
    }

    /// `Mg3By2` matches the `div_rem` oracle on a representative corpus.
    #[test]
    fn mg3by2_matches_reference() {
        let cases: &[(u64, u64, u64, u64, u64)] = &[
            (0, 0, 1, 1u64 << 63, 0),
            (0, 1, 0, 1u64 << 63, 0),
            ((1u64 << 63) - 1, u64::MAX, u64::MAX, 1u64 << 63, 1),
            (u64::MAX - 1, u64::MAX, u64::MAX, u64::MAX, u64::MAX),
            (0, 0, 1, u64::MAX, 1),
            (
                0xc0ffee,
                0xdead_beef,
                0xface_b00c,
                (1u64 << 63) | 0xc0ffee_u64,
                0xdead_beef_face_b00c,
            ),
            (0, 1, 2, (1u64 << 63) | 1, 2),
        ];
        for &(n2, n1, n0, d1, d0) in cases {
            assert!(d1 >> 63 == 1, "d1 not normalised: {d1:#x}");
            assert!(
                n2 < d1 || (n2 == d1 && n1 < d0),
                "test precondition (n2, n1) < (d1, d0) violated"
            );
            let mg = Mg3By2::new(d1, d0);
            let (q, r1, r0) = mg.div_rem(n2, n1, n0);

            let num = alloc::vec![n0, n1, n2];
            let den = alloc::vec![d0, d1];
            let mut q_ref = alloc::vec![0u64; 3];
            let mut r_ref = alloc::vec![0u64; 3];
            div_rem(&num, &den, &mut q_ref, &mut r_ref);

            assert_eq!(q_ref[0], q, "Mg3By2 q mismatch");
            assert_eq!(q_ref[1], 0, "Mg3By2 q higher limb non-zero");
            assert_eq!(q_ref[2], 0, "Mg3By2 q higher limb non-zero");
            assert_eq!(r_ref[0], r0, "Mg3By2 r0 mismatch");
            assert_eq!(r_ref[1], r1, "Mg3By2 r1 mismatch");
        }
    }

    /// `Mg2By1` matches a reference 2-by-1 divide.
    #[test]
    fn mg2by1_matches_reference() {
        let cases: &[(u64, u64, u64)] = &[
            (0, 1, 1u64 << 63),
            (0, u64::MAX, 1u64 << 63),
            ((1u64 << 63) - 1, u64::MAX, 1u64 << 63),
            (0, 1, u64::MAX),
            (u64::MAX - 1, u64::MAX, u64::MAX),
            (12345, 67890, (1u64 << 63) | 0xdead_beef_u64),
            (u64::MAX - 1, 0, u64::MAX),
        ];
        for &(u1, u0, d) in cases {
            assert!(d >> 63 == 1);
            assert!(u1 < d);
            let mg = Mg2By1::new(d);
            let (q, r) = mg.div_rem(u1, u0);
            let num = ((u1 as u128) << 64) | (u0 as u128);
            let exp_q = (num / (d as u128)) as u64;
            let exp_r = (num % (d as u128)) as u64;
            assert_eq!((q, r), (exp_q, exp_r), "Mg2By1 mismatch");
        }
    }

    /// `div_knuth` agrees with the dispatch path on a battery of shapes.
    #[test]
    fn knuth_matches_canonical_divmod() {
        let cases: &[(&[u64], &[u64])] = &[
            (&[42], &[7]),
            (&[u64::MAX, 0], &[2]),
            (&[1, 1, 0, 0], &[3]),
            (&[u64::MAX, u64::MAX, 1, 0], &[5, 9]),
            (&[u64::MAX, u64::MAX, u64::MAX, 0], &[1, 2, 3]),
            (&[100, 0, 0], &[200, 0, 1]),
            (&[0, 0, u64::MAX, u64::MAX], &[1, 2, u64::MAX]),
        ];
        for (num, den) in cases {
            let mut q_canon = [0u64; 8];
            let mut r_canon = [0u64; 8];
            div_rem_dispatch(num, den, &mut q_canon, &mut r_canon);
            let mut q_knuth = [0u64; 8];
            let mut r_knuth = [0u64; 8];
            div_knuth(num, den, &mut q_knuth, &mut r_knuth);
            assert_eq!(q_canon, q_knuth, "quotient mismatch on {:?} / {:?}", num, den);
            assert_eq!(r_canon, r_knuth, "remainder mismatch on {:?} / {:?}", num, den);
        }
    }

    /// `div_burnikel_ziegler_with_knuth` agrees with Knuth on medium-and-
    /// large operands. Recursion engages only above the threshold cutoff.
    #[test]
    fn bz_matches_knuth() {
        let mut num = [0u64; 40];
        for (i, slot) in num.iter_mut().enumerate() {
            *slot = (i as u64)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(i as u64);
        }
        let mut den = [0u64; 20];
        for (i, slot) in den.iter_mut().enumerate() {
            *slot = ((i + 1) as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        }
        let mut q_canon = [0u64; 40];
        let mut r_canon = [0u64; 40];
        div_knuth(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_bz = [0u64; 40];
        let mut r_bz = [0u64; 40];
        div_burnikel_ziegler_with_knuth(&num, &den, &mut q_bz, &mut r_bz);
        assert_eq!(q_canon, q_bz, "BZ quotient mismatch");
        assert_eq!(r_canon, r_bz, "BZ remainder mismatch");
    }

    /// Knuth's q̂-cap path fires when `u_top >= v_top`.
    #[test]
    fn knuth_q_hat_cap_branch_matches_canonical() {
        let num: [u64; 4] = [0, 0, u64::MAX, u64::MAX >> 1];
        let den: [u64; 3] = [1, 2, u64::MAX >> 1];
        let mut q_canon = [0u64; 4];
        let mut r_canon = [0u64; 4];
        div_rem_dispatch(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_knuth = [0u64; 4];
        let mut r_knuth = [0u64; 4];
        div_knuth(&num, &den, &mut q_knuth, &mut r_knuth);
        assert_eq!(q_canon, q_knuth);
        assert_eq!(r_canon, r_knuth);
    }

    /// BZ with a numerator that has trailing zero limbs strips them off
    /// before deciding whether to recurse.
    #[test]
    fn bz_strips_numerator_trailing_zeros() {
        let mut num = [0u64; 32];
        for slot in &mut num[..16] {
            *slot = 0xCAFE_F00D;
        }
        let mut den = [0u64; 20];
        den[0] = 7;
        let mut q_canon = [0u64; 32];
        let mut r_canon = [0u64; 32];
        div_knuth(&num, &den, &mut q_canon, &mut r_canon);
        let mut q_bz = [0u64; 32];
        let mut r_bz = [0u64; 32];
        div_burnikel_ziegler_with_knuth(&num, &den, &mut q_bz, &mut r_bz);
        assert_eq!(q_canon, q_bz);
        assert_eq!(r_canon, r_bz);
    }

    // ── fast-arm wrappers ──────────────────────────────────────────────

    /// The `N == 1` and `N == 2` native fast arms agree limb-for-limb with
    /// the generic dispatch path over the divmod edge cases.
    #[test]
    fn fast_arm_div_rem_matches_generic() {
        let vals1: [u64; 8] = [
            0,
            1,
            2,
            7,
            u64::MAX,
            u64::MAX - 1,
            0x8000_0000_0000_0000,
            123_456_789,
        ];
        for &num in &vals1 {
            for &den in &vals1 {
                if den == 0 {
                    continue;
                }
                let mut fq = [0u64; 1];
                let mut fr = [0u64; 1];
                div_rem_mag_fixed::<1>(&[num], &[den], &mut fq, &mut fr);
                let mut gq = [0u64; 1];
                let mut gr = [0u64; 1];
                div_rem_dispatch(&[num], &[den], &mut gq, &mut gr);
                assert_eq!(fq, gq, "N=1 quot mismatch {num}/{den}");
                assert_eq!(fr, gr, "N=1 rem mismatch {num}%{den}");
                assert_eq!(fq[0], num / den);
                assert_eq!(fr[0], num % den);
            }
        }

        let vals2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            u128::MAX - 1,
            1u128 << 127,
            (1u128 << 64) - 1,
            1u128 << 64,
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        let to_limbs = |v: u128| [v as u64, (v >> 64) as u64];
        for &num in &vals2 {
            for &den in &vals2 {
                if den == 0 {
                    continue;
                }
                let n = to_limbs(num);
                let d = to_limbs(den);
                let mut fq = [0u64; 2];
                let mut fr = [0u64; 2];
                div_rem_mag_fixed::<2>(&n, &d, &mut fq, &mut fr);
                let mut gq = [0u64; 2];
                let mut gr = [0u64; 2];
                div_rem_dispatch(&n, &d, &mut gq, &mut gr);
                assert_eq!(fq, gq, "N=2 quot mismatch {num}/{den}");
                assert_eq!(fr, gr, "N=2 rem mismatch {num}%{den}");
                assert_eq!(fq, to_limbs(num / den));
                assert_eq!(fr, to_limbs(num % den));
            }
        }
    }

    /// The native isqrt fast arms match the generic limb isqrt.
    #[test]
    fn fast_arm_isqrt_matches_generic() {
        let vals1: [u64; 9] = [
            0,
            1,
            2,
            3,
            4,
            15,
            16,
            u64::MAX,
            (u32::MAX as u64) * (u32::MAX as u64),
        ];
        for &v in &vals1 {
            let mut f = [0u64; 1];
            isqrt_mag_fixed::<1>(&[v], &mut f);
            let mut g = [0u64; 1];
            isqrt_newton(&[v], &mut g);
            assert_eq!(f, g, "N=1 isqrt mismatch sqrt({v})");
            assert_eq!(f[0], v.isqrt());
        }

        let vals2: [u128; 8] = [
            0,
            1,
            u128::MAX,
            (1u128 << 64) - 1,
            1u128 << 64,
            1u128 << 126,
            (u64::MAX as u128) * (u64::MAX as u128),
            0x0123_4567_89ab_cdef_fedc_ba98_7654_3210,
        ];
        for &v in &vals2 {
            let n = [v as u64, (v >> 64) as u64];
            let mut f = [0u64; 2];
            isqrt_mag_fixed::<2>(&n, &mut f);
            let mut g = [0u64; 2];
            isqrt_newton(&n, &mut g);
            assert_eq!(f, g, "N=2 isqrt mismatch sqrt({v})");
            let r = v.isqrt();
            assert_eq!(f, [r as u64, (r >> 64) as u64]);
        }
    }
}
