//! Coverage suite for the rest of the macro-emitted surface: cross-type
//! `PartialEq`, `PartialOrd`, the overflow-variant arithmetic family,
//! `Bounded` / `Zero` / `One` numerics, primitive `from_*` / `to_*`
//! conversions, bitwise ops, float-bridge, and `Default`.
//!
//! Each block targets a specific macro module so coverage gains map
//! back to the source file directly.

use decimal_scaled::DecimalArithmetic;
use decimal_scaled::{D18, D38};

type D9_2 = D18<2>;
type D18_2 = D18<2>;
type D38_2 = D38<2>;

// ─── macros/equalities.rs: cross-type PartialEq ────────────────────────
//
// Each primitive integer type yields one macro instantiation per
// decimal type. Exercise all of them in both directions, plus the
// "fractional, not equal" and "negative vs unsigned" branches.

#[test]
fn eq_d38_all_signed_ints() {
    let v = D38_2::from_int(42);
    assert_eq!(v, 42i8);
    assert_eq!(42i8, v);
    assert_eq!(v, 42i16);
    assert_eq!(42i16, v);
    assert_eq!(v, 42i32);
    assert_eq!(42i32, v);
    assert_eq!(v, 42i64);
    assert_eq!(42i64, v);
    assert_eq!(v, 42isize);
    assert_eq!(42isize, v);
    assert_eq!(v, 42i128);
    assert_eq!(42i128, v);

    // Fractional values are NEVER equal to any integer.
    let frac = D38_2::from_bits(decimal_scaled::Int::<2>::from_i128(4_201)); // 42.01
    assert_ne!(frac, 42i32);
    assert_ne!(frac, 42i64);
    assert_ne!(frac, 42i128);

    // Different magnitude.
    assert_ne!(v, 41i32);

    // Negative.
    let neg = D38_2::from_int(-7);
    assert_eq!(neg, -7i32);
    assert_ne!(neg, 7i32);
}

#[test]
fn eq_d38_all_unsigned_ints_and_sign_rejection() {
    let v = D38_2::from_int(42);
    assert_eq!(v, 42u8);
    assert_eq!(42u8, v);
    assert_eq!(v, 42u16);
    assert_eq!(42u16, v);
    assert_eq!(v, 42u32);
    assert_eq!(42u32, v);
    assert_eq!(v, 42u64);
    assert_eq!(42u64, v);
    assert_eq!(v, 42usize);
    assert_eq!(42usize, v);
    assert_eq!(v, 42u128);
    assert_eq!(42u128, v);

    // Negative decimal is never equal to any unsigned primitive.
    let neg = D38_2::from_int(-1);
    assert_ne!(neg, 0u32);
    assert_ne!(neg, 0u128);
    assert_ne!(neg, 5u32);

    // Fractional vs unsigned.
    let frac = D38_2::from_bits(decimal_scaled::Int::<2>::from_i128(4_201));
    assert_ne!(frac, 42u32);
    assert_ne!(frac, 42u128);
}

#[test]
fn eq_narrow_signed_unsigned_int() {
    let v9 = D9_2::from_int(7);
    assert_eq!(v9, 7i8);
    assert_eq!(v9, 7u8);
    assert_eq!(v9, 7i32);
    assert_eq!(v9, 7u32);
    let neg9 = D9_2::from_int(-3);
    assert_ne!(neg9, 0u8);

    let v18 = D18_2::from_int(100);
    assert_eq!(v18, 100i16);
    assert_eq!(v18, 100u16);
    assert_eq!(v18, 100i64);
    assert_eq!(v18, 100u64);
    let neg18 = D18_2::from_int(-1);
    assert_ne!(neg18, 0u64);
    assert_ne!(neg18, 0u128);
}

