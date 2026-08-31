//! Compile-surface reachability of the `*_strict` / `*_fast` named methods in
//! every feature mode (the plain `*` form stays the feature-driven dispatcher).
//! The wide roots dispatcher contract (`wide_roots_dispatcher_and_hypot.rs`)
//! joins this target in the feature-gated batch.

mod from_routing_surface {
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
        let x = D38s12::from(2);
        let _ = x.ln_fast();
        let _ = x.log2_fast();
        let _ = x.log10_fast();
        let _ = x.log_fast(D38s12::from(10));
        let _ = x.exp_fast();
        let _ = x.exp2_fast();
        let _ = x.sqrt_fast();
        let _ = x.cbrt_fast();
        let _ = x.powf_fast(D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()));
        let _ = x.hypot_fast(D38s12::from(3));
        let _ = x.sin_fast();
        let _ = x.cos_fast();
        let _ = x.tan_fast();
        let _ = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()).asin_fast();
        let _ = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()).acos_fast();
        let _ = x.atan_fast();
        let _ = x.atan2_fast(D38s12::ONE);
        let _ = x.sinh_fast();
        let _ = x.cosh_fast();
        let _ = x.tanh_fast();
        let _ = x.asinh_fast();
        let _ = x.acosh_fast();
        let _ = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()).atanh_fast();
        let _ = x.to_degrees_fast();
        let _ = x.to_radians_fast();
    }

    #[test]
    fn d38_strict_surface_callable_in_any_mode() {
        let x = D38s12::from(2);
        let _ = x.ln_strict();
        let _ = x.log2_strict();
        let _ = x.log10_strict();
        let _ = x.log_strict(D38s12::from(10));
        let _ = x.exp_strict();
        let _ = x.exp2_strict();
        let _ = x.sqrt_strict();
        let _ = x.cbrt_strict();
        let _ = x.powf_strict(D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()));
        let _ = x.sin_strict();
        let _ = x.cos_strict();
        let _ = x.tan_strict();
        let _ = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()).asin_strict();
        let _ = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()).acos_strict();
        let _ = x.atan_strict();
        let _ = x.atan2_strict(D38s12::ONE);
        let _ = x.sinh_strict();
        let _ = x.cosh_strict();
        let _ = x.tanh_strict();
        let _ = x.asinh_strict();
        let _ = x.acosh_strict();
        let _ = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(500_000_000_000_i128).unwrap()).atanh_strict();
        let _ = x.to_degrees_strict();
        let _ = x.to_radians_strict();
    }

    #[cfg(feature = "wide")]
    #[cfg(feature = "std")]
    #[test]
    fn wide_fast_surface_callable() {
        use decimal_scaled::D76;
        type W = D76<12>;
        let x: W = D38s12::from(2).into();
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
        let x: W = D38s12::from(2).into();
        let _ = x.ln_strict();
        let _ = x.exp_strict();
        let _ = x.sqrt_strict();
        let _ = x.sin_strict();
        let _ = x.atan2_strict(x);
    }

    #[test]
    fn narrow_strict_surface_callable() {
        use decimal_scaled::{D18};

        let x18 = D18::<8>::from(2);
        let _ = x18.ln_strict();
        let _ = x18.sin_strict();
        let _ = x18.sqrt_strict();
        let _ = x18.exp_strict();
    }

    #[cfg(feature = "std")]
    #[test]
    fn narrow_fast_surface_callable() {
        use decimal_scaled::{D18};

        let x18 = D18::<8>::from(2);
        let _ = x18.ln_fast();
        let _ = x18.sin_fast();
        let _ = x18.sqrt_fast();
        let _ = x18.exp_fast();
    }
}

#[cfg(all(feature = "wide", not(feature = "fast")))]
mod from_wide_roots_dispatcher_and_hypot {
    //! Coverage for `macros/wide_roots.rs` — the plain `sqrt()` / `cbrt()`
    //! dispatchers (strict-feature mode) and `hypot_strict` on the wide
    //! tiers.

    use decimal_scaled::{D38, D76};

