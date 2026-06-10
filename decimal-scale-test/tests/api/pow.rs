//! Integer-exponent `pow` / `powi` family plus its overflow variants.
//! The `from_src_powers` block is deduped against `from_macros_pow` (the
//! shared pow-contract restatements live there; unique cases moved here).

mod from_macros_pow {
    //! Coverage suite for `crate::macros::pow::decl_decimal_pow!` â€” the
    //! integer-exponent `pow` / `powi` family, plus the four overflow
    //! variants. Exercises D9 / D18 / D38 (and the wide tiers when their
    //! features are on). D38 has its own hand-written pow path
    //! (`powers_strict.rs::pow_int`), but its macro-aliased `pow` shares the
    //! same square-and-multiply contract, so the same assertions hold.
    //!
    //! The contract from `macros/pow.rs`:
    //! * `pow(0)` returns `ONE` for every base, including `ZERO`.
    //! * `pow(1)` returns the base.
    //! * For non-negative `exp`, `powi(exp) == pow(exp as u32)`.
    //! * For negative `exp`, `powi(exp) == ONE / pow(exp.unsigned_abs())`.
    //! * `checked_pow` returns `None` iff any intermediate `checked_mul`
    //!   overflows.
    //! * `wrapping_pow` matches the two's-complement wrap of the
    //!   mathematical result.
    //! * `saturating_pow` returns `MAX` on overflow when the mathematical
    //!   result is positive, `MIN` when negative.
    //! * `overflowing_pow` returns `(wrapping_result, true)` exactly when
    //!   any step overflowed.

    use decimal_scaled::{D18, D38};

    // â”€â”€â”€ Narrow-tier (D9<0>, D18<0>, D38<0>) base-case identities â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    //
    // SCALE = 0 keeps the storage equal to the integer value so the
    // assertions can be written in terms of plain integers.

    #[test]
    fn pow_exp_zero_returns_one_d18() {
        for raw in [0i64, 1, -1, 5, -5, 123_456_789_012i64] {
            let v = D18::<0>::from_bits(decimal_scaled::Int::<1>::from(raw));
            assert_eq!(v.pow(0).to_bits(), 1, "0^0=1 contract: input raw {raw}");
        }
    }

    #[test]
    fn pow_exp_one_returns_self_d18() {
        for raw in [0i64, 1, -1, 5, -5, 12345] {
            let v = D18::<0>::from_bits(decimal_scaled::Int::<1>::from(raw));
            assert_eq!(v.pow(1).to_bits(), raw, "x^1=x");
        }
    }

    #[test]
    fn pow_small_exponents_d18() {
        let two = D18::<0>::from(2);
        assert_eq!(two.pow(20).to_bits(), 1 << 20);
        let ten = D18::<0>::from(10);
        assert_eq!(ten.pow(9).to_bits(), 1_000_000_000);
    }

    #[test]
    fn pow_small_exponents_d38() {
        let two = D38::<0>::from(2);
        assert_eq!(two.pow(30).to_bits(), 1i128 << 30);
        let ten = D38::<0>::from(10);
        assert_eq!(ten.pow(18).to_bits(), 1_000_000_000_000_000_000i128);
    }

    // â”€â”€â”€ powi negative-exponent semantics â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    //
    // `powi(-n)` = `ONE / pow(n)`. At SCALE=0 the integer divide truncates,
    // so for |base| > 1 the result is ZERO. To exercise the
    // reciprocal-path without hitting truncation, work at SCALE>0.

    #[test]
    fn powi_negative_exponent_d38_scale12() {
        use decimal_scaled::D38s12;
        let two = D38s12::from(2);
        // 2^-3 = 0.125 â†’ raw 125_000_000_000
        assert_eq!(two.powi(-3).to_bits(), 125_000_000_000);
        // 2^3 == 8 (positive exp path)
        assert_eq!(two.powi(3).to_bits(), 8_000_000_000_000);
        // 2^0 == 1
        assert_eq!(two.powi(0).to_bits(), 1_000_000_000_000);
    }

    #[test]
    fn powi_d9_d18_positive_negative_exp() {
        use decimal_scaled::{D18};

        // D18<4>: 2^3 = 8 â†’ 80_000
        let two = D18::<4>::from(2);
        assert_eq!(two.powi(3).to_bits(), 80_000);
        assert_eq!(two.powi(0).to_bits(), 10_000);
        // 2^-3 = 0.125 â†’ 1_250
        assert_eq!(two.powi(-3).to_bits(), 1_250);

        // D18
        let two18 = D18::<8>::from(2);
        assert_eq!(two18.powi(3).to_bits(), 800_000_000);
        assert_eq!(two18.powi(0).to_bits(), 100_000_000);
        assert_eq!(two18.powi(-3).to_bits(), 12_500_000);
    }

