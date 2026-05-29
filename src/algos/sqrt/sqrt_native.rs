//! `sqrt_native` — bespoke square-root kernel that runs Newton directly in
//! a tight, concrete work integer `Int<W>` with an `f64::sqrt` seed,
//! instead of through the width-agnostic int slice `isqrt`.
//!
//! # Why a bespoke arm rather than the int `isqrt` slice
//!
//! The generic slice `isqrt`
//! ([`crate::int::algos::isqrt::isqrt_newton::isqrt_newton`]) is
//! width-agnostic over a **build-max** scratch buffer (`compute_limbs(2)`
//! limbs — up to ~288 limbs for the widest enabled tier), which it zeroes
//! several times per Newton iteration (`sq`, `q`, `r`, …). For the small
//! radicands of the mid-scale wide cells that fixed-cost buffer churn
//! dominates the real arithmetic, regressing those cells vs prod 0.4.4.
//! This kernel instead runs Newton directly in a concrete `Int<W>` (whose
//! width `W` the policy picks per `(N, SCALE)` cell to just cover
//! `mag · 10^SCALE`), so each `n / x` is one Knuth divide on tight
//! operands with no build-max zeroing.
//!
//! # Newton seed via the shared seed leaf (std / no_std agnostic)
//!
//! The over-estimate seed comes from the cross-algorithm seed leaf
//! [`crate::algo_x_support::seed::sqrt_seed`] — under `std` the `f64::sqrt`
//! of the operand's **top 64 significant bits**, scaled back to its
//! magnitude (so it is valid at any work width `W`, with no `f64::MAX`
//! ceiling); under `no_std` the classical pure-integer `2^⌈bits/2⌉`. Both
//! are safe over-estimates, so the monotone-decrease Newton loop settles
//! on the identical `⌊√n⌋` regardless of which seed body ran — the kernel
//! body stays cfg-free.
//!
//! Result is bit-for-bit identical to [`crate::algos::sqrt::sqrt_newton`]
//! under all six [`RoundingMode`] values; only the work-integer width
//! (a tight concrete `Int<W>` vs the slice's build-max scratch) and the
//! seed source change. See [`crate::algos::sqrt::sqrt_newton`] for the
//! round-step rounding algorithm.

