//! The unified `D<S, SCALE>` foundation: Clone / Copy / Eq / Ord / Hash,
//! `repr(transparent)` sizing, plus `Default` and the raw `from_bits`
//! constants. The in-src `src/types/{unified,widths}.rs` tests merge into
//! this target in the in-src batch.

mod from_d_unified_foundation {
    //! Sanity tests for the unified `D<S, SCALE>` foundation.
    //!
    //! At this stage `D<S, SCALE>` carries the struct definition +
    //! hand-rolled `Clone` / `Copy` / `PartialEq` / `Eq` /
    //! `PartialOrd` / `Ord` / `Hash` impls (no arithmetic, no constants,
    //! no method surface yet â€” those land as per-storage impls in
    //! follow-up commits during width migration). These tests exist so
    //! the foundation has real coverage before migration starts loading
    //! it down with per-storage `impl<const SCALE: u32> D<â€¦, SCALE>`
    //! blocks.
    //!
    //! `Debug` is provided per-storage by the width-specific display
    //! macro (`decl_decimal_display!`), not as a blanket on `D<S, SCALE>`
    //! â€” so it is exercised by the per-width Debug tests, not here.

    use core::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use decimal_scaled::D;

    #[test]
    fn construct_and_access_raw() {
        let d: D<i128, 3> = D(1500);
        assert_eq!(d.0, 1500);
    }

    #[test]
    fn copy_and_clone() {
        let a: D<i64, 2> = D(42);
        let b = a; // Copy
        let c = a; // Clone
        assert_eq!(a.0, b.0);
        assert_eq!(a.0, c.0);
    }

    #[test]
    fn equality_by_raw_storage() {
        // Uses `Int<1>` storage: `Debug` (needed by assert_eq!/assert_ne!) is
        // emitted per-storage by `decl_decimal_display!`. The primitive `i32`/
        // `i64`/`i128` types are no longer decimal storages (D18 backs onto
        // `Int<1>`, D38 onto `Int<2>`), so they carry no per-storage `Debug`.
        let a: D<decimal_scaled::Int<1>, 5> = D(decimal_scaled::Int::<1>::from(123_i64));
        let b: D<decimal_scaled::Int<1>, 5> = D(decimal_scaled::Int::<1>::from(123_i64));
        let c: D<decimal_scaled::Int<1>, 5> = D(decimal_scaled::Int::<1>::from(124_i64));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn ordering_by_raw_storage() {
        // `i64` is no longer a decimal storage (D18 backs onto `Int<1>`); ordering
        // operators live only on `Int<N>`-backed `D`, so use `Int<1>` here (mirrors
        // `equality_by_raw_storage`).
        let a: D<decimal_scaled::Int<1>, 4> = D(decimal_scaled::Int::<1>::from(10i64));
        let b: D<decimal_scaled::Int<1>, 4> = D(decimal_scaled::Int::<1>::from(20i64));
        assert!(a < b);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
    }

    #[test]
    fn hashable_consistent_with_eq() {
        let a: D<i128, 6> = D(9999);
        let b: D<i128, 6> = D(9999);
        let mut ha = DefaultHasher::new();
        let mut hb = DefaultHasher::new();
        a.hash(&mut ha);
        b.hash(&mut hb);
        assert_eq!(ha.finish(), hb.finish());
    }

    // `debug_includes_scale_and_raw` removed: `Debug` is no longer a
    // blanket impl on `D<S, SCALE>` (it's emitted per-storage by
    // `decl_decimal_display!` so the formatted form is the canonical
    // decimal string, not the raw integer). Per-width Debug coverage
    // lives in the width-specific display tests.

    #[test]
    fn repr_transparent_size_matches_storage() {
        assert_eq!(
            core::mem::size_of::<D<i32, 0>>(),
            core::mem::size_of::<i32>()
        );
        assert_eq!(
            core::mem::size_of::<D<i64, 0>>(),
            core::mem::size_of::<i64>()
        );
        assert_eq!(
            core::mem::size_of::<D<i128, 0>>(),
            core::mem::size_of::<i128>()
        );
    }

    #[test]
    fn different_scales_are_distinct_types() {
        // This test exists to assert the type-level distinction even when
        // raw storage matches â€” it would fail to compile if D<S, A> and
        // D<S, B> were considered the same type. The body just observes
        // that we can hold both without coercion.
        let a: D<i64, 3> = D(100);
        let b: D<i64, 5> = D(100);
        assert_eq!(a.0, b.0);
        // a == b is a type error (different `SCALE`), demonstrating the
        // const-generic distinction. Don't try to compare them.
        let _ = (a, b);
    }
}

mod from_macros_surface {
    //! The `Default` / raw-constant blocks of the retired
    //! `tests/macros_surface.rs`.

    use decimal_scaled::{D18, D38};

