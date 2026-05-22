//! Coverage suite for `crate::macros::pow::decl_decimal_pow!` — the
//! integer-exponent `pow` / `powi` family, plus the four overflow
//! variants. Exercises D9 / D18 / D38 (and the wide tiers when their
//! features are on). D38 has its own hand-written pow path
//! (`powers_strict.rs::pow_int`), but its macro-aliased `pow` shares the
//! same square-and-multiply contract, so the same assertions hold.
//!
//! The contract from `macros/pow.rs`:
//! * `pow(0)` returns `ONE` for every base, including `ZERO`.
//! * `pow(1)` returns the base.
//! * For non-negative `exp`, `powi(exp) == pow(exp as u32)`.
//! * For negative `exp`, `powi(exp) == ONE / pow(exp.unsigned_abs())`.
//! * `checked_pow` returns `None` iff any intermediate `checked_mul`
//!   overflows.
//! * `wrapping_pow` matches the two's-complement wrap of the
//!   mathematical result.
//! * `saturating_pow` returns `MAX` on overflow when the mathematical
//!   result is positive, `MIN` when negative.
//! * `overflowing_pow` returns `(wrapping_result, true)` exactly when
//!   any step overflowed.

use decimal_scaled::{D18, D38};

// ─── Narrow-tier (D9<0>, D18<0>, D38<0>) base-case identities ──────────
//
// SCALE = 0 keeps the storage equal to the integer value so the
// assertions can be written in terms of plain integers.

#[test]
fn pow_exp_zero_returns_one_d18() {
    for raw in [0i64, 1, -1, 5, -5, 123_456_789_012i64] {
        let v = D18::<0>::from_bits(decimal_scaled::Int::<1>::from_i64(raw));
        assert_eq!(v.pow(0).to_bits(), 1, "0^0=1 contract: input raw {raw}");
    }
}

#[test]
fn pow_exp_one_returns_self_d18() {
    for raw in [0i64, 1, -1, 5, -5, 12345] {
        let v = D18::<0>::from_bits(decimal_scaled::Int::<1>::from_i64(raw));
        assert_eq!(v.pow(1).to_bits(), raw, "x^1=x");
    }
}

#[test]
fn pow_small_exponents_d18() {
    let two = D18::<0>::from_int(2);
    assert_eq!(two.pow(20).to_bits(), 1 << 20);
    let ten = D18::<0>::from_int(10);
    assert_eq!(ten.pow(9).to_bits(), 1_000_000_000);
}

#[test]
fn pow_small_exponents_d38() {
    let two = D38::<0>::from_int(2);
    assert_eq!(two.pow(30).to_bits(), 1i128 << 30);
    let ten = D38::<0>::from_int(10);
    assert_eq!(ten.pow(18).to_bits(), 1_000_000_000_000_000_000i128);
}

// ─── powi negative-exponent semantics ──────────────────────────────────
//
// `powi(-n)` = `ONE / pow(n)`. At SCALE=0 the integer divide truncates,
// so for |base| > 1 the result is ZERO. To exercise the
// reciprocal-path without hitting truncation, work at SCALE>0.

#[test]
fn powi_negative_exponent_d38_scale12() {
    use decimal_scaled::D38s12;
    let two = D38s12::from_int(2);
    // 2^-3 = 0.125 → raw 125_000_000_000
    assert_eq!(two.powi(-3).to_bits(), 125_000_000_000);
    // 2^3 == 8 (positive exp path)
    assert_eq!(two.powi(3).to_bits(), 8_000_000_000_000);
    // 2^0 == 1
    assert_eq!(two.powi(0).to_bits(), 1_000_000_000_000);
}

