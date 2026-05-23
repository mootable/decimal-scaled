//! Coverage for the default-implemented methods on the `Decimal`
//! trait: `is_zero`, `is_one`, and `sum`. These are reachable only
//! through trait dispatch (the type's inherent `is_zero` etc. shadow
//! them), so tests must call them with explicit fully-qualified syntax.

use decimal_scaled::D38s12;
use decimal_scaled::DecimalArithmetic;

#[test]
fn decimal_trait_is_zero_default_impl() {
    assert!(<D38s12 as DecimalArithmetic>::is_zero(D38s12::ZERO));
    assert!(!<D38s12 as DecimalArithmetic>::is_zero(D38s12::ONE));
}

#[test]
fn decimal_trait_is_one_default_impl() {
    assert!(<D38s12 as DecimalArithmetic>::is_one(D38s12::ONE));
    assert!(!<D38s12 as DecimalArithmetic>::is_one(D38s12::ZERO));
}

#[test]
fn decimal_trait_sum_default_impl() {
    let vals = [
        D38s12::try_from(1).unwrap(),
        D38s12::try_from(2).unwrap(),
        D38s12::try_from(3).unwrap(),
    ];
    let s: D38s12 = <D38s12 as DecimalArithmetic>::sum(vals.iter().copied());
    assert_eq!(s, D38s12::try_from(6).unwrap());
    // Empty iter → ZERO
    let s: D38s12 = <D38s12 as DecimalArithmetic>::sum(core::iter::empty());
    assert_eq!(s, D38s12::ZERO);
}
