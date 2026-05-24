//! `hypot_isqrt` — `sqrt(a² + b²)` via the integer-layer `isqrt`.
//!
//! For two `D<Int<N>, SCALE>` values with raw storages `a` and `b`, the
//! logical hypotenuse is `sqrt((a/10^SCALE)² + (b/10^SCALE)²)`, whose raw
//! storage is
//!
//! ```text
//! round( sqrt(a² + b²) )
//! ```
//!
//! — both operands carry the same `10^SCALE` factor, so it divides out of
//! the root and no rescale is needed (contrast [`crate::algos::sqrt`],
//! which forms `raw · 10^SCALE`). The radicand `a² + b²` is formed exactly
//! in a wider work integer `W` (so the squares and their sum cannot
//! overflow), the exact integer square root is taken via the integer
//! wide-kernel surface ([`crate::int::types::traits::BigInt::isqrt`] — the
//! same int `isqrt` dispatch [`crate::algos::sqrt::sqrt_newton`] uses), and
//! a single round-to-nearest step lands the result on the type's last
//! representable place. This routes the root **down** through the integer
//! layer instead of calling the decimal `sqrt` surface on the tier's own
//! value.
//!
//! # Generic over the storage and work widths
//!
//! The kernel is parameterised over `(S, W)` via the
//! [`crate::int::types::traits::BigInt`] trait, exactly as
//! [`crate::algos::sqrt::sqrt_newton`] is. `S` is the storage type backing
//! the decimal; `W` is the next-up width used to form `a² + b²` without
//! overflow. There are **no** per-tier shims: the policy layer binds the
//! concrete `W` for each storage width when it dispatches here.
//!
//! # Semantics preserved
//!
//! - `hypot(0, 0) = 0` (the radicand is zero, `isqrt(0) = 0`);
//! - `hypot(0, x) = |x|` (`isqrt(x²) = |x|`, no rounding bump);
//! - overflow is reported (the kernel returns [`None`]) only when the true
//!   result `round(sqrt(a² + b²))` does not fit `S` — the in-`S` value is
//!   widened back to `W` and compared, so a `None` means the storage type
//!   genuinely cannot hold the hypotenuse. The caller turns `None` into the
//!   uniform out-of-range panic, matching the prior scale-trick's
//!   panic-on-overflow.

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `sqrt(a² + b²)` for the wide-integer family, taking the root through
/// the integer-layer `isqrt`.
///
/// `S` is the storage type backing `D<Int<N>, SCALE>` and `W` is the
/// next-up width used to form `a² + b²` without overflow. The rounding
/// step is identical to [`crate::algos::sqrt::sqrt_newton`]. Returns
/// [`None`] when the rounded root does not fit `S` (true overflow); the
/// caller maps that to the out-of-range panic.
#[inline]
#[must_use]
pub(crate) fn hypot_isqrt<S, W>(a: S, b: S, mode: RoundingMode) -> Option<S>
where
    S: BigInt,
    W: BigInt,
{
    let aw: W = a.resize_to::<W>();
    let bw: W = b.resize_to::<W>();
    let n: W = aw * aw + bw * bw;
    if n <= W::ZERO {
        return Some(S::ZERO);
    }
    let q: W = n.isqrt();
    let diff: W = n - q * q;
    let halfway_round_up = diff > q;
    let diff_nonzero = diff != W::ZERO;
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    let q = if bump { q + W::ONE } else { q };
    // Narrow to storage, then verify the value round-trips: a mismatch
    // means the rounded root genuinely exceeds `S`'s range (true overflow).
    let narrowed: S = q.resize_to::<S>();
    if narrowed.resize_to::<W>() == q {
        Some(narrowed)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::hypot_isqrt;
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

    /// Pythagorean triple 3-4-5: hypot_isqrt(3, 4) = 5 exactly (no rounding).
    /// The schoolbook `Schoolbook` arm delegates here, so this validates it.
    #[test]
    fn hypot_isqrt_pythagorean_3_4_5_all_modes() {
        let a = Int::<2>::from_i64(3);
        let b = Int::<2>::from_i64(4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            let got = hypot_isqrt::<Int<2>, Int<4>>(a, b, mode);
            assert_eq!(
                got,
                Some(expected),
                "hypot_isqrt(3,4) mode {mode:?}: expected 5, got {got:?}"
            );
        }
    }

    /// Pythagorean triple 5-12-13.
    #[test]
    fn hypot_isqrt_pythagorean_5_12_13_all_modes() {
        let a = Int::<2>::from_i64(5);
        let b = Int::<2>::from_i64(12);
        let expected = Int::<2>::from_i64(13);
        for mode in ALL_MODES {
            let got = hypot_isqrt::<Int<2>, Int<4>>(a, b, mode);
            assert_eq!(
                got,
                Some(expected),
                "hypot_isqrt(5,12) mode {mode:?}: expected 13, got {got:?}"
            );
        }
    }

    /// Non-perfect case: hypot(1, 1) = sqrt(2) ~ 1.414...
    /// Trunc/Floor gives 1; Ceiling gives 2. HalfToEven gives 1 (sqrt(2) < 1.5).
    #[test]
    fn hypot_isqrt_non_perfect_1_1() {
        let a = Int::<2>::from_i64(1);
        let b = Int::<2>::from_i64(1);
        // sqrt(2) ~ 1.414...; floor=1, Ceiling=2
        let got_floor = hypot_isqrt::<Int<2>, Int<4>>(a, b, RoundingMode::Trunc).unwrap();
        let got_ceil = hypot_isqrt::<Int<2>, Int<4>>(a, b, RoundingMode::Ceiling).unwrap();
        let got_half = hypot_isqrt::<Int<2>, Int<4>>(a, b, RoundingMode::HalfToEven).unwrap();
        assert_eq!(got_floor.as_i128(), 1, "Trunc of sqrt(2) must be 1");
        assert_eq!(got_ceil.as_i128(), 2, "Ceiling of sqrt(2) must be 2");
        assert_eq!(got_half.as_i128(), 1, "HalfToEven of sqrt(2) must be 1 (< 1.5)");
    }

    /// Zero operands: hypot(0, 0) = 0.
    #[test]
    fn hypot_isqrt_zero_zero() {
        let z = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            let got = hypot_isqrt::<Int<2>, Int<4>>(z, z, mode);
            assert_eq!(got, Some(z), "hypot(0,0) must be 0, mode {mode:?}");
        }
    }

    /// hypot(0, x) = |x| (isqrt(x^2) = |x|, exact).
    #[test]
    fn hypot_isqrt_zero_x_equals_abs_x() {
        let z = Int::<2>::from_i64(0);
        let x = Int::<2>::from_i64(42);
        for mode in ALL_MODES {
            let got = hypot_isqrt::<Int<2>, Int<4>>(z, x, mode);
            assert_eq!(
                got,
                Some(x),
                "hypot(0,42) must be 42, mode {mode:?}"
            );
        }
    }

    /// Negative inputs: sign drops out through squaring; hypot(-3, -4) = 5.
    #[test]
    fn hypot_isqrt_negative_inputs() {
        let a = Int::<2>::from_i64(-3);
        let b = Int::<2>::from_i64(-4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            let got = hypot_isqrt::<Int<2>, Int<4>>(a, b, mode);
            assert_eq!(
                got,
                Some(expected),
                "hypot(-3,-4) must be 5, mode {mode:?}"
            );
        }
    }
}
