//! Coverage suite for `macros/num_traits.rs` — the `num_traits`
//! impls emitted for every decimal width. Covers Num, Signed,
//! CheckedAdd/Sub/Mul/Div/Rem/Neg, FromPrimitive, ToPrimitive, NumCast.

use decimal_scaled::{D9, D18, D38};
use num_traits::{
    Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, FromPrimitive,
    Num, NumCast, One, Signed, ToPrimitive, Zero,
};

type D9_2 = D9<2>;
type D18_2 = D18<2>;
type D38_2 = D38<2>;

// ─── Num ───────────────────────────────────────────────────────────────

#[test]
fn num_from_str_radix_10() {
    let v = <D38_2 as Num>::from_str_radix("1.50", 10).unwrap();
    assert_eq!(v.to_bits(), 150);
    let v = <D9_2 as Num>::from_str_radix("3", 10).unwrap();
    assert_eq!(v.to_bits(), 300);
    let v = <D18_2 as Num>::from_str_radix("3", 10).unwrap();
    assert_eq!(v.to_bits(), 300);
}

#[test]
fn num_from_str_radix_non_10_returns_err() {
    // Non-decimal radix is rejected.
    assert!(<D38_2 as Num>::from_str_radix("FF", 16).is_err());
    assert!(<D9_2 as Num>::from_str_radix("10", 2).is_err());
    assert!(<D18_2 as Num>::from_str_radix("10", 8).is_err());
}

// ─── Checked* via trait dispatch ──────────────────────────────────────

#[test]
fn checked_add_sub_mul_div_rem_neg_traits() {
    let a = D38_2::from_int(7);
    let b = D38_2::from_int(2);
    assert_eq!(
        <D38_2 as CheckedAdd>::checked_add(&a, &b),
        Some(D38_2::from_int(9))
    );
    assert_eq!(
        <D38_2 as CheckedSub>::checked_sub(&a, &b),
        Some(D38_2::from_int(5))
    );
    assert_eq!(
        <D38_2 as CheckedMul>::checked_mul(&a, &b),
        Some(D38_2::from_int(14))
    );
    let q = <D38_2 as CheckedDiv>::checked_div(&a, &b).unwrap();
    assert_eq!(q.to_bits(), 350);
    let _ = <D38_2 as CheckedRem>::checked_rem(&a, &b).unwrap();
    let _ = <D38_2 as CheckedNeg>::checked_neg(&a).unwrap();
    // Failure paths
    assert!(<D38_2 as CheckedDiv>::checked_div(&a, &D38_2::ZERO).is_none());
    assert!(<D38_2 as CheckedAdd>::checked_add(&D38_2::MAX, &D38_2::ONE).is_none());
    assert!(<D38_2 as CheckedNeg>::checked_neg(&D38_2::MIN).is_none());

    // narrow widths:
    let a = D9_2::from_int(3);
    let b = D9_2::from_int(2);
    let _ = <D9_2 as CheckedAdd>::checked_add(&a, &b).unwrap();
    let _ = <D9_2 as CheckedSub>::checked_sub(&a, &b).unwrap();
    let _ = <D9_2 as CheckedMul>::checked_mul(&a, &b).unwrap();
    let _ = <D9_2 as CheckedDiv>::checked_div(&a, &b).unwrap();
    let _ = <D9_2 as CheckedRem>::checked_rem(&a, &b).unwrap();
    let _ = <D9_2 as CheckedNeg>::checked_neg(&a).unwrap();
    let a = D18_2::from_int(3);
    let b = D18_2::from_int(2);
    let _ = <D18_2 as CheckedAdd>::checked_add(&a, &b).unwrap();
    let _ = <D18_2 as CheckedSub>::checked_sub(&a, &b).unwrap();
    let _ = <D18_2 as CheckedMul>::checked_mul(&a, &b).unwrap();
    let _ = <D18_2 as CheckedDiv>::checked_div(&a, &b).unwrap();
    let _ = <D18_2 as CheckedRem>::checked_rem(&a, &b).unwrap();
    let _ = <D18_2 as CheckedNeg>::checked_neg(&a).unwrap();
}

// ─── Signed ────────────────────────────────────────────────────────────

