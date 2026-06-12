// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `cbrt_native` -- bespoke cube-root kernel that runs Newton directly in a
//! tight, concrete work integer `Int<W>` with an `f64::cbrt` Newton seed,
//! instead of through the width-agnostic int slice `icbrt` (mirrors prod
//! 0.4.4's narrow-work cbrt).
//!
//! # Why a bespoke arm rather than the int `icbrt` slice
//!
//! The generic slice `icbrt` ([`crate::int::algos::icbrt::icbrt_newton`])
//! is width-agnostic over a build-max scratch buffer (`compute_limbs(4)`
//! limbs — up to ~288 limbs for the widest enabled tier), which it zeroes
//! several times per Newton iteration (`sq`, `q`, `r`, `y`, `rem3`). For
//! the small radicands of the mid-scale wide cells that fixed-cost buffer
//! churn dominates the arithmetic, so routing the slice `icbrt` there
//! regressed those cells vs 0.4.4. This kernel instead runs Newton
//! directly in a concrete `Int<W>` (whose width `W` the policy picks per
//! `(N, SCALE)` cell to just cover `mag · 10^(2·SCALE)`), so each
//! `n / x²` is one wide multiply + one Knuth divide on tight operands with
//! no build-max zeroing.
//!
//! # Newton seed via the shared seed leaf (std / no_std agnostic)
//!
//! The over-estimate seed comes from the cross-algorithm seed leaf
//! [`crate::algo_x_support::seed::cbrt_seed`] — under `std` the `f64::cbrt`
//! of the operand's **top 64 significant bits**, scaled back to its
//! magnitude (so it is valid at any work width `W`, with no `f64::MAX`
//! ceiling); under `no_std` the classical pure-integer `2^⌈bits/3⌉`. Both
//! are safe over-estimates, so the monotone-decrease Newton loop settles
//! on the identical `⌊∛n⌋` regardless of which seed body ran — the kernel
//! body stays cfg-free.
//!
//! Result is bit-for-bit identical to [`crate::algos::cbrt::cbrt_newton`]
//! under all six [`RoundingMode`] values; only the work-integer width
//! (a tight concrete `Int<W>` vs the slice's build-max scratch) and the
//! seed source change. See [`crate::algos::cbrt::cbrt_newton`] for the
//! Newton + half-step rounding algorithm.
//!
//! NOT feature-gated: referenced by the feature-independent `cbrt` policy, so
//! it compiles in every build and is dead-arm-eliminated where unreached.

