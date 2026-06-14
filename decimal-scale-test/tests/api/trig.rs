//! Trig / hyperbolic / angle-conversion behaviour tests moved from
//! `src/types/trig.rs`. The f64 cross-check stays an independent
//! oracle; the two hand-pinned `sin(1)` deep-scale value pins stay
//! in src pending their golden-lead fold.

mod from_src_trig {
    use decimal_scaled::{D38s12, DecimalConstants, Int};

    /// `Int::as_i128` is crate-private; same accessor via the public
    /// `From<Int<2>> for i128` impl so the moved test bodies stay verbatim.
    trait AsI128 {
        fn as_i128(self) -> i128;
    }
    impl AsI128 for Int<2> {
        fn as_i128(self) -> i128 {
            i128::from(self)
        }
    }

    // Tolerance for single-operation results. The f64-bridge build is
    // one f64 round-trip (≤ 2 LSB); the integer-only `strict` build is
    // correctly rounded (≤ 0.5 ULP per call) and is held to the same
    // 2-LSB bound — a couple of LSB for the test's own expected-value
    // rounding.
    const TWO_LSB: i128 = 2;

    // Tolerance for results that chain multiple trig calls.
    const FOUR_LSB: i128 = 4;

    // Angle conversions amplify the f64 reference's pi quantization;
    // 32 LSB at SCALE = 12.
    const ANGLE_TOLERANCE_LSB: i128 = 32;

    fn within_lsb(actual: D38s12, expected: D38s12, lsb: i128) -> bool {
        let diff = (actual.to_bits().as_i128() - expected.to_bits().as_i128()).abs();
        diff <= lsb
    }

    // ── Forward trig ──────────────────────────────────────────────────

    /// The strict trig / hyperbolic family is correctly rounded:
    /// cross-check every method against the f64 bridge at D38<9>.
    #[cfg(any(not(feature = "fast"), feature = "strict"))]
    #[test]
    fn strict_trig_family_matches_f64() {
        macro_rules! check {
            ($name:literal, $raw:expr, $strict:expr, $f64expr:expr) => {{
                let strict: i128 = $strict;
                let v = $raw as f64 / 1e9;
                let reference = ($f64expr(v) * 1e9).round() as i128;
                assert!(
                    (strict - reference).abs() <= 2,
                    concat!($name, "({}) = {}, f64 reference {}"),
                    $raw,
                    strict,
                    reference
                );
            }};
        }
        for &raw in &[
            -7_000_000_000_i128,
            -1_000_000_000,
            -100_000_000,
            1,
            500_000_000,
            1_000_000_000,
            1_570_796_327,
            3_000_000_000,
            6_283_185_307,
            12_000_000_000,
        ] {
            let x = decimal_scaled::D::<Int<2>, 9>::from_bits(Int::<2>::try_from(raw).unwrap());
            check!("sin", raw, x.sin_strict().to_bits().as_i128(), f64::sin);
            check!("cos", raw, x.cos_strict().to_bits().as_i128(), f64::cos);
            check!("atan", raw, x.atan_strict().to_bits().as_i128(), f64::atan);
            check!("sinh", raw, x.sinh_strict().to_bits().as_i128(), f64::sinh);
            check!("cosh", raw, x.cosh_strict().to_bits().as_i128(), f64::cosh);
            check!("tanh", raw, x.tanh_strict().to_bits().as_i128(), f64::tanh);
            check!("asinh", raw, x.asinh_strict().to_bits().as_i128(), f64::asinh);
        }
        for &raw in &[
            -1_000_000_000_i128,
            -700_000_000,
            -100_000_000,
            0,
            250_000_000,
            500_000_000,
            999_999_999,
        ] {
            let x = decimal_scaled::D::<Int<2>, 9>::from_bits(Int::<2>::try_from(raw).unwrap());
            check!("asin", raw, x.asin_strict().to_bits().as_i128(), f64::asin);
            check!("acos", raw, x.acos_strict().to_bits().as_i128(), f64::acos);
        }
        for &raw in &[-900_000_000_i128, -300_000_000, 1, 300_000_000, 900_000_000] {
            let x = decimal_scaled::D::<Int<2>, 9>::from_bits(Int::<2>::try_from(raw).unwrap());
            check!("atanh", raw, x.atanh_strict().to_bits().as_i128(), f64::atanh);
        }
        for &raw in &[
            1_000_000_000_i128,
            1_500_000_000,
            3_000_000_000,
            50_000_000_000,
        ] {
            let x = decimal_scaled::D::<Int<2>, 9>::from_bits(Int::<2>::try_from(raw).unwrap());
            check!("acosh", raw, x.acosh_strict().to_bits().as_i128(), f64::acosh);
        }
        for &raw in &[
            -1_000_000_000_i128,
            1,
            500_000_000,
            1_000_000_000,
            1_400_000_000,
        ] {
            let x = decimal_scaled::D::<Int<2>, 9>::from_bits(Int::<2>::try_from(raw).unwrap());
            check!("tan", raw, x.tan_strict().to_bits().as_i128(), f64::tan);
        }
    }

