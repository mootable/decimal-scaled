//! Parity test: `powf_strict(D::try_from(n).unwrap())` must agree with `powi(n)`
//! to within storage ULP for every supported width.
//!
//! Integer-valued exponents (with `|n|` within the fast-path threshold)
//! short-circuit `powf_strict` to the exact `powi` square-and-multiply
//! path. The two surfaces must therefore produce bit-identical results
//! on those inputs — `powi` is exact and `powf_strict` is the
//! 0.5-ULP-correctly-rounded form, but `powi(n)` is also the exact
//! integer-only result, so they must coincide bit-exactly.
//!
//! For non-integer exponents (e.g. 0.5, 2.5), the fast path does not
//! fire; that case is exercised by the existing `powf_*` lib tests.

use decimal_scaled::{D18, D38};

/// Exercises both `from_i32` and a manual `from_bits(n * multiplier)`
/// shape, since the integer-detection helper checks
/// `exp_raw % multiplier == 0` directly on the storage. Both shapes
/// must hit the fast path.
fn d38_check<const S: u32>(base_raw: i128, n: i32) {
    let base = D38::<S>::from_bits(decimal_scaled::Int::<2>::try_from((base_raw) as i128).unwrap());
    let exp_d = D38::<S>::try_from(n).unwrap();
    let from_powf = base.powf_strict(exp_d);
    let from_powi = base.powi(n);
    assert_eq!(
        from_powf.to_bits(),
        from_powi.to_bits(),
        "D38<{S}>: powf_strict({base_raw}, {n}) != powi({n})",
    );
}

fn d18_check<const S: u32>(base_raw: i64, n: i32) {
    let base = D18::<S>::from_bits(decimal_scaled::Int::<1>::from((base_raw) as i64));
    let exp_d = D18::<S>::try_from(n).unwrap();
    let from_powf = base.powf_strict(exp_d);
    let from_powi = base.powi(n);
    assert_eq!(
        from_powf.to_bits(),
        from_powi.to_bits(),
        "D18<{S}>: powf_strict({base_raw}, {n}) != powi({n})",
    );
}

#[test]
fn d38_powf_integer_fastpath_parity_s12() {
    // 2.0, 3.0, 1.5, 0.7 at SCALE = 12.
    let bases: &[i128] = &[
        2_000_000_000_000,
        3_000_000_000_000,
        1_500_000_000_000,
        700_000_000_000,
        1_100_000_000_000,
    ];
    for &b in bases {
        for n in -5_i32..=10 {
            d38_check::<12>(b, n);
        }
    }
}

#[test]
fn d18_powf_integer_fastpath_parity_s6() {
    // 2.0, 3.0, 1.5 at SCALE = 6.
    let bases: &[i64] = &[2_000_000, 3_000_000, 1_500_000, 700_000];
    for &b in bases {
        for n in -5_i32..=10 {
            d18_check::<6>(b, n);
        }
    }
}


/// Zero exponent: must return ONE regardless of base (matches the
/// integer-exponent contract). Exercises the `n == 0` edge of the
/// fast-path bounds check.
#[test]
fn d38_powf_zero_exp_returns_one() {
    let base = D38::<12>::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap());
    let zero_exp = D38::<12>::try_from(0).unwrap();
    assert_eq!(
        base.powf_strict(zero_exp).to_bits(),
        D38::<12>::ONE.to_bits()
    );
}

/// Negative integer exponent: must agree with `powi(-n)`, which routes
/// through `ONE / pow(|n|)`. Exercises the sign branch.
#[test]
fn d38_powf_negative_integer_exp_parity() {
    let base = D38::<12>::from_bits(decimal_scaled::Int::<2>::try_from((2_000_000_000_000) as i128).unwrap()); // 2.0
    for n in [-1_i32, -2, -3, -5, -10] {
        let exp_d = D38::<12>::try_from(n).unwrap();
        assert_eq!(
            base.powf_strict(exp_d).to_bits(),
            base.powi(n).to_bits(),
            "D38<12>: powf_strict(2.0, {n}) != powi({n})",
        );
    }
}

#[cfg(feature = "wide")]
#[test]
fn d57_powf_integer_fastpath_parity() {
    use decimal_scaled::D57;
    // 2.0 at SCALE = 20 (a primary target scale for D57).
    let two = D57::<20>::try_from(2).unwrap();
    for n in -5_i32..=10 {
        let exp_d = D57::<20>::try_from(n).unwrap();
        let from_powf = two.powf_strict(exp_d);
        let from_powi = two.powi(n);
        assert_eq!(
            from_powf.to_bits(),
            from_powi.to_bits(),
            "D57<20>: powf_strict(2.0, {n}) != powi({n})",
        );
    }
}

#[cfg(feature = "wide")]
#[test]
fn d76_powf_integer_fastpath_parity() {
    use decimal_scaled::D76;
    // 2.0 at SCALE = 35.
    let two = D76::<35>::try_from(2).unwrap();
    for n in -3_i32..=8 {
        let exp_d = D76::<35>::try_from(n).unwrap();
        let from_powf = two.powf_strict(exp_d);
        let from_powi = two.powi(n);
        assert_eq!(
            from_powf.to_bits(),
            from_powi.to_bits(),
            "D76<35>: powf_strict(2.0, {n}) != powi({n})",
        );
    }
}

#[cfg(feature = "x-wide")]
#[test]
fn d307_powf_integer_fastpath_parity() {
    use decimal_scaled::D307;
    // 2.0 at SCALE = 150 (matches the bench's xx-wide cell).
    let two = D307::<150>::try_from(2).unwrap();
    for n in -3_i32..=8 {
        let exp_d = D307::<150>::try_from(n).unwrap();
        let from_powf = two.powf_strict(exp_d);
        let from_powi = two.powi(n);
        assert_eq!(
            from_powf.to_bits(),
            from_powi.to_bits(),
            "D307<150>: powf_strict(2.0, {n}) != powi({n})",
        );
    }
}
