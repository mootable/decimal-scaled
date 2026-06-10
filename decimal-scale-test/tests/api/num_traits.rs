//! The `num-traits` impl surface (Zero / One / Num / Bounded / Signed /
//! Checked* / FromPrimitive / ToPrimitive / NumCast) across the narrow widths
//! plus the wide spot-checks. `use ::num_traits` (leading colons) throughout:
//! this target module is itself named `num_traits`, so the bare path would be
//! ambiguous between the crate-root module and the external crate.

mod from_num_traits {
    //! `num-traits` 0.2 trait implementations for [`D38`].
    //!
    //! Allows generic numeric code (nalgebra, ndarray, statrs, and other
    //! crates that accept "any number type") to use `D38<SCALE>` as a
    //! scalar. Crates that provide generic numeric algorithms almost
    //! universally bound on [`num_traits`] traits rather than defining
    //! their own numeric interfaces.
    //!
    //! # Trait coverage
    //!
    //! - [`num_traits::Zero`] / [`num_traits::One`] â€” additive and
    //! multiplicative identities.
    //! - [`num_traits::Num`] â€” umbrella numeric trait combining
    //! `Zero + One + PartialEq + Add + Sub + Mul + Div + Rem` with a
    //! `from_str_radix` constructor.
    //! - [`num_traits::Bounded`] â€” `min_value()` / `max_value()` for
    //! generic clamping code.
    //! - [`num_traits::Signed`] â€” `abs`, `signum`, `is_positive`,
    //! `is_negative`, `abs_sub`.
    //! - [`num_traits::FromPrimitive`] / [`num_traits::ToPrimitive`] â€”
    //! fallible conversions to and from the primitive numeric types.
    //! - [`num_traits::CheckedAdd`] / [`num_traits::CheckedSub`] /
    //! [`num_traits::CheckedMul`] / [`num_traits::CheckedDiv`] /
    //! [`num_traits::CheckedRem`] / [`num_traits::CheckedNeg`] â€”
    //! overflow-safe variants returning `Option<Self>`.
    //!
    //! # `from_str_radix`
    //!
    //! [`num_traits::Num::from_str_radix`] delegates to
    //! [`core::str::FromStr`] for `radix == 10` and rejects every other
    //! radix. The compile-time signature is stable regardless of whether
    //! the underlying `FromStr` implementation is complete.
    //!
    //! # `CheckedMul` / `CheckedDiv`
    //!
    //! Both traits delegate to the inherent [`D38::checked_mul`] and
    //! [`D38::checked_div`] methods. The trait and inherent paths are
    //! bit-identical. `CheckedAdd`, `CheckedSub`, `CheckedRem`, and
    //! `CheckedNeg` operate directly on the raw `i128` storage and
    //! delegate to `i128`'s own checked intrinsics; no rescaling is
    //! needed for those operations.

    // `FromPrimitive` / `ToPrimitive` / `NumCast` for every decimal
    // width are emitted by `decl_decimal_num_traits_conversions!` (see
    // `src/macros/num_traits.rs` and the per-width invocations in
    // `core_type.rs`). This file keeps only the test module.

    // The `(N) as i128` casts and `try_from(...).unwrap()` forms below are
    // deliberate explicit value-construction in hand-written fixtures. clippy's
    // `from()` rewrite for the fallible conversions is ambiguous here (multiple
    // applicable `From` impls -> E0034) and rustfix cannot apply it, so these
    // cosmetic lints are allowed at the test-file scope.
    #![allow(clippy::unnecessary_cast, clippy::unnecessary_fallible_conversions)]

    use decimal_scaled::{D38, D38s12};
    // Zero / One / Num / Bounded / Signed / Checked* are emitted for
    // D38 by `decl_decimal_num_traits_basics!`; FromPrimitive /
    // ToPrimitive / NumCast stay hand-coded in this module. The tests
    // exercise the whole surface, so the traits are imported directly.
    use ::num_traits::{
        Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, FromPrimitive,
        Num, NumCast, One, Signed, ToPrimitive, Zero,
    };

    // ---------------------------------------------------------------------------
    // Zero / One
    // ---------------------------------------------------------------------------

    #[test]
    fn zero_is_zero_const() {
        assert_eq!(<D38s12 as Zero>::zero(), D38s12::ZERO);
    }

