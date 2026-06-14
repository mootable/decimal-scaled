//! Parse-error surface for the public `FromStr` grammar. Consolidated from the
//! root crate's integration tests; each source file rides in its own
//! `mod from_<source>` block. The wide-gated deep-scale parser tests
//! (`from_str_wide.rs`) join this target in the feature-gated batch.

mod from_parse_errors {
    //! `D38::from_str` error path coverage. `from_str` rejects malformed
    //! decimal literals with one of [`ParseError`]'s variants; each variant
    //! corresponds to a distinct grammar rule in the parser and is exercised
    //! here.

    use core::str::FromStr;
    use decimal_scaled::D38;

    #[test]
    fn sign_only_input_is_rejected() {
        assert!(
            D38::<2>::from_str("-").is_err(),
            "bare `-` should be SignOnly"
        );
        assert!(
            D38::<2>::from_str("+").is_err(),
            "bare `+` should be SignOnly"
        );
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(D38::<2>::from_str("").is_err());
    }

    #[test]
    fn dot_only_input_is_rejected() {
        assert!(D38::<2>::from_str(".").is_err());
    }

    #[test]
    fn missing_fractional_after_dot_is_rejected() {
        assert!(D38::<2>::from_str("1.").is_err());
    }

    #[test]
    fn missing_integer_before_dot_is_rejected() {
        assert!(D38::<2>::from_str(".5").is_err());
    }

    #[test]
    fn leading_zero_in_integer_part_is_rejected() {
        assert!(
            D38::<2>::from_str("01").is_err(),
            "leading-zero integer part is reserved for the literal `0`"
        );
        assert!(D38::<2>::from_str("01.5").is_err());
    }

    #[test]
    fn overlong_fractional_is_rejected() {
        // SCALE=2 allows at most 2 fractional digits.
        assert!(D38::<2>::from_str("1.123").is_err());
    }

    #[test]
    fn integer_part_above_i128_max_is_rejected() {
        // 41 digits â€” well past i128's 39-digit MAX (~1.7e38).
        assert!(D38::<0>::from_str("99999999999999999999999999999999999999999").is_err());
    }

    #[test]
    fn value_just_past_i128_max_is_rejected() {
        // i128::MAX + 1 = 2^127.
        let r = D38::<0>::from_str("170141183460469231731687303715884105728");
        assert!(r.is_err());
        // i128::MIN - 1.
        let r = D38::<0>::from_str("-170141183460469231731687303715884105729");
        assert!(r.is_err());
    }

    #[test]
    fn valid_canonical_forms_round_trip() {
        // Counter-tests so we know the reject paths above aren't false alarms.
        assert!(D38::<2>::from_str("0").is_ok());
        assert!(D38::<2>::from_str("0.00").is_ok());
        assert!(D38::<2>::from_str("1.5").is_ok());
        assert!(D38::<2>::from_str("-1.5").is_ok());
        assert!(D38::<0>::from_str("170141183460469231731687303715884105727").is_ok()); // i128::MAX
        assert!(D38::<0>::from_str("-170141183460469231731687303715884105728").is_ok()); // i128::MIN
    }
}

#[cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]
mod from_from_str_wide {
    //! Wide-tier `FromStr` coverage at deep scales.
    //!
    //! Regression guard for the historical `u128` ceiling on the shared
    //! parser. Before the fix, the parser materialised `10^SCALE` in
    //! `u128`, which overflows at `SCALE > 38` and made any wide-tier
    //! `from_str` call at deep scale fail with
    //! [`decimal_scaled::ParseError::OutOfRange`] (or worse â€” silently
    //! wrap to garbage). After the fix the wide-tier from-str macro
    //! arm does the arithmetic at the storage width, so deep-scale
    //! literals round-trip cleanly.

    use core::str::FromStr;
    use decimal_scaled::{D38, D76, D307, D1232};

