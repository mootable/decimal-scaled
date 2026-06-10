//! `pow` / `powi` / `powf`, roots, `mul_add` and `hypot` behaviour
//! tests moved from `src/types/powers.rs`. Deduped against
//! `macros_pow.rs`: the shared pow-contract restatements
//! (pow(0)=ONE, pow(1)=self, the checked/wrapping/saturating/
//! overflowing family) live there; only the unique cases moved.
//! The two correctly-rounded sqrt/cbrt witnesses stay in src
//! (they need crate-private 256-bit helpers / the root-only
//! `i256` dev-dependency).

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

    // powf — requires std feature

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
    /// exact `pow(7, 2)` to within 1 ULP — the whole `exp(y·ln(x))`
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

    // sqrt — requires std feature

    /// `sqrt(0) == 0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_zero_is_zero() {
        assert_eq!(D38s12::ZERO.sqrt(), D38s12::ZERO);
    }

    /// `sqrt(1) == 1` — bit-exact because `f64::sqrt(1.0) == 1.0`.
    #[cfg(feature = "std")]
    #[test]
    fn sqrt_one_is_one_bit_exact() {
        assert_eq!(D38s12::ONE.sqrt(), D38s12::ONE);
    }

    /// `sqrt(4) == 2` — bit-exact because `f64::sqrt(4.0) == 2.0`.
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

    // cbrt — requires std feature

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

    // hypot — requires std feature

    // Tolerance for hypot: composes divide + square + add + sqrt + multiply,
    // each with up to 1 LSB of f64-bridge slack; sqrt quantisation dominates.
    #[cfg(feature = "std")]
    const HYPOT_TOLERANCE_LSB: i128 = 32;

    /// `hypot(3, 4) ~= 5` — the Pythagorean triple.
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

    /// `hypot(0, 0) == 0` — bit-exact via the early-return path.
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

    /// `hypot(-a, b) == hypot(a, b)` — sign invariance from the abs step.
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
