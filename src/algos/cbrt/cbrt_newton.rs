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
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Limb scratch budget — matches the int root kernels' `SCRATCH_LIMBS`
/// (288 u64), covering the widest radicand (`4 · Int<64>` = 256 limbs).
use crate::int::algos::support::limbs::work_scratch;

const SCRATCH: usize = work_scratch(4);

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
fn mul_pow10_into(src: &[u64], pow: u32, dst: &mut [u64]) -> usize {
    let s = sig_len(src);
    dst[..s].copy_from_slice(&src[..s]);
    let mut len = s;
    let mut tmp = [0u64; SCRATCH];
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
fn cube_into(a: &[u64], la: usize, out: &mut [u64]) -> usize {
    let mut sq = [0u64; SCRATCH];
    let sq_len = (2 * la).min(SCRATCH);
    mul_schoolbook(&a[..la], &a[..la], &mut sq[..sq_len]);
    let sq_sig = sig_len(&sq[..sq_len]);
    let out_len = (sq_sig + la).min(SCRATCH);
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
pub(crate) fn cbrt_newton<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode) -> Int<N> {
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let negative = raw.is_negative();

    // ── radicand n = |raw| · 10^(2·scale) ───────────────────────────────
    let mut n = [0u64; SCRATCH];
    let nl = mul_pow10_into(raw.unsigned_abs().as_limbs(), 2 * scale, &mut n);

    // ── q = floor(cbrt(n)) via the int slice kernel ─────────────────────
    let mut q = [0u64; SCRATCH];
    icbrt_newton(&n[..nl], &mut q[..nl]);
    let ql = sig_len(&q[..nl]);

    // ── single half-step round (all six modes), via cube comparisons ────
    // eight_n = 8n
    let mut eight_n = [0u64; SCRATCH];
    shl(&n[..nl], 3, &mut eight_n[..nl + 1]);
    let en_len = sig_len(&eight_n[..nl + 1]);

    // t = 2q + 1; cube = t³
    let mut t = [0u64; SCRATCH];
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
    let mut cube = [0u64; SCRATCH];
    let cube_len = cube_into(&t, tl, &mut cube);

    // eight_q_cubed = (2q)³  (0 when q == 0)
    let mut two_q = [0u64; SCRATCH];
    shl(&q[..ql], 1, &mut two_q[..ql + 1]);
    let tql = sig_len(&two_q[..ql + 1]);
    let mut eight_q_cubed = [0u64; SCRATCH];
    let eqc_len = if ql == 1 && q[0] == 0 {
        eight_q_cubed[0] = 0;
        1
    } else {
        cube_into(&two_q, tql, &mut eight_q_cubed)
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