    // â”€â”€â”€ core_type.rs: Default + multipliers + raw constructors â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn default_impls() {
        assert_eq!(D18::<2>::default(), D18::<2>::ZERO);
        assert_eq!(D38::<2>::default(), D38::<2>::ZERO);
    }

    #[test]
    fn from_bits_zero_one_max_min() {
        assert_eq!(D18::<2>::ZERO.to_bits(), 0);
        assert_eq!(D38::<2>::ZERO.to_bits(), 0);
        assert_eq!(D18::<2>::ONE.to_bits(), 100);
        assert_eq!(D38::<2>::ONE.to_bits(), 100);
        assert!(D18::<2>::MAX > D18::<2>::ZERO);
        assert!(D38::<2>::MAX > D38::<2>::ZERO);
    }
}

mod from_src_widths {
    use decimal_scaled::{D38s12, Int};

    /// `from_bits` / `to_bits` round-trip is exact.
    #[test]
    fn from_bits_to_bits_round_trip() {
        let raw: i128 = 1_500_000_000_000;
        let v: D38s12 = D38s12::from_bits(Int::<2>::try_from(raw).unwrap());
        assert_eq!(v.to_bits(), raw);
    }

    /// `ZERO` has raw bit value 0.
    #[test]
    fn zero_is_zero_bits() {
        assert_eq!(D38s12::ZERO.to_bits(), 0);
    }

    /// Two instances with identical raw bits compare equal.
    #[test]
    fn equal_by_underlying_bits() {
        assert_eq!(
            D38s12::from_bits(Int::<2>::try_from(42_000_000_000_000_i128).unwrap()),
            D38s12::from_bits(Int::<2>::try_from(42_000_000_000_000_i128).unwrap())
        );
        assert_ne!(D38s12::from_bits(Int::<2>::try_from(42_i128).unwrap()), D38s12::from_bits(Int::<2>::try_from(43_i128).unwrap()));
    }

    /// Ord is derived from i128: smaller bits compare less.
    #[test]
    fn ord_by_underlying_bits() {
        assert!(D38s12::from_bits(Int::<2>::try_from(1_i128).unwrap()) < D38s12::from_bits(Int::<2>::try_from(2_i128).unwrap()));
        assert!(D38s12::from_bits(Int::<2>::try_from(-1_i128).unwrap()) < D38s12::from_bits(Int::<2>::try_from(0_i128).unwrap()));
    }

    /// `multiplier()` returns 10^SCALE. At SCALE = 12 that is 10^12.
    #[test]
    fn multiplier_is_ten_to_scale() {
        assert_eq!(D38s12::multiplier(), 1_000_000_000_000_i128);
    }

    /// `SCALE` associated const returns the const-generic scale.
    #[test]
    fn scale_const_matches_type_parameter() {
        assert_eq!(D38s12::SCALE, 12);
        const N: u32 = D38s12::SCALE;
        assert_eq!(N, 12);
    }

    /// `scale()` method returns the const-generic scale and is
    /// independent of the instance's value.
    #[test]
    fn scale_method_matches_type_parameter() {
        assert_eq!(D38s12::ZERO.scale(), 12);
        assert_eq!(D38s12::ONE.scale(), 12);
        assert_eq!(D38s12::from_bits(Int::<2>::try_from(i128::MAX).unwrap()).scale(), 12);
        assert_eq!(D38s12::from_bits(Int::<2>::try_from(-7_i128).unwrap()).scale(), 12);
    }