    #[test]
    fn zero_is_zero_predicate() {
        assert!(<D38s12 as Zero>::is_zero(&D38s12::ZERO));
        assert!(!<D38s12 as Zero>::is_zero(&D38s12::ONE));
        assert!(!<D38s12 as Zero>::is_zero(&D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap())));
    }

    #[test]
    fn one_is_one_const() {
        assert_eq!(<D38s12 as One>::one(), D38s12::ONE);
    }

    #[test]
    fn one_is_one_predicate() {
        assert!(<D38s12 as One>::is_one(&D38s12::ONE));
        assert!(!<D38s12 as One>::is_one(&D38s12::ZERO));
        // A non-canonical raw value (1 LSB) is not "one".
        assert!(!<D38s12 as One>::is_one(&D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap())));
    }

    // ---------------------------------------------------------------------------
    // Bounded
    // ---------------------------------------------------------------------------

    #[test]
    fn bounded_min_max() {
        assert_eq!(<D38s12 as Bounded>::min_value(), D38s12::MIN);
        assert_eq!(<D38s12 as Bounded>::max_value(), D38s12::MAX);
    }

    // ---------------------------------------------------------------------------
    // Signed
    // ---------------------------------------------------------------------------

    #[test]
    fn signed_abs_basic() {
        let pos = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap());
        let neg = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1_500_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as Signed>::abs(&pos), pos);
        assert_eq!(<D38s12 as Signed>::abs(&neg), pos);
        assert_eq!(<D38s12 as Signed>::abs(&D38s12::ZERO), D38s12::ZERO);
    }

    #[test]
    fn signed_signum_basic() {
        let pos = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap());
        let neg = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1_500_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as Signed>::signum(&pos), D38s12::ONE);
        assert_eq!(<D38s12 as Signed>::signum(&neg), -D38s12::ONE);
        assert_eq!(<D38s12 as Signed>::signum(&D38s12::ZERO), D38s12::ZERO);
    }

    #[test]
    fn signed_is_positive_negative() {
        let pos = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap());
        let neg = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1_500_000_000_000) as i128).unwrap());
        assert!(<D38s12 as Signed>::is_positive(&pos));
        assert!(!<D38s12 as Signed>::is_positive(&neg));
        assert!(!<D38s12 as Signed>::is_positive(&D38s12::ZERO));

        assert!(!<D38s12 as Signed>::is_negative(&pos));
        assert!(<D38s12 as Signed>::is_negative(&neg));
        assert!(!<D38s12 as Signed>::is_negative(&D38s12::ZERO));
    }

    /// `abs_sub(a, b)` clamps to zero when `a <= b`.
    #[test]
    fn signed_abs_sub_clamps_to_zero() {
        let two = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap());
        let five = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((5_000_000_000_000) as i128).unwrap());

        // 5 - 2 = 3 (positive case)
        let three = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((3_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as Signed>::abs_sub(&five, &two), three);

        // 2 - 5 clamps to ZERO (a <= b)
        assert_eq!(<D38s12 as Signed>::abs_sub(&two, &five), D38s12::ZERO);

        // 5 - 5 = ZERO (equal inputs)
        assert_eq!(<D38s12 as Signed>::abs_sub(&five, &five), D38s12::ZERO);
    }

    // ---------------------------------------------------------------------------
    // FromPrimitive
    // ---------------------------------------------------------------------------

    #[test]
    fn from_primitive_i64_in_range() {
        assert_eq!(<D38s12 as FromPrimitive>::from_i64(0), Some(D38s12::ZERO));
        assert_eq!(<D38s12 as FromPrimitive>::from_i64(1), Some(D38s12::ONE));
        assert_eq!(
            <D38s12 as FromPrimitive>::from_i64(42),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((42_000_000_000_000) as i128).unwrap()))
        );
        assert_eq!(
            <D38s12 as FromPrimitive>::from_i64(-42),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-42_000_000_000_000) as i128).unwrap()))
        );
    }

    #[test]
    fn from_primitive_u64_in_range() {
        assert_eq!(<D38s12 as FromPrimitive>::from_u64(0), Some(D38s12::ZERO));
        assert_eq!(
            <D38s12 as FromPrimitive>::from_u64(42),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((42_000_000_000_000) as i128).unwrap()))
        );
        // u64::MAX * 10^12 fits in i128, so this succeeds.
        let large = <D38s12 as FromPrimitive>::from_u64(u64::MAX);
        assert!(large.is_some());
    }

    #[test]
    fn from_primitive_i128_overflow_returns_none() {
        // i128::MAX cannot be scaled by 10^12 â€” TryFrom returns Err,
        // FromPrimitive surfaces that as None.
        assert_eq!(<D38s12 as FromPrimitive>::from_i128(i128::MAX), None);
        assert_eq!(<D38s12 as FromPrimitive>::from_i128(i128::MIN), None);

        // Small values succeed.
        assert_eq!(
            <D38s12 as FromPrimitive>::from_i128(7),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((7_000_000_000_000) as i128).unwrap()))
        );
    }

    #[test]
    fn from_primitive_u128_overflow_returns_none() {
        // u128::MAX > i128::MAX â€” the first try_from step fails.
        assert_eq!(<D38s12 as FromPrimitive>::from_u128(u128::MAX), None);

        // Small values succeed.
        assert_eq!(
            <D38s12 as FromPrimitive>::from_u128(99),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((99_000_000_000_000) as i128).unwrap()))
        );
    }

    #[test]
    fn from_primitive_f32_basic() {
        assert_eq!(<D38s12 as FromPrimitive>::from_f32(0.0), Some(D38s12::ZERO));
        assert_eq!(<D38s12 as FromPrimitive>::from_f32(1.0), Some(D38s12::ONE));
        // Non-finite inputs return None.
        assert_eq!(<D38s12 as FromPrimitive>::from_f32(f32::NAN), None);
        assert_eq!(<D38s12 as FromPrimitive>::from_f32(f32::INFINITY), None);
        assert_eq!(<D38s12 as FromPrimitive>::from_f32(f32::NEG_INFINITY), None);
    }

    #[test]
    fn from_primitive_f64_basic() {
        assert_eq!(<D38s12 as FromPrimitive>::from_f64(0.0), Some(D38s12::ZERO));
        assert_eq!(<D38s12 as FromPrimitive>::from_f64(1.0), Some(D38s12::ONE));
        // Use a value that is not close to any well-known math constant
        // so the approx_constant lint stays quiet.
        let v = <D38s12 as FromPrimitive>::from_f64(1.234567890123_f64);
        assert!(v.is_some());

        // Non-finite inputs return None.
        assert_eq!(<D38s12 as FromPrimitive>::from_f64(f64::NAN), None);
        assert_eq!(<D38s12 as FromPrimitive>::from_f64(f64::INFINITY), None);

        // Finite but out-of-range: 1e30 * 10^12 = 1e42 > i128::MAX.
        assert_eq!(<D38s12 as FromPrimitive>::from_f64(1e30), None);
    }

    /// `FromPrimitive` provides default impls for `from_i32`, `from_u32`, etc.
    /// via `from_i64` / `from_u64`. Verify the delegation chain works.
    #[test]
    fn from_primitive_smaller_int_types_via_default_impl() {
        assert_eq!(
            <D38s12 as FromPrimitive>::from_i32(7),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((7_000_000_000_000) as i128).unwrap()))
        );
        assert_eq!(
            <D38s12 as FromPrimitive>::from_i16(-3),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-3_000_000_000_000) as i128).unwrap()))
        );
        assert_eq!(<D38s12 as FromPrimitive>::from_i8(0), Some(D38s12::ZERO));
        assert_eq!(
            <D38s12 as FromPrimitive>::from_u32(7),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((7_000_000_000_000) as i128).unwrap()))
        );
        assert_eq!(
            <D38s12 as FromPrimitive>::from_u16(3),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((3_000_000_000_000) as i128).unwrap()))
        );
        assert_eq!(
            <D38s12 as FromPrimitive>::from_u8(255),
            Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((255_000_000_000_000) as i128).unwrap()))
        );
    }

    // ---------------------------------------------------------------------------
    // ToPrimitive
    // ---------------------------------------------------------------------------

    #[test]
    fn to_primitive_i64_in_range() {
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((42_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&v), Some(42_i64));

        let neg = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-42_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&neg), Some(-42_i64));

        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&D38s12::ZERO), Some(0_i64));
    }

    #[test]
    fn to_primitive_i64_truncates_toward_zero() {
        // 2.5 truncates to 2
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_500_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&v), Some(2_i64));

        // -2.5 truncates to -2 (toward zero, not toward -inf)
        let neg = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-2_500_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&neg), Some(-2_i64));
    }

    #[test]
    fn to_primitive_i64_out_of_range_returns_none() {
        // D38::MAX integer part ~= 1.7e26, which exceeds i64::MAX.
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&D38s12::MAX), None);
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&D38s12::MIN), None);
    }

    #[test]
    fn to_primitive_u64_negative_returns_none() {
        let neg = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_u64(&neg), None);
    }

    #[test]
    fn to_primitive_u64_in_range() {
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((42_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_u64(&v), Some(42_u64));

        assert_eq!(<D38s12 as ToPrimitive>::to_u64(&D38s12::ZERO), Some(0_u64));
    }

    #[test]
    fn to_primitive_i128_always_succeeds() {
        // Even MAX and MIN succeed because the integer part is bounded
        // by i128::MAX / 10^12, which is well within i128.
        assert!(<D38s12 as ToPrimitive>::to_i128(&D38s12::MAX).is_some());
        assert!(<D38s12 as ToPrimitive>::to_i128(&D38s12::MIN).is_some());
        assert_eq!(
            <D38s12 as ToPrimitive>::to_i128(&D38s12::ZERO),
            Some(0_i128)
        );
        assert_eq!(
            <D38s12 as ToPrimitive>::to_i128(&D38s12::from_bits(decimal_scaled::Int::<2>::try_from((42_000_000_000_000) as i128).unwrap())),
            Some(42_i128)
        );
    }

    #[test]
    fn to_primitive_u128_negative_returns_none() {
        assert_eq!(
            <D38s12 as ToPrimitive>::to_u128(&D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap())),
            None
        );
    }

    #[test]
    fn to_primitive_u128_in_range() {
        assert_eq!(
            <D38s12 as ToPrimitive>::to_u128(&D38s12::ZERO),
            Some(0_u128)
        );
        assert_eq!(
            <D38s12 as ToPrimitive>::to_u128(&D38s12::from_bits(decimal_scaled::Int::<2>::try_from((99_000_000_000_000) as i128).unwrap())),
            Some(99_u128)
        );
    }

    #[test]
    fn to_primitive_f64_round_trip_within_lsb() {
        let lsb = 1.0 / (i128::try_from(D38s12::multiplier()).unwrap() as f64);
        // Use a value not close to any well-known math constant.
        let v = D38s12::from_f64(1.234567890123_f64);
        let back = <D38s12 as ToPrimitive>::to_f64(&v).expect("to_f64 always returns Some");
        assert!(
            (back - 1.234567890123_f64).abs() <= lsb * 2.0,
            "round-trip exceeded 2 LSB: back = {back}, lsb = {lsb}"
        );
    }

    #[test]
    fn to_primitive_f32_matches_to_f32_lossy() {
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_f32(&v), Some(v.to_f32()));
    }

    /// `ToPrimitive` provides default impls for `to_i32`, `to_u32`, etc.
    /// via `to_i64` / `to_u64`. Verify the delegation chain works.
    #[test]
    fn to_primitive_smaller_int_types_via_default_impl() {
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((42_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as ToPrimitive>::to_i32(&v), Some(42_i32));
        assert_eq!(<D38s12 as ToPrimitive>::to_u32(&v), Some(42_u32));
        assert_eq!(<D38s12 as ToPrimitive>::to_i16(&v), Some(42_i16));
        assert_eq!(<D38s12 as ToPrimitive>::to_u16(&v), Some(42_u16));
        assert_eq!(<D38s12 as ToPrimitive>::to_i8(&v), Some(42_i8));
        assert_eq!(<D38s12 as ToPrimitive>::to_u8(&v), Some(42_u8));

        // Out-of-range narrowing returns None.
        let big = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((40_000_000_000_000_000) as i128).unwrap()); // 40_000
        assert_eq!(<D38s12 as ToPrimitive>::to_i8(&big), None);
        assert_eq!(<D38s12 as ToPrimitive>::to_u8(&big), None);
    }

    // ---------------------------------------------------------------------------
    // CheckedAdd / CheckedSub
    // ---------------------------------------------------------------------------

    #[test]
    fn checked_add_basic() {
        let one = D38s12::ONE;
        let two = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap());
        assert_eq!(<D38s12 as CheckedAdd>::checked_add(&one, &one), Some(two));
    }

    #[test]
    fn checked_add_overflow_returns_none() {
        // MAX + ONE overflows.
        assert_eq!(
            <D38s12 as CheckedAdd>::checked_add(&D38s12::MAX, &D38s12::ONE),
            None
        );
        // MAX + ZERO is fine.
        assert_eq!(
            <D38s12 as CheckedAdd>::checked_add(&D38s12::MAX, &D38s12::ZERO),
            Some(D38s12::MAX)
        );
    }

    #[test]
    fn checked_sub_basic() {
        let three = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((3_000_000_000_000) as i128).unwrap());
        let two = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap());
        assert_eq!(
            <D38s12 as CheckedSub>::checked_sub(&three, &two),
            Some(D38s12::ONE)
        );
    }

    #[test]
    fn checked_sub_underflow_returns_none() {
        // MIN - ONE underflows.
        assert_eq!(
            <D38s12 as CheckedSub>::checked_sub(&D38s12::MIN, &D38s12::ONE),
            None
        );
    }

    // ---------------------------------------------------------------------------
    // CheckedMul / CheckedDiv / CheckedRem
    // ---------------------------------------------------------------------------

    #[test]
    fn checked_mul_basic() {
        let half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((500_000_000_000) as i128).unwrap()); // 0.5
        let quarter = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((250_000_000_000) as i128).unwrap()); // 0.25
        assert_eq!(
            <D38s12 as CheckedMul>::checked_mul(&half, &half),
            Some(quarter)
        );
    }

    #[test]
    fn checked_mul_overflow_returns_none() {
        // MAX * 2 overflows.
        let two = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap());
        assert_eq!(
            <D38s12 as CheckedMul>::checked_mul(&D38s12::MAX, &two),
            None
        );
    }

    #[test]
    fn checked_div_basic() {
        let half = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((500_000_000_000) as i128).unwrap()); // 0.5
        let quarter = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((250_000_000_000) as i128).unwrap()); // 0.25
        let two = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap()); // 2.0
        // 0.5 / 2.0 == 0.25
        assert_eq!(
            <D38s12 as CheckedDiv>::checked_div(&half, &two),
            Some(quarter)
        );
    }

    #[test]
    fn checked_div_by_zero_returns_none() {
        assert_eq!(
            <D38s12 as CheckedDiv>::checked_div(&D38s12::ONE, &D38s12::ZERO),
            None
        );
    }

    #[test]
    fn checked_div_overflow_returns_none() {
        // The only true checked_div overflow is MIN / -ONE (negating i128::MIN
        // overflows in two's-complement).
        let neg_one = -D38s12::ONE;
        assert_eq!(
            <D38s12 as CheckedDiv>::checked_div(&D38s12::MIN, &neg_one),
            None
        );
        // MAX / ONE returns Some(MAX) via the widening path.
        assert_eq!(
            <D38s12 as CheckedDiv>::checked_div(&D38s12::MAX, &D38s12::ONE),
            Some(D38s12::MAX)
        );
    }

    #[test]
    fn checked_rem_basic() {
        let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((5_500_000_000_000) as i128).unwrap()); // 5.5
        let b = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap()); // 2.0
        let expected = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1_500_000_000_000) as i128).unwrap()); // 1.5
        assert_eq!(<D38s12 as CheckedRem>::checked_rem(&a, &b), Some(expected));
    }

    #[test]
    fn checked_rem_by_zero_returns_none() {
        assert_eq!(
            <D38s12 as CheckedRem>::checked_rem(&D38s12::ONE, &D38s12::ZERO),
            None
        );
    }

    // ---------------------------------------------------------------------------
    // CheckedNeg
    // ---------------------------------------------------------------------------

    #[test]
    fn checked_neg_basic() {
        let one = D38s12::ONE;
        let neg_one = -D38s12::ONE;
        assert_eq!(<D38s12 as CheckedNeg>::checked_neg(&one), Some(neg_one));
        assert_eq!(
            <D38s12 as CheckedNeg>::checked_neg(&D38s12::ZERO),
            Some(D38s12::ZERO)
        );
    }

    #[test]
    fn checked_neg_min_returns_none() {
        // i128::MIN has no positive counterpart, so checked_neg returns None.
        assert_eq!(<D38s12 as CheckedNeg>::checked_neg(&D38s12::MIN), None);
    }

    // ---------------------------------------------------------------------------
    // CheckedMul / CheckedDiv trait-vs-inherent alignment
    // ---------------------------------------------------------------------------
    //
    // Assert that the num-traits trait impls and the inherent methods
    // produce bit-identical results for 256 deterministic pairs plus
    // boundary cases. A failure here means the two paths diverged.

    /// Generates a deterministic sequence of `i128` values using a
    /// linear congruential generator seeded from `seed`.
    fn lcg_i128_seq(seed: i128, n: usize) -> Vec<i128> {
        // LCG constants from Knuth TAOCP Vol 2 (applied in i128 with wrapping).
        let mut state: i128 = seed;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005_i128)
                .wrapping_add(1_442_695_040_888_963_407_i128);
            out.push(state);
        }
        out
    }

    /// For 256 deterministic pairs, `<D38 as CheckedMul>::checked_mul`
    /// must equal `D38::checked_mul` (the inherent method).
    #[test]
    fn checked_mul_trait_matches_inherent_256_pairs() {
        let seeds = lcg_i128_seq(0x1234_5678_9ABC_DEF0, 512);
        for pair in seeds.chunks_exact(2) {
            let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((pair[0]) as i128).unwrap());
            let b = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((pair[1]) as i128).unwrap());
            let trait_result = <D38s12 as CheckedMul>::checked_mul(&a, &b);
            let inherent_result = a.checked_mul(b);
            assert_eq!(
                trait_result, inherent_result,
                "CheckedMul trait != inherent for a={a:?} b={b:?}"
            );
        }
    }

    /// For 256 deterministic pairs, `<D38 as CheckedDiv>::checked_div`
    /// must equal `D38::checked_div` (the inherent method).
    #[test]
    fn checked_div_trait_matches_inherent_256_pairs() {
        let seeds = lcg_i128_seq(0xDEAD_BEEF_CAFE_0001, 512);
        for pair in seeds.chunks_exact(2) {
            let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((pair[0]) as i128).unwrap());
            // Avoid divide-by-zero: if the LCG lands on zero, substitute ONE.
            // The by-zero case is covered by a dedicated test.
            let b_bits = if pair[1] == 0 {
                i128::try_from(D38s12::multiplier()).unwrap()
            } else {
                pair[1]
            };
            let b = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((b_bits) as i128).unwrap());
            let trait_result = <D38s12 as CheckedDiv>::checked_div(&a, &b);
            let inherent_result = a.checked_div(b);
            assert_eq!(
                trait_result, inherent_result,
                "CheckedDiv trait != inherent for a={a:?} b={b:?}"
            );
        }
    }

    /// Boundary cases for CheckedMul trait-vs-inherent alignment.
    #[test]
    fn checked_mul_trait_matches_inherent_boundary() {
        let cases: &[(D38s12, D38s12)] = &[
            (D38s12::MAX, D38s12::ZERO),
            (D38s12::MIN, D38s12::ZERO),
            (D38s12::MAX, D38s12::ONE),
            (D38s12::MIN, D38s12::ONE),
            (D38s12::MAX, D38s12::MAX),
            (D38s12::MIN, D38s12::MIN),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((0) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((0) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap())),
        ];
        for &(a, b) in cases {
            let trait_result = <D38s12 as CheckedMul>::checked_mul(&a, &b);
            let inherent_result = a.checked_mul(b);
            assert_eq!(
                trait_result, inherent_result,
                "CheckedMul trait != inherent at boundary a={a:?} b={b:?}"
            );
        }
    }

    /// Boundary cases for CheckedDiv trait-vs-inherent alignment.
    #[test]
    fn checked_div_trait_matches_inherent_boundary() {
        let neg_one = -D38s12::ONE;
        let cases: &[(D38s12, D38s12)] = &[
            (D38s12::MAX, D38s12::ONE),
            (D38s12::MIN, D38s12::ONE),
            (D38s12::MAX, D38s12::MAX),
            (D38s12::MIN, D38s12::MIN),
            (D38s12::ZERO, D38s12::ONE),
            (D38s12::ONE, D38s12::MAX),
            // divide by zero â€” both must return None
            (D38s12::ONE, D38s12::ZERO),
            (D38s12::MAX, D38s12::ZERO),
            // true overflow case: MIN / -ONE
            (D38s12::MIN, neg_one),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap())),
            (D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap()), D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap())),
        ];
        for &(a, b) in cases {
            let trait_result = <D38s12 as CheckedDiv>::checked_div(&a, &b);
            let inherent_result = a.checked_div(b);
            assert_eq!(
                trait_result, inherent_result,
                "CheckedDiv trait != inherent at boundary a={a:?} b={b:?}"
            );
        }
    }

    // ---------------------------------------------------------------------------
    // Num::from_str_radix
    // ---------------------------------------------------------------------------

    /// Non-base-10 radix is rejected without delegating to FromStr.
    #[test]
    fn from_str_radix_non_ten_returns_invalid() {
        let result = <D38s12 as Num>::from_str_radix("1", 16);
        assert!(result.is_err());

        let result_2 = <D38s12 as Num>::from_str_radix("1", 2);
        assert!(result_2.is_err());
    }

    /// Base-10 delegates to the FromStr implementation.
    #[test]
    fn from_str_radix_base_ten_delegates_to_from_str() {
        let parsed = <D38s12 as Num>::from_str_radix("1", 10).expect("parse 1");
        assert_eq!(parsed, D38s12::ONE);
    }

    // ---------------------------------------------------------------------------
    // Cross-scale exercise â€” non-default SCALE
    // ---------------------------------------------------------------------------

    /// At SCALE = 6 the trait surface works correctly.
    #[test]
    fn traits_compile_at_scale_6() {
        type D6 = D38<6>;
        assert_eq!(<D6 as Zero>::zero(), D6::ZERO);
        assert_eq!(<D6 as One>::one(), D6::ONE);
        assert_eq!(<D6 as Bounded>::min_value(), D6::MIN);
        assert_eq!(<D6 as Bounded>::max_value(), D6::MAX);

        let v: D6 = <D6 as FromPrimitive>::from_i64(42).unwrap();
        assert_eq!(<D6 as ToPrimitive>::to_i64(&v), Some(42_i64));
    }

    // ---------------------------------------------------------------------------
    // NumCast
    // ---------------------------------------------------------------------------

    /// `NumCast::from` round-trips an in-range `i32` exactly.
    #[test]
    fn numcast_from_i32() {
        let v: D38s12 = <D38s12 as NumCast>::from(42_i32).expect("in-range");
        assert_eq!(v, <D38s12 as From<i32>>::from(42_i32));
    }

    /// `NumCast::from` preserves the fractional part of an `f64` input
    /// because the float path runs before the integer truncation path.
    #[test]
    fn numcast_from_f64_preserves_fractional() {
        let v: D38s12 = <D38s12 as NumCast>::from(1.5_f64).expect("in-range");
        assert_eq!(v, D38s12::from_f64(1.5_f64));
    }

    /// `NumCast::from` returns `None` for `f64::NAN`.
    #[test]
    fn numcast_from_f64_nan_returns_none() {
        assert!(<D38s12 as NumCast>::from(f64::NAN).is_none());
    }

    /// `NumCast::from` returns `None` for finite out-of-range `f64`.
    #[test]
    fn numcast_from_f64_out_of_range_returns_none() {
        assert!(<D38s12 as NumCast>::from(1e30_f64).is_none());
    }

    /// `NumCast::from` keeps integer inputs exact for `i64` values above
    /// f64's 53-bit mantissa range, validating the integer fast path.
    #[test]
    fn numcast_from_i64_above_f64_mantissa_is_exact() {
        // 2^54 = 18_014_398_509_481_984 â€” above f64's exact-integer range.
        let v: i64 = 1_i64 << 54;
        let d: D38s12 = <D38s12 as NumCast>::from(v).expect("in-range");
        assert_eq!(<D38s12 as ToPrimitive>::to_i64(&d), Some(v));
    }
}

