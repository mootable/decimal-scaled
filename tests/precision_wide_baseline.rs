//! Measurement tests for the wide-tier strict transcendentals.
//!
//! Companion to `precision_strict_05_ulp.rs` (which asserts 0.5 ULP
//! for D128). This file records the **current** ULP gap of the wide
//! tier so we can see when the contract tightens. Assertions are
//! lenient now (≤ WIDE_TOLERANCE_LSB) and will tighten to 1 LSB once
//! the wide tier hits 0.5 ULP.
//!
//! Storage values at SCALE=6 fit `i128` cleanly, so comparison goes
//! through `to_i128_checked().unwrap()` on each wide-int result.

#![cfg(all(not(feature = "no_strict"), feature = "wide"))]

use decimal_scaled::D256;

const DEFAULT_IS_HALF_TO_EVEN: bool = !(cfg!(feature = "rounding-half-away-from-zero")
    || cfg!(feature = "rounding-half-toward-zero")
    || cfg!(feature = "rounding-trunc")
    || cfg!(feature = "rounding-floor")
    || cfg!(feature = "rounding-ceiling"));

/// Current wide-tier ULP gap allowance. Once truncating intermediate
/// `mul` / `div` are replaced with rounded ops this drops to 1.
const WIDE_TOLERANCE_LSB: i128 = 10;

#[track_caller]
fn agree_within(label: &str, wide: i128, narrow: i128) {
    let diff = (wide - narrow).abs();
    assert!(
        diff <= WIDE_TOLERANCE_LSB,
        "{label}: wide {wide} vs d128 {narrow} (diff {diff} > {WIDE_TOLERANCE_LSB} LSB)",
    );
}

fn wide_bits(d: D256<6>) -> i128 {
    d.to_bits()
        .to_i128_checked()
        .expect("D256<6> result fits i128 at SCALE=6")
}

#[test]
fn ln_d256_baseline() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    use decimal_scaled::D128;
    type D128_6 = D128<6>;
    let n = D128_6::from_int(2);
    let w: D256<6> = n.into();
    agree_within(
        "D256<6>::ln(2)",
        wide_bits(w.ln_strict()),
        n.ln_strict().to_bits(),
    );
}

#[test]
fn exp_d256_baseline() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    use decimal_scaled::D128;
    type D128_6 = D128<6>;
    let n = D128_6::ONE;
    let w: D256<6> = n.into();
    agree_within(
        "D256<6>::exp(1)",
        wide_bits(w.exp_strict()),
        n.exp_strict().to_bits(),
    );
}

#[test]
fn sin_d256_baseline() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    use decimal_scaled::D128;
    type D128_6 = D128<6>;
    for raw in [1_000_000i64, 2_345_678i64, 7_500_000i64] {
        let n = D128_6::from_bits(raw as i128);
        let w: D256<6> = n.into();
        agree_within("sin", wide_bits(w.sin_strict()), n.sin_strict().to_bits());
    }
}

#[test]
fn atan_d256_baseline() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    use decimal_scaled::D128;
    type D128_6 = D128<6>;
    for raw in [1_000_000i64, -1_500_000i64, 3_000_000i64] {
        let n = D128_6::from_bits(raw as i128);
        let w: D256<6> = n.into();
        agree_within("atan", wide_bits(w.atan_strict()), n.atan_strict().to_bits());
    }
}

#[test]
fn sqrt_d256_tight() {
    // Roots are exact; should be within 1 LSB always.
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    use decimal_scaled::D128;
    type D128_6 = D128<6>;
    for raw in [4_000_000i64, 9_000_000i64, 16_000_000i64, 25_000_000i64] {
        let n = D128_6::from_bits(raw as i128);
        let w: D256<6> = n.into();
        let wide = wide_bits(w.sqrt_strict());
        let narrow = n.sqrt_strict().to_bits();
        let diff = (wide - narrow).abs();
        assert!(
            diff <= 1,
            "sqrt({raw}): wide {wide} vs d128 {narrow} (diff {diff} > 1 LSB)",
        );
    }
}
