//! The overflow / divide-by-zero panic and overflow-variant contract on the
//! arithmetic surface: default ops panic in BOTH debug and release; the
//! checked_ / wrapping_ / saturating_ / overflowing_ family is the opt-out.
//! The tier-invariant transcendental overflow contract
//! (`transcendental_overflow_uniform.rs`, `hypot_edge_cases.rs`) and the
//! in-src `src/types/overflow_variants.rs` tests join this target in later
//! batches.

mod from_arithmetic_mode_aware {
    //! The panic-contract tests of the retired `tests/arithmetic_mode_aware.rs`;
    //! its mode-delegation tests live in `contracts/mode_delegation.rs`.

    use decimal_scaled::{D38s12, RoundingMode};

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_with_zero_panics() {
        let _ = D38s12::ONE.div_with(D38s12::ZERO, RoundingMode::HalfToEven);
    }
    // â”€â”€â”€ Overflow panic paths (panic in BOTH debug and release) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    // `mul_with` / `div_with` share the same overflow contract as the plain
    // `*` / `/` operators: panic on overflow in both debug and release. The
    // mode argument influences only the rounding step, not the overflow policy.

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn mul_with_overflow_panics() {
        let a = decimal_scaled::D38::<0>::MAX;
        let _ = a.mul_with(a, RoundingMode::HalfToEven);
    }

    #[test]
    #[should_panic(expected = "attempt to divide with overflow")]
    fn div_with_overflow_panics() {
        use decimal_scaled::D38;
        let a = D38::<0>::MIN;
        let _ = a.div_with(D38::<0>::from(-1), RoundingMode::HalfToEven);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn mul_overflow_panics() {
        // Choose operands such that the mg_divide path returns None and the
        // overflow panic fires. D38<0>::MAX * D38<0>::MAX overflows.
        use decimal_scaled::D38;
        let a = D38::<0>::MAX;
        let _ = a * a;
    }

    #[test]
    #[should_panic(expected = "attempt to divide with overflow")]
    fn div_overflow_panics() {
        // D38<0>::MIN / -1 overflows the i128 quotient.
        use decimal_scaled::D38;
        let a = D38::<0>::MIN;
        let _ = a / D38::<0>::from(-1);
    }
}

mod from_macros_surface {
    //! The overflow-variant blocks of the retired `tests/macros_surface.rs`.

    use decimal_scaled::{D18, D38};

    // â”€â”€â”€ macros/overflow.rs: checked_*, wrapping_*, saturating_*, overflowing_* â”€

    #[test]
    fn overflow_variants_add_d18() {
        let a = D18::<2>::MAX;
        let b = D18::<2>::from(1);
        assert!(a.checked_add(b).is_none());
        assert_eq!(a.saturating_add(b), D18::<2>::MAX);
        let (_, ov) = a.overflowing_add(b);
        assert!(ov);
        // wrapping (just exercises the branch)
        let _ = a.wrapping_add(b);
    }

    #[test]
    fn overflow_variants_sub_d18() {
        let a = D18::<2>::MIN;
        let b = D18::<2>::from(1);
        assert!(a.checked_sub(b).is_none());
        assert_eq!(a.saturating_sub(b), D18::<2>::MIN);
        let (_, ov) = a.overflowing_sub(b);
        assert!(ov);
        let _ = a.wrapping_sub(b);
    }

    #[test]
    fn overflow_variants_neg_d18() {
        // MIN is not representable as positive; negating overflows.
        assert!(D18::<2>::MIN.checked_neg().is_none());
        assert_eq!(D18::<2>::MIN.saturating_neg(), D18::<2>::MAX);
        let (_, ov) = D18::<2>::MIN.overflowing_neg();
        assert!(ov);
        let _ = D18::<2>::MIN.wrapping_neg();
        // Non-MIN case
        assert_eq!(D18::<2>::from(5).checked_neg().unwrap(), D18::<2>::from(-5));
    }

    #[test]
    fn overflow_variants_mul_d18() {
        let a = D18::<2>::MAX;
        let b = D18::<2>::from(2);
        assert!(a.checked_mul(b).is_none());
        assert_eq!(a.saturating_mul(b), D18::<2>::MAX);
        let (_, ov) = a.overflowing_mul(b);
        assert!(ov);
        let _ = a.wrapping_mul(b);

        // Negative * positive saturating goes to MIN
        let neg_max = D18::<2>::MIN;
        let big = D18::<2>::from(3);
        assert_eq!(neg_max.saturating_mul(big), D18::<2>::MIN);
    }

    #[test]
    fn overflow_variants_div_rem_d18() {
        let a = D18::<2>::from(7);
        let b = D18::<2>::from(2);
        let q = a.checked_div(b).unwrap();
        // 7.00 / 2.00 = 3.50 â†’ 350 at S=2.
        assert_eq!(q.to_bits(), 350);
        // div by zero â€” checked_div returns None (matches i32::checked_div);
        // saturating_div by zero panics (covered by saturating_div_by_zero_panics).
        assert!(D18::<2>::from(7).checked_div(D18::<2>::ZERO).is_none());
        // overflowing_div(0) / wrapping_div(0) / wrapping_rem(0) on most
        // integer storage types panic (matches i32::overflowing_div), so we
        // don't exercise those paths here â€” they're well-known and the
        // checked_/saturating_ surface is what callers should use.
        // rem
        let r = a.checked_rem(b).unwrap();
        let _ = r;
        assert!(D18::<2>::from(7).checked_rem(D18::<2>::ZERO).is_none());
    }

