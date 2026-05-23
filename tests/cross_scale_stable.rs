//! Integration tests for the stable cross-scale-op API
//! (`mul_of`, `add_of`, `sub_of`, `div_of`, `rem_of`, `max_of`,
//! `min_of`, `clamp_of`, `cmp_of`, `eq_of`, `ne_of`, `lt_of`,
//! `le_of`, `gt_of`, `ge_of`).
//!
//! Tests cover:
//! - Same-width, cross-scale combinations (foundation case).
//! - Cross-width, cross-scale combinations.
//! - All five constructor ops + `_with(mode)` rounding sweep.
//! - Overflow panics on the `mul_of` family.
//! - Comparator pairwise consistency (cmp_of vs eq_of/lt_of/etc.).

use decimal_scaled::{
    D18, D18s4, D18s6, D18s9, D38, D38s6, D38s9, D38s12, D38s18, RoundingMode,
};

// ── Same-width, cross-scale: D38 arithmetic. ─────────────────────────

#[test]
fn d38_mul_of_same_width_cross_scale() {
    let a = D38s6::try_from(2).unwrap(); // 2.000000 (SCALE = 6)
    let b = D38s12::try_from(3).unwrap(); // 3.000000000000 (SCALE = 12)
    // mul_of at target SCALE = 9: 2 × 3 = 6.000000000
    let c: D38s9 = D38s9::mul_of(a, b);
    assert_eq!(c, D38s9::try_from(6).unwrap());
}

#[test]
fn d38_add_of_same_width_cross_scale() {
    let a = D38s12::try_from(5).unwrap();
    let b = D38s6::try_from(7).unwrap();
    let c: D38s9 = D38s9::add_of(a, b);
    assert_eq!(c, D38s9::try_from(12).unwrap());
}

#[test]
fn d38_sub_of_same_width_cross_scale() {
    let a = D38s12::try_from(10).unwrap();
    let b = D38s6::try_from(3).unwrap();
    let c: D38s9 = D38s9::sub_of(a, b);
    assert_eq!(c, D38s9::try_from(7).unwrap());
}

#[test]
fn d38_div_of_same_width_cross_scale() {
    let a = D38s12::try_from(12).unwrap();
    let b = D38s6::try_from(4).unwrap();
    let c: D38s9 = D38s9::div_of(a, b);
    assert_eq!(c, D38s9::try_from(3).unwrap());
}

#[test]
fn d38_rem_of_same_width_cross_scale() {
    let a = D38s12::try_from(17).unwrap();
    let b = D38s6::try_from(5).unwrap();
    let c: D38s9 = D38s9::rem_of(a, b);
    // 17 % 5 = 2
    assert_eq!(c, D38s9::try_from(2).unwrap());
}

// ── Cross-width, cross-scale. ────────────────────────────────────────

#[test]
fn cross_width_d9_d18_into_d38() {
    let a = D18s4::try_from(5).unwrap();
    let b = D18s9::try_from(7).unwrap();
    let c: D38s12 = D38s12::mul_of(a, b);
    assert_eq!(c, D38s12::try_from(35).unwrap());
}

#[test]
fn cross_width_d18_d38_into_d38_via_add() {
    let a = D18s6::try_from(100).unwrap();
    let b = D38s18::try_from(50).unwrap();
    let c: D38s12 = D38s12::add_of(a, b);
    assert_eq!(c, D38s12::try_from(150).unwrap());
}

// ── Rounding mode sweep on the `_with` variants. ─────────────────────

#[test]
fn mul_of_with_rounding_modes() {
    // 1.5 (at SCALE=1) × 1 (at SCALE=0) into target SCALE = 0.
    // Rescaling the SCALE=1 operand down to SCALE=0 applies `mode`.
    let a = D38::<1>::from_bits(decimal_scaled::Int::<2>::try_from((15) as i128).unwrap()); // 1.5
    let b = D38::<0>::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()); // 1
    let trunc: D38<0> = D38::<0>::mul_of_with(a, b, RoundingMode::Trunc);
    assert_eq!(i128::from(trunc.to_bits()), 1); // 1.5 truncates to 1, then 1*1
    let away: D38<0> = D38::<0>::mul_of_with(a, b, RoundingMode::HalfAwayFromZero);
    assert_eq!(i128::from(away.to_bits()), 2); // 1.5 rounds away to 2, then 2*1
    let floor: D38<0> = D38::<0>::mul_of_with(a, b, RoundingMode::Floor);
    assert_eq!(i128::from(floor.to_bits()), 1);
    let ceil: D38<0> = D38::<0>::mul_of_with(a, b, RoundingMode::Ceiling);
    assert_eq!(i128::from(ceil.to_bits()), 2);
}

