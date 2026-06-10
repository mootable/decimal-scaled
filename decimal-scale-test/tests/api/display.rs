//! Display / Debug / exponent / radix formatting and `FromStr`
//! behaviour tests moved from `src/support/display.rs`. The in-src
//! `alloc` gates become `any(std, alloc)`: either feature enables
//! the library formatting surface these tests exercise.

mod from_src_display {
    use decimal_scaled::{D38s12, Int, ParseError};

    
    // ── Display ──

    /// ZERO renders as `0.000000000000` at SCALE = 12.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_zero_renders() {
        assert_eq!(D38s12::ZERO.to_string(), "0.000000000000");
    }

    /// ONE renders as `1.000000000000` at SCALE = 12.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_one_renders() {
        assert_eq!(D38s12::ONE.to_string(), "1.000000000000");
    }

    /// `1.5` renders with full SCALE fractional digits.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_one_point_five_renders() {
        let v = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        assert_eq!(v.to_string(), "1.500000000000");
    }

    /// Negative values get a leading `-`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_negative_renders() {
        let v = D38s12::from_bits(Int::<2>::try_from(-1_500_000_000_000_i128).unwrap());
        assert_eq!(v.to_string(), "-1.500000000000");
    }

    /// `0.001` (sub-unit positive) keeps leading-zero fractional.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_subunit_keeps_leading_zeros() {
        // 0.001 = 1_000_000_000 at SCALE 12
        let v = D38s12::from_bits(Int::<2>::try_from(1_000_000_000_i128).unwrap());
        assert_eq!(v.to_string(), "0.001000000000");
    }

    /// MAX renders without panicking. Spot-check the canonical form
    /// at SCALE 12: `170141183460469231731687303.715884105727`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_max_does_not_panic() {
        let s = D38s12::MAX.to_string();
        assert_eq!(s, "170141183460469231731687303.715884105727");
    }

    /// MIN renders without panicking. The unsigned-abs path handles
    /// the i128::MIN special case (|MIN| = MAX + 1, so the trailing
    /// digit is 8 not 7).
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_min_does_not_panic() {
        let s = D38s12::MIN.to_string();
        assert_eq!(s, "-170141183460469231731687303.715884105728");
    }

    /// SCALE = 0 has no decimal point.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn display_scale_zero_no_dot() {
        type D0 = decimal_scaled::D<Int<2>, 0>;
        assert_eq!(D0::ONE.to_string(), "1");
        assert_eq!(D0::ZERO.to_string(), "0");
        assert_eq!(D0::from_bits(Int::<2>::try_from(-42_i128).unwrap()).to_string(), "-42");
    }

    // ── Debug ──

    /// Debug delegates to Display + SCALE annotation.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn debug_includes_scale_and_value() {
        let v = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        let debug_str = format!("{v:?}");
        assert_eq!(debug_str, "D38<12>(1.500000000000)");
    }

    /// Debug on ZERO at a non-12 scale.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn debug_other_scale() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        let v = D6::ZERO;
        assert_eq!(format!("{v:?}"), "D38<6>(0.000000)");
    }

    // ── LowerExp / UpperExp ──

    /// `1.0` -> `1e0` (single digit mantissa).
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_exp_one() {
        let v = D38s12::ONE;
        assert_eq!(format!("{v:e}"), "1e0");
    }

    /// `1.5` -> `1.5e0`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_exp_one_point_five() {
        let v = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        assert_eq!(format!("{v:e}"), "1.5e0");
    }

    /// `15.0` -> `1.5e1`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_exp_fifteen() {
        let v = D38s12::from_bits(Int::<2>::try_from(15_000_000_000_000_i128).unwrap());
        assert_eq!(format!("{v:e}"), "1.5e1");
    }

    /// `0.0` -> `0e0`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_exp_zero() {
        assert_eq!(format!("{:e}", D38s12::ZERO), "0e0");
    }

    /// Sub-unit value -> negative exponent. `0.0015 = 1.5e-3`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_exp_subunit_negative_exponent() {
        // 0.0015 at SCALE 12 = 1_500_000_000
        let v = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_i128).unwrap());
        assert_eq!(format!("{v:e}"), "1.5e-3");
    }

    /// Negative value preserves sign.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_exp_negative() {
        let v = D38s12::from_bits(Int::<2>::try_from(-1_500_000_000_000_i128).unwrap());
        assert_eq!(format!("{v:e}"), "-1.5e0");
    }

    /// UpperExp uses `E`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn upper_exp_uses_capital_e() {
        let v = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        assert_eq!(format!("{v:E}"), "1.5E0");
    }

    // ── LowerHex / UpperHex / Octal / Binary ──

    /// LowerHex of D38s12::ONE is the hex of 10^12 (= 0xe8d4a51000),
    /// NOT the hex of `1.0` formatted as a decimal in hex.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn lower_hex_is_storage() {
        assert_eq!(format!("{:x}", D38s12::ONE), "e8d4a51000");
    }

    /// UpperHex of ONE: same digits in upper case.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn upper_hex_is_storage() {
        assert_eq!(format!("{:X}", D38s12::ONE), "E8D4A51000");
    }

    /// Octal of ZERO is `0`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn octal_zero() {
        assert_eq!(format!("{:o}", D38s12::ZERO), "0");
    }

    /// Binary of ONE has the `10^12` bit pattern (40 bits).
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn binary_one() {
        // 10^12 in binary: 1110_1000_1101_0100_1010_0101_0001_0000_0000_0000
        let s = format!("{:b}", D38s12::ONE);
        assert_eq!(s, "1110100011010100101001010001000000000000");
    }

    // ── ParseError Display ──

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn parse_error_display_messages() {
        assert_eq!(ParseError::Empty.to_string(), "empty input");
        assert_eq!(ParseError::SignOnly.to_string(), "sign with no digits");
        assert_eq!(
            ParseError::LeadingZero.to_string(),
            "redundant leading zero in integer part"
        );
        assert_eq!(
            ParseError::OverlongFractional.to_string(),
            "fractional part exceeds SCALE digits"
        );
        assert_eq!(
            ParseError::ScientificNotation.to_string(),
            "scientific notation not accepted"
        );
        assert_eq!(ParseError::InvalidChar.to_string(), "invalid character");
        assert_eq!(
            ParseError::OutOfRange.to_string(),
            "value out of representable range"
        );
        assert_eq!(
            ParseError::MissingDigits.to_string(),
            "decimal point with no adjacent digits"
        );
    }

    // ── FromStr happy path ──

    #[test]
    fn from_str_zero() {
        let v: D38s12 = "0".parse().unwrap();
        assert_eq!(v, D38s12::ZERO);
        let v: D38s12 = "0.0".parse().unwrap();
        assert_eq!(v, D38s12::ZERO);
    }

    #[test]
    fn from_str_one() {
        let v: D38s12 = "1".parse().unwrap();
        assert_eq!(v, D38s12::ONE);
        let v: D38s12 = "1.0".parse().unwrap();
        assert_eq!(v, D38s12::ONE);
    }

    /// Headline base-10 claim: `1.1` parses bit-exact.
    #[test]
    fn from_str_one_point_one_parses_exactly() {
        let v: D38s12 = "1.1".parse().unwrap();
        assert_eq!(v.to_bits(), 1_100_000_000_000);
    }

    /// Sign prefix.
    #[test]
    fn from_str_signs() {
        let neg: D38s12 = "-1.5".parse().unwrap();
        assert_eq!(neg.to_bits(), -1_500_000_000_000);

        let pos: D38s12 = "+1.5".parse().unwrap();
        assert_eq!(pos.to_bits(), 1_500_000_000_000);
    }

    /// Fractional with fewer digits than SCALE pads correctly.
    #[test]
    fn from_str_short_fractional_pads() {
        // "0.5" at SCALE 12 -> 5_000_000_000 (= 0.5 * 10^12).
        let v: D38s12 = "0.5".parse().unwrap();
        assert_eq!(v.to_bits(), 500_000_000_000);
    }

    /// Fractional with exactly SCALE digits is the natural form.
    #[test]
    fn from_str_full_scale_fractional() {
        let v: D38s12 = "1.500000000000".parse().unwrap();
        assert_eq!(v.to_bits(), 1_500_000_000_000);
    }

    // ── FromStr error paths ──

    #[test]
    fn from_str_empty_is_err() {
        let r: Result<D38s12, _> = "".parse();
        assert_eq!(r, Err(ParseError::Empty));
    }

    #[test]
    fn from_str_sign_only_is_err() {
        assert_eq!("-".parse::<D38s12>(), Err(ParseError::SignOnly));
        assert_eq!("+".parse::<D38s12>(), Err(ParseError::SignOnly));
    }

    #[test]
    fn from_str_leading_zero_is_err() {
        assert_eq!("01".parse::<D38s12>(), Err(ParseError::LeadingZero));
        assert_eq!("01.5".parse::<D38s12>(), Err(ParseError::LeadingZero));
        assert_eq!("00".parse::<D38s12>(), Err(ParseError::LeadingZero));
    }

    #[test]
    fn from_str_overlong_fractional_is_err() {
        // SCALE 12, fractional length 13 -> reject.
        let r: Result<D38s12, _> = "0.1234567890123".parse();
        assert_eq!(r, Err(ParseError::OverlongFractional));
    }

    #[test]
    fn from_str_scientific_notation_is_err() {
        assert_eq!("1e3".parse::<D38s12>(), Err(ParseError::ScientificNotation));
        assert_eq!(
            "1.5E2".parse::<D38s12>(),
            Err(ParseError::ScientificNotation)
        );
    }

    #[test]
    fn from_str_invalid_char_is_err() {
        assert_eq!("garbage".parse::<D38s12>(), Err(ParseError::InvalidChar));
        assert_eq!("1.2x".parse::<D38s12>(), Err(ParseError::InvalidChar));
        assert_eq!("1..2".parse::<D38s12>(), Err(ParseError::InvalidChar));
    }

    #[test]
    fn from_str_missing_digits_is_err() {
        assert_eq!(".5".parse::<D38s12>(), Err(ParseError::MissingDigits));
        assert_eq!("5.".parse::<D38s12>(), Err(ParseError::MissingDigits));
        assert_eq!("-.5".parse::<D38s12>(), Err(ParseError::MissingDigits));
    }

    #[test]
    fn from_str_out_of_range_is_err() {
        // 10^39 > i128::MAX (~1.7e38). At SCALE 12, the maximum
        // integer part is i128::MAX / 10^12 ~= 1.7e26, so an integer
        // part of 1e27 already overflows.
        let r: Result<D38s12, _> = "1000000000000000000000000000".parse();
        assert_eq!(r, Err(ParseError::OutOfRange));
    }

    /// Parse exactly at i128::MIN -- the asymmetric two's-complement
    /// boundary. At SCALE 12:
    /// `i128::MIN = -170141183460469231731687303715884105728`
    /// which splits into integer `170141183460469231731687303` and
    /// fractional `715884105728` (the negative form has the same
    /// digits since |MIN| = MAX + 1).
    #[test]
    fn from_str_i128_min_boundary() {
        let s = "-170141183460469231731687303.715884105728";
        let v: D38s12 = s.parse().unwrap();
        assert_eq!(v.to_bits(), i128::MIN);
    }

    /// Parse exactly at i128::MAX boundary. At SCALE 12 the canonical
    /// form is `170141183460469231731687303.715884105727`.
    #[test]
    fn from_str_i128_max_boundary() {
        let s = "170141183460469231731687303.715884105727";
        let v: D38s12 = s.parse().unwrap();
        assert_eq!(v.to_bits(), i128::MAX);
    }

    /// One-past-MAX positive overflows.
    #[test]
    fn from_str_just_above_max_overflows() {
        // ...728 is one fractional LSB above i128::MAX.
        let s = "170141183460469231731687303.715884105728";
        let r: Result<D38s12, _> = s.parse();
        assert_eq!(r, Err(ParseError::OutOfRange));
    }

    // ── Property tests: parse(value.to_string()) round-trip ──

    /// Round-trip property for representative storage values.
    /// Uses safe-decimal-test-values (no clippy approx_constant traps).
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn round_trip_representative_values() {
        let cases: &[i128] = &[
            0,
            1,
            -1,
            1_000_000_000_000, // 1.0
            -1_000_000_000_000,
            1_500_000_000_000, // 1.5
            -1_500_000_000_000,
            1_100_000_000_000, // 1.1 (the headline base-10 claim)
            2_200_000_000_000, // 2.2
            3_300_000_000_000, // 3.3
            // Safe arbitrary-looking literal (avoids approx_constant
            // triggers like 3.14, 2.718, 1.414 etc.):
            1_234_567_890_123, // ~1.234567890123
            -1_234_567_890_123,
            4_567_891_234_567, // ~4.567891234567
            7_890_123_456_789, // ~7.890123456789
            i128::MAX,
            i128::MIN,
            i128::MAX / 2,
            i128::MIN / 2,
        ];
        for &raw in cases {
            let v = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            let s = v.to_string();
            let parsed: D38s12 = s.parse().unwrap_or_else(|e| {
                panic!("round-trip parse failed for raw={raw}, s={s:?}, err={e:?}")
            });
            assert_eq!(
                parsed.to_bits(),
                raw,
                "round-trip mismatch: raw={raw}, s={s:?}, parsed_bits={}",
                parsed.to_bits()
            );
        }
    }

    /// Round-trip property at SCALE = 6 to exercise the const-generic
    /// path away from the v1 SCALE = 12.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn round_trip_other_scale() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        let cases: &[i128] = &[
            0,
            1,
            -1,
            1_000_000,
            -1_000_000,
            1_500_000,
            i128::MAX,
            i128::MIN,
        ];
        for &raw in cases {
            let v = D6::from_bits(Int::<2>::try_from(raw).unwrap());
            let s = v.to_string();
            let parsed: D6 = s.parse().expect("round-trip parse");
            assert_eq!(
                parsed.to_bits(),
                raw,
                "round-trip mismatch at SCALE=6, raw={raw}"
            );
        }
    }

    /// Round-trip at SCALE = 0 (integer-only) to exercise the
    /// no-decimal-point path.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn round_trip_scale_zero() {
        type D0 = decimal_scaled::D<Int<2>, 0>;
        let cases: &[i128] = &[0, 1, -1, 42, -42, i128::MAX, i128::MIN];
        for &raw in cases {
            let v = D0::from_bits(Int::<2>::try_from(raw).unwrap());
            let s = v.to_string();
            let parsed: D0 = s.parse().expect("round-trip parse");
            assert_eq!(
                parsed.to_bits(),
                raw,
                "round-trip mismatch at SCALE=0, raw={raw}"
            );
        }
    }
}
