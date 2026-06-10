//! Integer-method (`div_euclid` / `div_floor` / `midpoint` / ...) and
//! `to_int_with` mode-cell coverage.

mod from_macros_int_and_conversions {
    //! Coverage suite for `macros/int_methods.rs` and `macros/conversions.rs`.
    //!
    //! `int_methods` emits div_euclid / rem_euclid / div_floor / div_ceil /
    //! abs_diff / midpoint / is_zero / is_normal / is_nan / is_infinite /
    //! is_finite / mul_add. Several of these have sign-dependent branches.
    //!
    //! `conversions` emits `to_int_with(mode)` — a large `match` over
    //! `RoundingMode` with sign-dependent rounding logic. Each rounding mode
    //! has three branches (under-half / over-half / exact-half) and two sign
    //! branches; we systematically hit every cell.

    use decimal_scaled::{D18, D38, RoundingMode};

    // ─── int_methods coverage ──────────────────────────────────────────────

    #[test]
    fn div_euclid_rem_euclid_signs() {
        let a = D38::<2>::from(7);
        let b = D38::<2>::from(2);
        let q = a.div_euclid(b);
        let r = a.rem_euclid(b);
        // 7 / 2 = 3 (positive); remainder 1
        assert_eq!(q.to_bits(), 300);
        assert_eq!(r.to_bits(), 100);
        // Negative dividend → Euclidean keeps remainder non-negative.
        let neg_seven = D38::<2>::from(-7);
        let q = neg_seven.div_euclid(b);
        let r = neg_seven.rem_euclid(b);
        // -7.div_euclid(2) = -4; rem = 1
        assert_eq!(q.to_bits(), -400);
        assert_eq!(r.to_bits(), 100);
    }

    #[test]
    fn div_floor_div_ceil_signs() {
        let pos = D38::<2>::from(7);
        let neg = D38::<2>::from(-7);
        let two = D38::<2>::from(2);
        let neg_two = D38::<2>::from(-2);

        // 7.div_floor(2) = 3
        assert_eq!(pos.div_floor(two).to_bits(), 300);
        // (-7).div_floor(2) = -4 (sign-mismatch branch)
        assert_eq!(neg.div_floor(two).to_bits(), -400);
        // 7.div_floor(-2) = -4 (sign-mismatch branch reversed)
        assert_eq!(pos.div_floor(neg_two).to_bits(), -400);
        // (-7).div_floor(-2) = 3 (same-sign branch)
        assert_eq!(neg.div_floor(neg_two).to_bits(), 300);

        // div_ceil mirrors:
        assert_eq!(pos.div_ceil(two).to_bits(), 400); // 7/2 = 4
        assert_eq!(neg.div_ceil(two).to_bits(), -300); // -7/2 = -3
        assert_eq!(pos.div_ceil(neg_two).to_bits(), -300);
        assert_eq!(neg.div_ceil(neg_two).to_bits(), 400);

        // Exact division (no remainder) takes the early `q` branch.
        let eight = D38::<2>::from(8);
        let four = D38::<2>::from(4);
        assert_eq!(eight.div_floor(four).to_bits(), 200);
        assert_eq!(eight.div_ceil(four).to_bits(), 200);
    }

    #[test]
    fn abs_diff_midpoint_mul_add() {
        let a = D38::<2>::from(7);
        let b = D38::<2>::from(3);
        assert_eq!(a.abs_diff(b).to_bits(), 400);
        assert_eq!(b.abs_diff(a).to_bits(), 400);
        // Midpoint of 7 and 3 = 5
        let m = a.midpoint(b);
        assert_eq!(m.to_bits(), 500);
        // mul_add: 2*3 + 5 = 11
        let r = D38::<2>::from(2).mul_add(D38::<2>::from(3), D38::<2>::from(5));
        assert_eq!(r.to_bits(), 1100);

        // Narrow variant
        assert_eq!(
            D18::<2>::from(7).abs_diff(D18::<2>::from(3)).to_bits(),
            400
        );
        let _ = D18::<2>::from(2).mul_add(D18::<2>::from(3), D18::<2>::from(5));
    }

