//! Edge-case regression suite.
//!
//! This file is the safety net for the optimisation work: it pins down
//! behaviour at the boundaries — `MIN` / `MAX`, the zero / one
//! identities, overflow edges, scale extremes, rounding-mode
//! boundaries, and the documented panic preconditions — so that a
//! later change that alters functionality is caught here rather than
//! in the field.
//!
//! Every assertion is exact (bit-level) unless a method is documented
//! lossy, in which case the relevant rounding contract is asserted.

use decimal_scaled::{
    D128, D128s0, D128s12, D128s2, D128s38, D128s9, D32s0, D32s2, D32s4, D32s9, D64s0, D64s18,
    D64s6, D64s9, Decimal, RoundingMode,
};

// ─────────────────────────────────────────────────────────────────────
// Constants and storage boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn zero_one_max_min_storage_patterns() {
    // ZERO is always raw 0; ONE is always raw 10^SCALE; MAX/MIN are the
    // storage type's own MAX/MIN.
    assert_eq!(D128s0::ZERO.to_bits(), 0);
    assert_eq!(D128s0::ONE.to_bits(), 1);
    assert_eq!(D128s12::ONE.to_bits(), 1_000_000_000_000);
    assert_eq!(D128s38::ONE.to_bits(), 10_i128.pow(38));
    assert_eq!(D128s12::MAX.to_bits(), i128::MAX);
    assert_eq!(D128s12::MIN.to_bits(), i128::MIN);

    assert_eq!(D32s0::ONE.to_bits(), 1);
    assert_eq!(D32s9::ONE.to_bits(), 10_i32.pow(9));
    assert_eq!(D32s0::MAX.to_bits(), i32::MAX);
    assert_eq!(D32s0::MIN.to_bits(), i32::MIN);

    assert_eq!(D64s18::ONE.to_bits(), 10_i64.pow(18));
    assert_eq!(D64s0::MAX.to_bits(), i64::MAX);
}

#[test]
fn multiplier_at_scale_extremes() {
    assert_eq!(D128s0::multiplier(), 1);
    assert_eq!(D128s38::multiplier(), 10_i128.pow(38));
    assert_eq!(D32s9::multiplier(), 10_i32.pow(9));
    assert_eq!(D64s18::multiplier(), 10_i64.pow(18));
}

#[test]
fn max_scale_per_width() {
    assert_eq!(<D32s0 as Decimal>::MAX_SCALE, 9);
    assert_eq!(<D64s0 as Decimal>::MAX_SCALE, 18);
    assert_eq!(<D128s0 as Decimal>::MAX_SCALE, 38);
}

// ─────────────────────────────────────────────────────────────────────
// Overflow boundaries — checked / wrapping / saturating / overflowing
// ─────────────────────────────────────────────────────────────────────

#[test]
fn add_at_max_boundary() {
    let one_lsb = D128s12::from_bits(1);
    assert_eq!(D128s12::MAX.checked_add(one_lsb), None);
    assert_eq!(D128s12::MAX.saturating_add(one_lsb), D128s12::MAX);
    assert_eq!(D128s12::MAX.wrapping_add(one_lsb), D128s12::MIN);
    assert_eq!(D128s12::MAX.overflowing_add(one_lsb), (D128s12::MIN, true));
    // One short of the boundary does not overflow.
    let near_max = D128s12::from_bits(i128::MAX - 1);
    assert_eq!(near_max.checked_add(one_lsb), Some(D128s12::MAX));
}

#[test]
fn sub_at_min_boundary() {
    let one_lsb = D128s12::from_bits(1);
    assert_eq!(D128s12::MIN.checked_sub(one_lsb), None);
    assert_eq!(D128s12::MIN.saturating_sub(one_lsb), D128s12::MIN);
    assert_eq!(D128s12::MIN.wrapping_sub(one_lsb), D128s12::MAX);
}

#[test]
fn neg_of_min_overflows() {
    // Two's-complement MIN has no positive counterpart.
    assert_eq!(D128s12::MIN.checked_neg(), None);
    assert_eq!(D128s12::MIN.saturating_neg(), D128s12::MAX);
    assert_eq!(D128s12::MIN.wrapping_neg(), D128s12::MIN);
    assert_eq!(D128s12::MIN.overflowing_neg(), (D128s12::MIN, true));
    // Neg of MAX is fine.
    assert_eq!(D128s12::MAX.checked_neg(), Some(D128s12::from_bits(i128::MIN + 1)));
}