#[test]
fn powi_d9_d18_positive_negative_exp() {
    use decimal_scaled::{D18};

    // D18<4>: 2^3 = 8 → 80_000
    let two = D18::<4>::from_int(2);
    assert_eq!(two.powi(3).to_bits(), 80_000);
    assert_eq!(two.powi(0).to_bits(), 10_000);
    // 2^-3 = 0.125 → 1_250
    assert_eq!(two.powi(-3).to_bits(), 1_250);

    // D18
    let two18 = D18::<8>::from_int(2);
    assert_eq!(two18.powi(3).to_bits(), 800_000_000);
    assert_eq!(two18.powi(0).to_bits(), 100_000_000);
    assert_eq!(two18.powi(-3).to_bits(), 12_500_000);
}

#[test]
fn powi_handles_i32_min_without_signed_negation_overflow_d38() {
    // The `powi` code path uses `i32::unsigned_abs` to avoid the
    // signed-negation overflow that `(-i32::MIN) as u32` would cause.
    // Using ONE as the base keeps the multiply itself trivial so this
    // exercises the conversion edge alone.
    use decimal_scaled::D38s12;
    let one = D38s12::ONE;
    assert_eq!(one.powi(i32::MIN), D38s12::ONE);
    // ZERO base with i32::MIN exponent goes through ONE / pow(...) → ZERO
    // for any positive raised power, but 0^|MIN| would overflow the loop's
    // multiply. Skip this edge: behaviour matches the type's Mul.
}

// ─── checked_pow ───────────────────────────────────────────────────────

#[test]
fn checked_pow_normal_succeeds_d18() {
    let two = D18::<0>::from_int(2);
    assert_eq!(two.checked_pow(10), Some(D18::<0>::from_int(1024)));
    assert_eq!(two.checked_pow(0), Some(D18::<0>::ONE));
}

#[test]
fn checked_pow_overflow_returns_none_d18() {
    let ten = D18::<0>::from_int(10);
    assert!(ten.checked_pow(40).is_none(), "10^40 overflows D18<0>");
}

#[test]
fn checked_pow_overflow_returns_none_d38() {
    let ten = D38::<0>::from_int(10);
    assert!(ten.checked_pow(80).is_none(), "10^80 overflows D38<0>");
}

// ─── wrapping_pow ──────────────────────────────────────────────────────

#[test]
fn wrapping_pow_matches_arithmetic_d18() {
    let two = D18::<0>::from_int(2);
    // 2^10 == 1024, well within range.
    assert_eq!(two.wrapping_pow(10).to_bits(), 1024);
    // 2^63 wraps in i64: (2 as i64).wrapping_pow(63) == i64::MIN.
    assert_eq!(two.wrapping_pow(63).to_bits(), 2i64.wrapping_pow(63));
    // 2^64 wraps to zero (high bit only, drops).
    assert_eq!(two.wrapping_pow(64).to_bits(), 2i64.wrapping_pow(64));
}

#[test]
fn wrapping_pow_exp_zero_returns_one() {
    let v = D38::<0>::from_int(123);
    assert_eq!(v.wrapping_pow(0), D38::<0>::ONE);
}

// ─── saturating_pow ────────────────────────────────────────────────────

#[test]
fn saturating_pow_positive_overflow_saturates_to_max() {
    let ten = D18::<0>::from_int(10);
    assert_eq!(ten.saturating_pow(40), D18::<0>::MAX);
    let ten = D38::<0>::from_int(10);
    assert_eq!(ten.saturating_pow(80), D38::<0>::MAX);
}

#[test]
fn saturating_pow_negative_odd_saturates_to_min() {
    let neg_ten = D18::<0>::from_int(-10);
    assert_eq!(neg_ten.saturating_pow(41), D18::<0>::MIN);
    let neg_ten = D38::<0>::from_int(-10);
    assert_eq!(neg_ten.saturating_pow(81), D38::<0>::MIN);
}

#[test]
fn saturating_pow_negative_even_saturates_to_max() {
    // negative base raised to an even power is positive, so the
    // saturation direction is MAX, not MIN.
    let neg_ten = D18::<0>::from_int(-10);
    assert_eq!(neg_ten.saturating_pow(20), D18::<0>::MAX);
}

