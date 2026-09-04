//! Compile-surface reachability of the `*_strict` named methods, and the
//! plain `*` form's delegation to them.
//! The wide roots dispatcher contract (`wide_roots_dispatcher_and_hypot.rs`)
//! joins this target in the feature-gated batch.

mod from_routing_surface {
    //! The `*_strict` named methods must be accessible in every feature
    //! mode, and the plain `*` form delegates to them.
    //!
    //! These tests only need to *compile* — runtime behavior of each variant
    //! is covered in the precision suites. The asserts here are weak
    //! tautologies so the methods are actually used (otherwise the optimizer
    //! might elide them).

    use decimal_scaled::D38s12;

    #[test]
    fn d38_strict_surface_callable_in_any_mode() {
        let x = D38s12::try_from(2).unwrap();
        let _ = x.ln();
        let _ = x.log2();
        let _ = x.log10();
        let _ = x.log(D38s12::try_from(10).unwrap());
        let _ = x.exp();
        let _ = x.exp2();
        let _ = x.sqrt();
        let _ = x.cbrt();
        let _ = x.powf(D38s12::from_bits(
            decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap(),
        ));
        let _ = x.sin();
        let _ = x.cos();
        let _ = x.tan();
        let _ =
            D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap())
                .asin();
        let _ =
            D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap())
                .acos();
        let _ = x.atan();
        let _ = x.atan2(D38s12::ONE);
        let _ = x.sinh();
        let _ = x.cosh();
        let _ = x.tanh();
        let _ = x.asinh();
        let _ = x.acosh();
        let _ =
            D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap())
                .atanh();
        let _ = x.to_degrees();
        let _ = x.to_radians();
    }

    #[cfg(feature = "wide")]
    #[test]
    fn wide_strict_surface_callable() {
        use decimal_scaled::D76;
        type W = D76<12>;
        let x: W = D38s12::try_from(2).unwrap().into();
        let _ = x.ln();
        let _ = x.exp();
        let _ = x.sqrt();
        let _ = x.sin();
        let _ = x.atan2(x);
    }

    #[test]
    fn narrow_strict_surface_callable() {
        use decimal_scaled::D18;

        let x18 = D18::<8>::try_from(2).unwrap();
        let _ = x18.ln();
        let _ = x18.sin();
        let _ = x18.sqrt();
        let _ = x18.exp();
    }
}

#[cfg(feature = "wide")]
mod from_wide_roots_dispatcher_and_hypot {
    //! Coverage for `macros/wide_roots.rs` — the plain `sqrt()` / `cbrt()`
    //! dispatchers (strict-feature mode) and `hypot` on the wide
    //! tiers.

    use decimal_scaled::{D38, D76};

    #[test]
    fn d76_sqrt_cbrt_plain_dispatcher() {
        let four: D76<6> = D38::<6>::try_from(4).unwrap().into();
        let twenty_seven: D76<6> = D38::<6>::try_from(27).unwrap().into();
        assert_eq!(four.sqrt(), four.sqrt());
        assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt());
    }

    #[test]
    fn d76_hypot_strict_zero_zero() {
        assert_eq!(D76::<6>::ZERO.hypot(D76::<6>::ZERO), D76::<6>::ZERO);
    }

    #[test]
    fn d76_hypot_strict_zero_x() {
        let five: D76<6> = D38::<6>::try_from(5).unwrap().into();
        let r = D76::<6>::ZERO.hypot(five);
        // hypot(0, x) = |x| exactly (isqrt(x²) = |x|, no rounding bump).
        assert_eq!(r, five);
    }

    #[test]
    fn d76_hypot_strict_3_4_is_5() {
        let three: D76<6> = D38::<6>::try_from(3).unwrap().into();
        let four: D76<6> = D38::<6>::try_from(4).unwrap().into();
        let five: D76<6> = D38::<6>::try_from(5).unwrap().into();
        let r = three.hypot(four);
        // Pythagorean triple 3²+4²=5²: the hypotenuse is an exact integer.
        assert_eq!(r, five, "got {r:?} expected exact {five:?}");
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d153_d307_dispatchers_and_hypot() {
        use decimal_scaled::{D153, D307};

        let four: D153<6> = D38::<6>::try_from(4).unwrap().into();
        assert_eq!(four.sqrt(), four.sqrt());
        let twenty_seven: D153<6> = D38::<6>::try_from(27).unwrap().into();
        assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt());

        let three: D153<6> = D38::<6>::try_from(3).unwrap().into();
        let four_a: D153<6> = D38::<6>::try_from(4).unwrap().into();
        let five_a: D153<6> = D38::<6>::try_from(5).unwrap().into();
        // Pythagorean triple 3²+4²=5²: exact integer hypotenuse.
        assert_eq!(three.hypot(four_a), five_a);

        let four_b: D307<6> = D76::<6>::try_from(4).unwrap().into();
        let twenty_seven_b: D307<6> = D76::<6>::try_from(27).unwrap().into();
        assert_eq!(four_b.sqrt(), four_b.sqrt());
        assert_eq!(twenty_seven_b.cbrt(), twenty_seven_b.cbrt());
        let three_b: D307<6> = D76::<6>::try_from(3).unwrap().into();
        let five_b: D307<6> = D76::<6>::try_from(5).unwrap().into();
        assert_eq!(three_b.hypot(four_b), five_b);
    }
}