use crate::algo_x_support::seed::cbrt_seed;
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `⌊∛n⌋` over `Int<W>`, seeded via the shared seed leaf.
///
/// Returns `⌊∛n⌋` for `n > 0`. The seed comes from
/// [`crate::algo_x_support::seed::cbrt_seed`] — the `f64::cbrt` of the top
/// 64 significant bits scaled back under `std`, the classical pure-integer
/// `2^⌈bits/3⌉` under `no_std` — so it is a safe over-estimate at **any**
/// work width `W` (no `f64::MAX` ceiling, unlike a direct `n.as_f64()`),
/// and the std/no_std divergence stays inside the leaf. The
/// monotone-decrease Newton loop then settles on `⌊∛n⌋`.
#[inline]
fn icbrt_w_seeded<const W: usize>(n: Int<W>) -> Int<W> {
    let bits = n.bit_length();
    let mag = n.unsigned_abs();
    let mut seed_limbs = [0u64; W];
    cbrt_seed(mag.as_limbs(), bits, &mut seed_limbs);
    let x0 = Int::<W>::from_mag_limbs(&seed_limbs, false);
    let x0 = if x0 <= Int::<W>::ZERO { Int::<W>::ONE } else { x0 };
    let three = Int::<W>::from_i128(3);
    let mut x = x0;
    loop {
        let y = (x + x + n / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// Cube-root kernel running Newton in a tight `Int<W>`. `N` is the storage
/// limb count backing `D<Int<N>, SCALE>`; `W` is the concrete work width
/// the policy chose to cover `mag · 10^(2·SCALE)` for this `(N, SCALE)`
/// cell. The half-step rounding mirrors
/// [`crate::algos::cbrt::cbrt_newton`] exactly; the result is bit-identical
/// to the generic path under all six [`RoundingMode`] values, only the
/// iteration cost differs. Under `no_std` (no floats) this delegates to the
/// generic slice kernel.
#[inline]
#[must_use]
pub(crate) fn cbrt_native<const N: usize, const W: usize>(
    raw: Int<N>,
    pow10_2scale: Int<W>,
    mode: RoundingMode,
) -> Int<N> {
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let zero = Int::<W>::ZERO;
    let one = Int::<W>::ONE;
    let widened: Int<W> = raw.resize_to::<Int<W>>();
    let negative = widened < zero;
    let mag = if negative { -widened } else { widened };
    // `pow10_2scale` is `10^(2·SCALE)` in `Int<W>`, supplied pre-computed by
    // the caller so it folds at compile time (`const { Int::<W>::TEN.pow(2 *
    // SCALE) }` in the dispatch) instead of running the int pow at runtime
    // per call.
    let n: Int<W> = mag * pow10_2scale;

    let q = icbrt_w_seeded::<W>(n);

    // ── single half-step round (same logic as cbrt_newton). ─────────
    let eight_n = n << 3u32;
    let t = q + q + one;
    let cube = t * t * t;
    let halfway_geq = eight_n >= cube;
    let halfway_gt = eight_n > cube;
    let tie = halfway_geq && !halfway_gt;
    let two_q = q + q;
    let eight_q_cubed = if q == zero { zero } else { two_q * two_q * two_q };
    let residual_nonzero = eight_n > eight_q_cubed;
    let q_is_odd = (q % (one + one)) != zero;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && q_is_odd),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => negative && residual_nonzero,
        RoundingMode::Ceiling => !negative && residual_nonzero,
    };
    let q = if bump { q + one } else { q };
    let signed = if negative { -q } else { q };
    signed.resize_to::<Int<N>>()
}

/// `(D57, SCALE == 20)` cube-root entry point — the original bespoke cell
/// (storage `Int<3>`, work `Int<6>`). Delegates to the generic
/// [`cbrt_native`] with the fixed work width and scale; kept as a named
/// seam for the `root_kernel_ab` microbench and the policy's `(3, 20)` arm.
#[inline]
#[must_use]
pub(crate) fn cbrt_native_d57s20(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    cbrt_native::<3, 6>(raw, const { crate::consts::pow10::dispatch_int::<6>(2 * 20) }, mode)
}

// These tests drive Newton directly in a wide work `Int<W>`; the
// `n / (x·x)` step's build-max Knuth-divide scratch is sized
// `4·MAX_WORK_N + 2` u64 limbs, which only covers the work width `W` once a
// wide tier raises `MAX_WORK_N` to 16 (a narrow/default build sizes it 2).
// So each test/case is gated to exactly the `dNN` tier whose storage width
// `N` it instantiates — the tier that makes that width production-real AND
// the scratch large enough. The module guard is the precise union of those
// tiers (d57..d307); below it, each case carries its own tier gate. Gating
// the TESTS only — the kernel itself stays un-gated (it must compile in
// every build).
#[cfg(all(
    test,
    feature = "std",
    any(
        feature = "d57",
        feature = "d76",
        feature = "d115",
        feature = "d153",
        feature = "d230",
        feature = "d307"
    )
))]
mod tests {
    use super::cbrt_native;
    #[cfg(feature = "d57")]
    use super::cbrt_native_d57s20;
    use crate::algos::cbrt::cbrt_newton::cbrt_newton;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const SCALE: u32 = 20;
    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    /// Generic `cbrt_native<N, W>` is bit-identical to the proven-correct
    /// generic `cbrt_newton` for each routed `(N, W, SCALE)` cell across a
    /// spread of raw storages (perfect cubes, negatives, near-zero, large)
    /// and all six rounding modes. `cbrt_newton` is oracle-gated by
    /// `ulp_strict_golden`, so matching it certifies the bespoke arm
    /// correctly-rounded.
    fn check_cell<const N: usize, const W: usize>(scale: u32, raws: &[i128])
    where
        crate::int::types::compute_limbs::Limbs<N>: crate::int::types::compute_limbs::ComputeLimbs,
    {
        for &r in raws {
            let raw = Int::<N>::from_i128(r);
            for mode in ALL_MODES {
                let got = cbrt_native::<N, W>(raw, Int::<W>::TEN.pow(2 * scale), mode);
                let want = cbrt_newton::<N>(raw, scale, mode);
                assert_eq!(got, want, "N={N} W={W} scale={scale} raw={r} mode={mode:?}");
            }
        }
    }

    // D57 storage (N=3), work `Int<6>`.
    #[cfg(feature = "d57")]
    #[test]
    fn cbrt_native_matches_generic_newton_d57_s20_all_modes() {
        // Raw storages of D57<20> values: v has raw v * 10^20.
        let raws: [i128; 11] = [
            0,
            1,                                  // tiny
            100_000_000_000_000_000_000,        // 1.0
            150_000_000_000_000_000_000,        // 1.5
            -150_000_000_000_000_000_000,       // -1.5
            800_000_000_000_000_000_000,        // 8.0 -> exact cube root 2.0
            -800_000_000_000_000_000_000,       // -8.0
            2_700_000_000_000_000_000_000,      // 27.0 -> 3.0
            12_345_678_901_234_567_890,         // 0.123...
            (1i128 << 90) | 0xBEEF,             // large, raw > u128 fast band
            (1i128 << 120) | 0x1357,            // near i128 max magnitude
        ];
        check_cell::<3, 6>(SCALE, &raws);
    }

