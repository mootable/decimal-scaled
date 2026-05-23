//! Integration tests for the composed cross-width + cross-scale decimal
//! `convert_from` / `convert_from_with` constructors.
//!
//! These exercise the value-preserving width/scale ordering: widen then
//! rescale when the target is at least as wide as the source, rescale
//! then narrow when the target is narrower. Behaviour is asserted on
//! exact stored values.
//!
//! The default-mode (`convert_from`) cases assume `HalfToEven`, so the
//! whole file is compile-gated to a default-rounding build; that way no
//! test silently no-ops under a `rounding-*` feature.

#![cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
)))]

use decimal_scaled::{ConvertError, D18, D38, Int, RoundingMode};

/// Raw `D38<S>` constructor: `raw` is the stored integer (logical value
/// `raw / 10^S`).
fn d38_raw<const S: u32>(raw: i128) -> D38<S> {
    D38::<S>::from_bits(Int::<2>::try_from(raw).unwrap())
}

/// Raw `D18<S>` constructor.
fn d18_raw<const S: u32>(raw: i64) -> D18<S> {
    D18::<S>::from_bits(Int::<1>::try_from(raw as i128).unwrap())
}

// ── widen + scale-up (exact) ─────────────────────────────────────────

/// Widening to a wider tier and scaling UP appends zeros exactly; no
/// rounding occurs and the result is `Ok`.
#[test]
fn widen_scale_up_is_exact() {
    // 1.50 @ D18<2> -> D38<6> == 1.500000 (raw 1_500_000).
    let src: D18<2> = d18_raw::<2>(150);
    let out: D38<6> = D38::<6>::convert_from(src).unwrap();
    assert_eq!(i128::from(out.to_bits()), 1_500_000);
}

/// Same width, scale-up (equal-width branch).
#[test]
fn same_width_scale_up_is_exact() {
    let src: D38<2> = d38_raw::<2>(150);
    let out: D38<6> = D38::<6>::convert_from(src).unwrap();
    assert_eq!(i128::from(out.to_bits()), 1_500_000);
}

// ── widen + scale-down (rounding) ────────────────────────────────────

/// Widening with a scale-DOWN rounds the discarded digits per the mode.
/// 1.2345 @ D18<4> -> D38<2>: 1.2345 rounds half-to-even to 1.23 (the
/// dropped `45` is below the half boundary `50`), i.e. raw 123.
#[test]
fn widen_scale_down_rounds_half_to_even() {
    let src: D18<4> = d18_raw::<4>(12_345);
    let out: D38<2> = D38::<2>::convert_from(src).unwrap();
    assert_eq!(i128::from(out.to_bits()), 123);
}

/// Explicit-mode scale-down: an exact half rounds differently per mode.
/// 2.5 @ S=1 -> S=0. Half-to-even -> 2; half-away -> 3; ceiling -> 3;
/// floor/trunc -> 2.
#[test]
fn widen_scale_down_respects_explicit_mode() {
    let src: D18<1> = d18_raw::<1>(25); // 2.5
    let even: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::HalfToEven).unwrap();
    assert_eq!(i128::from(even.to_bits()), 2);
    let away: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::HalfAwayFromZero).unwrap();
    assert_eq!(i128::from(away.to_bits()), 3);
    let ceil: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::Ceiling).unwrap();
    assert_eq!(i128::from(ceil.to_bits()), 3);
    let floor: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::Floor).unwrap();
    assert_eq!(i128::from(floor.to_bits()), 2);
}

// ── narrow that fits (Ok, value preserved) ───────────────────────────

/// Narrowing a value that fits the target storage at the same scale is
/// exact and `Ok`.
#[test]
fn narrow_that_fits_preserves_value() {
    // 7.50 @ D38<2> -> D18<2>: comfortably fits i64.
    let src: D38<2> = d38_raw::<2>(750);
    let out: D18<2> = D18::<2>::convert_from(src).unwrap();
    assert_eq!(i128::from(out.to_bits()), 750);
}