#[test]
fn signed_traits() {
    let pos = D38_2::from_int(7);
    let neg = D38_2::from_int(-7);
    assert_eq!(pos.abs(), pos);
    assert_eq!(neg.abs(), pos);
    assert_eq!(pos.signum(), D38_2::ONE);
    assert_eq!(neg.signum(), -D38_2::ONE);
    assert_eq!(D38_2::ZERO.signum(), D38_2::ZERO);
    assert!(<D38_2 as Signed>::is_positive(&pos));
    assert!(!<D38_2 as Signed>::is_positive(&neg));
    assert!(<D38_2 as Signed>::is_negative(&neg));
    assert!(!<D38_2 as Signed>::is_negative(&pos));
    // abs_sub: 7-2=5; 2-7=0 (saturates)
    assert_eq!(pos.abs_sub(&D38_2::from_int(2)), D38_2::from_int(5));
    assert_eq!(D38_2::from_int(2).abs_sub(&pos), D38_2::ZERO);

    // narrow
    let pos9 = D9_2::from_int(3);
    let neg9 = D9_2::from_int(-3);
    assert_eq!(<D9_2 as Signed>::abs(&pos9), pos9);
    assert_eq!(<D9_2 as Signed>::signum(&neg9), -D9_2::ONE);
    assert!(<D9_2 as Signed>::is_positive(&pos9));
    assert!(<D9_2 as Signed>::is_negative(&neg9));
    let _ = pos9.abs_sub(&D9_2::from_int(1));

    let pos18 = D18_2::from_int(3);
    let neg18 = D18_2::from_int(-3);
    let _ = <D18_2 as Signed>::abs(&pos18);
    let _ = <D18_2 as Signed>::signum(&neg18);
    assert!(<D18_2 as Signed>::is_positive(&pos18));
    let _ = pos18.abs_sub(&D18_2::from_int(1));
}

// ─── FromPrimitive ─────────────────────────────────────────────────────

#[test]
fn from_primitive_all_widths() {
    // Trait methods are ambiguous with inherent ones (e.g. inherent
    // `from_f64` returns Self; trait `FromPrimitive::from_f64` returns
    // Option<Self>). Use fully-qualified syntax to pin the trait method.
    assert_eq!(
        <D38_2 as FromPrimitive>::from_i64(5).unwrap().to_bits(),
        500
    );
    assert_eq!(<D9_2 as FromPrimitive>::from_i64(5).unwrap().to_bits(), 500);
    assert_eq!(
        <D18_2 as FromPrimitive>::from_i64(5).unwrap().to_bits(),
        500
    );
    // u64 (in-range)
    assert_eq!(
        <D38_2 as FromPrimitive>::from_u64(5).unwrap().to_bits(),
        500
    );
    assert_eq!(<D9_2 as FromPrimitive>::from_u64(5).unwrap().to_bits(), 500);
    assert_eq!(
        <D18_2 as FromPrimitive>::from_u64(5).unwrap().to_bits(),
        500
    );
    // u64 overflow on D9 (D9<2> max ~21M; from_u64(u64::MAX) overflows)
    assert!(<D9_2 as FromPrimitive>::from_u64(u64::MAX).is_none());
    // i128 / u128
    assert_eq!(
        <D38_2 as FromPrimitive>::from_i128(5).unwrap().to_bits(),
        500
    );
    assert_eq!(
        <D38_2 as FromPrimitive>::from_u128(5).unwrap().to_bits(),
        500
    );
    // f32 / f64 — Option-returning trait variants
    assert_eq!(
        <D38_2 as FromPrimitive>::from_f32(1.5).unwrap().to_bits(),
        150
    );
    assert_eq!(
        <D38_2 as FromPrimitive>::from_f64(1.5).unwrap().to_bits(),
        150
    );
    assert!(<D38_2 as FromPrimitive>::from_f32(f32::NAN).is_none());
    assert!(<D38_2 as FromPrimitive>::from_f64(f64::INFINITY).is_none());

    // narrow widths f32/f64/i128/u128
    assert_eq!(
        <D9_2 as FromPrimitive>::from_f32(1.5).unwrap().to_bits(),
        150
    );
    assert_eq!(
        <D9_2 as FromPrimitive>::from_f64(1.5).unwrap().to_bits(),
        150
    );
    assert_eq!(
        <D9_2 as FromPrimitive>::from_i128(3).unwrap().to_bits(),
        300
    );
    assert_eq!(
        <D9_2 as FromPrimitive>::from_u128(3).unwrap().to_bits(),
        300
    );
    assert!(<D9_2 as FromPrimitive>::from_i128(i128::MAX).is_none());
    assert!(<D9_2 as FromPrimitive>::from_u128(u128::MAX).is_none());
    assert_eq!(
        <D18_2 as FromPrimitive>::from_f32(1.5).unwrap().to_bits(),
        150
    );
    assert_eq!(
        <D18_2 as FromPrimitive>::from_f64(1.5).unwrap().to_bits(),
        150
    );
    assert_eq!(
        <D18_2 as FromPrimitive>::from_i128(3).unwrap().to_bits(),
        300
    );
    assert_eq!(
        <D18_2 as FromPrimitive>::from_u128(3).unwrap().to_bits(),
        300
    );
}