use crate::algo_x_support::seed::sqrt_seed;
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `⌊√n⌋` over `Int<W>`, seeded via the shared seed leaf.
///
/// Returns `⌊√n⌋` for `n > 0`. The seed comes from
/// [`crate::algo_x_support::seed::sqrt_seed`] — the `f64::sqrt` of the top
/// 64 significant bits scaled back under `std`, the classical pure-integer
/// `2^⌈bits/2⌉` under `no_std` — so it is a safe over-estimate at **any**
/// work width `W` (no `f64::MAX` ceiling, unlike a direct `n.as_f64()`),
/// and the std/no_std divergence stays inside the leaf. The
/// monotone-decrease Newton loop then settles on `⌊√n⌋`.
#[inline]
fn isqrt_w_seeded<const W: usize>(n: Int<W>) -> Int<W> {
    // Build the over-estimate seed in the W-limb magnitude via the leaf.
    let bits = n.bit_length();
    let mag = n.unsigned_abs();
    let mut seed_limbs = [0u64; W];
    sqrt_seed(mag.as_limbs(), bits, &mut seed_limbs);
    let x0 = Int::<W>::from_mag_limbs(&seed_limbs, false);
    let x0 = if x0 <= Int::<W>::ZERO { Int::<W>::ONE } else { x0 };
    let two = Int::<W>::from_i128(2);
    let mut x = x0;
    loop {
        let y = (x + n / x) / two;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// Square-root kernel running Newton in a tight `Int<W>`. `N` is the
/// storage limb count backing `D<Int<N>, SCALE>`; `W` is the concrete work
/// width the policy chose to cover `mag · 10^scale` for this `(N, SCALE)`
/// cell. Input `raw` must be `> 0` (the policy saturates non-positive
/// inputs to zero before calling). The round-step mirrors
/// [`crate::algos::sqrt::sqrt_newton`] exactly; the result is bit-identical
/// to the generic path under all six [`RoundingMode`] values.
#[inline]
#[must_use]
pub(crate) fn sqrt_native<const N: usize, const W: usize>(
    raw: Int<N>,
    pow10_scale: Int<W>,
    mode: RoundingMode,
) -> Int<N> {
    if raw <= Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let zero = Int::<W>::ZERO;
    let one = Int::<W>::ONE;
    let widened: Int<W> = raw.resize_to::<Int<W>>();
    // `pow10_scale` is `10^SCALE` in `Int<W>`, supplied pre-computed by the
    // caller so it folds at compile time (`const { Int::<W>::TEN.pow(SCALE) }`
    // in the dispatch) instead of running the int pow at runtime per call.
    let n: Int<W> = widened * pow10_scale;

    let q = isqrt_w_seeded::<W>(n);

    // ── single round step (same logic as sqrt_newton). ──────────────
    // diff = n - q²  (q² ≤ n, so diff ≥ 0).
    let qsq = q * q;
    let diff = n - qsq;
    let halfway_round_up = diff > q;
    let diff_nonzero = diff != zero;
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    let q = if bump { q + one } else { q };
    q.resize_to::<Int<N>>()
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::sqrt_native;
    use crate::algos::sqrt::sqrt_newton::sqrt_newton;
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    /// `sqrt_native` is bit-identical to the proven-correct generic
    /// `sqrt_newton` across a spread of raw storages (incl. perfect
    /// squares, near-zero, large boundary radicands) and all six rounding
    /// modes, for each `(N, W, SCALE)` cell the policy routes to it. The
    /// generic kernel is oracle-gated by `ulp_strict_golden`, so matching
    /// it certifies the bespoke arm correctly-rounded.
    fn check_cell<const N: usize, const W: usize>(scale: u32, raws: &[i128])
    where
        crate::int::types::compute_limbs::Limbs<N>: crate::int::types::compute_limbs::ComputeLimbs,
    {
        for &r in raws {
            let raw = Int::<N>::from_i128(r);
            for mode in ALL_MODES {
                let got = sqrt_native::<N, W>(raw, Int::<W>::TEN.pow(scale), mode);
                let want = sqrt_newton::<N>(raw, scale, mode);
                assert_eq!(got, want, "N={N} W={W} scale={scale} raw={r} mode={mode:?}");
            }
        }
    }

    #[test]
    fn sqrt_native_matches_generic_newton_d76_s35() {
        // D76 (N=4) @ SCALE=35, work width Int<6>.
        let raws: [i128; 8] = [
            0,
            1,
            -5, // negative saturates to 0
            400_000_000_000_000_000_000_000_000_000_000_000, // 4.0 -> 2.0
            150_000_000_000_000_000_000_000_000_000_000_000,
            (1i128 << 100) | 0xBEEF,
            (1i128 << 120) | 0x1357,
            i128::MAX,
        ];
        check_cell::<4, 6>(35, &raws);
    }

    // Work width Int<11> exceeds the narrow default build's int-kernel
    // scratch (sized for the compiled decimal tiers); runs only where the
    // D153 tier is real.
    #[cfg(feature = "_wide-support")]
    #[test]
    fn sqrt_native_matches_generic_newton_d153_s75() {
        // D153 (N=8) @ SCALE=75, work width Int<11>.
        let raws: [i128; 6] = [
            0,
            1,
            (1i128 << 64),
            (1i128 << 100) | 0xABCD,
            (1i128 << 126) | 0x99,
            i128::MAX,
        ];
        check_cell::<8, 11>(75, &raws);
    }

    // Work width Int<21> needs the x-wide+ int-kernel scratch.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    #[test]
    fn sqrt_native_matches_generic_newton_d307_s150() {
        // D307 (N=16) @ SCALE=150, work width Int<21>.
        let raws: [i128; 5] = [
            0,
            1,
            (1i128 << 64) | 7,
            (1i128 << 120) | 0x1,
            i128::MAX,
        ];
        check_cell::<16, 21>(150, &raws);
    }

    #[test]
    fn sqrt_native_perfect_square_four_is_two() {
        // value 4.0 at D76<35>: raw = 4e35; sqrt = 2.0 -> raw 2e35.
        let four = Int::<4>::from_i128(4) * Int::<4>::from_i128(10).pow(35);
        let two = Int::<4>::from_i128(2) * Int::<4>::from_i128(10).pow(35);
        for mode in ALL_MODES {
            assert_eq!(sqrt_native::<4, 6>(four, Int::<6>::TEN.pow(35), mode), two, "mode {mode:?}");
        }
    }

    /// Worst-case work-width check: a near-storage-max radicand (all limbs
    /// filled, hence the widest `mag · 10^SCALE`) at each routed cell must
    /// still be bit-identical to the slice. Proves the literal `W` chosen
    /// in the policy is wide enough (a too-small `W` would overflow and
    /// diverge / be release-mode UB).
    #[cfg(feature = "_wide-support")]
    fn near_max<const N: usize>() -> Int<N> {
        let mut mag = [0u64; N];
        for m in mag.iter_mut() {
            *m = u64::MAX;
        }
        // Clear the top bit so the value stays a positive magnitude.
        mag[N - 1] = u64::MAX >> 1;
        Int::<N>::from_mag_limbs(&mag, false)
    }

    // Spans D76..D307; the D307 cell's work width (Int<24>) needs x-wide+.
    #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
    #[test]
    fn sqrt_native_near_max_magnitude_all_cells() {
        for mode in ALL_MODES {
            assert_eq!(sqrt_native::<4, 6>(near_max::<4>(), Int::<6>::TEN.pow(35), mode), sqrt_newton::<4>(near_max::<4>(), 35, mode), "D76 mode {mode:?}");
            assert_eq!(sqrt_native::<6, 9>(near_max::<6>(), Int::<9>::TEN.pow(57), mode), sqrt_newton::<6>(near_max::<6>(), 57, mode), "D115 mode {mode:?}");
            assert_eq!(sqrt_native::<8, 12>(near_max::<8>(), Int::<12>::TEN.pow(75), mode), sqrt_newton::<8>(near_max::<8>(), 75, mode), "D153s75 mode {mode:?}");
            assert_eq!(sqrt_native::<8, 12>(near_max::<8>(), Int::<12>::TEN.pow(76), mode), sqrt_newton::<8>(near_max::<8>(), 76, mode), "D153s76 mode {mode:?}");
            assert_eq!(sqrt_native::<12, 19>(near_max::<12>(), Int::<19>::TEN.pow(115), mode), sqrt_newton::<12>(near_max::<12>(), 115, mode), "D230 mode {mode:?}");
            assert_eq!(sqrt_native::<16, 24>(near_max::<16>(), Int::<24>::TEN.pow(150), mode), sqrt_newton::<16>(near_max::<16>(), 150, mode), "D307 mode {mode:?}");
        }
    }

    /// The `W = 2N` work widths the sqrt policy now routes the mid-wide
    /// tiers (D57/D76/D115/D153, routed by `N` at every scale) to, checked
    /// at a near-storage-max magnitude across the tier's scale range
    /// (including the tier's max usable scale, where `mag · 10^SCALE` is
    /// widest and a too-small `W` would overflow → release-mode UB). Each
    /// must stay bit-identical to the oracle-gated generic `sqrt_newton`.
    // Up to D153 W=16 — needs the wide-tier int-kernel scratch.
    #[cfg(feature = "_wide-support")]
    #[test]
    fn sqrt_native_routed_2n_widths_near_max() {
        for mode in ALL_MODES {
            for &s in &[0u32, 20, 28, 57] {
                assert_eq!(sqrt_native::<3, 6>(near_max::<3>(), Int::<6>::TEN.pow(s), mode), sqrt_newton::<3>(near_max::<3>(), s, mode), "D57 W=6 s={s} mode {mode:?}");
            }
            for &s in &[0u32, 20, 35, 76] {
                assert_eq!(sqrt_native::<4, 8>(near_max::<4>(), Int::<8>::TEN.pow(s), mode), sqrt_newton::<4>(near_max::<4>(), s, mode), "D76 W=8 s={s} mode {mode:?}");
            }
            for &s in &[0u32, 25, 57, 115] {
                assert_eq!(sqrt_native::<6, 12>(near_max::<6>(), Int::<12>::TEN.pow(s), mode), sqrt_newton::<6>(near_max::<6>(), s, mode), "D115 W=12 s={s} mode {mode:?}");
            }
            for &s in &[0u32, 25, 75, 153] {
                assert_eq!(sqrt_native::<8, 16>(near_max::<8>(), Int::<16>::TEN.pow(s), mode), sqrt_newton::<8>(near_max::<8>(), s, mode), "D153 W=16 s={s} mode {mode:?}");
            }
        }
    }

}