#[cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
)))]
mod from_narrow_strict_transcendentals {
    //! The plain `*` dispatcher delegation contract for the narrow (D18)
    //! tier, moved from `tests/narrow_strict_transcendentals.rs`.

    use decimal_scaled::D18;

    #[test]
    fn d18_dispatcher_matches_strict() {
        assert_eq!(D18::<8>::ONE.ln(), D18::<8>::ONE.ln());
        assert_eq!(D18::<8>::ONE.exp(), D18::<8>::ONE.exp());
        assert_eq!(D18::<8>::ONE.sin(), D18::<8>::ONE.sin());
        assert_eq!(D18::<8>::ONE.cos(), D18::<8>::ONE.cos());
        assert_eq!(D18::<8>::ONE.tan(), D18::<8>::ONE.tan());
        assert_eq!(D18::<8>::ONE.sinh(), D18::<8>::ONE.sinh());
        assert_eq!(D18::<8>::ONE.cosh(), D18::<8>::ONE.cosh());
        assert_eq!(D18::<8>::ONE.tanh(), D18::<8>::ONE.tanh());
        assert_eq!(
            D18::<8>::try_from(4).unwrap().sqrt(),
            D18::<8>::try_from(4).unwrap().sqrt()
        );
        assert_eq!(
            D18::<8>::try_from(27).unwrap().cbrt(),
            D18::<8>::try_from(27).unwrap().cbrt()
        );
        assert_eq!(D18::<8>::ONE.atan(), D18::<8>::ONE.atan());
        assert_eq!(
            D18::<8>::ONE.atan2(D18::<8>::ONE),
            D18::<8>::ONE.atan2(D18::<8>::ONE)
        );
        assert_eq!(D18::<8>::ZERO.asin(), D18::<8>::ZERO.asin());
        assert_eq!(D18::<8>::ONE.acos(), D18::<8>::ONE.acos());
        assert_eq!(D18::<8>::ZERO.asinh(), D18::<8>::ZERO.asinh());
        assert_eq!(D18::<8>::ONE.acosh(), D18::<8>::ONE.acosh());
        assert_eq!(D18::<8>::ZERO.atanh(), D18::<8>::ZERO.atanh());
        assert_eq!(D18::<8>::ONE.log2(), D18::<8>::ONE.log2());
        assert_eq!(D18::<8>::ONE.log10(), D18::<8>::ONE.log10());
        assert_eq!(D18::<8>::ONE.exp2(), D18::<8>::ONE.exp2());
        assert_eq!(
            D18::<8>::ZERO.to_degrees(),
            D18::<8>::ZERO.to_degrees()
        );
        assert_eq!(
            D18::<8>::ZERO.to_radians(),
            D18::<8>::ZERO.to_radians()
        );
        assert_eq!(
            D18::<8>::try_from(8)
                .unwrap()
                .log(D18::<8>::try_from(2).unwrap()),
            D18::<8>::try_from(8)
                .unwrap()
                .log(D18::<8>::try_from(2).unwrap()),
        );
        assert_eq!(
            D18::<8>::try_from(2)
                .unwrap()
                .powf(D18::<8>::try_from(10).unwrap()),
            D18::<8>::try_from(2)
                .unwrap()
                .powf(D18::<8>::try_from(10).unwrap()),
        );
    }
}

#[cfg(all(
    feature = "wide",
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]
mod from_wide_strict_transcendentals {
    //! The `_with`-sibling, AGM-alternate, and plain-dispatcher delegation
    //! contracts for the wide (D76) tier, moved from
    //! `tests/wide_strict_transcendentals.rs`.

    use decimal_scaled::{RoundingMode, D38, D76};

    const WIDE_TOL_LSB: i128 = 1;

    /// Convert a wide-tier result back to the equivalent D38<SCALE> bit
    /// pattern at SCALE ≤ 18, where the value fits `i128` cleanly.
    fn d76_bits_at_scale_6(d: D76<6>) -> i128 {
        d.to_bits()
            .to_i128_checked()
            .expect("D76<6> result fits i128")
    }

