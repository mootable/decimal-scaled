//! Unit tests for the decimal schoolbook baseline kernels.
//!
//! Tests call the algorithm fns directly (bypassing the policy dispatcher)
//! to exercise the naive reference paths. Every test asserts.

#[cfg(test)]
mod tests {
    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    fn i1(v: i64) -> Int<1> {
        Int::<1>::from_i64(v)
    }

    fn i2(v: i128) -> Int<2> {
        Int::<2>::from_i128(v)
    }

    // -- add_int_layer (Schoolbook = IntLayer for add) --

    #[test]
    fn add_schoolbook_identity_int1() {
        use crate::algos::add::add_int_layer::add_int_layer;
        let a = i1(123456);
        assert_eq!(add_int_layer::<1>(a, i1(0)), a);
    }

    #[test]
    fn add_schoolbook_sum_int1() {
        use crate::algos::add::add_int_layer::add_int_layer;
        assert_eq!(add_int_layer::<1>(i1(150), i1(275)), i1(425));
    }

    #[test]
    fn add_schoolbook_sum_int2() {
        use crate::algos::add::add_int_layer::add_int_layer;
        assert_eq!(add_int_layer::<2>(i2(1_000_000_000_000), i2(2_000_000_000_000)), i2(3_000_000_000_000));
    }

    // -- sub_int_layer (Schoolbook = IntLayer for sub) --

    #[test]
    fn sub_schoolbook_identity_int1() {
        use crate::algos::sub::sub_int_layer::sub_int_layer;
        let a = i1(99900);
        assert_eq!(sub_int_layer::<1>(a, i1(0)), a);
    }

    #[test]
    fn sub_schoolbook_basic_int1() {
        use crate::algos::sub::sub_int_layer::sub_int_layer;
        assert_eq!(sub_int_layer::<1>(i1(500), i1(325)), i1(175));
    }

    #[test]
    fn sub_schoolbook_negative_int1() {
        use crate::algos::sub::sub_int_layer::sub_int_layer;
        assert_eq!(sub_int_layer::<1>(i1(100), i1(300)), i1(-200));
    }

    // -- neg_int_layer (Schoolbook = IntLayer for neg) --

    #[test]
    fn neg_schoolbook_zero_int1() {
        use crate::algos::neg::neg_int_layer::neg_int_layer;
        assert_eq!(neg_int_layer::<1>(i1(0)), i1(0));
    }

    #[test]
    fn neg_schoolbook_positive_int1() {
        use crate::algos::neg::neg_int_layer::neg_int_layer;
        assert_eq!(neg_int_layer::<1>(i1(350)), i1(-350));
    }

    #[test]
    fn neg_schoolbook_double_identity_int2() {
        use crate::algos::neg::neg_int_layer::neg_int_layer;
        let v = i2(123_456_789_012);
        assert_eq!(neg_int_layer::<2>(neg_int_layer::<2>(v)), v);
    }

    // -- rem_int_layer (Schoolbook = IntLayer for rem) --

    #[test]
    fn rem_schoolbook_basic_int1() {
        use crate::algos::rem::rem_int_layer::rem_int_layer;
        assert_eq!(rem_int_layer::<1>(i1(700), i1(300)), i1(100));
    }

    #[test]
    fn rem_schoolbook_exact_int1() {
        use crate::algos::rem::rem_int_layer::rem_int_layer;
        assert_eq!(rem_int_layer::<1>(i1(600), i1(200)), i1(0));
    }

    #[test]
    fn rem_schoolbook_negative_dividend_int1() {
        use crate::algos::rem::rem_int_layer::rem_int_layer;
        assert_eq!(rem_int_layer::<1>(i1(-700), i1(300)), i1(-100));
    }

    #[test]
    fn rem_schoolbook_int2() {
        use crate::algos::rem::rem_int_layer::rem_int_layer;
        let a = i2(1_000_000_000_000_000_007);
        let b = i2(1_000_000_000_000_000_000);
        assert_eq!(rem_int_layer::<2>(a, b), i2(7));
    }

    // -- mul_schoolbook --

