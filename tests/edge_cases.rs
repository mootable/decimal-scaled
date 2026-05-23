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

use decimal_scaled::DecimalArithmetic;
use decimal_scaled::{
    D18s0, D18s6, D18s9, D18s17, D38, D38s0, D38s2, D38s9, D38s12, D38s37,
    RoundingMode,
};

// ─────────────────────────────────────────────────────────────────────
// Constants and storage boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn zero_one_max_min_storage_patterns() {
    // ZERO is always raw 0; ONE is always raw 10^SCALE; MAX/MIN are the
    // storage type's own MAX/MIN.
    assert_eq!(D38s0::ZERO.to_bits(), 0);
    assert_eq!(D38s0::ONE.to_bits(), 1);
    assert_eq!(D38s12::ONE.to_bits(), 1_000_000_000_000);
    assert_eq!(D38s37::ONE.to_bits(), 10_i128.pow(37));
    assert_eq!(D38s12::MAX.to_bits(), i128::MAX);
    assert_eq!(D38s12::MIN.to_bits(), i128::MIN);


    assert_eq!(D18s17::ONE.to_bits(), 10_i64.pow(17));
    assert_eq!(D18s0::MAX.to_bits(), i64::MAX);
}

#[test]
fn multiplier_at_scale_extremes() {
    assert_eq!(D38s0::multiplier(), 1);
    assert_eq!(D38s37::multiplier(), 10_i128.pow(37));
    assert_eq!(D18s17::multiplier(), 10_i64.pow(17));
}

#[test]
fn max_scale_per_width() {
    // v0.4.0 cap: MAX_SCALE = name - 1 (guarantees at least one integer
    // digit at every legal SCALE).
    assert_eq!(<D18s0 as DecimalArithmetic>::MAX_SCALE, 17);
    assert_eq!(<D38s0 as DecimalArithmetic>::MAX_SCALE, 37);
}

// ─────────────────────────────────────────────────────────────────────
// Overflow boundaries — checked / wrapping / saturating / overflowing
// ─────────────────────────────────────────────────────────────────────

#[test]
fn add_at_max_boundary() {
    let one_lsb = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap());
    assert_eq!(D38s12::MAX.checked_add(one_lsb), None);
    assert_eq!(D38s12::MAX.saturating_add(one_lsb), D38s12::MAX);
    assert_eq!(D38s12::MAX.wrapping_add(one_lsb), D38s12::MIN);
    assert_eq!(D38s12::MAX.overflowing_add(one_lsb), (D38s12::MIN, true));
    // One short of the boundary does not overflow.
    let near_max = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((i128::MAX - 1) as i128).unwrap());
    assert_eq!(near_max.checked_add(one_lsb), Some(D38s12::MAX));
}

#[test]
fn sub_at_min_boundary() {
    let one_lsb = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap());
    assert_eq!(D38s12::MIN.checked_sub(one_lsb), None);
    assert_eq!(D38s12::MIN.saturating_sub(one_lsb), D38s12::MIN);
    assert_eq!(D38s12::MIN.wrapping_sub(one_lsb), D38s12::MAX);
}

#[test]
fn neg_of_min_overflows() {
    // Two's-complement MIN has no positive counterpart.
    assert_eq!(D38s12::MIN.checked_neg(), None);
    assert_eq!(D38s12::MIN.saturating_neg(), D38s12::MAX);
    assert_eq!(D38s12::MIN.wrapping_neg(), D38s12::MIN);
    assert_eq!(D38s12::MIN.overflowing_neg(), (D38s12::MIN, true));
    // Neg of MAX is fine.
    assert_eq!(
        D38s12::MAX.checked_neg(),
        Some(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((i128::MIN + 1) as i128).unwrap()))
    );
}

#[test]
fn checked_div_by_zero_is_none() {
    assert_eq!(D38s12::ONE.checked_div(D38s12::ZERO), None);
    assert_eq!(D18s0::ONE.checked_div(D18s0::ZERO), None);
}

