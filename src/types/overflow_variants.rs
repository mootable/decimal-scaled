// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Behaviour tests for [`D38`] overflow-aware arithmetic variants.
//!
//! The `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*`
//! method families are emitted by the shared
//! `decl_decimal_overflow_variants!(wide D38, …)` macro over the
//! `Int<2>` storage (see `src/macros/overflow.rs`). This file retains
//! only the width-specific behaviour tests.
#[cfg(test)]
#[allow(clippy::arithmetic_side_effects)]
mod tests {
    use crate::types::widths::D38s12;

    /// Returns `-ONE` as a convenience value.
    fn neg_one() -> D38s12 {
        -D38s12::ONE
    }

    /// Returns `2.0` in `D38s12` canonical form.
    fn two() -> D38s12 {
        D38s12::from_bits(crate::int::types::Int::<2>::from_i128(2_000_000_000_000))
    }

    /// Returns `3.0` in `D38s12` canonical form.
    fn three() -> D38s12 {
        D38s12::from_bits(crate::int::types::Int::<2>::from_i128(3_000_000_000_000))
    }

    // Add variants

    #[test]
    fn checked_add_normal() {
        assert_eq!(D38s12::ONE.checked_add(D38s12::ONE), Some(two()));
    }

    #[test]
    fn checked_add_overflow_returns_none() {
        // MAX + ONE overflows (MAX is i128::MAX raw; ONE is 10^SCALE raw).
        assert_eq!(D38s12::MAX.checked_add(D38s12::ONE), None);
        // Boundary: MAX + 1 LSB also overflows.
        assert_eq!(D38s12::MAX.checked_add(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1))), None);
    }

    #[test]
    fn checked_add_negative_overflow_returns_none() {
        assert_eq!(D38s12::MIN.checked_add(neg_one()), None);
        // Boundary: MIN + (-1 LSB) also overflows.
        assert_eq!(D38s12::MIN.checked_add(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1))), None);
    }

    #[test]
    fn wrapping_add_normal_matches_op() {
        assert_eq!(D38s12::ONE.wrapping_add(D38s12::ONE), two());
    }

    #[test]
    fn wrapping_add_overflow_wraps_to_min() {
        // MAX + 1 LSB wraps to MIN under two's-complement.
        assert_eq!(D38s12::MAX.wrapping_add(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1))), D38s12::MIN);
    }

    #[test]
    fn wrapping_add_negative_overflow_wraps_to_max() {
        // MIN + (-1 LSB) wraps to MAX.
        assert_eq!(D38s12::MIN.wrapping_add(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1))), D38s12::MAX);
    }

    #[test]
    fn saturating_add_normal_matches_op() {
        assert_eq!(D38s12::ONE.saturating_add(D38s12::ONE), two());
    }

    #[test]
    fn saturating_add_overflow_clamps_to_max() {
        assert_eq!(D38s12::MAX.saturating_add(D38s12::ONE), D38s12::MAX);
    }

    #[test]
    fn saturating_add_negative_overflow_clamps_to_min() {
        assert_eq!(D38s12::MIN.saturating_add(neg_one()), D38s12::MIN);
    }

    #[test]
    fn overflowing_add_normal_no_overflow() {
        assert_eq!(D38s12::ONE.overflowing_add(D38s12::ONE), (two(), false));
    }

    #[test]
    fn overflowing_add_overflow_flagged() {
        // MAX + 1 LSB wraps exactly to MIN; overflow flag is set.
        assert_eq!(
            D38s12::MAX.overflowing_add(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1))),
            (D38s12::MIN, true)
        );
    }

    #[test]
    fn overflowing_add_negative_overflow_flagged() {
        // MIN + (-1 LSB) wraps exactly to MAX.
        assert_eq!(
            D38s12::MIN.overflowing_add(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1))),
            (D38s12::MAX, true)
        );
    }

    // Sub variants

    #[test]
    fn checked_sub_normal() {
        assert_eq!(three().checked_sub(D38s12::ONE), Some(two()));
    }

    #[test]
    fn checked_sub_underflow_returns_none() {
        assert_eq!(D38s12::MIN.checked_sub(D38s12::ONE), None);
    }

    #[test]
    fn checked_sub_positive_overflow_returns_none() {
        // MAX - (-ONE) = MAX + ONE -> overflows.
        assert_eq!(D38s12::MAX.checked_sub(neg_one()), None);
    }

    #[test]
    fn wrapping_sub_normal() {
        assert_eq!(three().wrapping_sub(D38s12::ONE), two());
    }

    #[test]
    fn wrapping_sub_underflow_wraps_to_max() {
        // MIN - 1 LSB wraps exactly to MAX.
        assert_eq!(D38s12::MIN.wrapping_sub(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1))), D38s12::MAX);
    }

    #[test]
    fn saturating_sub_normal() {
        assert_eq!(three().saturating_sub(D38s12::ONE), two());
    }

    #[test]
    fn saturating_sub_underflow_clamps_to_min() {
        assert_eq!(D38s12::MIN.saturating_sub(D38s12::ONE), D38s12::MIN);
    }

    #[test]
    fn saturating_sub_overflow_clamps_to_max() {
        // MAX - (-ONE) saturates to MAX.
        assert_eq!(D38s12::MAX.saturating_sub(neg_one()), D38s12::MAX);
    }

    #[test]
    fn overflowing_sub_normal() {
        assert_eq!(three().overflowing_sub(D38s12::ONE), (two(), false));
    }

    #[test]
    fn overflowing_sub_underflow_flagged() {
        // MIN - 1 LSB wraps exactly to MAX.
        assert_eq!(
            D38s12::MIN.overflowing_sub(D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1))),
            (D38s12::MAX, true)
        );
    }

    // Neg variants

    #[test]
    fn checked_neg_normal() {
        assert_eq!(D38s12::ONE.checked_neg(), Some(neg_one()));
        assert_eq!(neg_one().checked_neg(), Some(D38s12::ONE));
        assert_eq!(D38s12::ZERO.checked_neg(), Some(D38s12::ZERO));
    }

    #[test]
    fn checked_neg_min_returns_none() {
        assert_eq!(D38s12::MIN.checked_neg(), None);
    }

    #[test]
    fn checked_neg_max_succeeds() {
        // MAX = i128::MAX, -MAX = i128::MIN + 1, fits.
        let neg_max = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-i128::MAX));
        assert_eq!(D38s12::MAX.checked_neg(), Some(neg_max));
    }

    #[test]
    fn wrapping_neg_normal() {
        assert_eq!(D38s12::ONE.wrapping_neg(), neg_one());
        assert_eq!(D38s12::ZERO.wrapping_neg(), D38s12::ZERO);
    }

    #[test]
    fn wrapping_neg_min_returns_min() {
        // -i128::MIN wraps to i128::MIN under two's-complement.
        assert_eq!(D38s12::MIN.wrapping_neg(), D38s12::MIN);
    }

    #[test]
    fn saturating_neg_normal() {
        assert_eq!(D38s12::ONE.saturating_neg(), neg_one());
        assert_eq!(D38s12::ZERO.saturating_neg(), D38s12::ZERO);
    }

    #[test]
    fn saturating_neg_min_returns_max() {
        assert_eq!(D38s12::MIN.saturating_neg(), D38s12::MAX);
    }

    #[test]
    fn overflowing_neg_normal() {
        assert_eq!(D38s12::ONE.overflowing_neg(), (neg_one(), false));
        assert_eq!(D38s12::ZERO.overflowing_neg(), (D38s12::ZERO, false));
    }

    #[test]
    fn overflowing_neg_min_flagged() {
        assert_eq!(D38s12::MIN.overflowing_neg(), (D38s12::MIN, true));
    }

    // Mul variants

    #[test]
    fn checked_mul_normal() {
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let quarter = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(250_000_000_000));
        assert_eq!(half.checked_mul(half), Some(quarter));
    }

    #[test]
    fn checked_mul_zero() {
        assert_eq!(D38s12::MAX.checked_mul(D38s12::ZERO), Some(D38s12::ZERO));
        assert_eq!(D38s12::ZERO.checked_mul(D38s12::ZERO), Some(D38s12::ZERO));
    }

    #[test]
    fn checked_mul_one_identity() {
        let v = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(7_500_000_000_000)); // 7.5
        assert_eq!(v.checked_mul(D38s12::ONE), Some(v));
        assert_eq!(D38s12::ONE.checked_mul(v), Some(v));
    }

    #[test]
    fn checked_mul_overflow_returns_none() {
        // MAX * 2.0 overflows the final i128 quotient.
        assert_eq!(D38s12::MAX.checked_mul(two()), None);
    }

    #[test]
    fn checked_mul_min_overflow_returns_none() {
        // MIN * 2.0 overflows.
        assert_eq!(D38s12::MIN.checked_mul(two()), None);
    }

    #[test]
    fn wrapping_mul_normal() {
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let quarter = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(250_000_000_000));
        assert_eq!(half.wrapping_mul(half), quarter);
    }

    #[test]
    fn wrapping_mul_overflow_does_not_panic() {
        // Verify it does not panic; the exact bit pattern is unspecified.
        let _ = D38s12::MAX.wrapping_mul(two());
        let _ = D38s12::MIN.wrapping_mul(two());
    }

    #[test]
    fn saturating_mul_normal() {
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let quarter = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(250_000_000_000));
        assert_eq!(half.saturating_mul(half), quarter);
    }

    #[test]
    fn saturating_mul_positive_overflow_clamps_to_max() {
        // MAX * 2.0 (both positive) saturates to MAX.
        assert_eq!(D38s12::MAX.saturating_mul(two()), D38s12::MAX);
    }

    #[test]
    fn saturating_mul_negative_overflow_clamps_to_min() {
        // MAX * (-2.0) (mixed sign) saturates to MIN.
        assert_eq!(D38s12::MAX.saturating_mul(-two()), D38s12::MIN);
    }

    #[test]
    fn saturating_mul_min_times_two_clamps_to_min() {
        // MIN * 2.0 (MIN negative, 2 positive) saturates to MIN.
        assert_eq!(D38s12::MIN.saturating_mul(two()), D38s12::MIN);
    }

    #[test]
    fn saturating_mul_min_times_neg_two_clamps_to_max() {
        // MIN * -2.0 (both negative) saturates to MAX.
        assert_eq!(D38s12::MIN.saturating_mul(-two()), D38s12::MAX);
    }

    #[test]
    fn overflowing_mul_normal_no_overflow() {
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let quarter = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(250_000_000_000));
        assert_eq!(half.overflowing_mul(half), (quarter, false));
    }

    #[test]
    fn overflowing_mul_overflow_flagged() {
        let (_, ovf) = D38s12::MAX.overflowing_mul(two());
        assert!(ovf);
    }

    // Div variants

    #[test]
    fn checked_div_normal() {
        // 6.0 / 2.0 = 3.0
        let six = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(6_000_000_000_000));
        assert_eq!(six.checked_div(two()), Some(three()));
    }

    #[test]
    fn checked_div_by_zero_returns_none() {
        assert_eq!(D38s12::ONE.checked_div(D38s12::ZERO), None);
    }

    #[test]
    fn checked_div_overflow_returns_none() {
        // MAX / 0.5 = 2 * MAX -> overflows the final quotient.
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        assert_eq!(D38s12::MAX.checked_div(half), None);
    }

    #[test]
    fn checked_div_negative_normal() {
        let neg_six = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-6_000_000_000_000));
        assert_eq!(neg_six.checked_div(two()), Some(-three()));
    }

    #[test]
    fn wrapping_div_normal() {
        let six = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(6_000_000_000_000));
        assert_eq!(six.wrapping_div(two()), three());
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn wrapping_div_by_zero_panics() {
        let _ = D38s12::ONE.wrapping_div(D38s12::ZERO);
    }

    #[test]
    fn wrapping_div_overflow_does_not_panic() {
        // Verify it does not panic; the exact result is unspecified.
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let _ = D38s12::MAX.wrapping_div(half);
    }

    #[test]
    fn saturating_div_normal() {
        let six = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(6_000_000_000_000));
        assert_eq!(six.saturating_div(two()), three());
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn saturating_div_by_zero_panics() {
        let _ = D38s12::ONE.saturating_div(D38s12::ZERO);
    }

    #[test]
    fn saturating_div_overflow_clamps_to_max() {
        // MAX / 0.5 (both positive) saturates to MAX.
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        assert_eq!(D38s12::MAX.saturating_div(half), D38s12::MAX);
    }

    #[test]
    fn saturating_div_negative_overflow_clamps_to_min() {
        // MAX / -0.5 (mixed sign) saturates to MIN.
        let neg_half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-500_000_000_000));
        assert_eq!(D38s12::MAX.saturating_div(neg_half), D38s12::MIN);
    }

    #[test]
    fn checked_and_wrapping_div_round_like_the_operator() {
        // 20 / 3 = 6.666… does not divide evenly; the checked/wrapping
        // variants must round to nearest using the crate-default mode,
        // identically to the `/` operator — not truncate toward zero.
        let twenty = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(20_000_000_000_000));
        let three = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(3_000_000_000_000));
        let rounded = twenty / three;
        let truncated = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(6_666_666_666_666));
        assert_ne!(rounded, truncated, "the operator must round, not truncate");
        assert_eq!(twenty.checked_div(three), Some(rounded));
        assert_eq!(twenty.wrapping_div(three), rounded);
    }

    #[test]
    fn overflowing_div_normal() {
        let six = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(6_000_000_000_000));
        assert_eq!(six.overflowing_div(two()), (three(), false));
    }

    #[test]
    fn overflowing_div_overflow_flagged() {
        let half = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(500_000_000_000));
        let (_, ovf) = D38s12::MAX.overflowing_div(half);
        assert!(ovf);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn overflowing_div_by_zero_panics() {
        let _ = D38s12::ONE.overflowing_div(D38s12::ZERO);
    }

    // Rem variants

    #[test]
    fn checked_rem_normal() {
        // 5.5 % 2.0 = 1.5
        let a = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(5_500_000_000_000));
        let expected = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(a.checked_rem(two()), Some(expected));
    }

    #[test]
    fn checked_rem_by_zero_returns_none() {
        assert_eq!(D38s12::ONE.checked_rem(D38s12::ZERO), None);
    }

    #[test]
    fn checked_rem_min_neg_one_lsb_returns_none() {
        // The raw overflow case is `i128::MIN % -1` (because i128::MIN / -1
        // overflows). The divisor's raw bits are -1, not the decimal -ONE
        // (-10^12), which does not trigger this path.
        let neg_one_lsb = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1));
        assert_eq!(D38s12::MIN.checked_rem(neg_one_lsb), None);
    }

    #[test]
    fn wrapping_rem_normal() {
        let a = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(5_500_000_000_000));
        let expected = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(a.wrapping_rem(two()), expected);
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn wrapping_rem_by_zero_panics() {
        let _ = D38s12::ONE.wrapping_rem(D38s12::ZERO);
    }

    #[test]
    fn wrapping_rem_min_neg_one_lsb_returns_zero() {
        // i128::MIN % -1 wraps to 0 (the overflow case).
        let neg_one_lsb = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1));
        assert_eq!(D38s12::MIN.wrapping_rem(neg_one_lsb), D38s12::ZERO);
    }

    #[test]
    fn overflowing_rem_normal() {
        let a = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(5_500_000_000_000));
        let expected = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(1_500_000_000_000));
        assert_eq!(a.overflowing_rem(two()), (expected, false));
    }

    #[test]
    fn overflowing_rem_min_neg_one_lsb_flagged() {
        let neg_one_lsb = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1));
        assert_eq!(
            D38s12::MIN.overflowing_rem(neg_one_lsb),
            (D38s12::ZERO, true)
        );
    }

    // Cross-scale exercise

    /// Verifies that the variant family compiles and functions correctly at SCALE = 6.
    #[test]
    fn variants_at_scale_6() {
        type D6 = crate::D<crate::int::types::Int<2>, 6>;
        let one = D6::ONE;
        let two_d6 = D6::from_bits(crate::int::types::Int::<2>::from_i128(2_000_000)); // 2.0 at SCALE=6
        let one_lsb = D6::from_bits(crate::int::types::Int::<2>::from_i128(1));

        assert_eq!(one.checked_add(one), Some(two_d6));
        // MAX + 1 LSB overflows / wraps to MIN under two's-complement.
        assert_eq!(D6::MAX.checked_add(one_lsb), None);
        assert_eq!(D6::MAX.saturating_add(one_lsb), D6::MAX);
        assert_eq!(D6::MAX.wrapping_add(one_lsb), D6::MIN);
        assert_eq!(D6::MAX.overflowing_add(one_lsb), (D6::MIN, true));

        assert_eq!(D6::MIN.checked_neg(), None);
        assert_eq!(D6::MIN.wrapping_neg(), D6::MIN);
        assert_eq!(D6::MIN.saturating_neg(), D6::MAX);
    }

    /// Verifies that `checked_*` matches the base operator when no overflow occurs.
    #[test]
    fn checked_matches_op_in_range() {
        let a = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(7_500_000_000_000)); // 7.5
        let b = two();
        assert_eq!(a.checked_add(b), Some(a + b));
        assert_eq!(a.checked_sub(b), Some(a - b));
        assert_eq!(a.checked_mul(b), Some(a * b));
        assert_eq!(a.checked_div(b), Some(a / b));
        assert_eq!(a.checked_rem(b), Some(a % b));
    }

    /// Verifies that the `overflowing_*` flag agrees with `checked_*` returning `None`.
    #[test]
    fn overflowing_flag_matches_checked_none() {
        // Add: MAX + ONE
        let (_, ovf) = D38s12::MAX.overflowing_add(D38s12::ONE);
        assert_eq!(ovf, D38s12::MAX.checked_add(D38s12::ONE).is_none());

        // Sub: MIN - ONE
        let (_, ovf) = D38s12::MIN.overflowing_sub(D38s12::ONE);
        assert_eq!(ovf, D38s12::MIN.checked_sub(D38s12::ONE).is_none());

        // Mul: MAX * 2
        let (_, ovf) = D38s12::MAX.overflowing_mul(two());
        assert_eq!(ovf, D38s12::MAX.checked_mul(two()).is_none());

        // Neg: MIN
        let (_, ovf) = D38s12::MIN.overflowing_neg();
        assert_eq!(ovf, D38s12::MIN.checked_neg().is_none());

        // Rem: MIN % (-1 LSB) -- the raw i128::MIN % -1 case.
        let neg_one_lsb = D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1));
        let (_, ovf) = D38s12::MIN.overflowing_rem(neg_one_lsb);
        assert_eq!(ovf, D38s12::MIN.checked_rem(neg_one_lsb).is_none());
    }

    /// Verifies that `saturating_add`, `saturating_sub`, and `saturating_mul`
    /// never panic and always return a value within `[MIN, MAX]`.
    #[test]
    fn saturating_never_escapes_bounds() {
        let extremes = [
            D38s12::MIN,
            D38s12::from_bits(crate::int::types::Int::<2>::from_i128(-1)),
            D38s12::ZERO,
            D38s12::ONE,
            D38s12::MAX,
        ];
        for &a in &extremes {
            for &b in &extremes {
                let s_add = a.saturating_add(b);
                let s_sub = a.saturating_sub(b);
                let s_mul = a.saturating_mul(b);
                assert!(s_add >= D38s12::MIN && s_add <= D38s12::MAX);
                assert!(s_sub >= D38s12::MIN && s_sub <= D38s12::MAX);
                assert!(s_mul >= D38s12::MIN && s_mul <= D38s12::MAX);
            }
        }
    }
}