    #[cfg(feature = "wide")]
    #[test]
    fn overflow_variants_wide_d76() {
        use decimal_scaled::D76;

        let a = D76::<2>::MAX;
        let b: D76<2> = D38::<2>::from(2).into();
        assert!(a.checked_mul(b).is_none());
        assert_eq!(a.saturating_mul(b), D76::<2>::MAX);
        let _ = a.wrapping_mul(b);
        let (_, ov) = a.overflowing_mul(b);
        assert!(ov);
        // Add overflow
        let one: D76<2> = D38::<2>::from(1).into();
        assert!(D76::<2>::MAX.checked_add(one).is_none());
        // Sub overflow
        assert!(D76::<2>::MIN.checked_sub(one).is_none());
        // Neg overflow
        assert!(D76::<2>::MIN.checked_neg().is_none());
        // Div by zero
        assert!(D76::<2>::MAX.checked_div(D76::<2>::ZERO).is_none());
        assert!(D76::<2>::MAX.checked_rem(D76::<2>::ZERO).is_none());
    }
    // â”€â”€â”€ arithmetic.rs (D38 overflow paths via mg_divide) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    //
    // D38 mul/div go through mg_divide; the wrapping/saturating paths in
    // arithmetic.rs are reachable via these operators. Overflow at the MAX
    // boundary exercises them.

    #[test]
    fn d38_mul_overflow_wraps_in_release() {
        // In debug mode this would panic; in release mode it wraps.
        // The library deliberately mirrors i128 semantics, so we test the
        // checked_mul path which is always defined.
        let a = D38::<2>::MAX;
        let b = D38::<2>::from(2);
        assert!(a.checked_mul(b).is_none());
        let (_v, ov) = a.overflowing_mul(b);
        assert!(ov);
        assert_eq!(a.saturating_mul(b), D38::<2>::MAX);
    }

    #[test]
    fn d38_div_by_zero_overflow() {
        assert!(D38::<2>::ONE.checked_div(D38::<2>::ZERO).is_none());
        // overflowing_div(0) panics on most storage types (matches
        // i128::overflowing_div); checked_div is the safe surface.
    }

    #[test]
    fn d38_add_sub_overflow() {
        assert!(D38::<2>::MAX.checked_add(D38::<2>::ONE).is_none());
        assert!(D38::<2>::MIN.checked_sub(D38::<2>::ONE).is_none());
        let (_, ov) = D38::<2>::MAX.overflowing_add(D38::<2>::ONE);
        assert!(ov);
        let (_, ov) = D38::<2>::MIN.overflowing_sub(D38::<2>::ONE);
        assert!(ov);
    }
}

mod from_macros_bitwise_and_overflow {
    //! The overflow-variant blocks of the retired
    //! `tests/macros_bitwise_and_overflow.rs`.

    use decimal_scaled::D18;
    // D38 only feeds the wide-tier lift fixtures below.
    #[cfg(feature = "wide")]
    use decimal_scaled::D38;

    // â”€â”€â”€ overflow: wrapping_div / wrapping_rem (non-zero rhs path) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    //
    // `wrapping_div(0)` panics, but `wrapping_div(non_zero)` is a normal
    // path the high-level tests did not hit at the macro-emitted body. We
    // hit it here with rhs != 0.

    #[test]
    fn wrapping_div_rem_non_zero() {
        // narrow widths
        let a = D18::<2>::from(7);
        let b = D18::<2>::from(2);
        let q = a.wrapping_div(b);
        // 7.00 / 2.00 = 3.50 â†’ 350 at S=2
        assert_eq!(q.to_bits(), 350);
        let r = a.wrapping_rem(b);
        let _ = r;

        let a = D18::<2>::from(7);
        let b = D18::<2>::from(2);
        let _ = a.wrapping_div(b);
        let _ = a.wrapping_rem(b);

        // Wide
        #[cfg(feature = "wide")]
        {
            use decimal_scaled::D76;

            let a: D76<2> = D38::<2>::from(7).into();
            let b: D76<2> = D38::<2>::from(2).into();
            let q = a.wrapping_div(b);
            let expected: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(350_i128).unwrap()).into();
            assert_eq!(q, expected);
            let _ = a.wrapping_rem(b);
        }
    }

