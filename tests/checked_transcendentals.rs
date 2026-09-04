//! `checked_*` strict transcendentals: the checked form returns
//! `Some(bit-identical-to-default)` wherever the default form returns,
//! and `None` exactly where the default form panics (for the seamed
//! detection points — see `src/types/checked_transcendentals.rs` for
//! the per-method contract). Every `None` case is paired with a
//! `#[should_panic]` sibling proving the default form panics on the
//! same input.

#![cfg(feature = "strict")]

use decimal_scaled::{D18, D38, RoundingMode};

const MODES: [RoundingMode; 8] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::AwayFromZero,
    RoundingMode::ZeroFiveUp,
];

fn d38(v: i64) -> D38<10> {
    D38::<10>::try_from(v).unwrap()
}

// ── Some == bit-identical default, across modes and widths ──────────

#[test]
fn d38_checked_matches_default_across_modes() {
    let x = d38(3);
    let y = d38(2);
    for mode in MODES {
        assert_eq!(x.checked_ln_with(mode), Some(x.ln_with(mode)));
        assert_eq!(x.checked_exp_with(mode), Some(x.exp_with(mode)));
        assert_eq!(x.checked_exp2_with(mode), Some(x.exp2_with(mode)));
        assert_eq!(x.checked_log2_with(mode), Some(x.log2_with(mode)));
        assert_eq!(x.checked_log10_with(mode), Some(x.log10_with(mode)));
        assert_eq!(
            x.checked_log_with(y, mode),
            Some(x.log_with(y, mode))
        );
        assert_eq!(x.checked_sqrt_with(mode), Some(x.sqrt_with(mode)));
        assert_eq!(x.checked_cbrt_with(mode), Some(x.cbrt_with(mode)));
        assert_eq!(
            x.checked_powf_with(y, mode),
            Some(x.powf_with(y, mode))
        );
        assert_eq!(
            x.checked_hypot_with(y, mode),
            Some(x.hypot_with(y, mode))
        );
        assert_eq!(x.checked_sin_with(mode), Some(x.sin_with(mode)));
        assert_eq!(x.checked_cos_with(mode), Some(x.cos_with(mode)));
        assert_eq!(x.checked_tan_with(mode), Some(x.tan_with(mode)));
        assert_eq!(x.checked_atan_with(mode), Some(x.atan_with(mode)));
        assert_eq!(
            x.checked_atan2_with(y, mode),
            Some(x.atan2_with(y, mode))
        );
        assert_eq!(x.checked_sinh_with(mode), Some(x.sinh_with(mode)));
        assert_eq!(x.checked_cosh_with(mode), Some(x.cosh_with(mode)));
        assert_eq!(x.checked_tanh_with(mode), Some(x.tanh_with(mode)));
        assert_eq!(x.checked_asinh_with(mode), Some(x.asinh_with(mode)));
        assert_eq!(x.checked_acosh_with(mode), Some(x.acosh_with(mode)));
        assert_eq!(
            x.checked_to_degrees_with(mode),
            Some(x.to_degrees_with(mode))
        );
        assert_eq!(
            x.checked_to_radians_with(mode),
            Some(x.to_radians_with(mode))
        );
    }
    // In-(-1, 1) arguments for the inverse trig / atanh family.
    let h = D38::<10>::ONE / d38(2);
    for mode in MODES {
        assert_eq!(h.checked_asin_with(mode), Some(h.asin_with(mode)));
        assert_eq!(h.checked_acos_with(mode), Some(h.acos_with(mode)));
        assert_eq!(h.checked_atanh_with(mode), Some(h.atanh_with(mode)));
    }
}

#[test]
fn d18_checked_matches_default_across_modes() {
    let x = D18::<6>::try_from(3_i32).unwrap();
    let y = D18::<6>::try_from(2_i32).unwrap();
    for mode in MODES {
        assert_eq!(x.checked_ln_with(mode), Some(x.ln_with(mode)));
        assert_eq!(x.checked_exp_with(mode), Some(x.exp_with(mode)));
        assert_eq!(x.checked_log10_with(mode), Some(x.log10_with(mode)));
        assert_eq!(x.checked_sqrt_with(mode), Some(x.sqrt_with(mode)));
        assert_eq!(
            x.checked_powf_with(y, mode),
            Some(x.powf_with(y, mode))
        );
        assert_eq!(x.checked_sin_with(mode), Some(x.sin_with(mode)));
        assert_eq!(x.checked_sinh_with(mode), Some(x.sinh_with(mode)));
        assert_eq!(x.checked_cosh_with(mode), Some(x.cosh_with(mode)));
        assert_eq!(
            x.checked_to_degrees_with(mode),
            Some(x.to_degrees_with(mode))
        );
        assert_eq!(
            x.checked_hypot_with(y, mode),
            Some(x.hypot_with(y, mode))
        );
    }
}

