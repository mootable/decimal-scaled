//! The width-generic `Decimal` trait surface: default-implemented methods
//! reachable only via trait dispatch, and a single generic function driving
//! every width.

mod from_decimal_trait_default_methods {
    //! Coverage for the default-implemented methods on the `Decimal`
    //! trait: `is_zero`, `is_one`, and `sum`. These are reachable only
    //! through trait dispatch (the type's inherent `is_zero` etc. shadow
    //! them), so tests must call them with explicit fully-qualified syntax.

    use decimal_scaled::D38s12;
    use decimal_scaled::DecimalArithmetic;

    #[test]
    fn decimal_trait_is_zero_default_impl() {
        assert!(<D38s12 as DecimalArithmetic>::is_zero(D38s12::ZERO));
        assert!(!<D38s12 as DecimalArithmetic>::is_zero(D38s12::ONE));
    }

    #[test]
    fn decimal_trait_is_one_default_impl() {
        assert!(<D38s12 as DecimalArithmetic>::is_one(D38s12::ONE));
        assert!(!<D38s12 as DecimalArithmetic>::is_one(D38s12::ZERO));
    }

    #[test]
    fn decimal_trait_sum_default_impl() {
        let vals = [
            D38s12::try_from(1).unwrap(),
            D38s12::try_from(2).unwrap(),
            D38s12::try_from(3).unwrap(),
        ];
        let s: D38s12 = <D38s12 as DecimalArithmetic>::sum(vals.iter().copied());
        assert_eq!(s, D38s12::try_from(6).unwrap());
        // Empty iter → ZERO
        let s: D38s12 = <D38s12 as DecimalArithmetic>::sum(core::iter::empty());
        assert_eq!(s, D38s12::ZERO);
    }
}

mod from_decimal_trait_generic_surface {
    //! Validates that the [`Decimal`] trait exposes a uniform width-generic
    //! surface: a single generic function works on every width without
    //! reaching for inherent methods.

    use decimal_scaled::{D18, D38, Decimal, RoundingMode};
    use decimal_scaled::{DecimalArithmetic, DecimalConvert};

    /// A width-generic helper that touches each major surface area: ops,
    /// sign, overflow variants, integer methods, pow, and float bridge.
    /// Returning a tuple lets the caller assert per-width.
    fn surface_check<D: Decimal + TryFrom<i32>>(seed_i32: i32) -> (D, D, D, D)
    where
        <D as TryFrom<i32>>::Error: core::fmt::Debug,
    {
        let v = D::try_from(seed_i32).unwrap();
        // Operators via supertrait bounds.
        let doubled = v + v;
        // Intentional self-subtraction: the surface check verifies `a - a == 0`.
        #[allow(clippy::eq_op)]
        let zero_via_sub = v - v;
        assert!(zero_via_sub.is_zero());
        // Sign.
        assert!(v.is_positive() ^ v.is_negative() ^ v.is_zero() == (!v.is_zero()));
        // Pow + checked.
        let squared = v.pow(2);
        assert_eq!(v.checked_pow(0), Some(D::ONE));
        // Overflow variants — saturating mul by ONE is identity.
        assert_eq!(v.saturating_mul(D::ONE), v);
        // Integer methods.
        assert_eq!(v.abs_diff(D::ZERO), v.abs());
        // Float bridge (gated to keep the function callable in any feature
        // configuration; tests assert when std is present).
        #[cfg(feature = "std")]
        {
            let f = v.to_f64();
            let back = D::from_f64(f);
            assert_eq!(back, v);
        }
        (v, doubled, squared, v.signum())
    }

    #[test]
    fn surface_check_d18() {

        let (v, doubled, squared, _) = surface_check::<D18<4>>(5);
        assert_eq!(v.to_bits(), 50_000);
        assert_eq!(doubled.to_bits(), 100_000);
        assert_eq!(squared.to_bits(), 250_000);
    }