mod from_macros_num_traits {
    //! Coverage suite for `macros/num_traits.rs` â€” the `num_traits`
    //! impls emitted for every decimal width. Covers Num, Signed,
    //! CheckedAdd/Sub/Mul/Div/Rem/Neg, FromPrimitive, ToPrimitive, NumCast.

    // The `try_from(...).unwrap()` / cast forms below are deliberate explicit
    // value-construction in hand-written fixtures; clippy's `from()` rewrite is
    // ambiguous here (multiple applicable `From` impls -> E0034) and rustfix
    // cannot apply it, so these cosmetic lints are allowed at the test-file scope.
    #![allow(clippy::unnecessary_cast, clippy::unnecessary_fallible_conversions)]

    use decimal_scaled::{D18, D38};
    use ::num_traits::{
        Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, FromPrimitive,
        Num, NumCast, One, Signed, ToPrimitive, Zero,
    };

    // â”€â”€â”€ Num â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn num_from_str_radix_10() {
        let v = <D38<2> as Num>::from_str_radix("1.50", 10).unwrap();
        assert_eq!(v.to_bits(), 150);
        let v = <D18<2> as Num>::from_str_radix("3", 10).unwrap();
        assert_eq!(v.to_bits(), 300);
    }

    #[test]
    fn num_from_str_radix_non_10_returns_err() {
        // Non-decimal radix is rejected.
        assert!(<D38<2> as Num>::from_str_radix("FF", 16).is_err());
        assert!(<D18<2> as Num>::from_str_radix("10", 8).is_err());
    }

    // â”€â”€â”€ Checked* via trait dispatch â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn checked_add_sub_mul_div_rem_neg_traits() {
        let a = D38::<2>::try_from(7).unwrap();
        let b = D38::<2>::try_from(2).unwrap();
        assert_eq!(
            <D38<2> as CheckedAdd>::checked_add(&a, &b),
            Some(D38::<2>::try_from(9).unwrap())
        );
        assert_eq!(
            <D38<2> as CheckedSub>::checked_sub(&a, &b),
            Some(D38::<2>::try_from(5).unwrap())
        );
        assert_eq!(
            <D38<2> as CheckedMul>::checked_mul(&a, &b),
            Some(D38::<2>::try_from(14).unwrap())
        );
        let q = <D38<2> as CheckedDiv>::checked_div(&a, &b).unwrap();
        assert_eq!(q.to_bits(), 350);
        let _ = <D38<2> as CheckedRem>::checked_rem(&a, &b).unwrap();
        let _ = <D38<2> as CheckedNeg>::checked_neg(&a).unwrap();
        // Failure paths
        assert!(<D38<2> as CheckedDiv>::checked_div(&a, &D38::<2>::ZERO).is_none());
        assert!(<D38<2> as CheckedAdd>::checked_add(&D38::<2>::MAX, &D38::<2>::ONE).is_none());
        assert!(<D38<2> as CheckedNeg>::checked_neg(&D38::<2>::MIN).is_none());

        // narrow width:
        let a = D18::<2>::try_from(3).unwrap();
        let b = D18::<2>::try_from(2).unwrap();
        let _ = <D18<2> as CheckedAdd>::checked_add(&a, &b).unwrap();
        let _ = <D18<2> as CheckedSub>::checked_sub(&a, &b).unwrap();
        let _ = <D18<2> as CheckedMul>::checked_mul(&a, &b).unwrap();
        let _ = <D18<2> as CheckedDiv>::checked_div(&a, &b).unwrap();
        let _ = <D18<2> as CheckedRem>::checked_rem(&a, &b).unwrap();
        let _ = <D18<2> as CheckedNeg>::checked_neg(&a).unwrap();
    }

    // â”€â”€â”€ Signed â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn signed_traits() {
        let pos = D38::<2>::try_from(7).unwrap();
        let neg = D38::<2>::try_from(-7).unwrap();
        assert_eq!(pos.abs(), pos);
        assert_eq!(neg.abs(), pos);
        assert_eq!(pos.signum(), D38::<2>::ONE);
        assert_eq!(neg.signum(), -D38::<2>::ONE);
        assert_eq!(D38::<2>::ZERO.signum(), D38::<2>::ZERO);
        assert!(<D38<2> as Signed>::is_positive(&pos));
        assert!(!<D38<2> as Signed>::is_positive(&neg));
        assert!(<D38<2> as Signed>::is_negative(&neg));
        assert!(!<D38<2> as Signed>::is_negative(&pos));
        // abs_sub: 7-2=5; 2-7=0 (saturates)
        assert_eq!(pos.abs_sub(&D38::<2>::try_from(2).unwrap()), D38::<2>::try_from(5).unwrap());
        assert_eq!(D38::<2>::try_from(2).unwrap().abs_sub(&pos), D38::<2>::ZERO);

        // narrow
        let pos18 = D18::<2>::try_from(3).unwrap();
        let neg18 = D18::<2>::try_from(-3).unwrap();
        let _ = <D18<2> as Signed>::abs(&pos18);
        let _ = <D18<2> as Signed>::signum(&neg18);
        assert!(<D18<2> as Signed>::is_positive(&pos18));
        let _ = pos18.abs_sub(&D18::<2>::try_from(1).unwrap());
    }

    // â”€â”€â”€ FromPrimitive â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn from_primitive_all_widths() {
        // Trait methods are ambiguous with inherent ones (e.g. inherent
        // `from_f64` returns Self; trait `FromPrimitive::from_f64` returns
        // Option<Self>). Use fully-qualified syntax to pin the trait method.
        assert_eq!(
            <D38<2> as FromPrimitive>::from_i64(5).unwrap().to_bits(),
            500
        );
        assert_eq!(
            <D18<2> as FromPrimitive>::from_i64(5).unwrap().to_bits(),
            500
        );
        // u64 (in-range)
        assert_eq!(
            <D38<2> as FromPrimitive>::from_u64(5).unwrap().to_bits(),
            500
        );
        assert_eq!(
            <D18<2> as FromPrimitive>::from_u64(5).unwrap().to_bits(),
            500
        );
        // u64 overflow on narrow width (D18<2> max ~92PQ; from_u64(u64::MAX) overflows)
        assert!(<D18<2> as FromPrimitive>::from_u64(u64::MAX).is_none());
        // i128 / u128
        assert_eq!(
            <D38<2> as FromPrimitive>::from_i128(5).unwrap().to_bits(),
            500
        );
        assert_eq!(
            <D38<2> as FromPrimitive>::from_u128(5).unwrap().to_bits(),
            500
        );
        // f32 / f64 â€” Option-returning trait variants
        assert_eq!(
            <D38<2> as FromPrimitive>::from_f32(1.5).unwrap().to_bits(),
            150
        );
        assert_eq!(
            <D38<2> as FromPrimitive>::from_f64(1.5).unwrap().to_bits(),
            150
        );
        assert!(<D38<2> as FromPrimitive>::from_f32(f32::NAN).is_none());
        assert!(<D38<2> as FromPrimitive>::from_f64(f64::INFINITY).is_none());

        // narrow width f32/f64/i128/u128
        assert_eq!(
            <D18<2> as FromPrimitive>::from_f32(1.5).unwrap().to_bits(),
            150
        );
        assert_eq!(
            <D18<2> as FromPrimitive>::from_f64(1.5).unwrap().to_bits(),
            150
        );
        assert_eq!(
            <D18<2> as FromPrimitive>::from_i128(3).unwrap().to_bits(),
            300
        );
        assert_eq!(
            <D18<2> as FromPrimitive>::from_u128(3).unwrap().to_bits(),
            300
        );
        assert!(<D18<2> as FromPrimitive>::from_i128(i128::MAX).is_none());
        assert!(<D18<2> as FromPrimitive>::from_u128(u128::MAX).is_none());
    }

    // â”€â”€â”€ ToPrimitive â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn to_primitive_all_widths() {
        let v = D38::<2>::try_from(7).unwrap();
        assert_eq!(<D38<2> as ToPrimitive>::to_i64(&v), Some(7));
        assert_eq!(<D38<2> as ToPrimitive>::to_u64(&v), Some(7));
        assert_eq!(<D38<2> as ToPrimitive>::to_i128(&v), Some(7));
        assert_eq!(<D38<2> as ToPrimitive>::to_u128(&v), Some(7));
        assert_eq!(<D38<2> as ToPrimitive>::to_f32(&v), Some(7.0_f32));
        assert_eq!(<D38<2> as ToPrimitive>::to_f64(&v), Some(7.0));
        // Negative â†’ u64 is None
        let neg = D38::<2>::try_from(-7).unwrap();
        assert_eq!(<D38<2> as ToPrimitive>::to_u64(&neg), None);
        assert_eq!(<D38<2> as ToPrimitive>::to_u128(&neg), None);
        assert_eq!(<D38<2> as ToPrimitive>::to_i64(&neg), Some(-7));

        let v18 = D18::<2>::try_from(7).unwrap();
        assert_eq!(<D18<2> as ToPrimitive>::to_i64(&v18), Some(7));
        assert_eq!(<D18<2> as ToPrimitive>::to_u64(&v18), Some(7));
        assert_eq!(<D18<2> as ToPrimitive>::to_i128(&v18), Some(7));
        assert_eq!(<D18<2> as ToPrimitive>::to_u128(&v18), Some(7));
        assert_eq!(<D18<2> as ToPrimitive>::to_f32(&v18), Some(7.0_f32));
        let neg18 = D18::<2>::try_from(-7).unwrap();
        assert_eq!(<D18<2> as ToPrimitive>::to_u128(&neg18), None);
    }

    // â”€â”€â”€ NumCast â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn numcast_integer_path() {
        // Integer-shaped input takes the lossless i128 path.
        let v: D38<2> = <D38<2> as NumCast>::from(5i32).unwrap();
        assert_eq!(v.to_bits(), 500);
        let v: D38<2> = <D38<2> as NumCast>::from(5_u64).unwrap();
        assert_eq!(v.to_bits(), 500);
        let v: D18<2> = <D18<2> as NumCast>::from(5i32).unwrap();
        assert_eq!(v.to_bits(), 500);
    }

    #[test]
    fn numcast_float_path() {
        // Non-integer f64 takes the float path.
        let v: D38<2> = <D38<2> as NumCast>::from(1.5_f64).unwrap();
        assert_eq!(v.to_bits(), 150);
        let v: D18<2> = <D18<2> as NumCast>::from(1.5_f64).unwrap();
        assert_eq!(v.to_bits(), 150);
    }

    #[test]
    fn numcast_none_path() {
        // NaN / out-of-range returns None.
        assert!(<D18<2> as NumCast>::from(f64::NAN).is_none());
        assert!(<D38<2> as NumCast>::from(f64::INFINITY).is_none());
    }

    // â”€â”€â”€ Zero / One / Bounded (existing tests in macros_surface; here we
    //     additionally hit the trait-method bodies that may share text with
    //     the macro's `impl Zero { is_zero }` etc.) â”€â”€â”€

    #[test]
    fn zero_one_bounded_redux() {
        assert!(<D18<2> as Zero>::zero().is_zero());
        assert!(<D18<2> as One>::one().is_one());
        assert_eq!(<D18<2> as Bounded>::min_value(), D18::<2>::MIN);
        assert_eq!(<D18<2> as Bounded>::max_value(), D18::<2>::MAX);
    }

    // â”€â”€â”€ Wide variants â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[cfg(feature = "wide")]
    #[test]
    fn num_traits_wide_basics() {
        use decimal_scaled::D76;

        let one: D76<2> = D38::<2>::ONE.into();
        let two: D76<2> = D38::<2>::try_from(2).unwrap().into();
        let _ = <D76<2> as CheckedAdd>::checked_add(&one, &two);
        let _ = <D76<2> as CheckedSub>::checked_sub(&two, &one);
        let _ = <D76<2> as CheckedMul>::checked_mul(&one, &two);
        let _ = <D76<2> as CheckedDiv>::checked_div(&two, &one).unwrap();
        let _ = <D76<2> as CheckedRem>::checked_rem(&two, &one).unwrap();
        let _ = <D76<2> as CheckedNeg>::checked_neg(&one).unwrap();
        assert!(<D76<2> as CheckedDiv>::checked_div(&one, &D76::<2>::ZERO).is_none());
        // num_traits::Num
        let v = <D76<2> as Num>::from_str_radix("1.50", 10).unwrap();
        let exp: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()).into();
        assert_eq!(v, exp);
        assert!(<D76<2> as Num>::from_str_radix("FF", 16).is_err());
        // Signed
        let neg: D76<2> = D38::<2>::try_from(-3).unwrap().into();
        let pos: D76<2> = D38::<2>::try_from(3).unwrap().into();
        let _ = <D76<2> as Signed>::abs(&neg);
        let _ = <D76<2> as Signed>::signum(&neg);
        assert!(<D76<2> as Signed>::is_negative(&neg));
        assert!(<D76<2> as Signed>::is_positive(&pos));
        let _ = pos.abs_sub(&one);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn num_traits_wide_primitive_conversions() {
        use decimal_scaled::D76;
        use ::num_traits::{FromPrimitive, NumCast, ToPrimitive};

        let exp: D76<2> = D38::<2>::try_from(5).unwrap().into();
        assert_eq!(<D76<2> as FromPrimitive>::from_i64(5).unwrap(), exp);
        assert_eq!(<D76<2> as FromPrimitive>::from_u64(5).unwrap(), exp);
        assert_eq!(<D76<2> as FromPrimitive>::from_i128(5).unwrap(), exp);
        assert_eq!(<D76<2> as FromPrimitive>::from_u128(5).unwrap(), exp);
        let exp_f: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()).into();
        assert_eq!(<D76<2> as FromPrimitive>::from_f32(1.5).unwrap(), exp_f);
        assert_eq!(<D76<2> as FromPrimitive>::from_f64(1.5).unwrap(), exp_f);

        // ToPrimitive on wide â€” disambiguate trait method (inherent to_f32 returns f32 not Option<f32>).
        let v: D76<2> = D38::<2>::try_from(7).unwrap().into();
        assert_eq!(<D76<2> as ToPrimitive>::to_i64(&v), Some(7));
        assert_eq!(<D76<2> as ToPrimitive>::to_u64(&v), Some(7));
        assert_eq!(<D76<2> as ToPrimitive>::to_i128(&v), Some(7));
        assert_eq!(<D76<2> as ToPrimitive>::to_u128(&v), Some(7));
        assert_eq!(<D76<2> as ToPrimitive>::to_f64(&v), Some(7.0));
        assert_eq!(<D76<2> as ToPrimitive>::to_f32(&v), Some(7.0_f32));
        // Negative â†’ u64/u128 None
        let neg: D76<2> = D38::<2>::try_from(-7).unwrap().into();
        assert_eq!(<D76<2> as ToPrimitive>::to_u64(&neg), None);
        assert_eq!(<D76<2> as ToPrimitive>::to_u128(&neg), None);
        assert_eq!(<D76<2> as ToPrimitive>::to_i64(&neg), Some(-7));

        // NumCast
        let v: D76<2> = <D76<2> as NumCast>::from(5i32).unwrap();
        let exp5: D76<2> = D38::<2>::try_from(5).unwrap().into();
        assert_eq!(v, exp5);
        let v: D76<2> = <D76<2> as NumCast>::from(1.5_f64).unwrap();
        let exp_15: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((150) as i128).unwrap()).into();
        assert_eq!(v, exp_15);
        assert!(<D76<2> as NumCast>::from(f64::NAN).is_none());
    }
}