#[test]
fn default_mode_siblings_match_with_forms() {
    let x = d38(2);
    assert_eq!(
        x.checked_ln(),
        x.checked_ln_with(RoundingMode::HalfToEven)
    );
    assert_eq!(
        x.checked_exp(),
        x.checked_exp_with(RoundingMode::HalfToEven)
    );
    assert_eq!(
        x.checked_sqrt(),
        x.checked_sqrt_with(RoundingMode::HalfToEven)
    );
    assert_eq!(
        x.checked_sinh(),
        x.checked_sinh_with(RoundingMode::HalfToEven)
    );
}

// ── Domain errors: None exactly where the default panics ────────────

#[test]
fn ln_domain_none() {
    assert_eq!(D38::<10>::ZERO.checked_ln(), None);
    assert_eq!(d38(-1).checked_ln(), None);
    assert_eq!(D38::<10>::ZERO.checked_log2(), None);
    assert_eq!(D38::<10>::ZERO.checked_log10(), None);
    assert_eq!(D18::<6>::ZERO.checked_ln(), None);
    for mode in MODES {
        assert_eq!(d38(-3).checked_ln_with(mode), None);
    }
}

#[test]
#[should_panic(expected = "argument must be positive")]
fn ln_domain_default_panics() {
    let _ = D38::<10>::ZERO.ln();
}

#[test]
#[should_panic(expected = "argument must be positive")]
fn ln_negative_default_panics() {
    let _ = d38(-1).ln();
}

#[test]
fn log_domain_none() {
    let x = d38(5);
    assert_eq!(x.checked_log(D38::<10>::ONE), None);
    assert_eq!(x.checked_log(D38::<10>::ZERO), None);
    assert_eq!(x.checked_log(d38(-2)), None);
    assert_eq!(D38::<10>::ZERO.checked_log(d38(2)), None);
}

#[test]
#[should_panic(expected = "base must not equal 1")]
fn log_base_one_default_panics() {
    let _ = d38(5).log(D38::<10>::ONE);
}

#[test]
fn asin_acos_domain_none() {
    assert_eq!(d38(2).checked_asin(), None);
    assert_eq!(d38(-2).checked_asin(), None);
    assert_eq!(d38(2).checked_acos(), None);
    assert_eq!(d38(-2).checked_acos(), None);
    // The closed-interval boundary is in domain.
    assert!(D38::<10>::ONE.checked_asin().is_some());
    assert!((-D38::<10>::ONE).checked_acos().is_some());
}

#[test]
#[should_panic(expected = "argument out of domain")]
fn asin_domain_default_panics() {
    let _ = d38(2).asin();
}

#[test]
fn acosh_atanh_domain_none() {
    assert_eq!(D38::<10>::ZERO.checked_acosh(), None);
    let h = D38::<10>::ONE / d38(2);
    assert_eq!(h.checked_acosh(), None);
    // atanh's open-interval walls: both +/-1 are out of domain.
    assert_eq!(D38::<10>::ONE.checked_atanh(), None);
    assert_eq!((-D38::<10>::ONE).checked_atanh(), None);
    assert!(D38::<10>::ONE.checked_acosh().is_some());
}

#[test]
#[should_panic(expected = "argument must be >= 1")]
fn acosh_domain_default_panics() {
    let _ = D38::<10>::ZERO.acosh();
}

#[test]
#[should_panic(expected = "argument out of domain")]
fn atanh_domain_default_panics() {
    let _ = D38::<10>::ONE.atanh();
}

// ── Out-of-range results: None where the seamed default panics ──────

#[test]
fn exp_overflow_none_narrow() {
    // e^120 has 53 integer digits; D38<10> holds 28.
    assert_eq!(d38(120).checked_exp(), None);
    assert_eq!(D18::<6>::try_from(50_i32).unwrap().checked_exp(), None);
    for mode in MODES {
        assert_eq!(d38(120).checked_exp_with(mode), None);
    }
    // The negative side underflows to zero, never out of range.
    assert_eq!(d38(-120).checked_exp(), Some(d38(-120).exp()));
}

