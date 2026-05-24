//! Sanity-golden vectors for the EXACT (non-transcendental) decimal ops:
//! add, sub, mul, div, neg, rem, abs, signum, round, floor, ceil, trunc,
//! fract, rescale, div_euclid, rem_euclid, abs_diff, midpoint, pow.
//!
//! Every expected value is derived BY HAND from the documented
//! storage-level semantics. The crate is NOT consulted for the expected
//! result (that would be circular).
//!
//! D38s2 stores value*100 in its raw Int<2> (i128-range) storage: raw 100
//! == 1.00, raw -250 == -2.50, raw 1 == 0.01. i128::from(x.to_bits())
//! reads that raw integer back.
//!
//! Same-scale identities (src/macros/arithmetic.rs, rounding_methods.rs):
//!   add = raw_a + raw_b      sub = raw_a - raw_b      neg = -raw_a
//!   rem = raw_a % raw_b (truncated, sign of dividend)
//!   mul = round(raw_a * raw_b / 10^SCALE)
//!   div = round(raw_a * 10^SCALE / raw_b)
//!   abs = |raw_a|            signum = -ONE/ZERO/+ONE
//!   floor toward -inf, ceil toward +inf, trunc toward 0, fract = self-trunc
//!   round = half away from zero
//!
//! Gated to the HalfToEven default build so default-mode mul/div/round/
//! rescale vectors always assert.

#![cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
)))]

use decimal_scaled::{D38s2, D307s2, Int, RoundingMode};

const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

fn d(raw: i128) -> D38s2 {
    D38s2::from_bits(Int::<2>::try_from(raw).unwrap())
}
fn r(x: D38s2) -> i128 {
    i128::from(x.to_bits())
}
fn dw(raw: i128) -> D307s2 {
    D307s2::from_bits(Int::<16>::try_from(raw).unwrap())
}

// ===== ADD: raw_a + raw_b =====
#[test]
fn add_exact_vectors() {
    assert_eq!(r(d(125) + d(250)), 375);   // [middle] 1.25 + 2.50
    assert_eq!(r(d(420) + d(0)), 420);     // [zero id]
    assert_eq!(r(d(-100) + d(100)), 0);    // [cancel]
    assert_eq!(r(d(-150) + d(-250)), -400);// [both neg]
    assert_eq!(r(d(7) + d(93)), 100);      // [commute]
    assert_eq!(r(d(93) + d(7)), 100);
    assert_eq!(r(d(99) + d(1)), 100);      // [carry]
    assert_eq!(r(d(50) + d(-125)), -75);   // [cross 0]
    assert_eq!(r(d(100_000_000) + d(100)), 100_000_100); // [large]
    assert_eq!(r(d(0) + d(-333)), -333);   // [id right]
    assert_eq!(r(d(1) + d(1)), 2);         // [smallest unit]
}

// ===== SUB: raw_a - raw_b =====
#[test]
fn sub_exact_vectors() {
    assert_eq!(r(d(500) - d(125)), 375);   // [middle] 5.00 - 1.25
    assert_eq!(r(d(250) - d(250)), 0);     // [self]
    assert_eq!(r(d(314) - d(0)), 314);     // [x - 0]
    assert_eq!(r(d(0) - d(314)), -314);    // [0 - x]
    assert_eq!(r(d(-100) - d(200)), -300); // [neg - pos]
    assert_eq!(r(d(100) - d(-200)), 300);  // [pos - neg]
    assert_eq!(r(d(100) - d(1)), 99);      // [borrow]
    assert_eq!(r(d(25) - d(100)), -75);    // [cross 0]
    assert_eq!(r(d(-500) - d(-200)), -300);// [both neg]
    assert_eq!(r(d(100_000_000) - d(99)), 99_999_901); // [large]
}

