//! Regression test for the routing defect fix: both `*_strict` and
//! `*_fast` named methods must be accessible regardless of which feature
//! mode is selected. The plain `*` form remains the feature-driven
//! dispatcher.
//!
//! These tests only need to *compile* — runtime behavior of each variant
//! is covered in the precision suites. The asserts here are weak
//! tautologies so the methods are actually used (otherwise the optimizer
//! might elide them).

use decimal_scaled::D38s12;

#[cfg(feature = "std")]
#[test]
fn d38_fast_surface_callable_in_any_mode() {
    let x = D38s12::from_int(2);
    let _ = x.ln_fast();
    let _ = x.log2_fast();
    let _ = x.log10_fast();
    let _ = x.log_fast(D38s12::from_int(10));
    let _ = x.exp_fast();
    let _ = x.exp2_fast();
    let _ = x.sqrt_fast();
    let _ = x.cbrt_fast();
    let _ = x.powf_fast(D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)));
    let _ = x.hypot_fast(D38s12::from_int(3));
    let _ = x.sin_fast();
    let _ = x.cos_fast();
    let _ = x.tan_fast();
    let _ = D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)).asin_fast();
    let _ = D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)).acos_fast();
    let _ = x.atan_fast();
    let _ = x.atan2_fast(D38s12::ONE);
    let _ = x.sinh_fast();
    let _ = x.cosh_fast();
    let _ = x.tanh_fast();
    let _ = x.asinh_fast();
    let _ = x.acosh_fast();
    let _ = D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)).atanh_fast();
    let _ = x.to_degrees_fast();
    let _ = x.to_radians_fast();
}

#[test]
fn d38_strict_surface_callable_in_any_mode() {
    let x = D38s12::from_int(2);
    let _ = x.ln_strict();
    let _ = x.log2_strict();
    let _ = x.log10_strict();
    let _ = x.log_strict(D38s12::from_int(10));
    let _ = x.exp_strict();
    let _ = x.exp2_strict();
    let _ = x.sqrt_strict();
    let _ = x.cbrt_strict();
    let _ = x.powf_strict(D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)));
    let _ = x.sin_strict();
    let _ = x.cos_strict();
    let _ = x.tan_strict();
    let _ = D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)).asin_strict();
    let _ = D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)).acos_strict();
    let _ = x.atan_strict();
    let _ = x.atan2_strict(D38s12::ONE);
    let _ = x.sinh_strict();
    let _ = x.cosh_strict();
    let _ = x.tanh_strict();
    let _ = x.asinh_strict();
    let _ = x.acosh_strict();
    let _ = D38s12::from_bits(decimal_scaled::Int::<2>::from_i128(500_000_000_000)).atanh_strict();
    let _ = x.to_degrees_strict();
    let _ = x.to_radians_strict();
}

#[cfg(feature = "wide")]
#[cfg(feature = "std")]
#[test]
fn wide_fast_surface_callable() {
    use decimal_scaled::D76;
    type W = D76<12>;
    let x: W = D38s12::from_int(2).into();
    let _ = x.ln_fast();
    let _ = x.exp_fast();
    let _ = x.sqrt_fast();
    let _ = x.sin_fast();
    let _ = x.atan2_fast(x);
}

#[cfg(feature = "wide")]
#[test]
fn wide_strict_surface_callable() {
    use decimal_scaled::D76;
    type W = D76<12>;
    let x: W = D38s12::from_int(2).into();
    let _ = x.ln_strict();
    let _ = x.exp_strict();
    let _ = x.sqrt_strict();
    let _ = x.sin_strict();
    let _ = x.atan2_strict(x);
}

#[test]
fn narrow_strict_surface_callable() {
    use decimal_scaled::{D18};
    type D18_8 = D18<8>;
    let x18 = D18_8::from_int(2);
    let _ = x18.ln_strict();
    let _ = x18.sin_strict();
    let _ = x18.sqrt_strict();
    let _ = x18.exp_strict();
}

#[cfg(feature = "std")]
#[test]
fn narrow_fast_surface_callable() {
    use decimal_scaled::{D18};
    type D18_8 = D18<8>;
    let x18 = D18_8::from_int(2);
    let _ = x18.ln_fast();
    let _ = x18.sin_fast();
    let _ = x18.sqrt_fast();
    let _ = x18.exp_fast();
}
