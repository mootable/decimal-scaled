//! Near-zero directed-rounding pins for the exp / hyperbolic family.
//!
//! For a tiny argument `±10^-k` the Taylor residual after the storage grid
//! line is known analytically (`exp`: `v²/2 + …`; `cosh`: `v²/2 + …` above
//! `1`; `sinh`: `+v³/6`; `tanh`: `−v³/3`), and whether a directed mode may
//! round by it is governed by the crate's precision horizon (1233 fractional
//! digits — a residual past it is below the crate's resolution and exactly
//! zero). These tests assert the analytic expectations on real cells, both
//! sides of the horizon, plus that the nearest modes are unchanged.

#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use core::str::FromStr;
use decimal_scaled::RoundingMode::{
    Ceiling, Floor, HalfAwayFromZero, HalfToEven, HalfTowardZero, Trunc,
};
use decimal_scaled::{D1232, D38, D462, D57, D616, D924};

/// `"0.(k−1 zeros)1"` — the literal for `10^-k`.
fn tiny(k: usize) -> String {
    format!("0.{}1", "0".repeat(k - 1))
}

/// One-ULP bump of a value at scale `S`: adds the parsed `10^-S`.
macro_rules! plus_ulp {
    ($ty:ident, $s:expr, $v:expr) => {
        $v + $ty::<$s>::from_str(&tiny($s)).unwrap()
    };
}

// ── exp ─────────────────────────────────────────────────────────────

#[test]
fn exp_residual_within_horizon_rounds_ceiling_up() {
    // exp(1e-616) at scale 1231: residual v²/2 = 5·10^-1233 sits exactly AT
    // the horizon — visible, so Ceiling rounds up one ULP; Floor/Trunc and
    // the nearest modes keep 1 + v.
    let v = D1232::<1231>::from_str(&tiny(616)).unwrap();
    let g = D1232::<1231>::ONE + v;
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D1232, 1231, g));
    assert_eq!(v.exp_strict_with(Floor), g);
    assert_eq!(v.exp_strict_with(Trunc), g);
    assert_eq!(v.exp_strict_with(HalfToEven), g);
    assert_eq!(v.exp_strict_with(HalfAwayFromZero), g);
    assert_eq!(v.exp_strict_with(HalfTowardZero), g);
}

#[test]
fn exp_residual_past_horizon_is_exact() {
    // exp(±1e-693) at scale 1231: residual v²/2 = 5·10^-1387 lies past the
    // 1233-digit horizon — below the crate's resolution, so EVERY mode
    // (Ceiling included) returns exactly 1 + v.
    let v = D1232::<1231>::from_str(&tiny(693)).unwrap();
    let g = D1232::<1231>::ONE + v;
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.exp_strict_with(mode), g, "pos {mode:?}");
    }
    let n = -v;
    let gn = D1232::<1231>::ONE + n;
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(n.exp_strict_with(mode), gn, "neg {mode:?}");
    }
}

#[test]
fn exp_exact_half_tie_past_horizon() {
    // exp(±1e-462) at scale 924: v²/2 = 5·10^-925 is EXACTLY half an ULP and
    // the deciding v³/6 term (10^-1386) lies past the horizon — an exact tie.
    // HalfAwayFromZero rounds up; HalfToEven (even last digit) and
    // HalfTowardZero stay; Ceiling rounds the visible half up; Floor/Trunc stay.
    let v = D1232::<924>::from_str(&tiny(462)).unwrap();
    let g = D1232::<924>::ONE + v;
    assert_eq!(v.exp_strict_with(HalfAwayFromZero), plus_ulp!(D1232, 924, g));
    assert_eq!(v.exp_strict_with(HalfToEven), g);
    assert_eq!(v.exp_strict_with(HalfTowardZero), g);
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D1232, 924, g));
    assert_eq!(v.exp_strict_with(Floor), g);
    assert_eq!(v.exp_strict_with(Trunc), g);

    let n = -v;
    let gn = D1232::<924>::ONE + n;
    assert_eq!(n.exp_strict_with(HalfAwayFromZero), plus_ulp!(D1232, 924, gn));
    assert_eq!(n.exp_strict_with(HalfToEven), gn);
    assert_eq!(n.exp_strict_with(Ceiling), plus_ulp!(D1232, 924, gn));
    assert_eq!(n.exp_strict_with(Trunc), gn);
}

