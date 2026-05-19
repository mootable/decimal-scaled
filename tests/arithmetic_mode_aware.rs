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

// `mul_with` / `div_with` share the same overflow contract as the plain
// `*` / `/` operators: panic in debug, wrap in release. The mode argument
// influences only the rounding step, not the overflow policy.

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn mul_with_overflow_panics_in_debug() {
    let a = decimal_scaled::D38::<0>::MAX;
    let _ = a.mul_with(a, RoundingMode::HalfToEven);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "attempt to divide with overflow")]
fn div_with_overflow_panics_in_debug() {
    use decimal_scaled::D38;
    let a = D38::<0>::MIN;
    let _ = a.div_with(D38::<0>::from_int(-1), RoundingMode::HalfToEven);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn mul_overflow_panics_in_debug() {
    // Choose operands such that the mg_divide path returns None and the
    // panic_or_wrap_mul branch fires. D38<0>::MAX * 2 overflows.
    use decimal_scaled::D38;
    let a = D38::<0>::MAX;
    let _ = a * a;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "attempt to divide with overflow")]
fn div_overflow_panics_in_debug() {
    // D38<0>::MIN / -1 wraps in i128 / -1 -> overflows.
    use decimal_scaled::D38;
    let a = D38::<0>::MIN;
    let _ = a / D38::<0>::from_int(-1);
}

// ─── D18 mul / div via the u128/u64 fast path ──────────────────────────
//
// D18 at SCALE >= 10 routes through the new `i128_divrem_by_u64_with_mode`
// helper (the `__divti3` soft-call replacement). These tests pin the
// bit-exact behaviour across every RoundingMode and across positive /
// negative signs / tie / non-tie cases so the schoolbook divide stays
// identical to the prior `i128 / i128` path.

#[test]
fn d18_mul_with_modes_exact_at_s18() {
    use decimal_scaled::D18;
    // 1.5 * 2.0 = 3.0 (exact under every mode).
    let a = D18::<18>::from_bits(1_500_000_000_000_000_000);
    let b = D18::<18>::from_int(2);
    let expected = 3_000_000_000_000_000_000_i64;
    for m in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        let r = a.mul_with(b, m);
        assert_eq!(r.to_bits(), expected, "mode {m:?}");
    }
}

#[test]
fn d18_div_with_modes_one_third_at_s18() {
    use decimal_scaled::D18;
    // 1.0 / 3.0 at scale 18 — never exact. Check all six modes agree on
    // the truncated quotient and disagree by at most 1 LSB.
    let a = D18::<18>::from_int(1);
    let b = D18::<18>::from_int(3);
    let bits = [
        a.div_with(b, RoundingMode::HalfToEven).to_bits(),
        a.div_with(b, RoundingMode::HalfAwayFromZero).to_bits(),
        a.div_with(b, RoundingMode::HalfTowardZero).to_bits(),
        a.div_with(b, RoundingMode::Trunc).to_bits(),
        a.div_with(b, RoundingMode::Floor).to_bits(),
        a.div_with(b, RoundingMode::Ceiling).to_bits(),
    ];
    let min = *bits.iter().min().unwrap();
    let max = *bits.iter().max().unwrap();
    assert!(max - min <= 1, "modes diverged by > 1 LSB: {bits:?}");
    // Trunc = floor for positive — both equal 333…3 (18 digits).
    assert_eq!(bits[3], 333_333_333_333_333_333);
    assert_eq!(bits[4], 333_333_333_333_333_333);
    // Ceiling = trunc + 1.
    assert_eq!(bits[5], 333_333_333_333_333_334);
}

#[test]
fn d18_mul_negative_signs_at_s18() {
    use decimal_scaled::D18;
    let a = D18::<18>::from_bits(1_500_000_000_000_000_000);
    let b_pos = D18::<18>::from_int(2);
    let b_neg = -b_pos;
    // (+1.5) * (-2.0) = -3.0
    let r1 = a.mul_with(b_neg, RoundingMode::HalfToEven);
    assert_eq!(r1.to_bits(), -3_000_000_000_000_000_000);
    // (-1.5) * (+2.0) = -3.0
    let r2 = (-a).mul_with(b_pos, RoundingMode::HalfToEven);
    assert_eq!(r2.to_bits(), -3_000_000_000_000_000_000);
    // (-1.5) * (-2.0) = +3.0
    let r3 = (-a).mul_with(b_neg, RoundingMode::HalfToEven);
    assert_eq!(r3.to_bits(), 3_000_000_000_000_000_000);
}

