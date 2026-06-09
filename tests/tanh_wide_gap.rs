//! Wide `tanh` in the overflow GAP band (the latent twin of the narrow-D38 gap
//! fixed in `trig_series_2limb`). At a deep-scale wide tier the working scale `w`
//! is large enough (`w ≳ 0.67·S digits`) that forming the dominant `e^(+|x|)`
//! overflows the work integer BELOW the all-nines saturation onset `thr_x` — a
//! band where the old direct `(e^x − e^-x)/(e^x + e^-x)` form panicked. The
//! `(1 − m)/(1 + m)`, `m = e^(−2|x|)` identity (in `exp_generic::tanh_pos`) must
//! return the bounded value without ever forming `e^(+|x|)`.
//!
//! NOT covered by the golden set (no harvested input lands in this band), so this
//! is the dedicated proof. `tanh` is bounded in (−1, 1), so it can never overflow
//! the tier — a panic here is the bug.
#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use decimal_scaled::{RoundingMode, D1232};

#[test]
fn wide_tanh_overflow_gap_rounds_to_one_without_panicking() {
    // D1232<924>: w = 924 + GUARD ≈ 954, saturation onset thr_x ≈ 1101, and
    // `e^x` overflows the work integer for x ≳ 643 — so x = 1070 sits in the GAP
    // (above the overflow point, below thr_x). It is also deep enough that
    // 1 − tanh(1070) = 2·e^(−2140) ≈ 10^-929 < ½·ULP(10^-924), so tanh rounds to
    // exactly 1.0. The old direct form would form e^1070 (≈10^1418, past the
    // ~1233-digit work int) and panic; the identity must yield 1.0.
    // Inputs in the gap [~643, thr_x≈1101] AND deep enough (x ≳ 1064) that
    // 1 − tanh < ½·ULP, so each rounds to exactly ±1.0 via the (1−m)/(1+m) path.
    let one: D1232<924> = "1".parse().unwrap();
    for s in ["1070", "1090", "-1070", "-1090"] {
        let x: D1232<924> = s.parse().unwrap();
        let got = x.tanh_strict_with(RoundingMode::HalfToEven);
        let expected = if s.starts_with('-') { -one } else { one };
        assert_eq!(got, expected, "tanh({s}) @ D1232<924> in the overflow gap");
    }
}
