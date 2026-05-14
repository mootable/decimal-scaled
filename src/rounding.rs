//! Rounding-mode selector for scale-narrowing operations.
//!
//! Used by [`D128::rescale_with`] to control how fractional digits are
//! discarded when the target scale is less than the source scale. The
//! six modes cover IEEE-754's five rounding rules (`HalfToEven`,
//! `HalfTowardZero`, `Trunc`, `Floor`, `Ceiling`) plus the commercial
//! `HalfAwayFromZero` rule expected by users coming from
//! `bigdecimal` / `rust_decimal`.
//!
//! [`D128::rescale`] always uses [`RoundingMode::HalfToEven`] — the
//! IEEE-754 default and the rule with no systematic bias. Choose a
//! non-default mode only when your accounting rules require it.

/// Selector for the rounding rule applied when a scale-narrowing
/// operation discards fractional digits.
///
/// See the module-level documentation for when each rule applies.
///
/// # Precision
///
/// N/A: this is a tag; no arithmetic is performed by constructing
/// or comparing variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    /// Round to nearest; on ties, round to the even neighbour.
    /// IEEE-754 `roundTiesToEven`; also called banker's rounding.
    /// Unbiased — repeated rounding does not drift sums. Crate default.
    ///
    /// Examples (truncate to integer): `0.5 -> 0`, `1.5 -> 2`,
    /// `2.5 -> 2`, `-0.5 -> 0`, `-1.5 -> -2`.
    HalfToEven,
    /// Round to nearest; on ties, round away from zero. Commercial
    /// rounding. Mildly biased in magnitude.
    ///
    /// Examples: `0.5 -> 1`, `1.5 -> 2`, `-0.5 -> -1`, `-1.5 -> -2`.
    HalfAwayFromZero,
    /// Round to nearest; on ties, round toward zero. Mildly biased
    /// toward zero. Rare in practice; included for completeness.
    ///
    /// Examples: `0.5 -> 0`, `1.5 -> 1`, `-0.5 -> 0`, `-1.5 -> -1`.
    HalfTowardZero,
    /// Truncate toward zero. Discards the fractional part. Cheapest
    /// in integer arithmetic; matches Rust's `as` cast for integer
    /// narrowing.
    ///
    /// Examples: `0.7 -> 0`, `-0.7 -> 0`, `1.9 -> 1`, `-1.9 -> -1`.
    Trunc,
    /// Round toward negative infinity (floor).
    ///
    /// Examples: `0.7 -> 0`, `-0.7 -> -1`, `1.9 -> 1`, `-1.9 -> -2`.
    Floor,
    /// Round toward positive infinity (ceiling).
    ///
    /// Examples: `0.7 -> 1`, `-0.7 -> 0`, `1.9 -> 2`, `-1.9 -> -1`.
    Ceiling,
}

