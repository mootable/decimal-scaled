//! Coverage for `macros/wide_roots.rs` — the plain `sqrt()` / `cbrt()`
//! dispatchers (strict-feature mode) and `hypot_strict` on the wide
//! tiers.

#![cfg(all(feature = "wide", not(feature = "fast")))]

use decimal_scaled::{D38, D76};

#[cfg(feature = "strict")]
#[test]
fn d76_sqrt_cbrt_plain_dispatcher() {

    let four: D76<6> = D38::<6>::from_int(4).into();
    let twenty_seven: D76<6> = D38::<6>::from_int(27).into();
    assert_eq!(four.sqrt(), four.sqrt_strict());
    assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt_strict());
}

#[test]
fn d76_hypot_strict_zero_zero() {

    assert_eq!(D76::<6>::ZERO.hypot_strict(D76::<6>::ZERO), D76::<6>::ZERO);
}

#[test]
fn d76_hypot_strict_zero_x() {

    let five: D76<6> = D38::<6>::from_int(5).into();
    let r = D76::<6>::ZERO.hypot_strict(five);
    // |5| with possible 1 LSB rounding
    let diff = (r.to_bits() - five.to_bits())
        .to_i128_checked()
        .unwrap_or(99)
        .abs();
    assert!(diff <= 1);
}

#[test]
fn d76_hypot_strict_3_4_is_5() {

    let three: D76<6> = D38::<6>::from_int(3).into();
    let four: D76<6> = D38::<6>::from_int(4).into();
    let five: D76<6> = D38::<6>::from_int(5).into();
    let r = three.hypot_strict(four);
    let diff = (r.to_bits() - five.to_bits())
        .to_i128_checked()
        .unwrap_or(99)
        .abs();
    assert!(diff <= 1, "got {r:?} expected {five:?}");
}

#[cfg(all(feature = "x-wide", feature = "strict"))]
#[test]
fn d153_d307_dispatchers_and_hypot() {
    use decimal_scaled::{D153, D307};

    let four: D153<6> = D38::<6>::from_int(4).into();
    assert_eq!(four.sqrt(), four.sqrt_strict());
    let twenty_seven: D153<6> = D38::<6>::from_int(27).into();
    assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt_strict());

    let three: D153<6> = D38::<6>::from_int(3).into();
    let four_a: D153<6> = D38::<6>::from_int(4).into();
    let _ = three.hypot_strict(four_a);

    let four_b: D307<6> = D76::<6>::from_int(4).into();
    let twenty_seven_b: D307<6> = D76::<6>::from_int(27).into();
    assert_eq!(four_b.sqrt(), four_b.sqrt_strict());
    assert_eq!(twenty_seven_b.cbrt(), twenty_seven_b.cbrt_strict());
    let three_b: D307<6> = D76::<6>::from_int(3).into();
    let _ = three_b.hypot_strict(four_b);
}
