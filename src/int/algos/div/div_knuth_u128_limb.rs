// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Knuth Algorithm D on **u128 limbs** (base 2¹²⁸) — the divide side of the
//! [`LimbSize`] axis, parked pending the `div_kernel_ab` verdict.
//!
//! ## Why base 2¹²⁸ (and why NOT a u64-`q̂` hybrid)
//!
//! The wide-tier rem/div runs [`div_knuth`] at base 2⁶⁴: its hot
//! `O(m·n)` multiply-subtract walks `n = den.len()` u64 limbs. Storing the
//! running dividend/divisor in **u128 limbs** halves the limb-slot count
//! and the carry-chain hops — the same lever that makes the u128
//! truncated-low multiply win (`mul_low_limb`).
//!
//! The multiply, though, is the opposite trade. A clean native u128
//! carry-chain needs **u128-aligned** windows, which forces a **u128
//! quotient digit** (base 2¹²⁸): a u64 `q̂` emits 64-bit digits at u64
//! offsets, so half the multiply-subtract windows straddle a u128 boundary
//! and cannot run as aligned u128 ops (a u64-`q̂` scheme therefore degrades
//! to the u64 loop — no win, which is why the earlier scaffold delegated to
//! a u64 lens). Base 2¹²⁸ keeps every window aligned, at the cost of a
//! `q̂·v[i]` product that is a full 128×128→256 (4 u64-mults) instead of
//! 64×64→128 (1): **2× the limb-multiplies for ½ the carry-chain.** Whether
//! that nets out ahead of base-2⁶⁴ is a per-width microbench question
//! (`benches/micro/div_kernel_ab.rs`); the campaign's earlier estimate was
//! a wash, hence this kernel stays a parked candidate until the bench says
//! otherwise.
//!
//! ## Shape
//!
//! A PURE slice engine, drop-in alongside [`div_knuth`]: same `&[u64]`
//! operands and `&[u64]` quotient/remainder, so if it wins it wires as a
//! `LimbSize`-gated `Algorithm` arm in [`crate::int::policy::div_rem`]
//! (selected when the effective limb counts are EVEN and wide — packing
//! pairs two u64 per u128, so an odd effective count has no u128 form). It
//! normalises + packs in u64 space (reusing [`div_knuth`]'s proven
//! normalisation: a top-u64-limb MSB also sets the top u128 limb's bit
//! 127), runs base 2¹²⁸, then unpacks — no per-tier type, no macro
//! duplication. Odd/single-limb shapes fall back to [`div_knuth`].
//!
//! Bit-identical to [`div_knuth`] (the `#[cfg(test)]` differential below);
//! NOT WIRED.
//!
//! [`LimbSize`]: crate::int::types::compute_int::LimbSize

#![allow(dead_code)]

use crate::int::algos::div::div_mg::Mg3By2;
use crate::int::types::compute_int::{Limb, MAX_SINGLE_LIMBS};

/// u128-limb working scratch: half the u64 `MAX_SINGLE_LIMBS`, +2 slack
/// (the one-above window limb plus an even-rounding limb).
const SCRATCH_LIMBS_128: usize = MAX_SINGLE_LIMBS / 2 + 2;