#[test]
fn checked_mul_overflow_at_max() {
    let two = D38s12::from_int(2);
    assert_eq!(D38s12::MAX.checked_mul(two), None);
    // Saturating picks the sign-correct extreme.
    assert_eq!(D38s12::MAX.saturating_mul(two), D38s12::MAX);
    assert_eq!(D38s12::MIN.saturating_mul(two), D38s12::MIN);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn add_overflow_panics_in_debug() {
    // Debug builds panic on operator overflow (release wraps).
    let _ = D38s12::MAX + D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap());
}

#[test]
#[should_panic]
fn div_by_zero_operator_panics() {
    let _ = D38s12::ONE / D38s12::ZERO;
}

// ─────────────────────────────────────────────────────────────────────
// Rescale — scale extremes and every rounding mode at the tie
// ─────────────────────────────────────────────────────────────────────

#[test]
fn rescale_up_is_lossless_to_max_scale() {
    // 1 at scale 0 -> scale 37 (new MAX_SCALE) is exactly 10^37
    // (which fits i128). A larger integer would overflow when scaled
    // up by 10^37.
    let v = D38s0::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap());
    let up: D38<37> = v.rescale::<37>();
    assert_eq!(up.to_bits(), 10_i128.pow(37));
}

#[test]
fn rescale_same_scale_is_identity() {
    let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((123_456_789_012) as i128).unwrap());
    let same: D38s12 = v.rescale::<12>();
    assert_eq!(same.to_bits(), 123_456_789_012);
}

#[test]
fn rescale_down_every_mode_at_exact_half() {
    // 1.5 at scale 1 -> scale 0. The fractional digit is exactly the
    // half-way tie, so the modes diverge predictably.
    let half_tie = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((15) as i128).unwrap()); // 1.5
    let modes_and_bits = [
        (RoundingMode::HalfToEven, 2),       // ties to even (2)
        (RoundingMode::HalfAwayFromZero, 2), // ties away from zero
        (RoundingMode::HalfTowardZero, 1),   // ties toward zero
        (RoundingMode::Trunc, 1),            // toward zero
        (RoundingMode::Floor, 1),            // toward -inf
        (RoundingMode::Ceiling, 2),          // toward +inf
    ];
    for (mode, expected) in modes_and_bits {
        let r: D38<0> = half_tie.rescale_with::<0>(mode);
        assert_eq!(r.to_bits(), expected, "mode {mode:?}");
    }
    // 2.5 -> ties-to-even rounds down to 2 (2 is even).
    let other_tie = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((25) as i128).unwrap());
    assert_eq!(
        other_tie
            .rescale_with::<0>(RoundingMode::HalfToEven)
            .to_bits(),
        2
    );
}

#[test]
fn rescale_down_negative_tie_is_sign_symmetric() {
    let neg_tie = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((-15) as i128).unwrap()); // -1.5
    assert_eq!(
        neg_tie
            .rescale_with::<0>(RoundingMode::HalfAwayFromZero)
            .to_bits(),
        -2
    );
    assert_eq!(
        neg_tie
            .rescale_with::<0>(RoundingMode::HalfTowardZero)
            .to_bits(),
        -1
    );
    assert_eq!(neg_tie.rescale_with::<0>(RoundingMode::Floor).to_bits(), -2);
    assert_eq!(
        neg_tie.rescale_with::<0>(RoundingMode::Ceiling).to_bits(),
        -1
    );
}

#[test]
#[should_panic]
fn rescale_up_overflow_panics() {
    // Scaling i128::MAX up by another 10^26 cannot fit.
    let _ = D38s12::MAX.rescale::<38>();
}

// ─────────────────────────────────────────────────────────────────────
// Rounding methods at boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn floor_ceil_round_trunc_fract_signs() {
    // -2.5 at scale 1.
    let neg = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((-25) as i128).unwrap());
    assert_eq!(neg.floor().to_bits(), -30); // toward -inf
    assert_eq!(neg.ceil().to_bits(), -20); // toward +inf
    assert_eq!(neg.round().to_bits(), -30); // half away from zero
    assert_eq!(neg.trunc().to_bits(), -20); // toward zero
    assert_eq!(neg.fract().to_bits(), -5); // keeps sign of self

    // Exact integers are fixed points of all five.
    let exact = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((40) as i128).unwrap()); // 4.0
    assert_eq!(exact.floor(), exact);
    assert_eq!(exact.ceil(), exact);
    assert_eq!(exact.round(), exact);
    assert_eq!(exact.trunc(), exact);
    assert_eq!(exact.fract(), D38::<1>::ZERO);
}