#[cfg(feature = "wide")]
#[test]
fn eq_wide_int() {
    use decimal_scaled::D76;
    type D76_2 = D76<2>;
    let v: D76_2 = D38_2::from_int(42).into();
    assert_eq!(v, 42i32);
    assert_eq!(42i32, v);
    assert_eq!(v, 42u32);
    assert_eq!(v, 42i128);
    assert_eq!(v, 42u128);
    // Fractional
    let frac: D76_2 = D38_2::from_bits(decimal_scaled::Int::<2>::from_i128(4_201)).into();
    assert_ne!(frac, 42i32);
    assert_ne!(frac, 42u32);
    // Negative vs unsigned
    let neg: D76_2 = D38_2::from_int(-1).into();
    assert_ne!(neg, 0u32);
    assert_ne!(neg, 0u128);
}

#[cfg(feature = "std")]
#[test]
fn eq_d38_floats() {
    // Equal when the float round-trips through from_f64/to_f64 exactly.
    let v = D38_2::from_int(42);
    assert_eq!(v, 42.0_f64);
    assert_eq!(42.0_f64, v);
    assert_eq!(v, 42.0_f32);

    // NaN / infinities are always unequal.
    assert_ne!(v, f64::NAN);
    assert_ne!(v, f64::INFINITY);
    assert_ne!(v, f64::NEG_INFINITY);
    assert_ne!(v, f32::NAN);
}

// ─── macros/int_methods.rs: from_int / from_intN ───────────────────────

#[test]
fn from_int_narrow_signed() {
    assert_eq!(D9_2::from_int(7).to_bits(), 700);
    assert_eq!(D18_2::from_int(100).to_bits(), 10_000);
    assert_eq!(D38_2::from_int(42).to_bits(), 4_200);
    // negative
    assert_eq!(D38_2::from_int(-5).to_bits(), -500);
}

#[test]
fn from_primitive_paths_d38() {
    // D38 has impls for every primitive int type via decl_from_primitive.
    let _ = D38_2::from(7_i8);
    let _ = D38_2::from(7_i16);
    let _ = D38_2::from(7_i32);
    let _ = D38_2::from(7_i64);
    let _ = D38_2::from(7_u8);
    let _ = D38_2::from(7_u16);
    let _ = D38_2::from(7_u32);
    let _ = D38_2::from(7_u64);
    // i64 via from_int (D38's IntSrc is i64; i128 conversion is via TryFrom).
    assert_eq!(D38_2::from_int(0i64).to_bits(), 0);
    let _: D38_2 = i128::from(0i32).try_into().unwrap_or(D38_2::ZERO);
}

#[test]
fn from_primitive_paths_d9_d18() {
    let _ = D9_2::from(7_i8);
    let _ = D9_2::from(7_i16);
    let _ = D9_2::from(7_u8);
    let _ = D9_2::from(7_u16);
    let _ = D18_2::from(7_i8);
    let _ = D18_2::from(7_i16);
    let _ = D18_2::from(7_i32);
    let _ = D18_2::from(7_u8);
    let _ = D18_2::from(7_u16);
    let _ = D18_2::from(7_u32);
}

// ─── macros/overflow.rs: checked_*, wrapping_*, saturating_*, overflowing_* ─

#[test]
fn overflow_variants_add_d9_d18() {
    let a = D9_2::MAX;
    let b = D9_2::from_int(1);
    assert!(a.checked_add(b).is_none());
    assert_eq!(a.saturating_add(b), D9_2::MAX);
    let (_, ov) = a.overflowing_add(b);
    assert!(ov);
    // wrapping (just exercises the branch)
    let _ = a.wrapping_add(b);

    let a = D18_2::MAX;
    let b = D18_2::from_int(1);
    assert!(a.checked_add(b).is_none());
    assert_eq!(a.saturating_add(b), D18_2::MAX);
}

#[test]
fn overflow_variants_sub_d9_d18() {
    let a = D9_2::MIN;
    let b = D9_2::from_int(1);
    assert!(a.checked_sub(b).is_none());
    assert_eq!(a.saturating_sub(b), D9_2::MIN);
    let (_, ov) = a.overflowing_sub(b);
    assert!(ov);
    let _ = a.wrapping_sub(b);
}

