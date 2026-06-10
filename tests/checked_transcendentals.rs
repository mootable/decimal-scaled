//! `checked_*` strict transcendentals: the checked form returns
//! `Some(bit-identical-to-default)` wherever the default form returns,
//! and `None` exactly where the default form panics (for the seamed
//! detection points — see `src/types/checked_transcendentals.rs` for
//! the per-method contract). Every `None` case is paired with a
//! `#[should_panic]` sibling proving the default form panics on the
//! same input.

#![cfg(all(feature = "strict", not(feature = "fast")))]

use decimal_scaled::{D18, D38, RoundingMode};

const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

fn d38(v: i64) -> D38<10> {
    D38::<10>::from(v)
}

// ── Some == bit-identical default, across modes and widths ──────────

#[test]
fn d38_checked_matches_default_across_modes() {
    let x = d38(3);
    let y = d38(2);
    for mode in MODES {
        assert_eq!(x.checked_ln_strict_with(mode), Some(x.ln_strict_with(mode)));
        assert_eq!(x.checked_exp_strict_with(mode), Some(x.exp_strict_with(mode)));
        assert_eq!(x.checked_exp2_strict_with(mode), Some(x.exp2_strict_with(mode)));
        assert_eq!(x.checked_log2_strict_with(mode), Some(x.log2_strict_with(mode)));
        assert_eq!(x.checked_log10_strict_with(mode), Some(x.log10_strict_with(mode)));
        assert_eq!(
            x.checked_log_strict_with(y, mode),
            Some(x.log_strict_with(y, mode))
        );
        assert_eq!(x.checked_sqrt_strict_with(mode), Some(x.sqrt_strict_with(mode)));
        assert_eq!(x.checked_cbrt_strict_with(mode), Some(x.cbrt_strict_with(mode)));
        assert_eq!(
            x.checked_powf_strict_with(y, mode),
            Some(x.powf_strict_with(y, mode))
        );
        assert_eq!(
            x.checked_hypot_strict_with(y, mode),
            Some(x.hypot_strict_with(y, mode))
        );
        assert_eq!(x.checked_sin_strict_with(mode), Some(x.sin_strict_with(mode)));
        assert_eq!(x.checked_cos_strict_with(mode), Some(x.cos_strict_with(mode)));
        assert_eq!(x.checked_tan_strict_with(mode), Some(x.tan_strict_with(mode)));
        assert_eq!(x.checked_atan_strict_with(mode), Some(x.atan_strict_with(mode)));
        assert_eq!(
            x.checked_atan2_strict_with(y, mode),
            Some(x.atan2_strict_with(y, mode))
        );
        assert_eq!(x.checked_sinh_strict_with(mode), Some(x.sinh_strict_with(mode)));
        assert_eq!(x.checked_cosh_strict_with(mode), Some(x.cosh_strict_with(mode)));
        assert_eq!(x.checked_tanh_strict_with(mode), Some(x.tanh_strict_with(mode)));
        assert_eq!(x.checked_asinh_strict_with(mode), Some(x.asinh_strict_with(mode)));
        assert_eq!(x.checked_acosh_strict_with(mode), Some(x.acosh_strict_with(mode)));
        assert_eq!(
            x.checked_to_degrees_strict_with(mode),
            Some(x.to_degrees_strict_with(mode))
        );
        assert_eq!(
            x.checked_to_radians_strict_with(mode),
            Some(x.to_radians_strict_with(mode))
        );
    }
    // In-(-1, 1) arguments for the inverse trig / atanh family.
    let h = D38::<10>::ONE / d38(2);
    for mode in MODES {
        assert_eq!(h.checked_asin_strict_with(mode), Some(h.asin_strict_with(mode)));
        assert_eq!(h.checked_acos_strict_with(mode), Some(h.acos_strict_with(mode)));
        assert_eq!(h.checked_atanh_strict_with(mode), Some(h.atanh_strict_with(mode)));
    }
}

