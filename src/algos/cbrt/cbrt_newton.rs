// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `cbrt_newton` — Newton–Raphson integer cube root of `|raw| · 10^(2·SCALE)`,
//! with a single round step, sign-preserving, computed directly over `u64`
//! limbs.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the cube-root raw
//! storage is `round(cbrt(r) · 10^SCALE)`; working with the radicand
//! `|r| · 10^(2·SCALE)` keeps it exact, takes the floor cube
//! root via the int layer's width-agnostic slice kernel
//! ([`crate::int::algos::icbrt::icbrt_newton::icbrt_newton`]), and a single
//! half-step lands the result on the type's last place (within 0.5 ULP under
//! any rounding mode). The sign of a non-zero input is preserved.
//!
//! # Generic over the storage width only
//!
//! Like [`crate::algos::sqrt::sqrt_newton`], the work-width arithmetic
//! (radicand `≤ 4N` limbs, the cube-comparison rounding) is done in a limb
//! scratch buffer rather than a work *type* `Int<4N>` (unnameable from `N` on
//! stable). Integer work dispatches *down* to the int slice kernels:
//! `icbrt_newton` for the root and the multiply matcher's slice door
//! [`crate::int::policy::mul::dispatch_slice`] for the cube comparisons (so
//! the schoolbook-vs-Karatsuba choice is the matcher's, not hardcoded). No
//! work-width parameter; the policy stays a pure `(N, SCALE)` matcher.