    #[test]
    fn mul_schoolbook_basic_scale2() {
        use crate::algos::mul::mul_schoolbook::mul_schoolbook;
        let r = mul_schoolbook::<1, 2>(i1(150), i1(200), RoundingMode::HalfToEven);
        assert_eq!(r, i1(300));
    }

    #[test]
    fn mul_schoolbook_neg_pos() {
        use crate::algos::mul::mul_schoolbook::mul_schoolbook;
        let r = mul_schoolbook::<1, 2>(i1(-200), i1(300), RoundingMode::HalfToEven);
        assert_eq!(r, i1(-600));
    }

    #[test]
    fn mul_schoolbook_rounding_half() {
        use crate::algos::mul::mul_schoolbook::mul_schoolbook;
        let even = mul_schoolbook::<1, 2>(i1(150), i1(133), RoundingMode::HalfToEven);
        let away = mul_schoolbook::<1, 2>(i1(150), i1(133), RoundingMode::HalfAwayFromZero);
        assert_eq!(even, i1(200));
        assert_eq!(away, i1(200));
    }

    #[test]
    fn mul_schoolbook_rounding_floor_ceil() {
        use crate::algos::mul::mul_schoolbook::mul_schoolbook;
        let floor = mul_schoolbook::<1, 2>(i1(150), i1(133), RoundingMode::Floor);
        let ceil = mul_schoolbook::<1, 2>(i1(150), i1(133), RoundingMode::Ceiling);
        assert_eq!(floor, i1(199));
        assert_eq!(ceil, i1(200));
    }

    #[test]
    fn mul_schoolbook_scale4_exact() {
        use crate::algos::mul::mul_schoolbook::mul_schoolbook;
        let r = mul_schoolbook::<1, 4>(i1(12500), i1(40000), RoundingMode::HalfToEven);
        assert_eq!(r, i1(50000));
    }

    #[test]
    fn mul_schoolbook_matches_widen_divide_int2_scale6() {
        use crate::algos::mul::mul_schoolbook::mul_schoolbook;
        use crate::algos::mul::mul_widen_divide::mul_widen_divide;
        const SCALE: u32 = 6;
        let cases: &[(i128, i128)] = &[
            (1_000_000, 2_000_000), (1_500_000, 3_000_000), (-1_000_000, 2_000_000),
            (1_234_567, 9_876_543), (999_999_999, 1),
        ];
        for &(ra, rb) in cases {
            let a = i2(ra);
            let b = i2(rb);
            for mode in [RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
                RoundingMode::Floor, RoundingMode::Ceiling, RoundingMode::Trunc, RoundingMode::HalfTowardZero] {
                let sb = mul_schoolbook::<2, SCALE>(a, b, mode);
                let wd = mul_widen_divide::<2, SCALE>(a, b, mode);
                assert_eq!(sb, wd);
            }
        }
    }

    // -- div_schoolbook --