#[test]
fn checked_div_by_zero_is_none() {
    assert_eq!(D128s12::ONE.checked_div(D128s12::ZERO), None);
    assert_eq!(D32s0::ONE.checked_div(D32s0::ZERO), None);
    assert_eq!(D64s0::ONE.checked_div(D64s0::ZERO), None);
}

#[test]
fn checked_mul_overflow_at_max() {
    let two = D128s12::from_int(2);
    assert_eq!(D128s12::MAX.checked_mul(two), None);
    // Saturating picks the sign-correct extreme.
    assert_eq!(D128s12::MAX.saturating_mul(two), D128s12::MAX);
    assert_eq!(D128s12::MIN.saturating_mul(two), D128s12::MIN);
}

#[test]
#[should_panic]
fn add_overflow_panics_in_debug() {
    // Debug builds panic on operator overflow (release wraps).
    let _ = D128s12::MAX + D128s12::from_bits(1);
}

#[test]
#[should_panic]
fn div_by_zero_operator_panics() {
    let _ = D128s12::ONE / D128s12::ZERO;
}

// ─────────────────────────────────────────────────────────────────────
// Rescale — scale extremes and every rounding mode at the tie
// ─────────────────────────────────────────────────────────────────────

#[test]
fn rescale_up_is_lossless_to_max_scale() {
    // 1 at scale 0 -> scale 38 is exactly 10^38 (which fits i128).
    // A larger integer would overflow when scaled up by 10^38.
    let v = D128s0::from_bits(1);
    let up: D128<38> = v.rescale::<38>();
    assert_eq!(up.to_bits(), 10_i128.pow(38));
}

#[test]
fn rescale_same_scale_is_identity() {
    let v = D128s12::from_bits(123_456_789_012);
    let same: D128s12 = v.rescale::<12>();
    assert_eq!(same.to_bits(), 123_456_789_012);
}

#[test]
fn rescale_down_every_mode_at_exact_half() {
    // 1.5 at scale 1 -> scale 0. The fractional digit is exactly the
    // half-way tie, so the modes diverge predictably.
    let half_tie = D128::<1>::from_bits(15); // 1.5
    let modes_and_bits = [
        (RoundingMode::HalfToEven, 2),       // ties to even (2)
        (RoundingMode::HalfAwayFromZero, 2), // ties away from zero
        (RoundingMode::HalfTowardZero, 1),   // ties toward zero
        (RoundingMode::Trunc, 1),            // toward zero
        (RoundingMode::Floor, 1),            // toward -inf
        (RoundingMode::Ceiling, 2),          // toward +inf
    ];
    for (mode, expected) in modes_and_bits {
        let r: D128<0> = half_tie.rescale_with::<0>(mode);
        assert_eq!(r.to_bits(), expected, "mode {mode:?}");
    }
    // 2.5 -> ties-to-even rounds down to 2 (2 is even).
    let other_tie = D128::<1>::from_bits(25);
    assert_eq!(other_tie.rescale_with::<0>(RoundingMode::HalfToEven).to_bits(), 2);
}

#[test]
fn rescale_down_negative_tie_is_sign_symmetric() {
    let neg_tie = D128::<1>::from_bits(-15); // -1.5
    assert_eq!(neg_tie.rescale_with::<0>(RoundingMode::HalfAwayFromZero).to_bits(), -2);
    assert_eq!(neg_tie.rescale_with::<0>(RoundingMode::HalfTowardZero).to_bits(), -1);
    assert_eq!(neg_tie.rescale_with::<0>(RoundingMode::Floor).to_bits(), -2);
    assert_eq!(neg_tie.rescale_with::<0>(RoundingMode::Ceiling).to_bits(), -1);
}

#[test]
#[should_panic]
fn rescale_up_overflow_panics() {
    // Scaling i128::MAX up by another 10^26 cannot fit.
    let _ = D128s12::MAX.rescale::<38>();
}