    #[test]
    fn surface_check_d38() {

        let (v, doubled, squared, _) = surface_check::<D38<6>>(5);
        assert_eq!(v.to_bits(), 5_000_000);
        assert_eq!(doubled.to_bits(), 10_000_000);
        assert_eq!(squared.to_bits(), 25_000_000);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn surface_check_d76() {
        use decimal_scaled::D76;

        let (v, _, squared, _) = surface_check::<D76<6>>(5);
        let expected_v: D76<6> = D38::<6>::try_from(5).unwrap().into();
        let expected_squared: D76<6> = D38::<6>::try_from(25).unwrap().into();
        assert_eq!(v, expected_v);
        assert_eq!(squared, expected_squared);
    }

    /// Width-generic `sum` and `product` over an iterator.
    fn fold_sum_product<D: Decimal + TryFrom<i32>>() -> (D, D)
    where
        <D as TryFrom<i32>>::Error: core::fmt::Debug,
    {
        let vs: [D; 4] = [
            D::try_from(1).unwrap(),
            D::try_from(2).unwrap(),
            D::try_from(3).unwrap(),
            D::try_from(4).unwrap(),
        ];
        (D::sum(vs.iter().copied()), D::product(vs.iter().copied()))
    }

    #[test]
    fn sum_product_d38() {

        let (s, p) = fold_sum_product::<D38<2>>();
        assert_eq!(s, D38::<2>::try_from(10).unwrap());
        assert_eq!(p, D38::<2>::try_from(24).unwrap());
    }

    /// `to_int_with` exercised via trait dispatch under each rounding mode.
    #[test]
    fn trait_to_int_with_modes() {
        fn cast<D: Decimal>(d: D, mode: RoundingMode) -> i64 {
            DecimalConvert::to_int_with(d, mode)
        }
        let v = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(151_i128).unwrap());
        assert_eq!(cast(v, RoundingMode::Floor), 1);
        assert_eq!(cast(v, RoundingMode::Ceiling), 2);
        assert_eq!(cast(v, RoundingMode::Trunc), 1);
    }

    /// The trait supertrait bounds let a generic fn use the bitwise
    /// operators on the raw storage.
    #[test]
    fn trait_bitwise_supertraits() {
        fn high_bits_off<D: Decimal>(v: D, mask: D) -> D {
            v & !mask
        }
        let v = D38::<0>::from_bits(decimal_scaled::Int::<2>::try_from(0b1111_i128).unwrap());
        let mask = D38::<0>::from_bits(decimal_scaled::Int::<2>::try_from(0b1100_i128).unwrap());
        let r = high_bits_off(v, mask);
        assert_eq!(r.to_bits(), 0b0011);
    }

    /// Default `is_zero` / `is_one` / `is_normal` reachable via trait
    /// dispatch (verified via fully-qualified syntax).
    #[test]
    fn trait_default_predicates_per_width() {
        fn check<D: Decimal>() {
            assert!(<D as DecimalArithmetic>::is_zero(D::ZERO));
            assert!(<D as DecimalArithmetic>::is_one(D::ONE));
            assert!(!<D as DecimalArithmetic>::is_normal(D::ZERO));
            assert!(<D as DecimalArithmetic>::is_normal(D::ONE));
        }
        check::<D18<0>>();
        check::<D38<0>>();
        #[cfg(feature = "wide")]
        check::<decimal_scaled::D76<0>>();
    }
}

mod from_macros_surface {
    //! The trait-method block of the retired `tests/macros_surface.rs`.

    use decimal_scaled::DecimalArithmetic;
    use decimal_scaled::D38;

    // ─── decimal_trait.rs ──────────────────────────────────────────────────

    #[test]
    fn decimal_trait_methods() {
        let v = D38::<2>::try_from(7).unwrap();
        // scale() takes self
        assert_eq!(v.scale(), 2);
        // multiplier returns Storage type
        assert_eq!(<D38<2> as DecimalArithmetic>::multiplier(), 100_i128);
        // is_zero
        assert!(!v.is_zero());
        assert!(D38::<2>::ZERO.is_zero());
        // signum
        assert_eq!(v.signum(), D38::<2>::try_from(1).unwrap());
        assert_eq!(D38::<2>::ZERO.signum(), D38::<2>::ZERO);
        assert_eq!(D38::<2>::try_from(-5).unwrap().signum(), D38::<2>::try_from(-1).unwrap());
    }
}