    #[test]
    fn div_schoolbook_basic_scale2() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        let r = div_schoolbook::<1>(i1(600), i1(200), i1(100), RoundingMode::HalfToEven);
        assert_eq!(r, i1(300));
    }

    #[test]
    fn div_schoolbook_floor_truncation() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        let r = div_schoolbook::<1>(i1(100), i1(300), i1(100), RoundingMode::Floor);
        assert_eq!(r, i1(33));
    }

    #[test]
    fn div_schoolbook_ceiling_rounding() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        let r = div_schoolbook::<1>(i1(100), i1(300), i1(100), RoundingMode::Ceiling);
        assert_eq!(r, i1(34));
    }

    #[test]
    fn div_schoolbook_negative_dividend() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        let r = div_schoolbook::<1>(i1(-600), i1(200), i1(100), RoundingMode::HalfToEven);
        assert_eq!(r, i1(-300));
    }

    #[test]
    fn div_schoolbook_exact_half() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        let r = div_schoolbook::<1>(i1(100), i1(200), i1(100), RoundingMode::HalfToEven);
        assert_eq!(r, i1(50));
    }

    #[test]
    fn div_schoolbook_all_modes_one_third() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        let (a, b, mult) = (i1(100), i1(300), i1(100));
        assert_eq!(div_schoolbook::<1>(a, b, mult, RoundingMode::Floor), i1(33));
        assert_eq!(div_schoolbook::<1>(a, b, mult, RoundingMode::Ceiling), i1(34));
        assert_eq!(div_schoolbook::<1>(a, b, mult, RoundingMode::Trunc), i1(33));
        assert_eq!(div_schoolbook::<1>(a, b, mult, RoundingMode::HalfTowardZero), i1(33));
        assert_eq!(div_schoolbook::<1>(a, b, mult, RoundingMode::HalfToEven), i1(33));
        assert_eq!(div_schoolbook::<1>(a, b, mult, RoundingMode::HalfAwayFromZero), i1(33));
    }

    #[test]
    fn div_schoolbook_matches_widen_scale_int2_scale6() {
        use crate::algos::div::div_schoolbook::div_schoolbook;
        use crate::algos::div::div_widen_scale::div_widen_scale;
        const SCALE: u32 = 6;
        let mult = Int::<2>::TEN.pow(SCALE);
        let cases: &[(i128, i128)] = &[
            (2_000_000, 1_000_000), (1_000_000, 3_000_000), (7_000_000, 2_000_000),
            (-5_000_000, 2_000_000), (1_234_567, 9_876_543),
        ];
        for &(ra, rb) in cases {
            let a = i2(ra);
            let b = i2(rb);
            for mode in [RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
                RoundingMode::Floor, RoundingMode::Ceiling, RoundingMode::Trunc, RoundingMode::HalfTowardZero] {
                let sb = div_schoolbook::<2>(a, b, mult, mode);
                let wd = div_widen_scale::<2>(a, b, mult, mode);
                assert_eq!(sb, wd);
            }
        }
    }

    // -- div_native vs the widen reference (narrow N == 2, the routed band) --

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::Trunc,
        RoundingMode::HalfTowardZero,
    ];

    /// `div_native` (hardware i128) is bit-identical to `div_widen_scale`
    /// at `N == 1` (D18) across all six rounding modes, incl. negatives.
    #[test]
    fn div_native_matches_widen_scale_int1_scale6() {
        use crate::algos::div::div_native::div_native;
        use crate::algos::div::div_widen_scale::div_widen_scale;
        const SCALE: u32 = 6;
        let mult = Int::<1>::TEN.pow(SCALE);
        let cases: &[(i64, i64)] = &[
            (2_000_000, 1_000_000), (1_000_000, 3_000_000), (7_000_000, 2_000_000),
            (-5_000_000, 2_000_000), (1_234_567, 9_876_543), (-7, -13), (0, 5_000_000),
        ];
        for &(ra, rb) in cases {
            let a = i1(ra);
            let b = i1(rb);
            for mode in ALL_MODES {
                let nat = div_native::<1, SCALE>(a, b, mult, mode);
                let wd = div_widen_scale::<1>(a, b, mult, mode);
                assert_eq!(nat, wd, "div N1 ({ra},{rb}) mode {mode:?}");
            }
        }
    }

    /// `div_native` is bit-identical to `div_widen_scale` at `N == 2` (D38),
    /// incl. a scale that forces the 256-bit fallback path.
    #[test]
    fn div_native_matches_widen_scale_int2() {
        use crate::algos::div::div_native::div_native;
        use crate::algos::div::div_widen_scale::div_widen_scale;
        let cases: &[(i128, i128)] = &[
            (2_000_000, 1_000_000), (1_000_000, 3_000_000), (7_000_000, 2_000_000),
            (-5_000_000, 2_000_000), (1_234_567, 9_876_543),
            (98_765_432_109_876_543, 12_345_678_901_234_567),
        ];
        for &(ra, rb) in cases {
            let a = i2(ra);
            let b = i2(rb);
            for mode in ALL_MODES {
                let mult6 = Int::<2>::TEN.pow(6);
                assert_eq!(
                    div_native::<2, 6>(a, b, mult6, mode),
                    div_widen_scale::<2>(a, b, mult6, mode),
                    "div N2 s6 ({ra},{rb}) mode {mode:?}"
                );
                let mult22 = Int::<2>::TEN.pow(22);
                assert_eq!(
                    div_native::<2, 22>(a, b, mult22, mode),
                    div_widen_scale::<2>(a, b, mult22, mode),
                    "div N2 s22 ({ra},{rb}) mode {mode:?}"
                );
            }
        }
    }
}
