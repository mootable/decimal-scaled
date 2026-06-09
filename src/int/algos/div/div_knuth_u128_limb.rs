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
//! [`LimbSize`]: crate::int::types::compute_limbs::LimbSize

use crate::int::algos::div::div_knuth::knuth_d_core;
use crate::int::types::compute_limbs::{Limb, MAX_SINGLE_LIMBS};

/// u128-limb working scratch: half the u64 `MAX_SINGLE_LIMBS`, +2 slack
/// (the one-above window limb plus an even-rounding limb).
const SCRATCH_LIMBS_128: usize = MAX_SINGLE_LIMBS / 2 + 2;

/// Knuth Algorithm D at base 2¹²⁸ — build-max-scratch wrapper. Allocates the
/// u64 normalisation buffers and the packed u128 `u`/`v` at the build-max
/// width and delegates to [`div_knuth_u128_limb_into`]. The slice
/// [`dispatch`](crate::int::policy::div_rem::dispatch) calls this; a
/// concrete-`N` caller that can size the scratch exactly
/// (`Int<N>: ComputeLimbs`) calls `div_knuth_u128_limb_into` directly with its
/// own buffer family.
pub(crate) fn div_knuth_u128_limb(num: &[u64], den: &[u64], quot: &mut [u64], rem: &mut [u64]) {
    let mut u64buf = [0u64; MAX_SINGLE_LIMBS];
    let mut v64buf = [0u64; MAX_SINGLE_LIMBS];
    let mut u = [0u128; SCRATCH_LIMBS_128];
    let mut v = [0u128; SCRATCH_LIMBS_128];
    div_knuth_u128_limb_into(
        num, den, quot, rem, &mut u64buf, &mut v64buf, &mut u, &mut v,
    );
}