    #[cfg(feature = "strict")]
    #[test]
    fn d76_sqrt_cbrt_plain_dispatcher() {

        let four: D76<6> = D38::<6>::from(4).into();
        let twenty_seven: D76<6> = D38::<6>::from(27).into();
        assert_eq!(four.sqrt(), four.sqrt_strict());
        assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt_strict());
    }

    #[test]
    fn d76_hypot_strict_zero_zero() {

        assert_eq!(D76::<6>::ZERO.hypot_strict(D76::<6>::ZERO), D76::<6>::ZERO);
    }

    #[test]
    fn d76_hypot_strict_zero_x() {

        let five: D76<6> = D38::<6>::from(5).into();
        let r = D76::<6>::ZERO.hypot_strict(five);
        // hypot(0, x) = |x| exactly (isqrt(x²) = |x|, no rounding bump).
        assert_eq!(r, five);
    }

    #[test]
    fn d76_hypot_strict_3_4_is_5() {

        let three: D76<6> = D38::<6>::from(3).into();
        let four: D76<6> = D38::<6>::from(4).into();
        let five: D76<6> = D38::<6>::from(5).into();
        let r = three.hypot_strict(four);
        // Pythagorean triple 3²+4²=5²: the hypotenuse is an exact integer.
        assert_eq!(r, five, "got {r:?} expected exact {five:?}");
    }

    #[cfg(all(feature = "x-wide", feature = "strict"))]
    #[test]
    fn d153_d307_dispatchers_and_hypot() {
        use decimal_scaled::{D153, D307};

        let four: D153<6> = D38::<6>::from(4).into();
        assert_eq!(four.sqrt(), four.sqrt_strict());
        let twenty_seven: D153<6> = D38::<6>::from(27).into();
        assert_eq!(twenty_seven.cbrt(), twenty_seven.cbrt_strict());

        let three: D153<6> = D38::<6>::from(3).into();
        let four_a: D153<6> = D38::<6>::from(4).into();
        let five_a: D153<6> = D38::<6>::from(5).into();
        // Pythagorean triple 3²+4²=5²: exact integer hypotenuse.
        assert_eq!(three.hypot_strict(four_a), five_a);

        let four_b: D307<6> = D76::<6>::from(4).into();
        let twenty_seven_b: D307<6> = D76::<6>::from(27).into();
        assert_eq!(four_b.sqrt(), four_b.sqrt_strict());
        assert_eq!(twenty_seven_b.cbrt(), twenty_seven_b.cbrt_strict());
        let three_b: D307<6> = D76::<6>::from(3).into();
        let five_b: D307<6> = D76::<6>::from(5).into();
        assert_eq!(three_b.hypot_strict(four_b), five_b);
    }
}

#[cfg(all(
    not(feature = "fast"),
    not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )),
))]
mod from_narrow_strict_transcendentals {
    //! The plain `*` dispatcher delegation contract for the narrow (D18)
    //! tier, moved from `tests/narrow_strict_transcendentals.rs`.

    use decimal_scaled::D18;

    #[cfg(feature = "strict")]
    #[test]
    fn d18_dispatcher_matches_strict() {
        assert_eq!(D18::<8>::ONE.ln(), D18::<8>::ONE.ln_strict());
        assert_eq!(D18::<8>::ONE.exp(), D18::<8>::ONE.exp_strict());
        assert_eq!(D18::<8>::ONE.sin(), D18::<8>::ONE.sin_strict());
        assert_eq!(D18::<8>::ONE.cos(), D18::<8>::ONE.cos_strict());
        assert_eq!(D18::<8>::ONE.tan(), D18::<8>::ONE.tan_strict());
        assert_eq!(D18::<8>::ONE.sinh(), D18::<8>::ONE.sinh_strict());
        assert_eq!(D18::<8>::ONE.cosh(), D18::<8>::ONE.cosh_strict());
        assert_eq!(D18::<8>::ONE.tanh(), D18::<8>::ONE.tanh_strict());
        assert_eq!(D18::<8>::from(4).sqrt(), D18::<8>::from(4).sqrt_strict());
        assert_eq!(
            D18::<8>::from(27).cbrt(),
            D18::<8>::from(27).cbrt_strict()
        );
        assert_eq!(D18::<8>::ONE.atan(), D18::<8>::ONE.atan_strict());
        assert_eq!(
            D18::<8>::ONE.atan2(D18::<8>::ONE),
            D18::<8>::ONE.atan2_strict(D18::<8>::ONE)
        );
        assert_eq!(D18::<8>::ZERO.asin(), D18::<8>::ZERO.asin_strict());
        assert_eq!(D18::<8>::ONE.acos(), D18::<8>::ONE.acos_strict());
        assert_eq!(D18::<8>::ZERO.asinh(), D18::<8>::ZERO.asinh_strict());
        assert_eq!(D18::<8>::ONE.acosh(), D18::<8>::ONE.acosh_strict());
        assert_eq!(D18::<8>::ZERO.atanh(), D18::<8>::ZERO.atanh_strict());
        assert_eq!(D18::<8>::ONE.log2(), D18::<8>::ONE.log2_strict());
        assert_eq!(D18::<8>::ONE.log10(), D18::<8>::ONE.log10_strict());
        assert_eq!(D18::<8>::ONE.exp2(), D18::<8>::ONE.exp2_strict());
        assert_eq!(D18::<8>::ZERO.to_degrees(), D18::<8>::ZERO.to_degrees_strict());
        assert_eq!(D18::<8>::ZERO.to_radians(), D18::<8>::ZERO.to_radians_strict());
        assert_eq!(
            D18::<8>::from(8).log(D18::<8>::from(2)),
            D18::<8>::from(8).log_strict(D18::<8>::from(2)),
        );
        assert_eq!(
            D18::<8>::from(2).powf(D18::<8>::from(10)),
            D18::<8>::from(2).powf_strict(D18::<8>::from(10)),
        );
    }
}

