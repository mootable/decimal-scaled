//! Scale-changing operations on [`D128`].
//!
//! `D128<SCALE>` carries its scale in the type. Converting between two
//! scales — for instance accumulating cents (`D128<2>`) into a picometre-
//! precision running total (`D128<12>`) — requires an explicit rescale.
//!
//! Two surfaces:
//!
//! - [`D128::rescale`] is a `const fn` shorthand that uses
//!   round-half-to-even (IEEE-754 default; banker's rounding). Suitable
//!   for the overwhelming majority of cases.
//! - [`D128::rescale_with`] takes an explicit [`RoundingMode`] for users
//!   whose accounting rules mandate a non-default rule.
//!
//! Scale-up direction (target > source) is always exact: the stored
//! integer is multiplied by `10^diff`. Scale-down direction (target <
//! source) discards fractional digits using the requested rounding mode.
//!
//! Overflow on the scale-up direction is detected via `checked_mul` and
//! panics with a clear message in both debug and release builds.

use crate::core_type::D128;
use crate::rounding::{RoundingMode, apply_rounding};

impl<const SCALE: u32> D128<SCALE> {
    /// Rescales to `TARGET_SCALE` using round-half-to-even (the crate
    /// default rounding mode).
    ///
    /// - If `TARGET_SCALE == SCALE`: returns a bit-identical value.
    /// - If `TARGET_SCALE > SCALE`: multiplies by `10^(TARGET_SCALE - SCALE)`.
    ///   Lossless; panics on overflow.
    /// - If `TARGET_SCALE < SCALE`: divides by `10^(SCALE - TARGET_SCALE)`
    ///   with round-half-to-even rounding. The fractional part is lost.
    ///
    /// # Precision
    ///
    /// Strict on scale-up; lossy with bit-exact rounding on scale-down.
    ///
    /// # Panics
    ///
    /// Panics if the scale-up multiplication overflows `i128`. This is
    /// only reachable when `TARGET_SCALE - SCALE` is large enough that
    /// `self.to_bits() * 10^diff` exceeds `i128::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::{D128s2, D128s6};
    ///
    /// // Scale-up: cents (1.50) to micro-units (1.500000) - lossless.
    /// let cents = D128s2::from_bits(150);
    /// let micros: D128s6 = cents.rescale::<6>();
    /// assert_eq!(micros.to_bits(), 1_500_000);
    ///
    /// // Scale-down: micro (1.234567) to cents (1.23) - rounds half-to-even.
    /// let micros = D128s6::from_bits(1_234_567);
    /// let cents: D128s2 = micros.rescale::<2>();
    /// assert_eq!(cents.to_bits(), 123);
    /// ```
    #[inline]
    #[must_use]
    pub const fn rescale<const TARGET_SCALE: u32>(self) -> D128<TARGET_SCALE> {
        if TARGET_SCALE == SCALE {
            return D128::<TARGET_SCALE>(self.0);
        }
        if TARGET_SCALE > SCALE {
            let shift = TARGET_SCALE - SCALE;
            let multiplier = 10i128.pow(shift);
            let result = match self.0.checked_mul(multiplier) {
                Some(v) => v,
                None => panic!("rescale: scale-up overflows D128 range"),
            };
            return D128::<TARGET_SCALE>(result);
        }
        let shift = SCALE - TARGET_SCALE;
        let divisor = 10i128.pow(shift);
        let raw = self.0;
        let quotient = raw / divisor;
        let remainder = raw % divisor;
        if remainder == 0 {
            return D128::<TARGET_SCALE>(quotient);
        }
        let abs_rem = remainder.unsigned_abs();
        let half = (divisor / 2) as u128;
        let bits = if abs_rem < half {
            quotient
        } else if abs_rem > half {
            if raw >= 0 { quotient + 1 } else { quotient - 1 }
        } else if quotient % 2 == 0 {
            quotient
        } else if raw >= 0 {
            quotient + 1
        } else {
            quotient - 1
        };
        D128::<TARGET_SCALE>(bits)
    }

