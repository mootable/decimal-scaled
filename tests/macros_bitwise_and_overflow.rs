//! Coverage suite for `macros/bitwise.rs` and the remaining branches in
//! `macros/overflow.rs` that the high-level surface tests didn't reach.

use decimal_scaled::{D9, D18, D38};

type D9_2 = D9<2>;
type D18_2 = D18<2>;
type D38_2 = D38<2>;

// ─── bitwise: bit-manipulation methods on raw storage ──────────────────

#[test]
fn bitwise_methods_d9() {
    let v = D9_2::from_bits(0b1010);
    assert_eq!(v.count_ones(), 2);
    assert_eq!(v.count_zeros(), 32 - 2);
    assert_eq!(v.trailing_zeros(), 1);
    assert_eq!(v.leading_zeros(), 32 - 4);
    // unsigned_shr fills high bits with zero even for negative storage.
    let neg = D9_2::from_bits(-1); // all-ones
    let logical = neg.unsigned_shr(31);
    assert_eq!(logical.to_bits(), 1);
    // rotate_left/right
    let r = D9_2::from_bits(1).rotate_left(2);
    assert_eq!(r.to_bits(), 4);
    let r = D9_2::from_bits(4).rotate_right(2);
    assert_eq!(r.to_bits(), 1);
    // is_power_of_two / next_power_of_two
    assert!(D9_2::from_bits(8).is_power_of_two());
    assert!(!D9_2::from_bits(7).is_power_of_two());
    assert_eq!(D9_2::from_bits(5).next_power_of_two().to_bits(), 8);
}

#[test]
fn bitwise_methods_d18() {
    let v = D18_2::from_bits(0b1010);
    assert_eq!(v.count_ones(), 2);
    assert_eq!(v.count_zeros(), 64 - 2);
    assert_eq!(v.trailing_zeros(), 1);
    assert_eq!(v.leading_zeros(), 64 - 4);
    let neg = D18_2::from_bits(-1);
    let logical = neg.unsigned_shr(63);
    assert_eq!(logical.to_bits(), 1);
    let r = D18_2::from_bits(1).rotate_left(2);
    assert_eq!(r.to_bits(), 4);
    let r = D18_2::from_bits(4).rotate_right(2);
    assert_eq!(r.to_bits(), 1);
    assert!(D18_2::from_bits(8).is_power_of_two());
    assert!(!D18_2::from_bits(7).is_power_of_two());
    assert_eq!(D18_2::from_bits(5).next_power_of_two().to_bits(), 8);
}

#[test]
fn bitwise_methods_d38() {
    let v = D38_2::from_bits(0b1010);
    assert_eq!(v.count_ones(), 2);
    assert_eq!(v.count_zeros(), 128 - 2);
    assert_eq!(v.trailing_zeros(), 1);
    assert_eq!(v.leading_zeros(), 128 - 4);
    let neg = D38_2::from_bits(-1);
    let logical = neg.unsigned_shr(127);
    assert_eq!(logical.to_bits(), 1);
    let r = D38_2::from_bits(1).rotate_left(2);
    assert_eq!(r.to_bits(), 4);
    let r = D38_2::from_bits(4).rotate_right(2);
    assert_eq!(r.to_bits(), 1);
    assert!(D38_2::from_bits(8).is_power_of_two());
    assert!(!D38_2::from_bits(7).is_power_of_two());
    assert_eq!(D38_2::from_bits(5).next_power_of_two().to_bits(), 8);
}

#[cfg(feature = "wide")]
#[test]
fn bitwise_methods_wide() {
    use decimal_scaled::D76;
    type D76_2 = D76<2>;
    let one: D76_2 = D38_2::from_bits(1).into();
    let zero = D76_2::ZERO;
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
    let a: D76_2 = D38_2::from_bits(0b1100).into();
    let b: D76_2 = D38_2::from_bits(0b1010).into();
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
    let a = D9_2::from_int(7);
    let b = D9_2::from_int(2);
    let q = a.wrapping_div(b);
    // 7.00 / 2.00 = 3.50 → 350 at S=2
    assert_eq!(q.to_bits(), 350);
    let r = a.wrapping_rem(b);
    let _ = r;

    let a = D18_2::from_int(7);
    let b = D18_2::from_int(2);
    let _ = a.wrapping_div(b);
    let _ = a.wrapping_rem(b);

    // Wide
    #[cfg(feature = "wide")]
    {
        use decimal_scaled::D76;
        type D76_2 = D76<2>;
        let a: D76_2 = D38_2::from_int(7).into();
        let b: D76_2 = D38_2::from_int(2).into();
        let q = a.wrapping_div(b);
        let expected: D76_2 = D38_2::from_bits(350).into();
        assert_eq!(q, expected);
        let _ = a.wrapping_rem(b);
    }
}