#[test]
fn d18_div_negative_signs_at_s18() {
    use decimal_scaled::D18;
    let one = D18::<18>::from_int(1);
    let three_pos = D18::<18>::from_int(3);
    let three_neg = -three_pos;
    // (+1)/(-3) — both modes should equal sign-flipped (+1)/(+3) result
    // under HalfToEven.
    let pos_pos = one.div_with(three_pos, RoundingMode::HalfToEven);
    let pos_neg = one.div_with(three_neg, RoundingMode::HalfToEven);
    let neg_pos = (-one).div_with(three_pos, RoundingMode::HalfToEven);
    let neg_neg = (-one).div_with(three_neg, RoundingMode::HalfToEven);
    assert_eq!(pos_neg.to_bits(), -pos_pos.to_bits());
    assert_eq!(neg_pos.to_bits(), -pos_pos.to_bits());
    assert_eq!(neg_neg.to_bits(), pos_pos.to_bits());
}

#[test]
fn d18_mul_half_to_even_tie_at_s18() {
    use decimal_scaled::D18;
    // Construct a product whose discard exactly hits the tie boundary.
    // At SCALE = 18 the divisor is 10^18; we need (a * b) % 10^18 == 5e17
    // with the truncated quotient even vs odd to verify the half-to-even
    // bump fires only when q is odd.
    //
    // (q + 0.5e0) * 10^18  ↦ a * b = q * 10^18 + 5e17.
    //
    // Pick q = 2 (even, half-to-even tie ↦ stays at 2) — a = 1, b such
    // that a*b = 2.5e18. With scale-18 storage `a.0 = 1e18` (logical 1),
    // we need b.0 = 2.5e18 which exceeds i64; rescale: a.0 = 5e17 (0.5),
    // b.0 = 5e18 — also too big. Use a.0 = 1e9, b.0 = 2.5e9 ↦ product
    // 2.5e18, divided by 10^18 = 2.5; truncated 2, tie ↦ even ↦ 2.
    let a = D18::<18>::from_bits(1_000_000_000);
    let b = D18::<18>::from_bits(2_500_000_000);
    let r = a.mul_with(b, RoundingMode::HalfToEven);
    // 2.5 ↦ even ↦ 2 (storage 2e18 would overflow i64; the product is
    // 2.5e18 < i64::MAX = 9.22e18, so the result fits.)
    assert_eq!(r.to_bits(), 2);

    // q = 3 (odd, half-to-even tie ↦ bumps to 4).
    let a3 = D18::<18>::from_bits(1_000_000_000);
    let b3 = D18::<18>::from_bits(3_500_000_000);
    let r3 = a3.mul_with(b3, RoundingMode::HalfToEven);
    assert_eq!(r3.to_bits(), 4);
}

#[test]
fn d18_mul_scale_0_short_circuit() {
    use decimal_scaled::D18;
    // SCALE = 0: the `if SCALE == 0` arm bypasses the divrem helper.
    let a = D18::<0>::from_int(12_345);
    let b = D18::<0>::from_int(67_890);
    let r = a.mul_with(b, RoundingMode::HalfToEven);
    assert_eq!(r.to_bits(), 12_345_i64 * 67_890);
}

#[test]
fn d18_div_with_at_s10_fast_path() {
    use decimal_scaled::D18;
    // SCALE = 10 — divisor 10^10 still > 2^32; the new path applies.
    let a = D18::<10>::from_int(7);
    let b = D18::<10>::from_int(2);
    let r = a.div_with(b, RoundingMode::HalfToEven);
    // 7/2 = 3.5 ↦ storage 3.5 * 10^10 = 35_000_000_000.
    assert_eq!(r.to_bits(), 35_000_000_000);
}