    #[test]
    fn powi_handles_i32_min_without_signed_negation_overflow_d38() {
        // The `powi` code path uses `i32::unsigned_abs` to avoid the
        // signed-negation overflow that `(-i32::MIN) as u32` would cause.
        // Using ONE as the base keeps the multiply itself trivial so this
        // exercises the conversion edge alone.
        use decimal_scaled::D38s12;
        let one = D38s12::ONE;
        assert_eq!(one.powi(i32::MIN), D38s12::ONE);
        // ZERO base with i32::MIN exponent goes through ONE / pow(...) â†’ ZERO
        // for any positive raised power, but 0^|MIN| would overflow the loop's
        // multiply. Skip this edge: behaviour matches the type's Mul.
    }

    // â”€â”€â”€ checked_pow â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn checked_pow_normal_succeeds_d18() {
        let two = D18::<0>::from(2);
        assert_eq!(two.checked_pow(10), Some(D18::<0>::from(1024)));
        assert_eq!(two.checked_pow(0), Some(D18::<0>::ONE));
    }

    #[test]
    fn checked_pow_overflow_returns_none_d18() {
        let ten = D18::<0>::from(10);
        assert!(ten.checked_pow(40).is_none(), "10^40 overflows D18<0>");
    }

    #[test]
    fn checked_pow_overflow_returns_none_d38() {
        let ten = D38::<0>::from(10);
        assert!(ten.checked_pow(80).is_none(), "10^80 overflows D38<0>");
    }

    // â”€â”€â”€ wrapping_pow â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn wrapping_pow_matches_arithmetic_d18() {
        let two = D18::<0>::from(2);
        // 2^10 == 1024, well within range.
        assert_eq!(two.wrapping_pow(10).to_bits(), 1024);
        // 2^63 wraps in i64: (2 as i64).wrapping_pow(63) == i64::MIN.
        assert_eq!(two.wrapping_pow(63).to_bits(), 2i64.wrapping_pow(63));
        // 2^64 wraps to zero (high bit only, drops).
        assert_eq!(two.wrapping_pow(64).to_bits(), 2i64.wrapping_pow(64));
    }

    #[test]
    fn wrapping_pow_exp_zero_returns_one() {
        let v = D38::<0>::from(123);
        assert_eq!(v.wrapping_pow(0), D38::<0>::ONE);
    }

    // â”€â”€â”€ saturating_pow â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn saturating_pow_positive_overflow_saturates_to_max() {
        let ten = D18::<0>::from(10);
        assert_eq!(ten.saturating_pow(40), D18::<0>::MAX);
        let ten = D38::<0>::from(10);
        assert_eq!(ten.saturating_pow(80), D38::<0>::MAX);
    }

    #[test]
    fn saturating_pow_negative_odd_saturates_to_min() {
        let neg_ten = D18::<0>::from(-10);
        assert_eq!(neg_ten.saturating_pow(41), D18::<0>::MIN);
        let neg_ten = D38::<0>::from(-10);
        assert_eq!(neg_ten.saturating_pow(81), D38::<0>::MIN);
    }

    #[test]
    fn saturating_pow_negative_even_saturates_to_max() {
        // negative base raised to an even power is positive, so the
        // saturation direction is MAX, not MIN.
        let neg_ten = D18::<0>::from(-10);
        assert_eq!(neg_ten.saturating_pow(20), D18::<0>::MAX);
    }

    #[test]
    fn saturating_pow_exp_zero_returns_one() {
        let v = D38::<0>::from(123);
        assert_eq!(v.saturating_pow(0), D38::<0>::ONE);
    }

    // â”€â”€â”€ overflowing_pow â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn overflowing_pow_no_overflow_returns_false() {
        let two = D18::<0>::from(2);
        let (v, ov) = two.overflowing_pow(10);
        assert_eq!(v.to_bits(), 1024);
        assert!(!ov);
    }

    #[test]
    fn overflowing_pow_detects_overflow_d18_d38() {
        let ten18 = D18::<0>::from(10);
        let (v18, ov18) = ten18.overflowing_pow(40);
        assert!(ov18);
        assert_eq!(v18, ten18.wrapping_pow(40));

        let ten38 = D38::<0>::from(10);
        let (v38, ov38) = ten38.overflowing_pow(80);
        assert!(ov38);
        assert_eq!(v38, ten38.wrapping_pow(80));
    }

    #[test]
    fn overflowing_pow_exp_zero_returns_one_no_overflow() {
        let v = D38::<0>::from(123);
        assert_eq!(v.overflowing_pow(0), (D38::<0>::ONE, false));
    }

    // â”€â”€â”€ Wide-tier sanity (gated) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[cfg(feature = "wide")]
    #[test]
    fn pow_d76() {
        use decimal_scaled::D76;

        let two: D76<0> = D38::<0>::from(2).into();
        let r = two.pow(10);
        let expected: D76<0> = D38::<0>::from(1024).into();
        assert_eq!(r, expected);

        // exp=0 â†’ ONE
        assert_eq!(two.pow(0), D76::<0>::ONE);
        // exp=1 â†’ self
        assert_eq!(two.pow(1), two);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn pow_d76_negative_base_odd_even() {
        use decimal_scaled::D76;

        let neg_two: D76<0> = D38::<0>::from(-2).into();
        let expected_pos: D76<0> = D38::<0>::from(16).into();
        let expected_neg: D76<0> = D38::<0>::from(-8).into();
        assert_eq!(neg_two.pow(4), expected_pos);
        assert_eq!(neg_two.pow(3), expected_neg);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn checked_pow_overflow_d76() {
        use decimal_scaled::D76;

        let ten: D76<0> = D38::<0>::from(10).into();
        // D76<0> max â‰ˆ 10^76. 10^80 overflows.
        assert!(ten.checked_pow(80).is_none());
        // 10^20 fits comfortably.
        assert!(ten.checked_pow(20).is_some());
    }

    #[cfg(feature = "wide")]
    #[test]
    fn saturating_overflowing_pow_d76() {
        use decimal_scaled::D76;

        let ten: D76<0> = D38::<0>::from(10).into();
        assert_eq!(ten.saturating_pow(80), D76::<0>::MAX);
        let (_, ov) = ten.overflowing_pow(80);
        assert!(ov);
        let neg_ten: D76<0> = D38::<0>::from(-10).into();
        assert_eq!(neg_ten.saturating_pow(81), D76::<0>::MIN);
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn pow_d153_d307() {
        use decimal_scaled::{D153, D307};

        let two_a: D153<0> = D38::<0>::from(2).into();
        assert_eq!(two_a.pow(0), D153::<0>::ONE);
        let two_b: D307<0> = D307::<0>::from(2);
        assert_eq!(two_b.pow(0), D307::<0>::ONE);
        // small exponent
        let r_a = two_a.pow(8);
        let expected_a: D153<0> = D38::<0>::from(256).into();
        assert_eq!(r_a, expected_a);
    }
}

mod from_src_powers {
    use decimal_scaled::{D38s12, Int};

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

    // Tolerance for f64-bridge tests.
    const TWO_LSB: i128 = 2;

    fn within_lsb(actual: D38s12, expected: D38s12, lsb: i128) -> bool {
        let diff = (actual.to_bits().as_i128() - expected.to_bits().as_i128()).abs();
        diff <= lsb
    }

    // pow (integer)

    /// `pow(2)` equals `self * self` for an integer value.
    #[test]
    fn pow_two_matches_mul() {
        let v = D38s12::from(13);
        assert_eq!(v.pow(2), v * v);
    }

    /// `pow(2)` equals `self * self` for a fractional value.
    #[test]
    fn pow_two_matches_mul_fractional() {
        // 1.5 in raw bits at SCALE = 12.
        let v = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        assert_eq!(v.pow(2), v * v);
    }

    /// `2^10 == 1024`.
    #[test]
    fn pow_two_to_the_ten() {
        let two = D38s12::from(2);
        assert_eq!(two.pow(10), D38s12::from(1024));
    }

    /// `pow(0, n)` for `n > 0` is `ZERO`.
    #[test]
    fn zero_pow_positive_is_zero() {
        assert_eq!(D38s12::ZERO.pow(1), D38s12::ZERO);
        assert_eq!(D38s12::ZERO.pow(5), D38s12::ZERO);
    }

    /// `pow(n)` of `ONE` is always `ONE`.
    #[test]
    fn one_pow_n_is_one() {
        assert_eq!(D38s12::ONE.pow(0), D38s12::ONE);
        assert_eq!(D38s12::ONE.pow(1), D38s12::ONE);
        assert_eq!(D38s12::ONE.pow(100), D38s12::ONE);
    }

    /// `(-1)^n` alternates sign.
    #[test]
    fn negative_one_pow_alternates() {
        let neg_one = -D38s12::ONE;
        assert_eq!(neg_one.pow(0), D38s12::ONE);
        assert_eq!(neg_one.pow(1), neg_one);
        assert_eq!(neg_one.pow(2), D38s12::ONE);
        assert_eq!(neg_one.pow(3), neg_one);
    }

    // powi (signed integer)

    /// `powi(-1)` returns `ONE / self`.
    #[test]
    fn powi_minus_one_is_reciprocal() {
        let v = D38s12::from(7);
        assert_eq!(v.powi(-1), D38s12::ONE / v);
    }

    /// `powi` agrees with `pow` for non-negative exponents.
    #[test]
    fn powi_positive_matches_pow() {
        let v = D38s12::from(3);
        for e in 0_i32..6 {
            assert_eq!(v.powi(e), v.pow(e as u32));
        }
    }

    // powf â€” requires std feature

    /// `powf(0.5)` approximates `sqrt` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn powf_half_matches_sqrt() {
        let v = D38s12::from(4);
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap()); // 0.5 at SCALE=12
        let powf_result = v.powf(half);
        let sqrt_result = v.sqrt();
        assert!(
            within_lsb(powf_result, sqrt_result, TWO_LSB),
            "powf(0.5)={}, sqrt={}, diff={}",
            powf_result.to_bits().as_i128(),
            sqrt_result.to_bits().as_i128(),
            (powf_result.to_bits().as_i128() - sqrt_result.to_bits().as_i128()).abs(),
        );
    }

    /// `powf(2)` agrees with `pow(2)` within 2 LSB (f64 bridge).
    #[cfg(all(feature = "fast", not(feature = "strict")))]
    #[test]
    fn powf_two_matches_pow_two_within_lsb() {
        let v = D38s12::from(7);
        let two = D38s12::from(2);
        assert!(within_lsb(v.powf(two), v.pow(2), TWO_LSB));
    }

    /// Strict `powf` is correctly rounded: `powf(7, 2)` agrees with the
    /// exact `pow(7, 2)` to within 1 ULP â€” the whole `exp(yÂ·ln(x))`
    /// chain runs in the shared wide guard-digit intermediate.
    #[cfg(any(not(feature = "fast"), feature = "strict"))]
    #[test]
    fn powf_two_matches_pow_two_within_lsb() {
        let v = D38s12::from(7);
        let two = D38s12::from(2);
        assert!(within_lsb(v.powf(two), v.pow(2), 1));
        // A few more integer-exponent cross-checks against exact `pow`.
        for base in [2_i64, 3, 5, 11] {
            let b = D38s12::from(base);
            assert!(
                within_lsb(b.powf(D38s12::from(3)), b.pow(3), 1),
                "powf({base}, 3)"
            );
        }
    }

    // sqrt â€” requires std feature

    /// `sqrt(0) == 0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sqrt(), D38s12::ZERO);
    }

    /// `sqrt(1) == 1` â€” bit-exact because `f64::sqrt(1.0) == 1.0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_one_is_one_bit_exact() {
        assert_eq!(D38s12::ONE.sqrt(), D38s12::ONE);
    }

    /// `sqrt(4) == 2` â€” bit-exact because `f64::sqrt(4.0) == 2.0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_four_is_two() {
        let four = D38s12::from(4);
        let two = D38s12::from(2);
        assert_eq!(four.sqrt(), two);
    }

    /// `sqrt(self * self) ~= self.abs()` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_of_square_recovers_abs() {
        let v = D38s12::from_bits(Int::<2>::try_from(1_234_567_890_123_i128).unwrap());
        let squared = v * v;
        let recovered = squared.sqrt();
        let abs_v = v.abs();
        assert!(
            within_lsb(recovered, abs_v, TWO_LSB),
            "sqrt({})={}, expected~={}, diff={}",
            squared.to_bits().as_i128(),
            recovered.to_bits().as_i128(),
            abs_v.to_bits().as_i128(),
            (recovered.to_bits().as_i128() - abs_v.to_bits().as_i128()).abs(),
        );
    }

    /// `sqrt(self * self) ~= self.abs()` within 2 LSB for negative self.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_of_square_negative_recovers_abs() {
        let v = -D38s12::from_bits(Int::<2>::try_from(4_567_891_234_567_i128).unwrap());
        let squared = v * v;
        let recovered = squared.sqrt();
        let abs_v = v.abs();
        assert!(within_lsb(recovered, abs_v, TWO_LSB));
    }

    /// A negative input produces NaN in f64, which maps to ZERO.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_negative_saturates_to_zero() {
        let v = -D38s12::from(4);
        assert_eq!(v.sqrt(), D38s12::ZERO);
    }

    // cbrt â€” requires std feature

    /// `cbrt(0) == 0`.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_zero_is_zero() {
        assert_eq!(D38s12::ZERO.cbrt(), D38s12::ZERO);
    }

    /// `cbrt(1) == 1`.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_one_is_one() {
        assert_eq!(D38s12::ONE.cbrt(), D38s12::ONE);
    }

    /// `cbrt(8) ~= 2` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_eight_is_two() {
        let eight = D38s12::from(8);
        let two = D38s12::from(2);
        assert!(within_lsb(eight.cbrt(), two, TWO_LSB));
    }

    /// `cbrt(-8) ~= -2` within 2 LSB.
    #[cfg(feature = "std")]
    #[test]
    fn cbrt_minus_eight_is_minus_two() {
        let neg_eight = D38s12::from(-8);
        let neg_two = D38s12::from(-2);
        assert!(
            within_lsb(neg_eight.cbrt(), neg_two, TWO_LSB),
            "cbrt(-8) = {}, expected ~ {}",
            neg_eight.cbrt().to_bits().as_i128(),
            neg_two.to_bits().as_i128(),
        );
    }

    // checked_pow / wrapping_pow / saturating_pow / overflowing_pow

    /// `pow(2) == self * self` for several representative raw values.
    #[test]
    fn pow_two_property_safe_values() {
        for raw in [
            1_234_567_890_123_i128,
            4_567_891_234_567_i128,
            7_890_123_456_789_i128,
        ] {
            let v = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
            assert_eq!(v.pow(2), v * v, "raw bits {raw}");
        }
    }

    // mul_add (always available)

    /// `mul_add(0, 0, 0) == 0`.
    #[test]
    fn mul_add_zero_zero_zero_is_zero() {
        let z = D38s12::ZERO;
        assert_eq!(z.mul_add(z, z), D38s12::ZERO);
    }

    /// `mul_add(2, 3, 4) == 10`.
    #[test]
    fn mul_add_two_three_four_is_ten() {
        let two = D38s12::from(2);
        let three = D38s12::from(3);
        let four = D38s12::from(4);
        assert_eq!(two.mul_add(three, four), D38s12::from(10));
    }

    /// `mul_add(self, ONE, ZERO) == self`.
    #[test]
    fn mul_add_identity_collapses() {
        let v = D38s12::from(7);
        assert_eq!(v.mul_add(D38s12::ONE, D38s12::ZERO), v);
    }

    /// `mul_add(self, ZERO, b) == b`.
    #[test]
    fn mul_add_zero_factor_yields_addend() {
        let v = D38s12::from(7);
        let b = D38s12::from(13);
        assert_eq!(v.mul_add(D38s12::ZERO, b), b);
    }

    /// `mul_add(a, b, c) == a * b + c` for representative raw values.
    #[test]
    fn mul_add_matches_mul_then_add_safe_values() {
        for (a_raw, b_raw, c_raw) in [
            (
                1_234_567_890_123_i128,
                2_345_678_901_234_i128,
                3_456_789_012_345_i128,
            ),
            (
                4_567_891_234_567_i128,
                5_678_912_345_678_i128,
                6_789_123_456_789_i128,
            ),
            (
                7_890_123_456_789_i128,
                8_901_234_567_891_i128,
                9_012_345_678_912_i128,
            ),
        ] {
            let a = D38s12::from_bits(Int::<2>::try_from(a_raw).unwrap());
            let b = D38s12::from_bits(Int::<2>::try_from(b_raw).unwrap());
            let c = D38s12::from_bits(Int::<2>::try_from(c_raw).unwrap());
            assert_eq!(
                a.mul_add(b, c),
                a * b + c,
                "raw bits ({a_raw}, {b_raw}, {c_raw})",
            );
        }
    }

    /// `(-a).mul_add(b, c)` propagates the sign through the multiply step.
    #[test]
    fn mul_add_sign_propagates_through_factor() {
        let a = D38s12::from(3);
        let b = D38s12::from(5);
        let c = D38s12::from(7);
        // (-3) * 5 + 7 = -15 + 7 = -8
        assert_eq!((-a).mul_add(b, c), D38s12::from(-8));
    }

    // hypot â€” requires std feature

    // Tolerance for hypot: composes divide + square + add + sqrt + multiply,
    // each with up to 1 LSB of f64-bridge slack; sqrt quantisation dominates.
    #[cfg(feature = "std")]
    const HYPOT_TOLERANCE_LSB: i128 = 32;

    /// `hypot(3, 4) ~= 5` â€” the Pythagorean triple.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_three_four_is_five() {
        let three = D38s12::from(3);
        let four = D38s12::from(4);
        let five = D38s12::from(5);
        let result = three.hypot(four);
        assert!(
            within_lsb(result, five, HYPOT_TOLERANCE_LSB),
            "hypot(3, 4)={}, expected~={}, diff={}",
            result.to_bits().as_i128(),
            five.to_bits().as_i128(),
            (result.to_bits().as_i128() - five.to_bits().as_i128()).abs(),
        );
    }

    /// `hypot(0, 0) == 0` â€” bit-exact via the early-return path.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_zero_zero_is_zero_bit_exact() {
        assert_eq!(D38s12::ZERO.hypot(D38s12::ZERO), D38s12::ZERO);
    }

    /// `hypot(0, x) ~= x.abs()` for nonzero x.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_zero_x_is_abs_x() {
        let x = D38s12::from(7);
        let result = D38s12::ZERO.hypot(x);
        assert!(
            within_lsb(result, x.abs(), HYPOT_TOLERANCE_LSB),
            "hypot(0, 7)={}, expected~={}",
            result.to_bits().as_i128(),
            x.abs().to_bits().as_i128(),
        );
    }

    /// `hypot(x, 0) ~= x.abs()` for nonzero x.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_x_zero_is_abs_x() {
        let x = D38s12::from(-9);
        let result = x.hypot(D38s12::ZERO);
        assert!(
            within_lsb(result, x.abs(), HYPOT_TOLERANCE_LSB),
            "hypot(-9, 0)={}, expected~={}",
            result.to_bits().as_i128(),
            x.abs().to_bits().as_i128(),
        );
    }

    /// `hypot(-a, b) == hypot(a, b)` â€” sign invariance from the abs step.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_sign_invariant() {
        let three = D38s12::from(3);
        let four = D38s12::from(4);
        let pos = three.hypot(four);
        let neg_a = (-three).hypot(four);
        let neg_b = three.hypot(-four);
        let neg_both = (-three).hypot(-four);
        assert_eq!(pos, neg_a);
        assert_eq!(pos, neg_b);
        assert_eq!(pos, neg_both);
    }

    /// `hypot` does not panic at large magnitudes that the naive form
    /// would overflow.
    ///
    /// At SCALE=12 with `i128::MAX / 2` raw bits, the true hypotenuse
    /// is well below `D38::MAX / sqrt(2)`, so no overflow occurs and
    /// the result is a nonzero positive value.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_large_magnitudes_does_not_panic() {
        let half_max = D38s12::from_bits(Int::<2>::try_from(i128::MAX / 2).unwrap());
        let result = half_max.hypot(half_max);
        assert!(result > D38s12::ZERO);
        assert!(result >= half_max);
    }

    /// `hypot(a, b)` matches the naive `sqrt(a^2 + b^2)` within tolerance
    /// for small magnitudes where the naive form does not overflow.
    #[cfg(feature = "std")]
    #[test]
    fn hypot_matches_naive_sqrt_of_sum_of_squares() {
        let a = D38s12::from(12);
        let b = D38s12::from(13);
        let h = a.hypot(b);
        let naive = (a * a + b * b).sqrt();
        assert!(
            within_lsb(h, naive, HYPOT_TOLERANCE_LSB),
            "hypot(12, 13)={}, naive sqrt(a^2+b^2)={}, diff={}",
            h.to_bits().as_i128(),
            naive.to_bits().as_i128(),
            (h.to_bits().as_i128() - naive.to_bits().as_i128()).abs(),
        );
    }

}
