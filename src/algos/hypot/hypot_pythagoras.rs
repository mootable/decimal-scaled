// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `hypot_pythagoras` -- decimal hypotenuse via the int-tier hypot.
//!
//! For two `D<Int<N>, SCALE>` values with raw storages `lhs` and `rhs`, the
//! hypotenuse raw storage is `round(sqrt(lhs^2 + rhs^2))` -- both operands
//! carry the same `10^SCALE` factor, so it divides out of the root and no
//! rescale is needed (contrast [`crate::algos::sqrt`], which forms
//! `raw * 10^SCALE`). Decimal hypot is therefore *exactly* integer hypot on
//! the raw storages.
//!
//! This kernel dispatches DOWN to the integer-tier hypot
//! ([`crate::int::policy::hypot::dispatch`]) instead of re-implementing the
//! radicand-and-root arithmetic: clean layering, single source of truth.
//! The int tier forms `lhs^2 + rhs^2` in a limb scratch buffer, takes the floor
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

use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// `round(sqrt(lhs^2 + rhs^2))` on the raw storages, dispatched DOWN to the
/// integer-tier hypot. `N` is the storage limb count backing
/// `D<Int<N>, SCALE>`. Returns [`None`] on true overflow (the rounded root
/// does not fit `Int<N>`).
#[inline]
#[must_use]
pub(crate) fn hypot_pythagoras<const N: usize>(lhs: Int<N>, rhs: Int<N>, mode: RoundingMode) -> Option<Int<N>>
where
    Limbs<N>: ComputeLimbs,
{
    crate::int::policy::hypot::dispatch::<N>(lhs, rhs, mode)
}

#[cfg(test)]
mod tests {
    use super::hypot_pythagoras;
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

    #[test]
    fn hypot_pythagoras_pythagorean_3_4_5_all_modes() {
        let lhs = Int::<2>::from_i64(3);
        let rhs = Int::<2>::from_i64(4);
        let expected = Int::<2>::from_i64(5);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(lhs, rhs, mode), Some(expected), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_pythagoras_non_perfect_1_1() {
        let lhs = Int::<2>::from_i64(1);
        let rhs = Int::<2>::from_i64(1);
        assert_eq!(hypot_pythagoras::<2>(lhs, rhs, RoundingMode::Trunc).unwrap().as_i128(), 1);
        assert_eq!(hypot_pythagoras::<2>(lhs, rhs, RoundingMode::Ceiling).unwrap().as_i128(), 2);
        assert_eq!(hypot_pythagoras::<2>(lhs, rhs, RoundingMode::HalfToEven).unwrap().as_i128(), 1);
    }

    #[test]
    fn hypot_pythagoras_zero_zero() {
        let zero = Int::<2>::from_i64(0);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(zero, zero, mode), Some(zero), "mode {mode:?}");
        }
    }

    #[test]
    fn hypot_pythagoras_zero_x_equals_abs_x() {
        let zero = Int::<2>::from_i64(0);
        let x = Int::<2>::from_i64(42);
        for mode in ALL_MODES {
            assert_eq!(hypot_pythagoras::<2>(zero, x, mode), Some(x), "mode {mode:?}");
        }
    }
}
