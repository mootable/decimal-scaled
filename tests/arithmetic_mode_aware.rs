//! Coverage for the mode-aware D38 mul_with / div_with siblings and the
//! `*Assign` operators, plus the overflow fallback paths in
//! `panic_or_wrap_*` (release-build wrapping only — debug panics are
//! validated by `should_panic` tests).

use decimal_scaled::{D38s12, RoundingMode};

#[test]
fn mul_with_modes() {
    // 1.5 * 2.0 = 3.0 (exact at any mode)
    let a = D38s12::from_bits(1_500_000_000_000);
    let b = D38s12::from_int(2);
    for m in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        let r = a.mul_with(b, m);
        assert_eq!(r.to_bits(), 3_000_000_000_000, "mode {m:?}");
    }
}

#[test]
fn div_with_modes() {
    let a = D38s12::from_int(1);
    let b = D38s12::from_int(3);
    // 1/3 = 0.333… — different modes yield slightly different LSBs.
    let r_even = a.div_with(b, RoundingMode::HalfToEven);
    let r_away = a.div_with(b, RoundingMode::HalfAwayFromZero);
    let r_trunc = a.div_with(b, RoundingMode::Trunc);
    let r_floor = a.div_with(b, RoundingMode::Floor);
    let r_ceil = a.div_with(b, RoundingMode::Ceiling);
    // Same magnitude (off by ≤ 1 LSB).
    let bits = [
        r_even.to_bits(),
        r_away.to_bits(),
        r_trunc.to_bits(),
        r_floor.to_bits(),
        r_ceil.to_bits(),
    ];
    let min = *bits.iter().min().unwrap();
    let max = *bits.iter().max().unwrap();
    assert!(max - min <= 1, "modes diverged by > 1 LSB: {bits:?}");
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn div_with_zero_panics() {
    let _ = D38s12::ONE.div_with(D38s12::ZERO, RoundingMode::HalfToEven);
}

// ─── MulAssign / DivAssign ─────────────────────────────────────────────

#[test]
fn mul_assign_div_assign() {
    let mut v = D38s12::from_bits(1_500_000_000_000); // 1.5
    v *= D38s12::from_int(2);
    assert_eq!(v.to_bits(), 3_000_000_000_000);
    v /= D38s12::from_int(3);
    assert_eq!(v.to_bits(), 1_000_000_000_000);
}

// ─── Overflow panic paths (debug builds: panic; release: wrap) ─────────

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn mul_overflow_panics_in_debug() {
    // Choose operands such that the mg_divide path returns None and the
    // panic_or_wrap_mul branch fires. D38<0>::MAX * 2 overflows.
    use decimal_scaled::D38;
    let a = D38::<0>::MAX;
    let _ = a * a;
}

#[test]
#[should_panic(expected = "attempt to divide with overflow")]
fn div_overflow_panics_in_debug() {
    // D38<0>::MIN / -1 wraps in i128 / -1 -> overflows.
    use decimal_scaled::D38;
    let a = D38::<0>::MIN;
    let _ = a / D38::<0>::from_int(-1);
}