    /// Rescales to `TARGET_SCALE` using the supplied rounding mode.
    ///
    /// Semantics mirror [`Self::rescale`] except that the scale-down
    /// rounding policy is chosen by the caller. See [`RoundingMode`] for
    /// the available options.
    ///
    /// # Precision
    ///
    /// Strict on scale-up; lossy with bit-exact rounding on scale-down
    /// per the selected mode.
    ///
    /// # Panics
    ///
    /// Panics if the scale-up multiplication overflows `i128`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_scaled::{D128s2, D128s6, RoundingMode};
    ///
    /// let micros = D128s6::from_bits(1_235_000);   // 1.235000, exact half at cents
    /// assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits(), 124);
    /// assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfAwayFromZero).to_bits(), 124);
    /// assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfTowardZero).to_bits(), 123);
    /// assert_eq!(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits(), 123);
    /// assert_eq!(micros.rescale_with::<2>(RoundingMode::Floor).to_bits(), 123);
    /// assert_eq!(micros.rescale_with::<2>(RoundingMode::Ceiling).to_bits(), 124);
    /// ```
    #[inline]
    #[must_use]
    pub fn rescale_with<const TARGET_SCALE: u32>(
        self,
        mode: RoundingMode,
    ) -> D128<TARGET_SCALE> {
        if TARGET_SCALE == SCALE {
            return D128::<TARGET_SCALE>::from_bits(self.0);
        }
        if TARGET_SCALE > SCALE {
            let shift = TARGET_SCALE - SCALE;
            let multiplier = 10i128.pow(shift);
            let result = self
                .0
                .checked_mul(multiplier)
                .expect("rescale_with: scale-up overflows D128 range");
            return D128::<TARGET_SCALE>::from_bits(result);
        }
        let shift = SCALE - TARGET_SCALE;
        let divisor = 10i128.pow(shift);
        let bits = apply_rounding(self.0, divisor, mode);
        D128::<TARGET_SCALE>::from_bits(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::{D128s2, D128s6, D128s12};

    // --- scale-up direction --------------------------------------------

    #[test]
    fn rescale_up_appends_zeros() {
        let cents = D128s2::from_bits(150);
        let micros = cents.rescale::<6>();
        assert_eq!(micros.to_bits(), 1_500_000);
    }

    #[test]
    fn rescale_up_negative() {
        let cents = D128s2::from_bits(-150);
        let micros = cents.rescale::<6>();
        assert_eq!(micros.to_bits(), -1_500_000);
    }

    #[test]
    fn rescale_up_zero() {
        let z = D128s2::from_bits(0);
        let m = z.rescale::<12>();
        assert_eq!(m.to_bits(), 0);
    }

    #[test]
    #[should_panic(expected = "scale-up overflows")]
    fn rescale_up_overflow_panics() {
        let big = D128s12::from_bits(i128::MAX);
        // Going from scale 12 to scale 38 multiplies by 10^26, which
        // overflows for any non-tiny source.
        let _ = big.rescale::<38>();
    }

    // --- scale-down direction (default = HalfToEven) -------------------

    #[test]
    fn rescale_down_truncates_zero_remainder() {
        let micros = D128s6::from_bits(1_500_000);
        let cents = micros.rescale::<2>();
        assert_eq!(cents.to_bits(), 150);
    }

    #[test]
    fn rescale_down_half_to_even_rounds_to_even() {
        // 1.235000 at cents: tie -> 1.24 (4 is even)
        let micros = D128s6::from_bits(1_235_000);
        assert_eq!(micros.rescale::<2>().to_bits(), 124);

        // 1.225000 at cents: tie -> 1.22 (2 is even)
        let micros = D128s6::from_bits(1_225_000);
        assert_eq!(micros.rescale::<2>().to_bits(), 122);
    }

    #[test]
    fn rescale_down_non_half_goes_nearest() {
        // 1.234999 -> 1.23 (below half)
        let micros = D128s6::from_bits(1_234_999);
        assert_eq!(micros.rescale::<2>().to_bits(), 123);
        // 1.235001 -> 1.24 (above half)
        let micros = D128s6::from_bits(1_235_001);
        assert_eq!(micros.rescale::<2>().to_bits(), 124);
    }

    #[test]
    fn rescale_down_negative_half_to_even() {
        // -1.235000 -> -1.24 (tie, 4 is even — sign symmetric)
        let micros = D128s6::from_bits(-1_235_000);
        assert_eq!(micros.rescale::<2>().to_bits(), -124);
    }

    // --- rescale_with mode coverage ------------------------------------

    #[test]
    fn rescale_with_each_mode_at_exact_half() {
        let micros = D128s6::from_bits(1_235_000); // 1.235000

        assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits(), 124);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfAwayFromZero).to_bits(), 124);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfTowardZero).to_bits(), 123);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits(), 123);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Floor).to_bits(), 123);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Ceiling).to_bits(), 124);
    }

    #[test]
    fn rescale_with_each_mode_at_exact_half_negative() {
        let micros = D128s6::from_bits(-1_235_000); // -1.235000

        assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits(), -124);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfAwayFromZero).to_bits(), -124);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::HalfTowardZero).to_bits(), -123);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits(), -123);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Floor).to_bits(), -124);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Ceiling).to_bits(), -123);
    }

    #[test]
    fn rescale_with_trunc_vs_floor_diverge_on_negative() {
        // -1.234500 (below the half-tie boundary in magnitude)
        let micros = D128s6::from_bits(-1_234_500);
        // Trunc rounds toward zero -> -1.23 (the half-tie isn't here; remainder is below half on this one)
        // Wait: divisor = 10^4 = 10000, abs_rem = 4500 which is < half (5000). So no rounding occurs.
        // Both Trunc and Floor return quotient = -123.
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits(), -123);
        assert_eq!(micros.rescale_with::<2>(RoundingMode::Floor).to_bits(), -124);
    }

    // --- equal scale identity ------------------------------------------

    #[test]
    fn rescale_same_scale_is_bit_identity() {
        let v = D128s12::from_bits(123_456_789_012);
        let same: D128s12 = v.rescale::<12>();
        assert_eq!(same.to_bits(), 123_456_789_012);
    }

    #[test]
    fn rescale_with_same_scale_is_bit_identity_for_every_mode() {
        let v = D128s12::from_bits(123_456_789_012);
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            assert_eq!(v.rescale_with::<12>(m).to_bits(), 123_456_789_012, "{m:?}");
        }
    }

    // --- const-fn usability --------------------------------------------

    #[test]
    fn rescale_works_in_const_context() {
        const SRC: D128s6 = D128s6::from_bits(1_235_000);
        const DST: D128s2 = SRC.rescale::<2>();
        assert_eq!(DST.to_bits(), 124);
    }
}
