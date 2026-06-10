//! Cross-type equality surface.
//! Migrated from `tests/equalities_wide_branches.rs`.

#[cfg(feature = "wide")]
mod from_equalities_wide_branches {
    //! Coverage for the remaining `macros/equalities.rs` branches: wide-arm
    //! reciprocal directions (`prim == D76`) and the `to_*_checked()` `None`
    //! branches for values that don't fit a primitive.

    use decimal_scaled::{D38, D76};

    #[test]
    fn reciprocal_signed_int_eq() {
        let v: D76<2> = D38::<2>::from(42).into();
        assert!(42_i32 == v);
        assert!(42_i64 == v);
        assert!(42_i128 == v);
        // Fractional vs primitive — false even from the primitive side.
        let frac: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(4201_i128).unwrap()).into();
        assert!(!(42_i32 == frac));
        assert!(!(42_i64 == frac));
    }

    #[test]
    fn reciprocal_unsigned_int_eq() {
        let v: D76<2> = D38::<2>::from(42).into();
        assert!(42_u8 == v);
        assert!(42_u16 == v);
        assert!(42_u32 == v);
        assert!(42_u64 == v);
        assert!(42_u128 == v);
        // Negative D76 vs unsigned — false from primitive side.
        let neg: D76<2> = D38::<2>::from(-1).into();
        assert!(!(0_u32 == neg));
    }

    #[test]
    fn wide_i128_quotient_out_of_range_is_false() {
        // A D76 value whose integer quotient exceeds i128 should never equal
        // any i128.

        let big = D76::<0>::MAX; // exceeds i128
        assert!(!(big == 0_i32));
        assert!(!(big == 0_i64));
        assert!(!(big == 0_i128));
        assert!(!(big == 0_u8));
        assert!(!(big == 0_u16));
        assert!(!(big == 0_u32));
        assert!(!(big == 0_u64));
        assert!(!(big == 0_u128));
        // Negative side
        let neg_big = D76::<0>::MIN;
        assert!(!(neg_big == 0_i64));
        assert!(!(neg_big == 0_i128));
    }

    #[test]
    fn wide_fractional_vs_i128_is_false() {
        let frac: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(123_i128).unwrap()).into(); // 0.... at S=2
        assert!(!(frac == 0_i128));
    }
}