// ─── overflow: saturating_div sign-aware ───────────────────────────────

#[cfg(feature = "wide")]
#[test]
fn wide_overflow_variants_success_cases() {
    use decimal_scaled::D76;
    type D76_2 = D76<2>;
    let a: D76_2 = D38_2::from_int(7).into();
    let b: D76_2 = D38_2::from_int(2).into();
    // saturating_mul success path
    let _ = a.saturating_mul(b);
    // saturating_div success path
    let _ = a.saturating_div(b);
    // overflowing_div success path
    let (_, ov) = a.overflowing_div(b);
    assert!(!ov);
    // saturating_div with non-overflow + div-by-zero
    let z = D76_2::ZERO;
    let r = a.saturating_div(z);
    assert!(r == D76_2::MAX || r == D76_2::MIN);
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
    let tiny: D = D38::<74>::from_bits(decimal_scaled::Int::<2>::from_i128(1)).into();
    let r = a.checked_div(tiny);
    assert!(r.is_none(), "tiny divisor on huge dividend should overflow");
}

#[test]
fn saturating_div_overflow_signs() {
    // D9<2>::MIN / -ONE wraps because MIN's negation is unrepresentable.
    // saturating_div should clamp.
    let r = D9_2::MIN.saturating_div(-D9_2::ONE);
    assert!(r == D9_2::MIN || r == D9_2::MAX);
    let r = D18_2::MIN.saturating_div(-D18_2::ONE);
    assert!(r == D18_2::MIN || r == D18_2::MAX);
}

// ─── overflow: overflowing_rem ─────────────────────────────────────────

#[test]
fn overflowing_rem_non_zero_no_overflow() {
    let a = D9_2::from_int(7);
    let b = D9_2::from_int(2);
    let (_, ov) = a.overflowing_rem(b);
    assert!(!ov);
    let a = D18_2::from_int(7);
    let b = D18_2::from_int(2);
    let (_, ov) = a.overflowing_rem(b);
    assert!(!ov);

    #[cfg(feature = "wide")]
    {
        use decimal_scaled::D76;
        type D76_2 = D76<2>;
        let a: D76_2 = D38_2::from_int(7).into();
        let b: D76_2 = D38_2::from_int(2).into();
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
        let _ = D38_2::from_f64_with(1.005, mode);
        let _ = D9_2::from_f64_with(1.005, mode);
        let _ = D18_2::from_f64_with(1.005, mode);
    }

    // Negative side
    let _ = D38_2::from_f64_with(-1.005, RoundingMode::Floor);
    let _ = D9_2::from_f64_with(-1.005, RoundingMode::Ceiling);

    // Exact zero
    assert_eq!(
        D38_2::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
        0
    );
    assert_eq!(
        D9_2::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
        0
    );
    assert_eq!(
        D18_2::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
        0
    );

    #[cfg(feature = "wide")]
    {
        use decimal_scaled::D76;
        type D76_2 = D76<2>;
        for mode in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            let _ = D76_2::from_f64_with(1.005, mode);
        }
    }
}

// ─── equalities: float NaN/inf branches via wide tier ──────────────────

#[cfg(all(feature = "wide", feature = "std"))]
#[test]
fn eq_wide_float() {
    use decimal_scaled::D76;
    type D76_2 = D76<2>;
    let v: D76_2 = D38_2::from_int(42).into();
    assert_eq!(v, 42.0_f64);
    assert_eq!(42.0_f64, v);
    assert_eq!(v, 42.0_f32);
    assert_ne!(v, f64::NAN);
    assert_ne!(v, f64::INFINITY);
    assert_ne!(v, f32::NAN);
    // Fractional
    let frac: D76_2 = D38_2::from_bits(4_201).into();
    assert_ne!(frac, 42.0_f64);
}
