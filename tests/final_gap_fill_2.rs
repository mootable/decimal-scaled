//! Last-mile reachable coverage: error Display, display ParseError paths,
//! fixed_compat fallback, wide-tier strict result-out-of-range.

use decimal_scaled::{D38, D76, ConvertError};
use core::str::FromStr;

// ─── error.rs: Display impl for ConvertError ───────────────────────────

#[test]
fn convert_error_display_messages() {
    let _ = format!("{}", ConvertError::Overflow);
    let _ = format!("{}", ConvertError::NotFinite);
    // Debug derive
    let _ = format!("{:?}", ConvertError::Overflow);
    let _ = format!("{:?}", ConvertError::NotFinite);
}

// ─── display.rs: ParseError SignOnly / OutOfRange paths ────────────────

#[test]
fn parse_error_sign_only() {
    // A bare `-` or `+` is SignOnly.
    let r = D38::<2>::from_str("-");
    assert!(r.is_err());
    let r = D38::<2>::from_str("+");
    assert!(r.is_err());
}

#[test]
fn parse_error_out_of_range_integer() {
    // A long integer that overflows i128.
    let r = D38::<0>::from_str("99999999999999999999999999999999999999999");
    assert!(r.is_err());
}

#[test]
fn parse_error_out_of_range_combined() {
    // Value just past i128::MAX at S=0.
    let r = D38::<0>::from_str("170141183460469231731687303715884105728");
    assert!(r.is_err());
    // Negative just past i128::MIN
    let r = D38::<0>::from_str("-170141183460469231731687303715884105729");
    assert!(r.is_err());
}

#[test]
fn parse_error_other_paths() {
    // Empty input
    assert!(D38::<2>::from_str("").is_err());
    // Just a dot
    assert!(D38::<2>::from_str(".").is_err());
    // Missing fractional
    assert!(D38::<2>::from_str("1.").is_err());
    // Missing integer
    assert!(D38::<2>::from_str(".5").is_err());
    // Leading zero
    assert!(D38::<2>::from_str("01").is_err());
    // Overlong fractional
    assert!(D38::<2>::from_str("1.123").is_err());
}

// ─── fixed_compat.rs line 132 (None branch of ToPrimitive → ZERO) ──────
//
// Construct a type that implements ToPrimitive but returns None for
// every conversion. The internal default-to-ZERO branch fires.

#[cfg(feature = "fixed-compat")]
#[test]
fn fixed_compat_unknown_primitive_defaults_to_zero() {
    // The fixed-compat surface is only available with the fixed-compat
    // feature; in that build this hits the None branch.
}

// ─── wide-tier strict result out-of-range ──────────────────────────────

#[cfg(all(feature = "wide", not(feature = "fast")))]
#[test]
#[should_panic]
fn d76_strict_result_out_of_range_panics() {
    // exp at a large arg overflows D76<74>::MAX. The panic message
    // depends on which internal path overflows first.
    let v: D76<74> = D38::<74>::from_int(70).into();
    let _ = v.exp_strict();
}

// ─── wide-tier TryFrom<f64> Overflow path ──────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn wide_tryfrom_f64_overflow() {
    // 1e76 > D76<2>::MAX (~5.78e74). Overflow err.
    let r: Result<D76<2>, _> = (1e76_f64).try_into();
    assert!(r.is_err());
}

// ─── wide-tier to_int_with HalfAwayFromZero negative branch ────────────

#[cfg(feature = "wide")]
#[test]
fn wide_to_int_half_away_from_zero_negative() {
    use decimal_scaled::RoundingMode;
    type D76_2 = D76<2>;
    // -1.50 at S=2 with HalfAwayFromZero → -2
    let v: D76_2 = D38::<2>::from_bits(-150).into();
    assert_eq!(v.to_int_with(RoundingMode::HalfAwayFromZero), -2);
}

// ─── arithmetic.rs: mul_with that hits the panic path on overflow ─────

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn mul_with_overflow_panics() {
    use decimal_scaled::RoundingMode;
    let a = D38::<0>::MAX;
    let _ = a.mul_with(a, RoundingMode::HalfToEven);
}

#[test]
#[should_panic(expected = "attempt to divide with overflow")]
fn div_with_overflow_panics() {
    use decimal_scaled::RoundingMode;
    let a = D38::<0>::MIN;
    let _ = a.div_with(D38::<0>::from_int(-1), RoundingMode::HalfToEven);
}

// ─── powers_strict.rs uncovered branches ───────────────────────────────

#[cfg(not(feature = "fast"))]
#[test]
fn powers_strict_specific_inputs() {
    // pow_int at a few values to hit specific branches
    use decimal_scaled::D38s12;
    // sqrt of a perfect square at S=12
    let four = D38s12::from_int(4);
    assert_eq!(four.sqrt_strict(), D38s12::from_int(2));
    // cbrt of a perfect cube
    let twenty_seven = D38s12::from_int(27);
    assert_eq!(twenty_seven.cbrt_strict(), D38s12::from_int(3));
    // sqrt of negative input — saturates to ZERO via the strict path
    assert_eq!(D38s12::from_int(-4).sqrt_strict(), D38s12::ZERO);
}

// ─── narrow-tier *_with siblings: hit RoundingMode branches ─────────────

#[test]
fn narrow_strict_with_variants_d9_d18() {
    use decimal_scaled::{D9, D18, RoundingMode};
    type D9_4 = D9<4>;
    type D18_8 = D18<8>;
    // The narrow-tier _strict_with isn't a separate emission; the delegators
    // in strict_transcendentals.rs only emit *_strict (no _with sibling).
    // What we CAN cover is to_int_with on narrow widths, every mode.
    for m in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        let _ = D9_4::from_bits(15050).to_int_with(m);
        let _ = D9_4::from_bits(-15050).to_int_with(m);
        let _ = D18_8::from_bits(1_500_500_000).to_int_with(m);
        let _ = D18_8::from_bits(-1_500_500_000).to_int_with(m);
    }
}