#[test]
fn exp_visible_quadratic_grid_point_with_visible_cubic() {
    // exp(1e-230) at scale 461: the value is 1 + v + 5·10^-461 (the v²/2 term
    // lands exactly on the last storage digit) plus a POSITIVE v³/6 residual
    // at 10^-691 — within the horizon. Trunc/Floor keep the grid point,
    // Ceiling rounds up.
    let v = D462::<461>::from_str(&tiny(230)).unwrap();
    let q2 = D462::<461>::from_str(&format!("0.{}5", "0".repeat(460))).unwrap();
    let g = D462::<461>::ONE + v + q2;
    assert_eq!(v.exp_strict_with(Trunc), g);
    assert_eq!(v.exp_strict_with(Floor), g);
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D462, 461, g));
    assert_eq!(v.exp_strict_with(HalfToEven), g);
}

#[test]
fn exp_visible_quadratic_grid_point_with_invisible_cubic() {
    // exp(-1e-461) at scale 923: value = 1 − v + 5·10^-923 exactly on the
    // grid at horizon precision (the −v³/6 correction at 10^-1384 is past
    // the horizon) — every mode returns the grid point.
    let n = -D924::<923>::from_str(&tiny(461)).unwrap();
    let q2 = D924::<923>::from_str(&format!("0.{}5", "0".repeat(922))).unwrap();
    let g = D924::<923>::ONE + n + q2;
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(n.exp_strict_with(mode), g, "{mode:?}");
    }
}

// ── cosh ────────────────────────────────────────────────────────────

#[test]
fn cosh_narrow_sub_resolution_excess_bumps_ceiling() {
    // cosh(±1e-17) at D38<37>: the result is 1 + 5·10^-35 + 4.16·10^-70 —
    // the x⁴/24 term sits below the narrow working scale but well within the
    // precision horizon, so Ceiling must round the grid point up one ULP;
    // Floor/Trunc and the nearest modes keep it. cosh is even.
    let q2 = D38::<37>::from_str(&format!("0.{}5{}", "0".repeat(34), "0".repeat(2))).unwrap();
    let g = D38::<37>::ONE + q2;
    for v in [
        D38::<37>::from_str(&tiny(17)).unwrap(),
        -D38::<37>::from_str(&tiny(17)).unwrap(),
    ] {
        assert_eq!(v.cosh_strict_with(Ceiling), plus_ulp!(D38, 37, g));
        assert_eq!(v.cosh_strict_with(Floor), g);
        assert_eq!(v.cosh_strict_with(Trunc), g);
        assert_eq!(v.cosh_strict_with(HalfToEven), g);
    }
}

#[test]
fn cosh_residual_past_horizon_is_exact() {
    // cosh(±1e-693) at scale 923: the x²/2 excess (10^-1386/2) lies past the
    // precision horizon — below the crate's resolution — so every mode,
    // Ceiling included, returns exactly 1.
    let v = D924::<923>::from_str(&tiny(693)).unwrap();
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.cosh_strict_with(mode), D924::<923>::ONE, "{mode:?}");
        assert_eq!((-v).cosh_strict_with(mode), D924::<923>::ONE, "neg {mode:?}");
    }
}