// ── Overflow panic on mul. ───────────────────────────────────────────

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn add_of_overflow_panics_in_debug() {
    // D18<0>::MAX + 1 overflows the storage range.
    let a = D18::<0>::MAX;
    let b = D18::<0>::try_from(1).unwrap();
    let _c: D18<0> = D18::<0>::add_of(a, b);
}

// ── max_of / min_of / clamp_of. ──────────────────────────────────────

#[test]
fn max_of_picks_larger_at_higher_scale() {
    let a = D18s6::try_from(3).unwrap(); // 3.0
    let b = D18s9::try_from(2).unwrap(); // 2.0
    let c: D38s12 = D38s12::max_of(a, b);
    assert_eq!(c, D38s12::try_from(3).unwrap());
}

#[test]
fn min_of_picks_smaller_at_higher_scale() {
    let a = D18s6::try_from(3).unwrap();
    let b = D18s9::try_from(2).unwrap();
    let c: D38s12 = D38s12::min_of(a, b);
    assert_eq!(c, D38s12::try_from(2).unwrap());
}

#[test]
fn clamp_of_clamps_correctly() {
    let v = D38s12::try_from(15).unwrap();
    let lo = D18s4::try_from(0).unwrap();
    let hi = D18s9::try_from(10).unwrap();
    let c: D38s12 = D38s12::clamp_of(v, lo, hi);
    assert_eq!(c, D38s12::try_from(10).unwrap());

    let v2 = D38s12::try_from(-5).unwrap();
    let c2: D38s12 = D38s12::clamp_of(v2, lo, hi);
    assert_eq!(c2, D38s12::try_from(0).unwrap());

    let v3 = D38s12::try_from(7).unwrap();
    let c3: D38s12 = D38s12::clamp_of(v3, lo, hi);
    assert_eq!(c3, D38s12::try_from(7).unwrap());
}

// ── Comparators. ─────────────────────────────────────────────────────

#[test]
fn cmp_of_cross_width_cross_scale_consistent() {
    let a = D38s12::try_from(5).unwrap();
    let b = D18s6::try_from(5).unwrap();
    assert_eq!(a.cmp_of(b), core::cmp::Ordering::Equal);
    assert!(a.eq_of(b));
    assert!(!a.ne_of(b));
    assert!(a.le_of(b));
    assert!(a.ge_of(b));
    assert!(!a.lt_of(b));
    assert!(!a.gt_of(b));
}

#[test]
fn cmp_of_picks_higher_scale_for_exact_compare() {
    // 1.0 at SCALE=12 vs 1.0 at SCALE=6 — should be exactly equal.
    let a = D38s12::try_from(1).unwrap();
    let b = D18s6::try_from(1).unwrap();
    assert_eq!(a.cmp_of(b), core::cmp::Ordering::Equal);
}

#[test]
fn cmp_of_distinguishes_small_difference() {
    // 1.000000000001 vs 1.000000
    let a = D38::<12>::from_bits(decimal_scaled::Int::<2>::try_from((1_000_000_000_001) as i128).unwrap());
    let b = D38::<6>::try_from(1).unwrap();
    assert!(a.gt_of(b));
    assert!(b.lt_of(a));
}

// ── Comparator chain across widths. ──────────────────────────────────

#[test]
fn lt_gt_le_ge_chain() {
    let a = D38s12::try_from(1).unwrap();
    let b = D38s12::try_from(2).unwrap();
    assert!(a.lt_of(b));
    assert!(a.le_of(b));
    assert!(b.gt_of(a));
    assert!(b.ge_of(a));
}

#[test]
fn rem_of_with_modes_rescales_inputs() {
    // 10 (SCALE 0) % 3 (SCALE 0) at SCALE = 0.
    // (Use signed input.)
    let a = D38::<0>::try_from(10).unwrap();
    let b = D38::<0>::try_from(3).unwrap();
    let c: D38<0> = D38::<0>::rem_of_with(a, b, RoundingMode::HalfToEven);
    assert_eq!(c, D38::<0>::try_from(1).unwrap());
}

// ── Operator overload cross-width PartialEq / PartialOrd (same SCALE). ─

