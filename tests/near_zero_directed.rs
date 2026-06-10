//! Near-zero directed-rounding pins for the exp / hyperbolic family.
//!
//! For a tiny argument `±10^-k` the Taylor residual after the storage grid
//! line is known analytically (`exp`: `v²/2 + …`; `cosh`: `v²/2 + …` above
//! `1`; `sinh`: `+v³/6`; `tanh`: `−v³/3`), and the functions are irrational
//! at every nonzero rational argument — the residual NEVER terminates, so a
//! directed mode must honour it however deep it sits. These tests assert the
//! analytic expectations on real cells, shallow and deep, plus the nearest
//! modes' behaviour at the half-ULP boundaries.

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
fn exp_quadratic_residual_rounds_ceiling_up() {
    // exp(1e-616) at scale 1231: residual v²/2 = 5·10^-1233 — Ceiling rounds
    // up one ULP; Floor/Trunc and the nearest modes keep 1 + v.
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
fn exp_deep_residual_still_rounds_ceiling_up() {
    // exp(±1e-693) at scale 1231: the v²/2 residual sits at 10^-1387 — far
    // below the storage scale, but exp of a nonzero rational is irrational,
    // so the residual is strictly positive and Ceiling must still round up;
    // every other mode keeps 1 + v.
    let v = D1232::<1231>::from_str(&tiny(693)).unwrap();
    let g = D1232::<1231>::ONE + v;
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D1232, 1231, g));
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.exp_strict_with(mode), g, "pos {mode:?}");
    }
    let n = -v;
    let gn = D1232::<1231>::ONE + n;
    assert_eq!(n.exp_strict_with(Ceiling), plus_ulp!(D1232, 1231, gn));
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(n.exp_strict_with(mode), gn, "neg {mode:?}");
    }
}

#[test]
fn exp_half_boundary_with_tail_rounds_up() {
    // exp(±1e-462) at scale 924: v²/2 = 5·10^-925 lands exactly on the
    // half-ULP boundary, and the never-terminating tail below it puts the
    // residual strictly ABOVE half — every nearest mode and Ceiling round
    // up; Floor/Trunc keep 1 + v.
    for v in [
        D1232::<924>::from_str(&tiny(462)).unwrap(),
        -D1232::<924>::from_str(&tiny(462)).unwrap(),
    ] {
        let g = D1232::<924>::ONE + v;
        let up = plus_ulp!(D1232, 924, g);
        assert_eq!(v.exp_strict_with(HalfToEven), up);
        assert_eq!(v.exp_strict_with(HalfAwayFromZero), up);
        assert_eq!(v.exp_strict_with(HalfTowardZero), up);
        assert_eq!(v.exp_strict_with(Ceiling), up);
        assert_eq!(v.exp_strict_with(Floor), g);
        assert_eq!(v.exp_strict_with(Trunc), g);
    }
}

#[test]
fn exp_visible_quadratic_grid_point_with_visible_cubic() {
    // exp(1e-230) at scale 461: the value is 1 + v + 5·10^-461 (the v²/2 term
    // lands exactly on the last storage digit) plus a POSITIVE v³/6 residual
    // at 10^-691. Trunc/Floor keep the grid point, Ceiling rounds up.
    let v = D462::<461>::from_str(&tiny(230)).unwrap();
    let q2 = D462::<461>::from_str(&format!("0.{}5", "0".repeat(460))).unwrap();
    let g = D462::<461>::ONE + v + q2;
    assert_eq!(v.exp_strict_with(Trunc), g);
    assert_eq!(v.exp_strict_with(Floor), g);
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D462, 461, g));
    assert_eq!(v.exp_strict_with(HalfToEven), g);
}

#[test]
fn exp_deep_quadratic_grid_point_rounds_ceiling_up() {
    // exp(-1e-461) at scale 923: the carried value is 1 − v + 5·10^-923,
    // landing on the grid line at the crate's resolution; the never-exact
    // sub-resolution tail rounds Ceiling up one ULP, every other mode keeps
    // the grid point.
    let n = -D924::<923>::from_str(&tiny(461)).unwrap();
    let q2 = D924::<923>::from_str(&format!("0.{}5", "0".repeat(922))).unwrap();
    let g = D924::<923>::ONE + n + q2;
    assert_eq!(n.exp_strict_with(Ceiling), plus_ulp!(D924, 923, g));
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(n.exp_strict_with(mode), g, "{mode:?}");
    }
}