#[test]
fn saturating_pow_exp_zero_returns_one() {
    let v = D38::<0>::from_int(123);
    assert_eq!(v.saturating_pow(0), D38::<0>::ONE);
}

// ─── overflowing_pow ───────────────────────────────────────────────────

#[test]
fn overflowing_pow_no_overflow_returns_false() {
    let two = D18::<0>::from_int(2);
    let (v, ov) = two.overflowing_pow(10);
    assert_eq!(v.to_bits(), 1024);
    assert!(!ov);
}

#[test]
fn overflowing_pow_detects_overflow_d18_d38() {
    let ten18 = D18::<0>::from_int(10);
    let (v18, ov18) = ten18.overflowing_pow(40);
    assert!(ov18);
    assert_eq!(v18, ten18.wrapping_pow(40));

    let ten38 = D38::<0>::from_int(10);
    let (v38, ov38) = ten38.overflowing_pow(80);
    assert!(ov38);
    assert_eq!(v38, ten38.wrapping_pow(80));
}

#[test]
fn overflowing_pow_exp_zero_returns_one_no_overflow() {
    let v = D38::<0>::from_int(123);
    assert_eq!(v.overflowing_pow(0), (D38::<0>::ONE, false));
}

// ─── Wide-tier sanity (gated) ──────────────────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn pow_d76() {
    use decimal_scaled::D76;

    let two: D76<0> = D38::<0>::from_int(2).into();
    let r = two.pow(10);
    let expected: D76<0> = D38::<0>::from_int(1024).into();
    assert_eq!(r, expected);

    // exp=0 → ONE
    assert_eq!(two.pow(0), D76::<0>::ONE);
    // exp=1 → self
    assert_eq!(two.pow(1), two);
}

#[cfg(feature = "wide")]
#[test]
fn pow_d76_negative_base_odd_even() {
    use decimal_scaled::D76;

    let neg_two: D76<0> = D38::<0>::from_int(-2).into();
    let expected_pos: D76<0> = D38::<0>::from_int(16).into();
    let expected_neg: D76<0> = D38::<0>::from_int(-8).into();
    assert_eq!(neg_two.pow(4), expected_pos);
    assert_eq!(neg_two.pow(3), expected_neg);
}

#[cfg(feature = "wide")]
#[test]
fn checked_pow_overflow_d76() {
    use decimal_scaled::D76;

    let ten: D76<0> = D38::<0>::from_int(10).into();
    // D76<0> max ≈ 10^76. 10^80 overflows.
    assert!(ten.checked_pow(80).is_none());
    // 10^20 fits comfortably.
    assert!(ten.checked_pow(20).is_some());
}

#[cfg(feature = "wide")]
#[test]
fn saturating_overflowing_pow_d76() {
    use decimal_scaled::D76;

    let ten: D76<0> = D38::<0>::from_int(10).into();
    assert_eq!(ten.saturating_pow(80), D76::<0>::MAX);
    let (_, ov) = ten.overflowing_pow(80);
    assert!(ov);
    let neg_ten: D76<0> = D38::<0>::from_int(-10).into();
    assert_eq!(neg_ten.saturating_pow(81), D76::<0>::MIN);
}

#[cfg(feature = "x-wide")]
#[test]
fn pow_d153_d307() {
    use decimal_scaled::{D153, D307};

    let two_a: D153<0> = D38::<0>::from_int(2).into();
    assert_eq!(two_a.pow(0), D153::<0>::ONE);
    let two_b: D307<0> = D307::<0>::from_int(2);
    assert_eq!(two_b.pow(0), D307::<0>::ONE);
    // small exponent
    let r_a = two_a.pow(8);
    let expected_a: D153<0> = D38::<0>::from_int(256).into();
    assert_eq!(r_a, expected_a);
}
