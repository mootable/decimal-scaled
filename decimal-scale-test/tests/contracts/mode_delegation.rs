//! Mode-delegation contract on the arithmetic surface: the mode-aware
//! `mul_with` / `div_with` siblings agree with the default operators, across
//! every RoundingMode, sign, tie and fast-path cell. The strict-gated
//! transcendental delegation matrix (`rounding_mode_matrix.rs`) joins this
//! target in the feature-gated batch.

mod from_arithmetic_mode_aware {
    //! The mode-delegation tests of the retired `tests/arithmetic_mode_aware.rs`;
    //! its overflow / divide-by-zero panic tests live in `contracts/overflow.rs`.

    use decimal_scaled::{D38s12, RoundingMode};

    #[test]
    fn mul_with_modes() {
        // 1.5 * 2.0 = 3.0 (exact at any mode)
        let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_000_i128).unwrap());
        let b = D38s12::from(2);
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            let r = a.mul_with(b, m);
            assert_eq!(r.to_bits(), 3_000_000_000_000, "mode {m:?}");
        }
    }

    #[test]
    fn div_with_modes() {
        let a = D38s12::from(1);
        let b = D38s12::from(3);
        // 1/3 = 0.333â€¦ â€” different modes yield slightly different LSBs.
        let r_even = a.div_with(b, RoundingMode::HalfToEven);
        let r_away = a.div_with(b, RoundingMode::HalfAwayFromZero);
        let r_trunc = a.div_with(b, RoundingMode::Trunc);
        let r_floor = a.div_with(b, RoundingMode::Floor);
        let r_ceil = a.div_with(b, RoundingMode::Ceiling);
        // Same magnitude (off by â‰¤ 1 LSB).
        let bits = [
            r_even.to_bits(),
            r_away.to_bits(),
            r_trunc.to_bits(),
            r_floor.to_bits(),
            r_ceil.to_bits(),
        ];
        let min = *bits.iter().min().unwrap();
        let max = *bits.iter().max().unwrap();
        assert!(max - min <= 1, "modes diverged by > 1 LSB: {bits:?}");
    }
    // â”€â”€â”€ MulAssign / DivAssign â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn mul_assign_div_assign() {
        let mut v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_000_000_i128).unwrap()); // 1.5
        v *= D38s12::from(2);
        assert_eq!(v.to_bits(), 3_000_000_000_000);
        v /= D38s12::from(3);
        assert_eq!(v.to_bits(), 1_000_000_000_000);
    }
    // â”€â”€â”€ D18 mul / div via the u128/u64 fast path â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    //
    // D18 at SCALE >= 10 routes through the new `i128_divrem_by_u64_with_mode`
    // helper (the `__divti3` soft-call replacement). These tests pin the
    // bit-exact behaviour across every RoundingMode and across positive /
    // negative signs / tie / non-tie cases so the schoolbook divide stays
    // identical to the prior `i128 / i128` path.

    #[test]
    fn d18_mul_with_modes_exact_at_s18() {
        use decimal_scaled::D18;
        // 1.5 * 2.0 = 3.0 (exact under every mode).
        let a = D18::<18>::from_bits(decimal_scaled::Int::<1>::from(1_500_000_000_000_000_000_i64));
        let b = D18::<18>::from(2);
        let expected = 3_000_000_000_000_000_000_i64;
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            let r = a.mul_with(b, m);
            assert_eq!(r.to_bits(), expected, "mode {m:?}");
        }
    }

    #[test]
    fn d18_div_with_modes_one_third_at_s18() {
        use decimal_scaled::D18;
        // 1.0 / 3.0 at scale 18 â€” never exact. Check all six modes agree on
        // the truncated quotient and disagree by at most 1 LSB.
        let a = D18::<18>::from(1);
        let b = D18::<18>::from(3);
        let bits = [
            a.div_with(b, RoundingMode::HalfToEven).to_bits(),
            a.div_with(b, RoundingMode::HalfAwayFromZero).to_bits(),
            a.div_with(b, RoundingMode::HalfTowardZero).to_bits(),
            a.div_with(b, RoundingMode::Trunc).to_bits(),
            a.div_with(b, RoundingMode::Floor).to_bits(),
            a.div_with(b, RoundingMode::Ceiling).to_bits(),
        ];
        let min = *bits.iter().min().unwrap();
        let max = *bits.iter().max().unwrap();
        assert!(max - min <= 1, "modes diverged by > 1 LSB: {bits:?}");
        // Trunc = floor for positive â€” both equal 333â€¦3 (18 digits).
        assert_eq!(bits[3], 333_333_333_333_333_333);
        assert_eq!(bits[4], 333_333_333_333_333_333);
        // Ceiling = trunc + 1.
        assert_eq!(bits[5], 333_333_333_333_333_334);
    }

    #[test]
    fn d18_mul_negative_signs_at_s18() {
        use decimal_scaled::D18;
        let a = D18::<18>::from_bits(decimal_scaled::Int::<1>::from(1_500_000_000_000_000_000_i64));
        let b_pos = D18::<18>::from(2);
        let b_neg = -b_pos;
        // (+1.5) * (-2.0) = -3.0
        let r1 = a.mul_with(b_neg, RoundingMode::HalfToEven);
        assert_eq!(r1.to_bits(), -3_000_000_000_000_000_000);
        // (-1.5) * (+2.0) = -3.0
        let r2 = (-a).mul_with(b_pos, RoundingMode::HalfToEven);
        assert_eq!(r2.to_bits(), -3_000_000_000_000_000_000);
        // (-1.5) * (-2.0) = +3.0
        let r3 = (-a).mul_with(b_neg, RoundingMode::HalfToEven);
        assert_eq!(r3.to_bits(), 3_000_000_000_000_000_000);
    }

    #[test]
    fn d18_div_negative_signs_at_s18() {
        use decimal_scaled::D18;
        let one = D18::<18>::from(1);
        let three_pos = D18::<18>::from(3);
        let three_neg = -three_pos;
        // (+1)/(-3) â€” both modes should equal sign-flipped (+1)/(+3) result
        // under HalfToEven.
        let pos_pos = one.div_with(three_pos, RoundingMode::HalfToEven);
        let pos_neg = one.div_with(three_neg, RoundingMode::HalfToEven);
        let neg_pos = (-one).div_with(three_pos, RoundingMode::HalfToEven);
        let neg_neg = (-one).div_with(three_neg, RoundingMode::HalfToEven);
        assert_eq!(pos_neg.to_bits(), -pos_pos.to_bits());
        assert_eq!(neg_pos.to_bits(), -pos_pos.to_bits());
        assert_eq!(neg_neg.to_bits(), pos_pos.to_bits());
    }

    #[test]
    fn d18_mul_half_to_even_tie_at_s18() {
        use decimal_scaled::D18;
        // Construct a product whose discard exactly hits the tie boundary.
        // At SCALE = 18 the divisor is 10^18; we need (a * b) % 10^18 == 5e17
        // with the truncated quotient even vs odd to verify the half-to-even
        // bump fires only when q is odd.
        //
        // (q + 0.5e0) * 10^18  â†¦ a * b = q * 10^18 + 5e17.
        //
        // Pick q = 2 (even, half-to-even tie â†¦ stays at 2) â€” a = 1, b such
        // that a*b = 2.5e18. With scale-18 storage `a.0 = 1e18` (logical 1),
        // we need b.0 = 2.5e18 which exceeds i64; rescale: a.0 = 5e17 (0.5),
        // b.0 = 5e18 â€” also too big. Use a.0 = 1e9, b.0 = 2.5e9 â†¦ product
        // 2.5e18, divided by 10^18 = 2.5; truncated 2, tie â†¦ even â†¦ 2.
        let a = D18::<18>::from_bits(decimal_scaled::Int::<1>::from(1_000_000_000_i64));
        let b = D18::<18>::from_bits(decimal_scaled::Int::<1>::from(2_500_000_000_i64));
        let r = a.mul_with(b, RoundingMode::HalfToEven);
        // 2.5 â†¦ even â†¦ 2 (storage 2e18 would overflow i64; the product is
        // 2.5e18 < i64::MAX = 9.22e18, so the result fits.)
        assert_eq!(r.to_bits(), 2);

        // q = 3 (odd, half-to-even tie â†¦ bumps to 4).
        let a3 = D18::<18>::from_bits(decimal_scaled::Int::<1>::from(1_000_000_000_i64));
        let b3 = D18::<18>::from_bits(decimal_scaled::Int::<1>::from(3_500_000_000_i64));
        let r3 = a3.mul_with(b3, RoundingMode::HalfToEven);
        assert_eq!(r3.to_bits(), 4);
    }

    #[test]
    fn d18_mul_scale_0_short_circuit() {
        use decimal_scaled::D18;
        // SCALE = 0: the `if SCALE == 0` arm bypasses the divrem helper.
        let a = D18::<0>::from(12_345);
        let b = D18::<0>::from(67_890);
        let r = a.mul_with(b, RoundingMode::HalfToEven);
        assert_eq!(r.to_bits(), 12_345_i64 * 67_890);
    }

    #[test]
    fn d18_div_with_at_s10_fast_path() {
        use decimal_scaled::D18;
        // SCALE = 10 â€” divisor 10^10 still > 2^32; the new path applies.
        let a = D18::<10>::from(7);
        let b = D18::<10>::from(2);
        let r = a.div_with(b, RoundingMode::HalfToEven);
        // 7/2 = 3.5 â†¦ storage 3.5 * 10^10 = 35_000_000_000.
        assert_eq!(r.to_bits(), 35_000_000_000);
    }
}