    // â”€â”€â”€ overflow: saturating_div sign-aware â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[cfg(feature = "wide")]
    #[test]
    fn wide_overflow_variants_success_cases() {
        use decimal_scaled::D76;

        let a: D76<2> = D38::<2>::from(7).into();
        let b: D76<2> = D38::<2>::from(2).into();
        // saturating_mul success path
        let _ = a.saturating_mul(b);
        // saturating_div success path
        let _ = a.saturating_div(b);
        // overflowing_div success path
        let (_, ov) = a.overflowing_div(b);
        assert!(!ov);
        // saturating_div by zero panics (matches the primitive contract);
        // that path is covered by `saturating_div_by_zero_panics`, so this
        // success-cases test does not exercise it.
        // overflowing_div with non-zero rhs (success)
        let (q, ov) = a.overflowing_div(b);
        assert!(!ov);
        assert_eq!(q, a / b);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn wide_checked_div_quotient_overflow() {
        // Engineer rhs so that the quotient exceeds wide storage. With
        // D76<74>, multiplier=10^74; q = (a*10^74)/b. If b is tiny and a is
        // near MAX, q can overflow.
        use decimal_scaled::D76;
        type D = D76<74>;
        let a = D::MAX;
        // 1.0 at S=74 is 10^74 â€” make rhs = 1 LSB so q = a * 10^74 / 1 â†’ way past storage.
        // Build a 1-LSB tiny value by lifting D38<74> from_bits(1).
        let tiny: D = D38::<74>::from_bits(decimal_scaled::Int::<2>::try_from(1_i128).unwrap()).into();
        let r = a.checked_div(tiny);
        assert!(r.is_none(), "tiny divisor on huge dividend should overflow");
    }

    #[test]
    fn saturating_div_overflow_signs() {
        // D18<2>::MIN / -ONE wraps because MIN's negation is unrepresentable.
        // saturating_div should clamp.
        let r = D18::<2>::MIN.saturating_div(-D18::<2>::ONE);
        assert!(r == D18::<2>::MIN || r == D18::<2>::MAX);
        let r = D18::<2>::MIN.saturating_div(-D18::<2>::ONE);
        assert!(r == D18::<2>::MIN || r == D18::<2>::MAX);
    }

    // â”€â”€â”€ overflow: overflowing_rem â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn overflowing_rem_non_zero_no_overflow() {
        let a = D18::<2>::from(7);
        let b = D18::<2>::from(2);
        let (_, ov) = a.overflowing_rem(b);
        assert!(!ov);
        let a = D18::<2>::from(7);
        let b = D18::<2>::from(2);
        let (_, ov) = a.overflowing_rem(b);
        assert!(!ov);

        #[cfg(feature = "wide")]
        {
            use decimal_scaled::D76;

            let a: D76<2> = D38::<2>::from(7).into();
            let b: D76<2> = D38::<2>::from(2).into();
            let (_, ov) = a.overflowing_rem(b);
            assert!(!ov);
        }
    }
}

#[allow(clippy::arithmetic_side_effects)]
mod from_src_overflow_variants {
    use decimal_scaled::{D38s12, Int};

    /// Returns `-ONE` as a convenience value.
    fn neg_one() -> D38s12 {
        -D38s12::ONE
    }

    /// Returns `2.0` in `D38s12` canonical form.
    fn two() -> D38s12 {
        D38s12::from_bits(Int::<2>::try_from(2_000_000_000_000_i128).unwrap())
    }

    /// Returns `3.0` in `D38s12` canonical form.
    fn three() -> D38s12 {
        D38s12::from_bits(Int::<2>::try_from(3_000_000_000_000_i128).unwrap())
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
        assert_eq!(D38s12::MAX.checked_add(D38s12::from_bits(Int::<2>::try_from(1_i128).unwrap())), None);
    }