    #[test]
    fn d76_deep_scale_parses_one_point_five() {
        // SCALE = 60 was the first failure point: 10^60 overflows u128
        // (u128::MAX ~ 3.4e38).
        let v = D76::<60>::from_str("1.5").expect("D76<60>::from_str(\"1.5\")");
        let round_trip = v.to_string();
        // SCALE = 60 fractional digits, leading "1.5" then 59 zeros.
        let expected = format!("1.5{}", "0".repeat(59));
        assert_eq!(round_trip, expected);
    }

    #[test]
    fn d307_deep_scale_parses_one_point_five() {
        let v = D307::<150>::from_str("1.5").expect("D307<150>::from_str(\"1.5\")");
        let round_trip = v.to_string();
        let expected = format!("1.5{}", "0".repeat(149));
        assert_eq!(round_trip, expected);
    }

    #[test]
    fn d307_deep_scale_round_trip_back_to_value() {
        let s_in = "1.5";
        let v = D307::<150>::from_str(s_in).expect("parse");
        let s_out = v.to_string();
        let reparsed = D307::<150>::from_str(&s_out).expect("reparse");
        assert_eq!(v, reparsed);
    }

    #[test]
    fn d1231_deepest_scale_parses_one_point_five() {
        // Deepest supported tier Ã— scale combination.
        let v = D1232::<1230>::from_str("1.5").expect("D1232<1230>::from_str(\"1.5\")");
        let round_trip = v.to_string();
        let expected = format!("1.5{}", "0".repeat(1229));
        assert_eq!(round_trip, expected);
    }

    #[test]
    fn d1231_deepest_scale_handles_negative() {
        // Negative path: the per-storage accumulator subtracts each digit
        // rather than negating after the fact, so the asymmetric two's-
        // complement `MIN` boundary is reachable without overflowing on
        // the positive side. Spot-check a non-boundary negative value.
        let v = D1232::<1230>::from_str("-1.5").expect("negative parse");
        let round_trip = v.to_string();
        let expected = format!("-1.5{}", "0".repeat(1229));
        assert_eq!(round_trip, expected);
    }

    #[test]
    fn d307_deep_scale_many_fractional_digits() {
        // Exercise the per-digit `checked_mul(10) + checked_add(digit)`
        // loop with a long fractional run.
        let frac = "1234567890".repeat(10); // 100 digits
        let input = format!("0.{frac}");
        let v = D307::<150>::from_str(&input).expect("parse");
        let s_out = v.to_string();
        // Round-trip preserves the value bit-exactly; the displayed form
        // has SCALE = 150 fractional digits, so 50 trailing zeros pad
        // the 100-digit input.
        let expected = format!("0.{frac}{}", "0".repeat(50));
        assert_eq!(s_out, expected);
    }

    #[test]
    fn d307_deep_scale_zero() {
        let v = D307::<150>::from_str("0").expect("parse 0");
        assert_eq!(v.to_string(), format!("0.{}", "0".repeat(150)));
        let v = D307::<150>::from_str("0.0").expect("parse 0.0");
        assert_eq!(v.to_string(), format!("0.{}", "0".repeat(150)));
    }

    #[test]
    fn d307_deep_scale_overlong_fractional_is_err() {
        // SCALE = 150, fractional length 151 â†’ reject.
        let frac = "0".repeat(151);
        let input = format!("0.{frac}");
        assert!(D307::<150>::from_str(&input).is_err());
    }

    #[test]
    fn d76_deep_scale_integer_only() {
        // No decimal point: only the integer-scale path is exercised.
        let v = D76::<60>::from_str("42").expect("parse 42");
        let expected = format!("42.{}", "0".repeat(60));
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn d38_shallow_scale_unchanged_after_refactor() {
        // Sanity: the narrow-tier parser still produces the same value
        // after the front-end was extracted to `parse_components`.
        let v: D38<12> = "1.5".parse().expect("D38<12> parses");
        assert_eq!(v.to_string(), "1.500000000000");

        let neg: D38<12> = "-1.5".parse().expect("D38<12> negative parses");
        assert_eq!(neg.to_string(), "-1.500000000000");
    }
}
