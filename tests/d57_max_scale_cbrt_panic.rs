//! Regression: `D57<MAX_SCALE = 57>::cbrt_strict` used to overflow the
//! work integer inside the generic-wide cbrt kernel.
//!
//! At SCALE=57 the kernel needs `mag * 10^(2*SCALE) = mag * 10^114`. A
//! `mag` of any non-trivial magnitude together with `10^114` reaches
//! `~10^115`+, which does not fit `Int384`'s ~`10^115` capacity. The
//! work-integer width was bumped to `Int768` so the kernel is correct
//! at any SCALE up to `D57::MAX_SCALE`.
//!
//! Note: At SCALE=57 the representable value range is bounded by
//! `Int192::MAX / 10^57 ≈ 3.14`, so all test inputs must be small.

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

use decimal_scaled::D57;

/// Reproducer from the bug report. Previously panicked with
/// `Int384: mul overflow` from inside the cbrt kernel; should now
/// run to completion (cbrt of a value near MAX returns a value near
/// `cbrt(MAX)`, which is the small cube-root of the saturated input).
#[test]
fn d57_max_scale_cbrt_does_not_panic() {
    let v = D57::<57>::from_f64(8.0);
    let _ = v.cbrt_strict();
}

/// cbrt(1.0) at D57<MAX_SCALE> — small input that fits the
/// representable range exactly.
#[test]
fn d57_max_scale_cbrt_of_one() {
    let one = D57::<57>::from_bits(decimal_scaled::Int192::from_str_radix(
        "1000000000000000000000000000000000000000000000000000000000",
        10,
    )
    .unwrap());
    let r = one.cbrt_strict();
    assert_eq!(r, one, "cbrt(1) at D57<57> should equal 1");
}

/// cbrt(0.125) = 0.5 at D57<MAX_SCALE>.
#[test]
fn d57_max_scale_cbrt_of_one_eighth() {
    // 0.125 = 1/8 at SCALE=57 → raw = 125 * 10^54.
    let raw = decimal_scaled::Int192::from_str_radix(
        "125000000000000000000000000000000000000000000000000000000",
        10,
    )
    .unwrap();
    let half_raw = decimal_scaled::Int192::from_str_radix(
        "500000000000000000000000000000000000000000000000000000000",
        10,
    )
    .unwrap();
    let v = D57::<57>::from_bits(raw);
    let expected = D57::<57>::from_bits(half_raw);
    let r = v.cbrt_strict();
    assert_eq!(r, expected, "cbrt(0.125) at D57<57> should equal 0.5");
}

/// cbrt(0.0) = 0.0 at D57<MAX_SCALE> — guard that the zero short-cut
/// path is unaffected.
#[test]
fn d57_max_scale_cbrt_of_zero() {
    let zero = D57::<57>::from_bits(decimal_scaled::Int192::from_str_radix("0", 10).unwrap());
    let r = zero.cbrt_strict();
    assert_eq!(r, zero, "cbrt(0) at D57<57> should equal 0");
}