// ─────────────────────────────────────────────────────────────────────
// Rounding methods at boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn floor_ceil_round_trunc_fract_signs() {
    // -2.5 at scale 1.
    let neg = D128::<1>::from_bits(-25);
    assert_eq!(neg.floor().to_bits(), -30); // toward -inf
    assert_eq!(neg.ceil().to_bits(), -20); // toward +inf
    assert_eq!(neg.round().to_bits(), -30); // half away from zero
    assert_eq!(neg.trunc().to_bits(), -20); // toward zero
    assert_eq!(neg.fract().to_bits(), -5); // keeps sign of self

    // Exact integers are fixed points of all five.
    let exact = D128::<1>::from_bits(40); // 4.0
    assert_eq!(exact.floor(), exact);
    assert_eq!(exact.ceil(), exact);
    assert_eq!(exact.round(), exact);
    assert_eq!(exact.trunc(), exact);
    assert_eq!(exact.fract(), D128::<1>::ZERO);
}

// ─────────────────────────────────────────────────────────────────────
// Sign methods at boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn signum_abs_at_extremes() {
    assert_eq!(D128s12::ZERO.signum(), D128s12::ZERO);
    assert_eq!(D128s12::ONE.signum(), D128s12::ONE);
    assert_eq!((-D128s12::ONE).signum(), -D128s12::ONE);
    assert_eq!(D128s12::MAX.signum(), D128s12::ONE);
    assert_eq!(D128s12::MIN.signum(), -D128s12::ONE);
    // abs(MAX) is fine; abs(MIN) is the documented overflow case.
    assert_eq!(D128s12::MAX.abs(), D128s12::MAX);
}

#[test]
#[should_panic]
fn abs_of_min_panics_in_debug() {
    let _ = D128s12::MIN.abs();
}

#[test]
fn is_positive_is_negative_is_zero_partition() {
    // Every value is in exactly one of the three classes.
    for v in [
        D128s12::ZERO,
        D128s12::ONE,
        -D128s12::ONE,
        D128s12::MAX,
        D128s12::MIN,
        D128s12::from_bits(1),
        D128s12::from_bits(-1),
    ] {
        let p = v.is_positive() as u8;
        let n = v.is_negative() as u8;
        let z = v.is_zero() as u8;
        assert_eq!(p + n + z, 1, "value {v:?} must be in exactly one sign class");
    }
}

// ─────────────────────────────────────────────────────────────────────
// Conversions at boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn cross_width_narrowing_at_boundary() {
    // A D128 value exactly at i64::MAX narrows; one past it does not.
    let at_edge = D128s0::from_bits(i64::MAX as i128);
    let narrowed: Result<D64s0, _> = at_edge.try_into();
    assert!(narrowed.is_ok());

    let past_edge = D128s0::from_bits(i64::MAX as i128 + 1);
    let fail: Result<D64s0, _> = past_edge.try_into();
    assert!(fail.is_err());

    // Widening is always lossless.
    let small = D32s0::from_bits(i32::MIN);
    let wide: D128s0 = small.into();
    assert_eq!(wide.to_bits(), i32::MIN as i128);
}

#[test]
fn try_from_i128_overflow_boundary() {
    // At scale 12, the largest exact integer is i128::MAX / 10^12.
    let max_int = i128::MAX / 10_i128.pow(12);
    assert!(D128s12::try_from(max_int).is_ok());
    assert!(D128s12::try_from(max_int + 1).is_err());
}

#[test]
fn to_int_lossy_saturates_and_rounds() {
    // 2.5 at scale 1 rounds per mode; the integer part saturates to i64.
    let v = D128::<1>::from_bits(25); // 2.5
    assert_eq!(v.to_int_lossy_with(RoundingMode::HalfToEven), 2);
    assert_eq!(v.to_int_lossy_with(RoundingMode::HalfAwayFromZero), 3);
    // A value whose integer part exceeds i64 saturates.
    let huge = D128s0::MAX;
    assert_eq!(huge.to_int_lossy(), i64::MAX);
    let tiny = D128s0::MIN;
    assert_eq!(tiny.to_int_lossy(), i64::MIN);
}

// ─────────────────────────────────────────────────────────────────────
// Identities that must hold for every width
// ─────────────────────────────────────────────────────────────────────

