//! `hypot_isqrt` -- decimal hypotenuse via the int-tier hypot.
//!
//! For two `D<Int<N>, SCALE>` values with raw storages `a` and `b`, the
//! hypotenuse raw storage is `round(sqrt(a^2 + b^2))` -- both operands
//! carry the same `10^SCALE` factor, so it divides out of the root and no
//! rescale is needed (contrast [`crate::algos::sqrt`], which forms
//! `raw * 10^SCALE`). Decimal hypot is therefore *exactly* integer hypot on
//! the raw storages.
//!
//! This kernel dispatches DOWN to the integer-tier hypot
//! ([`crate::int::policy::hypot::dispatch`]) instead of re-implementing the
//! radicand-and-root arithmetic: clean layering, single source of truth.
//! The int tier forms `a^2 + b^2` in a limb scratch buffer, takes the floor
//! root via the int slice `isqrt`, and applies the round step; it returns
//! [`None`] on true overflow, which this layer maps back to [`None`] for
//! the policy's out-of-range panic. The old inversion (calling the decimal
//! `sqrt` surface on the tier's own value) is gone.
//!
//! # Generic over the storage width only
//!
//! No `W = Int<2N>` work type -- the work-width arithmetic lives in the int
//! tier's limb scratch. Returns [`None`] when the rounded root does not fit
//! `Int<N>` (true overflow); the caller maps that to the out-of-range
//! panic.
//!
//! Semantics preserved: `hypot(0, 0) = 0`; `hypot(0, x) = |x|`.

use crate::int::types::work_scratch::WorkScratch;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `round(sqrt(a^2 + b^2))` on the raw storages, dispatched DOWN to the
/// integer-tier hypot. `N` is the storage limb count backing
/// `D<Int<N>, SCALE>`. Returns [`None`] on true overflow (the rounded root
/// does not fit `Int<N>`).
#[inline]
#[must_use]
pub(crate) fn hypot_isqrt<const N: usize>(a: Int<N>, b: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Int<N>: WorkScratch,
{
    crate::int::policy::hypot::dispatch::<N>(a, b, mode)
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

    #[test]
    fn hypot_isqrt_pythagorean_3_4_5_all_modes() {
        let a = Int::<2>::from_i64(3);
        let b = Int::<2>::from_i64(4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(a, b, mode), Some(expected), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_isqrt_non_perfect_1_1() {
        let a = Int::<2>::from_i64(1);
        let b = Int::<2>::from_i64(1);
        assert_eq!(hypot_isqrt::<2>(a, b, RoundingMode::Trunc).unwrap().as_i128(), 1);
        assert_eq!(hypot_isqrt::<2>(a, b, RoundingMode::Ceiling).unwrap().as_i128(), 2);
        assert_eq!(hypot_isqrt::<2>(a, b, RoundingMode::HalfToEven).unwrap().as_i128(), 1);
    }

    #[test]
    fn hypot_isqrt_zero_zero() {
        let z = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(z, z, mode), Some(z), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_isqrt_zero_x_equals_abs_x() {
        let z = Int::<2>::from_i64(0);
        let x = Int::<2>::from_i64(42);
        for mode in ALL_MODES {
            assert_eq!(hypot_isqrt::<2>(z, x, mode), Some(x), "mode {mode:?}");
        }
    }
}