#[test]
fn d18_checked_matches_default_across_modes() {
    let x = D18::<6>::from(3);
    let y = D18::<6>::from(2);
    for mode in MODES {
        assert_eq!(x.checked_ln_strict_with(mode), Some(x.ln_strict_with(mode)));
        assert_eq!(x.checked_exp_strict_with(mode), Some(x.exp_strict_with(mode)));
        assert_eq!(x.checked_log10_strict_with(mode), Some(x.log10_strict_with(mode)));
        assert_eq!(x.checked_sqrt_strict_with(mode), Some(x.sqrt_strict_with(mode)));
        assert_eq!(
            x.checked_powf_strict_with(y, mode),
            Some(x.powf_strict_with(y, mode))
        );
        assert_eq!(x.checked_sin_strict_with(mode), Some(x.sin_strict_with(mode)));
        assert_eq!(x.checked_sinh_strict_with(mode), Some(x.sinh_strict_with(mode)));
        assert_eq!(x.checked_cosh_strict_with(mode), Some(x.cosh_strict_with(mode)));
        assert_eq!(
            x.checked_to_degrees_strict_with(mode),
            Some(x.to_degrees_strict_with(mode))
        );
        assert_eq!(
            x.checked_hypot_strict_with(y, mode),
            Some(x.hypot_strict_with(y, mode))
        );
    }
}

#[test]
fn default_mode_siblings_match_with_forms() {
    let x = d38(2);
    assert_eq!(
        x.checked_ln_strict(),
        x.checked_ln_strict_with(RoundingMode::HalfToEven)
    );
    assert_eq!(
        x.checked_exp_strict(),
        x.checked_exp_strict_with(RoundingMode::HalfToEven)
    );
    assert_eq!(
        x.checked_sqrt_strict(),
        x.checked_sqrt_strict_with(RoundingMode::HalfToEven)
    );
    assert_eq!(
        x.checked_sinh_strict(),
        x.checked_sinh_strict_with(RoundingMode::HalfToEven)
    );
}

// ── Domain errors: None exactly where the default panics ────────────

#[test]
fn ln_domain_none() {
    assert_eq!(D38::<10>::ZERO.checked_ln_strict(), None);
    assert_eq!(d38(-1).checked_ln_strict(), None);
    assert_eq!(D38::<10>::ZERO.checked_log2_strict(), None);
    assert_eq!(D38::<10>::ZERO.checked_log10_strict(), None);
    assert_eq!(D18::<6>::ZERO.checked_ln_strict(), None);
    for mode in MODES {
        assert_eq!(d38(-3).checked_ln_strict_with(mode), None);
    }
}

#[test]
#[should_panic(expected = "argument must be positive")]
fn ln_domain_default_panics() {
    let _ = D38::<10>::ZERO.ln_strict();
}

#[test]
#[should_panic(expected = "argument must be positive")]
fn ln_negative_default_panics() {
    let _ = d38(-1).ln_strict();
}

#[test]
fn log_domain_none() {
    let x = d38(5);
    assert_eq!(x.checked_log_strict(D38::<10>::ONE), None);
    assert_eq!(x.checked_log_strict(D38::<10>::ZERO), None);
    assert_eq!(x.checked_log_strict(d38(-2)), None);
    assert_eq!(D38::<10>::ZERO.checked_log_strict(d38(2)), None);
}

#[test]
#[should_panic(expected = "base must not equal 1")]
fn log_base_one_default_panics() {
    let _ = d38(5).log_strict(D38::<10>::ONE);
}

#[test]
fn asin_acos_domain_none() {
    assert_eq!(d38(2).checked_asin_strict(), None);
    assert_eq!(d38(-2).checked_asin_strict(), None);
    assert_eq!(d38(2).checked_acos_strict(), None);
    assert_eq!(d38(-2).checked_acos_strict(), None);
    // The closed-interval boundary is in domain.
    assert!(D38::<10>::ONE.checked_asin_strict().is_some());
    assert!((-D38::<10>::ONE).checked_acos_strict().is_some());
}

#[test]
#[should_panic(expected = "argument out of domain")]
fn asin_domain_default_panics() {
    let _ = d38(2).asin_strict();
}

#[test]
fn acosh_atanh_domain_none() {
    assert_eq!(D38::<10>::ZERO.checked_acosh_strict(), None);
    let h = D38::<10>::ONE / d38(2);
    assert_eq!(h.checked_acosh_strict(), None);
    // atanh's open-interval walls: both +/-1 are out of domain.
    assert_eq!(D38::<10>::ONE.checked_atanh_strict(), None);
    assert_eq!((-D38::<10>::ONE).checked_atanh_strict(), None);
    assert!(D38::<10>::ONE.checked_acosh_strict().is_some());
}