// ─────────────────────────────────────────────────────────────────────
// Sign methods at boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn signum_abs_at_extremes() {
    assert_eq!(D38s12::ZERO.signum(), D38s12::ZERO);
    assert_eq!(D38s12::ONE.signum(), D38s12::ONE);
    assert_eq!((-D38s12::ONE).signum(), -D38s12::ONE);
    assert_eq!(D38s12::MAX.signum(), D38s12::ONE);
    assert_eq!(D38s12::MIN.signum(), -D38s12::ONE);
    // abs(MAX) is fine; abs(MIN) is the documented overflow case.
    assert_eq!(D38s12::MAX.abs(), D38s12::MAX);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn abs_of_min_panics_in_debug() {
    let _ = D38s12::MIN.abs();
}

#[test]
fn is_positive_is_negative_is_zero_partition() {
    // Every value is in exactly one of the three classes.
    for v in [
        D38s12::ZERO,
        D38s12::ONE,
        -D38s12::ONE,
        D38s12::MAX,
        D38s12::MIN,
        D38s12::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()),
        D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap()),
    ] {
        let p = v.is_positive() as u8;
        let n = v.is_negative() as u8;
        let z = v.is_zero() as u8;
        assert_eq!(
            p + n + z,
            1,
            "value {v:?} must be in exactly one sign class"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────
// Conversions at boundaries
// ─────────────────────────────────────────────────────────────────────

#[test]
fn cross_width_narrowing_at_boundary() {
    // A D38 value exactly at i64::MAX narrows; one past it does not.
    let at_edge = D38s0::from_bits(decimal_scaled::Int::<2>::try_from((i64::MAX as i128) as i128).unwrap());
    let narrowed: Result<D18s0, _> = at_edge.try_into();
    assert!(narrowed.is_ok());

    let past_edge = D38s0::from_bits(decimal_scaled::Int::<2>::try_from((i64::MAX as i128 + 1) as i128).unwrap());
    let fail: Result<D18s0, _> = past_edge.try_into();
    assert!(fail.is_err());

}

#[test]
fn try_from_i128_overflow_boundary() {
    // At scale 12, the largest exact integer is i128::MAX / 10^12.
    let max_int = i128::MAX / 10_i128.pow(12);
    assert!(D38s12::try_from(max_int).is_ok());
    assert!(D38s12::try_from(max_int + 1).is_err());
}

#[test]
fn to_int_lossy_saturates_and_rounds() {
    // 2.5 at scale 1 rounds per mode; the integer part saturates to i64.
    let v = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((25) as i128).unwrap()); // 2.5
    assert_eq!(v.to_int_with(RoundingMode::HalfToEven), 2);
    assert_eq!(v.to_int_with(RoundingMode::HalfAwayFromZero), 3);
    // A value whose integer part exceeds i64 saturates.
    let huge = D38s0::MAX;
    assert_eq!(huge.to_int(), i64::MAX);
    let tiny = D38s0::MIN;
    assert_eq!(tiny.to_int(), i64::MIN);
}

// ─────────────────────────────────────────────────────────────────────
// Identities that must hold for every width
// ─────────────────────────────────────────────────────────────────────

#[test]
fn additive_and_multiplicative_identities() {
    macro_rules! check {
        // `$seven` is the raw storage value `7` typed for `$t`'s storage
        // (a primitive for D9/D18, `Int<2>` for D38) — `from_bits` takes the
        // storage type, which differs per width.
        ($t:ty, $seven:expr) => {{
            let one = <$t>::ONE;
            let zero = <$t>::ZERO;
            // A small raw value — large enough to be non-trivial, small
            // enough that `v * one` (which widens through the scale
            // factor) cannot overflow even at MAX_SCALE.
            let v = <$t>::from_bits($seven);
            assert_eq!(v + zero, v);
            assert_eq!(v - zero, v);
            assert_eq!(v * one, v);
            assert_eq!(v / one, v);
            assert_eq!(zero - v, -v);
            assert_eq!(v - v, zero);
        }};
    }
    check!(D18s17, decimal_scaled::Int::<1>::from((7) as i64));
    check!(D38s12, decimal_scaled::Int::<2>::try_from((7) as i128).unwrap());
    check!(D38s37, decimal_scaled::Int::<2>::try_from((7) as i128).unwrap());
}

#[test]
fn from_bits_to_bits_round_trips_at_extremes() {
    for raw in [
        0_i128,
        1,
        -1,
        i128::MAX,
        i128::MIN,
        i128::MAX - 1,
        i128::MIN + 1,
    ] {
        assert_eq!(D38s12::from_bits(decimal_scaled::Int::<2>::try_from((raw) as i128).unwrap()).to_bits(), raw);
    }
    for raw in [0_i32, 1, -1, i32::MAX, i32::MIN] {
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
    let v64 = D18s9::from_bits(decimal_scaled::Int::<1>::from((i64::MIN + 1) as i64));
    let wide: D38s9 = v64.into();
    let back: D18s9 = wide.try_into().unwrap();
    assert_eq!(back, v64);
}

#[test]
fn overflow_variants_consistency_across_widths() {
    // checked / wrapping / overflowing must agree: overflowing's value
    // equals wrapping's, and its bool equals checked.is_none().
    macro_rules! check {
        // `$mk` converts an `i128` to `$t`'s raw storage type (primitive
        // for D9/D18, `Int<2>` for D38), since `from_bits` is storage-typed.
        ($t:ty, $mk:expr) => {{
            let mk = $mk;
            let max = <$t>::MAX;
            let one = <$t>::from_bits(mk(1));
            let (wrapped, did) = max.overflowing_add(one);
            assert_eq!(wrapped, max.wrapping_add(one));
            assert_eq!(did, max.checked_add(one).is_none());
            // No-overflow case: all three agree on the plain result.
            let a = <$t>::from_bits(mk(10));
            let b = <$t>::from_bits(mk(20));
            assert_eq!(a.checked_add(b), Some(a.wrapping_add(b)));
            assert_eq!(a.overflowing_add(b), (a.wrapping_add(b), false));
        }};
    }
    check!(D18s9, |x: i128| decimal_scaled::Int::<1>::from((x as i64) as i64));
    check!(D38s12, |x: i128| decimal_scaled::Int::<2>::try_from((x) as i128).unwrap());
}

#[test]
fn rounding_methods_on_every_width() {
    // floor <= trunc-toward-zero behaviour and fract sign, on each width.
    macro_rules! check {
        // `$half` is the raw storage for 2.5 at the type's scale (typed
        // for the width's storage — primitive for D9/D18, `Int<2>` for D38).
        ($t:ty, $half:expr) => {{
            let v = <$t>::from_bits($half);
            // 2.5: floor 2, ceil 3, round 3 (half away), trunc 2. Compared
            // as decimal values (storage-agnostic) rather than raw-bits
            // arithmetic, since `multiplier()` differs in type per width.
            assert_eq!(v.floor(), <$t>::from_int(2));
            assert_eq!(v.ceil(), <$t>::from_int(3));
            assert_eq!(v.round(), <$t>::from_int(3));
            assert_eq!(v.trunc(), <$t>::from_int(2));
            // fract keeps the sign and is self - trunc.
            assert_eq!(v.fract(), v - v.trunc());
            let n = -v;
            assert_eq!(n.fract(), n - n.trunc());
        }};
    }
    check!(D18s6, decimal_scaled::Int::<1>::from((2_500_000) as i64));
    check!(D38s12, decimal_scaled::Int::<2>::try_from((2_500_000_000_000) as i128).unwrap());
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
        let da = D38s0::from_bits(decimal_scaled::Int::<2>::try_from((a) as i128).unwrap());
        let db = D38s0::from_bits(decimal_scaled::Int::<2>::try_from((b) as i128).unwrap());
        let q = da.div_euclid(db);
        let r = da.rem_euclid(db);
        // q is an integer multiple of ONE; r in [0, |b|).
        assert!(!r.is_negative(), "rem_euclid({a},{b}) negative");
        // b * (q / ONE) + r == a  (compare in i128; a, b are i128 seeds)
        let q_int = i128::from(q.to_bits()) / i128::from(D38s0::multiplier());
        assert_eq!(b * q_int + i128::from(r.to_bits()), a, "euclid identity ({a},{b})");
    }
}

#[test]
fn midpoint_is_overflow_free_at_extremes() {
    // midpoint must not overflow even at MAX/MIN — the whole point of
    // the branch-free identity.
    assert_eq!(D38s0::MAX.midpoint(D38s0::MAX), D38s0::MAX);
    assert_eq!(D38s0::MIN.midpoint(D38s0::MIN), D38s0::MIN);
    // midpoint(MAX, MIN) is -1 raw (floor of -0.5) — no overflow.
    let mid = D38s0::MAX.midpoint(D38s0::MIN);
    assert_eq!(mid.to_bits(), -1);
}

#[test]
fn from_str_round_trips_display_at_scale_extremes() {
    use core::str::FromStr;
    // A value with the maximum fractional digits round-trips exactly
    // (v0.4.0 cap: MAX_SCALE for D38 is 37, so 37 frac digits).
    let s = "1.2345678901234567890123456789012345678"; // 37 frac digits
    let v = D38s37::from_str(s).unwrap();
    assert_eq!(format!("{v}"), s);
    // Scale 0 has no fractional part.
    let v0 = D38s0::from_str("-42").unwrap();
    assert_eq!(format!("{v0}"), "-42");
    assert_eq!(v0.to_bits(), -42);
}

#[test]
fn bitwise_storage_semantics() {
    // Bitwise ops act on the raw storage, not the logical value.
    let a = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((0b1100) as i128).unwrap());
    let b = D38s12::from_bits(decimal_scaled::Int::<2>::try_from((0b1010) as i128).unwrap());
    assert_eq!((a & b).to_bits(), 0b1000);
    assert_eq!((a | b).to_bits(), 0b1110);
    assert_eq!((a ^ b).to_bits(), 0b0110);
    assert_eq!((!D38s12::ZERO).to_bits(), -1);
    // Arithmetic `>>` is sign-extending: -8 >> 1 == -4.
    assert_eq!((D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-8) as i128).unwrap()) >> 1u32).to_bits(), -4);
    // `unsigned_shr` is logical: -8 (all-ones tail) viewed as u128 is
    // 2^128 - 8; >> 1 is 2^127 - 4, which fits i128 as i128::MAX - 3.
    assert_eq!(
        D38s12::from_bits(decimal_scaled::Int::<2>::try_from((-8) as i128).unwrap()).unsigned_shr(1).to_bits(),
        i128::MAX - 3
    );
}