#[test]
fn cross_width_eq_operator_works() {
    let a: D38<12> = D38::<12>::try_from(5).unwrap();
    let b: D18<12> = D18::<12>::try_from(5).unwrap();
    assert!(a == b);
    assert!(b == a);
    let c: D18<2> = D18::<2>::try_from(3).unwrap();
    let d: D38<2> = D38::<2>::try_from(3).unwrap();
    assert!(c == d);
    assert!(d == c);
}

#[test]
fn cross_width_ord_operator_works() {
    let a: D38<12> = D38::<12>::try_from(5).unwrap();
    let b: D18<12> = D18::<12>::try_from(10).unwrap();
    assert!(a < b);
    assert!(b > a);
    assert!(a <= b);
    assert!(b >= a);
}

// ── Operator overload cross-SCALE PartialEq / PartialOrd (story 1.3.3).
// These now work on stable via the const cmp_cross_scaled comparator —
// no nightly / generic_const_exprs needed for COMPARISON. ─────────────

#[test]
fn cross_scale_eq_operator_works() {
    // 5.000000 (SCALE 6) == 5.000000000000 (SCALE 12), same width.
    let a: D38<6> = D38::<6>::try_from(5).unwrap();
    let b: D38<12> = D38::<12>::try_from(5).unwrap();
    assert!(a == b);
    assert!(b == a);
}

#[test]
fn cross_scale_ord_operator_works() {
    // 5 at SCALE 6 vs 10 at SCALE 12 — across scales, same width.
    let a: D38<6> = D38::<6>::try_from(5).unwrap();
    let b: D38<12> = D38::<12>::try_from(10).unwrap();
    assert!(a < b);
    assert!(b > a);
    assert!(a <= b);
    assert!(b >= a);
}

#[test]
fn cross_width_and_scale_eq_operator_works() {
    // D18<2> 1.50 == D38<4> 1.5000 — cross-width AND cross-scale.
    let a: D18<2> = D18::<2>::try_from(3).unwrap();
    let b: D38<4> = D38::<4>::try_from(3).unwrap();
    assert!(a == b);
    assert!(b == a);
    // Strictly greater across width+scale.
    let bigger: D38<4> = D38::<4>::try_from(4).unwrap();
    assert!(a < bigger);
    assert!(bigger > a);
}

#[test]
fn cross_scale_negatives_operator() {
    // -3 at SCALE 4 vs -3 at SCALE 2 — equal; -4 at SCALE 2 is less.
    let a: D38<4> = D38::<4>::try_from(-3).unwrap();
    let b: D18<2> = D18::<2>::try_from(-3).unwrap();
    assert!(a == b);
    let more_neg: D18<2> = D18::<2>::try_from(-4).unwrap();
    assert!(more_neg < a);
    assert!(a > more_neg);
}

// ── Wide tier smoke test ─────────────────────────────────────────────

#[cfg(feature = "wide")]
mod wide {
    use decimal_scaled::{D38, D38s12, D38s18, D76, D153, D307};

    #[test]
    fn d76_mul_of_accepts_narrow_inputs() {
        let a = D38s12::try_from(7).unwrap();
        let b = D38s18::try_from(11).unwrap();
        let c: D76<24> = D76::<24>::mul_of(a, b);
        // 7 * 11 = 77, at SCALE 24 should be 77 * 10^24 in raw bits.
        // Round-trip via to_int.
        assert_eq!(c.to_int(), 77);
    }

    #[test]
    fn d307_cmp_of_against_narrow_d38() {
        let a: D307<100> = D307::<100>::try_from(42).unwrap();
        let b: D38<12> = D38::<12>::try_from(42).unwrap();
        assert!(a.eq_of(b));
    }

    #[test]
    fn d76_max_of_narrower_widths() {
        let small = D38s12::try_from(5).unwrap();
        let big = D38s18::try_from(10).unwrap();
        let r: D76<20> = D76::<20>::max_of(small, big);
        assert_eq!(r.to_int(), 10);
    }

    #[test]
    fn d153_clamp_of_mixed_widths() {
        let v: D38<12> = D38::<12>::try_from(50).unwrap();
        let lo: D38<6> = D38::<6>::try_from(0).unwrap();
        let hi: D76<12> = D76::<12>::try_from(20).unwrap();
        let r: D153<30> = D153::<30>::clamp_of(v, lo, hi);
        assert_eq!(r.to_int(), 20);
    }
}