// ─── ToPrimitive ───────────────────────────────────────────────────────

#[test]
fn to_primitive_all_widths() {
    let v = D38_2::from_int(7);
    assert_eq!(<D38_2 as ToPrimitive>::to_i64(&v), Some(7));
    assert_eq!(<D38_2 as ToPrimitive>::to_u64(&v), Some(7));
    assert_eq!(<D38_2 as ToPrimitive>::to_i128(&v), Some(7));
    assert_eq!(<D38_2 as ToPrimitive>::to_u128(&v), Some(7));
    assert_eq!(<D38_2 as ToPrimitive>::to_f32(&v), Some(7.0_f32));
    assert_eq!(<D38_2 as ToPrimitive>::to_f64(&v), Some(7.0));
    // Negative → u64 is None
    let neg = D38_2::from_int(-7);
    assert_eq!(<D38_2 as ToPrimitive>::to_u64(&neg), None);
    assert_eq!(<D38_2 as ToPrimitive>::to_u128(&neg), None);
    assert_eq!(<D38_2 as ToPrimitive>::to_i64(&neg), Some(-7));

    let v9 = D9_2::from_int(7);
    assert_eq!(<D9_2 as ToPrimitive>::to_i64(&v9), Some(7));
    assert_eq!(<D9_2 as ToPrimitive>::to_u64(&v9), Some(7));
    assert_eq!(<D9_2 as ToPrimitive>::to_i128(&v9), Some(7));
    assert_eq!(<D9_2 as ToPrimitive>::to_u128(&v9), Some(7));
    assert_eq!(<D9_2 as ToPrimitive>::to_f64(&v9), Some(7.0));
    // negative narrow → u64/u128 None
    let neg9 = D9_2::from_int(-7);
    assert_eq!(<D9_2 as ToPrimitive>::to_u64(&neg9), None);
    assert_eq!(<D9_2 as ToPrimitive>::to_u128(&neg9), None);

    let v18 = D18_2::from_int(7);
    assert_eq!(<D18_2 as ToPrimitive>::to_i64(&v18), Some(7));
    assert_eq!(<D18_2 as ToPrimitive>::to_u64(&v18), Some(7));
    assert_eq!(<D18_2 as ToPrimitive>::to_i128(&v18), Some(7));
    assert_eq!(<D18_2 as ToPrimitive>::to_u128(&v18), Some(7));
    assert_eq!(<D18_2 as ToPrimitive>::to_f32(&v18), Some(7.0_f32));
    let neg18 = D18_2::from_int(-7);
    assert_eq!(<D18_2 as ToPrimitive>::to_u128(&neg18), None);
}

// ─── NumCast ───────────────────────────────────────────────────────────

#[test]
fn numcast_integer_path() {
    // Integer-shaped input takes the lossless i128 path.
    let v: D38_2 = <D38_2 as NumCast>::from(5i32).unwrap();
    assert_eq!(v.to_bits(), 500);
    let v: D38_2 = <D38_2 as NumCast>::from(5_u64).unwrap();
    assert_eq!(v.to_bits(), 500);
    let v: D9_2 = <D9_2 as NumCast>::from(5i32).unwrap();
    assert_eq!(v.to_bits(), 500);
    let v: D18_2 = <D18_2 as NumCast>::from(5i32).unwrap();
    assert_eq!(v.to_bits(), 500);
}

#[test]
fn numcast_float_path() {
    // Non-integer f64 takes the float path.
    let v: D38_2 = <D38_2 as NumCast>::from(1.5_f64).unwrap();
    assert_eq!(v.to_bits(), 150);
    let v: D9_2 = <D9_2 as NumCast>::from(1.5_f64).unwrap();
    assert_eq!(v.to_bits(), 150);
}

#[test]
fn numcast_none_path() {
    // NaN / out-of-range returns None.
    assert!(<D9_2 as NumCast>::from(f64::NAN).is_none());
    assert!(<D38_2 as NumCast>::from(f64::INFINITY).is_none());
}

// ─── Zero / One / Bounded (existing tests in macros_surface; here we
//     additionally hit the trait-method bodies that may share text with
//     the macro's `impl Zero { is_zero }` etc.) ───

#[test]
fn zero_one_bounded_redux() {
    assert!(<D9_2 as Zero>::zero().is_zero());
    assert!(<D9_2 as One>::one().is_one());
    assert_eq!(<D9_2 as Bounded>::min_value(), D9_2::MIN);
    assert_eq!(<D9_2 as Bounded>::max_value(), D9_2::MAX);
    assert!(<D18_2 as Zero>::zero().is_zero());
    assert!(<D18_2 as One>::one().is_one());
    assert_eq!(<D18_2 as Bounded>::min_value(), D18_2::MIN);
    assert_eq!(<D18_2 as Bounded>::max_value(), D18_2::MAX);
}

