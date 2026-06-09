//! Regression: deep-underflow `exp` / `tanh` at the WIDE tiers. For a very
//! negative argument the wide exp kernel's range reduction provisions
//! `extra ≈ |k|·0.3` guard digits, pushing the working scale and the `k·ln2`
//! term past the work integer's capacity (`Int: mul overflow`). The kernel must
//! short-circuit to the correctly-rounded sub-resolution result — 0 under
//! nearest, the smallest representable positive under Ceiling — not panic.

#![cfg(all(feature = "wide", feature = "x-wide"))]

use decimal_scaled::{RoundingMode, D307, D57};

#[test]
fn wide_exp_deep_underflow_nearest_and_ceiling() {
    let a: D57<28> = "-1062".parse().unwrap();
    assert_eq!(
        a.exp_strict_with(RoundingMode::HalfToEven).to_string(),
        "0.0000000000000000000000000000" // 28 fractional zeros
    );
    assert_eq!(
        a.exp_strict_with(RoundingMode::Ceiling).to_string(),
        "0.0000000000000000000000000001" // smallest positive at scale 28
    );

    let b: D307<153> = "-2331".parse().unwrap();
    let zeros = format!("0.{}", "0".repeat(153));
    assert_eq!(
        b.exp_strict_with(RoundingMode::HalfToEven).to_string(),
        zeros
    );
}

#[test]
fn wide_tanh_deep_underflow() {
    // tanh(-1063) computes exp(-2126) internally (deep underflow); the result is
    // ≈ -1, which must round (not panic).
    let t: D57<28> = "-1063".parse().unwrap();
    let r = t.tanh_strict_with(RoundingMode::HalfToEven).to_string();
    assert_eq!(r, "-1.0000000000000000000000000000");
}