/// Knuth Algorithm D at base 2¹²⁸. `num` / `den` are little-endian u64
/// slices; `quot` / `rem` are written in u64 limbs to match [`div_knuth`]'s
/// contract bit-for-bit. Even effective limb counts run the u128 core;
/// odd / single-limb / `num < den` shapes fall back to [`div_knuth`].
pub(crate) fn div_knuth_u128_limb(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }

    // Effective u64 limb counts (strip leading zeros).
    let mut n64 = den.len();
    while n64 > 0 && den[n64 - 1] == 0 {
        n64 -= 1;
    }
    assert!(n64 > 0, "div_knuth_u128_limb: divide by zero");
    let mut top64 = num.len();
    while top64 > 0 && num[top64 - 1] == 0 {
        top64 -= 1;
    }
    if top64 < n64 {
        let copy = num.len().min(rem.len());
        rem[..copy].copy_from_slice(&num[..copy]);
        return;
    }

    // The u128 core needs an EVEN-u64-limb-count divisor of at least TWO
    // u128 limbs (`n128 >= 2`, i.e. `n64 >= 4`): the base-2¹²⁸ 3-by-2 q̂
    // refinement reads `v[n128 - 2]`. Everything else — odd `n64` (no exact
    // u128 form), or `n64 < 4` — defers to base-2⁶⁴ Knuth.
    if n64 < 4 || n64 % 2 != 0 {
        crate::int::algos::div::div_knuth::div_knuth(num, den, quot, rem);
        return;
    }

    // Normalise so the divisor's top u64 limb has its MSB set; this ALSO
    // normalises the top u128 limb (its bit 127 = the top u64 limb's bit
    // 63), so the packed divisor is base-2¹²⁸ normalised. Shift in u64
    // space (div_knuth's proven path), then pack pairs of u64 into u128.
    let shift = den[n64 - 1].leading_zeros();
    let mut u64buf = [0u64; MAX_SINGLE_LIMBS];
    let mut v64buf = [0u64; MAX_SINGLE_LIMBS];
    debug_assert!(top64 < MAX_SINGLE_LIMBS && n64 <= MAX_SINGLE_LIMBS);

    if shift == 0 {
        u64buf[..top64].copy_from_slice(&num[..top64]);
        v64buf[..n64].copy_from_slice(&den[..n64]);
    } else {
        let mut carry = 0u64;
        for i in 0..top64 {
            let val = num[i];
            u64buf[i] = (val << shift) | carry;
            carry = val >> (64 - shift);
        }
        u64buf[top64] = carry;
        carry = 0;
        for i in 0..n64 {
            let val = den[i];
            v64buf[i] = (val << shift) | carry;
            carry = val >> (64 - shift);
        }
    }

    // Round the dividend up to an even u64 length (so it packs cleanly),
    // including the normalisation carry limb at `u64buf[top64]`.
    let mut u_len64 = if u64buf[top64] != 0 { top64 + 1 } else { top64 };
    u_len64 += u_len64 & 1; // round up to even
    let n128 = n64 / 2;
    let u_len128 = u_len64 / 2;

    // Pack into u128 limbs (little-endian: limb = lo | hi << 64).
    let mut u = [0u128; SCRATCH_LIMBS_128];
    let mut v = [0u128; SCRATCH_LIMBS_128];
    debug_assert!(u_len128 < SCRATCH_LIMBS_128 && n128 <= SCRATCH_LIMBS_128);
    <u128 as Limb>::pack(&u64buf[..u_len64], &mut u[..u_len128]);
    <u128 as Limb>::pack(&v64buf[..n64], &mut v[..n128]);

    // Base-2¹²⁸ Knuth D. The quotient has `m128 + 1` u128 digits; `u` has a
    // zeroed limb above the live dividend (`u[u_len128]`) for the window top.
    let m128 = u_len128 - n128;
    knuth_d_base_u128(&mut u, &v, n128, m128, quot);

    // Unpack the remainder (low `n128` u128 limbs of `u` → `n64` u64 limbs),
    // denormalise by `shift`.
    let mut r64 = [0u64; MAX_SINGLE_LIMBS];
    <u128 as Limb>::unpack(&u[..n128], &mut r64[..n64]);
    if shift == 0 {
        let copy = n64.min(rem.len());
        rem[..copy].copy_from_slice(&r64[..copy]);
    } else {
        for i in 0..n64 {
            if i < rem.len() {
                let lo = r64[i] >> shift;
                let hi = if i + 1 < n64 { r64[i + 1] << (64 - shift) } else { 0 };
                rem[i] = lo | hi;
            }
        }
    }
}

