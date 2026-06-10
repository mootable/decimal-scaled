//! Directed-rounding pins for `sin` / `cos` at the bounded extremum.
//!
//! For an argument within the input granularity of `π/2 + kπ` (sin) or
//! `kπ` (cos) the deviation `δ²/2` from `±1` sits below any reachable
//! working scale, yet sin/cos are STRICTLY interior to `(−1, 1)` for every
//! representable nonzero argument — so the directed side is known a
//! priori: just below `+1`, Floor/Trunc step down one ULP; just above
//! `−1`, Ceiling/Trunc step toward zero. The D57 SCALE 18..=22 narrow
//! band routes through the shared directed-aware kernel; these tests pin
//! the band cells the full golden gate flagged, plus the boundary cases
//! the adjust must NOT touch (exact zero argument; a resolvable interior
//! residual).

#![cfg(any(feature = "d57", feature = "wide"))]

use core::str::FromStr;
use decimal_scaled::RoundingMode::{
    Ceiling, Floor, HalfAwayFromZero, HalfToEven, HalfTowardZero, Trunc,
};
use decimal_scaled::D57;

// ── sin at the band scale, argument hugging π/2 ─────────────────────

#[test]
fn sin_band_near_half_pi_directed_steps_inside() {
    // π/2 truncated to 19 fraction digits: sin = 1 − δ²/2 with
    // δ ≈ 3.1·10⁻²¹, so the deviation (~5·10⁻⁴²) is far below the band's
    // working scale — the kernel sees exactly 1 and the interior side
    // decides the directed modes.
    let v = D57::<20>::from_str("1.5707963267948966192").unwrap();
    let one = D57::<20>::ONE;
    let just_below = one - D57::<20>::from_str("0.00000000000000000001").unwrap();
    assert_eq!(v.sin_strict_with(Floor), just_below);
    assert_eq!(v.sin_strict_with(Trunc), just_below);
    assert_eq!(v.sin_strict_with(Ceiling), one);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sin_strict_with(mode), one, "{mode:?}");
    }
}

#[test]
fn sin_band_near_half_pi_negative_mirrors() {
    // sin is odd: at −(π/2 truncated) the value sits just ABOVE −1, so
    // Ceiling/Trunc step toward zero and Floor keeps −1.
    let v = D57::<20>::from_str("-1.5707963267948966192").unwrap();
    let neg_one = -D57::<20>::ONE;
    let just_above = neg_one + D57::<20>::from_str("0.00000000000000000001").unwrap();
    assert_eq!(v.sin_strict_with(Ceiling), just_above);
    assert_eq!(v.sin_strict_with(Trunc), just_above);
    assert_eq!(v.sin_strict_with(Floor), neg_one);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sin_strict_with(mode), neg_one, "{mode:?}");
    }
}

#[test]
fn sin_band_lower_edge_near_half_pi_directed() {
    // The band's lower edge (SCALE 18): π/2 truncated to 18 fraction
    // digits leaves δ ≈ 2.3·10⁻¹⁹ (deviation ~2.7·10⁻³⁸) — still below
    // half a storage ULP at scale 18, so nearest sees 1 and the directed
    // modes step per the interior side.
    let v = D57::<18>::from_str("1.570796326794896619").unwrap();
    let one = D57::<18>::ONE;
    let just_below = one - D57::<18>::from_str("0.000000000000000001").unwrap();
    assert_eq!(v.sin_strict_with(Floor), just_below);
    assert_eq!(v.sin_strict_with(Trunc), just_below);
    assert_eq!(v.sin_strict_with(Ceiling), one);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sin_strict_with(mode), one, "{mode:?}");
    }
}

// ── cos at the band scales, argument hugging kπ ─────────────────────

#[test]
fn cos_band_near_pi_directed_steps_inside() {
    // π truncated to 19 fraction digits: cos = −1 + δ²/2, strictly above
    // −1 — Ceiling/Trunc step toward zero, Floor keeps −1.
    let v = D57::<20>::from_str("3.1415926535897932384").unwrap();
    let neg_one = -D57::<20>::ONE;
    let just_above = neg_one + D57::<20>::from_str("0.00000000000000000001").unwrap();
    assert_eq!(v.cos_strict_with(Ceiling), just_above);
    assert_eq!(v.cos_strict_with(Trunc), just_above);
    assert_eq!(v.cos_strict_with(Floor), neg_one);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.cos_strict_with(mode), neg_one, "{mode:?}");
    }
}