#[test]
fn additive_and_multiplicative_identities() {
    macro_rules! check {
        ($t:ty) => {{
            let one = <$t>::ONE;
            let zero = <$t>::ZERO;
            // A small raw value — large enough to be non-trivial, small
            // enough that `v * one` (which widens through the scale
            // factor) cannot overflow even at MAX_SCALE.
            let v = <$t>::from_bits(7);
            assert_eq!(v + zero, v);
            assert_eq!(v - zero, v);
            assert_eq!(v * one, v);
            assert_eq!(v / one, v);
            assert_eq!(zero - v, -v);
            assert_eq!(v - v, zero);
        }};
    }
    check!(D32s9);
    check!(D64s18);
    check!(D128s12);
    check!(D128s38);
}

#[test]
fn from_bits_to_bits_round_trips_at_extremes() {
    for raw in [0_i128, 1, -1, i128::MAX, i128::MIN, i128::MAX - 1, i128::MIN + 1] {
        assert_eq!(D128s12::from_bits(raw).to_bits(), raw);
    }
    for raw in [0_i32, 1, -1, i32::MAX, i32::MIN] {
        assert_eq!(D32s0::from_bits(raw).to_bits(), raw);
    }
}

// ─────────────────────────────────────────────────────────────────────
// Part 2 — wider permutation coverage: cross-width round-trips, the
// rounding-method / overflow-variant families across every width, and
// the wide tier (feature-gated).
// ─────────────────────────────────────────────────────────────────────

#[test]
fn cross_width_widen_then_narrow_round_trips() {
    // Widening is lossless; narrowing back recovers the original when
    // the value is in range.
    let v32 = D32s2::from_bits(-12_345);
    let wide: D128s2 = v32.into();
    let back: D32s2 = wide.try_into().unwrap();
    assert_eq!(back, v32);

    let v64 = D64s9::from_bits(i64::MIN + 1);
    let wide: D128s9 = v64.into();
    let back: D64s9 = wide.try_into().unwrap();
    assert_eq!(back, v64);
}

#[test]
fn overflow_variants_consistency_across_widths() {
    // checked / wrapping / overflowing must agree: overflowing's value
    // equals wrapping's, and its bool equals checked.is_none().
    macro_rules! check {
        ($t:ty) => {{
            let max = <$t>::MAX;
            let one = <$t>::from_bits(1);
            let (wrapped, did) = max.overflowing_add(one);
            assert_eq!(wrapped, max.wrapping_add(one));
            assert_eq!(did, max.checked_add(one).is_none());
            // No-overflow case: all three agree on the plain result.
            let a = <$t>::from_bits(10);
            let b = <$t>::from_bits(20);
            assert_eq!(a.checked_add(b), Some(a.wrapping_add(b)));
            assert_eq!(a.overflowing_add(b), (a.wrapping_add(b), false));
        }};
    }
    check!(D32s4);
    check!(D64s9);
    check!(D128s12);
}

#[test]
fn rounding_methods_on_every_width() {
    // floor <= trunc-toward-zero behaviour and fract sign, on each width.
    macro_rules! check {
        ($t:ty, $half:expr) => {{
            // $half is the raw storage for 2.5 at the type's scale.
            let v = <$t>::from_bits($half);
            // 2.5: floor 2, ceil 3, round 3 (half away), trunc 2.
            let m = <$t>::multiplier();
            assert_eq!(v.floor().to_bits(), 2 * m);
            assert_eq!(v.ceil().to_bits(), 3 * m);
            assert_eq!(v.round().to_bits(), 3 * m);
            assert_eq!(v.trunc().to_bits(), 2 * m);
            // fract keeps the sign and is self - trunc.
            assert_eq!(v.fract(), v - v.trunc());
            let n = -v;
            assert_eq!(n.fract(), n - n.trunc());
        }};
    }
    check!(D32s2, 250);
    check!(D64s6, 2_500_000);
    check!(D128s12, 2_500_000_000_000);
}