#[test]
fn exp_near_min_pin_prefilter_boundary_band() {
    // The pin's bit-length pre-filter must keep the WHOLE |v| < 10^(-S/2)
    // band: |raw| = 10^28 − 1 at D57<56> (bit length 94) sat in the band's
    // top quarter the off-by-one filter dropped. Residual v²/2 ≈ 5·10^-57 is
    // sub-half-ULP: Ceiling rounds up, the rest keep 1 + v.
    let v = D57::<56>::from_str(&format!("0.{}{}", "0".repeat(28), "9".repeat(28))).unwrap();
    let g = D57::<56>::ONE + v;
    assert_eq!(v.exp_strict_with(Ceiling), plus_ulp!(D57, 56, g));
    assert_eq!(v.exp_strict_with(Floor), g);
    assert_eq!(v.exp_strict_with(Trunc), g);
    assert_eq!(v.exp_strict_with(HalfToEven), g);
    // Just OUTSIDE the window (|raw| = 10^28): the v²/2 = half-ULP boundary
    // is decided by the v³/6 tail — every nearest mode rounds up for a
    // positive argument.
    let b = D57::<56>::from_str(&tiny(28)).unwrap();
    let gb = D57::<56>::ONE + b;
    assert_eq!(b.exp_strict_with(HalfToEven), plus_ulp!(D57, 56, gb));
    assert_eq!(b.exp_strict_with(Ceiling), plus_ulp!(D57, 56, gb));
    assert_eq!(b.exp_strict_with(Trunc), gb);
}

// ── cosh ────────────────────────────────────────────────────────────

#[test]
fn cosh_narrow_sub_resolution_excess_bumps_ceiling() {
    // cosh(±1e-17) at D38<37>: the result is 1 + 5·10^-35 + 4.16·10^-70 —
    // the x⁴/24 term sits below the narrow working scale, but cosh of a
    // nonzero rational is irrational, so Ceiling must round the grid point
    // up one ULP; Floor/Trunc and the nearest modes keep it. cosh is even.
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
fn cosh_deep_excess_rounds_ceiling_up() {
    // cosh(±1e-693) at scale 923: the x²/2 excess (10^-1386/2) sits far
    // below the storage scale but is strictly positive — Ceiling rounds 1 up
    // one ULP; every other mode keeps exactly 1.
    let v = D924::<923>::from_str(&tiny(693)).unwrap();
    let one = D924::<923>::ONE;
    let up = plus_ulp!(D924, 923, one);
    for x in [v, -v] {
        assert_eq!(x.cosh_strict_with(Ceiling), up);
        for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
            assert_eq!(x.cosh_strict_with(mode), one, "{mode:?}");
        }
    }
}

#[test]
fn cosh_half_boundary_with_tail_rounds_up() {
    // cosh(±1e-308) at scale 616: the rest below the scale is `5` at the
    // half boundary plus the strictly positive x⁴/24 tail — above half, so
    // every nearest mode and Ceiling round up; Trunc/Floor keep 1.
    let v = D1232::<616>::from_str(&tiny(308)).unwrap();
    let g = D1232::<616>::ONE;
    let up = plus_ulp!(D1232, 616, g);
    assert_eq!(v.cosh_strict_with(HalfToEven), up);
    assert_eq!(v.cosh_strict_with(HalfTowardZero), up);
    assert_eq!(v.cosh_strict_with(HalfAwayFromZero), up);
    assert_eq!(v.cosh_strict_with(Ceiling), up);
    assert_eq!(v.cosh_strict_with(Trunc), g);
    assert_eq!(v.cosh_strict_with(Floor), g);
}

// ── sinh ────────────────────────────────────────────────────────────

