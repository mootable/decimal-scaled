//! Coverage of the `widen()` / `narrow()` hop methods on D38 / D76 /
//! D153, and the per-tier `Default` impls.

#![cfg(feature = "wide")]

use decimal_scaled::{D38, D76};

#[test]
fn d38_widen_to_d76() {
    let a = D38::<12>::from_int(7);
    let w: D76<12> = a.widen();
    let expected: D76<12> = a.into();
    assert_eq!(w, expected);
}

#[test]
fn d76_narrow_to_d38_in_range() {
    let w: D76<12> = D38::<12>::from_int(7).into();
    let n: D38<12> = w.narrow().unwrap();
    assert_eq!(n, D38::<12>::from_int(7));
}

#[test]
fn d76_narrow_to_d38_out_of_range_errors() {
    // D76<0>::MAX is way above D38<0>::MAX
    let w = D76::<0>::MAX;
    let r: Result<D38<0>, _> = w.narrow();
    assert!(r.is_err());
}

#[test]
fn defaults_per_tier() {
    assert_eq!(D76::<2>::default(), D76::<2>::ZERO);
    #[cfg(feature = "x-wide")]
    {
        use decimal_scaled::{D153, D307};
        assert_eq!(D153::<2>::default(), D153::<2>::ZERO);
        assert_eq!(D307::<2>::default(), D307::<2>::ZERO);
    }
}

#[cfg(feature = "x-wide")]
#[test]
fn d76_widen_to_d153_d307() {
    use decimal_scaled::{D153, D307};
    let a: D76<6> = D38::<6>::from_int(7).into();
    let b: D153<6> = a.widen();
    let _ = b;
    let n: D76<6> = b.narrow().unwrap();
    assert_eq!(n, a);

    let c: D307<6> = b.widen();
    let n: D153<6> = c.narrow().unwrap();
    assert_eq!(n, b);

    // Out-of-range narrow
    let big_153 = D153::<0>::MAX;
    let r: Result<D76<0>, _> = big_153.narrow();
    assert!(r.is_err());
    let big_307 = D307::<0>::MAX;
    let r: Result<D153<0>, _> = big_307.narrow();
    assert!(r.is_err());
}

#[cfg(feature = "x-wide")]
#[test]
fn cross_width_narrowing_d76_to_d18_d9() {
    use decimal_scaled::{D18, D9};
    let w: D76<2> = D38::<2>::from_int(7).into();
    let n18: D18<2> = w.try_into().unwrap();
    assert_eq!(n18.to_bits(), 700);
    let n9: D9<2> = w.try_into().unwrap();
    assert_eq!(n9.to_bits(), 700);

    // Out of range
    let big = D76::<2>::MAX;
    let r: Result<D18<2>, _> = big.try_into();
    assert!(r.is_err());
    let r: Result<D9<2>, _> = big.try_into();
    assert!(r.is_err());
}