#[test]
fn div_euclid_rem_euclid_invariant() {
    // The Euclidean identity: a == b * div_euclid(a,b) + rem_euclid(a,b),
    // and the remainder is non-negative.
    for &(a, b) in &[
        (7_i128, 3_i128),
        (-7, 3),
        (7, -3),
        (-7, -3),
        (123_456_789, 1_000),
    ] {
        let da = D128s0::from_bits(a);
        let db = D128s0::from_bits(b);
        let q = da.div_euclid(db);
        let r = da.rem_euclid(db);
        // q is an integer multiple of ONE; r in [0, |b|).
        assert!(!r.is_negative(), "rem_euclid({a},{b}) negative");
        // b * (q / ONE) + r == a
        let q_int = q.to_bits() / D128s0::multiplier();
        assert_eq!(b * q_int + r.to_bits(), a, "euclid identity ({a},{b})");
    }
}

#[test]
fn midpoint_is_overflow_free_at_extremes() {
    // midpoint must not overflow even at MAX/MIN — the whole point of
    // the branch-free identity.
    assert_eq!(D128s0::MAX.midpoint(D128s0::MAX), D128s0::MAX);
    assert_eq!(D128s0::MIN.midpoint(D128s0::MIN), D128s0::MIN);
    // midpoint(MAX, MIN) is -1 raw (floor of -0.5) — no overflow.
    let mid = D128s0::MAX.midpoint(D128s0::MIN);
    assert_eq!(mid.to_bits(), -1);
}

#[test]
fn from_str_round_trips_display_at_scale_extremes() {
    use core::str::FromStr;
    // A value with the maximum fractional digits round-trips exactly.
    let s = "1.23456789012345678901234567890123456789"; // 38 frac digits
    let v = D128s38::from_str(s).unwrap();
    assert_eq!(format!("{v}"), s);
    // Scale 0 has no fractional part.
    let v0 = D128s0::from_str("-42").unwrap();
    assert_eq!(format!("{v0}"), "-42");
    assert_eq!(v0.to_bits(), -42);
}

#[test]
fn bitwise_storage_semantics() {
    // Bitwise ops act on the raw storage, not the logical value.
    let a = D128s12::from_bits(0b1100);
    let b = D128s12::from_bits(0b1010);
    assert_eq!((a & b).to_bits(), 0b1000);
    assert_eq!((a | b).to_bits(), 0b1110);
    assert_eq!((a ^ b).to_bits(), 0b0110);
    assert_eq!((!D128s12::ZERO).to_bits(), -1);
    // Arithmetic `>>` is sign-extending: -8 >> 1 == -4.
    assert_eq!((D128s12::from_bits(-8) >> 1u32).to_bits(), -4);
    // `unsigned_shr` is logical: -8 (all-ones tail) viewed as u128 is
    // 2^128 - 8; >> 1 is 2^127 - 4, which fits i128 as i128::MAX - 3.
    assert_eq!(
        D128s12::from_bits(-8).unsigned_shr(1).to_bits(),
        i128::MAX - 3
    );
}

#[test]
fn float_shape_predicates_are_total() {
    // is_nan / is_infinite are always false; is_finite always true; a
    // fixed-point decimal has no special values.
    for v in [D128s12::ZERO, D128s12::ONE, D128s12::MAX, D128s12::MIN, -D128s12::ONE] {
        assert!(!v.is_nan());
        assert!(!v.is_infinite());
        assert!(v.is_finite());
        assert_eq!(v.is_normal(), !v.is_zero());
    }
}

// ── Wide tier (feature-gated) ────────────────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn wide_tier_boundaries() {
    use decimal_scaled::{D256, D256s0, D512s0, D1024s0};
    // ONE / MAX / MIN round-trip through to_bits / from_bits.
    let one = D256s0::ONE;
    assert_eq!(D256::<0>::from_bits(one.to_bits()), one);
    // Arithmetic identities hold on the wide tier.
    let v = D256s0::from_int(7i128);
    assert_eq!(v + D256s0::ZERO, v);
    assert_eq!(v * D256s0::ONE, v);
    assert_eq!(v - v, D256s0::ZERO);
    // Overflow at the wide MAX is still checked.
    assert_eq!(D256s0::MAX.checked_add(D256s0::ONE), None);
    assert_eq!(D512s0::MAX.checked_add(D512s0::ONE), None);
    assert_eq!(D1024s0::MAX.checked_add(D1024s0::ONE), None);
    // Cross-tier widen/narrow round-trip.
    let mid: D256s0 = D128s0::from_bits(123_456).into();
    let back: D128s0 = mid.try_into().unwrap();
    assert_eq!(back.to_bits(), 123_456);
}