    // D76 storage (N=4), work `Int<8>`.
    #[cfg(feature = "d76")]
    #[test]
    fn cbrt_native_matches_generic_newton_d76_s35() {
        // D76 (N=4) @ SCALE=35, work width Int<8>.
        let raws: [i128; 7] = [
            0,
            1,
            -800_000_000_000_000_000_000,
            800_000_000_000_000_000_000,
            (1i128 << 100) | 0xBEEF,
            -((1i128 << 120) | 0x1357),
            i128::MAX,
        ];
        check_cell::<4, 8>(35, &raws);
    }

    // D153 storage (N=8), work `Int<16>`.
    #[cfg(feature = "d153")]
    #[test]
    fn cbrt_native_matches_generic_newton_d153_s75() {
        // D153 (N=8) @ SCALE=75, work width Int<16>.
        let raws: [i128; 5] = [
            0,
            1,
            -((1i128 << 100) | 0xABCD),
            (1i128 << 120) | 0x99,
            i128::MAX,
        ];
        check_cell::<8, 16>(75, &raws);
    }

    // D57 storage (N=3) via `cbrt_native_d57s20`.
    #[cfg(feature = "d57")]
    #[test]
    fn cbrt_native_zero_is_zero() {
        for mode in ALL_MODES {
            assert_eq!(cbrt_native_d57s20(Int::<3>::ZERO, mode), Int::<3>::ZERO, "mode {mode:?}");
        }
    }

    // D57 storage (N=3) via `cbrt_native_d57s20`.
    #[cfg(feature = "d57")]
    #[test]
    fn cbrt_native_perfect_cube_eight_is_two() {
        // value 8.0 -> raw 8e20; cbrt = 2.0 -> raw 2e20.
        let raw = Int::<3>::from_i128(800_000_000_000_000_000_000);
        let two = Int::<3>::from_i128(200_000_000_000_000_000_000);
        for mode in ALL_MODES {
            assert_eq!(cbrt_native_d57s20(raw, mode), two, "mode {mode:?}");
        }
    }

    /// Worst-case work-width check: a near-storage-max radicand (all limbs
    /// filled, hence the widest `mag · 10^(2·SCALE)`) at each routed cell
    /// must still be bit-identical to the slice. Proves the literal `W`
    /// chosen in the policy is wide enough (a too-small `W` would overflow
    /// and diverge / be release-mode UB). Both signs.
    fn near_max<const N: usize>(neg: bool) -> Int<N> {
        let mut mag = [0u64; N];
        for m in mag.iter_mut() {
            *m = u64::MAX;
        }
        mag[N - 1] = u64::MAX >> 1;
        Int::<N>::from_mag_limbs(&mag, neg)
    }

    #[test]
    fn cbrt_native_near_max_magnitude_all_cells() {
        // One assertion per routed cell, each gated to the `dNN` tier whose
        // storage width `N` it instantiates (W up to 32 — safe once the
        // tier sets MAX_WORK_N=16). Every covered tier appears here, so any
        // single-tier build runs exactly its own cell.
        for &neg in &[false, true] {
            for mode in ALL_MODES {
                #[cfg(feature = "d57")]
                assert_eq!(cbrt_native::<3, 6>(near_max::<3>(neg), Int::<6>::TEN.pow(2 * 20), mode), cbrt_newton::<3>(near_max::<3>(neg), 20, mode), "D57 neg={neg} mode {mode:?}");
                #[cfg(feature = "d76")]
                assert_eq!(cbrt_native::<4, 8>(near_max::<4>(neg), Int::<8>::TEN.pow(2 * 35), mode), cbrt_newton::<4>(near_max::<4>(neg), 35, mode), "D76 neg={neg} mode {mode:?}");
                #[cfg(feature = "d115")]
                assert_eq!(cbrt_native::<6, 12>(near_max::<6>(neg), Int::<12>::TEN.pow(2 * 57), mode), cbrt_newton::<6>(near_max::<6>(neg), 57, mode), "D115 neg={neg} mode {mode:?}");
                #[cfg(feature = "d153")]
                assert_eq!(cbrt_native::<8, 16>(near_max::<8>(neg), Int::<16>::TEN.pow(2 * 75), mode), cbrt_newton::<8>(near_max::<8>(neg), 75, mode), "D153s75 neg={neg} mode {mode:?}");
                #[cfg(feature = "d153")]
                assert_eq!(cbrt_native::<8, 16>(near_max::<8>(neg), Int::<16>::TEN.pow(2 * 76), mode), cbrt_newton::<8>(near_max::<8>(neg), 76, mode), "D153s76 neg={neg} mode {mode:?}");
                #[cfg(feature = "d230")]
                assert_eq!(cbrt_native::<12, 25>(near_max::<12>(neg), Int::<25>::TEN.pow(2 * 115), mode), cbrt_newton::<12>(near_max::<12>(neg), 115, mode), "D230 neg={neg} mode {mode:?}");
                #[cfg(feature = "d307")]
                assert_eq!(cbrt_native::<16, 32>(near_max::<16>(neg), Int::<32>::TEN.pow(2 * 150), mode), cbrt_newton::<16>(near_max::<16>(neg), 150, mode), "D307 neg={neg} mode {mode:?}");
            }
        }
    }

}