// ===== NEG: -raw_a =====
#[test]
fn neg_exact_vectors() {
    assert_eq!(r(-d(125)), -125);
    assert_eq!(r(-d(-250)), 250);
    assert_eq!(r(-d(0)), 0);
    assert_eq!(r(-d(1)), -1);
    assert_eq!(r(-d(-1)), 1);
    assert_eq!(r(-d(100_000_000)), -100_000_000);
    assert_eq!(r(-(-d(314))), 314);
    assert_eq!(r(-d(99)), -99);
    assert_eq!(r(-d(-99)), 99);
    assert_eq!(r(-d(7)), -7);
}

// `-MIN` is unrepresentable in two's-complement. Like `i128`, the `Neg`
// operator panics on overflow in debug builds and WRAPS in release
// (`-MIN == MIN`). This test therefore only holds under `debug_assertions`;
// release callers that must detect the overflow use `checked_neg`.
#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn neg_min_panics() {
    let _ = -D38s2::MIN;
}

// ===== MUL: round(raw_a * raw_b / 100) =====
#[test]
fn mul_exact_vectors() {
    assert_eq!(r(d(150) * d(200)), 300);   // [middle] 1.50*2.00=3.00
    assert_eq!(r(d(314) * d(100)), 314);   // [by one]
    assert_eq!(r(d(999) * d(0)), 0);       // [by zero]
    assert_eq!(r(d(-200) * d(300)), -600); // [neg*pos]
    assert_eq!(r(d(-200) * d(-300)), 600); // [neg*neg]
    assert_eq!(r(d(50) * d(50)), 25);      // [fraction] 0.50*0.50=0.25
    assert_eq!(r(d(10) * d(10)), 1);       // [exact] 0.10*0.10=0.01
    assert_eq!(r(d(5) * d(5)), 0);         // 0.0025: 0.25 LSB HalfToEven->0
    assert_eq!(r(d(15) * d(10)), 2);       // 0.015: 1.5 LSB HalfToEven->2
    assert_eq!(r(d(100_000) * d(100_000)), 100_000_000); // [large]
}

// ===== DIV: round(raw_a * 100 / raw_b) =====
#[test]
fn div_exact_vectors() {
    assert_eq!(r(d(600) / d(200)), 300);   // [middle] 6/2=3
    assert_eq!(r(d(314) / d(100)), 314);   // [by one]
    assert_eq!(r(d(100) / d(400)), 25);    // [exact] 1/4=0.25
    assert_eq!(r(d(-600) / d(200)), -300); // [neg/pos]
    assert_eq!(r(d(-600) / d(-200)), 300); // [neg/neg]
    assert_eq!(r(d(100) / d(800)), 12);    // 0.125: 12.5 even=12
    assert_eq!(r(d(300) / d(800)), 38);    // 0.375: 37.5 even=38
    assert_eq!(r(d(100) / d(300)), 33);    // 1/3: 33.33 -> 33
    assert_eq!(r(d(200) / d(300)), 67);    // 2/3: 66.66 -> 67
    assert_eq!(r(d(500) / d(500)), 100);   // [identity]
}

#[test]
#[should_panic(expected = "divide by zero")]
fn div_by_zero_panics() {
    let _ = d(100) / d(0);
}

#[test]
fn div_one_third_all_modes() {
    let a = d(100);
    let b = d(300);
    for m in ALL_MODES {
        let got = r(a.div_with(b, m));
        let expected = match m {
            RoundingMode::Floor
            | RoundingMode::Trunc
            | RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => 33,
            RoundingMode::Ceiling => 34,
        };
        assert_eq!(got, expected, "div mode {m:?}");
    }
}

// ===== REM: raw_a % raw_b (truncated, sign of dividend) =====
#[test]
fn rem_exact_vectors() {
    assert_eq!(r(d(700) % d(200)), 100);   // [middle] 7%2=1
    assert_eq!(r(d(600) % d(200)), 0);     // [exact]
    assert_eq!(r(d(-700) % d(200)), -100); // [neg dividend]
    assert_eq!(r(d(700) % d(-200)), 100);  // [neg divisor]
    assert_eq!(r(d(-700) % d(-200)), -100);// [both neg]
    assert_eq!(r(d(100) % d(300)), 100);   // [|a|<|b|]
    assert_eq!(r(d(7) % d(3)), 1);         // [fractional] 0.07%0.03
    assert_eq!(r(d(500) % d(500)), 0);     // [a==b]
    assert_eq!(r(d(100_000_001) % d(100)), 1); // [large]
    assert_eq!(r(d(0) % d(300)), 0);       // [zero dividend]
}

