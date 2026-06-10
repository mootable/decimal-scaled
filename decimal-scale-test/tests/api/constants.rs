//! `DecimalConstants` trait-surface tests moved from
//! `src/types/consts/decimal_constants.rs`. Pure public-trait
//! access (pi/tau/e/golden/deg-rad + EPSILON/MIN_POSITIVE); the
//! crate-default-rounding guards become def-rnd cfg gates.

mod from_src_decimal_constants {
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

    // Bit-exact assertions at SCALE = 12.
    //
    // At SCALE = 12 each constant is the 37-digit raw integer divided by
    // 10^23, rounded half-to-even.

    /// pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 3_141_592_653_589.
    /// 14th digit is 7 (from position 14 of the raw) -> round up.
    /// Expected: 3_141_592_653_590.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn pi_is_bit_exact_at_scale_12() {
        assert_eq!(D38s12::pi().to_bits().as_i128(), 3_141_592_653_590_i128);
    }

    /// tau at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 6_283_185_307_179.
    /// 14th digit is 5 -> round up. Expected: 6_283_185_307_180.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn tau_is_bit_exact_at_scale_12() {
        assert_eq!(D38s12::tau().to_bits().as_i128(), 6_283_185_307_180_i128);
    }

    /// half_pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_570_796_326_794.
    /// 14th digit is 8 -> round up. Expected: 1_570_796_326_795.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn half_pi_is_bit_exact_at_scale_12() {
        assert_eq!(D38s12::half_pi().to_bits().as_i128(), 1_570_796_326_795_i128);
    }

    /// quarter_pi at SCALE=12: raw / 10^23.
    /// Truncated 12 digits: 785_398_163_397.
    /// 13th digit is 4 -> no round-up. Expected: 785_398_163_397.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn quarter_pi_is_bit_exact_at_scale_12() {
        assert_eq!(D38s12::quarter_pi().to_bits().as_i128(), 785_398_163_397_i128);
    }

    /// e at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 2_718_281_828_459.
    /// 14th digit is 0 -> no round-up. Expected: 2_718_281_828_459.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn e_is_bit_exact_at_scale_12() {
        assert_eq!(D38s12::e().to_bits().as_i128(), 2_718_281_828_459_i128);
    }

    /// golden at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_618_033_988_749.
    /// 14th digit is 8 -> round up. Expected: 1_618_033_988_750.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn golden_is_bit_exact_at_scale_12() {
        assert_eq!(D38s12::golden().to_bits().as_i128(), 1_618_033_988_750_i128);
    }

    /// deg_per_rad (180/pi ≈ 57.29577951308232...) at SCALE=12.
    /// raw floor at scale 12 = 57_295_779_513_082, dropped tail < 1/2
    /// (round_up = 0), so half-to-even keeps the floor.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn deg_per_rad_is_bit_exact_at_scale_12() {
        assert_eq!(
            D38s12::deg_per_rad().to_bits().as_i128(),
            57_295_779_513_082_i128
        );
    }

    /// rad_per_deg (pi/180 ≈ 0.017453292519943295...) at SCALE=12.
    /// raw floor at scale 12 = 17_453_292_519, dropped tail >= 1/2
    /// (round_up = 1), so half-to-even rounds up to 17_453_292_520.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn rad_per_deg_is_bit_exact_at_scale_12() {
        assert_eq!(
            D38s12::rad_per_deg().to_bits().as_i128(),
            17_453_292_520_i128
        );
    }

    /// Directed-mode siblings: `deg_per_rad_with(Ceiling)` bumps the
    /// floor (the tail is non-zero for an irrational), and
    /// `rad_per_deg_with(Floor)` keeps the floor. Independent of the
    /// crate-default mode, so no half-to-even guard.
    #[test]
    fn deg_rad_directed_modes_at_scale_12() {
        use decimal_scaled::RoundingMode::{Ceiling, Floor};
        assert_eq!(
            D38s12::deg_per_rad_with(Ceiling).to_bits().as_i128(),
            57_295_779_513_083_i128
        );
        assert_eq!(
            D38s12::rad_per_deg_with(Floor).to_bits().as_i128(),
            17_453_292_519_i128
        );
    }

    // Closeness checks against core::f64::consts.
    // These verify that the correct reference digits were selected; the
    // bit-exact tests above are the primary acceptance criteria.

    /// pi() converted to f64 is within 1e-11 of `core::f64::consts::PI`.
    /// At SCALE=12, 1 LSB = 1e-12, so 1e-11 covers rescale rounding plus
    /// the f64 conversion step.
    #[test]
    fn pi_close_to_f64_pi() {
        let diff = (D38s12::pi().to_f64() - core::f64::consts::PI).abs();
        assert!(diff < 1e-11, "pi diverges from f64 PI by {diff}");
    }

    #[test]
    fn tau_close_to_f64_tau() {
        let diff = (D38s12::tau().to_f64() - core::f64::consts::TAU).abs();
        assert!(diff < 1e-11, "tau diverges from f64 TAU by {diff}");
    }

    #[test]
    fn half_pi_close_to_f64_frac_pi_2() {
        let diff = (D38s12::half_pi().to_f64() - core::f64::consts::FRAC_PI_2).abs();
        assert!(
            diff < 1e-11,
            "half_pi diverges from f64 FRAC_PI_2 by {diff}"
        );
    }

    #[test]
    fn quarter_pi_close_to_f64_frac_pi_4() {
        let diff = (D38s12::quarter_pi().to_f64() - core::f64::consts::FRAC_PI_4).abs();
        assert!(
            diff < 1e-11,
            "quarter_pi diverges from f64 FRAC_PI_4 by {diff}"
        );
    }

    #[test]
    fn e_close_to_f64_e() {
        let diff = (D38s12::e().to_f64() - core::f64::consts::E).abs();
        assert!(diff < 1e-11, "e diverges from f64 E by {diff}");
    }

    /// golden() converted to f64 is within 1e-11 of the closed form
    /// `(1 + sqrt(5)) / 2`. Requires std for `f64::sqrt`.
    #[cfg(feature = "std")]
    #[test]
    fn golden_close_to_closed_form() {
        let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
        let diff = (D38s12::golden().to_f64() - expected).abs();
        assert!(diff < 1e-11, "golden diverges from closed-form by {diff}");
    }

    // EPSILON / MIN_POSITIVE

    #[test]
    fn epsilon_is_one_ulp() {
        assert_eq!(D38s12::EPSILON.to_bits().as_i128(), 1_i128);
        assert!(D38s12::EPSILON > D38s12::ZERO);
    }

    #[test]
    fn min_positive_is_one_ulp() {
        assert_eq!(D38s12::MIN_POSITIVE.to_bits().as_i128(), 1_i128);
        assert_eq!(D38s12::MIN_POSITIVE, D38s12::EPSILON);
    }

    /// At SCALE = 6 the LSB is 10^-6; EPSILON is still raw 1.
    #[test]
    fn epsilon_at_scale_6_is_one_ulp() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        assert_eq!(D6::EPSILON.to_bits().as_i128(), 1_i128);
        assert_eq!(D6::MIN_POSITIVE.to_bits().as_i128(), 1_i128);
    }

    // Cross-scale exercises

    /// At SCALE = 6, pi() should equal 3.141593 (rounded half-to-even from
    /// 3.1415926535...). Expected raw bits: 3_141_593.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn pi_at_scale_6_is_bit_exact() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        assert_eq!(D6::pi().to_bits().as_i128(), 3_141_593_i128);
    }

    /// At SCALE = 0, pi() rounds to 3 (first fractional digit is 1, no
    /// round-up).
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn pi_at_scale_0_is_three() {
        type D0 = decimal_scaled::D<Int<2>, 0>;
        assert_eq!(D0::pi().to_bits().as_i128(), 3_i128);
    }

    /// `D38<37>::pi()` is the canonical pi rounded half-to-even to 37
    /// fractional digits. The 75-digit Int<4> reference is rescaled
    /// down to 37 digits; the result is bit-identical to the
    /// hand-tabulated constant.
    #[test]
    fn pi_at_scale_37_matches_canonical_37_digit_rounding() {
        type D37 = decimal_scaled::D<Int<2>, 37>;
        // pi to 38 digits: 3.14159265358979323846264338327950288420
        //                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                   keep 37 frac digits; the 38th digit is 0
        //                   so half-to-even rounds down — no bump.
        let expected: i128 = 31_415_926_535_897_932_384_626_433_832_795_028_842;
        assert_eq!(D37::pi().to_bits().as_i128(), expected);
    }

    // `D38<38>` storage range is approximately ±1.70141 (i128::MAX /
    // 10^38). The three constants whose magnitude exceeds that bound
    // must panic with a clear "out of storage range" message:
    //
    // - pi    ≈ 3.14159    > 1.70141 → must panic
    // - tau   ≈ 6.28318    > 1.70141 → must panic
    // - e     ≈ 2.71828    > 1.70141 → must panic
    //
    // The three that DO fit must be correctly rounded to 0.5 ULP:
    //
    // - half_pi    ≈ 1.57079   < 1.70141 → must round to 0.5 ULP
    // - quarter_pi ≈ 0.78540   < 1.70141 → must round to 0.5 ULP
    // - golden     ≈ 1.61803   < 1.70141 → must round to 0.5 ULP

    #[test]
    #[should_panic(expected = "out of storage range")]
    fn pi_at_scale_38_panics_storage_range() {
        let _ = decimal_scaled::D::<Int<2>, 38>::pi();
    }

    #[test]
    #[should_panic(expected = "out of storage range")]
    fn tau_at_scale_38_panics_storage_range() {
        let _ = decimal_scaled::D::<Int<2>, 38>::tau();
    }

    #[test]
    #[should_panic(expected = "out of storage range")]
    fn e_at_scale_38_panics_storage_range() {
        let _ = decimal_scaled::D::<Int<2>, 38>::e();
    }

    /// `half_pi` / `quarter_pi` / `golden` at `D38<38>` must not panic
    /// (their magnitudes are inside the type's ±1.7 storage range) and
    /// each must be correctly rounded to 0.5 ULP (= 1 LSB).
    #[test]
    fn fitting_constants_at_scale_38_are_correctly_rounded() {
        // half_pi to 38 digits: 1.57079632679489661923132169163975144210
        let expected_half_pi: i128 = 157_079_632_679_489_661_923_132_169_163_975_144_210;
        let got = decimal_scaled::D::<Int<2>, 38>::half_pi().to_bits().as_i128();
        let diff = (got - expected_half_pi).abs();
        assert!(
            diff <= 1,
            "half_pi: got {got}, expected {expected_half_pi}, diff {diff} > 1 LSB"
        );

        // quarter_pi to 38 digits: 0.78539816339744830961566084581987572105
        let expected_quarter_pi: i128 = 78_539_816_339_744_830_961_566_084_581_987_572_105;
        let got = decimal_scaled::D::<Int<2>, 38>::quarter_pi().to_bits().as_i128();
        let diff = (got - expected_quarter_pi).abs();
        assert!(
            diff <= 1,
            "quarter_pi: got {got}, expected {expected_quarter_pi}, diff {diff} > 1 LSB"
        );

        // golden to 38 digits: 1.61803398874989484820458683436563811772
        let expected_golden: i128 = 161_803_398_874_989_484_820_458_683_436_563_811_772;
        let got = decimal_scaled::D::<Int<2>, 38>::golden().to_bits().as_i128();
        let diff = (got - expected_golden).abs();
        assert!(
            diff <= 1,
            "golden: got {got}, expected {expected_golden}, diff {diff} > 1 LSB"
        );
    }

    /// Negative-side rounding: negating pi gives the expected raw bits.
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn neg_pi_round_trip() {
        let pi = D38s12::pi();
        let neg_pi = -pi;
        assert_eq!(neg_pi.to_bits().as_i128(), -3_141_592_653_590_i128);
    }

    // (`rescale_from_ref` boundary tests removed: the rounding logic now
    // lives in `D38::rescale` / `src/rounding.rs::apply_rounding` and is
    // covered by the tests in those modules.)

}