mod from_macros_surface {
    //! The Bounded / Zero / One blocks of the retired `tests/macros_surface.rs`.

    use decimal_scaled::{D18, D38};

    // â”€â”€â”€ macros/num_traits.rs: Bounded / Zero / One / Num / Inv â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    #[test]
    fn num_traits_zero_one_bounded() {
        use ::num_traits::{Bounded, One, Zero};
        assert_eq!(<D38<2> as Zero>::zero(), D38::<2>::ZERO);
        assert!(<D38<2> as Zero>::zero().is_zero());
        assert!(!D38::<2>::ONE.is_zero());
        assert_eq!(<D38<2> as One>::one(), D38::<2>::ONE);
        assert!(<D38<2> as One>::one().is_one());
        assert_eq!(<D38<2> as Bounded>::min_value(), D38::<2>::MIN);
        assert_eq!(<D38<2> as Bounded>::max_value(), D38::<2>::MAX);

        assert_eq!(<D18<2> as Zero>::zero(), D18::<2>::ZERO);
        assert!(<D18<2> as Zero>::zero().is_zero());
        assert!(<D18<2> as One>::one().is_one());
        assert_eq!(<D18<2> as Bounded>::min_value(), D18::<2>::MIN);
        assert_eq!(<D18<2> as Bounded>::max_value(), D18::<2>::MAX);
    }

