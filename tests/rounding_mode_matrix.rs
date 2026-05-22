//! Mode-aware and precision-aware transcendental matrix.
//!
//! Each strict / approx method now ships with a `_with(mode)` sibling
//! and (for the transcendentals) an `_approx(working_digits)` family
//! that lets callers trade guard-width for speed. These tests pin the
//! delegation contract on a few representative methods across the
//! narrow (D38) and wide (D76) tiers:
//!
//! - `*_strict()` is bit-equal to `*_strict_with(DEFAULT_ROUNDING_MODE)`.
//! - `*_approx(g)` is bit-equal to `*_approx_with(g, DEFAULT)`.
//! - `*_approx_with(STRICT_GUARD, mode)` is bit-equal to
//!   `*_strict_with(mode)` (the redirect-to-strict shortcut).
//! - Non-half rounding modes (`Floor` / `Ceiling`) produce results
//!   that bracket the half-mode result for inexact inputs.

#![cfg(all(
    feature = "strict",
    not(feature = "fast"),
    any(feature = "d76", feature = "wide"),
))]

use decimal_scaled::{D38, D76, RoundingMode};
use std::str::FromStr;

const STRICT_GUARD: u32 = 30;

fn d38s19(s: &str) -> D38<19> {
    D38::<19>::from_str(s).expect("parse D38s19")
}

fn d76s30(s: &str) -> D76<30> {
    D76::<30>::from_str(s).expect("parse D76s30")
}

// ─────────────────────────────────────────────────────────────────────
// D38: delegation contracts
// ─────────────────────────────────────────────────────────────────────

#[test]
fn d38_ln_strict_delegates_to_strict_with_default() {
    let x = d38s19("3");
    assert_eq!(x.ln_strict(), x.ln_strict_with(RoundingMode::HalfToEven));
}

#[test]
fn d38_ln_approx_delegates_to_approx_with_default() {
    let x = d38s19("3");
    assert_eq!(
        x.ln_approx(10),
        x.ln_approx_with(10, RoundingMode::HalfToEven),
    );
}