#[test]
#[should_panic]
fn rem_by_zero_panics() {
    let _ = d(100) % d(0);
}

// `MIN % -1` hits the same quotient-overflow boundary as `MIN / -1`. Like
// `i128`, `Rem` panics on it in debug and WRAPS in release (remainder 0), so
// this only holds under `debug_assertions`.
#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn rem_min_by_neg_one_panics() {
    // The i128 `MIN % -1` quotient-overflow boundary needs the RAW divisor
    // to be exactly -1, i.e. the value -0.01 at scale 2 (raw -1) -- NOT
    // -ONE (raw -100, for which MIN % -100 is well-defined).
    let _ = D38s2::MIN % d(-1);
}

// ===== ABS / SIGNUM =====
#[test]
fn abs_exact_vectors() {
    assert_eq!(r(d(125).abs()), 125);
    assert_eq!(r(d(-125).abs()), 125);
    assert_eq!(r(d(0).abs()), 0);
    assert_eq!(r(d(1).abs()), 1);
    assert_eq!(r(d(-1).abs()), 1);
    assert_eq!(r(d(-100_000_000).abs()), 100_000_000);
    assert_eq!(r(d(99).abs()), 99);
    assert_eq!(r(d(-99).abs()), 99);
    assert_eq!(r(d(-7).abs()), 7);
    assert_eq!(r(d(250).abs()), 250);
}

#[test]
fn signum_exact_vectors() {
    assert_eq!(r(d(125).signum()), 100);
    assert_eq!(r(d(-125).signum()), -100);
    assert_eq!(r(d(0).signum()), 0);
    assert_eq!(r(d(1).signum()), 100);
    assert_eq!(r(d(-1).signum()), -100);
    assert_eq!(r(d(100_000_000).signum()), 100);
    assert_eq!(r(d(-100_000_000).signum()), -100);
    assert_eq!(r(d(99).signum()), 100);
    assert_eq!(r(d(-99).signum()), -100);
    assert_eq!(r(d(250).signum()), 100);
}

// ===== FLOOR (toward -inf) =====
#[test]
fn floor_exact_vectors() {
    assert_eq!(r(d(150).floor()), 100);
    assert_eq!(r(d(199).floor()), 100);
    assert_eq!(r(d(100).floor()), 100);   // exact
    assert_eq!(r(d(0).floor()), 0);
    assert_eq!(r(d(-150).floor()), -200); // -1.50 -> -2.00
    assert_eq!(r(d(-101).floor()), -200);
    assert_eq!(r(d(-100).floor()), -100); // exact
    assert_eq!(r(d(1).floor()), 0);       // 0.01 -> 0
    assert_eq!(r(d(-1).floor()), -100);   // -0.01 -> -1.00
    assert_eq!(r(d(250).floor()), 200);
}

// ===== CEIL (toward +inf) =====
#[test]
fn ceil_exact_vectors() {
    assert_eq!(r(d(150).ceil()), 200);
    assert_eq!(r(d(101).ceil()), 200);
    assert_eq!(r(d(100).ceil()), 100);    // exact
    assert_eq!(r(d(0).ceil()), 0);
    assert_eq!(r(d(-150).ceil()), -100);  // -1.50 -> -1.00
    assert_eq!(r(d(-199).ceil()), -100);
    assert_eq!(r(d(-100).ceil()), -100);  // exact
    assert_eq!(r(d(1).ceil()), 100);      // 0.01 -> 1.00
    assert_eq!(r(d(-1).ceil()), 0);       // -0.01 -> 0
    assert_eq!(r(d(250).ceil()), 300);
}

