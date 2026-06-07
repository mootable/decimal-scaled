//! Regression: `exp` of a large-negative argument underflows below the tier
//! resolution. The narrow exp kernel reassembles `2^k · exp(s)` with a deeply
//! negative `k` (≈ −320 for `exp(-222)`), shifting the 256-bit `Fixed` right by
//! ≥ 256 — which previously panicked (`Fixed::shr` overflowing the per-limb u128
//! shift). The correctly-rounded result is `0` under nearest, the smallest
//! positive under Ceiling; the kernel must return it, not panic.

use decimal_scaled::{DecimalTranscendental, RoundingMode, D18, D38};

#[test]
fn exp_deep_underflow_nearest_is_zero() {
    let a: D38<19> = "-222".parse().unwrap();
    assert_eq!(
        a.exp_strict_with(RoundingMode::HalfToEven).to_string(),
        "0.0000000000000000000"
    );

    let b: D18<9> = "-222".parse().unwrap();
    assert_eq!(
        b.exp_strict_with(RoundingMode::HalfToEven).to_string(),
        "0.000000000"
    );
}

#[test]
fn exp_deep_underflow_ceiling_is_smallest_positive() {
    // exp(-222) ≈ 4e-97 > 0, so Ceiling rounds up to the smallest representable
    // positive value at the tier scale (the directed-mode underflow path).
    let a: D38<19> = "-222".parse().unwrap();
    assert_eq!(
        a.exp_strict_with(RoundingMode::Ceiling).to_string(),
        "0.0000000000000000001"
    );
}