/// 256-by-128 → 128 division `(hi·2¹²⁸ + lo) / d`, requiring `hi < d` and
/// `d` normalised (bit 127 set). Returns `(q, r)` with `q < 2¹²⁸`.
///
/// Implemented as a base-2⁶⁴ n=2 Knuth divide of the four u64 limbs by the
/// two u64 limbs of `d`, two exact [`Mg3By2`] passes — no software u256
/// reciprocal. `hi < d` guarantees each pass's `(n2, n1) < (d1, d0)`
/// precondition and a 128-bit quotient.
#[inline]
fn div_256_by_128(hi: u128, lo: u128, d: u128) -> (u128, u128) {
    debug_assert!(hi < d, "div_256_by_128: high word must be < divisor");
    let d1 = (d >> 64) as u64;
    let d0 = d as u64;
    debug_assert!(d1 >> 63 == 1, "div_256_by_128: divisor must be normalised");
    let a3 = (hi >> 64) as u64;
    let a2 = hi as u64;
    let a1 = (lo >> 64) as u64;
    let a0 = lo as u64;

    let mg = Mg3By2::new(d1, d0);
    // High quotient limb: (a3·B² + a2·B + a1) / (d1·B + d0).
    let (q1, r1, r0) = mg.div_rem(a3, a2, a1);
    // Low quotient limb: (r·B + a0) / (d1·B + d0).
    let (q0, s1, s0) = mg.div_rem(r1, r0, a0);

    let q = ((q1 as u128) << 64) | (q0 as u128);
    let r = ((s1 as u128) << 64) | (s0 as u128);
    (q, r)
}