// ─── Wide variants ─────────────────────────────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn num_traits_wide_basics() {
    use decimal_scaled::D76;
    type D76_2 = D76<2>;
    let one: D76_2 = D38_2::ONE.into();
    let two: D76_2 = D38_2::from_int(2).into();
    let _ = <D76_2 as CheckedAdd>::checked_add(&one, &two);
    let _ = <D76_2 as CheckedSub>::checked_sub(&two, &one);
    let _ = <D76_2 as CheckedMul>::checked_mul(&one, &two);
    let _ = <D76_2 as CheckedDiv>::checked_div(&two, &one).unwrap();
    let _ = <D76_2 as CheckedRem>::checked_rem(&two, &one).unwrap();
    let _ = <D76_2 as CheckedNeg>::checked_neg(&one).unwrap();
    assert!(<D76_2 as CheckedDiv>::checked_div(&one, &D76_2::ZERO).is_none());
    // num_traits::Num
    let v = <D76_2 as Num>::from_str_radix("1.50", 10).unwrap();
    let exp: D76_2 = D38_2::from_bits(150).into();
    assert_eq!(v, exp);
    assert!(<D76_2 as Num>::from_str_radix("FF", 16).is_err());
    // Signed
    let neg: D76_2 = D38_2::from_int(-3).into();
    let pos: D76_2 = D38_2::from_int(3).into();
    let _ = <D76_2 as Signed>::abs(&neg);
    let _ = <D76_2 as Signed>::signum(&neg);
    assert!(<D76_2 as Signed>::is_negative(&neg));
    assert!(<D76_2 as Signed>::is_positive(&pos));
    let _ = pos.abs_sub(&one);
}

#[cfg(feature = "wide")]
#[test]
fn num_traits_wide_primitive_conversions() {
    use decimal_scaled::D76;
    use num_traits::{FromPrimitive, NumCast, ToPrimitive};
    type D76_2 = D76<2>;
    let exp: D76_2 = D38_2::from_int(5).into();
    assert_eq!(<D76_2 as FromPrimitive>::from_i64(5).unwrap(), exp);
    assert_eq!(<D76_2 as FromPrimitive>::from_u64(5).unwrap(), exp);
    assert_eq!(<D76_2 as FromPrimitive>::from_i128(5).unwrap(), exp);
    assert_eq!(<D76_2 as FromPrimitive>::from_u128(5).unwrap(), exp);
    let exp_f: D76_2 = D38_2::from_bits(150).into();
    assert_eq!(<D76_2 as FromPrimitive>::from_f32(1.5).unwrap(), exp_f);
    assert_eq!(<D76_2 as FromPrimitive>::from_f64(1.5).unwrap(), exp_f);

    // ToPrimitive on wide — disambiguate trait method (inherent to_f32 returns f32 not Option<f32>).
    let v: D76_2 = D38_2::from_int(7).into();
    assert_eq!(<D76_2 as ToPrimitive>::to_i64(&v), Some(7));
    assert_eq!(<D76_2 as ToPrimitive>::to_u64(&v), Some(7));
    assert_eq!(<D76_2 as ToPrimitive>::to_i128(&v), Some(7));
    assert_eq!(<D76_2 as ToPrimitive>::to_u128(&v), Some(7));
    assert_eq!(<D76_2 as ToPrimitive>::to_f64(&v), Some(7.0));
    assert_eq!(<D76_2 as ToPrimitive>::to_f32(&v), Some(7.0_f32));
    // Negative → u64/u128 None
    let neg: D76_2 = D38_2::from_int(-7).into();
    assert_eq!(<D76_2 as ToPrimitive>::to_u64(&neg), None);
    assert_eq!(<D76_2 as ToPrimitive>::to_u128(&neg), None);
    assert_eq!(<D76_2 as ToPrimitive>::to_i64(&neg), Some(-7));

    // NumCast
    let v: D76_2 = <D76_2 as NumCast>::from(5i32).unwrap();
    let exp5: D76_2 = D38_2::from_int(5).into();
    assert_eq!(v, exp5);
    let v: D76_2 = <D76_2 as NumCast>::from(1.5_f64).unwrap();
    let exp_15: D76_2 = D38_2::from_bits(150).into();
    assert_eq!(v, exp_15);
    assert!(<D76_2 as NumCast>::from(f64::NAN).is_none());
}