    #[cfg(feature = "wide")]
    #[test]
    fn num_traits_wide() {
        use decimal_scaled::D76;
        use ::num_traits::{Bounded, One, Zero};

        assert!(<D76<2> as Zero>::zero().is_zero());
        assert!(<D76<2> as One>::one().is_one());
        assert_eq!(<D76<2> as Bounded>::min_value(), D76::<2>::MIN);
        assert_eq!(<D76<2> as Bounded>::max_value(), D76::<2>::MAX);
    }
}

mod from_src_num_traits {
    use decimal_scaled::{D38s12, Int};

    // from_num â€” thin delegate over NumCast / FromPrimitive.

    /// `from_num(i32)` matches the idiomatic `From<i32>` impl.
    #[test]
    fn from_num_i32_round_trip() {
        let d = D38s12::from_num(42_i32);
        assert_eq!(d, D38s12::from(42_i32));
        assert_eq!(d.to_num::<i32>(), 42_i32);
    }

    /// `from_num(i64)` matches `From<i64>`.
    #[test]
    fn from_num_i64_matches_from() {
        let d = D38s12::from_num(1_000_i64);
        assert_eq!(d, D38s12::from(1_000_i64));
    }

    /// `from_num(f64)` for an in-range value matches `from_f64`.
    #[test]
    fn from_num_f64_within_range() {
        let d = D38s12::from_num(1.5_f64);
        assert_eq!(d, D38s12::from_f64(1.5_f64));
    }

