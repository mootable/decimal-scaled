//! Coverage suite for `macros/bitwise.rs` and the remaining branches in
//! `macros/overflow.rs` that the high-level surface tests didn't reach.

use decimal_scaled::{D18, D38};

// ─── bitwise: bit-manipulation methods on raw storage ──────────────────

#[test]
fn bitwise_methods_d18() {
    let v = D18::<2>::from_bits(decimal_scaled::Int::<1>::from((0b1010) as i64));
    assert_eq!(v.count_ones(), 2);
    assert_eq!(v.count_zeros(), 64 - 2);
    assert_eq!(v.trailing_zeros(), 1);
    assert_eq!(v.leading_zeros(), 64 - 4);
    let neg = D18::<2>::from_bits(decimal_scaled::Int::<1>::from((-1) as i64));
    let logical = neg.unsigned_shr(63);
    assert_eq!(logical.to_bits(), 1);
    let r = D18::<2>::from_bits(decimal_scaled::Int::<1>::from((1) as i64)).rotate_left(2);
    assert_eq!(r.to_bits(), 4);
    let r = D18::<2>::from_bits(decimal_scaled::Int::<1>::from((4) as i64)).rotate_right(2);
    assert_eq!(r.to_bits(), 1);
    assert!(D18::<2>::from_bits(decimal_scaled::Int::<1>::from((8) as i64)).is_power_of_two());
    assert!(!D18::<2>::from_bits(decimal_scaled::Int::<1>::from((7) as i64)).is_power_of_two());
    assert_eq!(D18::<2>::from_bits(decimal_scaled::Int::<1>::from((5) as i64)).next_power_of_two().to_bits(), 8);
}

#[test]
fn bitwise_methods_d38() {
    let v = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((0b1010) as i128).unwrap());
    assert_eq!(v.count_ones(), 2);
    assert_eq!(v.count_zeros(), 128 - 2);
    assert_eq!(v.trailing_zeros(), 1);
    assert_eq!(v.leading_zeros(), 128 - 4);
    let neg = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((-1) as i128).unwrap());
    let logical = neg.unsigned_shr(127);
    assert_eq!(logical.to_bits(), 1);
    let r = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()).rotate_left(2);
    assert_eq!(r.to_bits(), 4);
    let r = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((4) as i128).unwrap()).rotate_right(2);
    assert_eq!(r.to_bits(), 1);
    assert!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((8) as i128).unwrap()).is_power_of_two());
    assert!(!D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((7) as i128).unwrap()).is_power_of_two());
    assert_eq!(D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((5) as i128).unwrap()).next_power_of_two().to_bits(), 8);
}

#[cfg(feature = "wide")]
#[test]
fn bitwise_methods_wide() {
    use decimal_scaled::D76;

    let one: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()).into();
    let zero = D76::<2>::ZERO;
    assert_eq!(zero.count_ones(), 0);
    assert!(one.count_ones() >= 1);
    let _ = zero.count_zeros();
    let _ = zero.trailing_zeros();
    let _ = zero.leading_zeros();
    let _ = one.unsigned_shr(1);
    let _ = one.rotate_left(2);
    let _ = one.rotate_right(2);
    // is_power_of_two on the wide tier
    let _ = one.is_power_of_two();
    let _ = one.next_power_of_two();

    // Bitwise operators on wide
    let a: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((0b1100) as i128).unwrap()).into();
    let b: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((0b1010) as i128).unwrap()).into();
    let _ = a & b;
    let _ = a | b;
    let _ = a ^ b;
    let _ = !a;
    let mut c = a;
    c &= b;
    let mut c = a;
    c |= b;
    let mut c = a;
    c ^= b;
    let _ = c;
    // shifts
    let _ = a << 1u32;
    let _ = a >> 1u32;
    let mut c = a;
    c <<= 1u32;
    let mut c = a;
    c >>= 1u32;
    let _ = c;
}

// ─── overflow: wrapping_div / wrapping_rem (non-zero rhs path) ──────────
//
// `wrapping_div(0)` panics, but `wrapping_div(non_zero)` is a normal
// path the high-level tests did not hit at the macro-emitted body. We
// hit it here with rhs != 0.

#[test]
fn wrapping_div_rem_non_zero() {
    // narrow widths
    let a = D18::<2>::try_from(7).unwrap();
    let b = D18::<2>::try_from(2).unwrap();
    let q = a.wrapping_div(b);
    // 7.00 / 2.00 = 3.50 → 350 at S=2
    assert_eq!(q.to_bits(), 350);
    let r = a.wrapping_rem(b);
    let _ = r;

    let a = D18::<2>::try_from(7).unwrap();
    let b = D18::<2>::try_from(2).unwrap();
    let _ = a.wrapping_div(b);
    let _ = a.wrapping_rem(b);

    // Wide
    #[cfg(feature = "wide")]
    {
        use decimal_scaled::D76;

        let a: D76<2> = D38::<2>::try_from(7).unwrap().into();
        let b: D76<2> = D38::<2>::try_from(2).unwrap().into();
        let q = a.wrapping_div(b);
        let expected: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((350) as i128).unwrap()).into();
        assert_eq!(q, expected);
        let _ = a.wrapping_rem(b);
    }
}

