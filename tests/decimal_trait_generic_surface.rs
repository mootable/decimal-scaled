//! Validates that the [`Decimal`] trait exposes a uniform width-generic
//! surface: a single generic function works on every width without
//! reaching for inherent methods.

use decimal_scaled::{D18, D38, Decimal, RoundingMode};
use decimal_scaled::{DecimalArithmetic, DecimalConvert};

/// A width-generic helper that touches each major surface area: ops,
/// sign, overflow variants, integer methods, pow, and float bridge.
/// Returning a tuple lets the caller assert per-width.
fn surface_check<D: Decimal>(seed_i32: i32) -> (D, D, D, D) {
    let v = D::from_i32(seed_i32);
    // Operators via supertrait bounds.
    let doubled = v + v;
    let zero_via_sub = v - v;
    assert!(zero_via_sub.is_zero());
    // Sign.
    assert!(v.is_positive() ^ v.is_negative() ^ v.is_zero() == (!v.is_zero()));
    // Pow + checked.
    let squared = v.pow(2);
    assert_eq!(v.checked_pow(0), Some(D::ONE));
    // Overflow variants — saturating mul by ONE is identity.
    assert_eq!(v.saturating_mul(D::ONE), v);
    // Integer methods.
    assert_eq!(v.abs_diff(D::ZERO), v.abs());
    // Float bridge (gated to keep the function callable in any feature
    // configuration; tests assert when std is present).
    #[cfg(feature = "std")]
    {
        let f = v.to_f64();
        let back = D::from_f64(f);
        assert_eq!(back, v);
    }
    (v, doubled, squared, v.signum())
}

#[test]
fn surface_check_d18() {

    let (v, doubled, squared, _) = surface_check::<D18<4>>(5);
    assert_eq!(v.to_bits(), 50_000);
    assert_eq!(doubled.to_bits(), 100_000);
    assert_eq!(squared.to_bits(), 250_000);
}

#[test]
fn surface_check_d38() {

    let (v, doubled, squared, _) = surface_check::<D38<6>>(5);
    assert_eq!(v.to_bits(), 5_000_000);
    assert_eq!(doubled.to_bits(), 10_000_000);
    assert_eq!(squared.to_bits(), 25_000_000);
}

#[cfg(feature = "wide")]
#[test]
fn surface_check_d76() {
    use decimal_scaled::D76;

    let (v, _, squared, _) = surface_check::<D76<6>>(5);
    let expected_v: D76<6> = D38::<6>::from_int(5).into();
    let expected_squared: D76<6> = D38::<6>::from_int(25).into();
    assert_eq!(v, expected_v);
    assert_eq!(squared, expected_squared);
}

/// Width-generic `sum` and `product` over an iterator.
fn fold_sum_product<D: Decimal>() -> (D, D) {
    let vs: [D; 4] = [
        D::from_i32(1),
        D::from_i32(2),
        D::from_i32(3),
        D::from_i32(4),
    ];
    (D::sum(vs.iter().copied()), D::product(vs.iter().copied()))
}

#[test]
fn sum_product_d38() {

    let (s, p) = fold_sum_product::<D38<2>>();
    assert_eq!(s, D38::<2>::from_i32(10));
    assert_eq!(p, D38::<2>::from_i32(24));
}

/// `to_int_with` exercised via trait dispatch under each rounding mode.
#[test]
fn trait_to_int_with_modes() {
    fn cast<D: Decimal>(d: D, mode: RoundingMode) -> i64 {
        DecimalConvert::to_int_with(d, mode)
    }
    let v = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((151) as i128).unwrap());
    assert_eq!(cast(v, RoundingMode::Floor), 1);
    assert_eq!(cast(v, RoundingMode::Ceiling), 2);
    assert_eq!(cast(v, RoundingMode::Trunc), 1);
}

/// The trait supertrait bounds let a generic fn use the bitwise
/// operators on the raw storage.
#[test]
fn trait_bitwise_supertraits() {
    fn high_bits_off<D: Decimal>(v: D, mask: D) -> D {
        v & !mask
    }
    let v = D38::<0>::from_bits(decimal_scaled::Int::<2>::try_from((0b1111) as i128).unwrap());
    let mask = D38::<0>::from_bits(decimal_scaled::Int::<2>::try_from((0b1100) as i128).unwrap());
    let r = high_bits_off(v, mask);
    assert_eq!(r.to_bits(), 0b0011);
}

/// Default `is_zero` / `is_one` / `is_normal` reachable via trait
/// dispatch (verified via fully-qualified syntax).
#[test]
fn trait_default_predicates_per_width() {
    fn check<D: Decimal>() {
        assert!(<D as DecimalArithmetic>::is_zero(D::ZERO));
        assert!(<D as DecimalArithmetic>::is_one(D::ONE));
        assert!(!<D as DecimalArithmetic>::is_normal(D::ZERO));
        assert!(<D as DecimalArithmetic>::is_normal(D::ONE));
    }
    check::<D18<0>>();
    check::<D38<0>>();
    #[cfg(feature = "wide")]
    check::<decimal_scaled::D76<0>>();
}