    /// `from_num(f64::INFINITY)` saturates to `MAX`.
    #[test]
    fn from_num_f64_inf_saturates_max() {
        assert_eq!(D38s12::from_num(f64::INFINITY), D38s12::MAX);
    }

    /// `from_num(f64::NEG_INFINITY)` saturates to `MIN`.
    #[test]
    fn from_num_f64_neg_inf_saturates_min() {
        assert_eq!(D38s12::from_num(f64::NEG_INFINITY), D38s12::MIN);
    }

    /// `from_num(f64::NAN)` returns `ZERO` (deterministic NaN policy).
    #[test]
    fn from_num_f64_nan_is_zero() {
        assert_eq!(D38s12::from_num(f64::NAN), D38s12::ZERO);
    }

    /// Finite out-of-range f64 saturates by sign.
    #[test]
    fn from_num_f64_finite_oor_saturates() {
        // 1e30 * 10^12 = 1e42 > i128::MAX ~1.7e38; positive â†’ MAX.
        assert_eq!(D38s12::from_num(1e30_f64), D38s12::MAX);
        // negative â†’ MIN.
        assert_eq!(D38s12::from_num(-1e30_f64), D38s12::MIN);
    }

    /// `from_num(f32::INFINITY)` saturates (validates f32 path).
    #[test]
    fn from_num_f32_inf_saturates() {
        assert_eq!(D38s12::from_num(f32::INFINITY), D38s12::MAX);
        assert_eq!(D38s12::from_num(f32::NEG_INFINITY), D38s12::MIN);
        assert_eq!(D38s12::from_num(f32::NAN), D38s12::ZERO);
    }