#[test]
fn d38_ln_approx_at_strict_guard_redirects_to_strict() {
    let x = d38s19("3");
    assert_eq!(
        x.ln_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
        x.ln_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d38_sin_approx_at_strict_guard_redirects_to_strict() {
    let x = d38s19("1");
    assert_eq!(
        x.sin_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
        x.sin_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d38_atan2_approx_at_strict_guard_redirects_to_strict() {
    let y = d38s19("1");
    let x = d38s19("2");
    assert_eq!(
        y.atan2_approx_with(x, STRICT_GUARD, RoundingMode::HalfToEven),
        y.atan2_strict_with(x, RoundingMode::HalfToEven),
    );
}

#[test]
fn d38_powf_approx_at_strict_guard_redirects_to_strict() {
    let x = d38s19("2");
    let y = d38s19("0.5");
    assert_eq!(
        x.powf_approx_with(y, STRICT_GUARD, RoundingMode::HalfToEven),
        x.powf_strict_with(y, RoundingMode::HalfToEven),
    );
}

// ─────────────────────────────────────────────────────────────────────
// D38: rounding modes diverge on inexact transcendentals
// ─────────────────────────────────────────────────────────────────────

#[test]
fn d38_ln_floor_and_ceiling_bracket_half() {
    // ln(3) is irrational; the last storage place differs by mode.
    let x = d38s19("3");
    let floor_v = x.ln_strict_with(RoundingMode::Floor);
    let ceil_v = x.ln_strict_with(RoundingMode::Ceiling);
    let half_v = x.ln_strict_with(RoundingMode::HalfToEven);
    assert!(
        floor_v <= half_v && half_v <= ceil_v,
        "ln(3): floor {floor_v:?} <= half {half_v:?} <= ceiling {ceil_v:?}",
    );
    // Floor < Ceiling for an irrational with non-zero residual.
    assert!(floor_v < ceil_v);
}

#[test]
fn d38_sin_floor_le_ceiling() {
    let x = d38s19("1");
    let floor_v = x.sin_strict_with(RoundingMode::Floor);
    let ceil_v = x.sin_strict_with(RoundingMode::Ceiling);
    assert!(floor_v <= ceil_v);
}

// ─────────────────────────────────────────────────────────────────────
// sqrt / cbrt: mode dispatch
// ─────────────────────────────────────────────────────────────────────

#[test]
fn d38_sqrt_strict_delegates_to_strict_with_default() {
    let x = d38s19("2");
    assert_eq!(
        x.sqrt_strict(),
        x.sqrt_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d38_sqrt_trunc_is_floor_for_positive() {
    // sqrt(2) ≈ 1.41421356… — irrational at any finite scale, so
    // Trunc / Floor give the smaller neighbour and Ceiling gives the
    // larger.
    let x = d38s19("2");
    let trunc_v = x.sqrt_strict_with(RoundingMode::Trunc);
    let floor_v = x.sqrt_strict_with(RoundingMode::Floor);
    let ceil_v = x.sqrt_strict_with(RoundingMode::Ceiling);
    assert_eq!(trunc_v, floor_v);
    assert!(floor_v < ceil_v);
}

#[test]
fn d38_sqrt_perfect_square_modes_agree() {
    // sqrt(4) = 2 exactly — no residual, all modes return 2.
    let x = d38s19("4");
    let two = d38s19("2");
    for m in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        assert_eq!(x.sqrt_strict_with(m), two, "sqrt(4) under {m:?}");
    }
}

#[test]
fn d38_cbrt_negative_floor_more_negative_than_ceiling() {
    // cbrt(-3) ≈ -1.44… — negative; Floor pushes further negative
    // (greater magnitude), Ceiling pulls toward zero (smaller mag).
    let x = d38s19("-3");
    let floor_v = x.cbrt_strict_with(RoundingMode::Floor);
    let ceil_v = x.cbrt_strict_with(RoundingMode::Ceiling);
    assert!(floor_v < ceil_v);
}

#[test]
fn d38_cbrt_perfect_cube_modes_agree() {
    let x = d38s19("8");
    let two = d38s19("2");
    for m in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        assert_eq!(x.cbrt_strict_with(m), two, "cbrt(8) under {m:?}");
    }
}

#[test]
fn d38_hypot_strict_delegates_to_strict_with_default() {
    let a = d38s19("3");
    let b = d38s19("4");
    assert_eq!(
        a.hypot_strict(b),
        a.hypot_strict_with(b, RoundingMode::HalfToEven),
    );
}

// ─────────────────────────────────────────────────────────────────────
// Wide tier (D76): same contracts
// ─────────────────────────────────────────────────────────────────────

#[test]
fn d76_ln_strict_delegates_to_strict_with_default() {
    let x = d76s30("3");
    assert_eq!(x.ln_strict(), x.ln_strict_with(RoundingMode::HalfToEven));
}

#[test]
fn d76_ln_approx_at_strict_guard_redirects_to_strict() {
    let x = d76s30("3");
    assert_eq!(
        x.ln_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
        x.ln_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d76_sin_approx_at_strict_guard_redirects_to_strict() {
    let x = d76s30("1");
    assert_eq!(
        x.sin_approx_with(STRICT_GUARD, RoundingMode::HalfToEven),
        x.sin_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d76_sqrt_floor_le_ceiling_for_irrational() {
    let x = d76s30("2");
    let floor_v = x.sqrt_strict_with(RoundingMode::Floor);
    let ceil_v = x.sqrt_strict_with(RoundingMode::Ceiling);
    assert!(floor_v < ceil_v);
}

#[test]
fn d76_cbrt_strict_delegates_to_strict_with_default() {
    let x = d76s30("8");
    assert_eq!(
        x.cbrt_strict(),
        x.cbrt_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d76_atan2_approx_at_strict_guard_redirects_to_strict() {
    let y = d76s30("1");
    let x = d76s30("2");
    assert_eq!(
        y.atan2_approx_with(x, STRICT_GUARD, RoundingMode::HalfToEven),
        y.atan2_strict_with(x, RoundingMode::HalfToEven),
    );
}

#[test]
fn d76_sin_cos_strict_delegates_to_strict_with_default() {
    let x = d76s30("0.5");
    assert_eq!(
        x.sin_cos_strict(),
        x.sin_cos_strict_with(RoundingMode::HalfToEven),
    );
}

#[test]
fn d76_sinh_cosh_strict_delegates_to_strict_with_default() {
    let x = d76s30("0.5");
    assert_eq!(
        x.sinh_cosh_strict(),
        x.sinh_cosh_strict_with(RoundingMode::HalfToEven),
    );
}

// ─────────────────────────────────────────────────────────────────────
// Approx-vs-strict: lower guard widths still hit fast paths exactly
// ─────────────────────────────────────────────────────────────────────

#[test]
fn d38_ln_one_is_zero_under_all_guards() {
    let one = d38s19("1");
    for g in [6u32, 10, 15, STRICT_GUARD] {
        assert_eq!(one.ln_approx(g), D38::<19>::ZERO, "ln(1) at guard {g}");
    }
}

#[test]
fn d38_sin_zero_is_zero_under_all_guards() {
    let zero = D38::<19>::ZERO;
    for g in [6u32, 10, 15, STRICT_GUARD] {
        assert_eq!(zero.sin_approx(g), D38::<19>::ZERO, "sin(0) at guard {g}");
    }
}

#[test]
fn d76_ln_one_is_zero_under_all_guards() {
    let one = d76s30("1");
    for g in [6u32, 10, 15, STRICT_GUARD] {
        assert_eq!(one.ln_approx(g), D76::<30>::ZERO, "ln(1) at guard {g}");
    }
}