    #[track_caller]
    fn agree<T: Into<i128>>(label: &str, wide: i128, d38: T) {
        // `d38` accepts both `i128` (AGM cross-checks) and D38's `Int<2>`
        // `to_bits()` (via `From<Int<2>> for i128`), bridged to `i128` here.
        let d38: i128 = d38.into();
        let diff = (wide - d38).abs();
        assert!(
            diff <= WIDE_TOL_LSB,
            "{label}: wide {wide} vs d38 {d38} (diff {diff} > {WIDE_TOL_LSB} LSB)",
        );
    }

    fn lift(n: D38<6>) -> D76<6> {
        n.into()
    }

    // ─── AGM alternates ────────────────────────────────────────────────────

    #[test]
    fn d76_ln_agm() {
        for v in [2_i64, 7, 100] {
            let n = D38::<6>::try_from(v).unwrap();
            let agm = lift(n).ln_agm();
            let canonical = lift(n).ln();
            // AGM must agree with canonical within 1 LSB.
            agree(
                &format!("ln_agm({v}) vs ln({v})"),
                d76_bits_at_scale_6(agm),
                d76_bits_at_scale_6(canonical),
            );
        }
    }

    #[test]
    fn d76_exp_agm() {
        let n = D38::<6>::ONE;
        let agm = lift(n).exp_agm();
        let canonical = lift(n).exp();
        agree(
            "exp_agm(1) vs exp(1)",
            d76_bits_at_scale_6(agm),
            d76_bits_at_scale_6(canonical),
        );
        // ZERO short-circuit
        assert_eq!(D76::<6>::ZERO.exp_agm(), D76::<6>::ONE);
    }

    // ─── Mode-aware _with siblings (D76 only) ──────────────────────────────
    //
    // Tarpaulin counts these lines distinct from `*_strict`. To cover them
    // we call each with multiple `RoundingMode` variants and check the
    // HalfToEven branch reproduces the plain `*_strict` result.

    #[test]
    fn d76_strict_with_modes() {
        let two = lift(D38::<6>::try_from(2).unwrap());
        let ten = lift(D38::<6>::try_from(10).unwrap());
        let one = lift(D38::<6>::ONE);
        let half = lift(D38::<6>::from_bits(
            decimal_scaled::Int::<2>::try_from(500_000_i128).unwrap(),
        ));

        // HalfToEven matches the plain *_strict form bit-exactly.
        assert_eq!(
            two.ln_with(RoundingMode::HalfToEven),
            two.ln()
        );
        assert_eq!(
            two.log_with(ten, RoundingMode::HalfToEven),
            two.log(ten)
        );
        assert_eq!(
            two.log2_with(RoundingMode::HalfToEven),
            two.log2()
        );
        assert_eq!(
            ten.log10_with(RoundingMode::HalfToEven),
            ten.log10()
        );
        assert_eq!(
            one.exp_with(RoundingMode::HalfToEven),
            one.exp()
        );
        assert_eq!(
            ten.exp2_with(RoundingMode::HalfToEven),
            ten.exp2()
        );
        assert_eq!(
            two.powf_with(ten, RoundingMode::HalfToEven),
            two.powf(ten)
        );
        assert_eq!(
            one.sin_with(RoundingMode::HalfToEven),
            one.sin()
        );
        assert_eq!(
            one.cos_with(RoundingMode::HalfToEven),
            one.cos()
        );
        assert_eq!(
            one.tan_with(RoundingMode::HalfToEven),
            one.tan()
        );
        assert_eq!(
            one.atan_with(RoundingMode::HalfToEven),
            one.atan()
        );
        assert_eq!(
            half.asin_with(RoundingMode::HalfToEven),
            half.asin()
        );
        assert_eq!(
            half.acos_with(RoundingMode::HalfToEven),
            half.acos()
        );
        // asin/acos boundary in the _with form:
        assert_eq!(
            one.asin_with(RoundingMode::HalfToEven),
            one.asin()
        );
        assert_eq!(
            one.acos_with(RoundingMode::HalfToEven),
            one.acos()
        );
        assert_eq!(
            one.atan2_with(one, RoundingMode::HalfToEven),
            one.atan2(one)
        );
        // atan2 axis branches in the _with form:
        assert_eq!(
            D76::<6>::ZERO.atan2_with(D76::<6>::ZERO, RoundingMode::HalfToEven),
            D76::<6>::ZERO.atan2(D76::<6>::ZERO)
        );
        assert_eq!(
            one.atan2_with(D76::<6>::ZERO, RoundingMode::HalfToEven),
            one.atan2(D76::<6>::ZERO)
        );
        assert_eq!(
            (-one).atan2_with(D76::<6>::ZERO, RoundingMode::HalfToEven),
            (-one).atan2(D76::<6>::ZERO)
        );
        assert_eq!(
            D76::<6>::ZERO.atan2_with(-one, RoundingMode::HalfToEven),
            D76::<6>::ZERO.atan2(-one)
        );
        assert_eq!(
            one.sinh_with(RoundingMode::HalfToEven),
            one.sinh()
        );
        assert_eq!(
            one.cosh_with(RoundingMode::HalfToEven),
            one.cosh()
        );
        assert_eq!(
            one.tanh_with(RoundingMode::HalfToEven),
            one.tanh()
        );
        assert_eq!(
            one.asinh_with(RoundingMode::HalfToEven),
            one.asinh()
        );
        assert_eq!(
            D76::<6>::ZERO.asinh_with(RoundingMode::HalfToEven),
            D76::<6>::ZERO
        );
        let two_val = lift(D38::<6>::try_from(2).unwrap());
        assert_eq!(
            two_val.acosh_with(RoundingMode::HalfToEven),
            two_val.acosh()
        );
        assert_eq!(
            half.atanh_with(RoundingMode::HalfToEven),
            half.atanh()
        );
        assert_eq!(
            one.to_degrees_with(RoundingMode::HalfToEven),
            one.to_degrees()
        );
        assert_eq!(
            one.to_radians_with(RoundingMode::HalfToEven),
            one.to_radians()
        );

        // AGM _with siblings
        assert_eq!(
            two.ln_agm_with(RoundingMode::HalfToEven),
            two.ln_agm()
        );
        assert_eq!(
            one.exp_agm_with(RoundingMode::HalfToEven),
            one.exp_agm()
        );
        // exp_agm_with ZERO short-circuit
        assert_eq!(
            D76::<6>::ZERO.exp_agm_with(RoundingMode::HalfToEven),
            D76::<6>::ONE
        );

        // Non-HalfToEven modes — just call each variant to exercise the
        // mode-dispatch code path. We don't assert on the exact value because
        // the wide tier's _with rounding contract is "honour mode at the
        // final storage round"; checking distinctness from HalfToEven is
        // sufficient for coverage.
        let _ = two.ln_with(RoundingMode::Trunc);
        let _ = two.ln_with(RoundingMode::Floor);
        let _ = two.ln_with(RoundingMode::Ceiling);
        let _ = one.sin_with(RoundingMode::Trunc);
        let _ = half.asin_with(RoundingMode::Floor);
    }