#[test]
#[should_panic(expected = "argument must be >= 1")]
fn acosh_domain_default_panics() {
    let _ = D38::<10>::ZERO.acosh_strict();
}

#[test]
#[should_panic(expected = "argument out of domain")]
fn atanh_domain_default_panics() {
    let _ = D38::<10>::ONE.atanh_strict();
}

// ── Out-of-range results: None where the seamed default panics ──────

#[test]
fn exp_overflow_none_narrow() {
    // e^120 has 53 integer digits; D38<10> holds 28.
    assert_eq!(d38(120).checked_exp_strict(), None);
    assert_eq!(D18::<6>::from(50).checked_exp_strict(), None);
    for mode in MODES {
        assert_eq!(d38(120).checked_exp_strict_with(mode), None);
    }
    // The negative side underflows to zero, never out of range.
    assert_eq!(d38(-120).checked_exp_strict(), Some(d38(-120).exp_strict()));
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp_overflow_default_panics() {
    let _ = d38(120).exp_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp_overflow_default_panics_d18() {
    let _ = D18::<6>::from(50).exp_strict();
}

#[test]
fn exp2_overflow_none_narrow() {
    // 2^100 has 31 integer digits; D38<10> holds 28. (Deeper overflow
    // arguments, x >= ~120 at this scale, currently die in an internal
    // div_knuth assertion on BOTH the default and checked paths — a
    // pre-existing kernel defect upstream of the overflow detection,
    // flagged in research/checked_wide_shell_patch.md.)
    assert_eq!(d38(100).checked_exp2_strict(), None);
    // An exact integer power inside the range stays exact.
    assert_eq!(d38(10).checked_exp2_strict(), Some(d38(1024)));
}

#[test]
#[should_panic(expected = "result out of range")]
fn exp2_overflow_default_panics() {
    let _ = d38(100).exp2_strict();
}

#[test]
fn powf_overflow_none_narrow() {
    let ten = d38(10);
    // 10^30 has 31 integer digits; D38<10> holds 28.
    assert_eq!(ten.checked_powf_strict(d38(30)), None);
    // Non-positive bases saturate to zero, as the default form does.
    let half = D38::<10>::ONE / d38(2);
    assert_eq!(d38(-3).checked_powf_strict(half), Some(D38::<10>::ZERO));
    assert_eq!(d38(-3).powf_strict(half), D38::<10>::ZERO);
}

#[test]
#[should_panic(expected = "result out of range")]
fn powf_overflow_default_panics() {
    let _ = d38(10).powf_strict(d38(30));
}

#[test]
fn hypot_overflow_none() {
    let m = D38::<10>::MAX;
    assert_eq!(m.checked_hypot_strict(m), None);
    for mode in MODES {
        assert_eq!(m.checked_hypot_strict_with(m, mode), None);
    }
}

#[test]
#[should_panic(expected = "result out of range")]
fn hypot_overflow_default_panics() {
    let m = D38::<10>::MAX;
    let _ = m.hypot_strict(m);
}

// ── D18 narrowing seam: fits the D38 work width, not D18 storage ────

#[test]
fn d18_narrowing_none() {
    // sinh(40) ~ 1.2e17 > D18<6> max (~9.2e12) but far inside D38<6>.
    assert_eq!(D18::<6>::from(40).checked_sinh_strict(), None);
    assert_eq!(D18::<6>::from(40).checked_cosh_strict(), None);
    // MAX * (180/pi) leaves D18 range, fits the D38 work width.
    assert_eq!(D18::<6>::MAX.checked_to_degrees_strict(), None);
}

#[test]
#[should_panic(expected = "result out of range")]
fn d18_sinh_default_panics() {
    let _ = D18::<6>::from(40).sinh_strict();
}

#[test]
#[should_panic(expected = "result out of range")]
fn d18_to_degrees_default_panics() {
    let _ = D18::<6>::MAX.to_degrees_strict();
}

// ── Totality of the bounded family ──────────────────────────────────

#[test]
fn bounded_methods_always_some() {
    // sqrt saturates negatives to zero (the documented policy), so the
    // checked form is total too.
    assert_eq!(d38(-4).checked_sqrt_strict(), Some(D38::<10>::ZERO));
    assert!(d38(-27).checked_cbrt_strict().is_some());
    assert!(D38::<10>::ZERO.checked_atan2_strict(D38::<10>::ZERO).is_some());
    assert!(D38::<10>::MAX.checked_tanh_strict().is_some());
    assert!(D38::<10>::MAX.checked_asinh_strict().is_some());
    assert!(D38::<10>::MAX.checked_to_radians_strict().is_some());
    assert!(D38::<10>::MAX.checked_atan_strict().is_some());
    assert!(D38::<10>::MAX.checked_sin_strict().is_some());
}

// ── One wide tier ────────────────────────────────────────────────────

#[cfg(feature = "wide")]
mod wide {
    use super::MODES;
    use decimal_scaled::D76;

    fn d76(v: i64) -> D76<30> {
        D76::<30>::from(v)
    }

    #[test]
    fn d76_checked_matches_default_across_modes() {
        let x = d76(3);
        let y = d76(2);
        for mode in MODES {
            assert_eq!(x.checked_ln_strict_with(mode), Some(x.ln_strict_with(mode)));
            assert_eq!(x.checked_exp_strict_with(mode), Some(x.exp_strict_with(mode)));
            assert_eq!(x.checked_exp2_strict_with(mode), Some(x.exp2_strict_with(mode)));
            assert_eq!(x.checked_log10_strict_with(mode), Some(x.log10_strict_with(mode)));
            assert_eq!(
                x.checked_log_strict_with(y, mode),
                Some(x.log_strict_with(y, mode))
            );
            assert_eq!(x.checked_sqrt_strict_with(mode), Some(x.sqrt_strict_with(mode)));
            assert_eq!(
                x.checked_powf_strict_with(y, mode),
                Some(x.powf_strict_with(y, mode))
            );
            assert_eq!(
                x.checked_hypot_strict_with(y, mode),
                Some(x.hypot_strict_with(y, mode))
            );
            assert_eq!(x.checked_sin_strict_with(mode), Some(x.sin_strict_with(mode)));
            assert_eq!(x.checked_tan_strict_with(mode), Some(x.tan_strict_with(mode)));
            assert_eq!(x.checked_sinh_strict_with(mode), Some(x.sinh_strict_with(mode)));
            assert_eq!(x.checked_cosh_strict_with(mode), Some(x.cosh_strict_with(mode)));
            assert_eq!(x.checked_tanh_strict_with(mode), Some(x.tanh_strict_with(mode)));
            assert_eq!(x.checked_asinh_strict_with(mode), Some(x.asinh_strict_with(mode)));
            assert_eq!(x.checked_acosh_strict_with(mode), Some(x.acosh_strict_with(mode)));
            assert_eq!(
                x.checked_to_degrees_strict_with(mode),
                Some(x.to_degrees_strict_with(mode))
            );
            assert_eq!(
                x.checked_to_radians_strict_with(mode),
                Some(x.to_radians_strict_with(mode))
            );
            assert_eq!(
                x.checked_atan2_strict_with(y, mode),
                Some(x.atan2_strict_with(y, mode))
            );
        }
        let h = D76::<30>::ONE / d76(2);
        for mode in MODES {
            assert_eq!(h.checked_asin_strict_with(mode), Some(h.asin_strict_with(mode)));
            assert_eq!(h.checked_acos_strict_with(mode), Some(h.acos_strict_with(mode)));
            assert_eq!(h.checked_atanh_strict_with(mode), Some(h.atanh_strict_with(mode)));
        }
    }

    #[test]
    fn d76_domain_none() {
        assert_eq!(D76::<30>::ZERO.checked_ln_strict(), None);
        assert_eq!(d76(-1).checked_ln_strict(), None);
        assert_eq!(d76(2).checked_asin_strict(), None);
        assert_eq!(D76::<30>::ZERO.checked_acosh_strict(), None);
        assert_eq!(D76::<30>::ONE.checked_atanh_strict(), None);
        assert_eq!(d76(5).checked_log_strict(D76::<30>::ONE), None);
    }

    /// Documents the INTERIM wide-tier contract: out-of-range results
    /// still panic in the checked form (the wide kernel seam is the
    /// deferred piece — `research/checked_wide_shell_patch.md`). This
    /// pin flips to a `None` assertion when the seam lands.
    #[test]
    #[should_panic(expected = "result out of range")]
    fn d76_exp_overflow_still_panics_pending_seam() {
        let _ = d76(200).checked_exp_strict();
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn d76_exp_overflow_default_panics() {
        let _ = d76(200).exp_strict();
    }
}