use crate::int::algos::icbrt::icbrt_newton::icbrt_newton;
use crate::int::policy::mul::dispatch_slice as mul_slice;
use crate::int::algos::support::limbs::{cmp_cross, shl};
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Significant limb length of `limbs` (index of the highest non-zero limb
/// + 1), clamped to at least 1.
#[inline]
fn sig_len(limbs: &[u64]) -> usize {
    let mut len = limbs.len();
    while len > 1 && limbs[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// `dst[..len] = src[..src_len] * 10^exponent`, returning the new significant
/// length. `dst` must be wide enough for the result.
#[inline]
fn mul_pow10_into<const N: usize>(src: &[u64], exponent: u32, dst: &mut [u64]) -> usize
where
    Limbs<N>: ComputeLimbs,
{
    let src_len = sig_len(src);
    dst[..src_len].copy_from_slice(&src[..src_len]);
    let mut len = src_len;
    let mut product_buf = Limbs::<N>::quad_buffered_u64();
    let product = product_buf.as_mut();
    for _ in 0..exponent {
        let product_len = len + 1;
        for limb in product[..product_len].iter_mut() {
            *limb = 0;
        }
        mul_slice(&dst[..len], &[10u64], &mut product[..product_len]);
        dst[..product_len].copy_from_slice(&product[..product_len]);
        len = sig_len(&dst[..product_len]);
    }
    len
}

/// `out[..2*base_len] = base[..base_len]³` (cube via two schoolbook
/// multiplies), returning the cube's significant length.
#[inline]
fn cube_into<const N: usize>(base: &[u64], base_len: usize, out: &mut [u64]) -> usize
where
    Limbs<N>: ComputeLimbs,
{
    let mut square_buf = Limbs::<N>::quad_buffered_u64();
    let square = square_buf.as_mut();
    let square_cap = square.len();
    let square_len = (2 * base_len).min(square_cap);
    mul_slice(&base[..base_len], &base[..base_len], &mut square[..square_len]);
    let square_sig_len = sig_len(&square[..square_len]);
    let out_len = (square_sig_len + base_len).min(square_cap);
    for limb in out[..out_len].iter_mut() {
        *limb = 0;
    }
    mul_slice(&square[..square_sig_len], &base[..base_len], &mut out[..out_len]);
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
    let is_negative = raw.is_negative();

    // ── radicand = |raw| · 10^(2·scale) ─────────────────────────────────
    let mut radicand_buf = Limbs::<N>::quad_buffered_u64();
    let radicand = radicand_buf.as_mut();
    let radicand_len =
        mul_pow10_into::<N>(raw.unsigned_abs().as_limbs(), 2 * scale, radicand);

    // ── root = floor(cbrt(radicand)) via the int slice kernel ───────────
    let mut root_buf = Limbs::<N>::quad_buffered_u64();
    let root = root_buf.as_mut();
    icbrt_newton(&radicand[..radicand_len], &mut root[..radicand_len]);
    let root_len = sig_len(&root[..radicand_len]);

    // ── single half-step round (every mode), via cube comparisons ────
    // eight_radicand = 8·radicand
    let mut eight_radicand_buf = Limbs::<N>::quad_buffered_u64();
    let eight_radicand = eight_radicand_buf.as_mut();
    shl(&radicand[..radicand_len], 3, &mut eight_radicand[..radicand_len + 1]);
    let eight_radicand_len = sig_len(&eight_radicand[..radicand_len + 1]);

    // doubled_midpoint = 2·root + 1; cube = doubled_midpoint³
    let mut doubled_midpoint_buf = Limbs::<N>::quad_buffered_u64();
    let doubled_midpoint = doubled_midpoint_buf.as_mut();
    shl(&root[..root_len], 1, &mut doubled_midpoint[..root_len + 1]);
    // +1
    {
        let mut i = 0;
        loop {
            let (sum, carry) = doubled_midpoint[i].overflowing_add(1);
            doubled_midpoint[i] = sum;
            if !carry {
                break;
            }
            i += 1;
        }
    }
    let doubled_midpoint_len = sig_len(&doubled_midpoint[..root_len + 1]);
    let mut cube_buf = Limbs::<N>::quad_buffered_u64();
    let cube = cube_buf.as_mut();
    let cube_len = cube_into::<N>(doubled_midpoint, doubled_midpoint_len, cube);

    // eight_root_cubed = (2·root)³  (0 when root == 0)
    let mut two_root_buf = Limbs::<N>::quad_buffered_u64();
    let two_root = two_root_buf.as_mut();
    shl(&root[..root_len], 1, &mut two_root[..root_len + 1]);
    let two_root_len = sig_len(&two_root[..root_len + 1]);
    let mut eight_root_cubed_buf = Limbs::<N>::quad_buffered_u64();
    let eight_root_cubed = eight_root_cubed_buf.as_mut();
    let eight_root_cubed_len = if root_len == 1 && root[0] == 0 {
        eight_root_cubed[0] = 0;
        1
    } else {
        cube_into::<N>(two_root, two_root_len, eight_root_cubed)
    };

    let cmp_cube = cmp_cross(&eight_radicand[..eight_radicand_len], &cube[..cube_len]);
    let halfway_geq = cmp_cube >= 0;
    let halfway_gt = cmp_cube > 0;
    let tie = halfway_geq && !halfway_gt;
    let residual_nonzero = cmp_cross(
        &eight_radicand[..eight_radicand_len],
        &eight_root_cubed[..eight_root_cubed_len]) > 0;
    // Last decimal digit of the root magnitude, which spans `root_len` limbs —
    // the low limb alone cannot carry it.
    let root_mod_10 = crate::support::rounding::limbs_mod_10(&root[..root_len]);
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && root_mod_10 & 1 == 1),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => is_negative && residual_nonzero,
        RoundingMode::Ceiling => !is_negative && residual_nonzero,
        // `root` is the magnitude, so away-from-zero is a bump either sign.
        RoundingMode::AwayFromZero => residual_nonzero,
        RoundingMode::ZeroFiveUp => residual_nonzero && matches!(root_mod_10, 0 | 5),
    };
    if bump {
        let mut i = 0;
        loop {
            let (sum, carry) = root[i].overflowing_add(1);
            root[i] = sum;
            if !carry {
                break;
            }
            i += 1;
        }
    }

    // ── narrow + apply sign ─────────────────────────────────────────────
    let mut root_limbs = [0u64; N];
    root_limbs.copy_from_slice(&root[..N]);
    let root_magnitude = Int::<N>::from_limbs(root_limbs);
    if is_negative {
        -root_magnitude
    } else {
        root_magnitude
    }
}