#[test]
fn cos_band_near_thousand_pi_directed_steps_inside() {
    // 1000π truncated to 14 fraction digits: cos = 1 − δ²/2 with
    // δ ≈ 8.5·10⁻¹⁵ (deviation ~3.6·10⁻²⁹, below the band's working
    // scale) — Floor/Trunc step down one ULP, Ceiling keeps 1. Exercises
    // the multi-revolution range reduction feeding the same extremum pin.
    let v = D57::<20>::from_str("3141.59265358979323").unwrap();
    let one = D57::<20>::ONE;
    let just_below = one - D57::<20>::from_str("0.00000000000000000001").unwrap();
    assert_eq!(v.cos_strict_with(Floor), just_below);
    assert_eq!(v.cos_strict_with(Trunc), just_below);
    assert_eq!(v.cos_strict_with(Ceiling), one);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.cos_strict_with(mode), one, "{mode:?}");
    }
}

#[test]
fn cos_band_upper_edge_near_pi_directed() {
    // The band's upper edge (SCALE 22): π truncated to 22 fraction
    // digits leaves δ ≈ 4.3·10⁻²³ (deviation ~9.4·10⁻⁴⁶, past the work
    // integer's probe reach) — the never-resolvable extremum case at the
    // band's other boundary.
    let v = D57::<22>::from_str("3.1415926535897932384626").unwrap();
    let neg_one = -D57::<22>::ONE;
    let just_above = neg_one + D57::<22>::from_str("0.0000000000000000000001").unwrap();
    assert_eq!(v.cos_strict_with(Ceiling), just_above);
    assert_eq!(v.cos_strict_with(Trunc), just_above);
    assert_eq!(v.cos_strict_with(Floor), neg_one);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.cos_strict_with(mode), neg_one, "{mode:?}");
    }
}

// ── boundary OUT: cells the extremum adjust must not touch ──────────

#[test]
fn cos_band_zero_argument_stays_exactly_one() {
    // cos(0) = 1 exactly (the SOLE grid-line case); the interior adjust
    // must not step it down.
    let v = D57::<20>::ZERO;
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.cos_strict_with(mode), D57::<20>::ONE, "{mode:?}");
    }
}

#[test]
fn cos_band_resolvable_near_extremum_rounds_normally() {
    // cos(3.140681501) = −0.99999958490050777447|1846… — close to −1 but
    // the residual is WITHIN the working scale's reach: the value is off
    // the ±1 grid line, so the adjust must not fire and ordinary directed
    // rounding decides (Floor steps away from zero, Ceiling/Trunc keep).
    let v = D57::<20>::from_str("3.140681501").unwrap();
    let toward_zero = D57::<20>::from_str("-0.99999958490050777447").unwrap();
    let away = D57::<20>::from_str("-0.99999958490050777448").unwrap();
    assert_eq!(v.cos_strict_with(Ceiling), toward_zero);
    assert_eq!(v.cos_strict_with(Trunc), toward_zero);
    assert_eq!(v.cos_strict_with(Floor), away);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.cos_strict_with(mode), toward_zero, "{mode:?}");
    }
}

#[test]
fn sin_band_zero_argument_stays_exact() {
    // sin(0) = 0 exactly; no directed mode may nudge it.
    let v = D57::<20>::ZERO;
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sin_strict_with(mode), D57::<20>::ZERO, "{mode:?}");
    }
}

#[test]
fn sin_band_interior_residual_rounds_normally() {
    // sin(0.1223474614721582) = 0.12204245532892141616|0903… — the
    // residual resolves at the working scale, so the ordinary directed
    // rules apply and the extremum adjust is a no-op.
    let v = D57::<20>::from_str("0.1223474614721582").unwrap();
    let down = D57::<20>::from_str("0.12204245532892141616").unwrap();
    let up = D57::<20>::from_str("0.12204245532892141617").unwrap();
    assert_eq!(v.sin_strict_with(Floor), down);
    assert_eq!(v.sin_strict_with(Trunc), down);
    assert_eq!(v.sin_strict_with(Ceiling), up);
    for mode in [HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sin_strict_with(mode), down, "{mode:?}");
    }
}