#[test]
fn sinh_visible_cubic_keeps_directed_nudge() {
    // sinh(±1e-308) at scale 615: the cubic excess x³/6 (1.67·10^-925) sits
    // below the scale — sinh expands, so Ceiling rounds a positive argument
    // up one ULP (and Floor a negative one down); Trunc and the nearest
    // modes keep the grid line.
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
fn sinh_deep_cubic_keeps_directed_nudge() {
    // sinh(±1e-461) at scale 615: the cubic excess sits at 10^-1384 — far
    // below the scale, but strictly positive (sinh of a nonzero rational is
    // irrational), so the expanding nudge applies however deep it is.
    let v = D616::<615>::from_str(&tiny(461)).unwrap();
    assert_eq!(v.sinh_strict_with(Ceiling), plus_ulp!(D616, 615, v));
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.sinh_strict_with(mode), v, "pos {mode:?}");
    }
    let n = -v;
    assert_eq!(n.sinh_strict_with(Floor), n - D616::<615>::from_str(&tiny(615)).unwrap());
    for mode in [Ceiling, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(n.sinh_strict_with(mode), n, "neg {mode:?}");
    }
}

// ── tanh ────────────────────────────────────────────────────────────

#[test]
fn tanh_visible_deficit_keeps_compressing_nudge() {
    // tanh(±1e-616) at scale 1231: the cubic deficit x³/3 survives at the
    // carried precision (its 9-run below the leading digit is real) — tanh
    // compresses, so Trunc and Floor round a positive argument one ULP
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
fn tanh_deep_deficit_positive_tail_outcomes() {
    // tanh(±1e-693): the cubic deficit's relative position (~10^-1386 of the
    // value) lies past the carried precision — the carried value rounds back
    // to exactly x and only the never-exact positive sub-resolution tail
    // remains, so the outcomes flip to the just-above-the-line side: Ceiling
    // rounds a positive argument up one ULP (Floor a negative one down);
    // Trunc and the nearest modes keep raw. Checked at a deep scale and at
    // the scale where the argument is a single ULP.
    let v = D1232::<1231>::from_str(&tiny(693)).unwrap();
    assert_eq!(v.tanh_strict_with(Ceiling), plus_ulp!(D1232, 1231, v));
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.tanh_strict_with(mode), v, "pos {mode:?}");
    }
    let n = -v;
    assert_eq!(n.tanh_strict_with(Floor), n - D1232::<1231>::from_str(&tiny(1231)).unwrap());
    for mode in [Ceiling, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(n.tanh_strict_with(mode), n, "neg {mode:?}");
    }
    let u = D924::<693>::from_str(&tiny(693)).unwrap();
    assert_eq!(u.tanh_strict_with(Ceiling), plus_ulp!(D924, 693, u));
    for mode in [Floor, Trunc, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(u.tanh_strict_with(mode), u, "ulp pos {mode:?}");
    }
}

// ── exp2 ────────────────────────────────────────────────────────────

#[test]
fn exp2_hidden_tail_bumps_ceiling_only() {
    // exp2(1e-924) at scale 1231: the result is `1 + ln2·10^-924 + …`, whose
    // digits just below the scale (`ln 2`'s 308th and 309th digits) are zero
    // — but the tail beneath them never terminates, so Ceiling rounds up one
    // ULP while every other mode keeps the kept digits.
    let v = D1232::<1231>::from_str(&tiny(924)).unwrap();
    let t = v.exp2_strict_with(Trunc);
    for mode in [Floor, HalfToEven, HalfAwayFromZero, HalfTowardZero] {
        assert_eq!(v.exp2_strict_with(mode), t, "{mode:?}");
    }
    assert_eq!(v.exp2_strict_with(Ceiling), plus_ulp!(D1232, 1231, t));
}

#[test]
fn exp2_below_grid_nine_run_visible() {
    // exp2(-1e-924) at scale 1231: the value sits just below the next grid
    // line (digits `99…` under the scale): Trunc/Floor keep the lower grid
    // line, Ceiling and nearest round up one ULP.
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
    // `49…` — just below half — so nearest rounds DOWN (not a tie); Ceiling
    // rounds up.
    let n = -D1232::<1231>::from_str(&tiny(308)).unwrap();
    let t = n.exp2_strict_with(Trunc);
    assert_eq!(n.exp2_strict_with(HalfToEven), t);
    assert_eq!(n.exp2_strict_with(HalfAwayFromZero), t);
    assert_eq!(n.exp2_strict_with(Ceiling), plus_ulp!(D1232, 1231, t));
}
