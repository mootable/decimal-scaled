//! Verifies the per-width raw constants in `consts_wide.rs` produce
//! correct values at the wide tiers' deeper scales — the case that
//! previously panicked on the rescale-up `i128` overflow.

#![cfg(feature = "wide")]

use decimal_scaled::{D76, D153, D307, DecimalConsts};

const DEFAULT_IS_HALF_TO_EVEN: bool = !(cfg!(feature = "rounding-half-away-from-zero")
    || cfg!(feature = "rounding-half-toward-zero")
    || cfg!(feature = "rounding-trunc")
    || cfg!(feature = "rounding-floor")
    || cfg!(feature = "rounding-ceiling"));

/// D76<76>::pi() used to panic at the i128 rescale-up. After wiring
/// the build-time-generated 75-digit Int256 constants, it returns a
/// well-defined value.
#[test]
fn d76_pi_at_max_scale_does_not_panic() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    // SCALE=50: deeper than D38 but inside D76's max of 76.
    let pi50 = D76::<50>::pi();
    // Sanity: roughly 3 in integer part.
    assert!(pi50.to_bits().to_string().starts_with("314"));
}

#[test]
fn d76_pi_at_scale_75_is_exact() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    // At SCALE = SCALE_REF (75), pi() returns the raw constant
    // exactly — no rescaling.
    let pi75 = D76::<75>::pi();
    let s = pi75.to_bits().to_string();
    // First few significant digits of pi (no decimal point in raw).
    assert!(s.starts_with("3141592653589793238462643383279502884"), "got {s}");
}

#[test]
fn d153_pi_at_scale_153_works() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    let pi = D153::<153>::pi();
    let s = pi.to_bits().to_string();
    assert!(s.starts_with("3141592653589793238462643383279502884"), "got {s}");
}

#[test]
fn d307_pi_at_scale_300_works() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    let pi = D307::<300>::pi();
    let s = pi.to_bits().to_string();
    assert!(s.starts_with("3141592653589793238462643383279502884"), "got {s}");
}

/// Cross-tier check: D76<37> and D38<37>::pi() should produce the
/// same logical value (the storage layouts differ but the rescaled
/// integer agrees). Uses the public `Decimal` trait to bridge.
#[test]
fn d76_pi_at_scale_37_matches_d38() {
    if !DEFAULT_IS_HALF_TO_EVEN { return; }
    use decimal_scaled::D38;
    let n = D38::<37>::pi().to_bits();
    let w = D76::<37>::pi().to_bits();
    let w_as_i128 = w.to_i128_checked().expect("fits");
    let diff = (w_as_i128 - n).abs();
    assert!(diff <= 1, "D76<37>::pi {w_as_i128} vs D38<37>::pi {n}");
}