#[test]
fn overflow_variants_neg_d9_d18() {
    // MIN is not representable as positive; negating overflows.
    assert!(D9_2::MIN.checked_neg().is_none());
    assert_eq!(D9_2::MIN.saturating_neg(), D9_2::MAX);
    let (_, ov) = D9_2::MIN.overflowing_neg();
    assert!(ov);
    let _ = D9_2::MIN.wrapping_neg();
    // Non-MIN case
    assert_eq!(D9_2::from_int(5).checked_neg().unwrap(), D9_2::from_int(-5));
}

#[test]
fn overflow_variants_mul_d9_d18() {
    let a = D9_2::MAX;
    let b = D9_2::from_int(2);
    assert!(a.checked_mul(b).is_none());
    assert_eq!(a.saturating_mul(b), D9_2::MAX);
    let (_, ov) = a.overflowing_mul(b);
    assert!(ov);
    let _ = a.wrapping_mul(b);

    // Negative * positive saturating goes to MIN
    let neg_max = D9_2::MIN;
    let big = D9_2::from_int(3);
    assert_eq!(neg_max.saturating_mul(big), D9_2::MIN);

    let a = D18_2::MAX;
    let b = D18_2::from_int(2);
    assert!(a.checked_mul(b).is_none());
}

#[test]
fn overflow_variants_div_rem_d9_d18() {
    let a = D9_2::from_int(7);
    let b = D9_2::from_int(2);
    let q = a.checked_div(b).unwrap();
    // 7.00 / 2.00 = 3.50 → 350 at S=2.
    assert_eq!(q.to_bits(), 350);
    // div by zero — checked_div returns None (matches i32::checked_div);
    // saturating_div by zero panics (covered by saturating_div_by_zero_panics).
    assert!(D9_2::from_int(7).checked_div(D9_2::ZERO).is_none());
    // overflowing_div(0) / wrapping_div(0) / wrapping_rem(0) on most
    // integer storage types panic (matches i32::overflowing_div), so we
    // don't exercise those paths here — they're well-known and the
    // checked_/saturating_ surface is what callers should use.
    // rem
    let r = a.checked_rem(b).unwrap();
    let _ = r;
    assert!(D9_2::from_int(7).checked_rem(D9_2::ZERO).is_none());

    // D18 variants
    let a = D18_2::from_int(7);
    let b = D18_2::from_int(2);
    let _ = a.checked_div(b).unwrap();
    let _ = a.checked_rem(b).unwrap();
}

#[cfg(feature = "wide")]
#[test]
fn overflow_variants_wide_d76() {
    use decimal_scaled::D76;
    type D76_2 = D76<2>;
    let a = D76_2::MAX;
    let b: D76_2 = D38_2::from_int(2).into();
    assert!(a.checked_mul(b).is_none());
    assert_eq!(a.saturating_mul(b), D76_2::MAX);
    let _ = a.wrapping_mul(b);
    let (_, ov) = a.overflowing_mul(b);
    assert!(ov);
    // Add overflow
    let one: D76_2 = D38_2::from_int(1).into();
    assert!(D76_2::MAX.checked_add(one).is_none());
    // Sub overflow
    assert!(D76_2::MIN.checked_sub(one).is_none());
    // Neg overflow
    assert!(D76_2::MIN.checked_neg().is_none());
    // Div by zero
    assert!(D76_2::MAX.checked_div(D76_2::ZERO).is_none());
    assert!(D76_2::MAX.checked_rem(D76_2::ZERO).is_none());
}

// ─── macros/num_traits.rs: Bounded / Zero / One / Num / Inv ────────────

#[test]
fn num_traits_zero_one_bounded() {
    use num_traits::{Bounded, One, Zero};
    assert_eq!(<D38_2 as Zero>::zero(), D38_2::ZERO);
    assert!(<D38_2 as Zero>::zero().is_zero());
    assert!(!D38_2::ONE.is_zero());
    assert_eq!(<D38_2 as One>::one(), D38_2::ONE);
    assert!(<D38_2 as One>::one().is_one());
    assert_eq!(<D38_2 as Bounded>::min_value(), D38_2::MIN);
    assert_eq!(<D38_2 as Bounded>::max_value(), D38_2::MAX);

    assert_eq!(<D9_2 as Zero>::zero(), D9_2::ZERO);
    assert_eq!(<D18_2 as Zero>::zero(), D18_2::ZERO);
    assert!(<D9_2 as Zero>::zero().is_zero());
    assert!(<D18_2 as One>::one().is_one());
    assert_eq!(<D9_2 as Bounded>::min_value(), D9_2::MIN);
    assert_eq!(<D18_2 as Bounded>::max_value(), D18_2::MAX);
}