    /// `from_num` accepts values past i64's range that still fit
    /// `D38<SCALE>`'s storage â€” at `SCALE = 12`, D38's integer range is
    /// roughly Â±1.7e14 model units.
    #[test]
    fn from_num_does_not_saturate_for_wider_than_i64_decimal_range() {
        let v: i64 = 10_000_000_000_i64;
        let d = D38s12::from_num(v);
        assert_eq!(d.to_int(), v);
    }

    // to_num â€” thin delegate over NumCast / ToPrimitive.

    /// `D38::ONE.to_num::<f64>() == 1.0`.
    #[test]
    fn to_num_f64_lossy() {
        assert_eq!(D38s12::ONE.to_num::<f64>(), 1.0_f64);
        assert_eq!((-D38s12::ONE).to_num::<f64>(), -1.0_f64);
        assert_eq!(D38s12::ZERO.to_num::<f64>(), 0.0_f64);
    }

    /// `D38::ONE.to_num::<f32>() == 1.0`.
    #[test]
    fn to_num_f32_lossy() {
        assert_eq!(D38s12::ONE.to_num::<f32>(), 1.0_f32);
        assert_eq!((-D38s12::ONE).to_num::<f32>(), -1.0_f32);
    }

    /// `D38::from(42_i32).to_num::<i32>() == 42`.
    #[test]
    fn to_num_i32_in_range() {
        let d = D38s12::from(42_i32);
        assert_eq!(d.to_num::<i32>(), 42_i32);

        let neg = D38s12::from(-42_i32);
        assert_eq!(neg.to_num::<i32>(), -42_i32);
    }