/// Applies `mode` to integer division `raw / divisor`, returning the
/// rounded quotient.
///
/// `divisor` must be positive and at most `10^38` (i.e. `< i128::MAX`),
/// so `divisor / 2` and `raw % divisor` are safe operations.
///
/// Used internally by [`D128::rescale_with`] and (via inline copies) by
/// `consts.rs::rescale_from_ref`.
#[inline]
pub(crate) fn apply_rounding(raw: i128, divisor: i128, mode: RoundingMode) -> i128 {
    let quotient = raw / divisor;
    let remainder = raw % divisor;

    if remainder == 0 {
        return quotient;
    }

    let abs_rem = remainder.unsigned_abs();
    let half = (divisor / 2) as u128;

    match mode {
        RoundingMode::HalfToEven => {
            if abs_rem < half {
                quotient
            } else if abs_rem > half {
                if raw >= 0 { quotient + 1 } else { quotient - 1 }
            } else if quotient % 2 == 0 {
                quotient
            } else if raw >= 0 {
                quotient + 1
            } else {
                quotient - 1
            }
        }
        RoundingMode::HalfAwayFromZero => {
            if abs_rem < half {
                quotient
            } else if raw >= 0 {
                quotient + 1
            } else {
                quotient - 1
            }
        }
        RoundingMode::HalfTowardZero => {
            if abs_rem > half {
                if raw >= 0 { quotient + 1 } else { quotient - 1 }
            } else {
                quotient
            }
        }
        RoundingMode::Trunc => quotient,
        RoundingMode::Floor => {
            if raw >= 0 { quotient } else { quotient - 1 }
        }
        RoundingMode::Ceiling => {
            if raw >= 0 { quotient + 1 } else { quotient }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modes() -> [RoundingMode; 6] {
        [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ]
    }

    /// Zero remainder is exact for every mode.
    #[test]
    fn zero_remainder_is_quotient_for_all_modes() {
        for m in modes() {
            assert_eq!(apply_rounding(20, 10, m), 2, "{m:?}");
            assert_eq!(apply_rounding(-20, 10, m), -2, "{m:?}");
            assert_eq!(apply_rounding(0, 10, m), 0, "{m:?}");
        }
    }

    /// Half-to-even: ties go to even neighbour.
    #[test]
    fn half_to_even_ties() {
        let m = RoundingMode::HalfToEven;
        assert_eq!(apply_rounding(5, 10, m), 0);     // 0.5  -> 0 (even)
        assert_eq!(apply_rounding(15, 10, m), 2);    // 1.5  -> 2
        assert_eq!(apply_rounding(25, 10, m), 2);    // 2.5  -> 2 (even)
        assert_eq!(apply_rounding(35, 10, m), 4);    // 3.5  -> 4
        assert_eq!(apply_rounding(-5, 10, m), 0);    // -0.5 -> 0
        assert_eq!(apply_rounding(-15, 10, m), -2);  // -1.5 -> -2
        assert_eq!(apply_rounding(-25, 10, m), -2);  // -2.5 -> -2
        assert_eq!(apply_rounding(-35, 10, m), -4);  // -3.5 -> -4
    }

    /// Half-away-from-zero: ties go away from zero.
    #[test]
    fn half_away_from_zero_ties() {
        let m = RoundingMode::HalfAwayFromZero;
        assert_eq!(apply_rounding(5, 10, m), 1);
        assert_eq!(apply_rounding(15, 10, m), 2);
        assert_eq!(apply_rounding(25, 10, m), 3);
        assert_eq!(apply_rounding(-5, 10, m), -1);
        assert_eq!(apply_rounding(-15, 10, m), -2);
        assert_eq!(apply_rounding(-25, 10, m), -3);
    }

    /// Half-toward-zero: ties go toward zero.
    #[test]
    fn half_toward_zero_ties() {
        let m = RoundingMode::HalfTowardZero;
        assert_eq!(apply_rounding(5, 10, m), 0);
        assert_eq!(apply_rounding(15, 10, m), 1);
        assert_eq!(apply_rounding(25, 10, m), 2);
        assert_eq!(apply_rounding(-5, 10, m), 0);
        assert_eq!(apply_rounding(-15, 10, m), -1);
        assert_eq!(apply_rounding(-25, 10, m), -2);
    }

    /// Trunc: always toward zero, regardless of magnitude.
    #[test]
    fn trunc_always_toward_zero() {
        let m = RoundingMode::Trunc;
        assert_eq!(apply_rounding(7, 10, m), 0);
        assert_eq!(apply_rounding(9, 10, m), 0);
        assert_eq!(apply_rounding(19, 10, m), 1);
        assert_eq!(apply_rounding(-7, 10, m), 0);
        assert_eq!(apply_rounding(-19, 10, m), -1);
    }

    /// Floor: always toward negative infinity.
    #[test]
    fn floor_toward_negative_infinity() {
        let m = RoundingMode::Floor;
        assert_eq!(apply_rounding(1, 10, m), 0);
        assert_eq!(apply_rounding(7, 10, m), 0);
        assert_eq!(apply_rounding(9, 10, m), 0);
        assert_eq!(apply_rounding(-1, 10, m), -1);
        assert_eq!(apply_rounding(-7, 10, m), -1);
        assert_eq!(apply_rounding(-19, 10, m), -2);
    }

    /// Ceiling: always toward positive infinity.
    #[test]
    fn ceiling_toward_positive_infinity() {
        let m = RoundingMode::Ceiling;
        assert_eq!(apply_rounding(1, 10, m), 1);
        assert_eq!(apply_rounding(7, 10, m), 1);
        assert_eq!(apply_rounding(19, 10, m), 2);
        assert_eq!(apply_rounding(-1, 10, m), 0);
        assert_eq!(apply_rounding(-7, 10, m), 0);
        assert_eq!(apply_rounding(-19, 10, m), -1);
    }

    /// Non-half values go to the nearest neighbour for every "half"
    /// mode and ignore the half-tie rule.
    #[test]
    fn non_half_goes_to_nearest() {
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
        ] {
            assert_eq!(apply_rounding(4, 10, m), 0, "{m:?} 0.4");
            assert_eq!(apply_rounding(6, 10, m), 1, "{m:?} 0.6");
            assert_eq!(apply_rounding(-4, 10, m), 0, "{m:?} -0.4");
            assert_eq!(apply_rounding(-6, 10, m), -1, "{m:?} -0.6");
        }
    }
}