    #[test]
    fn checked_add_negative_overflow_returns_none() {
        assert_eq!(D38s12::MIN.checked_add(neg_one()), None);
        // Boundary: MIN + (-1 LSB) also overflows.
        assert_eq!(D38s12::MIN.checked_add(D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap())), None);
    }

    #[test]
    fn wrapping_add_normal_matches_op() {
        assert_eq!(D38s12::ONE.wrapping_add(D38s12::ONE), two());
    }

    #[test]
    fn wrapping_add_overflow_wraps_to_min() {
        // MAX + 1 LSB wraps to MIN under two's-complement.
        assert_eq!(D38s12::MAX.wrapping_add(D38s12::from_bits(Int::<2>::try_from(1_i128).unwrap())), D38s12::MIN);
    }

    #[test]
    fn wrapping_add_negative_overflow_wraps_to_max() {
        // MIN + (-1 LSB) wraps to MAX.
        assert_eq!(D38s12::MIN.wrapping_add(D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap())), D38s12::MAX);
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
            D38s12::MAX.overflowing_add(D38s12::from_bits(Int::<2>::try_from(1_i128).unwrap())),
            (D38s12::MIN, true)
        );
    }

    #[test]
    fn overflowing_add_negative_overflow_flagged() {
        // MIN + (-1 LSB) wraps exactly to MAX.
        assert_eq!(
            D38s12::MIN.overflowing_add(D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap())),
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
        assert_eq!(D38s12::MIN.wrapping_sub(D38s12::from_bits(Int::<2>::try_from(1_i128).unwrap())), D38s12::MAX);
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
            D38s12::MIN.overflowing_sub(D38s12::from_bits(Int::<2>::try_from(1_i128).unwrap())),
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
        let neg_max = D38s12::from_bits(Int::<2>::try_from(-i128::MAX).unwrap());
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
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        let quarter = D38s12::from_bits(Int::<2>::try_from(250_000_000_000_i128).unwrap());
        assert_eq!(half.checked_mul(half), Some(quarter));
    }

    #[test]
    fn checked_mul_zero() {
        assert_eq!(D38s12::MAX.checked_mul(D38s12::ZERO), Some(D38s12::ZERO));
        assert_eq!(D38s12::ZERO.checked_mul(D38s12::ZERO), Some(D38s12::ZERO));
    }

    #[test]
    fn checked_mul_one_identity() {
        let v = D38s12::from_bits(Int::<2>::try_from(7_500_000_000_000_i128).unwrap()); // 7.5
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
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        let quarter = D38s12::from_bits(Int::<2>::try_from(250_000_000_000_i128).unwrap());
        assert_eq!(half.wrapping_mul(half), quarter);
    }

    #[test]
    fn wrapping_mul_overflow_does_not_panic() {
        // The intermediate product widens exactly; ONLY the final narrowing
        // wraps (two's-complement, like the primitives). Multiplying by 2.0
        // scales the raw bits by exactly 2 (the 10^SCALE factors cancel), so
        // the wrapped raw must equal the storage's own wrapping double.
        let w = D38s12::MAX.wrapping_mul(two());
        assert_eq!(i128::from(w.to_bits()), i128::MAX.wrapping_mul(2));
        // overflowing_mul must return the same wrapped value plus the flag.
        assert_eq!(D38s12::MAX.overflowing_mul(two()), (w, true));

        let w = D38s12::MIN.wrapping_mul(two());
        assert_eq!(i128::from(w.to_bits()), i128::MIN.wrapping_mul(2));
        assert_eq!(D38s12::MIN.overflowing_mul(two()), (w, true));
    }

    #[test]
    fn saturating_mul_normal() {
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        let quarter = D38s12::from_bits(Int::<2>::try_from(250_000_000_000_i128).unwrap());
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
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        let quarter = D38s12::from_bits(Int::<2>::try_from(250_000_000_000_i128).unwrap());
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
        let six = D38s12::from_bits(Int::<2>::try_from(6_000_000_000_000_i128).unwrap());
        assert_eq!(six.checked_div(two()), Some(three()));
    }

    #[test]
    fn checked_div_by_zero_returns_none() {
        assert_eq!(D38s12::ONE.checked_div(D38s12::ZERO), None);
    }

    #[test]
    fn checked_div_overflow_returns_none() {
        // MAX / 0.5 = 2 * MAX -> overflows the final quotient.
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        assert_eq!(D38s12::MAX.checked_div(half), None);
    }

    #[test]
    fn checked_div_negative_normal() {
        let neg_six = D38s12::from_bits(Int::<2>::try_from(-6_000_000_000_000_i128).unwrap());
        assert_eq!(neg_six.checked_div(two()), Some(-three()));
    }

    #[test]
    fn wrapping_div_normal() {
        let six = D38s12::from_bits(Int::<2>::try_from(6_000_000_000_000_i128).unwrap());
        assert_eq!(six.wrapping_div(two()), three());
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn wrapping_div_by_zero_panics() {
        let _ = D38s12::ONE.wrapping_div(D38s12::ZERO);
    }

    #[test]
    fn wrapping_div_overflow_does_not_panic() {
        // MAX / 0.5 = 2·MAX. The scale-up intermediate is exact in the wider
        // integer and the division is remainder-free (rounding never
        // engages); ONLY the final narrowing wraps two's-complement, so the
        // wrapped raw must equal the storage's own wrapping double.
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        let w = D38s12::MAX.wrapping_div(half);
        assert_eq!(i128::from(w.to_bits()), i128::MAX.wrapping_mul(2));
        // overflowing_div must return the same wrapped value plus the flag.
        assert_eq!(D38s12::MAX.overflowing_div(half), (w, true));
    }

    #[test]
    fn saturating_div_normal() {
        let six = D38s12::from_bits(Int::<2>::try_from(6_000_000_000_000_i128).unwrap());
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
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
        assert_eq!(D38s12::MAX.saturating_div(half), D38s12::MAX);
    }

    #[test]
    fn saturating_div_negative_overflow_clamps_to_min() {
        // MAX / -0.5 (mixed sign) saturates to MIN.
        let neg_half = D38s12::from_bits(Int::<2>::try_from(-500_000_000_000_i128).unwrap());
        assert_eq!(D38s12::MAX.saturating_div(neg_half), D38s12::MIN);
    }

    #[test]
    fn checked_and_wrapping_div_round_like_the_operator() {
        // 20 / 3 = 6.666â€¦ does not divide evenly; the checked/wrapping
        // variants must round to nearest using the crate-default mode,
        // identically to the `/` operator â€” not truncate toward zero.
        let twenty = D38s12::from_bits(Int::<2>::try_from(20_000_000_000_000_i128).unwrap());
        let three = D38s12::from_bits(Int::<2>::try_from(3_000_000_000_000_i128).unwrap());
        let rounded = twenty / three;
        let truncated = D38s12::from_bits(Int::<2>::try_from(6_666_666_666_666_i128).unwrap());
        assert_ne!(rounded, truncated, "the operator must round, not truncate");
        assert_eq!(twenty.checked_div(three), Some(rounded));
        assert_eq!(twenty.wrapping_div(three), rounded);
    }

    #[test]
    fn overflowing_div_normal() {
        let six = D38s12::from_bits(Int::<2>::try_from(6_000_000_000_000_i128).unwrap());
        assert_eq!(six.overflowing_div(two()), (three(), false));
    }

    #[test]
    fn overflowing_div_overflow_flagged() {
        let half = D38s12::from_bits(Int::<2>::try_from(500_000_000_000_i128).unwrap());
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
        let a = D38s12::from_bits(Int::<2>::try_from(5_500_000_000_000_i128).unwrap());
        let expected = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
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
        let neg_one_lsb = D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap());
        assert_eq!(D38s12::MIN.checked_rem(neg_one_lsb), None);
    }

    #[test]
    fn wrapping_rem_normal() {
        let a = D38s12::from_bits(Int::<2>::try_from(5_500_000_000_000_i128).unwrap());
        let expected = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
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
        let neg_one_lsb = D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap());
        assert_eq!(D38s12::MIN.wrapping_rem(neg_one_lsb), D38s12::ZERO);
    }

    #[test]
    fn overflowing_rem_normal() {
        let a = D38s12::from_bits(Int::<2>::try_from(5_500_000_000_000_i128).unwrap());
        let expected = D38s12::from_bits(Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        assert_eq!(a.overflowing_rem(two()), (expected, false));
    }

    #[test]
    fn overflowing_rem_min_neg_one_lsb_flagged() {
        let neg_one_lsb = D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap());
        assert_eq!(
            D38s12::MIN.overflowing_rem(neg_one_lsb),
            (D38s12::ZERO, true)
        );
    }

    // Cross-scale exercise

    /// Verifies that the variant family compiles and functions correctly at SCALE = 6.
    #[test]
    fn variants_at_scale_6() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        let one = D6::ONE;
        let two_d6 = D6::from_bits(Int::<2>::try_from(2_000_000_i128).unwrap()); // 2.0 at SCALE=6
        let one_lsb = D6::from_bits(Int::<2>::try_from(1_i128).unwrap());

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
        let a = D38s12::from_bits(Int::<2>::try_from(7_500_000_000_000_i128).unwrap()); // 7.5
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
        let neg_one_lsb = D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap());
        let (_, ovf) = D38s12::MIN.overflowing_rem(neg_one_lsb);
        assert_eq!(ovf, D38s12::MIN.checked_rem(neg_one_lsb).is_none());
    }

    /// Verifies that `saturating_add`, `saturating_sub`, and `saturating_mul`
    /// never panic and always return a value within `[MIN, MAX]`.
    #[test]
    fn saturating_never_escapes_bounds() {
        let extremes = [
            D38s12::MIN,
            D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap()),
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

#[cfg(all(feature = "wide", feature = "strict", not(feature = "fast")))]
mod from_hypot_edge_cases {
    //! Structural edge-case gate for `hypot_strict_with` — the non-value
    //! assertions the correctly-rounded golden set cannot express.
    //!
    //! The numeric VALUE coverage for `hypot` (Pythagorean triples, the
    //! non-perfect `√` cases, and the adversarial seam/band-edge inputs) now
    //! lives in the shared golden set (`decimal-scaled-golden/golden/hypot.golden`)
    //! and is checked, bit-exact across all six rounding modes, by
    //! `tests/ulp_strict_golden.rs` — the single source of truth. What
    //! remains here are the cases golden's `(input, floor, cls)` →
    //! `delta == 0` format cannot carry:
    //!
    //!   * `hypot(0, 0) = 0` bit-exact at every tier;
    //!   * `hypot(0, x) = |x|` (including the negative-operand mirror);
    //!   * the storage-overflow contract: `hypot(MAX, 0) = MAX` (fits, no
    //!     panic) versus `hypot(MAX, MAX) ≈ MAX·√2` (out of range → panic).
    //!
    //! A golden cell can only pin a representable result; the overflow case
    //! has no representable answer, so it is asserted here as a panic.

    use decimal_scaled::{RoundingMode, D18, D307, D38, D57};

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    #[test]
    fn hypot_zero_zero_is_zero_bit_exact_all_tiers_all_modes() {
        for mode in ALL_MODES {
            assert_eq!(
                D18::<6>::ZERO.hypot_strict_with(D18::<6>::ZERO, mode),
                D18::<6>::ZERO,
                "D18 hypot(0,0) mode {mode:?}",
            );
            assert_eq!(
                D38::<6>::ZERO.hypot_strict_with(D38::<6>::ZERO, mode),
                D38::<6>::ZERO,
                "D38 hypot(0,0) mode {mode:?}",
            );
            assert_eq!(
                D57::<6>::ZERO.hypot_strict_with(D57::<6>::ZERO, mode),
                D57::<6>::ZERO,
                "D57 hypot(0,0) mode {mode:?}",
            );
            assert_eq!(
                D307::<30>::ZERO.hypot_strict_with(D307::<30>::ZERO, mode),
                D307::<30>::ZERO,
                "D307 hypot(0,0) mode {mode:?}",
            );
        }
    }

    #[test]
    fn hypot_zero_x_is_abs_x_all_tiers_all_modes() {
        // hypot(0, x) = |x| exactly, and hypot(0, -x) = |x|.
        for &x in &[3i64, 7, 42, 100] {
            for mode in ALL_MODES {
                let d38 = D38::<6>::from(x);
                let d38n = D38::<6>::from(-x);
                assert_eq!(
                    D38::<6>::ZERO.hypot_strict_with(d38, mode),
                    d38,
                    "D38 hypot(0,{x}) mode {mode:?}",
                );
                assert_eq!(
                    D38::<6>::ZERO.hypot_strict_with(d38n, mode),
                    d38,
                    "D38 hypot(0,-{x}) mode {mode:?} (= |{x}|)",
                );
                let d307 = D307::<30>::from(x);
                assert_eq!(
                    D307::<30>::ZERO.hypot_strict_with(d307, mode),
                    d307,
                    "D307 hypot(0,{x}) mode {mode:?}",
                );
            }
        }
    }

    #[test]
    fn hypot_near_max_does_not_overflow_when_in_range() {
        // a near MAX, b = 0 -> hypot = a exactly, must not panic or overflow.
        let a = D38::<0>::MAX;
        assert_eq!(a.hypot_strict_with(D38::<0>::ZERO, RoundingMode::HalfToEven), a);
    }

    #[test]
    #[should_panic(expected = "hypot: result out of range")]
    fn hypot_overflow_panics_d38() {
        // a = b = MAX magnitude: hypot ~= MAX·√2 exceeds the type range.
        let m = D38::<0>::MAX;
        let _ = m.hypot_strict_with(m, RoundingMode::HalfToEven);
    }
}

#[cfg(all(not(feature = "fast"), not(any(feature = "rounding-half-away-from-zero", feature = "rounding-half-toward-zero", feature = "rounding-trunc", feature = "rounding-floor", feature = "rounding-ceiling"))))]
mod from_transcendental_overflow_uniform {
    //! Tier-invariant strict-transcendental overflow contract.
    //!
    //! `docs/ARCHITECTURE.md` → "Overflow & domain behaviour — one contract,
    //! invariant across tier and scale": a strict transcendental whose result
    //! is out of the storage range PANICS — identically at every width and
    //! scale, in BOTH debug and release. There is no ∞/NaN in a fixed-width
    //! decimal, so a wrapped/saturated value would be a silent wrong number;
    //! the default fails loudly instead.
    //!
    //! The historic bug this guards: a sufficiently large argument made the
    //! internal `exp` working-width arithmetic WRAP (`wrapping_sqr_low_u128`
    //! truncates to the low bits → an overflowed square collapses to 0), and
    //! the post-narrowing fit check — seeing only the small wrapped value —
    //! never fired, so `D18<0>::from(349).exp_strict()` returned `0` instead
    //! of panicking, while neighbouring arguments and the wide tiers panicked.
    //! `exp_generic::exp_fixed` now rejects such an argument up front, so the
    //! contract is uniform. Each `#[should_panic]` below fires in debug AND
    //! release because the panic route (`overflow_panic_with_scale` / the
    //! kernel's own `panic!`) is NOT gated behind `cfg!(debug_assertions)` —
    //! that gating is the separate i64-style ARITHMETIC contract, out of scope
    //! here.

    // The strict surface is the default build; skip under a non-default
    // rounding mode or the f64-bridge `fast` path.
    #![cfg(all(
        not(feature = "fast"),
        not(any(
            feature = "rounding-half-away-from-zero",
            feature = "rounding-half-toward-zero",
            feature = "rounding-trunc",
            feature = "rounding-floor",
            feature = "rounding-ceiling",
        )),
    ))]

    use decimal_scaled::{D18, D38};

    // ── Narrow tier (D18, i64 storage) ─────────────────────────────────────
    //
    // e^349 ≈ 10^151, far beyond i64. Pre-fix this RETURNED 0 (the internal
    // squaring wrapped the work integer to a value that survived the storage
    // fit check). Must panic, uniform with every other tier.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_exp_far_overflow_panics() {
        let _ = D18::<0>::from(349).exp_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_exp2_far_overflow_panics() {
        // 2^400 ≈ 2.6e120, far beyond i64. Pre-fix returned 0.
        let _ = D18::<0>::from(400).exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_cosh_far_overflow_panics() {
        let _ = D18::<0>::from(500).cosh_strict();
    }

    // ── Mid tier (D38, i128 storage) ───────────────────────────────────────
    //
    // e^349 ≈ 10^151, far beyond i128. Pre-fix this RETURNED 0.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp_far_overflow_panics() {
        let _ = D38::<0>::from(349).exp_strict();
    }

    // ── Wide tier (feature-gated) ──────────────────────────────────────────
    //
    // The wide tiers were the half that ALWAYS panicked (the contract the
    // narrow tier failed to match); the same far-out-of-range argument that
    // wrapped a wide work integer (e.g. D57 exp(1000) → 0 pre-fix) now panics
    // too, proving the fix closes the wrap on every tier — narrow AND wide.

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide {
        use decimal_scaled::D57;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d57_exp_far_overflow_panics() {
            // e^1000 ≈ 10^434, far beyond D57 storage AND its Wexp work integer.
            // Pre-fix this RETURNED 0 (the Wexp squaring wrapped).
            let _ = D57::<0>::from(1000).exp_strict();
        }

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d57_cosh_overflow_panics() {
            let _ = D57::<0>::from(140).cosh_strict();
        }
    }

    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    mod widest {
        use decimal_scaled::D1232;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn widest_d1232_cosh_far_overflow_panics() {
            // cosh(5000) ≈ e^5000/2 ≈ 10^2171, far beyond D1232 storage. Pre-fix
            // this RETURNED a wrapped multi-limb value.
            let _ = D1232::<0>::from(5000).cosh_strict();
        }
    }

    // ── D76 storage-narrow gap (peak-margin regression guard) ──────────────
    //
    // D76 is the SOLE tier whose exp work integer `Wexp == W == Int<16>`
    // (1024 bits): a result overflowing the work integer cannot lift any
    // wider, so `exp_generic`'s peak gate IS the last line for it. The peak
    // model's flat margin was over-large (+512 bits ≈ half D76's work budget)
    // and false-panicked IN-RANGE band-edge cosh/sinh at D76<0> (those
    // in-range cells are covered by `ulp_strict_golden` band_edges); the
    // margin is now a small +64 slack. This pins the COUNTERPART case the
    // smaller margin newly routes through the storage-narrowing path: a cosh
    // whose result overflows STORAGE but whose internal squaring peak still
    // fits Int<16> — cosh(180) ≈ 7e77 is 78 digits (> D76's 76-digit storage)
    // while its internal peak is ≈ 764 bits (< 1024). It must still PANIC,
    // caught by the narrowing fit check (`round_to_storage_with_g`), not the
    // work-int peak gate — keeping the out-of-range contract uniform.
    #[cfg(any(feature = "d76", feature = "wide"))]
    mod d76_gap {
        use decimal_scaled::D76;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn d76_cosh_storage_overflow_panics() {
            let _ = D76::<0>::from(180).cosh_strict();
        }
    }

    // ── In-range cells must still return a value (no false-positive panic) ──
    //
    // The guard must reject ONLY genuinely out-of-range results. These cells
    // sit just inside the storage edge and must compute normally.

    #[test]
    fn narrow_d18_exp_in_range_returns_value() {
        // e^43 ≈ 4.7e18 < i64::MAX ≈ 9.2e18.
        let _ = D18::<0>::from(43).exp_strict();
    }

    #[test]
    fn mid_d38_exp_in_range_returns_value() {
        // e^88 ≈ 1.65e38 < i128::MAX ≈ 1.70e38 — the last in-range integer arg.
        let _ = D38::<0>::from(88).exp_strict();
    }

    // ── exp2 deep-overflow band (fractional arguments) ─────────────────────
    //
    // Pre-fix, a FRACTIONAL deep-overflow exp2 argument (the integer ones are
    // caught by the exact-power pin) crashed in INTERNAL kernel machinery
    // instead of the contractual panic: the narrow kernel lifted its working
    // scale by the result's integer-digit count, so the kernel's own
    // `k = round(v/ln 2)` divide outgrew the build's divide scratch
    // (`div_knuth` index-out-of-bounds / `top < u.len()` assertion). The
    // analytic gate now proves the overflow from the result's integer-digit
    // count BEFORE any working-scale arithmetic: `2^x ≥ 10^(d-1)` carries
    // `d + scale` storage digits, and `i128` holds fewer than 40, so
    // `d + scale >= 40` panics up front — at every scale and width the
    // shared narrow kernel serves.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_exp2_deep_band_low_scale_panics() {
        // Pre-fix: index-out-of-bounds inside div_knuth.
        let v: D18<5> = "150.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn narrow_d18_exp2_deep_band_far_panics() {
        // Pre-fix: `top < u.len()` assertion inside div_knuth.
        let v: D18<5> = "400.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp2_deep_band_low_scale_panics() {
        // Scale 1 is the lowest scale with a fractional (non-pin) argument.
        let v: D38<1> = "200.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp2_deep_band_mid_scale_panics() {
        // Pre-fix: index-out-of-bounds inside div_knuth.
        let v: D38<10> = "150.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp2_deep_band_far_mid_scale_panics() {
        // Pre-fix: `top < u.len()` assertion inside div_knuth.
        let v: D38<10> = "400.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp2_deep_band_high_scale_panics() {
        // 2^20.5 ≈ 1.5e6: 7 integer digits + scale 35 ≥ 40 storage digits.
        let v: D38<35> = "20.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    #[test]
    fn mid_d38_exp2_near_edge_still_computes() {
        // 2^93.5 ≈ 1.4e28 at scale 10 is the last fractional in-range cell of
        // the probe grid; the gate must not touch it.
        let v: D38<10> = "93.5".parse().unwrap();
        assert_eq!(
            format!("{}", v.exp2_strict()),
            "14005692743696534979682984556.8645429406"
        );
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp2_first_out_of_range_cell_panics() {
        // 2^94.5 ≈ 2.8e28 · 10^10 just exceeds i128 — the clean-path edge.
        let v: D38<10> = "94.5".parse().unwrap();
        let _ = v.exp2_strict();
    }

    // ── exp2 deep-underflow band (negative fractional arguments) ───────────
    //
    // The mirror band: a deep NEGATIVE exp2 argument inflated the working
    // scale by the (meaningless for a sub-one result) integer-digit count of
    // `2^|x|` and crashed the same way — on an IN-RANGE cell whose result is
    // 0 (or 1 ULP under Ceiling). These must COMPUTE, never panic.

    #[test]
    fn mid_d38_exp2_deep_underflow_computes_zero() {
        // Pre-fix: `top < u.len()` assertion inside div_knuth.
        let v: D38<10> = "-150.5".parse().unwrap();
        assert_eq!(format!("{}", v.exp2_strict()), "0.0000000000");
    }

    #[test]
    fn mid_d38_exp2_deep_underflow_high_scale_computes() {
        // 2^-80.5 ≈ 5.9e-25 ≈ 0.59 ULP at scale 24 — rounds to 1 ULP under
        // nearest. Pre-fix: index-out-of-bounds inside div_knuth.
        let v: D38<24> = "-80.5".parse().unwrap();
        assert_eq!(
            format!("{}", v.exp2_strict()),
            "0.000000000000000000000001"
        );
    }

    #[test]
    fn mid_d38_exp2_deep_underflow_ceiling_is_one_ulp() {
        use decimal_scaled::RoundingMode;
        // A positive sub-resolution result must round UP under Ceiling.
        let v: D38<10> = "-150.5".parse().unwrap();
        assert_eq!(
            format!("{}", v.exp2_strict_with(RoundingMode::Ceiling)),
            "0.0000000001"
        );
    }

    // ── exp/sinh extreme band (argument magnitude beyond the work model) ───
    //
    // Pre-fix the extreme band was WORSE than a crash: `k = x/ln 2` exceeded
    // `i128`, truncated, and flipped sign — `exp(1.2e38)` and `sinh(1.2e38)`
    // SILENTLY returned 0 / a small wrapped value, and the peak model's
    // `|k|·30103` product overflowed `u128` (debug panic, release wrap). The
    // argument-magnitude pre-gate classifies these from the bit length alone.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp_extreme_band_panics() {
        // Pre-fix: debug `attempt to multiply with overflow` in the peak model.
        let v: D38<0> = "99999999999999999999999999999999999".parse().unwrap();
        let _ = v.exp_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_exp_max_arg_panics() {
        // Pre-fix: k wrapped negative through i128 and exp(MAX) returned 0.
        let _ = D38::<0>::MAX.exp_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_sinh_max_arg_panics() {
        // Pre-fix: returned a small wrapped value silently.
        let _ = D38::<0>::MAX.sinh_strict();
    }

    #[test]
    fn mid_d38_exp_extreme_negative_computes_zero() {
        // In-range deep underflow. Pre-fix: internal `div_u512_by_pow10`
        // invariant panic in the 256-bit kernel's range reduction.
        let v: D38<0> = "-150000000000000000000000000000000000000".parse().unwrap();
        assert_eq!(format!("{}", v.exp_strict()), "0");
    }

    // ── powf deep-overflow band ─────────────────────────────────────────────
    //
    // powf composes `exp(y·ln x)` on the 256-bit kernel with NO result-regime
    // routing, so a deep-overflow exponent reached the kernel's 2^k
    // reassembly assertion — loud, but not the contractual message — and an
    // extreme one could wrap the `k` shift narrowing. The analytic gate on
    // the composition argument (`arg ≥ (39−scale)·ln 10` proves the storage
    // overflow) fires first at every scale.

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_powf_deep_band_panics() {
        // 1.5^300.5 ≈ 10^52.9, far past i128 at scale 10.
        let b: D38<10> = "1.5".parse().unwrap();
        let e: D38<10> = "300.5".parse().unwrap();
        let _ = b.powf_strict(e);
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn mid_d38_powf_extreme_band_panics() {
        // y·ln x / ln 2 ≫ 2^32: the band where the pre-fix `k as u32` shift
        // narrowing could wrap.
        let b: D38<10> = "2.5".parse().unwrap();
        let e: D38<10> = "10000000000.5".parse().unwrap();
        let _ = b.powf_strict(e);
    }

    #[test]
    fn mid_d38_powf_near_edge_still_computes() {
        // 1.5^150.5 ≈ 10^26.5 fits scale 10 with room; the gate must not fire.
        let b: D38<10> = "1.5".parse().unwrap();
        let e: D38<10> = "150.5".parse().unwrap();
        let r = b.powf_strict(e);
        assert!(r > D38::<10>::from(0));
    }

    #[test]
    fn mid_d38_powf_deep_underflow_computes_zero() {
        let b: D38<10> = "0.5".parse().unwrap();
        let e: D38<10> = "10000000000.5".parse().unwrap();
        assert_eq!(format!("{}", b.powf_strict(e)), "0.0000000000");
    }

    // ── Wide-tier uniformity of the same bands ──────────────────────────────

    #[cfg(any(feature = "d76", feature = "wide"))]
    mod wide_deep_bands {
        use decimal_scaled::D76;

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d76_exp2_deep_band_panics() {
            // 2^300.5 ≈ 10^90.5 exceeds D76<10>'s 66 integer digits.
            let v: D76<10> = "300.5".parse().unwrap();
            let _ = v.exp2_strict();
        }

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d76_exp2_extreme_band_panics() {
            let v: D76<10> = "100000.5".parse().unwrap();
            let _ = v.exp2_strict();
        }

        #[test]
        fn wide_d76_exp_deep_negative_computes_zero() {
            // Regression guard for the peak-model pre-gate: a deep-underflow
            // argument must keep reporting "does not fit" so the cell takes the
            // wider lift (the ungated per-tier body's `k·ln2` formation cannot
            // carry it), landing on the canonical 0.
            let v: D76<10> = "-1000.5".parse().unwrap();
            assert_eq!(format!("{}", v.exp_strict()), "0.0000000000");
        }

        #[test]
        #[should_panic(expected = "result out of range")]
        fn wide_d76_powf_deep_band_panics() {
            let b: D76<10> = "1.5".parse().unwrap();
            let e: D76<10> = "1000.5".parse().unwrap();
            let _ = b.powf_strict(e);
        }
    }
}