/// Base-2¹²⁸ Knuth D core: u128-limb running dividend `u` (with a zeroed
/// limb above the live window) and divisor `v` (length `n128`, normalised),
/// emitting `m128 + 1` u128 quotient digits into `quot` as pairs of u64
/// limbs (`quot[2·j] = q̂ as u64`, `quot[2·j + 1] = (q̂ >> 64) as u64`).
///
/// The multiply-subtract and add-back run natively on u128 limbs (one
/// 128×128→256 [`Limb::widening_mul`] per limb, a u128 carry-merge), the
/// aligned-window win this kernel exists to test.
#[inline]
fn knuth_d_base_u128(u: &mut [u128], v: &[u128], n128: usize, m128: usize, quot: &mut [u64]) {
    let v_top = v[n128 - 1]; // normalised: bit 127 set
    let v_below = v[n128 - 2];

    let mut j = m128 + 1;
    while j > 0 {
        j -= 1;
        let jn = j + n128;
        let u_top = u[jn];
        let u_next = u[jn - 1];

        // q̂ = min(floor((u_top·2¹²⁸ + u_next) / v_top), 2¹²⁸ − 1), with the
        // standard `u_top >= v_top` clamp so `div_256_by_128`'s `hi < d`
        // precondition holds.
        let (mut q_hat, mut r_hat, overflow) = if u_top >= v_top {
            (u128::MAX, u_next.wrapping_add(v_top), u_next.wrapping_add(v_top) < u_next)
        } else {
            let (q, r) = div_256_by_128(u_top, u_next, v_top);
            (q, r, false)
        };

        // Refinement against v[n128-2]: while q̂·v_below > r_hat·2¹²⁸ + u[jn-2],
        // decrement q̂ (and bump r_hat by v_top). `overflow` means r_hat has
        // already wrapped past 2¹²⁸, so the comparison is satisfied.
        if !overflow {
            loop {
                let (p_lo, p_hi) = <u128 as Limb>::widening_mul(q_hat, v_below);
                if p_hi < r_hat || (p_hi == r_hat && p_lo <= u[jn - 2]) {
                    break;
                }
                q_hat = q_hat.wrapping_sub(1);
                let (nr, of) = r_hat.overflowing_add(v_top);
                r_hat = nr;
                if of {
                    break;
                }
            }
        }

        // D4: u[j..=j+n128] -= q̂ · v[0..n128], native u128 carry-merge. The
        // carry is a single u128 (the propagated high word): `q̂·v[i]` is a
        // 256-bit (p_lo, p_hi); add the incoming carry into p_lo, subtract
        // p_lo from u[j+i], propagate p_hi + borrow. Bounds (mirroring the
        // base-2⁶⁴ proof): p_hi ≤ 2¹²⁸ − 2, so p_hi + 1 and carry + borrow
        // never overflow the u128.
        let mut carry: u128 = 0;
        let mut i = 0;
        while i < n128 {
            let (p_lo, p_hi) = <u128 as Limb>::widening_mul(q_hat, v[i]);
            let (acc_lo, k) = p_lo.overflowing_add(carry);
            let acc_hi = p_hi + (k as u128);
            let (res, b) = u[j + i].overflowing_sub(acc_lo);
            u[j + i] = res;
            carry = acc_hi + (b as u128);
            i += 1;
        }
        let (s2, b1) = u[jn].overflowing_sub(carry);
        u[jn] = s2;

        // Over-estimate correction: q̂ was at most 1 too big (the 3-by-2
        // refinement bounds the error), so a single add-back of v restores
        // a non-negative dividend.
        if b1 {
            q_hat = q_hat.wrapping_sub(1);
            let mut carry: u128 = 0;
            let mut i = 0;
            while i < n128 {
                let (s1, c1) = u[j + i].overflowing_add(v[i]);
                let (s2, c2) = s1.overflowing_add(carry);
                u[j + i] = s2;
                carry = (c1 as u128) + (c2 as u128);
                i += 1;
            }
            u[jn] = u[jn].wrapping_add(carry);
        }

        // Store the u128 quotient digit as two u64 limbs (little-endian).
        let lo64 = q_hat as u64;
        let hi64 = (q_hat >> 64) as u64;
        if 2 * j < quot.len() {
            quot[2 * j] = lo64;
        }
        if 2 * j + 1 < quot.len() {
            quot[2 * j + 1] = hi64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::div_knuth_u128_limb;
    use crate::int::algos::div::div_knuth::div_knuth;

    // Bit-identity vs the production base-2⁶⁴ div_knuth on even-limb,
    // multi-limb-divisor shapes (the regime this kernel handles), across a
    // spread of even divisor widths up to the widest wide tier. DO NOT run
    // as part of a sweep — focused differential only.
    #[test]
    fn u128_limb_knuth_matches_div_knuth() {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        // Even divisor widths covering D76(4)..D1232(64) + odd-shaped
        // dividends to exercise the even-rounding of the dividend length.
        for &n64 in &[2usize, 4, 6, 8, 12, 16, 24, 32, 48, 64] {
            for _ in 0..400 {
                let extra = 1 + (next() % (n64 as u64 + 2)) as usize; // 1..=n64+2 u64
                let top64 = n64 + extra;
                let mut num = alloc::vec![0u64; top64];
                let mut den = alloc::vec![0u64; n64];
                for x in num.iter_mut() {
                    *x = next();
                }
                for x in den.iter_mut() {
                    *x = next();
                }
                if den[n64 - 1] == 0 {
                    den[n64 - 1] = 1; // ensure effective top limb
                }
                let mut q_ref = alloc::vec![0u64; top64 + 1];
                let mut r_ref = alloc::vec![0u64; top64 + 1];
                div_knuth(&num, &den, &mut q_ref, &mut r_ref);
                let mut q_c = alloc::vec![0u64; top64 + 1];
                let mut r_c = alloc::vec![0u64; top64 + 1];
                div_knuth_u128_limb(&num, &den, &mut q_c, &mut r_c);
                assert_eq!(q_c, q_ref, "quot mismatch n64={n64} num={num:?} den={den:?}");
                assert_eq!(
                    r_c[..n64],
                    r_ref[..n64],
                    "rem mismatch n64={n64} num={num:?} den={den:?}"
                );
            }
        }
    }

    // The odd / single-limb fallback returns div_knuth's exact result.
    #[test]
    fn u128_limb_knuth_odd_falls_back() {
        let num = alloc::vec![0x1234u64, 0x5678, 0x9abc, 0xdef0, 0x1111];
        for den in [
            alloc::vec![0x3u64],                 // single limb
            alloc::vec![0x7u64, 0x9, 0xb],       // odd (3) limbs
        ] {
            let mut q_ref = alloc::vec![0u64; num.len() + 1];
            let mut r_ref = alloc::vec![0u64; num.len() + 1];
            div_knuth(&num, &den, &mut q_ref, &mut r_ref);
            let mut q_c = alloc::vec![0u64; num.len() + 1];
            let mut r_c = alloc::vec![0u64; num.len() + 1];
            div_knuth_u128_limb(&num, &den, &mut q_c, &mut r_c);
            assert_eq!(q_c, q_ref, "fallback quot mismatch den={den:?}");
            assert_eq!(r_c, r_ref, "fallback rem mismatch den={den:?}");
        }
    }
}
