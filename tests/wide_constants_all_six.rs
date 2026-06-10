//! Coverage suite for `consts_wide.rs` — all six wide-tier constants
//! (`pi`, `tau`, `half_pi`, `quarter_pi`, `golden`, `e`) on D76 / D153
//! / D307 at multiple scales.
//!
//! The existing `wide_constants_high_scale.rs` covers π and one high
//! scale per tier. This file exercises every constant on every wide
//! tier at the canonical reference scale and at a small storage scale.
//! Every test asserts the additive identities between the constants
//! (`τ = π + π`, `π = π/2 + π/2`, `π/2 = π/4 + π/4`, within 1 LSB to
//! absorb the independent per-constant rounding) plus coarse magnitude
//! bounds for `e` and `golden` — digit-exact values are the golden
//! gate's job, identities and reachability are this file's.

#![cfg(feature = "wide")]

use decimal_scaled::DecimalConstants;

/// Assert the six constants' additive identities + magnitude bounds at one
/// `(tier, scale)` cell. `$one_bits` builds the 1-LSB witness from raw storage.
macro_rules! check_constants {
    ($D:ty, $Int:ty) => {{
        type D = $D;
        let one_lsb = <D>::from_bits(<$Int>::try_from(1_i128).unwrap());
        let within_one_lsb = |a: D, b: D, what: &str| {
            let diff = if a > b { a - b } else { b - a };
            assert!(diff <= one_lsb, "{what}: diff = {diff:?}");
        };
        within_one_lsb(D::tau(), D::pi() + D::pi(), "tau vs pi + pi");
        within_one_lsb(D::pi(), D::half_pi() + D::half_pi(), "pi vs 2 half_pi");
        within_one_lsb(D::half_pi(), D::quarter_pi() + D::quarter_pi(), "half_pi vs 2 quarter_pi");
        let two: D = "2".parse().unwrap();
        let three: D = "3".parse().unwrap();
        let one: D = "1".parse().unwrap();
        assert!(D::e() > two && D::e() < three, "e in (2, 3)");
        assert!(D::golden() > one && D::golden() < two, "golden in (1, 2)");
    }};
}

#[test]
fn d76_all_six_constants_at_scale_12() {
    check_constants!(decimal_scaled::D76<12>, decimal_scaled::Int<4>);
}

#[test]
fn d76_all_six_constants_at_scale_37() {
    check_constants!(decimal_scaled::D76<37>, decimal_scaled::Int<4>);
}

#[cfg(feature = "x-wide")]
#[test]
fn d153_all_six_constants() {
    check_constants!(decimal_scaled::D153<37>, decimal_scaled::Int<8>);
    // The canonical reference scale (S = 152).
    check_constants!(decimal_scaled::D153<152>, decimal_scaled::Int<8>);
}

#[cfg(feature = "x-wide")]
#[test]
fn d307_all_six_constants() {
    check_constants!(decimal_scaled::D307<37>, decimal_scaled::Int<16>);
    check_constants!(decimal_scaled::D307<306>, decimal_scaled::Int<16>);
}