    /// Both forms agree at non-default scales.
    #[test]
    fn scale_at_other_scales() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        type D0 = decimal_scaled::D<Int<2>, 0>;
        type D38 = decimal_scaled::D<Int<2>, 38>;
        assert_eq!(D6::SCALE, 6);
        assert_eq!(D0::SCALE, 0);
        assert_eq!(D38::SCALE, 38);
        assert_eq!(D6::ZERO.scale(), 6);
        assert_eq!(D0::ZERO.scale(), 0);
        assert_eq!(D38::ZERO.scale(), 38);
    }

    /// `ONE` has bit pattern 10^SCALE so that the logical value is 1.
    #[test]
    fn one_has_scaled_bit_pattern() {
        assert_eq!(D38s12::ONE.to_bits(), 1_000_000_000_000_i128);
    }

    /// `MAX` is `i128::MAX`.
    #[test]
    fn max_is_i128_max() {
        assert_eq!(D38s12::MAX.to_bits(), i128::MAX);
    }

    /// `MIN` is `i128::MIN`.
    #[test]
    fn min_is_i128_min() {
        assert_eq!(D38s12::MIN.to_bits(), i128::MIN);
    }

    /// `ONE` is not equal to `ZERO`.
    #[test]
    fn one_is_not_zero() {
        assert_ne!(D38s12::ONE, D38s12::ZERO);
        assert!(D38s12::ONE.is_positive());
    }

    /// `multiplier()` works correctly at non-default scales.
    #[test]
    fn multiplier_at_other_scales() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        assert_eq!(D6::multiplier(), 1_000_000_i128);
        assert_eq!(D6::ONE.to_bits(), 1_000_000_i128);

        type D0 = decimal_scaled::D<Int<2>, 0>;
        assert_eq!(D0::multiplier(), 1_i128);
        assert_eq!(D0::ONE.to_bits(), 1_i128);
    }

    // ----- D18 sanity tests -----

    #[test]
    fn d18_basics() {
        assert_eq!(decimal_scaled::D18s9::ZERO.to_bits(), 0_i64);
        assert_eq!(decimal_scaled::D18s9::ONE.to_bits(), 1_000_000_000_i64);
        assert_eq!(decimal_scaled::D18s9::multiplier(), 1_000_000_000_i64);
        assert_eq!(decimal_scaled::D18s9::SCALE, 9);
    }

    #[test]
    fn d18_arithmetic() {
        let a = decimal_scaled::D18s9::from_bits(Int::<1>::from(1_500_000_000_i64)); // 1.5
        let b = decimal_scaled::D18s9::from_bits(Int::<1>::from(2_500_000_000_i64)); // 2.5
        assert_eq!((a + b).to_bits(), 4_000_000_000);
        assert_eq!((b - a).to_bits(), 1_000_000_000);
        assert_eq!((-a).to_bits(), -1_500_000_000);

        let x = decimal_scaled::D18s9::from_bits(Int::<1>::from(2_000_000_000_i64)); // 2.0
        let y = decimal_scaled::D18s9::from_bits(Int::<1>::from(3_000_000_000_i64)); // 3.0
        assert_eq!((x * y).to_bits(), 6_000_000_000);
        assert_eq!((y / x).to_bits(), 1_500_000_000);
        assert_eq!((y % x).to_bits(), 1_000_000_000);
    }

    #[test]
    fn d18_display() {
        let v: decimal_scaled::D18s9 = decimal_scaled::D18s9::from_bits(Int::<1>::from(1_500_000_000_i64)); // 1.500000000
        assert_eq!(format!("{}", v), "1.500000000");
        let neg: decimal_scaled::D18s9 = decimal_scaled::D18s9::from_bits(Int::<1>::from(-1_500_000_000_i64));
        assert_eq!(format!("{}", neg), "-1.500000000");
    }

    #[test]
    fn cross_width_widening_d18_to_d38() {
        let mid: decimal_scaled::D18s9 = decimal_scaled::D18s9::from_bits(Int::<1>::from(i64::MAX));
        let wider: decimal_scaled::D38s9 = mid.into();
        assert_eq!(wider.to_bits(), i64::MAX as i128);
    }

    #[test]
    fn cross_width_narrowing_d38_to_d18_in_range() {
        let wide: decimal_scaled::D38s9 = decimal_scaled::D38s9::from_bits(Int::<2>::try_from(1_500_000_000_i128).unwrap());
        let narrow: decimal_scaled::D18s9 = wide.try_into().unwrap();
        assert_eq!(narrow.to_bits(), 1_500_000_000);
    }

    #[test]
    fn cross_width_narrowing_d38_to_d18_out_of_range() {
        let wide: decimal_scaled::D38s9 = decimal_scaled::D38s9::from_bits(Int::<2>::try_from(i128::MAX).unwrap());
        let narrow: Result<decimal_scaled::D18s9, _> = wide.try_into();
        assert!(narrow.is_err());
    }

    #[test]
    fn d18_from_str() {
        use core::str::FromStr;
        let v = decimal_scaled::D18s9::from_str("1.500000000").unwrap();
        assert_eq!(v.to_bits(), 1_500_000_000);
        let neg = decimal_scaled::D18s9::from_str("-1.500000000").unwrap();
        assert_eq!(neg.to_bits(), -1_500_000_000);
    }

    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn d18_consts() {
        use decimal_scaled::DecimalConstants;
        type D18s12 = decimal_scaled::D<Int<1>, 12>;
        // pi at scale 12 = 3.141592653590 (matches D38s12).
        assert_eq!(D18s12::pi().to_bits(), 3_141_592_653_590);
        // tau at scale 12 = 6.283185307180.
        assert_eq!(D18s12::tau().to_bits(), 6_283_185_307_180);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_basics() {
        use decimal_scaled::DecimalArithmetic;
        use Int;
        assert_eq!(
            decimal_scaled::D76s2::ZERO.to_bits(),
            Int::<4>::from_str_radix("0", 10).unwrap()
        );
        assert_eq!(
            decimal_scaled::D76s2::ONE.to_bits(),
            Int::<4>::from_str_radix("100", 10).unwrap()
        );
        assert_eq!(decimal_scaled::D76s2::MAX.to_bits(), Int::<4>::MAX);
        assert_eq!(decimal_scaled::D76s2::MIN.to_bits(), Int::<4>::MIN);
        assert_eq!(
            decimal_scaled::D76s2::multiplier(),
            Int::<4>::from_str_radix("100", 10).unwrap()
        );
        assert_eq!(decimal_scaled::D76s2::SCALE, 2);
        assert_eq!(decimal_scaled::D76s2::ZERO.scale(), 2);
        // SCALE = 75 (new MAX_SCALE) multiplier is 10^75, well within 256-bit range.
        let m75 = decimal_scaled::D76s75::multiplier();
        assert_eq!(
            m75,
            Int::<4>::from_str_radix(
                "1000000000000000000000000000000000000000000000000000000000000000000000000000",
                10
            )
            .unwrap()
        );
        assert_eq!(<decimal_scaled::D76s12 as DecimalArithmetic>::MAX_SCALE, 75);
        // round-trip
        let raw = Int::<4>::from_str_radix("123456789012345678901234567890", 10).unwrap();
        assert_eq!(decimal_scaled::D76s12::from_bits(raw).to_bits(), raw);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_arithmetic() {
        type D = decimal_scaled::D<Int<4>, 12>;
        let one = D::ONE;
        let two = D::from_bits(D::multiplier() + D::multiplier());
        let three =
            D::from_bits(D::multiplier() * Int::<4>::from_str_radix("3", 10).unwrap());
        // add / sub / neg
        assert_eq!((one + two), three);
        assert_eq!((three - one), two);
        assert_eq!((-one).to_bits(), -D::multiplier());
        // mul: 2 * 3 == 6
        let six =
            D::from_bits(D::multiplier() * Int::<4>::from_str_radix("6", 10).unwrap());
        assert_eq!((two * three), six);
        // div: 6 / 2 == 3
        assert_eq!((six / two), three);
        // rem: 6 % 2 == 0 (storage-level remainder)
        assert_eq!((six % two), D::ZERO);
        // assign forms
        let mut v = one;
        v += two;
        assert_eq!(v, three);
        v *= two;
        assert_eq!(v, six);
        v /= two;
        assert_eq!(v, three);
        v -= one;
        assert_eq!(v, two);
        v %= two;
        assert_eq!(v, D::ZERO);
        // fractional: 1.5 * 1.5 == 2.25 at scale 12
        let half =
            D::from_bits(D::multiplier() / Int::<4>::from_str_radix("2", 10).unwrap());
        let one_and_half = one + half;
        let product = one_and_half * one_and_half;
        let expected = D::from_bits(
            D::multiplier() * Int::<4>::from_str_radix("2", 10).unwrap()
                + D::multiplier() / Int::<4>::from_str_radix("4", 10).unwrap(),
        );
        assert_eq!(product, expected);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_display() {
        type D = decimal_scaled::D<Int<4>, 12>;
        let one = D::ONE;
        assert_eq!(format!("{}", one), "1.000000000000");
        assert_eq!(format!("{}", -one), "-1.000000000000");
        assert_eq!(format!("{}", D::ZERO), "0.000000000000");
        let half =
            D::from_bits(D::multiplier() / Int::<4>::from_str_radix("2", 10).unwrap());
        assert_eq!(format!("{}", half), "0.500000000000");
        assert_eq!(format!("{:?}", one), "D76<12>(1.000000000000)");
        // scale 0 prints no fractional part
        let int_only: decimal_scaled::D<Int<4>, 0> = decimal_scaled::D::<Int<4>, 0>::ONE;
        assert_eq!(format!("{}", int_only), "1");
        // very large magnitude near the 75-digit ceiling (new MAX_SCALE)
        let big = decimal_scaled::D76s75::MAX;
        let s = format!("{}", big);
        assert!(s.starts_with("57.8960446"));
        assert_eq!(s.len(), "57.".len() + 75);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_sign_and_helpers() {
        type D = decimal_scaled::D<Int<4>, 6>;
        let neg = -D::ONE;
        assert!(neg.is_negative());
        assert!(D::ONE.is_positive());
        assert!(!D::ZERO.is_positive());
        assert_eq!(neg.abs(), D::ONE);
        assert_eq!(D::ONE.signum(), D::ONE);
        assert_eq!(neg.signum(), neg);
        assert_eq!(D::ZERO.signum(), D::ZERO);
        // min / max / clamp
        let two = D::ONE + D::ONE;
        assert_eq!(D::ONE.min(two), D::ONE);
        assert_eq!(D::ONE.max(two), two);
        assert_eq!(two.clamp(D::ZERO, D::ONE), D::ONE);
        // copysign
        assert_eq!(D::ONE.copysign(neg), neg);
        assert_eq!(neg.copysign(D::ONE), D::ONE);
        // recip: 1/2 at scale 6
        let half =
            D::from_bits(D::multiplier() / Int::<4>::from_str_radix("2", 10).unwrap());
        assert_eq!(two.recip(), half);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_overflow_variants() {
        type D = decimal_scaled::D<Int<4>, 2>;
        // checked_add overflow at MAX
        assert_eq!(D::MAX.checked_add(D::ONE), None);
        assert_eq!(D::ONE.checked_add(D::ONE), Some(D::ONE + D::ONE));
        // saturating
        assert_eq!(D::MAX.saturating_add(D::ONE), D::MAX);
        assert_eq!(D::MIN.saturating_sub(D::ONE), D::MIN);
        // checked_neg of MIN overflows
        assert_eq!(D::MIN.checked_neg(), None);
        assert_eq!(D::ONE.checked_neg(), Some(-D::ONE));
        // checked_mul / checked_div
        let two = D::ONE + D::ONE;
        let three = two + D::ONE;
        assert_eq!(
            two.checked_mul(three),
            Some(D::from_bits(
                D::multiplier() * Int::<4>::from_str_radix("6", 10).unwrap()
            ))
        );
        assert_eq!(D::ONE.checked_div(D::ZERO), None);
        assert_eq!((three).checked_div(D::ONE), Some(three));
        // wrapping_add of one storage LSB at MAX wraps around to MIN.
        let one_lsb = D::from_bits(Int::<4>::from_str_radix("1", 10).unwrap());
        assert_eq!(D::MAX.wrapping_add(one_lsb), D::MIN);
        // overflowing
        assert_eq!(D::ONE.overflowing_add(D::ONE), (two, false));
        assert_eq!(D::MAX.overflowing_add(D::ONE).1, true);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_consts_and_from_str() {
        use decimal_scaled::DecimalConstants;
        use core::str::FromStr;
        // pi at scale 12 matches the D38 reference.
        assert_eq!(
            decimal_scaled::D::<Int<4>, 12>::pi().to_bits(),
            Int::<4>::from_str_radix("3141592653590", 10).unwrap()
        );
        assert_eq!(
            decimal_scaled::D::<Int<4>, 4>::e().to_bits(),
            Int::<4>::from_str_radix("27183", 10).unwrap()
        );
        // FromStr within i128 range
        let v = decimal_scaled::D::<Int<4>, 2>::from_str("1.50").unwrap();
        assert_eq!(
            v.to_bits(),
            Int::<4>::from_str_radix("150", 10).unwrap()
        );
        let neg = decimal_scaled::D::<Int<4>, 2>::from_str("-20.50").unwrap();
        assert_eq!(
            neg.to_bits(),
            Int::<4>::from_str_radix("-2050", 10).unwrap()
        );
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_conversions() {
        use Int;
        type D = decimal_scaled::D<Int<4>, 6>;
        // From<primitive int>
        let from_i32: D = 5i32.into();
        assert_eq!(
            from_i32.to_bits(),
            Int::<4>::from_str_radix("5000000", 10).unwrap()
        );
        let from_u64: D = 7u64.into();
        assert_eq!(
            from_u64.to_bits(),
            Int::<4>::from_str_radix("7000000", 10).unwrap()
        );
        let from_neg: D = (-3i16).into();
        assert_eq!(
            from_neg.to_bits(),
            Int::<4>::from_str_radix("-3000000", 10).unwrap()
        );
        // TryFrom<i128> / TryFrom<u128>
        let from_i128 = D::try_from(123i128).unwrap();
        assert_eq!(
            from_i128.to_bits(),
            Int::<4>::from_str_radix("123000000", 10).unwrap()
        );
        let from_u128 = D::try_from(u128::MAX).unwrap();
        assert_eq!(
            from_u128.to_bits(),
            Int::<4>::from_str_radix("340282366920938463463374607431768211455", 10).unwrap()
                * Int::<4>::from_str_radix("1000000", 10).unwrap()
        );
        // TryFrom<f64>
        let from_f64 = D::try_from(2.5f64).unwrap();
        assert_eq!(
            from_f64.to_bits(),
            Int::<4>::from_str_radix("2500000", 10).unwrap()
        );
        assert!(D::try_from(f64::NAN).is_err());
        // from_int / from_i32
        assert_eq!(D::try_from(9i128).unwrap(), D::from(9i32));
        assert_eq!(D::from(-4i32), D::from(-4i32));
        // to_int: 2.5 with HalfToEven -> 2
        use decimal_scaled::RoundingMode;
        let two_and_half = D::from_bits(Int::<4>::from_str_radix("2500000", 10).unwrap());
        assert_eq!(two_and_half.to_int_with(RoundingMode::HalfToEven), 2);
        assert_eq!(two_and_half.to_int_with(RoundingMode::HalfAwayFromZero), 3);
        assert_eq!(two_and_half.to_int_with(RoundingMode::Ceiling), 3);
        assert_eq!(two_and_half.to_int_with(RoundingMode::Floor), 2);
        let neg_two_and_half = -two_and_half;
        assert_eq!(neg_two_and_half.to_int_with(RoundingMode::Floor), -3);
        assert_eq!(neg_two_and_half.to_int_with(RoundingMode::Trunc), -2);
        // cross-width widening D38 -> D76 (lossless)
        let d38: decimal_scaled::D38s6 = decimal_scaled::D38s6::from_bits(Int::<2>::try_from(-150_i128).unwrap());
        let widened: decimal_scaled::D<Int<4>, 6> = d38.into();
        assert_eq!(widened.to_bits(), Int::<4>::from_str_radix("-150", 10).unwrap());
        // cross-width narrowing D76 -> D38 in range
        let in_range: decimal_scaled::D<Int<4>, 6> =
            decimal_scaled::D::<Int<4>, 6>::from_bits(Int::<4>::from_str_radix("999", 10).unwrap());
        let narrowed: decimal_scaled::D38s6 = in_range.try_into().unwrap();
        assert_eq!(narrowed.to_bits(), 999i128);
        // cross-width narrowing D76 -> D38 out of range
        let out_of_range = decimal_scaled::D76s75::MAX;
        let narrow_fail: Result<decimal_scaled::D<Int<2>, 75>, _> = out_of_range.try_into();
        assert!(narrow_fail.is_err());
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_rescale_rounding_floats() {
        use decimal_scaled::RoundingMode;
        use Int;
        type D6 = decimal_scaled::D<Int<4>, 6>;
        // rescale up (lossless): scale 6 -> scale 9
        let v = D6::from_bits(Int::<4>::from_str_radix("1500000", 10).unwrap()); // 1.5
        let up: decimal_scaled::D<Int<4>, 9> = v.rescale::<9>();
        assert_eq!(
            up.to_bits(),
            Int::<4>::from_str_radix("1500000000", 10).unwrap()
        );
        // rescale down (lossy, HalfToEven): scale 6 -> scale 2
        let down: decimal_scaled::D<Int<4>, 2> = v.rescale::<2>();
        assert_eq!(down.to_bits(), Int::<4>::from_str_radix("150", 10).unwrap());
        // rescale down with explicit mode: 2.5 (scale 0 representation) ...
        let two_p_five = decimal_scaled::D::<Int<4>, 1>::from_bits(Int::<4>::from_str_radix("25", 10).unwrap());
        let r0: decimal_scaled::D<Int<4>, 0> = two_p_five.rescale_with::<0>(RoundingMode::HalfToEven);
        assert_eq!(r0.to_bits(), Int::<4>::from_str_radix("2", 10).unwrap());
        let r0b: decimal_scaled::D<Int<4>, 0> = two_p_five.rescale_with::<0>(RoundingMode::HalfAwayFromZero);
        assert_eq!(r0b.to_bits(), Int::<4>::from_str_radix("3", 10).unwrap());
        // floor / ceil / round / trunc / fract on 1.5 at scale 6
        assert_eq!(v.floor(), D6::ONE);
        assert_eq!(v.ceil(), D6::ONE + D6::ONE);
        assert_eq!(v.round(), D6::ONE + D6::ONE); // half away from zero
        assert_eq!(v.trunc(), D6::ONE);
        assert_eq!(
            v.fract(),
            D6::from_bits(Int::<4>::from_str_radix("500000", 10).unwrap())
        );
        // negative: -1.5
        let neg = -v;
        assert_eq!(neg.floor(), -(D6::ONE + D6::ONE));
        assert_eq!(neg.ceil(), -D6::ONE);
        assert_eq!(neg.round(), -(D6::ONE + D6::ONE));
        // float bridge
        let from_f = D6::from_f64(2.5);
        assert_eq!(
            from_f.to_bits(),
            Int::<4>::from_str_radix("2500000", 10).unwrap()
        );
        assert_eq!(D6::from_f64(f64::NAN), D6::ZERO);
        assert_eq!(D6::from_f64(f64::INFINITY), D6::MAX);
        let round_trip = D6::ONE.to_f64();
        assert!((round_trip - 1.0).abs() < 1e-9);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d153_smoke() {
        use decimal_scaled::DecimalArithmetic;
        use Int;
        type D = decimal_scaled::D<Int<8>, 35>;
        assert_eq!(<D as DecimalArithmetic>::MAX_SCALE, 152);
        assert_eq!(D::ZERO.to_bits(), Int::<8>::from_str_radix("0", 10).unwrap());
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::try_from(6i128).unwrap());
        assert_eq!((three * two) / two, three);
        assert_eq!(format!("{}", one).len(), "1.".len() + 35);
        assert_eq!(D::try_from(5i128).unwrap().to_int(), 5);
        // rescale across the wide range
        let up: decimal_scaled::D<Int<8>, 150> = one.rescale::<150>();
        assert_eq!(up, decimal_scaled::D::<Int<8>, 150>::ONE);
        // 152-digit ceiling multiplier fits in Int<8> (new MAX_SCALE)
        let _ = decimal_scaled::D153s152::multiplier();
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d307_smoke() {
        use decimal_scaled::DecimalArithmetic;
        use Int;
        type D = decimal_scaled::D<Int<16>, 35>;
        assert_eq!(<D as DecimalArithmetic>::MAX_SCALE, 306);
        let one = D::ONE;
        let two = one + one;
        let three = two + one;
        assert_eq!(two * three, D::try_from(6i128).unwrap());
        assert_eq!((three * two) / two, three);
        assert_eq!(D::ZERO.to_bits(), Int::<16>::from_str_radix("0", 10).unwrap());
        assert_eq!(format!("{}", one).len(), "1.".len() + 35);
        // cross-width: D76 -> D307 widening, D307 -> D76 narrowing
        #[cfg(feature = "wide")]
        {
            let small: decimal_scaled::D<Int<4>, 35> = decimal_scaled::D::<Int<4>, 35>::ONE;
            let widened: decimal_scaled::D<Int<16>, 35> = small.into();
            assert_eq!(widened, D::ONE);
            let narrowed: decimal_scaled::D<Int<4>, 35> = widened.try_into().unwrap();
            assert_eq!(narrowed, decimal_scaled::D::<Int<4>, 35>::ONE);
        }
        // 306-digit ceiling multiplier fits in Int<16> (new MAX_SCALE)
        let _ = decimal_scaled::D307s306::multiplier();
    }

}

mod from_src_unified {
    use decimal_scaled::Int;

    /// Equal logical values compare equal across decimal widths at the
    /// same SCALE.
    #[test]
    fn cross_width_equal_values() {
        let narrow: decimal_scaled::D<Int<1>, 2> = decimal_scaled::D::<Int<1>, 2>::try_from(5_i64).unwrap();
        let wide: decimal_scaled::D<Int<2>, 2> = decimal_scaled::D::<Int<2>, 2>::from(5_i64);
        assert!(narrow == wide);
        assert!(wide == narrow);
    }

    /// Ordering holds across widths at the same SCALE, both directions.
    #[test]
    fn cross_width_ordering() {
        let narrow: decimal_scaled::D<Int<1>, 2> = decimal_scaled::D::<Int<1>, 2>::try_from(5_i64).unwrap();
        let wide_bigger: decimal_scaled::D<Int<2>, 2> = decimal_scaled::D::<Int<2>, 2>::from(6_i64);
        assert!(narrow < wide_bigger);
        assert!(wide_bigger > narrow);
        assert_ne!(narrow, wide_bigger);
    }

    /// A value that fits only in the wider tier still compares correctly
    /// against a narrow value (no overflow, no wraparound).
    #[test]
    fn cross_width_value_only_in_wider_tier() {
        // D38<2> scales by 10^2, so from_int(10^16) stores 10^18 in the
        // i128 backend â€” beyond the i64-backed D18 storage range, so the
        // value only fits the wider tier. The comparison must not wrap.
        let huge: decimal_scaled::D<Int<2>, 2> = decimal_scaled::D::<Int<2>, 2>::from(10_000_000_000_000_000_i64);
        let small: decimal_scaled::D<Int<1>, 2> = decimal_scaled::D::<Int<1>, 2>::try_from(1_i64).unwrap();
        assert!(small < huge);
        assert!(huge > small);
    }

    /// Negative values compare correctly across widths.
    #[test]
    fn cross_width_negatives() {
        let narrow_neg: decimal_scaled::D<Int<1>, 2> = decimal_scaled::D::<Int<1>, 2>::try_from(-3_i64).unwrap();
        let wide_neg: decimal_scaled::D<Int<2>, 2> = decimal_scaled::D::<Int<2>, 2>::from(-3_i64);
        let wide_more_neg: decimal_scaled::D<Int<2>, 2> = decimal_scaled::D::<Int<2>, 2>::from(-4_i64);
        assert_eq!(narrow_neg, wide_neg);
        assert!(wide_more_neg < narrow_neg);
        assert!(narrow_neg > wide_more_neg);
        // Sign boundary: negative narrow < non-negative wide.
        let wide_pos: decimal_scaled::D<Int<2>, 2> = decimal_scaled::D::<Int<2>, 2>::from(1_i64);
        assert!(narrow_neg < wide_pos);
    }

    /// Same-type values sort via the generic `Ord` path.
    #[test]
    fn same_type_sort() {
        let mut v = [
            decimal_scaled::D::<Int<2>, 2>::from(3_i64),
            decimal_scaled::D::<Int<2>, 2>::from(-1_i64),
            decimal_scaled::D::<Int<2>, 2>::from(2_i64),
            decimal_scaled::D::<Int<2>, 2>::from(0_i64),
        ];
        v.sort();
        assert_eq!(
            v,
            [
                decimal_scaled::D::<Int<2>, 2>::from(-1_i64),
                decimal_scaled::D::<Int<2>, 2>::from(0_i64),
                decimal_scaled::D::<Int<2>, 2>::from(2_i64),
                decimal_scaled::D::<Int<2>, 2>::from(3_i64),
            ]
        );
    }

    // â”€â”€ Cross-scale comparison (story 1.3.3). â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    
    /// `D38<S>` raw constructor: `raw` is the stored integer (logical
    /// value `raw / 10^S`).
    fn d38_raw<const S: u32>(raw: i128) -> decimal_scaled::D<Int<2>, S> {
        decimal_scaled::D::<Int<2>, S>::from_bits(Int::<2>::try_from(raw).unwrap())
    }
    fn d18_raw<const S: u32>(raw: i64) -> decimal_scaled::D<Int<1>, S> {
        decimal_scaled::D::<Int<1>, S>::from_bits(Int::<1>::try_from(raw as i128).unwrap())
    }

    /// Cross-scale EQUAL value: 1.50 (raw 150 @ S=2) == 1.5000
    /// (raw 15000 @ S=4).
    #[test]
    fn cross_scale_equal_value() {
        let a: decimal_scaled::D<Int<2>, 2> = d38_raw::<2>(150); // 1.50
        let b: decimal_scaled::D<Int<2>, 4> = d38_raw::<4>(15_000); // 1.5000
        assert!(a == b);
        assert!(b == a);
        assert_eq!(a.partial_cmp(&b), Some(core::cmp::Ordering::Equal));
    }

    /// Cross-scale ORDER: 1.51 > 1.50 across scales.
    #[test]
    fn cross_scale_order_greater() {
        let a: decimal_scaled::D<Int<2>, 4> = d38_raw::<4>(15_100); // 1.51
        let b: decimal_scaled::D<Int<2>, 2> = d38_raw::<2>(150); // 1.50
        assert!(a > b);
        assert!(b < a);
        assert_ne!(a, b);
    }

    /// Small magnitudes: 0.001 < 0.01 across scales.
    #[test]
    fn cross_scale_order_small() {
        let a: decimal_scaled::D<Int<2>, 3> = d38_raw::<3>(1); // 0.001
        let b: decimal_scaled::D<Int<2>, 2> = d38_raw::<2>(1); // 0.01
        assert!(a < b);
        assert!(b > a);
    }

    /// Negatives compare correctly across scales: -1.50 > -1.51.
    #[test]
    fn cross_scale_negatives() {
        let a: decimal_scaled::D<Int<2>, 2> = d38_raw::<2>(-150); // -1.50
        let b: decimal_scaled::D<Int<2>, 4> = d38_raw::<4>(-15_100); // -1.51
        assert!(a > b);
        assert!(b < a);
        // Equal negative across scales.
        let c: decimal_scaled::D<Int<2>, 4> = d38_raw::<4>(-15_000); // -1.50
        assert_eq!(a, c);
    }

    /// Combined cross-WIDTH and cross-SCALE: D18<2> vs D38<4>.
    #[test]
    fn cross_width_and_scale() {
        let narrow: decimal_scaled::D<Int<1>, 2> = d18_raw::<2>(150); // 1.50, i64 backend
        let wide_eq: decimal_scaled::D<Int<2>, 4> = d38_raw::<4>(15_000); // 1.5000, i128 backend
        assert!(narrow == wide_eq);
        assert!(wide_eq == narrow);

        let wide_bigger: decimal_scaled::D<Int<2>, 4> = d38_raw::<4>(15_001); // 1.5001
        assert!(narrow < wide_bigger);
        assert!(wide_bigger > narrow);
    }

    /// Tie-break on the remainder: quotients equal but the higher-scale
    /// operand carries extra low digits â†’ it is the larger magnitude.
    #[test]
    fn cross_scale_remainder_tiebreak() {
        // 1.5 @ S=1 (raw 15) vs 1.50001 @ S=5 (raw 150_001).
        // Scale-down of 150_001 by 10^4 â†’ quotient 15, remainder 1.
        let a: decimal_scaled::D<Int<2>, 1> = d38_raw::<1>(15); // 1.5
        let b: decimal_scaled::D<Int<2>, 5> = d38_raw::<5>(150_001); // 1.50001
        assert!(a < b);
        assert!(b > a);
        assert_ne!(a, b);
        // Exact tie: 1.5 vs 1.50000 â†’ equal (remainder zero).
        let c: decimal_scaled::D<Int<2>, 5> = d38_raw::<5>(150_000);
        assert_eq!(a, c);
    }
}
