//! Coverage suite for `consts_wide.rs` — all six wide-tier constants
//! (`pi`, `tau`, `half_pi`, `quarter_pi`, `golden`, `e`) on D76 / D153
//! / D307 at multiple scales.
//!
//! The existing `wide_constants_high_scale.rs` covers π and one high
//! scale per tier. This file exercises every constant on every wide
//! tier at the canonical reference scale and at a small storage scale,
//! so every `*_at_target_d76`, `*_at_target_d153`, `*_at_target_d307`
//! function is invoked.

#![cfg(feature = "wide")]

use decimal_scaled::DecimalConstants;

#[test]
fn d76_all_six_constants_at_scale_12() {
    use decimal_scaled::D76;
    type D = D76<12>;
    let _pi = D::pi();
    let _tau = D::tau();
    let _half_pi = D::half_pi();
    let _quarter_pi = D::quarter_pi();
    let _e = D::e();
    let _golden = D::golden();

    // Consistency: tau == 2*pi (within 1 LSB at the storage scale).
    let tau_via_pi = D::pi() + D::pi();
    let tau = D::tau();
    // Cross-witness: compute the difference via the wide arithmetic. The
    // two paths share the same reference, so the diff should be small.
    let diff = if tau > tau_via_pi {
        tau - tau_via_pi
    } else {
        tau_via_pi - tau
    };
    // Allow up to 1 LSB slack to absorb the independent rounding paths.
    let one: D = decimal_scaled::D38::<12>::from_bits(decimal_scaled::Int::<2>::from_i128(1)).into();
    assert!(diff <= one, "tau vs 2*pi diff = {diff:?}");
}

#[test]
fn d76_all_six_constants_at_scale_37() {
    use decimal_scaled::D76;
    type D = D76<37>;
    let _ = D::pi();
    let _ = D::tau();
    let _ = D::half_pi();
    let _ = D::quarter_pi();
    let _ = D::e();
    let _ = D::golden();
}

#[cfg(feature = "x-wide")]
#[test]
fn d153_all_six_constants() {
    use decimal_scaled::D153;
    type D = D153<37>;
    let _ = D::pi();
    let _ = D::tau();
    let _ = D::half_pi();
    let _ = D::quarter_pi();
    let _ = D::e();
    let _ = D::golden();

    // Also at the canonical reference scale (S=152, v0.4.0 new MAX_SCALE).
    type D2 = D153<152>;
    let _ = D2::pi();
    let _ = D2::tau();
    let _ = D2::half_pi();
    let _ = D2::quarter_pi();
    let _ = D2::e();
    let _ = D2::golden();
}

#[cfg(feature = "x-wide")]
#[test]
fn d307_all_six_constants() {
    use decimal_scaled::D307;
    type D = D307<37>;
    let _ = D::pi();
    let _ = D::tau();
    let _ = D::half_pi();
    let _ = D::quarter_pi();
    let _ = D::e();
    let _ = D::golden();

    type D2 = D307<306>;
    let _ = D2::pi();
    let _ = D2::tau();
    let _ = D2::half_pi();
    let _ = D2::quarter_pi();
    let _ = D2::e();
    let _ = D2::golden();
}