#[cfg(all(feature = "strict", not(feature = "fast"), any(feature = "d76", feature = "wide")))]
mod from_rounding_mode_matrix {
    //! Mode-aware and precision-aware transcendental matrix.
    //!
    //! Each strict / approx method now ships with a `_with(mode)` sibling
    //! and (for the transcendentals) an `_approx(working_digits)` family
    //! that lets callers trade guard-width for speed. These tests pin the
    //! delegation contract on a few representative methods across the
    //! narrow (D38) and wide (D76) tiers:
    //!
    //! - `*_strict()` is bit-equal to `*_strict_with(DEFAULT_ROUNDING_MODE)`.
    //! - `*_approx(g)` is bit-equal to `*_approx_with(g, DEFAULT)`.
    //! - `*_approx_with(STRICT_GUARD, mode)` is bit-equal to
    //!   `*_strict_with(mode)` (the redirect-to-strict shortcut).
    //! - Non-half rounding modes (`Floor` / `Ceiling`) produce results
    //!   that bracket the half-mode result for inexact inputs.

    use decimal_scaled::{D38, D76, RoundingMode};
    use std::str::FromStr;

    const STRICT_GUARD: u32 = 30;

    fn d38s19(s: &str) -> D38<19> {
        D38::<19>::from_str(s).expect("parse D38s19")
    }

