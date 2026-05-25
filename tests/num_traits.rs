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
//! - [`num_traits::Zero`] / [`num_traits::One`] — additive and
//! multiplicative identities.
//! - [`num_traits::Num`] — umbrella numeric trait combining
//! `Zero + One + PartialEq + Add + Sub + Mul + Div + Rem` with a
//! `from_str_radix` constructor.
//! - [`num_traits::Bounded`] — `min_value()` / `max_value()` for
//! generic clamping code.
//! - [`num_traits::Signed`] — `abs`, `signum`, `is_positive`,
//! `is_negative`, `abs_sub`.
//! - [`num_traits::FromPrimitive`] / [`num_traits::ToPrimitive`] —
//! fallible conversions to and from the primitive numeric types.
//! - [`num_traits::CheckedAdd`] / [`num_traits::CheckedSub`] /
//! [`num_traits::CheckedMul`] / [`num_traits::CheckedDiv`] /
//! [`num_traits::CheckedRem`] / [`num_traits::CheckedNeg`] —
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
use num_traits::{
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
    // i128::MAX cannot be scaled by 10^12 — TryFrom returns Err,
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
    // u128::MAX > i128::MAX — the first try_from step fails.
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
        // divide by zero — both must return None
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
// Cross-scale exercise — non-default SCALE
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
    // 2^54 = 18_014_398_509_481_984 — above f64's exact-integer range.
    let v: i64 = 1_i64 << 54;
    let d: D38s12 = <D38s12 as NumCast>::from(v).expect("in-range");
    assert_eq!(<D38s12 as ToPrimitive>::to_i64(&d), Some(v));
}
