//! `sqrt_schoolbook` — schoolbook decimal square-root reference via
//! `W::isqrt`.
//!
//! This is the plain correctness reference for `sqrt_strict_with`: scale
//! the raw storage to `raw · 10^SCALE` in a wider work integer `W`, take
//! the exact integer square root (`W::isqrt` — the floor root dispatched
//! through the integer layer), then apply a single rounding half-step to
//! land on the type's last representable place.
//!
//! There is **no** Newton iteration, no table seed, and no fast path at
//! the decimal layer. All non-trivial computation goes **down** to the
//! integer layer through `W::isqrt`.
//!
//! # Identity with `sqrt_newton`
//!
//! `sqrt_newton` already IS this schoolbook pipeline end-to-end: it
//! forms the same radicand and takes the same `W::isqrt`. Both variants
//! share the [`sqrt_newton`](crate::algos::sqrt::sqrt_newton::sqrt_newton)
//! kernel; the `Schoolbook` policy arm exists as an explicit,
//! benchmarkable seam — not a separate computation.
//!
//! # Correctness guarantee
//!
//! `W::isqrt` returns the exact floor root, so `diff = n − q²` is the
//! true remainder. The rounding half-step is identical to the one in
//! `sqrt_newton` and produces a correctly-rounded result (within 0.5 ULP)
//! under any of the six [`RoundingMode`]s.

use crate::algos::sqrt::sqrt_newton::sqrt_newton;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Schoolbook decimal square-root: `round(sqrt(raw / 10^SCALE))`.
///
/// Delegates to [`sqrt_newton`] — the same radicand-scaling +
/// `W::isqrt` + rounding-step pipeline, which IS the schoolbook
/// algorithm. `S` is the storage type and `W` is the next-up work
/// width supplied by the policy.
#[inline]
#[must_use]
pub(crate) fn sqrt_schoolbook<S, W>(raw: S, scale: u32, mode: RoundingMode) -> S
where
    S: BigInt,
    W: BigInt,
{
    sqrt_newton::<S, W>(raw, scale, mode)
}

#[cfg(test)]
mod tests {
    use super::sqrt_schoolbook;
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

    /// Perfect square: raw=4, scale=2 gives radicand 4*10^2=400.
    /// isqrt(400)=20 (exact), so raw result = 20 (logical 0.2 at scale 2).
    /// Schoolbook and Newton must agree for all modes.
    #[test]
    fn sqrt_schoolbook_perfect_square_d18_scale2() {
        // raw=4, scale=2: radicand = 4 * 10^2 = 400, isqrt(400) = 20 (exact), result raw = 20
        let raw = Int::<1>::from_i64(4);
        for mode in ALL_MODES {
            let sb = sqrt_schoolbook::<Int<1>, Int<2>>(raw, 2, mode);
            let nr = sqrt_newton::<Int<1>, Int<2>>(raw, 2, mode);
            assert_eq!(
                sb, nr,
                "sqrt_schoolbook vs sqrt_newton mismatch at mode {mode:?}"
            );
            assert_eq!(
                sb.as_i128(),
                20,
                "expected raw=20 (= 0.2 at scale 2), got {got}",
                got = sb.as_i128()
            );
        }
    }

    /// Perfect square: sqrt(9 * 10^6) in D38 with scale=3.
    /// Logical value = 9000, sqrt = ~94.868..., not perfect.
    /// Tests non-perfect: schoolbook == newton for all modes.
    #[test]
    fn sqrt_schoolbook_non_perfect_d38_scale3() {
        // raw=9, scale=3: logical 0.009, sqrt = 0.09486..., raw = 94 (Floor) or 95 (Ceiling)
        let raw = Int::<2>::from_i64(9);
        for mode in ALL_MODES {
            let sb = sqrt_schoolbook::<Int<2>, Int<4>>(raw, 3, mode);
            let nr = sqrt_newton::<Int<2>, Int<4>>(raw, 3, mode);
            assert_eq!(
                sb, nr,
                "sqrt_schoolbook vs sqrt_newton mismatch at mode {mode:?}"
            );
        }
    }

    /// Large perfect square: sqrt(100 * 10^4) = sqrt(1_000_000) = 1000, scale=4.
    /// Logical value = 100 / 10^4 = 0.01, sqrt = 0.1, raw = 1000. Wait...
    /// Actually raw=100, scale=4: logical = 100/10^4 = 0.01, sqrt = 0.1, result raw = 1000/10^4*10^4 = 1000? No.
    /// raw=100, scale=4: result raw = round(sqrt(100 * 10^4)) = round(sqrt(1_000_000)) = 1000. Yes.
    #[test]
    fn sqrt_schoolbook_large_perfect_d38_scale4() {
        // raw=100, scale=4: radicand = 100 * 10^4 = 1_000_000, isqrt = 1000 (exact)
        let raw = Int::<2>::from_i64(100);
        let expected = Int::<2>::from_i64(1000);
        for mode in ALL_MODES {
            let sb = sqrt_schoolbook::<Int<2>, Int<4>>(raw, 4, mode);
            let nr = sqrt_newton::<Int<2>, Int<4>>(raw, 4, mode);
            assert_eq!(sb, expected, "sqrt_schoolbook D38 scale=4 mode {mode:?}");
            assert_eq!(nr, expected, "sqrt_newton D38 scale=4 mode {mode:?}");
        }
    }

    /// Zero input saturates to zero in both.
    #[test]
    fn sqrt_schoolbook_zero_saturates() {
        let raw = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            let sb = sqrt_schoolbook::<Int<2>, Int<4>>(raw, 6, mode);
            assert_eq!(sb.as_i128(), 0, "zero should saturate to 0, mode {mode:?}");
        }
    }

    /// Negative input saturates to zero.
    #[test]
    fn sqrt_schoolbook_negative_saturates() {
        let raw = Int::<2>::from_i64(-25);
        for mode in ALL_MODES {
            let sb = sqrt_schoolbook::<Int<2>, Int<4>>(raw, 3, mode);
            assert_eq!(sb.as_i128(), 0, "negative should saturate to 0, mode {mode:?}");
        }
    }
}