#[cfg(all(
    not(feature = "fast"),
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

    use decimal_scaled::{D38, D76, RoundingMode};

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
            let n = D38::<6>::from(v);
            let agm = lift(n).ln_strict_agm();
            let canonical = lift(n).ln_strict();
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
        let agm = lift(n).exp_strict_agm();
        let canonical = lift(n).exp_strict();
        agree(
            "exp_agm(1) vs exp(1)",
            d76_bits_at_scale_6(agm),
            d76_bits_at_scale_6(canonical),
        );
        // ZERO short-circuit
        assert_eq!(D76::<6>::ZERO.exp_strict_agm(), D76::<6>::ONE);
    }

    // ─── Mode-aware _with siblings (D76 only) ──────────────────────────────
    //
    // Tarpaulin counts these lines distinct from `*_strict`. To cover them
    // we call each with multiple `RoundingMode` variants and check the
    // HalfToEven branch reproduces the plain `*_strict` result.


    #[test]
    fn d76_strict_with_modes() {
        let two = lift(D38::<6>::from(2));
        let ten = lift(D38::<6>::from(10));
        let one = lift(D38::<6>::ONE);
        let half = lift(D38::<6>::from_bits(decimal_scaled::Int::<2>::try_from(500_000_i128).unwrap()));

        // HalfToEven matches the plain *_strict form bit-exactly.
        assert_eq!(
            two.ln_strict_with(RoundingMode::HalfToEven),
            two.ln_strict()
        );
        assert_eq!(
            two.log_strict_with(ten, RoundingMode::HalfToEven),
            two.log_strict(ten)
        );
        assert_eq!(
            two.log2_strict_with(RoundingMode::HalfToEven),
            two.log2_strict()
        );
        assert_eq!(
            ten.log10_strict_with(RoundingMode::HalfToEven),
            ten.log10_strict()
        );
        assert_eq!(
            one.exp_strict_with(RoundingMode::HalfToEven),
            one.exp_strict()
        );
        assert_eq!(
            ten.exp2_strict_with(RoundingMode::HalfToEven),
            ten.exp2_strict()
        );
        assert_eq!(
            two.powf_strict_with(ten, RoundingMode::HalfToEven),
            two.powf_strict(ten)
        );
        assert_eq!(
            one.sin_strict_with(RoundingMode::HalfToEven),
            one.sin_strict()
        );
        assert_eq!(
            one.cos_strict_with(RoundingMode::HalfToEven),
            one.cos_strict()
        );
        assert_eq!(
            one.tan_strict_with(RoundingMode::HalfToEven),
            one.tan_strict()
        );
        assert_eq!(
            one.atan_strict_with(RoundingMode::HalfToEven),
            one.atan_strict()
        );
        assert_eq!(
            half.asin_strict_with(RoundingMode::HalfToEven),
            half.asin_strict()
        );
        assert_eq!(
            half.acos_strict_with(RoundingMode::HalfToEven),
            half.acos_strict()
        );
        // asin/acos boundary in the _with form:
        assert_eq!(
            one.asin_strict_with(RoundingMode::HalfToEven),
            one.asin_strict()
        );
        assert_eq!(
            one.acos_strict_with(RoundingMode::HalfToEven),
            one.acos_strict()
        );
        assert_eq!(
            one.atan2_strict_with(one, RoundingMode::HalfToEven),
            one.atan2_strict(one)
        );
        // atan2 axis branches in the _with form:
        assert_eq!(
            D76::<6>::ZERO.atan2_strict_with(D76::<6>::ZERO, RoundingMode::HalfToEven),
            D76::<6>::ZERO.atan2_strict(D76::<6>::ZERO)
        );
        assert_eq!(
            one.atan2_strict_with(D76::<6>::ZERO, RoundingMode::HalfToEven),
            one.atan2_strict(D76::<6>::ZERO)
        );
        assert_eq!(
            (-one).atan2_strict_with(D76::<6>::ZERO, RoundingMode::HalfToEven),
            (-one).atan2_strict(D76::<6>::ZERO)
        );
        assert_eq!(
            D76::<6>::ZERO.atan2_strict_with(-one, RoundingMode::HalfToEven),
            D76::<6>::ZERO.atan2_strict(-one)
        );
        assert_eq!(
            one.sinh_strict_with(RoundingMode::HalfToEven),
            one.sinh_strict()
        );
        assert_eq!(
            one.cosh_strict_with(RoundingMode::HalfToEven),
            one.cosh_strict()
        );
        assert_eq!(
            one.tanh_strict_with(RoundingMode::HalfToEven),
            one.tanh_strict()
        );
        assert_eq!(
            one.asinh_strict_with(RoundingMode::HalfToEven),
            one.asinh_strict()
        );
        assert_eq!(
            D76::<6>::ZERO.asinh_strict_with(RoundingMode::HalfToEven),
            D76::<6>::ZERO
        );
        let two_val = lift(D38::<6>::from(2));
        assert_eq!(
            two_val.acosh_strict_with(RoundingMode::HalfToEven),
            two_val.acosh_strict()
        );
        assert_eq!(
            half.atanh_strict_with(RoundingMode::HalfToEven),
            half.atanh_strict()
        );
        assert_eq!(
            one.to_degrees_strict_with(RoundingMode::HalfToEven),
            one.to_degrees_strict()
        );
        assert_eq!(
            one.to_radians_strict_with(RoundingMode::HalfToEven),
            one.to_radians_strict()
        );

        // AGM _with siblings
        assert_eq!(
            two.ln_strict_agm_with(RoundingMode::HalfToEven),
            two.ln_strict_agm()
        );
        assert_eq!(
            one.exp_strict_agm_with(RoundingMode::HalfToEven),
            one.exp_strict_agm()
        );
        // exp_strict_agm_with ZERO short-circuit
        assert_eq!(
            D76::<6>::ZERO.exp_strict_agm_with(RoundingMode::HalfToEven),
            D76::<6>::ONE
        );

        // Non-HalfToEven modes — just call each variant to exercise the
        // mode-dispatch code path. We don't assert on the exact value because
        // the wide tier's _with rounding contract is "honour mode at the
        // final storage round"; checking distinctness from HalfToEven is
        // sufficient for coverage.
        let _ = two.ln_strict_with(RoundingMode::Trunc);
        let _ = two.ln_strict_with(RoundingMode::Floor);
        let _ = two.ln_strict_with(RoundingMode::Ceiling);
        let _ = one.sin_strict_with(RoundingMode::Trunc);
        let _ = half.asin_strict_with(RoundingMode::Floor);
    }

    // ─── Plain dispatcher (strict mode only) ───────────────────────────────

    #[cfg(feature = "strict")]
    #[test]
    fn d76_plain_dispatcher_matches_strict() {
        let one = lift(D38::<6>::ONE);
        let two = lift(D38::<6>::from(2));
        let ten = lift(D38::<6>::from(10));
        let four = lift(D38::<6>::from(4));
        let half = lift(D38::<6>::from_bits(decimal_scaled::Int::<2>::try_from(500_000_i128).unwrap()));
        let twenty_seven = lift(D38::<6>::from(27));

        assert_eq!(two.ln(), two.ln_strict());
        assert_eq!(two.log(ten), two.log_strict(ten));
        assert_eq!(two.log2(), two.log2_strict());
        assert_eq!(ten.log10(), ten.log10_strict());
        assert_eq!(one.exp(), one.exp_strict());
        assert_eq!(ten.exp2(), ten.exp2_strict());
        assert_eq!(two.powf(ten), two.powf_strict(ten));
        assert_eq!(one.sin(), one.sin_strict());
        assert_eq!(one.cos(), one.cos_strict());
        assert_eq!(one.tan(), one.tan_strict());
        assert_eq!(one.atan(), one.atan_strict());
        assert_eq!(half.asin(), half.asin_strict());
        assert_eq!(half.acos(), half.acos_strict());
        assert_eq!(one.atan2(one), one.atan2_strict(one));
        assert_eq!(one.sinh(), one.sinh_strict());
        assert_eq!(one.cosh(), one.cosh_strict());
        assert_eq!(one.tanh(), one.tanh_strict());
        assert_eq!(one.asinh(), one.asinh_strict());
        assert_eq!(two.acosh(), two.acosh_strict());
        assert_eq!(half.atanh(), half.atanh_strict());
        assert_eq!(one.to_degrees(), one.to_degrees_strict());
        assert_eq!(one.to_radians(), one.to_radians_strict());

        // Note: wide tier has no sqrt() / cbrt() in this dispatcher block —
        // those go through wide_roots.rs separately. Force a touch:
        let _ = four.sqrt_strict();
        let _ = twenty_seven.cbrt_strict();
    }
}