    /// `D38::MAX.to_num::<i32>() == i32::MAX` (saturating positive).
    #[test]
    fn to_num_i32_out_of_range_saturates_max() {
        assert_eq!(D38s12::MAX.to_num::<i32>(), i32::MAX);
    }

    /// `D38::MIN.to_num::<i32>() == i32::MIN` (saturating negative).
    #[test]
    fn to_num_i32_out_of_range_saturates_min() {
        assert_eq!(D38s12::MIN.to_num::<i32>(), i32::MIN);
    }

    /// `to_num::<i64>()` saturates at i64 bounds.
    #[test]
    fn to_num_i64_saturates() {
        assert_eq!(D38s12::MAX.to_num::<i64>(), i64::MAX);
        assert_eq!(D38s12::MIN.to_num::<i64>(), i64::MIN);
        assert_eq!(D38s12::from(42_i64).to_num::<i64>(), 42_i64);
    }

    /// `to_num::<u32>()` returns 0 for negative values (saturates to
    /// `u32::MIN = 0`).
    #[test]
    fn to_num_u32_negative_saturates_to_zero() {
        assert_eq!((-D38s12::ONE).to_num::<u32>(), u32::MIN);
        assert_eq!(D38s12::MIN.to_num::<u32>(), u32::MIN);
        // Positive out-of-range â†’ u32::MAX.
        assert_eq!(D38s12::MAX.to_num::<u32>(), u32::MAX);
    }

    /// Round-trip via from_num / to_num for representative i32 values.
    #[test]
    fn from_num_to_num_round_trip_i32() {
        for v in [0_i32, 1, -1, 42, -42, 1_000_000, -1_000_000] {
            let d = D38s12::from_num(v);
            assert_eq!(d.to_num::<i32>(), v);
        }
    }

    // Cross-scale exercise â€” non-default SCALE.

    /// Compat surface works at non-default SCALE.
    #[test]
    fn from_num_to_num_at_scale_6() {
        type D6 = decimal_scaled::D<Int<2>, 6>;
        let d = D6::from_num(7_i32);
        assert_eq!(d, D6::from(7_i32));
        assert_eq!(d.to_num::<i32>(), 7_i32);
    }

    // Integer-typed inputs must not route through f64 for sign
    // detection.

    /// `from_num(i128::MAX)` saturates to `D38::MAX` via the i128 sign
    /// signal, not through a f64 round-trip. `i128::MAX * 10^12`
    /// overflows i128 storage, so `NumCast::from` returns `None`; the
    /// saturation fallback reads sign directly from i128.
    #[test]
    fn from_num_i128_max_saturates_via_int_signal() {
        assert_eq!(D38s12::from_num(i128::MAX), D38s12::MAX);
    }

    /// `from_num(i128::MIN)` saturates to `D38::MIN` via the i128 sign
    /// signal.
    #[test]
    fn from_num_i128_min_saturates_via_int_signal() {
        assert_eq!(D38s12::from_num(i128::MIN), D38s12::MIN);
    }

    /// `from_num(u128::MAX)` saturates to `D38::MAX` via the u128 sign
    /// signal. `to_i128` returns None for u128 > i128::MAX, so the u128
    /// fallback path is exercised here.
    #[test]
    fn from_num_u128_max_saturates_via_uint_signal() {
        assert_eq!(D38s12::from_num(u128::MAX), D38s12::MAX);
    }

    /// `from_num(u64::MAX)` succeeds without saturation â€” u64::MAX fits
    /// in D38's storage at `SCALE = 12`.
    #[test]
    fn from_num_u64_max_succeeds_without_saturation() {
        let d = D38s12::from_num(u64::MAX);
        assert_eq!(d, D38s12::from(u64::MAX));
    }
}