// ===== TRUNC (toward zero) =====
#[test]
fn trunc_exact_vectors() {
    assert_eq!(r(d(150).trunc()), 100);
    assert_eq!(r(d(199).trunc()), 100);
    assert_eq!(r(d(100).trunc()), 100);
    assert_eq!(r(d(0).trunc()), 0);
    assert_eq!(r(d(-150).trunc()), -100); // toward 0
    assert_eq!(r(d(-199).trunc()), -100);
    assert_eq!(r(d(-100).trunc()), -100);
    assert_eq!(r(d(1).trunc()), 0);
    assert_eq!(r(d(-1).trunc()), 0);
    assert_eq!(r(d(250).trunc()), 200);
}

// ===== FRACT (self - trunc) =====
#[test]
fn fract_exact_vectors() {
    assert_eq!(r(d(150).fract()), 50);
    assert_eq!(r(d(199).fract()), 99);
    assert_eq!(r(d(100).fract()), 0);
    assert_eq!(r(d(0).fract()), 0);
    assert_eq!(r(d(-150).fract()), -50);
    assert_eq!(r(d(-199).fract()), -99);
    assert_eq!(r(d(1).fract()), 1);
    assert_eq!(r(d(-1).fract()), -1);
    assert_eq!(r(d(250).fract()), 50);
    assert_eq!(r(d(314).fract()), 14);
}

// ===== ROUND (half AWAY from zero) =====
#[test]
fn round_exact_vectors() {
    assert_eq!(r(d(149).round()), 100);
    assert_eq!(r(d(150).round()), 200);   // half away
    assert_eq!(r(d(151).round()), 200);
    assert_eq!(r(d(100).round()), 100);   // exact
    assert_eq!(r(d(0).round()), 0);
    assert_eq!(r(d(-149).round()), -100);
    assert_eq!(r(d(-150).round()), -200); // half away
    assert_eq!(r(d(-151).round()), -200);
    assert_eq!(r(d(50).round()), 100);    // 0.50 -> 1.00
    assert_eq!(r(d(-50).round()), -100);
}

// ===== RESCALE: up appends zeros; down rounds (HalfToEven default) =====
#[test]
fn rescale_exact_vectors() {
    assert_eq!(i128::from(d(150).rescale::<6>().to_bits()), 1_500_000);
    assert_eq!(i128::from(d(-150).rescale::<6>().to_bits()), -1_500_000);
    assert_eq!(i128::from(d(1).rescale::<6>().to_bits()), 10_000);
    assert_eq!(i128::from(d(0).rescale::<6>().to_bits()), 0);
    assert_eq!(i128::from(d(150).rescale::<0>().to_bits()), 2);  // 1.50 even
    assert_eq!(i128::from(d(250).rescale::<0>().to_bits()), 2);  // 2.50 even
    assert_eq!(i128::from(d(350).rescale::<0>().to_bits()), 4);  // 3.50 even
    assert_eq!(i128::from(d(149).rescale::<0>().to_bits()), 1);
    assert_eq!(i128::from(d(151).rescale::<0>().to_bits()), 2);
    assert_eq!(i128::from(d(-250).rescale::<0>().to_bits()), -2);// -2.50 even
}

// ===== DIV_EUCLID (integer quotient, remainder >= 0) =====
#[test]
fn div_euclid_exact_vectors() {
    assert_eq!(r(d(700).div_euclid(d(200))), 300);
    assert_eq!(r(d(700).div_euclid(d(300))), 200);
    assert_eq!(r(d(-700).div_euclid(d(200))), -400);
    assert_eq!(r(d(700).div_euclid(d(-200))), -300);
    assert_eq!(r(d(-700).div_euclid(d(-200))), 400);
    assert_eq!(r(d(600).div_euclid(d(200))), 300);
    assert_eq!(r(d(100).div_euclid(d(300))), 0);
    assert_eq!(r(d(-100).div_euclid(d(300))), -100);
    assert_eq!(r(d(500).div_euclid(d(500))), 100);
    assert_eq!(r(d(0).div_euclid(d(300))), 0);
}