#[cfg(feature = "wide")]
#[test]
fn num_traits_wide() {
    use decimal_scaled::D76;
    use num_traits::{Bounded, One, Zero};
    type D76_2 = D76<2>;
    assert!(<D76_2 as Zero>::zero().is_zero());
    assert!(<D76_2 as One>::one().is_one());
    assert_eq!(<D76_2 as Bounded>::min_value(), D76_2::MIN);
    assert_eq!(<D76_2 as Bounded>::max_value(), D76_2::MAX);
}

// ─── macros/bitwise.rs ─────────────────────────────────────────────────

#[test]
fn bitwise_ops_d9_d18() {
    use core::ops::*;
    let a = D9_2::from_bits(decimal_scaled::Int::<1>::from_i64(0b1100));
    let b = D9_2::from_bits(decimal_scaled::Int::<1>::from_i64(0b1010));
    assert_eq!((a & b).to_bits(), 0b1000);
    assert_eq!((a | b).to_bits(), 0b1110);
    assert_eq!((a ^ b).to_bits(), 0b0110);
    let mut c = a;
    c &= b;
    assert_eq!(c.to_bits(), 0b1000);
    let mut c = a;
    c |= b;
    assert_eq!(c.to_bits(), 0b1110);
    let mut c = a;
    c ^= b;
    assert_eq!(c.to_bits(), 0b0110);
    // Not
    assert_eq!((!D9_2::from_bits(decimal_scaled::Int::<1>::from_i64(0))).to_bits(), !0i64);
    // Shifts
    let s = D9_2::from_bits(decimal_scaled::Int::<1>::from_i64(1));
    assert_eq!((s.shl(3_u32)).to_bits(), 8);
    assert_eq!((D9_2::from_bits(decimal_scaled::Int::<1>::from_i64(16)).shr(2_u32)).to_bits(), 4);

    // D18
    let a = D18_2::from_bits(decimal_scaled::Int::<1>::from_i64(0b1100));
    let b = D18_2::from_bits(decimal_scaled::Int::<1>::from_i64(0b1010));
    assert_eq!((a & b).to_bits(), 0b1000);
    assert_eq!((a | b).to_bits(), 0b1110);
    assert_eq!((a ^ b).to_bits(), 0b0110);
}

// ─── macros/float_bridge.rs: from_f32 / to_f32 / from_f64 / to_f64 ─────

#[cfg(feature = "std")]
#[test]
fn float_bridge_narrow() {
    // f64
    assert_eq!(D38_2::from_f64(1.5).to_bits(), 150);
    assert_eq!(D38_2::ZERO.to_f64(), 0.0);
    // NaN saturates to ZERO
    assert_eq!(D38_2::from_f64(f64::NAN), D38_2::ZERO);
    // +inf saturates to MAX
    assert_eq!(D38_2::from_f64(f64::INFINITY), D38_2::MAX);
    // -inf saturates to MIN
    assert_eq!(D38_2::from_f64(f64::NEG_INFINITY), D38_2::MIN);

    // f64 bridge for narrow widths
    assert_eq!(D9_2::from_f64(1.5).to_bits(), 150);
    assert_eq!(D18_2::from_f64(1.5).to_bits(), 150);
    assert_eq!(D9_2::from_f64(2.5).to_bits(), 250);
    assert_eq!(D18_2::from_f64(2.5).to_bits(), 250);

    // Out-of-range saturation
    assert_eq!(D9_2::from_f64(f64::INFINITY), D9_2::MAX);
    assert_eq!(D9_2::from_f64(f64::NEG_INFINITY), D9_2::MIN);
    assert_eq!(D9_2::from_f64(f64::NAN), D9_2::ZERO);
    assert_eq!(D18_2::from_f64(f64::INFINITY), D18_2::MAX);

    // to_f32 / to_f64
    assert_eq!(D9_2::from_int(1).to_f32(), 1.0_f32);
    assert_eq!(D9_2::from_int(1).to_f64(), 1.0);
    assert_eq!(D18_2::from_int(1).to_f32(), 1.0_f32);
    assert_eq!(D18_2::from_int(1).to_f64(), 1.0);

    // from_f64_with: rounding-mode-aware variant
    use decimal_scaled::RoundingMode;
    let v = D38_2::from_f64_with(1.5, RoundingMode::HalfToEven);
    assert_eq!(v.to_bits(), 150);
    let v = D38_2::from_f64_with(1.5, RoundingMode::Trunc);
    assert_eq!(v.to_bits(), 150);
}

