//! `requantize` — change storage width and SCALE in one call, any direction.
//!
//! The order matters: scaling UP at a narrow width can overflow where the target
//! width would hold the value, so growing must widen first. These tests pin that
//! behaviour, not just the happy path.
//!
//! Note the call style: the target width and SCALE are inferred from the binding
//! (`let r: D38<6> = a.requantize();`). A caller never writes limb counts — those
//! are an internal representation detail and must not appear at a call site.

use decimal_scaled::{D18, D38, RoundingMode};

#[test]
fn identity_requantize_is_a_no_op() {
    let a = D18::<2>::try_from(42i64).unwrap();
    let r: D18<2> = a.requantize();
    assert_eq!(r, a);
}

#[test]
fn widen_and_scale_up_together() {
    let a = D18::<2>::try_from(7i64).unwrap();
    let r: D38<6> = a.requantize();
    assert_eq!(r, D38::<6>::try_from(7i64).unwrap());
    assert_eq!(r.to_string(), "7.000000");
}

#[test]
fn narrow_and_scale_down_together() {
    let a = D38::<6>::try_from(7i64).unwrap();
    let r: D18<2> = a.requantize();
    assert_eq!(r, D18::<2>::try_from(7i64).unwrap());
    assert_eq!(r.to_string(), "7.00");
}

#[test]
fn growing_scales_up_at_the_wider_width_not_the_narrow_one() {
    // This is the case that a naive "quantize then resize" gets wrong: the
    // scale-up would overflow D18 even though D38 holds the result easily.
    let a = D18::<0>::try_from(1_000_000_000_000_000_000i64).unwrap(); // 1e18
    let r: D38<6> = a.requantize(); // needs 1e24
    assert_eq!(r.to_string(), "1000000000000000000.000000");
}

#[test]
fn scale_down_rounds_with_the_default_mode() {
    // 0.005 -> SCALE 2 under HalfToEven -> 0.00
    let a = D38::<6>::from_bits(decimal_scaled::Int::<2>::from(5_000i64));
    let r: D18<2> = a.requantize();
    assert_eq!(r.to_string(), "0.00");
}

#[test]
fn scale_down_honours_an_explicit_rounding_mode() {
    let a = D38::<6>::from_bits(decimal_scaled::Int::<2>::from(5_000i64)); // 0.005
    let up: D18<2> = a.requantize_with(RoundingMode::HalfAwayFromZero);
    assert_eq!(up.to_string(), "0.01");
    let down: D18<2> = a.requantize_with(RoundingMode::Trunc);
    assert_eq!(down.to_string(), "0.00");
}

#[test]
fn negative_values_survive_both_directions() {
    let a = D18::<2>::try_from(-7i64).unwrap();
    let wide: D38<6> = a.requantize();
    assert_eq!(wide, D38::<6>::try_from(-7i64).unwrap());
    let back: D18<2> = wide.requantize();
    assert_eq!(back, a);
}

#[test]
fn zero_requantizes_cleanly() {
    let a = D18::<2>::try_from(0i64).unwrap();
    let r: D38<9> = a.requantize();
    assert_eq!(r, D38::<9>::try_from(0i64).unwrap());
}

#[test]
#[should_panic(expected = "attempt to requantize with overflow")]
fn narrowing_a_value_that_does_not_fit_panics() {
    let big = D38::<0>::try_from(i64::MAX).unwrap() * D38::<0>::try_from(1_000i64).unwrap();
    let _: D18<0> = big.requantize();
}

#[test]
#[should_panic(expected = "attempt to requantize with overflow")]
fn scaling_up_past_the_target_width_panics() {
    // fits D18<0>, but 1e18 * 10^6 does not fit D18 at SCALE 6
    let a = D18::<0>::try_from(1_000_000_000_000_000_000i64).unwrap();
    let _: D18<6> = a.requantize();
}
