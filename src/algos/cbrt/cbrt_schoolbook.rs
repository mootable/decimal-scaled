//! `cbrt_schoolbook` — schoolbook decimal cube-root reference via
//! `W::icbrt`.
//!
//! This is the plain correctness reference for `cbrt_strict_with`: scale
//! the magnitude to `|raw| · 10^(2·SCALE)` in a wider work integer `W`,
//! take the exact integer cube root (`W::icbrt` — dispatched through the
//! integer layer), then apply a single rounding half-step to land on the
//! type's last representable place. Sign is preserved.
//!
//! There is **no** Newton iteration, no table seed, and no fast path at
//! the decimal layer. All non-trivial computation goes **down** to the
//! integer layer through `W::icbrt`.
//!
//! # Identity with `cbrt_newton`
//!
//! `cbrt_newton` already IS this schoolbook pipeline end-to-end: it
//! forms the same radicand and takes the same `W::icbrt`. Both variants
//! share the [`cbrt_newton`](crate::algos::cbrt::cbrt_newton::cbrt_newton)
//! kernel; the `Schoolbook` policy arm exists as an explicit,
//! benchmarkable seam — not a separate computation.
//!
//! # Correctness guarantee
//!
//! `W::icbrt` returns the exact floor root, so the rounding half-step
//! (identical to `cbrt_newton`) produces a correctly-rounded result
//! (within 0.5 ULP) under any of the six [`RoundingMode`]s.

use crate::algos::cbrt::cbrt_newton::cbrt_newton;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Schoolbook decimal cube-root: `round(cbrt(raw / 10^SCALE))`.
///
/// Delegates to [`cbrt_newton`] — the same magnitude-scaling +
/// `W::icbrt` + rounding-step pipeline, which IS the schoolbook
/// algorithm. `S` is the storage type and `W` is the double-bumped work
/// width supplied by the policy.
#[inline]
#[must_use]
pub(crate) fn cbrt_schoolbook<S, W>(raw: S, scale: u32, mode: RoundingMode) -> S
where
    S: BigInt,
    W: BigInt,
{
    cbrt_newton::<S, W>(raw, scale, mode)
}

#[cfg(test)]
mod tests {
    use super::cbrt_schoolbook;
    use crate::algos::cbrt::cbrt_newton::cbrt_newton;
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

    /// Perfect cube: cbrt(8 * 10^(2*2)) = cbrt(800) ... no.
    /// raw=8, scale=2: result raw = round(cbrt(8 * 10^(2*2))) = round(cbrt(80000)) = round(43.08...)
    /// Wait -- cbrt(raw / 10^scale) = cbrt(8/100) = cbrt(0.08) = 0.431..., raw result = 43.
    /// Let's use raw=1000, scale=1: logical=100, cbrt=~4.64..., not perfect.
    /// Use raw=27, scale=0: logical=27, cbrt=3, raw result=3. scale=0 is valid.
    #[test]
    fn cbrt_schoolbook_perfect_cube_d18_scale0() {
        // raw=27, scale=0: logical 27, cbrt = 3, raw result = 3
        let raw = Int::<1>::from_i64(27);
        let expected = Int::<1>::from_i64(3);
        for mode in ALL_MODES {
            let sb = cbrt_schoolbook::<Int<1>, Int<2>>(raw, 0, mode);
            let nr = cbrt_newton::<Int<1>, Int<2>>(raw, 0, mode);
            assert_eq!(sb, expected, "cbrt_schoolbook D18 scale=0 raw=27 mode {mode:?}");
            assert_eq!(nr, expected, "cbrt_newton D18 scale=0 raw=27 mode {mode:?}");
        }
    }

    /// Perfect cube: cbrt(8000000) in D38, scale=2.
    /// raw=8000000, scale=2: logical=80000, cbrt=~43.08..., not perfect.
    /// Use raw=1000, scale=1: logical=100, cbrt=~4.64..., non-perfect. Good.
    /// Use raw=27000000, scale=2: logical=2700, cbrt=~13.924..., non-perfect.
    /// Use raw=8, scale=0: logical=8, cbrt=2, raw result=2.
    #[test]
    fn cbrt_schoolbook_perfect_cube_d38_scale0() {
        // raw=8, scale=0: logical 8, cbrt = 2 exactly
        let raw = Int::<2>::from_i64(8);
        let expected = Int::<2>::from_i64(2);
        for mode in ALL_MODES {
            let sb = cbrt_schoolbook::<Int<2>, Int<4>>(raw, 0, mode);
            let nr = cbrt_newton::<Int<2>, Int<4>>(raw, 0, mode);
            assert_eq!(sb, expected, "cbrt_schoolbook D38 scale=0 raw=8 mode {mode:?}");
            assert_eq!(nr, expected, "cbrt_newton D38 scale=0 raw=8 mode {mode:?}");
        }
    }

    /// Non-perfect cube: cbrt(10) with scale=3.
    /// raw=10, scale=3: logical=0.01, cbrt=0.2154..., raw result~2154 (Trunc).
    /// Schoolbook must match Newton for all modes.
    #[test]
    fn cbrt_schoolbook_non_perfect_d38_scale3() {
        let raw = Int::<2>::from_i64(10);
        for mode in ALL_MODES {
            let sb = cbrt_schoolbook::<Int<2>, Int<4>>(raw, 3, mode);
            let nr = cbrt_newton::<Int<2>, Int<4>>(raw, 3, mode);
            assert_eq!(
                sb, nr,
                "cbrt_schoolbook vs cbrt_newton mismatch mode {mode:?}"
            );
        }
    }

    /// Negative input sign is preserved.
    #[test]
    fn cbrt_schoolbook_negative_sign_preserved() {
        // raw=-27, scale=0: cbrt(-27) = -3
        let raw = Int::<1>::from_i64(-27);
        let expected = Int::<1>::from_i64(-3);
        for mode in ALL_MODES {
            let sb = cbrt_schoolbook::<Int<1>, Int<2>>(raw, 0, mode);
            let nr = cbrt_newton::<Int<1>, Int<2>>(raw, 0, mode);
            assert_eq!(sb, expected, "cbrt_schoolbook D18 scale=0 raw=-27 mode {mode:?}");
            assert_eq!(nr, expected, "cbrt_newton D18 scale=0 raw=-27 mode {mode:?}");
        }
    }

    /// Zero input returns zero.
    #[test]
    fn cbrt_schoolbook_zero_returns_zero() {
        let raw = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            let sb = cbrt_schoolbook::<Int<2>, Int<4>>(raw, 5, mode);
            assert_eq!(sb.as_i128(), 0, "zero should return 0, mode {mode:?}");
        }
    }
}