#[test]
fn cosh_truncation_tie_keeps_even_grid() {
    // cosh(±1e-308) at scale 616: the rest below the scale is exactly
    // `5000…` at horizon precision (the deciding x⁴/24 term at position
    // 1234 lies past the truncation line) — an exact half-ULP tie:
    // HalfToEven (even last digit) and HalfTowardZero keep the grid point,
    // HalfAwayFromZero and Ceiling round up.
    let v = D1232::<616>::from_str(&tiny(308)).unwrap();
    let g = D1232::<616>::ONE;
    let up = plus_ulp!(D1232, 616, g);
    assert_eq!(v.cosh_strict_with(HalfToEven), g);
    assert_eq!(v.cosh_strict_with(HalfTowardZero), g);
    assert_eq!(v.cosh_strict_with(HalfAwayFromZero), up);
    assert_eq!(v.cosh_strict_with(Ceiling), up);
    assert_eq!(v.cosh_strict_with(Trunc), g);
}

// ── sinh ────────────────────────────────────────────────────────────

#[test]
fn sinh_visible_cubic_keeps_directed_nudge() {
    // sinh(±1e-308) at scale 615: the cubic excess x³/6 (1.67·10^-925) is
    // within the precision horizon — sinh expands, so Ceiling rounds a
    // positive argument up one ULP (and Floor a negative one down); Trunc
    // and the nearest modes keep the grid line.
    let v = D616::<615>::from_str(&tiny(308)).unwrap();
    assert_eq!(v.sinh_strict_with(Ceiling), plus_ulp!(D616, 615, v));
    assert_eq!(v.sinh_strict_with(Trunc), v);
    assert_eq!(v.sinh_strict_with(Floor), v);
    assert_eq!(v.sinh_strict_with(HalfToEven), v);
    let n = -v;
    assert_eq!(n.sinh_strict_with(Floor), n - D616::<615>::from_str(&tiny(615)).unwrap());
    assert_eq!(n.sinh_strict_with(Ceiling), n);
    assert_eq!(n.sinh_strict_with(Trunc), n);
}

#[test]
fn sinh_cubic_past_horizon_is_exact() {
    // sinh(±1e-461) at scale 615: the cubic excess (at position 1384) lies
    // past the precision horizon — sinh(x) is exactly x at the crate's
    // resolution, so NO mode nudges.
    let v = D616::<615>::from_str(&tiny(461)).unwrap();
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sinh_strict_with(mode), v, "pos {mode:?}");
        assert_eq!((-v).sinh_strict_with(mode), -v, "neg {mode:?}");
    }
}

// ── tanh ────────────────────────────────────────────────────────────

#[test]
fn tanh_visible_deficit_keeps_directed_nudge() {
    // tanh(±1e-616) at scale 1231: the cubic deficit x³/3 survives at the
    // crate's precision (its 9-run below the leading digit is visible) —
    // tanh compresses, so Trunc and Floor round a positive argument one ULP
    // toward zero / down; Ceiling and the nearest modes keep the grid line.
    let v = D1232::<1231>::from_str(&tiny(616)).unwrap();
    let down = v - D1232::<1231>::from_str(&tiny(1231)).unwrap();
    assert_eq!(v.tanh_strict_with(Trunc), down);
    assert_eq!(v.tanh_strict_with(Floor), down);
    assert_eq!(v.tanh_strict_with(Ceiling), v);
    assert_eq!(v.tanh_strict_with(HalfToEven), v);
    let n = -v;
    assert_eq!(n.tanh_strict_with(Trunc), -down);
    assert_eq!(n.tanh_strict_with(Ceiling), -down);
    assert_eq!(n.tanh_strict_with(Floor), n);
}

#[test]
fn tanh_deficit_past_horizon_is_exact() {
    // tanh(±1e-693): the cubic deficit's relative position (~1387) lies past
    // the carried value precision — the value rounds back to exactly x, so
    // NO mode nudges. Checked both at a deep scale and at the scale where
    // the argument is a single ULP (where a stale Trunc nudge would have
    // collapsed it to zero).
    let v = D1232::<1231>::from_str(&tiny(693)).unwrap();
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.tanh_strict_with(mode), v, "pos {mode:?}");
        assert_eq!((-v).tanh_strict_with(mode), -v, "neg {mode:?}");
    }
    let u = D924::<693>::from_str(&tiny(693)).unwrap();
    for mode in [Ceiling, Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(u.tanh_strict_with(mode), u, "ulp pos {mode:?}");
        assert_eq!((-u).tanh_strict_with(mode), -u, "ulp neg {mode:?}");
    }
}

