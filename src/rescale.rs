//! Scale-changing operations on [`D128`].
//!
//! `D128<SCALE>` carries its scale in the type. Converting between two
//! scales — for instance accumulating cents (`D128<2>`) into a picometre-
//! precision running total (`D128<12>`) — requires an explicit rescale.
//!
//! Two surfaces:
//!
//! - [`D128::rescale`] is a `const fn` shorthand that uses
//! round-half-to-even (IEEE-754 default; banker's rounding). Suitable
//! for the overwhelming majority of cases.
//! - [`D128::rescale_with`] takes an explicit [`RoundingMode`] for users
//! whose accounting rules mandate a non-default rule.
//!
//! Scale-up direction (target > source) is always exact: the stored
//! integer is multiplied by `10^diff`. Scale-down direction (target <
//! source) discards fractional digits using the requested rounding mode.
//!
//! Overflow on the scale-up direction is detected via `checked_mul` and
//! panics with a clear message in both debug and release builds.

use crate::core_type::{D128, D32, D64};

// The rescale / rescale_with methods are emitted by the
// `decl_decimal_rescale!` macro (see `src/decimal_rescale_macro.rs`).
crate::macros::rescale::decl_decimal_rescale!(D128, i128);
crate::macros::rescale::decl_decimal_rescale!(D64, i64);
crate::macros::rescale::decl_decimal_rescale!(D32, i32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::{D128s2, D128s6, D128s12};
    use crate::rounding::RoundingMode;

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
    #[should_panic(expected = "scale-up overflow")]
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
        use crate::rounding::RoundingMode;
        // Pin the mode so this test verifies HalfToEven specifically,
        // regardless of which `rounding-*` feature happens to be set.
        // 1.235000 at cents: tie -> 1.24 (4 is even)
        let micros = D128s6::from_bits(1_235_000);
        assert_eq!(
            micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits(),
            124
        );

        // 1.225000 at cents: tie -> 1.22 (2 is even)
        let micros = D128s6::from_bits(1_225_000);
        assert_eq!(
            micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits(),
            122
        );
    }

    #[test]
    fn rescale_down_non_half_goes_nearest() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        // 1.234999 -> 1.23 (below half)
        let micros = D128s6::from_bits(1_234_999);
        assert_eq!(micros.rescale::<2>().to_bits(), 123);
        // 1.235001 -> 1.24 (above half)
        let micros = D128s6::from_bits(1_235_001);
        assert_eq!(micros.rescale::<2>().to_bits(), 124);
    }

    #[test]
    fn rescale_down_negative_half_to_even() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
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
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        const SRC: D128s6 = D128s6::from_bits(1_235_000);
        const DST: D128s2 = SRC.rescale::<2>();
        assert_eq!(DST.to_bits(), 124);
    }
}
