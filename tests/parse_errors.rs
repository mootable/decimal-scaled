//! `D38::from_str` error path coverage. `from_str` rejects malformed
//! decimal literals with one of [`ParseError`]'s variants; each variant
//! corresponds to a distinct grammar rule in the parser and is exercised
//! here.

use core::str::FromStr;
use decimal_scaled::D38;

#[test]
fn sign_only_input_is_rejected() {
    assert!(
        D38::<2>::from_str("-").is_err(),
        "bare `-` should be SignOnly"
    );
    assert!(
        D38::<2>::from_str("+").is_err(),
        "bare `+` should be SignOnly"
    );
}

#[test]
fn empty_input_is_rejected() {
    assert!(D38::<2>::from_str("").is_err());
}

#[test]
fn dot_only_input_is_rejected() {
    assert!(D38::<2>::from_str(".").is_err());
}

#[test]
fn missing_fractional_after_dot_is_rejected() {
    assert!(D38::<2>::from_str("1.").is_err());
}

#[test]
fn missing_integer_before_dot_is_rejected() {
    assert!(D38::<2>::from_str(".5").is_err());
}

#[test]
fn leading_zero_in_integer_part_is_rejected() {
    assert!(
        D38::<2>::from_str("01").is_err(),
        "leading-zero integer part is reserved for the literal `0`"
    );
    assert!(D38::<2>::from_str("01.5").is_err());
}

#[test]
fn overlong_fractional_is_rejected() {
    // SCALE=2 allows at most 2 fractional digits.
    assert!(D38::<2>::from_str("1.123").is_err());
}

#[test]
fn integer_part_above_i128_max_is_rejected() {
    // 41 digits — well past i128's 39-digit MAX (~1.7e38).
    assert!(D38::<0>::from_str("99999999999999999999999999999999999999999").is_err());
}

#[test]
fn value_just_past_i128_max_is_rejected() {
    // i128::MAX + 1 = 2^127.
    let r = D38::<0>::from_str("170141183460469231731687303715884105728");
    assert!(r.is_err());
    // i128::MIN - 1.
    let r = D38::<0>::from_str("-170141183460469231731687303715884105729");
    assert!(r.is_err());
}

#[test]
fn valid_canonical_forms_round_trip() {
    // Counter-tests so we know the reject paths above aren't false alarms.
    assert!(D38::<2>::from_str("0").is_ok());
    assert!(D38::<2>::from_str("0.00").is_ok());
    assert!(D38::<2>::from_str("1.5").is_ok());
    assert!(D38::<2>::from_str("-1.5").is_ok());
    assert!(D38::<0>::from_str("170141183460469231731687303715884105727").is_ok()); // i128::MAX
    assert!(D38::<0>::from_str("-170141183460469231731687303715884105728").is_ok()); // i128::MIN
}