    /// `sin(0) == 0`.
    #[test]
    fn sin_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sin(), D38s12::ZERO);
    }

    /// `cos(0) == 1`.
    #[test]
    fn cos_zero_is_one() {
        assert_eq!(D38s12::ZERO.cos(), D38s12::ONE);
    }

    /// `tan(0) == 0`.
    #[test]
    fn tan_zero_is_zero() {
        assert_eq!(D38s12::ZERO.tan(), D38s12::ZERO);
    }

    /// Pythagorean identity.
    #[test]
    fn sin_squared_plus_cos_squared_is_one() {
        for raw in [
            1_234_567_890_123_i128,
            -2_345_678_901_234_i128,
            500_000_000_000_i128,
            -500_000_000_000_i128,
            4_567_891_234_567_i128,
        ] {
            let x = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let s = x.sin();
            let c = x.cos();
            let sum = (s * s) + (c * c);
            assert!(
                within_lsb(sum, D38s12::ONE, FOUR_LSB),
                "sin^2 + cos^2 != 1 for raw={raw}: got bits {} (delta {})",
                sum.to_bits().as_i128(),
                (sum.to_bits().as_i128() - D38s12::ONE.to_bits().as_i128()).abs(),
            );
        }
    }

    // ── Inverse trig ──────────────────────────────────────────────────

    #[test]
    fn asin_zero_is_zero() {
        assert_eq!(D38s12::ZERO.asin(), D38s12::ZERO);
    }

    #[test]
    fn acos_one_is_zero() {
        assert_eq!(D38s12::ONE.acos(), D38s12::ZERO);
    }

    #[test]
    fn acos_zero_is_half_pi() {
        let result = D38s12::ZERO.acos();
        assert!(
            within_lsb(result, D38s12::half_pi(), FOUR_LSB),
            "acos(0) bits {}, half_pi bits {}",
            result.to_bits().as_i128(),
            D38s12::half_pi().to_bits().as_i128(),
        );
    }

    #[test]
    fn atan_zero_is_zero() {
        assert_eq!(D38s12::ZERO.atan(), D38s12::ZERO);
    }

    #[test]
    fn asin_of_sin_round_trip() {
        for raw in [
            123_456_789_012_i128,
            -123_456_789_012_i128,
            456_789_012_345_i128,
            -456_789_012_345_i128,
            1_234_567_890_123_i128,
            -1_234_567_890_123_i128,
        ] {
            let x = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let recovered = x.sin().asin();
            assert!(
                within_lsb(recovered, x, FOUR_LSB),
                "asin(sin(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits().as_i128(),
                (recovered.to_bits().as_i128() - x.to_bits().as_i128()).abs(),
            );
        }
    }

    // ── atan2 ─────────────────────────────────────────────────────────

    #[test]
    fn atan2_first_quadrant_diagonal() {
        let one = D38s12::ONE;
        let result = one.atan2(one);
        assert!(
            within_lsb(result, D38s12::quarter_pi(), TWO_LSB),
            "atan2(1, 1) bits {}, quarter_pi bits {}",
            result.to_bits().as_i128(),
            D38s12::quarter_pi().to_bits().as_i128(),
        );
    }

    #[test]
    fn atan2_third_quadrant_diagonal() {
        let neg_one = -D38s12::ONE;
        let result = neg_one.atan2(neg_one);
        let three = D38s12::try_from(3).unwrap();
        let expected = -(D38s12::quarter_pi() * three);
        assert!(
            within_lsb(result, expected, TWO_LSB),
            "atan2(-1, -1) bits {}, expected -3pi/4 bits {}",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
        );
    }

    #[test]
    fn atan2_second_quadrant_diagonal() {
        let one = D38s12::ONE;
        let neg_one = -D38s12::ONE;
        let result = one.atan2(neg_one);
        let three = D38s12::try_from(3).unwrap();
        let expected = D38s12::quarter_pi() * three;
        assert!(
            within_lsb(result, expected, TWO_LSB),
            "atan2(1, -1) bits {}, expected 3pi/4 bits {}",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
        );
    }

    #[test]
    fn atan2_fourth_quadrant_diagonal() {
        let one = D38s12::ONE;
        let neg_one = -D38s12::ONE;
        let result = neg_one.atan2(one);
        let expected = -D38s12::quarter_pi();
        assert!(
            within_lsb(result, expected, TWO_LSB),
            "atan2(-1, 1) bits {}, expected -pi/4 bits {}",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
        );
    }

    #[test]
    fn atan2_positive_x_axis_is_zero() {
        let zero = D38s12::ZERO;
        let one = D38s12::ONE;
        assert_eq!(zero.atan2(one), D38s12::ZERO);
    }

    // ── Hyperbolic ────────────────────────────────────────────────────

    #[test]
    fn sinh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sinh(), D38s12::ZERO);
    }

    #[test]
    fn cosh_zero_is_one() {
        assert_eq!(D38s12::ZERO.cosh(), D38s12::ONE);
    }

    #[test]
    fn tanh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.tanh(), D38s12::ZERO);
    }

    #[test]
    fn asinh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.asinh(), D38s12::ZERO);
    }

    #[test]
    fn acosh_one_is_zero() {
        assert_eq!(D38s12::ONE.acosh(), D38s12::ZERO);
    }

    #[test]
    fn atanh_zero_is_zero() {
        assert_eq!(D38s12::ZERO.atanh(), D38s12::ZERO);
    }

    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn cosh_squared_minus_sinh_squared_is_one() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_234_567_890_123_i128,
            -1_234_567_890_123_i128,
            2_500_000_000_000_i128,
        ] {
            let x = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let ch = x.cosh();
            let sh = x.sinh();
            let diff = (ch * ch) - (sh * sh);
            assert!(
                within_lsb(diff, D38s12::ONE, FOUR_LSB),
                "cosh^2 - sinh^2 != 1 for raw={raw}: got bits {} (delta {})",
                diff.to_bits().as_i128(),
                (diff.to_bits().as_i128() - D38s12::ONE.to_bits().as_i128()).abs(),
            );
        }
    }

    // ── Angle conversions ─────────────────────────────────────────────

    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn to_degrees_pi_is_180() {
        let pi = D38s12::pi();
        let result = pi.to_degrees();
        let expected = D38s12::try_from(180).unwrap();
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_degrees(pi) bits {}, expected 180 bits {} (delta {})",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
            (result.to_bits().as_i128() - expected.to_bits().as_i128()).abs(),
        );
    }

    #[test]
    fn to_radians_180_is_pi() {
        let one_eighty = D38s12::try_from(180).unwrap();
        let result = one_eighty.to_radians();
        let expected = D38s12::pi();
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_radians(180) bits {}, expected pi bits {} (delta {})",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
            (result.to_bits().as_i128() - expected.to_bits().as_i128()).abs(),
        );
    }

    #[test]
    fn to_degrees_zero_is_zero() {
        assert_eq!(D38s12::ZERO.to_degrees(), D38s12::ZERO);
    }

    #[test]
    fn to_radians_zero_is_zero() {
        assert_eq!(D38s12::ZERO.to_radians(), D38s12::ZERO);
    }

    #[test]
    fn to_radians_to_degrees_round_trip() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_234_567_890_123_i128,
            -2_345_678_901_234_i128,
        ] {
            let x = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let recovered = x.to_degrees().to_radians();
            assert!(
                within_lsb(recovered, x, FOUR_LSB),
                "to_radians(to_degrees(x)) != x for raw={raw}: got bits {} (delta {})",
                recovered.to_bits().as_i128(),
                (recovered.to_bits().as_i128() - x.to_bits().as_i128()).abs(),
            );
        }
    }

    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn to_degrees_half_pi_is_90() {
        let result = D38s12::half_pi().to_degrees();
        let expected = D38s12::try_from(90).unwrap();
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_degrees(half_pi) bits {}, expected 90 bits {} (delta {})",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
            (result.to_bits().as_i128() - expected.to_bits().as_i128()).abs(),
        );
    }

    #[test]
    fn to_degrees_quarter_pi_is_45() {
        let result = D38s12::quarter_pi().to_degrees();
        let expected = D38s12::try_from(45).unwrap();
        assert!(
            within_lsb(result, expected, ANGLE_TOLERANCE_LSB),
            "to_degrees(quarter_pi) bits {}, expected 45 bits {} (delta {})",
            result.to_bits().as_i128(),
            expected.to_bits().as_i128(),
            (result.to_bits().as_i128() - expected.to_bits().as_i128()).abs(),
        );
    }

    // ── Cross-method consistency ──────────────────────────────────────

    #[test]
    fn tan_matches_sin_over_cos() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_000_000_000_000_i128,
            -1_000_000_000_000_i128,
            123_456_789_012_i128,
        ] {
            let x = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let t = x.tan();
            let sc = x.sin() / x.cos();
            assert!(
                within_lsb(t, sc, FOUR_LSB),
                "tan(x) != sin/cos for raw={raw}: tan bits {}, sin/cos bits {}",
                t.to_bits().as_i128(),
                sc.to_bits().as_i128(),
            );
        }
    }

    #[test]
    fn tanh_matches_sinh_over_cosh() {
        for raw in [
            500_000_000_000_i128,
            -500_000_000_000_i128,
            1_234_567_890_123_i128,
            -2_345_678_901_234_i128,
        ] {
            let x = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let t = x.tanh();
            let sc = x.sinh() / x.cosh();
            assert!(
                within_lsb(t, sc, FOUR_LSB),
                "tanh(x) != sinh/cosh for raw={raw}: tanh bits {}, sinh/cosh bits {}",
                t.to_bits().as_i128(),
                sc.to_bits().as_i128(),
            );
        }
    }

}