// ── narrow that overflows (Err) ──────────────────────────────────────

/// Narrowing a magnitude that does not fit the target storage at the
/// requested scale returns `Err(Overflow)`.
#[test]
fn narrow_overflow_is_err() {
    // ~1e30 stored in D38<0> cannot fit i64 (max ~9.2e18) at scale 0.
    let big: i128 = 1_000_000_000_000_000_000_000_000_000_000; // 1e30
    let src: D38<0> = d38_raw::<0>(big);
    let out: Result<D18<0>, ConvertError> = D18::<0>::convert_from(src);
    assert_eq!(out, Err(ConvertError::Overflow));
}

// ── precision-preserving narrow with scale-down ──────────────────────

/// The motivating case for rescaling at the SOURCE (wider) width before
/// narrowing: a value that does NOT fit the target at the source scale,
/// but DOES fit after the scale-down, must convert to `Ok` (not `Err`).
///
/// Source: 5e19 @ D38<2> (logical 5e17). i64::MAX is ~9.2e18, so the
/// raw 5e19 magnitude does not fit i64. Converting to D18<0> scales
/// down by 10^2 -> raw 5e17, which DOES fit i64. If the narrow were
/// done first this would spuriously overflow; the source-width-first
/// ordering makes it succeed.
#[test]
fn narrow_with_scale_down_fits_after_rescale() {
    let raw_src: i128 = 50_000_000_000_000_000_000; // 5e19, > i64::MAX
    assert!(raw_src > i128::from(i64::MAX));
    let src: D38<2> = d38_raw::<2>(raw_src); // logical 5e17
    let out: D18<0> = D18::<0>::convert_from(src).unwrap();
    // 5e17 fits i64 (< 9.2e18).
    assert_eq!(i128::from(out.to_bits()), 500_000_000_000_000_000);
}

// ── same width, same scale (identity) ────────────────────────────────

/// Same width and same scale is the identity (bit-for-bit).
#[test]
fn same_width_same_scale_is_identity() {
    let src: D38<6> = d38_raw::<6>(1_234_567);
    let out: D38<6> = D38::<6>::convert_from(src).unwrap();
    assert_eq!(i128::from(out.to_bits()), 1_234_567);

    // Cross-width but same scale, identity value.
    let narrow: D18<6> = d18_raw::<6>(9_999);
    let widened: D38<6> = D38::<6>::convert_from(narrow).unwrap();
    assert_eq!(i128::from(widened.to_bits()), 9_999);
}

// ── round-trip (lossless) ────────────────────────────────────────────

/// A lossless widen followed by the inverse narrow recovers the original
/// value. Widening up in scale + width then narrowing back is exact when
/// no precision is dropped.
#[test]
fn round_trip_widen_then_narrow_is_lossless() {
    let original: D18<2> = d18_raw::<2>(4_242);
    // Widen to D38<6> (scale-up, exact), then narrow back to D18<2>.
    let wide: D38<6> = D38::<6>::convert_from(original).unwrap();
    let back: D18<2> = D18::<2>::convert_from(wide).unwrap();
    assert_eq!(i128::from(back.to_bits()), i128::from(original.to_bits()));
}

/// Negative values preserve sign through both branches.
#[test]
fn negative_values_round_correctly() {
    // -1.2345 @ D18<4> -> D38<2>: half-to-even, dropped 45 < 50 -> -1.23.
    let src: D18<4> = d18_raw::<4>(-12_345);
    let out: D38<2> = D38::<2>::convert_from(src).unwrap();
    assert_eq!(i128::from(out.to_bits()), -123);

    // Negative narrow that fits.
    let neg: D38<2> = d38_raw::<2>(-750);
    let narrowed: D18<2> = D18::<2>::convert_from(neg).unwrap();
    assert_eq!(i128::from(narrowed.to_bits()), -750);
}