// ─── overflow: saturating_div sign-aware ───────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn wide_overflow_variants_success_cases() {
    use decimal_scaled::D76;

    let a: D76<2> = D38::<2>::try_from(7).unwrap().into();
    let b: D76<2> = D38::<2>::try_from(2).unwrap().into();
    // saturating_mul success path
    let _ = a.saturating_mul(b);
    // saturating_div success path
    let _ = a.saturating_div(b);
    // overflowing_div success path
    let (_, ov) = a.overflowing_div(b);
    assert!(!ov);
    // saturating_div by zero panics (matches the primitive contract);
    // that path is covered by `saturating_div_by_zero_panics`, so this
    // success-cases test does not exercise it.
    // overflowing_div with non-zero rhs (success)
    let (q, ov) = a.overflowing_div(b);
    assert!(!ov);
    assert_eq!(q, a / b);
}

#[cfg(feature = "wide")]
#[test]
fn wide_checked_div_quotient_overflow() {
    // Engineer rhs so that the quotient exceeds wide storage. With
    // D76<74>, multiplier=10^74; q = (a*10^74)/b. If b is tiny and a is
    // near MAX, q can overflow.
    use decimal_scaled::D76;
    type D = D76<74>;
    let a = D::MAX;
    // 1.0 at S=74 is 10^74 — make rhs = 1 LSB so q = a * 10^74 / 1 → way past storage.
    // Build a 1-LSB tiny value by lifting D38<74> from_bits(1).
    let tiny: D = D38::<74>::from_bits(decimal_scaled::Int::<2>::try_from((1) as i128).unwrap()).into();
    let r = a.checked_div(tiny);
    assert!(r.is_none(), "tiny divisor on huge dividend should overflow");
}

#[test]
fn saturating_div_overflow_signs() {
    // D18<2>::MIN / -ONE wraps because MIN's negation is unrepresentable.
    // saturating_div should clamp.
    let r = D18::<2>::MIN.saturating_div(-D18::<2>::ONE);
    assert!(r == D18::<2>::MIN || r == D18::<2>::MAX);
    let r = D18::<2>::MIN.saturating_div(-D18::<2>::ONE);
    assert!(r == D18::<2>::MIN || r == D18::<2>::MAX);
}

// ─── overflow: overflowing_rem ─────────────────────────────────────────

#[test]
fn overflowing_rem_non_zero_no_overflow() {
    let a = D18::<2>::try_from(7).unwrap();
    let b = D18::<2>::try_from(2).unwrap();
    let (_, ov) = a.overflowing_rem(b);
    assert!(!ov);
    let a = D18::<2>::try_from(7).unwrap();
    let b = D18::<2>::try_from(2).unwrap();
    let (_, ov) = a.overflowing_rem(b);
    assert!(!ov);

    #[cfg(feature = "wide")]
    {
        use decimal_scaled::D76;

        let a: D76<2> = D38::<2>::try_from(7).unwrap().into();
        let b: D76<2> = D38::<2>::try_from(2).unwrap().into();
        let (_, ov) = a.overflowing_rem(b);
        assert!(!ov);
    }
}

// ─── float_bridge: from_f64 boundary cases and rounding modes ──────────

#[cfg(feature = "std")]
#[test]
fn float_bridge_rounding_modes() {
    use decimal_scaled::RoundingMode;

    // Each mode at the half-LSB boundary.
    // At SCALE=2, 1.005 sits between 100 and 101 (half-LSB).
    for mode in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        // exercise the dispatch branch in from_f64_with for each mode.
        let _ = D38::<2>::from_f64_with(1.005, mode);
        let _ = D18::<2>::from_f64_with(1.005, mode);
        let _ = D18::<2>::from_f64_with(1.005, mode);
    }

    // Negative side
    let _ = D38::<2>::from_f64_with(-1.005, RoundingMode::Floor);
    let _ = D18::<2>::from_f64_with(-1.005, RoundingMode::Ceiling);

    // Exact zero
    assert_eq!(
        D38::<2>::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
        0
    );
    assert_eq!(
        D18::<2>::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
        0
    );
    assert_eq!(
        D18::<2>::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
        0
    );

    #[cfg(feature = "wide")]
    {
        use decimal_scaled::D76;

        for mode in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            let _ = D76::<2>::from_f64_with(1.005, mode);
        }
    }
}

// ─── equalities: float NaN/inf branches via wide tier ──────────────────

#[cfg(all(feature = "wide", feature = "std"))]
#[test]
fn eq_wide_float() {
    use decimal_scaled::D76;

    let v: D76<2> = D38::<2>::try_from(42).unwrap().into();
    assert_eq!(v, 42.0_f64);
    assert_eq!(42.0_f64, v);
    assert_eq!(v, 42.0_f32);
    assert_ne!(v, f64::NAN);
    assert_ne!(v, f64::INFINITY);
    assert_ne!(v, f32::NAN);
    // Fractional
    let frac: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from((4_201) as i128).unwrap()).into();
    assert_ne!(frac, 42.0_f64);
}
