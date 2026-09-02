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
//! `icbrt_newton_into` for the root and the multiply matcher's slice door
//! [`crate::int::policy::mul::dispatch_slice`] for the cube comparisons (so
//! the schoolbook-vs-Karatsuba choice is the matcher's, not hardcoded). No
//! work-width parameter; the policy stays a pure `(N, SCALE)` matcher.
//!
//! # Exact scratch — every buffer sized from `N`, none from the build
//!
//! `N` is concrete here, so every working buffer comes from `ComputeLimbs` on
//! the `Limbs<N>` carrier — including the ones the ROOT needs. The slice
//! kernel has no `N` of its own, so its width-agnostic door would size six
//! Newton buffers and (per divide) two Knuth normalisation buffers from
//! `MAX_WORK_N`, which the build's WIDTH FEATURES select. That is the R10
//! defect — enabling `xx-wide` for a single D1232 value made every D57 cbrt
//! zero ~4× the buffer for identical work — so this kernel threads its own
//! scratch through the `_into` door and the cost tracks `N` instead. The
//! radicand `|r| · 10^(2·SCALE)` is likewise built by ONE multiply against the
//! baked const-table entry rather than one `×10` pass per decimal digit.

use crate::int::algos::icbrt::icbrt_newton::icbrt_newton_into;
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
/// length. `dst` must be wide enough for the result and **zeroed** on entry.
///
/// ONE multiply against the baked `10^exponent` table entry, not `exponent`
/// multiplies by 10. The per-digit chain re-zeroed and rewrote the whole
/// product prefix once per decimal digit, and `cbrt`'s exponent is `2·SCALE` —
/// at D616 scale 255 that was 510 passes over 40+ limbs to build a value the
/// const table already holds. `pow10_limbs` covers exp < 513 in EVERY build
/// (the always-present NARROW band) and further under the width features; the
/// ×10 chain survives only as the fallback for an exponent past the enabled
/// bands, and produces the same value.
#[inline]
fn mul_pow10_into<const N: usize>(src: &[u64], exponent: u32, dst: &mut [u64]) -> usize
where
    Limbs<N>: ComputeLimbs,
{
    let src_len = sig_len(src);
    if let Some(pow10) = crate::consts::pow10_limbs(exponent) {
        let pow10_len = sig_len(pow10);
        // A product of an `m`-limb by a `p`-limb magnitude spans at most
        // `m + p` limbs; `dst` is zeroed, so the accumulating multiply lands
        // the exact value.
        let product_len = src_len + pow10_len;
        debug_assert!(product_len <= dst.len(), "cbrt radicand scratch overflow");
        mul_slice(&src[..src_len], &pow10[..pow10_len], &mut dst[..product_len]);
        return sig_len(&dst[..product_len]);
    }
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
    // EXACT scratch, sourced here where `N` is concrete: the slice kernel has
    // no `N` of its own, so its build-max door would size six Newton buffers
    // and two Knuth normalisation buffers PER DIVIDE from `MAX_WORK_N` — a
    // width chosen by the build's features, not by this value. That is the
    // R10 defect (enabling `xx-wide` for one D1232 value slowing every D57
    // cbrt); the `_into` door takes these instead, so the cost tracks `N`.
    let mut root_buf = Limbs::<N>::quad_buffered_u64();
    let root = root_buf.as_mut();
    let mut newton_x = Limbs::<N>::quad_buffered_u64();
    let mut newton_sq = Limbs::<N>::quad_buffered_u64();
    let mut newton_q = Limbs::<N>::quad_buffered_u64();
    let mut newton_r = Limbs::<N>::quad_buffered_u64();
    let mut knuth_u = Limbs::<N>::quad_buffered_u64();
    let mut knuth_v = Limbs::<N>::quad_buffered_u64();
    let mut knuth_u128_u = Limbs::<N>::quad_buffered_u128();
    let mut knuth_u128_v = Limbs::<N>::quad_buffered_u128();
    debug_assert!(radicand_len + 2 <= knuth_u.as_ref().len(), "cbrt Knuth scratch overflow");
    icbrt_newton_into(
        &radicand[..radicand_len],
        &mut root[..radicand_len],
        newton_x.as_mut(),
        newton_sq.as_mut(),
        newton_q.as_mut(),
        newton_r.as_mut(),
        knuth_u.as_mut(),
        knuth_v.as_mut(),
        knuth_u128_u.as_mut(),
        knuth_u128_v.as_mut(),
    );
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

#[cfg(test)]
mod tests {
    use super::{cbrt_newton, cube_into, sig_len};
    use crate::int::algos::icbrt::icbrt_newton::icbrt_newton;
    use crate::int::algos::support::limbs::{cmp_cross, shl};
    use crate::int::policy::mul::dispatch_slice as mul_slice;
    use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// `mul_pow10_into` EXACTLY as it stood before the change: one `mul_slice`
    /// by `[10]` per decimal digit.
    fn mul_pow10_into_reference<const N: usize>(src: &[u64], exponent: u32, dst: &mut [u64])
        -> usize
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

    /// The kernel EXACTLY as it stood before the exact-scratch / table-radicand
    /// change: the `10^(2·SCALE)` chain above and the root taken through the
    /// build-max [`icbrt_newton`] door. The rounding half-step is untouched by
    /// the change, so it is shared with the live kernel via [`cube_into`].
    fn cbrt_newton_reference<const N: usize>(raw: Int<N>, scale: u32, mode: RoundingMode)
        -> Int<N>
    where
        Limbs<N>: ComputeLimbs,
    {
        if raw == Int::<N>::ZERO {
            return Int::<N>::ZERO;
        }
        let is_negative = raw.is_negative();
        let mut radicand_buf = Limbs::<N>::quad_buffered_u64();
        let radicand = radicand_buf.as_mut();
        let radicand_len =
            mul_pow10_into_reference::<N>(raw.unsigned_abs().as_limbs(), 2 * scale, radicand);
        let mut root_buf = Limbs::<N>::quad_buffered_u64();
        let root = root_buf.as_mut();
        icbrt_newton(&radicand[..radicand_len], &mut root[..radicand_len]);
        let root_len = sig_len(&root[..radicand_len]);

        let mut eight_radicand_buf = Limbs::<N>::quad_buffered_u64();
        let eight_radicand = eight_radicand_buf.as_mut();
        shl(&radicand[..radicand_len], 3, &mut eight_radicand[..radicand_len + 1]);
        let eight_radicand_len = sig_len(&eight_radicand[..radicand_len + 1]);
        let mut doubled_midpoint_buf = Limbs::<N>::quad_buffered_u64();
        let doubled_midpoint = doubled_midpoint_buf.as_mut();
        shl(&root[..root_len], 1, &mut doubled_midpoint[..root_len + 1]);
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
        let root_mod_10 = crate::support::rounding::limbs_mod_10(&root[..root_len]);
        let bump = match mode {
            RoundingMode::HalfToEven => halfway_gt || (tie && root_mod_10 & 1 == 1),
            RoundingMode::HalfAwayFromZero => halfway_geq,
            RoundingMode::HalfTowardZero => halfway_gt,
            RoundingMode::Trunc => false,
            RoundingMode::Floor => is_negative && residual_nonzero,
            RoundingMode::Ceiling => !is_negative && residual_nonzero,
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
        let mut root_limbs = [0u64; N];
        root_limbs.copy_from_slice(&root[..N]);
        let root_magnitude = Int::<N>::from_limbs(root_limbs);
        if is_negative {
            -root_magnitude
        } else {
            root_magnitude
        }
    }

    /// Every `(raw, scale, mode)` in the cell must agree with the reference.
    /// Returns the comparison count so the caller can prove it fired.
    fn check_cell<const N: usize>(scales: &[u32], raws: &[i128]) -> usize
    where
        Limbs<N>: ComputeLimbs,
    {
        let mut checked = 0usize;
        for &scale in scales {
            for &raw_value in raws {
                let raw = Int::<N>::from_i128(raw_value);
                for mode in ALL_MODES {
                    assert_eq!(
                        cbrt_newton::<N>(raw, scale, mode),
                        cbrt_newton_reference::<N>(raw, scale, mode),
                        "N={N} scale={scale} raw={raw_value} mode={mode:?}"
                    );
                    checked += 1;
                }
            }
        }
        checked
    }

    /// A DENSE magnitude filling all `N` storage limbs, graded at every mode
    /// and BOTH signs. Small `i128` operands leave the wide tiers rooting a
    /// handful of limbs however large `N` is; only a full-width magnitude makes
    /// the radicand span the width the exact-scratch buffers are sized for.
    fn check_dense<const N: usize>(scales: &[u32]) -> usize
    where
        Limbs<N>: ComputeLimbs,
    {
        let mut limbs = [0u64; N];
        let mut state: u64 = 0xD1B5_4A32_D192_ED03;
        for limb in limbs.iter_mut() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *limb = state;
        }
        limbs[N - 1] &= !(1u64 << 63);
        limbs[N - 1] |= 1 << 62;
        let mut checked = 0usize;
        for &negative in &[false, true] {
            let raw = Int::<N>::from_mag_limbs(&limbs, negative);
            for &scale in scales {
                for mode in ALL_MODES {
                    assert_eq!(
                        cbrt_newton::<N>(raw, scale, mode),
                        cbrt_newton_reference::<N>(raw, scale, mode),
                        "dense N={N} scale={scale} negative={negative} mode={mode:?}"
                    );
                    checked += 1;
                }
            }
        }
        checked
    }

    /// Bit-identity wall for the exact-scratch + table-radicand change. Signs
    /// matter here (the cube root is sign-preserving and `Floor`/`Ceiling`
    /// split on it), so every value is graded positive AND negative. Widths
    /// are gated by the build's width features because the REFERENCE is the
    /// build-max path and cannot hold a radicand wider than the enabled tiers.
    #[cfg(feature = "exact-scratch")]
    #[test]
    fn matches_pre_change_reference_all_modes() {
        // Perfect cubes, near-zero, primes, a full u64 limb — each signed both
        // ways.
        let raws: &[i128] = &[
            1, -1, 2, -2, 7, 8, -8, 27, -27, 1000, 12345, -12345,
            999_999_999_999_999_999, -999_999_999_999_999_999,
        ];
        let small: &[i128] = &[1, -1, 2, 7, -7, 12345];
        let mut checked = 0usize;
        // Per-tier counters, so "the wide cells ran" is proven directly rather
        // than inferred from a total a narrow build could also reach.
        #[allow(unused_mut)]
        let mut wide_checked = 0usize;
        #[allow(unused_mut)]
        let mut x_wide_checked = 0usize;
        #[allow(unused_mut)]
        let mut xx_wide_checked = 0usize;

        // Narrow: the default build's build-max reference holds an 8-limb
        // radicand, so `N <= 2` is all that is gradable without width features.
        checked += check_cell::<1>(&[0, 1, 8], &[1, -1, 2, 7, -8, 12345]);
        checked += check_cell::<2>(&[0, 1, 8], raws);
        checked += check_dense::<1>(&[0, 1]);
        checked += check_dense::<2>(&[0, 1, 8]);

        #[cfg(feature = "wide")]
        {
            wide_checked += check_cell::<3>(&[0, 1, 9, 28], raws);
            wide_checked += check_cell::<4>(&[0, 1, 9, 38], raws);
            wide_checked += check_cell::<6>(&[0, 1, 9, 47], raws);
            wide_checked += check_cell::<8>(&[0, 1, 9, 63], small);
            wide_checked += check_cell::<12>(&[0, 1, 95], small);
            wide_checked += check_cell::<16>(&[0, 1, 127], small);
            wide_checked += check_dense::<3>(&[0, 1, 9, 28]);
            wide_checked += check_dense::<4>(&[0, 9, 38]);
            wide_checked += check_dense::<6>(&[0, 9, 47]);
            wide_checked += check_dense::<8>(&[0, 9, 63]);
            wide_checked += check_dense::<12>(&[0, 95]);
            wide_checked += check_dense::<16>(&[0, 127]);
        }
        #[cfg(feature = "x-wide")]
        {
            x_wide_checked += check_cell::<24>(&[0, 1, 191], small);
            x_wide_checked += check_cell::<32>(&[0, 255], small);
            x_wide_checked += check_dense::<24>(&[0, 191]);
            x_wide_checked += check_dense::<32>(&[0, 255]);
        }
        #[cfg(feature = "xx-wide")]
        {
            xx_wide_checked += check_cell::<48>(&[0, 191], &[1, -1, 7]);
            xx_wide_checked += check_cell::<64>(&[0, 255], &[1, -1, 7]);
            xx_wide_checked += check_dense::<48>(&[0, 191]);
            xx_wide_checked += check_dense::<64>(&[0, 255]);
        }
        checked += wide_checked + x_wide_checked + xx_wide_checked;

        // Prove the mechanism FIRED — a cfg-gated test that grades nothing
        // still exits 0.
        assert!(checked >= 400, "only {checked} comparisons made");
        #[cfg(feature = "wide")]
        assert!(wide_checked >= 1000, "wide cells graded only {wide_checked}");
        #[cfg(feature = "x-wide")]
        assert!(x_wide_checked >= 200, "x-wide cells graded only {x_wide_checked}");
        #[cfg(feature = "xx-wide")]
        assert!(xx_wide_checked >= 100, "xx-wide cells graded only {xx_wide_checked}");
    }
}