// ── exp2 ────────────────────────────────────────────────────────────

#[test]
fn exp2_truncated_exact_tail_all_modes_agree() {
    // exp2(1e-924) at scale 1231: the result is `1 + ln2·10^-924 + …`, whose
    // digits just below the scale (positions 1232..1233, `ln 2`'s 308th and
    // 309th digits) are zero, and everything deeper lies past the precision
    // horizon's truncation — the result is exact at horizon precision, so
    // every mode (Ceiling included — the over-bump this pins) agrees.
    let v = D1232::<1231>::from_str(&tiny(924)).unwrap();
    let c = v.exp2_strict_with(Ceiling);
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.exp2_strict_with(mode), c, "{mode:?}");
    }
}

#[test]
fn exp2_below_grid_nine_run_visible() {
    // exp2(-1e-924) at scale 1231: the deficit to the next grid line sits at
    // position ~1235 — within the carried value precision — so the 9-run
    // below the scale is VISIBLE: Trunc/Floor keep the lower grid line,
    // Ceiling and nearest round up one ULP.
    let n = -D1232::<1231>::from_str(&tiny(924)).unwrap();
    let t = n.exp2_strict_with(Trunc);
    assert_eq!(n.exp2_strict_with(Floor), t);
    let up = plus_ulp!(D1232, 1231, t);
    assert_eq!(n.exp2_strict_with(Ceiling), up);
    assert_eq!(n.exp2_strict_with(HalfToEven), up);
}

#[test]
fn exp2_just_below_half_rounds_down() {
    // exp2(-1e-308) at scale 1231: the rest digits below the scale are
    // `49…` — just below half by a sub-truncation amount that is still
    // within the carried value precision, so nearest rounds DOWN (not a
    // tie); Ceiling rounds up.
    let n = -D1232::<1231>::from_str(&tiny(308)).unwrap();
    let t = n.exp2_strict_with(Trunc);
    assert_eq!(n.exp2_strict_with(HalfToEven), t);
    assert_eq!(n.exp2_strict_with(HalfAwayFromZero), t);
    assert_eq!(n.exp2_strict_with(Ceiling), plus_ulp!(D1232, 1231, t));
}

#[test]
fn exp_near_min_pin_prefilter_boundary_band() {
    // The pin's bit-length pre-filter must keep the WHOLE |v| < 10^(-S/2)
    // band: |raw| = 10^28 − 1 at D57<56> (bit length 94) sat in the band's
    // top quarter the off-by-one filter dropped. Residual v²/2 ≈ 5·10^-57 is
    // sub-half-ULP and well within the horizon: Ceiling rounds up, the rest
    // keep 1 + v.
    let v = D57::<56>::from_str(&format!("0.{}{}", "0".repeat(28), "9".repeat(28))).unwrap();
    let g = D57::<56>::ONE + v;
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D57, 56, g));
    assert_eq!(v.exp_strict_with(Floor), g);
    assert_eq!(v.exp_strict_with(Trunc), g);
    assert_eq!(v.exp_strict_with(HalfToEven), g);
    // Just OUTSIDE the window (|raw| = 10^28): the v²/2 = half-ULP tie is
    // decided by the visible v³/6 term — every nearest mode rounds up for a
    // positive argument.
    let b = D57::<56>::from_str(&tiny(28)).unwrap();
    let gb = D57::<56>::ONE + b;
    assert_eq!(b.exp_strict_with(HalfToEven), plus_ulp!(D57, 56, gb));
    assert_eq!(b.exp_strict_with(Ceiling), plus_ulp!(D57, 56, gb));
    assert_eq!(b.exp_strict_with(Trunc), gb);
}
