// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `cbrt_newton` — Newton–Raphson integer cube root of `|raw| · 10^(2·SCALE)`,
//! with a single round step, sign-preserving, computed directly over `u64`
//! limbs.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the cube-root raw
//! storage is `round(cbrt(r) · 10^SCALE)`; working with
//! `n = |r| · 10^(2·SCALE)` keeps the radicand exact, takes the floor cube
//! root via the int layer's width-agnostic slice kernel
//! ([`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`]), and a single
//! half-step lands the result on the type's last place (within 0.5 ULP under
//! any of the six rounding modes). The sign of a non-zero input is preserved.
//!
//! # Generic over the storage width only
//!
//! Like [`crate::algos::sqrt::sqrt_newton`], the work-width arithmetic
//! (radicand `≤ 4N` limbs, the cube-comparison rounding) is done in a limb
//! scratch buffer rather than a work *type* `Int<4N>` (unnameable from `N` on
//! stable). Integer work dispatches *down* to the int slice kernels:
//! `icbrt_newton` for the root and
//! [`crate::int::algos::mul::mul_schoolbook::mul_schoolbook`] for the cube
//! comparisons. No work-width parameter; the policy stays a pure `(N, SCALE)`
//! matcher.

use crate::int::algos::icbrt::icbrt_newton::icbrt_newton;
use crate::int::algos::mul::mul_schoolbook::mul_schoolbook;
use crate::int::algos::support::limbs::{cmp_cross, shl};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Significant limb length of `a` (index of the highest non-zero limb + 1),
/// clamped to at least 1.
#[inline]
fn sig_len(a: &[u64]) -> usize {
    let mut l = a.len();
    while l > 1 && a[l - 1] == 0 {
        l -= 1;
    }
    l
}

/// `dst[..len] = src[..src_len] * 10^pow`, returning the new significant
/// length. `dst` must be wide enough for the result.
#[inline]
fn mul_pow10_into<const N: usize>(src: &[u64], pow: u32, dst: &mut [u64]) -> usize
where
    Limbs<N>: ComputeLimbs,
{
    let s = sig_len(src);
    dst[..s].copy_from_slice(&src[..s]);
    let mut len = s;
    let mut tmp_buf = Limbs::<N>::quad_buffered_u64();
    let tmp = tmp_buf.as_mut();
    for _ in 0..pow {
        let out = len + 1;
        for t in tmp[..out].iter_mut() {
            *t = 0;
        }
        mul_schoolbook(&dst[..len], &[10u64], &mut tmp[..out]);
        dst[..out].copy_from_slice(&tmp[..out]);
        len = sig_len(&dst[..out]);
    }
    len
}

/// `out[..2*la] = a[..la]³` (cube via two schoolbook multiplies), returning
/// the cube's significant length.
#[inline]
fn cube_into<const N: usize>(a: &[u64], la: usize, out: &mut [u64]) -> usize
where
    Limbs<N>: ComputeLimbs,
{
    let mut sq_buf = Limbs::<N>::quad_buffered_u64();
    let sq = sq_buf.as_mut();
    let sq_cap = sq.len();
    let sq_len = (2 * la).min(sq_cap);
    mul_schoolbook(&a[..la], &a[..la], &mut sq[..sq_len]);
    let sq_sig = sig_len(&sq[..sq_len]);
    let out_len = (sq_sig + la).min(sq_cap);
    for o in out[..out_len].iter_mut() {
        *o = 0;
    }
    mul_schoolbook(&sq[..sq_sig], &a[..la], &mut out[..out_len]);
    sig_len(&out[..out_len])
}

/// Newton integer cube-root kernel, computed in limbs. `N` is the storage
/// limb count backing `D<Int<N>, SCALE>`.
#[inline]
#[must_use]
pub(crate) fn cbrt_newton<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let negative = raw.is_negative();

    // ── radicand n = |raw| · 10^(2·scale) ───────────────────────────────
    let mut n_buf = Limbs::<N>::quad_buffered_u64();
    let n = n_buf.as_mut();
    let nl = mul_pow10_into::<N>(raw.unsigned_abs().as_limbs(), 2 * scale, n);

    // ── q = floor(cbrt(n)) via the int slice kernel ─────────────────────
    let mut q_buf = Limbs::<N>::quad_buffered_u64();
    let q = q_buf.as_mut();
    icbrt_newton(&n[..nl], &mut q[..nl]);
    let ql = sig_len(&q[..nl]);

    // ── single half-step round (all six modes), via cube comparisons ────
    // eight_n = 8n
    let mut eight_n_buf = Limbs::<N>::quad_buffered_u64();
    let eight_n = eight_n_buf.as_mut();
    shl(&n[..nl], 3, &mut eight_n[..nl + 1]);
    let en_len = sig_len(&eight_n[..nl + 1]);

    // t = 2q + 1; cube = t³
    let mut t_buf = Limbs::<N>::quad_buffered_u64();
    let t = t_buf.as_mut();
    shl(&q[..ql], 1, &mut t[..ql + 1]);
    // +1
    {
        let mut i = 0;
        loop {
            let (v, c) = t[i].overflowing_add(1);
            t[i] = v;
            if !c {
                break;
            }
            i += 1;
        }
    }
    let tl = sig_len(&t[..ql + 1]);
    let mut cube_buf = Limbs::<N>::quad_buffered_u64();
    let cube = cube_buf.as_mut();
    let cube_len = cube_into::<N>(t, tl, cube);

    // eight_q_cubed = (2q)³  (0 when q == 0)
    let mut two_q_buf = Limbs::<N>::quad_buffered_u64();
    let two_q = two_q_buf.as_mut();
    shl(&q[..ql], 1, &mut two_q[..ql + 1]);
    let tql = sig_len(&two_q[..ql + 1]);
    let mut eight_q_cubed_buf = Limbs::<N>::quad_buffered_u64();
    let eight_q_cubed = eight_q_cubed_buf.as_mut();
    let eqc_len = if ql == 1 && q[0] == 0 {
        eight_q_cubed[0] = 0;
        1
    } else {
        cube_into::<N>(two_q, tql, eight_q_cubed)
    };

    let cmp_cube = cmp_cross(&eight_n[..en_len], &cube[..cube_len]);
    let halfway_geq = cmp_cube >= 0;
    let halfway_gt = cmp_cube > 0;
    let tie = halfway_geq && !halfway_gt;
    let residual_nonzero = cmp_cross(&eight_n[..en_len], &eight_q_cubed[..eqc_len]) > 0;
    let q_is_odd = (q[0] & 1) == 1;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && q_is_odd),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => negative && residual_nonzero,
        RoundingMode::Ceiling => !negative && residual_nonzero,
    };
    if bump {
        let mut i = 0;
        loop {
            let (v, c) = q[i].overflowing_add(1);
            q[i] = v;
            if !c {
                break;
            }
            i += 1;
        }
    }

    // ── narrow + apply sign ─────────────────────────────────────────────
    let mut out = [0u64; N];
    out.copy_from_slice(&q[..N]);
    let v = Int::<N>::from_limbs(out);
    if negative {
        -v
    } else {
        v
    }
}