    #[test]
    fn is_zero_normal_nan_infinite_finite() {
        assert!(D38::<2>::ZERO.is_zero());
        assert!(!D38::<2>::ONE.is_zero());
        assert!(D38::<2>::ONE.is_normal());
        assert!(!D38::<2>::ZERO.is_normal());
        assert!(!D38::<2>::ZERO.is_nan());
        assert!(!D38::<2>::ZERO.is_infinite());
        assert!(D38::<2>::ZERO.is_finite());
        // narrow variants
        assert!(D18::<2>::ZERO.is_zero());
        assert!(!D18::<2>::ONE.is_zero());
        assert!(D18::<2>::ONE.is_normal());
        assert!(!D18::<2>::ONE.is_nan());
        assert!(D18::<2>::ONE.is_finite());
    }

    #[cfg(feature = "wide")]
    #[test]
    fn int_methods_wide() {
        use decimal_scaled::D76;

        let pos: D76<2> = D38::<2>::from(7).into();
        let neg: D76<2> = D38::<2>::from(-7).into();
        let two: D76<2> = D38::<2>::from(2).into();
        let neg_two: D76<2> = D38::<2>::from(-2).into();

        let three: D76<2> = D38::<2>::from(3).into();
        let neg_four: D76<2> = D38::<2>::from(-4).into();
        let neg_three: D76<2> = D38::<2>::from(-3).into();
        let four: D76<2> = D38::<2>::from(4).into();
        assert_eq!(pos.div_floor(two), three);
        assert_eq!(neg.div_floor(two), neg_four);
        assert_eq!(pos.div_floor(neg_two), neg_four);
        assert_eq!(neg.div_floor(neg_two), three);

        assert_eq!(pos.div_ceil(two), four);
        assert_eq!(neg.div_ceil(two), neg_three);

        // Exact branch
        let eight: D76<2> = D38::<2>::from(8).into();
        let four_actual: D76<2> = D38::<2>::from(4).into();
        let two_actual: D76<2> = D38::<2>::from(2).into();
        assert_eq!(eight.div_floor(four_actual), two_actual);
        assert_eq!(eight.div_ceil(four_actual), two_actual);

        // is_zero / is_normal
        assert!(D76::<2>::ZERO.is_zero());
        assert!(!D76::<2>::ZERO.is_normal());
        assert!(!D76::<2>::ZERO.is_nan());
        assert!(!D76::<2>::ZERO.is_infinite());
        assert!(D76::<2>::ZERO.is_finite());

        // div_euclid / rem_euclid / abs_diff / midpoint / mul_add
        let _ = pos.div_euclid(two);
        let _ = pos.rem_euclid(two);
        let _ = pos.abs_diff(neg);
        let _ = pos.midpoint(neg);
        let _ = pos.mul_add(two, four);
    }

    // ─── conversions: to_int_with — every rounding mode × every sign × every magnitude ───

