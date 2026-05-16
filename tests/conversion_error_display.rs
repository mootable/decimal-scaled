//! `ConvertError` Display + Debug surface. Users see these strings when
//! a `TryFrom` returns an error, so the wording is part of the public
//! contract.

use decimal_scaled::ConvertError;

#[test]
fn overflow_display_message() {
    assert_eq!(
        format!("{}", ConvertError::Overflow),
        "decimal conversion overflow",
    );
}

#[test]
fn not_finite_display_message() {
    assert_eq!(
        format!("{}", ConvertError::NotFinite),
        "decimal conversion from non-finite float",
    );
}

#[test]
fn debug_renders() {
    // Debug derives — call it once for each variant to ensure the impl
    // exists and doesn't panic.
    let _ = format!("{:?}", ConvertError::Overflow);
    let _ = format!("{:?}", ConvertError::NotFinite);
}

#[test]
fn try_from_returns_overflow_message() {
    let r: Result<decimal_scaled::D9<2>, _> = i128::MAX.try_into();
    let e = r.unwrap_err();
    assert_eq!(format!("{e}"), "decimal conversion overflow");
}

#[cfg(feature = "std")]
#[test]
fn try_from_nan_returns_not_finite_message() {
    let r: Result<decimal_scaled::D9<2>, _> = f64::NAN.try_into();
    let e = r.unwrap_err();
    assert_eq!(format!("{e}"), "decimal conversion from non-finite float");
}
