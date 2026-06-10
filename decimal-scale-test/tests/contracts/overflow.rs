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
    // ─── Overflow panic paths (panic in BOTH debug and release) ────────────

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

    // ─── macros/overflow.rs: checked_*, wrapping_*, saturating_*, overflowing_* ─

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
        // 7.00 / 2.00 = 3.50 → 350 at S=2.
        assert_eq!(q.to_bits(), 350);
        // div by zero — checked_div returns None (matches i32::checked_div);
        // saturating_div by zero panics (covered by saturating_div_by_zero_panics).
        assert!(D18::<2>::from(7).checked_div(D18::<2>::ZERO).is_none());
        // overflowing_div(0) / wrapping_div(0) / wrapping_rem(0) on most
        // integer storage types panic (matches i32::overflowing_div), so we
        // don't exercise those paths here — they're well-known and the
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
    // ─── arithmetic.rs (D38 overflow paths via mg_divide) ──────────────────
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

    // ─── overflow: wrapping_div / wrapping_rem (non-zero rhs path) ──────────
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
        // 7.00 / 2.00 = 3.50 → 350 at S=2
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

    // ─── overflow: saturating_div sign-aware ───────────────────────────────

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
        // 1.0 at S=74 is 10^74 — make rhs = 1 LSB so q = a * 10^74 / 1 → way past storage.
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

    // ─── overflow: overflowing_rem ─────────────────────────────────────────

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
