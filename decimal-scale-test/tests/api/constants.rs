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

    /// deg_per_rad (180/pi â‰ˆ 57.29577951308232...) at SCALE=12.
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

    /// rad_per_deg (pi/180 â‰ˆ 0.017453292519943295...) at SCALE=12.
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
        //                   so half-to-even rounds down â€” no bump.
        let expected: i128 = 31_415_926_535_897_932_384_626_433_832_795_028_842;
        assert_eq!(D37::pi().to_bits().as_i128(), expected);
    }

    // `D38<38>` storage range is approximately Â±1.70141 (i128::MAX /
    // 10^38). The three constants whose magnitude exceeds that bound
    // must panic with a clear "out of storage range" message:
    //
    // - pi    â‰ˆ 3.14159    > 1.70141 â†’ must panic
    // - tau   â‰ˆ 6.28318    > 1.70141 â†’ must panic
    // - e     â‰ˆ 2.71828    > 1.70141 â†’ must panic
    //
    // The three that DO fit must be correctly rounded to 0.5 ULP:
    //
    // - half_pi    â‰ˆ 1.57079   < 1.70141 â†’ must round to 0.5 ULP
    // - quarter_pi â‰ˆ 0.78540   < 1.70141 â†’ must round to 0.5 ULP
    // - golden     â‰ˆ 1.61803   < 1.70141 â†’ must round to 0.5 ULP

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
    /// (their magnitudes are inside the type's Â±1.7 storage range) and
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

#[cfg(feature = "wide")]
mod from_wide_constants_all_six {
    //! Coverage suite for `consts_wide.rs` â€” all six wide-tier constants
    //! (`pi`, `tau`, `half_pi`, `quarter_pi`, `golden`, `e`) on D76 / D153
    //! / D307 at multiple scales.
    //!
    //! The existing `wide_constants_high_scale.rs` covers Ï€ and one high
    //! scale per tier. This file exercises every constant on every wide
    //! tier at the canonical reference scale and at a small storage scale.
    //! Every test asserts the additive identities between the constants
    //! (`Ï„ = Ï€ + Ï€`, `Ï€ = Ï€/2 + Ï€/2`, `Ï€/2 = Ï€/4 + Ï€/4`, within 1 LSB to
    //! absorb the independent per-constant rounding) plus coarse magnitude
    //! bounds for `e` and `golden` â€” digit-exact values are the golden
    //! gate's job, identities and reachability are this file's.

    use decimal_scaled::DecimalConstants;

    /// Assert the six constants' additive identities + magnitude bounds at one
    /// `(tier, scale)` cell. `$one_bits` builds the 1-LSB witness from raw storage.
    macro_rules! check_constants {
        ($D:ty, $Int:ty) => {{
            type D = $D;
            let one_lsb = <D>::from_bits(<$Int>::try_from(1_i128).unwrap());
            let within_one_lsb = |a: D, b: D, what: &str| {
                let diff = if a > b { a - b } else { b - a };
                assert!(diff <= one_lsb, "{what}: diff = {diff:?}");
            };
            within_one_lsb(D::tau(), D::pi() + D::pi(), "tau vs pi + pi");
            within_one_lsb(D::pi(), D::half_pi() + D::half_pi(), "pi vs 2 half_pi");
            within_one_lsb(D::half_pi(), D::quarter_pi() + D::quarter_pi(), "half_pi vs 2 quarter_pi");
            let two: D = "2".parse().unwrap();
            let three: D = "3".parse().unwrap();
            let one: D = "1".parse().unwrap();
            assert!(D::e() > two && D::e() < three, "e in (2, 3)");
            assert!(D::golden() > one && D::golden() < two, "golden in (1, 2)");
        }};
    }

    #[test]
    fn d76_all_six_constants_at_scale_12() {
        check_constants!(decimal_scaled::D76<12>, decimal_scaled::Int<4>);
    }

    #[test]
    fn d76_all_six_constants_at_scale_37() {
        check_constants!(decimal_scaled::D76<37>, decimal_scaled::Int<4>);
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d153_all_six_constants() {
        check_constants!(decimal_scaled::D153<37>, decimal_scaled::Int<8>);
        // The canonical reference scale (S = 152).
        check_constants!(decimal_scaled::D153<152>, decimal_scaled::Int<8>);
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d307_all_six_constants() {
        check_constants!(decimal_scaled::D307<37>, decimal_scaled::Int<16>);
        check_constants!(decimal_scaled::D307<306>, decimal_scaled::Int<16>);
    }
}

#[cfg(all(feature = "wide", not(any(feature = "rounding-half-away-from-zero", feature = "rounding-half-toward-zero", feature = "rounding-trunc", feature = "rounding-floor", feature = "rounding-ceiling"))))]
mod from_wide_constants_high_scale {
    //! Verifies the per-width raw constants in `consts_wide.rs` produce
    //! correct values at the wide tiers' deeper scales â€” the case that
    //! previously panicked on the rescale-up `i128` overflow.

    // Truth strings below are the half-to-even-rounded pi reference; gate
    // the module to the default rounding mode so every test always asserts.
    use decimal_scaled::{D76, D153, D307, DecimalConstants};

    /// D76<76>::pi() used to panic at the i128 rescale-up. After wiring
    /// the build-time-generated 75-digit Int256 constants, it returns a
    /// well-defined value.
    #[test]
    fn d76_pi_at_max_scale_does_not_panic() {
        // SCALE=50: deeper than D38 but inside D76's max of 76.
        let pi50 = D76::<50>::pi();
        // Sanity: roughly 3 in integer part.
        assert!(pi50.to_bits().to_string().starts_with("314"));
    }

    #[test]
    fn d76_pi_at_scale_75_is_exact() {
        // At SCALE = SCALE_REF (75), pi() returns the raw constant
        // exactly â€” no rescaling.
        let pi75 = D76::<75>::pi();
        let s = pi75.to_bits().to_string();
        // First few significant digits of pi (no decimal point in raw).
        assert!(
            s.starts_with("3141592653589793238462643383279502884"),
            "got {s}"
        );
    }

    #[test]
    fn d153_pi_at_scale_152_works() {
        // v0.4.0 cap: MAX_SCALE for D153 is 152.
        let pi = D153::<152>::pi();
        let s = pi.to_bits().to_string();
        assert!(
            s.starts_with("3141592653589793238462643383279502884"),
            "got {s}"
        );
    }

    #[test]
    fn d307_pi_at_scale_300_works() {
        let pi = D307::<300>::pi();
        let s = pi.to_bits().to_string();
        assert!(
            s.starts_with("3141592653589793238462643383279502884"),
            "got {s}"
        );
    }

    /// Cross-tier check: D76<37> and D38<37>::pi() should produce the
    /// same logical value (the storage layouts differ but the rescaled
    /// integer agrees). Uses the public `Decimal` trait to bridge.
    #[test]
    fn d76_pi_at_scale_37_matches_d38() {
        use decimal_scaled::D38;
        let n = i128::from(D38::<37>::pi().to_bits());
        let w = D76::<37>::pi().to_bits();
        let w_as_i128 = w.to_i128_checked().expect("fits");
        let diff = (w_as_i128 - n).abs();
        assert!(diff <= 1, "D76<37>::pi {w_as_i128} vs D38<37>::pi {n}");
    }
}