// ===== REM_EUCLID (result in [0, |b|)) =====
#[test]
fn rem_euclid_exact_vectors() {
    assert_eq!(r(d(700).rem_euclid(d(200))), 100);
    assert_eq!(r(d(-700).rem_euclid(d(200))), 100);
    assert_eq!(r(d(700).rem_euclid(d(-200))), 100);
    assert_eq!(r(d(-700).rem_euclid(d(-200))), 100);
    assert_eq!(r(d(600).rem_euclid(d(200))), 0);
    assert_eq!(r(d(100).rem_euclid(d(300))), 100);
    assert_eq!(r(d(-100).rem_euclid(d(300))), 200);
    assert_eq!(r(d(7).rem_euclid(d(3))), 1);
    assert_eq!(r(d(500).rem_euclid(d(500))), 0);
    assert_eq!(r(d(0).rem_euclid(d(300))), 0);
}

// ===== ABS_DIFF: |a - b| =====
#[test]
fn abs_diff_exact_vectors() {
    assert_eq!(r(d(500).abs_diff(d(200))), 300);
    assert_eq!(r(d(200).abs_diff(d(500))), 300);
    assert_eq!(r(d(0).abs_diff(d(0))), 0);
    assert_eq!(r(d(-100).abs_diff(d(100))), 200);
    assert_eq!(r(d(100).abs_diff(d(-100))), 200);
    assert_eq!(r(d(-500).abs_diff(d(-200))), 300);
    assert_eq!(r(d(314).abs_diff(d(314))), 0);
    assert_eq!(r(d(1).abs_diff(d(0))), 1);
    assert_eq!(r(d(99).abs_diff(d(100))), 1);
    assert_eq!(r(d(250).abs_diff(d(125))), 125);
}

// ===== MIDPOINT: (a + b) / 2 =====
#[test]
fn midpoint_exact_vectors() {
    assert_eq!(r(d(100).midpoint(d(300))), 200);
    assert_eq!(r(d(0).midpoint(d(0))), 0);
    assert_eq!(r(d(-100).midpoint(d(100))), 0);
    assert_eq!(r(d(200).midpoint(d(200))), 200);
    assert_eq!(r(d(-200).midpoint(d(-400))), -300);
    assert_eq!(r(d(100).midpoint(d(200))), 150);
    assert_eq!(r(d(0).midpoint(d(100))), 50);
    assert_eq!(r(d(50).midpoint(d(150))), 100);
    assert_eq!(r(d(1).midpoint(d(3))), 2);
    assert_eq!(r(d(700).midpoint(d(300))), 500);
}

// ===== POW (integer exponent) =====
#[test]
fn pow_exact_vectors() {
    assert_eq!(r(d(200).pow(0)), 100);   // x^0 = 1.00
    assert_eq!(r(d(200).pow(1)), 200);
    assert_eq!(r(d(200).pow(2)), 400);   // 2^2
    assert_eq!(r(d(200).pow(3)), 800);   // 2^3
    assert_eq!(r(d(300).pow(2)), 900);   // 3^2
    assert_eq!(r(d(-200).pow(2)), 400);  // (-2)^2
    assert_eq!(r(d(-200).pow(3)), -800); // (-2)^3
    assert_eq!(r(d(100).pow(5)), 100);   // 1^5
    assert_eq!(r(d(150).pow(2)), 225);   // 1.5^2 = 2.25
    assert_eq!(r(d(0).pow(3)), 0);
    assert_eq!(r(d(50).pow(2)), 25);     // 0.5^2 = 0.25
}

// ===== WIDE-TIER spot-check (D307 / Int<16>) =====
#[test]
fn wide_tier_exact_vectors() {
    assert_eq!(dw(125) + dw(250), dw(375));
    assert_eq!(dw(500) - dw(125), dw(375));
    assert_eq!(-dw(-250), dw(250));
    assert_eq!(dw(150) * dw(200), dw(300));
    assert_eq!(dw(600) / dw(200), dw(300));
    assert_eq!(dw(700) % dw(200), dw(100));
    assert_eq!(dw(-125).abs(), dw(125));
    assert_eq!(dw(-150).floor(), dw(-200));
    assert_eq!(dw(101).ceil(), dw(200));
    assert_eq!(dw(150).round(), dw(200));
}