// ─── macros/conversions.rs: TryFrom narrowing ──────────────────────────

#[test]
fn try_from_narrowing_d38_to_d9_in_range() {
    // 5.00 fits D9_2.
    let v = D38_2::from_int(5);
    let r: D9_2 = v.try_into().unwrap();
    assert_eq!(r.to_bits(), 500);
}


#[test]
fn try_from_d38_to_d18_in_range() {
    let v = D38_2::from_int(5);
    let r: D18_2 = v.try_into().unwrap();
    assert_eq!(r.to_bits(), 500);
}


// ─── core_type.rs: Default + multipliers + raw constructors ────────────

#[test]
fn default_impls() {
    assert_eq!(D9_2::default(), D9_2::ZERO);
    assert_eq!(D18_2::default(), D18_2::ZERO);
    assert_eq!(D38_2::default(), D38_2::ZERO);
}

#[test]
fn from_bits_zero_one_max_min() {
    assert_eq!(D9_2::ZERO.to_bits(), 0);
    assert_eq!(D18_2::ZERO.to_bits(), 0);
    assert_eq!(D38_2::ZERO.to_bits(), 0);
    assert_eq!(D9_2::ONE.to_bits(), 100);
    assert_eq!(D18_2::ONE.to_bits(), 100);
    assert_eq!(D38_2::ONE.to_bits(), 100);
    assert!(D9_2::MAX > D9_2::ZERO);
    assert!(D18_2::MAX > D18_2::ZERO);
    assert!(D38_2::MAX > D38_2::ZERO);
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
    let a = D38_2::MAX;
    let b = D38_2::from_int(2);
    assert!(a.checked_mul(b).is_none());
    let (_v, ov) = a.overflowing_mul(b);
    assert!(ov);
    assert_eq!(a.saturating_mul(b), D38_2::MAX);
}

#[test]
fn d38_div_by_zero_overflow() {
    assert!(D38_2::ONE.checked_div(D38_2::ZERO).is_none());
    // overflowing_div(0) panics on most storage types (matches
    // i128::overflowing_div); checked_div is the safe surface.
}

#[test]
fn d38_add_sub_overflow() {
    assert!(D38_2::MAX.checked_add(D38_2::ONE).is_none());
    assert!(D38_2::MIN.checked_sub(D38_2::ONE).is_none());
    let (_, ov) = D38_2::MAX.overflowing_add(D38_2::ONE);
    assert!(ov);
    let (_, ov) = D38_2::MIN.overflowing_sub(D38_2::ONE);
    assert!(ov);
}

// ─── decimal_trait.rs ──────────────────────────────────────────────────

#[test]
fn decimal_trait_methods() {
    let v = D38_2::from_int(7);
    // scale() takes self
    assert_eq!(v.scale(), 2);
    // multiplier returns Storage type
    assert_eq!(<D38_2 as DecimalArithmetic>::multiplier(), 100_i128);
    // is_zero
    assert!(!v.is_zero());
    assert!(D38_2::ZERO.is_zero());
    // signum
    assert_eq!(v.signum(), D38_2::from_int(1));
    assert_eq!(D38_2::ZERO.signum(), D38_2::ZERO);
    assert_eq!(D38_2::from_int(-5).signum(), D38_2::from_int(-1));
}