    #[test]
    fn to_int_half_to_even_all_branches() {
        // S=2 storage; pick raws so we hit (under-half, over-half, exact-half)
        // with both signs, and (exact-half with even/odd quotient).
        // raw=149 → 1.49 (under half) → 1
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(149_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            1
        );
        // raw=151 → 1.51 (over half) → 2
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(151_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            2
        );
        // raw=150 → 1.50 (exact half, quot=1 odd) → 2
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            2
        );
        // raw=250 → 2.50 (exact half, quot=2 even) → 2
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(250_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            2
        );
        // negative side
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-149_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            -1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-151_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            -2
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-150_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            -2
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-250_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            -2
        );
        // exact (no remainder) — no rounding branch
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(300_i128).unwrap()).to_int_with(RoundingMode::HalfToEven),
            3
        );
    }

    #[test]
    fn to_int_half_away_from_zero_all_branches() {
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(149_i128).unwrap()).to_int_with(RoundingMode::HalfAwayFromZero),
            1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap()).to_int_with(RoundingMode::HalfAwayFromZero),
            2
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(151_i128).unwrap()).to_int_with(RoundingMode::HalfAwayFromZero),
            2
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-149_i128).unwrap()).to_int_with(RoundingMode::HalfAwayFromZero),
            -1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-150_i128).unwrap()).to_int_with(RoundingMode::HalfAwayFromZero),
            -2
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-151_i128).unwrap()).to_int_with(RoundingMode::HalfAwayFromZero),
            -2
        );
    }

    #[test]
    fn to_int_half_toward_zero_all_branches() {
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(149_i128).unwrap()).to_int_with(RoundingMode::HalfTowardZero),
            1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap()).to_int_with(RoundingMode::HalfTowardZero),
            1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(151_i128).unwrap()).to_int_with(RoundingMode::HalfTowardZero),
            2
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-149_i128).unwrap()).to_int_with(RoundingMode::HalfTowardZero),
            -1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-150_i128).unwrap()).to_int_with(RoundingMode::HalfTowardZero),
            -1
        );
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-151_i128).unwrap()).to_int_with(RoundingMode::HalfTowardZero),
            -2
        );
    }

    #[test]
    fn to_int_trunc() {
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(199_i128).unwrap()).to_int_with(RoundingMode::Trunc), 1);
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-199_i128).unwrap()).to_int_with(RoundingMode::Trunc), -1);
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(100_i128).unwrap()).to_int_with(RoundingMode::Trunc), 1);
    }

    #[test]
    fn to_int_floor() {
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(199_i128).unwrap()).to_int_with(RoundingMode::Floor), 1);
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-101_i128).unwrap()).to_int_with(RoundingMode::Floor), -2);
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(100_i128).unwrap()).to_int_with(RoundingMode::Floor), 1); // exact
    }

    #[test]
    fn to_int_ceiling() {
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(101_i128).unwrap()).to_int_with(RoundingMode::Ceiling), 2);
        assert_eq!(
            D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-199_i128).unwrap()).to_int_with(RoundingMode::Ceiling),
            -1
        );
        assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(100_i128).unwrap()).to_int_with(RoundingMode::Ceiling), 1); // exact
    }

    #[test]
    fn to_int_saturation() {
        // Construct a value whose integer part overflows i64.
        // D38<0>::MAX has integer part = i128::MAX > i64::MAX → saturates.
        type D = D38<0>;
        assert_eq!(D::MAX.to_int(), i64::MAX);
        assert_eq!(D::MIN.to_int(), i64::MIN);
        assert_eq!(D::MAX.to_int_with(RoundingMode::Trunc), i64::MAX);
        assert_eq!(D::MIN.to_int_with(RoundingMode::HalfToEven), i64::MIN);
    }

    #[test]
    fn to_int_narrow_widths() {
        // Each width has its own to_int_with body emitted by the macro,
        // so call them all.
        assert_eq!(
            D18::<2>::from_bits(decimal_scaled::Int::<1>::from(149_i64)).to_int_with(RoundingMode::HalfToEven),
            1
        );
        assert_eq!(
            D18::<2>::from_bits(decimal_scaled::Int::<1>::from(250_i64)).to_int_with(RoundingMode::HalfToEven),
            2
        );
        assert_eq!(D18::<2>::from_bits(decimal_scaled::Int::<1>::from(-150_i64)).to_int_with(RoundingMode::Floor), -2);
        assert_eq!(
            D18::<2>::from_bits(decimal_scaled::Int::<1>::from(-150_i64)).to_int_with(RoundingMode::Ceiling),
            -1
        );
        assert_eq!(
            D18::<2>::from_bits(decimal_scaled::Int::<1>::from(150_i64)).to_int_with(RoundingMode::HalfAwayFromZero),
            2
        );
        assert_eq!(
            D18::<2>::from_bits(decimal_scaled::Int::<1>::from(-150_i64)).to_int_with(RoundingMode::HalfTowardZero),
            -1
        );

        // to_int default delegator
        let _ = D18::<2>::from(5).to_int();
        let _ = D38::<2>::from(5).to_int();
    }

    #[cfg(feature = "wide")]
    #[test]
    fn to_int_wide_all_modes_and_signs() {
        use decimal_scaled::D76;

        // Easier: lift D38 raws.
        let lift_bits = |raw: i64| -> D76<2> {
            // Build by lifting a D38<2> with the right raw.
            let n38 = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(raw as i128).unwrap());
            n38.into()
        };
        assert_eq!(lift_bits(149).to_int_with(RoundingMode::HalfToEven), 1);
        assert_eq!(lift_bits(150).to_int_with(RoundingMode::HalfToEven), 2);
        assert_eq!(lift_bits(250).to_int_with(RoundingMode::HalfToEven), 2);
        assert_eq!(lift_bits(151).to_int_with(RoundingMode::HalfToEven), 2);
        assert_eq!(lift_bits(-149).to_int_with(RoundingMode::HalfToEven), -1);
        assert_eq!(lift_bits(-150).to_int_with(RoundingMode::HalfToEven), -2);
        assert_eq!(lift_bits(-250).to_int_with(RoundingMode::HalfToEven), -2);
        assert_eq!(
            lift_bits(150).to_int_with(RoundingMode::HalfAwayFromZero),
            2
        );
        assert_eq!(
            lift_bits(-150).to_int_with(RoundingMode::HalfAwayFromZero),
            -2
        );
        assert_eq!(lift_bits(150).to_int_with(RoundingMode::HalfTowardZero), 1);
        assert_eq!(lift_bits(151).to_int_with(RoundingMode::HalfTowardZero), 2);
        assert_eq!(
            lift_bits(-150).to_int_with(RoundingMode::HalfTowardZero),
            -1
        );
        assert_eq!(lift_bits(199).to_int_with(RoundingMode::Trunc), 1);
        assert_eq!(lift_bits(-199).to_int_with(RoundingMode::Trunc), -1);
        assert_eq!(lift_bits(150).to_int_with(RoundingMode::Floor), 1);
        assert_eq!(lift_bits(-101).to_int_with(RoundingMode::Floor), -2);
        assert_eq!(lift_bits(101).to_int_with(RoundingMode::Ceiling), 2);
        assert_eq!(lift_bits(-199).to_int_with(RoundingMode::Ceiling), -1);
        // Exact remainder
        assert_eq!(lift_bits(300).to_int_with(RoundingMode::HalfToEven), 3);
        // from_int / from_i32 (wide arm)
        let five_wide: D76<2> = D38::<2>::from(5).into();
        assert_eq!(D76::<2>::from(5), five_wide);
        assert_eq!(D76::<2>::from(5), five_wide);
        // to_int default delegator
        let _ = D76::<2>::ONE.to_int();
    }

    #[cfg(feature = "wide")]
    #[test]
    fn to_int_wide_saturation() {
        use decimal_scaled::D76;
        type D = D76<0>;
        // D76<0>::MAX is way above i64::MAX → saturate to i64::MAX
        assert_eq!(D::MAX.to_int(), i64::MAX);
        assert_eq!(D::MIN.to_int(), i64::MIN);
    }

    // ─── TryFrom<u128> / TryFrom<i128> / TryFrom<f64> ──────────────────────

    #[test]
    fn try_from_i128_narrow() {
        // In-range
        let v: D18<2> = 100_i128.try_into().unwrap();
        assert_eq!(v.to_bits(), 10_000);
        // Overflow
        let res: Result<D18<2>, _> = i128::MAX.try_into();
        assert!(res.is_err());
        let res: Result<D38<2>, _> = i128::MAX.try_into();
        assert!(res.is_err());
    }

    #[test]
    fn try_from_u128_narrow() {
        // Below i128::MAX
        let v: D18<2> = 100_u128.try_into().unwrap();
        assert_eq!(v.to_bits(), 10_000);
        let v: D38<2> = 100_u128.try_into().unwrap();
        assert_eq!(v.to_bits(), 10_000);
        // u128 above i128::MAX
        let res: Result<D18<2>, _> = u128::MAX.try_into();
        assert!(res.is_err());
        let res: Result<D38<2>, _> = u128::MAX.try_into();
        assert!(res.is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn try_from_f64_narrow() {
        let v: D18<2> = 1.5_f64.try_into().unwrap();
        assert_eq!(v.to_bits(), 150);
        let v: D38<2> = 1.5_f64.try_into().unwrap();
        assert_eq!(v.to_bits(), 150);
        // Non-finite → NotFinite
        let res: Result<D18<2>, _> = f64::NAN.try_into();
        assert!(res.is_err());
        let res: Result<D18<2>, _> = f64::INFINITY.try_into();
        assert!(res.is_err());
        // Overflow
        let res: Result<D18<2>, _> = (1e18_f64).try_into();
        assert!(res.is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn try_from_f32_narrow() {
        let v: D18<2> = 1.5_f32.try_into().unwrap();
        assert_eq!(v.to_bits(), 150);
        let v: D38<2> = 1.5_f32.try_into().unwrap();
        assert_eq!(v.to_bits(), 150);
        let res: Result<D18<2>, _> = f32::INFINITY.try_into();
        assert!(res.is_err());
    }

    #[cfg(feature = "wide")]
    #[cfg(feature = "std")]
    #[test]
    fn try_from_f64_overflow_wide() {
        use decimal_scaled::D76;
        // 1e76 exceeds D76<2>'s storage range (~5.78e74 logical).
        let r: Result<D76<2>, _> = (1e76_f64).try_into();
        assert!(r.is_err());
    }

    #[cfg(feature = "wide")]
    #[test]
    fn to_int_wide_half_away_from_zero_negative() {
        // Hits the `HalfAwayFromZero` negative branch in the wide-arm
        // `to_int_with` body (the body has the same shape per-tier so D76 is
        // representative).
        use decimal_scaled::{D76, RoundingMode};
        let v: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(-150_i128).unwrap()).into();
        assert_eq!(v.to_int_with(RoundingMode::HalfAwayFromZero), -2);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn try_from_wide_paths() {
        use decimal_scaled::D76;

        // i128 → D76 (wide arm)
        let v: D76<2> = 100i128.try_into().unwrap();
        let expected: D76<2> = D38::<2>::from(100).into();
        assert_eq!(v, expected);
        // u128 → D76 (wide arm — values above i128::MAX should still succeed
        // because wide storage can hold them).
        let v: D76<2> = (i128::MAX as u128 + 1).try_into().unwrap();
        let _ = v; // just exercise the branch
        // overflow on the multiplication step — D76<74> scales by 10^74, so
        // u128::MAX * 10^74 ~= 3.4e112 overflows 256-bit storage.

        let res: Result<D76<74>, _> = u128::MAX.try_into();
        assert!(res.is_err());
        // f64 → D76 (wide arm)
        #[cfg(feature = "std")]
        {
            let v: D76<2> = 1.5_f64.try_into().unwrap();
            let expected_f: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap()).into();
            assert_eq!(v, expected_f);
            let res: Result<D76<2>, _> = f64::NAN.try_into();
            assert!(res.is_err());
            let res: Result<D76<2>, _> = f64::INFINITY.try_into();
            assert!(res.is_err());
        }
    }
}