    fn d76s30(s: &str) -> D76<30> {
        D76::<30>::from_str(s).expect("parse D76s30")
    }

    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    // D38: delegation contracts
    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn d38_ln_strict_delegates_to_strict_with_default() {
        let x = d38s19("3");
        assert_eq!(x.ln_strict(), x.ln_strict_with(RoundingMode::HalfToEven));
    }

    #[test]
    fn d38_ln_approx_delegates_to_approx_with_default() {
        let x = d38s19("3");
        assert_eq!(
            x.ln_approx(10),
            x.ln_approx_with(10, RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d38_ln_approx_at_strict_guard_redirects_to_strict() {
        let x = d38s19("3");
        assert_eq!(
            x.ln_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
            x.ln_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d38_sin_approx_at_strict_guard_redirects_to_strict() {
        let x = d38s19("1");
        assert_eq!(
            x.sin_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
            x.sin_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d38_atan2_approx_at_strict_guard_redirects_to_strict() {
        let y = d38s19("1");
        let x = d38s19("2");
        assert_eq!(
            y.atan2_approx_with(x, STRICT_GUARD, RoundingMode::HalfToEven),
            y.atan2_strict_with(x, RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d38_powf_approx_at_strict_guard_redirects_to_strict() {
        let x = d38s19("2");
        let y = d38s19("0.5");
        assert_eq!(
            x.powf_approx_with(y, STRICT_GUARD, RoundingMode::HalfToEven),
            x.powf_strict_with(y, RoundingMode::HalfToEven),
        );
    }

    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    // D38: rounding modes diverge on inexact transcendentals
    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn d38_ln_floor_and_ceiling_bracket_half() {
        // ln(3) is irrational; the last storage place differs by mode.
        let x = d38s19("3");
        let floor_v = x.ln_strict_with(RoundingMode::Floor);
        let ceil_v = x.ln_strict_with(RoundingMode::Ceiling);
        let half_v = x.ln_strict_with(RoundingMode::HalfToEven);
        assert!(
            floor_v <= half_v && half_v <= ceil_v,
            "ln(3): floor {floor_v:?} <= half {half_v:?} <= ceiling {ceil_v:?}",
        );
        // Floor < Ceiling for an irrational with non-zero residual.
        assert!(floor_v < ceil_v);
    }

    #[test]
    fn d38_sin_floor_le_ceiling() {
        let x = d38s19("1");
        let floor_v = x.sin_strict_with(RoundingMode::Floor);
        let ceil_v = x.sin_strict_with(RoundingMode::Ceiling);
        assert!(floor_v <= ceil_v);
    }

    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    // sqrt / cbrt: mode dispatch
    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn d38_sqrt_strict_delegates_to_strict_with_default() {
        let x = d38s19("2");
        assert_eq!(
            x.sqrt_strict(),
            x.sqrt_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d38_sqrt_trunc_is_floor_for_positive() {
        // sqrt(2) â‰ˆ 1.41421356â€¦ â€” irrational at any finite scale, so
        // Trunc / Floor give the smaller neighbour and Ceiling gives the
        // larger.
        let x = d38s19("2");
        let trunc_v = x.sqrt_strict_with(RoundingMode::Trunc);
        let floor_v = x.sqrt_strict_with(RoundingMode::Floor);
        let ceil_v = x.sqrt_strict_with(RoundingMode::Ceiling);
        assert_eq!(trunc_v, floor_v);
        assert!(floor_v < ceil_v);
    }

    #[test]
    fn d38_sqrt_perfect_square_modes_agree() {
        // sqrt(4) = 2 exactly â€” no residual, all modes return 2.
        let x = d38s19("4");
        let two = d38s19("2");
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            assert_eq!(x.sqrt_strict_with(m), two, "sqrt(4) under {m:?}");
        }
    }

    #[test]
    fn d38_cbrt_negative_floor_more_negative_than_ceiling() {
        // cbrt(-3) â‰ˆ -1.44â€¦ â€” negative; Floor pushes further negative
        // (greater magnitude), Ceiling pulls toward zero (smaller mag).
        let x = d38s19("-3");
        let floor_v = x.cbrt_strict_with(RoundingMode::Floor);
        let ceil_v = x.cbrt_strict_with(RoundingMode::Ceiling);
        assert!(floor_v < ceil_v);
    }

    #[test]
    fn d38_cbrt_perfect_cube_modes_agree() {
        let x = d38s19("8");
        let two = d38s19("2");
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            assert_eq!(x.cbrt_strict_with(m), two, "cbrt(8) under {m:?}");
        }
    }

    #[test]
    fn d38_hypot_strict_delegates_to_strict_with_default() {
        let a = d38s19("3");
        let b = d38s19("4");
        assert_eq!(
            a.hypot_strict(b),
            a.hypot_strict_with(b, RoundingMode::HalfToEven),
        );
    }

    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    // Wide tier (D76): same contracts
    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn d76_ln_strict_delegates_to_strict_with_default() {
        let x = d76s30("3");
        assert_eq!(x.ln_strict(), x.ln_strict_with(RoundingMode::HalfToEven));
    }

    #[test]
    fn d76_ln_approx_at_strict_guard_redirects_to_strict() {
        let x = d76s30("3");
        assert_eq!(
            x.ln_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
            x.ln_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d76_sin_approx_at_strict_guard_redirects_to_strict() {
        let x = d76s30("1");
        assert_eq!(
            x.sin_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
            x.sin_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d76_sqrt_floor_le_ceiling_for_irrational() {
        let x = d76s30("2");
        let floor_v = x.sqrt_strict_with(RoundingMode::Floor);
        let ceil_v = x.sqrt_strict_with(RoundingMode::Ceiling);
        assert!(floor_v < ceil_v);
    }

    #[test]
    fn d76_cbrt_strict_delegates_to_strict_with_default() {
        let x = d76s30("8");
        assert_eq!(
            x.cbrt_strict(),
            x.cbrt_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d76_atan2_approx_at_strict_guard_redirects_to_strict() {
        let y = d76s30("1");
        let x = d76s30("2");
        assert_eq!(
            y.atan2_approx_with(x, STRICT_GUARD, RoundingMode::HalfToEven),
            y.atan2_strict_with(x, RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d76_sin_cos_strict_delegates_to_strict_with_default() {
        let x = d76s30("0.5");
        assert_eq!(
            x.sin_cos_strict(),
            x.sin_cos_strict_with(RoundingMode::HalfToEven),
        );
    }

    #[test]
    fn d76_sinh_cosh_strict_delegates_to_strict_with_default() {
        let x = d76s30("0.5");
        assert_eq!(
            x.sinh_cosh_strict(),
            x.sinh_cosh_strict_with(RoundingMode::HalfToEven),
        );
    }

    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    // Approx-vs-strict: lower guard widths still hit fast paths exactly
    // â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn d38_ln_one_is_zero_under_all_guards() {
        let one = d38s19("1");
        for g in [6u32, 10, 15, STRICT_GUARD] {
            assert_eq!(one.ln_approx(g), D38::<19>::ZERO, "ln(1) at guard {g}");
        }
    }

    #[test]
    fn d38_sin_zero_is_zero_under_all_guards() {
        let zero = D38::<19>::ZERO;
        for g in [6u32, 10, 15, STRICT_GUARD] {
            assert_eq!(zero.sin_approx(g), D38::<19>::ZERO, "sin(0) at guard {g}");
        }
    }

    #[test]
    fn d76_ln_one_is_zero_under_all_guards() {
        let one = d76s30("1");
        for g in [6u32, 10, 15, STRICT_GUARD] {
            assert_eq!(one.ln_approx(g), D76::<30>::ZERO, "ln(1) at guard {g}");
        }
    }
}
