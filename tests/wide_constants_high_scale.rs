//! Verifies the per-width raw constants in `consts_wide.rs` produce
//! correct values at the wide tiers' deeper scales — the case that
//! previously panicked on the rescale-up `i128` overflow.

// Truth strings below are the half-to-even-rounded pi reference; gate
// the file to the default rounding mode so every test always asserts.
#![cfg(all(
    feature = "wide",
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]

use decimal_scaled::{D76, D153, D307, DecimalConstants};

/// D76<76>::pi() used to panic at the i128 rescale-up. After wiring
/// the build-time-generated 75-digit Int256 constants, it returns a
/// well-defined value.
#[test]
fn d76_pi_at_max_scale_does_not_panic() {
    // SCALE=50: deeper than D38 but inside D76's max of 76.
    let pi50 = D76::<50>::pi();
    // Sanity: roughly 3 in integer part.
    assert!(pi50.to_bits().to_string().starts_with("314"));
}

#[test]
fn d76_pi_at_scale_75_is_exact() {
    // At SCALE = SCALE_REF (75), pi() returns the raw constant
    // exactly — no rescaling.
    let pi75 = D76::<75>::pi();
    let s = pi75.to_bits().to_string();
    // First few significant digits of pi (no decimal point in raw).
    assert!(
        s.starts_with("3141592653589793238462643383279502884"),
        "got {s}"
    );
}

#[test]
fn d153_pi_at_scale_152_works() {
    // v0.4.0 cap: MAX_SCALE for D153 is 152.
    let pi = D153::<152>::pi();
    let s = pi.to_bits().to_string();
    assert!(
        s.starts_with("3141592653589793238462643383279502884"),
        "got {s}"
    );
}

#[test]
fn d307_pi_at_scale_300_works() {
    let pi = D307::<300>::pi();
    let s = pi.to_bits().to_string();
    assert!(
        s.starts_with("3141592653589793238462643383279502884"),
        "got {s}"
    );
}

/// Cross-tier check: D76<37> and D38<37>::pi() should produce the
/// same logical value (the storage layouts differ but the rescaled
/// integer agrees). Uses the public `Decimal` trait to bridge.
#[test]
fn d76_pi_at_scale_37_matches_d38() {
    use decimal_scaled::D38;
    let n = D38::<37>::pi().to_bits().as_i128();
    let w = D76::<37>::pi().to_bits();
    let w_as_i128 = w.to_i128_checked().expect("fits");
    let diff = (w_as_i128 - n).abs();
    assert!(diff <= 1, "D76<37>::pi {w_as_i128} vs D38<37>::pi {n}");
}
