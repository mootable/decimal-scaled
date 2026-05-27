//! Structural edge-case gate for `hypot_strict_with` — the non-value
//! assertions the correctly-rounded golden corpus cannot express.
//!
//! The numeric VALUE coverage for `hypot` (Pythagorean triples, the
//! non-perfect `√` cases, and the adversarial seam/band-edge inputs) now
//! lives in the shared golden corpus under `tests/golden/hypot_d*_s*.txt`
//! and is checked, bit-exact across all six rounding modes, by
//! `tests/ulp_strict_golden.rs` — the single source of truth. What
//! remains here are the cases golden's `(input, floor, cls)` →
//! `delta == 0` format cannot carry:
//!
//!   * `hypot(0, 0) = 0` bit-exact at every tier;
//!   * `hypot(0, x) = |x|` (including the negative-operand mirror);
//!   * the storage-overflow contract: `hypot(MAX, 0) = MAX` (fits, no
//!     panic) versus `hypot(MAX, MAX) ≈ MAX·√2` (out of range → panic).
//!
//! A golden cell can only pin a representable result; the overflow case
//! has no representable answer, so it is asserted here as a panic.

#![cfg(all(feature = "wide", feature = "strict", not(feature = "fast")))]

use decimal_scaled::{RoundingMode, D18, D307, D38, D57};

const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

#[test]
fn hypot_zero_zero_is_zero_bit_exact_all_tiers_all_modes() {
    for mode in ALL_MODES {
        assert_eq!(
            D18::<6>::ZERO.hypot_strict_with(D18::<6>::ZERO, mode),
            D18::<6>::ZERO,
            "D18 hypot(0,0) mode {mode:?}",
        );
        assert_eq!(
            D38::<6>::ZERO.hypot_strict_with(D38::<6>::ZERO, mode),
            D38::<6>::ZERO,
            "D38 hypot(0,0) mode {mode:?}",
        );
        assert_eq!(
            D57::<6>::ZERO.hypot_strict_with(D57::<6>::ZERO, mode),
            D57::<6>::ZERO,
            "D57 hypot(0,0) mode {mode:?}",
        );
        assert_eq!(
            D307::<30>::ZERO.hypot_strict_with(D307::<30>::ZERO, mode),
            D307::<30>::ZERO,
            "D307 hypot(0,0) mode {mode:?}",
        );
    }
}

#[test]
fn hypot_zero_x_is_abs_x_all_tiers_all_modes() {
    // hypot(0, x) = |x| exactly, and hypot(0, -x) = |x|.
    for &x in &[3i64, 7, 42, 100] {
        for mode in ALL_MODES {
            let d38 = D38::<6>::from(x);
            let d38n = D38::<6>::from(-x);
            assert_eq!(
                D38::<6>::ZERO.hypot_strict_with(d38, mode),
                d38,
                "D38 hypot(0,{x}) mode {mode:?}",
            );
            assert_eq!(
                D38::<6>::ZERO.hypot_strict_with(d38n, mode),
                d38,
                "D38 hypot(0,-{x}) mode {mode:?} (= |{x}|)",
            );
            let d307 = D307::<30>::from(x);
            assert_eq!(
                D307::<30>::ZERO.hypot_strict_with(d307, mode),
                d307,
                "D307 hypot(0,{x}) mode {mode:?}",
            );
        }
    }
}

#[test]
fn hypot_near_max_does_not_overflow_when_in_range() {
    // a near MAX, b = 0 -> hypot = a exactly, must not panic or overflow.
    let a = D38::<0>::MAX;
    assert_eq!(a.hypot_strict_with(D38::<0>::ZERO, RoundingMode::HalfToEven), a);
}

#[test]
#[should_panic(expected = "hypot: result out of range")]
fn hypot_overflow_panics_d38() {
    // a = b = MAX magnitude: hypot ~= MAX·√2 exceeds the type range.
    let m = D38::<0>::MAX;
    let _ = m.hypot_strict_with(m, RoundingMode::HalfToEven);
}
