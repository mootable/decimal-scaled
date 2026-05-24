//! Coverage for `macros/wide_roots.rs` — the plain `sqrt()` / `cbrt()`
//! dispatchers (strict-feature mode) and `hypot_strict` on the wide
//! tiers.

#![cfg(all(feature = "wide", not(feature = "fast")))]

use decimal_scaled::{D38, D76};

#[cfg(feature = "strict")]
#[test]
fn d76_sqrt_cbrt_plain_dispatcher() {

    let four: D76<6> = D38::<6>::try_from(4).unwrap().into();
    let twenty_seven: D76<6> = D38::<6>::try_from(27).unwrap().into();
    assert_eq!(four.sqrt(), four.sqrt_strict());
    assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt_strict());
}

#[test]
fn d76_hypot_strict_zero_zero() {

    assert_eq!(D76::<6>::ZERO.hypot_strict(D76::<6>::ZERO), D76::<6>::ZERO);
}

#[test]
fn d76_hypot_strict_zero_x() {

    let five: D76<6> = D38::<6>::try_from(5).unwrap().into();
    let r = D76::<6>::ZERO.hypot_strict(five);
    // hypot(0, x) = |x| exactly (isqrt(x²) = |x|, no rounding bump).
    assert_eq!(r, five);
}

#[test]
fn d76_hypot_strict_3_4_is_5() {

    let three: D76<6> = D38::<6>::try_from(3).unwrap().into();
    let four: D76<6> = D38::<6>::try_from(4).unwrap().into();
    let five: D76<6> = D38::<6>::try_from(5).unwrap().into();
    let r = three.hypot_strict(four);
    // Pythagorean triple 3²+4²=5²: the hypotenuse is an exact integer.
    assert_eq!(r, five, "got {r:?} expected exact {five:?}");
}

#[cfg(all(feature = "x-wide", feature = "strict"))]
#[test]
fn d153_d307_dispatchers_and_hypot() {
    use decimal_scaled::{D153, D307};

    let four: D153<6> = D38::<6>::try_from(4).unwrap().into();
    assert_eq!(four.sqrt(), four.sqrt_strict());
    let twenty_seven: D153<6> = D38::<6>::try_from(27).unwrap().into();
    assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt_strict());

    let three: D153<6> = D38::<6>::try_from(3).unwrap().into();
    let four_a: D153<6> = D38::<6>::try_from(4).unwrap().into();
    let five_a: D153<6> = D38::<6>::try_from(5).unwrap().into();
    // Pythagorean triple 3²+4²=5²: exact integer hypotenuse.
    assert_eq!(three.hypot_strict(four_a), five_a);

    let four_b: D307<6> = D76::<6>::try_from(4).unwrap().into();
    let twenty_seven_b: D307<6> = D76::<6>::try_from(27).unwrap().into();
    assert_eq!(four_b.sqrt(), four_b.sqrt_strict());
    assert_eq!(twenty_seven_b.cbrt(), twenty_seven_b.cbrt_strict());
    let three_b: D307<6> = D76::<6>::try_from(3).unwrap().into();
    let five_b: D307<6> = D76::<6>::try_from(5).unwrap().into();
    assert_eq!(three_b.hypot_strict(four_b), five_b);
}