/// Base-2¹²⁸ Knuth Algorithm D in caller-provided scratch — the exact-scratch
/// sibling of [`div_knuth_u128_limb`]. `num` / `den` are little-endian u64
/// slices; `quot` / `rem` are written in u64 limbs to match [`div_knuth`]'s
/// contract bit-for-bit. Even effective limb counts run the u128 core; odd /
/// single-limb / `num < den` shapes fall back to [`div_knuth_into`].
///
/// A concrete-`N` caller sources the four scratch buffers from its
/// `ComputeLimbs` family — for the decimal `/` wide shape (`2N`-dividend,
/// `N`-divisor): `u64buf` = `double_buffered_u64`, `v64buf` =
/// `single_buffered_u64`, `u` = `double_buffered_u128`, `v` = `single_u128`.
/// All four slices are **zeroed here**, so the caller may reuse them across
/// calls. Required minimum lengths: `u64buf ≥ num.len() + 2`,
/// `v64buf ≥ den.len()`, `u ≥ ⌈(num.len()+2)/2⌉ + 1`, `v ≥ ⌈den.len()/2⌉`.
/// `u64buf` is reused as the remainder unpack scratch after the dividend has
/// been packed into `u`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn div_knuth_u128_limb_into(
    num: &[u64],
    den: &[u64],
    quot: &mut [u64],
    rem: &mut [u64],
    u64buf: &mut [u64],
    v64buf: &mut [u64],
    u: &mut [u128],
    v: &mut [u128],
) {
    for q in quot.iter_mut() {
        *q = 0;
    }
    for r in rem.iter_mut() {
        *r = 0;
    }
    // Zero the caller's scratch — the pack relies on the high limbs being
    // zero (the window-top `u[u_len128]`, the dividend's even-rounding limb),
    // and the caller may have reused these buffers.
    for x in u64buf.iter_mut() {
        *x = 0;
    }
    for x in v64buf.iter_mut() {
        *x = 0;
    }
    for x in u.iter_mut() {
        *x = 0;
    }
    for x in v.iter_mut() {
        *x = 0;
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
    // u128 form), or `n64 < 4` — defers to base-2⁶⁴ Knuth, reusing the
    // caller's (now zeroed) u64 scratch as Knuth's `u`/`v`.
    if n64 < 4 || n64 % 2 != 0 {
        crate::int::algos::div::div_knuth::div_knuth_into(num, den, quot, rem, u64buf, v64buf);
        return;
    }

    // Normalise so the divisor's top u64 limb has its MSB set; this ALSO
    // normalises the top u128 limb (its bit 127 = the top u64 limb's bit
    // 63), so the packed divisor is base-2¹²⁸ normalised. Shift in u64
    // space (div_knuth's proven path), then pack pairs of u64 into u128.
    let shift = den[n64 - 1].leading_zeros();
    debug_assert!(top64 < u64buf.len() && n64 <= v64buf.len());

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
    debug_assert!(u_len128 < u.len() && n128 <= v.len());
    <u128 as Limb>::pack(&u64buf[..u_len64], &mut u[..u_len128]);
    <u128 as Limb>::pack(&v64buf[..n64], &mut v[..n128]);

    // Base-2¹²⁸ Knuth D — the `L = u128` monomorphisation of the limb-generic
    // [`knuth_d_core`](crate::int::algos::div::div_knuth::knuth_d_core) (ONE
    // kernel shared with the base-2⁶⁴ `div_knuth`). The quotient has `m128 + 1`
    // u128 digits, written into `quot` as little-endian u64 pairs by
    // [`DivLimb::store_quot_digit`]; `u` has a zeroed limb above the live
    // dividend (`u[u_len128]`) for the window top.
    let m128 = u_len128 - n128;
    knuth_d_core::<u128>(&mut u[..=u_len128], &v[..n128], n128, m128, quot);

    // Unpack the remainder (low `n128` u128 limbs of `u` → `n64` u64 limbs)
    // into `u64buf` (reused as `r64` now the dividend is consumed), then
    // denormalise by `shift`.
    let r64 = u64buf;
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

#[cfg(test)]
mod tests {
    use super::div_knuth_u128_limb;
    #[cfg(feature = "wide")]
    use super::div_knuth_u128_limb_into;
    use crate::int::algos::div::div_knuth::div_knuth;

    // Bit-identity vs the production base-2⁶⁴ div_knuth on even-limb,
    // multi-limb-divisor shapes (the regime this kernel handles), across a
    // spread of even divisor widths up to the widest wide tier. DO NOT run
    // as part of a sweep — focused differential only.
    // Exercises divisor widths up to 64 limbs (D1232) — needs xx-wide scratch.
    #[cfg(feature = "xx-wide")]
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
                let mut num = vec![0u64; top64];
                let mut den = vec![0u64; n64];
                for x in num.iter_mut() {
                    *x = next();
                }
                for x in den.iter_mut() {
                    *x = next();
                }
                if den[n64 - 1] == 0 {
                    den[n64 - 1] = 1; // ensure effective top limb
                }
                let mut q_ref = vec![0u64; top64 + 1];
                let mut r_ref = vec![0u64; top64 + 1];
                div_knuth(&num, &den, &mut q_ref, &mut r_ref);
                let mut q_c = vec![0u64; top64 + 1];
                let mut r_c = vec![0u64; top64 + 1];
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
        let num = vec![0x1234u64, 0x5678, 0x9abc, 0xdef0, 0x1111];
        for den in [
            vec![0x3u64],                 // single limb
            vec![0x7u64, 0x9, 0xb],       // odd (3) limbs
        ] {
            let mut q_ref = vec![0u64; num.len() + 1];
            let mut r_ref = vec![0u64; num.len() + 1];
            div_knuth(&num, &den, &mut q_ref, &mut r_ref);
            let mut q_c = vec![0u64; num.len() + 1];
            let mut r_c = vec![0u64; num.len() + 1];
            div_knuth_u128_limb(&num, &den, &mut q_c, &mut r_c);
            assert_eq!(q_c, q_ref, "fallback quot mismatch den={den:?}");
            assert_eq!(r_c, r_ref, "fallback rem mismatch den={den:?}");
        }
    }

    // `div_knuth_u128_limb_into` (the exact-scratch sibling) on the decimal
    // `/` wide shape — a `2N`-limb scaled numerator over an `N`-limb divisor —
    // with the buffers sized by the SAME `ComputeLimbs` formulas `div_widen_scale`
    // uses (`u64buf`=double_buffered_u64, `v64buf`=single_buffered_u64,
    // `u`=double_buffered_u128, `v`=single_u128). Validates both the result
    // (bit-identical to `div_knuth`) AND that the exact buffers are large
    // enough (run in debug, the `debug_assert!`s in the engine fire on
    // undersizing). Even storage widths where the matcher engages u128 (≥24).
    // Exact-scratch shapes for storage widths up to 64 (D1232) — xx-wide.
    #[cfg(feature = "xx-wide")]
    #[test]
    fn u128_limb_into_exact_scratch_wide_shape() {
        let mut state: u64 = 0xD1B5_4A32_D192_ED03;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for &n in &[24usize, 32, 48, 64] {
            // Exact-scratch sizes — the `ComputeLimbs` family formulas.
            let u64buf_len = 2 * n + n.div_ceil(2); // double_buffered_u64
            let v64buf_len = n + 2; // single_buffered_u64
            let u128_u_len = (2 * n + n.div_ceil(2)).div_ceil(2); // double_buffered_u128
            let u128_v_len = n.div_ceil(2); // single_u128
            for _ in 0..200 {
                let top = 2 * n; // full wide `2N` dividend
                let mut num = vec![0u64; top];
                let mut den = vec![0u64; n];
                for x in num.iter_mut() {
                    *x = next();
                }
                for x in den.iter_mut() {
                    *x = next();
                }
                den[n - 1] |= 1 << 63; // full-width even divisor (`den_n == n`)
                let mut q_ref = vec![0u64; top + 1];
                let mut r_ref = vec![0u64; top + 1];
                div_knuth(&num, &den, &mut q_ref, &mut r_ref);

                let mut q_c = vec![0u64; top + 1];
                let mut r_c = vec![0u64; n];
                let mut u64buf = vec![0u64; u64buf_len];
                let mut v64buf = vec![0u64; v64buf_len];
                let mut u128_u = vec![0u128; u128_u_len];
                let mut u128_v = vec![0u128; u128_v_len];
                div_knuth_u128_limb_into(
                    &num,
                    &den,
                    &mut q_c,
                    &mut r_c,
                    &mut u64buf,
                    &mut v64buf,
                    &mut u128_u,
                    &mut u128_v,
                );
                assert_eq!(q_c, q_ref, "into quot mismatch n={n}");
                assert_eq!(r_c[..n], r_ref[..n], "into rem mismatch n={n}");
            }
        }
    }
}