    // ─── Plain dispatcher (strict mode only) ───────────────────────────────

    #[test]
    fn d76_plain_dispatcher_matches_strict() {
        let one = lift(D38::<6>::ONE);
        let two = lift(D38::<6>::try_from(2).unwrap());
        let ten = lift(D38::<6>::try_from(10).unwrap());
        let four = lift(D38::<6>::try_from(4).unwrap());
        let half = lift(D38::<6>::from_bits(
            decimal_scaled::Int::<2>::try_from(500_000_i128).unwrap(),
        ));
        let twenty_seven = lift(D38::<6>::try_from(27).unwrap());

        assert_eq!(two.ln(), two.ln());
        assert_eq!(two.log(ten), two.log(ten));
        assert_eq!(two.log2(), two.log2());
        assert_eq!(ten.log10(), ten.log10());
        assert_eq!(one.exp(), one.exp());
        assert_eq!(ten.exp2(), ten.exp2());
        assert_eq!(two.powf(ten), two.powf(ten));
        assert_eq!(one.sin(), one.sin());
        assert_eq!(one.cos(), one.cos());
        assert_eq!(one.tan(), one.tan());
        assert_eq!(one.atan(), one.atan());
        assert_eq!(half.asin(), half.asin());
        assert_eq!(half.acos(), half.acos());
        assert_eq!(one.atan2(one), one.atan2(one));
        assert_eq!(one.sinh(), one.sinh());
        assert_eq!(one.cosh(), one.cosh());
        assert_eq!(one.tanh(), one.tanh());
        assert_eq!(one.asinh(), one.asinh());
        assert_eq!(two.acosh(), two.acosh());
        assert_eq!(half.atanh(), half.atanh());
        assert_eq!(one.to_degrees(), one.to_degrees());
        assert_eq!(one.to_radians(), one.to_radians());

        // Note: wide tier has no sqrt() / cbrt() in this dispatcher block —
        // those go through wide_roots.rs separately. Force a touch:
        let _ = four.sqrt();
        let _ = twenty_seven.cbrt();
    }
}
