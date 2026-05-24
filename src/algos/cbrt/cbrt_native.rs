//! `cbrt_native` -- bespoke cube-root kernel for the `(D57, SCALE == 20)`
//! cell, with an `f64::cbrt` Newton seed (mirrors prod 0.4.4's D57 cbrt).
//!
//! The generic D57 cbrt kernel works in `Int<12>` because `MAX_SCALE = 57`
//! forces `mag · 10^(2·SCALE)` to span up to `~10^171`, which overflows
//! narrower work integers. At `SCALE = 20` the radicand is bounded by
//! `mag · 10^40 ≤ 10^57 · 10^40 = 10^97` which fits `Int<6>` (~`10^115`) --
//! half the limb count of the generic `Int<12>` path.
//!
//! # Why a bespoke arm rather than the int `icbrt` policy
//!
//! The generic slice `icbrt` ([`crate::int::algos::icbrt::icbrt_newton`])
//! is width-agnostic over a build-max scratch buffer (`work_scratch(4)`
//! limbs), which it zeroes several times per Newton iteration. For the
//! 6-limb D57<20> radicand that fixed-cost buffer churn dominates the
//! arithmetic, so routing `Int<6>::icbrt()` here regressed the cell ~3×
//! vs 0.4.4. This kernel instead runs Newton directly in tight `Int<6>`
//! values with an `f64::cbrt` seed (std) -- two iterations to convergence
//! -- exactly as 0.4.4's D57 cbrt did.
//!
//! # `f64`-bridge Newton seed (std) / generic fallback (no_std)
//!
//! Under `std`, `n.as_f64().cbrt()` lands a seed within ~2⁻⁵² relative
//! error of the true `∛n`; `as_f64` preserves 53 mantissa bits and
//! `f64::cbrt` is correctly rounded. One unconditional Newton pre-step
//! lifts any positive seed to `≥ ⌈∛n⌉` (AM-GM on `(x, x, n/x²)` gives
//! `(2x + n/x²)/3 ≥ ∛n`); the monotone-decrease loop then settles on
//! `⌊∛n⌋`. Under `no_std` (no floats: the integer-only policy) the cell
//! delegates to the generic slice kernel
//! [`crate::algos::cbrt::cbrt_newton`], whose seed leaf is pure-integer.
//!
//! Result is bit-for-bit identical to [`crate::algos::cbrt::cbrt_newton`]
//! under all six [`RoundingMode`] values; only the work-integer width
//! (`Int<6>` vs the generic `Int<12>`) and the seed source change. See
//! [`crate::algos::cbrt::cbrt_newton`] for the Newton + half-step rounding
//! algorithm.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

const SCALE: u32 = 20;

/// `⌊∛n⌋` over `Int<6>` seeded via the `f64::cbrt` bridge.
///
/// Returns `⌊∛n⌋` for `n > 0`. `n.as_f64()` + `f64::cbrt` lands a seed
/// within ~2⁻⁵² relative error of the true `∛n`. One unconditional Newton
/// step from any positive seed lifts to `≥ ⌈∛n⌉` (AM-GM on
/// `(x, x, n/x²)`); the monotone-decrease loop then settles on `⌊∛n⌋`.
#[cfg(feature = "std")]
#[inline]
fn icbrt6_f64_seeded(n: Int<6>) -> Int<6> {
    let seed_f64 = n.as_f64().cbrt();
    let seed = Int::<6>::from_f64(seed_f64);
    let x0 = if seed <= Int::<6>::ZERO { Int::<6>::ONE } else { seed };
    let three = Int::<6>::from_i128(3);
    // Unconditional first Newton step. AM-GM ⇒ result ≥ ⌈∛n⌉.
    let mut x = (x0 + x0 + n / (x0 * x0)) / three;
    if x <= Int::<6>::ZERO {
        x = Int::<6>::ONE;
    }
    loop {
        let y = (x + x + n / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}

/// `D57<20>` cube-root kernel. Runs Newton in `Int<6>` with an `f64::cbrt`
/// seed under `std`; falls back to the generic slice kernel on `no_std`.
/// The half-step rounding mirrors [`crate::algos::cbrt::cbrt_newton`]
/// exactly; the result is bit-identical to the generic path under all six
/// [`RoundingMode`] values, only the iteration cost differs.
#[inline]
#[must_use]
pub(crate) fn cbrt_native(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    #[cfg(not(feature = "std"))]
    {
        return crate::algos::cbrt::cbrt_newton::cbrt_newton::<3>(raw, SCALE, mode);
    }
    #[cfg(feature = "std")]
    {
        if raw == Int::<3>::ZERO {
            return Int::<3>::ZERO;
        }
        let zero = Int::<6>::ZERO;
        let one = Int::<6>::ONE;
        let widened: Int<6> = raw.resize_to::<Int<6>>();
        let negative = widened < zero;
        let mag = if negative { -widened } else { widened };
        let n: Int<6> = mag * Int::<6>::TEN.pow(2 * SCALE);

        let q = icbrt6_f64_seeded(n);

        // ── single half-step round (same logic as cbrt_newton). ──────
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
        signed.resize_to::<Int<3>>()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::cbrt_native;
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

    /// `cbrt_native` is bit-identical to the proven-correct generic
    /// `cbrt_newton` for the `(D57, 20)` cell across a spread of raw
    /// storages (incl. perfect cubes, negatives, near-zero, large) and
    /// all six rounding modes. The generic kernel is itself oracle-gated
    /// by `ulp_strict_golden` (cbrt_d57_s28) and the wide-roots tests, so
    /// matching it certifies the bespoke arm correctly-rounded at s20.
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
        for &r in &raws {
            let raw = Int::<3>::from_i128(r);
            for mode in ALL_MODES {
                let got = cbrt_native(raw, mode);
                let want = cbrt_newton::<3>(raw, SCALE, mode);
                assert_eq!(got, want, "raw={r} mode={mode:?}");
            }
        }
    }

    #[test]
    fn cbrt_native_zero_is_zero() {
        for mode in ALL_MODES {
            assert_eq!(cbrt_native(Int::<3>::ZERO, mode), Int::<3>::ZERO, "mode {mode:?}");
        }
    }

    #[test]
    fn cbrt_native_perfect_cube_eight_is_two() {
        // value 8.0 -> raw 8e20; cbrt = 2.0 -> raw 2e20.
        let raw = Int::<3>::from_i128(800_000_000_000_000_000_000);
        let two = Int::<3>::from_i128(200_000_000_000_000_000_000);
        for mode in ALL_MODES {
            assert_eq!(cbrt_native(raw, mode), two, "mode {mode:?}");
        }
    }
}