#[test]
fn exp_deep_overflow_none_narrow() {
    // The deep band is decided by the wider work integer's ANALYTIC
    // verdicts, threaded through the same seam: e^2000 by the internal
    // squaring/reassembly peak model, e^100000 and e^1e17 by the
    // argument-magnitude pre-gate — all before the kernel's range
    // reduction runs.
    assert_eq!(d38(2_000).checked_exp(), None);
    assert_eq!(d38(100_000).checked_exp(), None);
    assert_eq!(d38(100_000_000_000_000_000).checked_exp(), None);
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp_deep_overflow_default_panics() {
    let _ = d38(100_000).exp();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp_deepest_overflow_default_panics() {
    let _ = d38(100_000_000_000_000_000).exp();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp_overflow_default_panics() {
    let _ = d38(120).exp();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp_overflow_default_panics_d18() {
    let _ = D18::<6>::try_from(50_i32).unwrap().exp();
}

#[test]
fn exp2_overflow_none_narrow() {
    // 2^100 has 31 integer digits; D38<10> holds 28 — the exact-power
    // pin's ladder overflow is the proof.
    assert_eq!(d38(100).checked_exp2(), None);
    // 2^95.5 (29 digits): a fractional argument past the pin — the
    // series kernel computes it in the wider work integer and the
    // post-narrowing fit check signals the None.
    let frac = d38(95) + D38::<10>::ONE / d38(2);
    assert_eq!(frac.checked_exp2(), None);
    // Deep band: 2^7000 (exact integer — pin ladder proof) and 2^7000.5
    // (fractional — the analytic integer-digit gate, before any kernel
    // arithmetic runs).
    assert_eq!(d38(7_000).checked_exp2(), None);
    let deep_frac = d38(7_000) + D38::<10>::ONE / d38(2);
    assert_eq!(deep_frac.checked_exp2(), None);
    // An exact integer power inside the range stays exact.
    assert_eq!(d38(10).checked_exp2(), Some(d38(1024)));
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp2_overflow_default_panics() {
    let _ = d38(100).exp2();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp2_fractional_overflow_default_panics() {
    let _ = (d38(95) + D38::<10>::ONE / d38(2)).exp2();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp2_deep_overflow_default_panics() {
    let _ = d38(7_000).exp2();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp2_deep_fractional_overflow_default_panics() {
    let _ = (d38(7_000) + D38::<10>::ONE / d38(2)).exp2();
}

#[test]
fn powf_overflow_none_narrow() {
    let ten = d38(10);
    // 10^30 has 31 integer digits; D38<10> holds 28.
    assert_eq!(ten.checked_powf(d38(30)), None);
    // 3^100.5 (48 digits): a non-integer exponent past the pins — the
    // composition's analytic argument gate decides it before the exp
    // kernel runs.
    let frac_exp = d38(100) + D38::<10>::ONE / d38(2);
    assert_eq!(d38(3).checked_powf(frac_exp), None);
    // Non-positive bases saturate to zero, as the default form does.
    let half = D38::<10>::ONE / d38(2);
    assert_eq!(d38(-3).checked_powf(half), Some(D38::<10>::ZERO));
    assert_eq!(d38(-3).powf(half), D38::<10>::ZERO);
}

#[test]
#[should_panic(expected = "result out of range")]
fn powf_overflow_default_panics() {
    let _ = d38(10).powf(d38(30));
}

#[test]
#[should_panic(expected = "result out of range")]
fn powf_fractional_overflow_default_panics() {
    let _ = d38(3).powf(d38(100) + D38::<10>::ONE / d38(2));
}

#[test]
fn hypot_overflow_none() {
    let m = D38::<10>::MAX;
    assert_eq!(m.checked_hypot(m), None);
    for mode in MODES {
        assert_eq!(m.checked_hypot_with(m, mode), None);
    }
}

#[test]
#[should_panic(expected = "result out of range")]
fn hypot_overflow_default_panics() {
    let m = D38::<10>::MAX;
    let _ = m.hypot(m);
}

// ── D18 narrowing seam: fits the D38 work width, not D18 storage ────

#[test]
fn d18_narrowing_none() {
    // sinh(40) ~ 1.2e17 > D18<6> max (~9.2e12) but far inside D38<6>.
    assert_eq!(D18::<6>::try_from(40_i32).unwrap().checked_sinh(), None);
    assert_eq!(D18::<6>::try_from(40_i32).unwrap().checked_cosh(), None);
    // MAX * (180/pi) leaves D18 range, fits the D38 work width.
    assert_eq!(D18::<6>::MAX.checked_to_degrees(), None);
}

#[test]
#[should_panic(expected = "result out of range")]
fn d18_sinh_default_panics() {
    let _ = D18::<6>::try_from(40_i32).unwrap().sinh();
}

#[test]
#[should_panic(expected = "result out of range")]
fn d18_to_degrees_default_panics() {
    let _ = D18::<6>::MAX.to_degrees();
}

// ── Totality of the bounded family ──────────────────────────────────

#[test]
fn bounded_methods_always_some() {
    // sqrt saturates negatives to zero (the documented policy), so the
    // checked form is total too.
    assert_eq!(d38(-4).checked_sqrt(), Some(D38::<10>::ZERO));
    assert!(d38(-27).checked_cbrt().is_some());
    assert!(D38::<10>::ZERO.checked_atan2(D38::<10>::ZERO).is_some());
    assert!(D38::<10>::MAX.checked_tanh().is_some());
    assert!(D38::<10>::MAX.checked_asinh().is_some());
    assert!(D38::<10>::MAX.checked_to_radians().is_some());
    assert!(D38::<10>::MAX.checked_atan().is_some());
    assert!(D38::<10>::MAX.checked_sin().is_some());
}

// ── One wide tier ────────────────────────────────────────────────────

#[cfg(feature = "wide")]
mod wide {
    use super::MODES;
    use decimal_scaled::D76;

    fn d76(v: i64) -> D76<30> {
        D76::<30>::try_from(v).unwrap()
    }

    #[test]
    fn d76_checked_matches_default_across_modes() {
        let x = d76(3);
        let y = d76(2);
        for mode in MODES {
            assert_eq!(x.checked_ln_with(mode), Some(x.ln_with(mode)));
            assert_eq!(x.checked_exp_with(mode), Some(x.exp_with(mode)));
            assert_eq!(x.checked_exp2_with(mode), Some(x.exp2_with(mode)));
            assert_eq!(x.checked_log10_with(mode), Some(x.log10_with(mode)));
            assert_eq!(
                x.checked_log_with(y, mode),
                Some(x.log_with(y, mode))
            );
            assert_eq!(x.checked_sqrt_with(mode), Some(x.sqrt_with(mode)));
            assert_eq!(
                x.checked_powf_with(y, mode),
                Some(x.powf_with(y, mode))
            );
            assert_eq!(
                x.checked_hypot_with(y, mode),
                Some(x.hypot_with(y, mode))
            );
            assert_eq!(x.checked_sin_with(mode), Some(x.sin_with(mode)));
            assert_eq!(x.checked_tan_with(mode), Some(x.tan_with(mode)));
            assert_eq!(x.checked_sinh_with(mode), Some(x.sinh_with(mode)));
            assert_eq!(x.checked_cosh_with(mode), Some(x.cosh_with(mode)));
            assert_eq!(x.checked_tanh_with(mode), Some(x.tanh_with(mode)));
            assert_eq!(x.checked_asinh_with(mode), Some(x.asinh_with(mode)));
            assert_eq!(x.checked_acosh_with(mode), Some(x.acosh_with(mode)));
            assert_eq!(
                x.checked_to_degrees_with(mode),
                Some(x.to_degrees_with(mode))
            );
            assert_eq!(
                x.checked_to_radians_with(mode),
                Some(x.to_radians_with(mode))
            );
            assert_eq!(
                x.checked_atan2_with(y, mode),
                Some(x.atan2_with(y, mode))
            );
        }
        let h = D76::<30>::ONE / d76(2);
        for mode in MODES {
            assert_eq!(h.checked_asin_with(mode), Some(h.asin_with(mode)));
            assert_eq!(h.checked_acos_with(mode), Some(h.acos_with(mode)));
            assert_eq!(h.checked_atanh_with(mode), Some(h.atanh_with(mode)));
        }
    }

    #[test]
    fn d76_domain_none() {
        assert_eq!(D76::<30>::ZERO.checked_ln(), None);
        assert_eq!(d76(-1).checked_ln(), None);
        assert_eq!(d76(2).checked_asin(), None);
        assert_eq!(D76::<30>::ZERO.checked_acosh(), None);
        assert_eq!(D76::<30>::ONE.checked_atanh(), None);
        assert_eq!(d76(5).checked_log(D76::<30>::ONE), None);
    }

    /// Documents the INTERIM wide-tier contract: out-of-range results
    /// still panic in the checked form (the wide kernel seam is the
    /// deferred piece — `research/checked_wide_shell_patch.md`). This
    /// pin flips to a `None` assertion when the seam lands.
    #[test]
    #[should_panic(expected = "result out of range")]
    fn d76_exp_overflow_still_panics_pending_seam() {
        let _ = d76(200).checked_exp();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn d76_exp_overflow_default_panics() {
        let _ = d76(200).exp();
    }
}