#[test]
fn float_shape_predicates_are_total() {
    // is_nan / is_infinite are always false; is_finite always true; a
    // fixed-point decimal has no special values.
    for v in [
        D38s12::ZERO,
        D38s12::ONE,
        D38s12::MAX,
        D38s12::MIN,
        -D38s12::ONE,
    ] {
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
    use decimal_scaled::{D76, D76s0, D153s0, D307s0};
    // ONE / MAX / MIN round-trip through to_bits / from_bits.
    let one = D76s0::ONE;
    assert_eq!(D76::<0>::from_bits(one.to_bits()), one);
    // Arithmetic identities hold on the wide tier.
    let v = D76s0::from_int(7i128);
    assert_eq!(v + D76s0::ZERO, v);
    assert_eq!(v * D76s0::ONE, v);
    assert_eq!(v - v, D76s0::ZERO);
    // Overflow at the wide MAX is still checked.
    assert_eq!(D76s0::MAX.checked_add(D76s0::ONE), None);
    assert_eq!(D153s0::MAX.checked_add(D153s0::ONE), None);
    assert_eq!(D307s0::MAX.checked_add(D307s0::ONE), None);
    // Cross-tier widen/narrow round-trip.
    let mid: D76s0 = D38s0::from_bits(decimal_scaled::Int::<2>::try_from((123_456) as i128).unwrap()).into();
    let back: D38s0 = mid.try_into().unwrap();
    assert_eq!(back.to_bits(), 123_456);
}
