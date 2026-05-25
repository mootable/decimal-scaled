//! Phase-1 capstone (story 1.6): invariant / property tests that encode
//! the 1.1–1.5 rulings. Each test asserts; none silently no-ops.
//!
//! - 1.2  width round-trip: `narrow_n(widen_n(x)) == x`.
//! - 1.3  cross-width AND cross-scale exact value-equality.
//! - 1.1  conversion round-trips (`from_int` / `to_int`, `Int::widen` /
//!   `Int::narrow`).
//! - 1.4  overflow contract: debug-panic on the operator path, `None`
//!   on the `checked_*` path.

use decimal_scaled::{D18s9, D38, D38s12, Int, D};

// ─── 1.2 — width round-trip: narrow_n(widen_n(x)) == x ────────────────

#[test]
fn widen_then_narrow_is_identity_positive() {
    let x: D18s9 = D18s9::from(7);
    // Widen the storage Int<1> -> Int<2>, then narrow back.
    let wide: D<Int<2>, 9> = x.widen_n::<2>();
    let back: D<Int<1>, 9> = wide.narrow_n::<1>().expect("value fits Int<1>");
    assert_eq!(back, x);
}

#[test]
fn widen_then_narrow_is_identity_negative() {
    let x: D18s9 = D18s9::from(-12);
    let wide: D<Int<2>, 9> = x.widen_n::<2>();
    let back: D<Int<1>, 9> = wide.narrow_n::<1>().expect("value fits Int<1>");
    assert_eq!(back, x);
}

#[test]
fn widen_then_narrow_is_identity_zero() {
    let x: D18s9 = D18s9::ZERO;
    let wide: D<Int<4>, 9> = x.widen_n::<4>();
    let back: D<Int<1>, 9> = wide.narrow_n::<1>().expect("zero fits Int<1>");
    assert_eq!(back, x);
}

#[test]
fn narrow_n_rejects_out_of_range() {
    // A value that only fits the wider tier must NOT narrow back.
    // from_int(10^17) at scale 2 stores 10^19 > i64::MAX, so it cannot
    // round-trip into the Int<1>-backed tier.
    let huge: D38<2> = D38::<2>::from(100_000_000_000_000_000_i64);
    assert!(huge.narrow_n::<1>().is_none());
}

// ─── 1.3 — cross-width / cross-scale exact value equality ─────────────

#[test]
fn cross_width_same_scale_value_equal() {
    let narrow: D18s9 = D18s9::from(5);
    let wide: D<Int<2>, 9> = D::<Int<2>, 9>::from(5);
    assert_eq!(narrow, wide);
    assert_eq!(wide, narrow);
}

#[test]
fn cross_scale_value_equal() {
    // 5 at scale 9 and 5 at scale 12 are the same logical value.
    let a: D18s9 = D18s9::from(5);
    let b: D38s12 = D38s12::from(5);
    assert_eq!(a, b);
    assert_eq!(b, a);
}

#[test]
fn cross_scale_unequal_when_fraction_differs() {
    // 5.000000001 (scale 9) vs 5.000000000000 (scale 12): not equal.
    let frac: D18s9 = D18s9::from_bits(Int::<1>::from(5_000_000_001_i64));
    let whole: D38s12 = D38s12::from(5);
    assert_ne!(frac, whole);
    assert!(frac > whole);
    assert!(whole < frac);
}

#[test]
fn d_eq_primitive_int_exact() {
    assert!(D38s12::from(42) == 42_i32);
    assert!(42_i64 == D38s12::from(42));
    // A fractional decimal is never equal to an integer.
    let half: D38s12 = D38s12::from_bits(Int::<2>::from(5_500_000_000_000_i64));
    assert!(!(half == 5_i32));
    assert!(!(half == 6_i32));
}

// ─── 1.1 — conversion round-trips ─────────────────────────────────────

#[test]
fn from_int_to_int_round_trip() {
    for n in [-9_i64, -1, 0, 1, 7, 1234, 9_999_999] {
        let d: D38s12 = D38s12::from(n);
        assert_eq!(d.to_int(), n);
    }
}

#[test]
fn int_widen_narrow_round_trip() {
    for n in [-123_i64, -1, 0, 5, 987_654_321] {
        let small = Int::<1>::from(n);
        let wide = small.widen::<4>();
        let back: Int<1> = wide.narrow::<1>().expect("value fits");
        assert_eq!(back, small);
    }
}

// ─── 1.4 — overflow contract ──────────────────────────────────────────

#[test]
fn checked_add_returns_none_on_overflow() {
    // The checked path never panics; it reports the overflow.
    assert!(D38s12::MAX.checked_add(D38s12::ONE).is_none());
    // A non-overflowing checked add succeeds.
    assert!(D38s12::ZERO.checked_add(D38s12::ONE).is_some());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "overflow")]
fn operator_add_panics_on_overflow_in_debug() {
    // The `+` operator follows Rust's integer-overflow contract: it
    // panics in debug builds. (In release it wraps, so this assertion
    // is debug-gated.)
    let _ = D38s12::MAX + D38s12::ONE;
}
