//! `rescale` / `rescale_with` / `with_scale` semantics: direction, tie
//! handling per mode, identity, and the scale-up overflow panic.

mod from_rescale {
    //! `D38::rescale` / `rescale_with` integration tests. Moved out of
    //! `src/rescale.rs` so that file carries only macro invocations.
    //!
    //! Several tests below use the plain `rescale::<N>()` form, whose
    //! rounding behaviour depends on the crate-default mode. Compile-gate
    //! the whole file to the `HalfToEven` default so every test always
    //! executes its assertions (no silent skip under a `rounding-*` build).

    #![cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]

    use decimal_scaled::{D38s2, D38s6, D38s12, RoundingMode};

    // --- with_scale alias ----------------------------------------------

    #[test]
    fn with_scale_matches_rescale() {
        // Native tier.
        let a = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap());
        assert_eq!(i128::from(a.with_scale::<6>().to_bits()), i128::from(a.rescale::<6>().to_bits()));
        assert_eq!(i128::from(a.with_scale::<2>().to_bits()), i128::from(a.to_bits()));

        // The builder-style name is the only difference; semantics are
        // bit-identical to rescale.
        let b = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(12_345_678_901_234_i128).unwrap());
        assert_eq!(i128::from(b.with_scale::<6>().to_bits()), i128::from(b.rescale::<6>().to_bits()));
    }

    // --- scale-up direction --------------------------------------------

    #[test]
    fn rescale_up_appends_zeros() {
        let cents = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap());
        let micros = cents.rescale::<6>();
        assert_eq!(i128::from(micros.to_bits()), 1_500_000);
    }

    #[test]
    fn rescale_up_negative() {
        let cents = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(-150_i128).unwrap());
        let micros = cents.rescale::<6>();
        assert_eq!(i128::from(micros.to_bits()), -1_500_000);
    }

    #[test]
    fn rescale_up_zero() {
        let z = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(0_i128).unwrap());
        let m = z.rescale::<12>();
        assert_eq!(i128::from(m.to_bits()), 0);
    }

    #[test]
    #[should_panic(expected = "scale-up overflow")]
    fn rescale_up_overflow_panics() {
        let big = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(i128::MAX).unwrap());
        // Going from scale 12 to scale 38 multiplies by 10^26, which
        // overflows for any non-tiny source.
        let _ = big.rescale::<38>();
    }

    // --- scale-down direction (default = HalfToEven) -------------------

    #[test]
    fn rescale_down_truncates_zero_remainder() {
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_i128).unwrap());
        let cents = micros.rescale::<2>();
        assert_eq!(i128::from(cents.to_bits()), 150);
    }

    #[test]
    fn rescale_down_half_to_even_rounds_to_even() {
        use decimal_scaled::RoundingMode;
        // Pin the mode so this test verifies HalfToEven specifically,
        // regardless of which `rounding-*` feature happens to be set.
        // 1.235000 at cents: tie -> 1.24 (4 is even)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_000_i128).unwrap());
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits()),
            124
        );

        // 1.225000 at cents: tie -> 1.22 (2 is even)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_225_000_i128).unwrap());
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits()),
            122
        );
    }

    #[test]
    fn rescale_down_non_half_goes_nearest() {
        // 1.234999 -> 1.23 (below half)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_234_999_i128).unwrap());
        assert_eq!(i128::from(micros.rescale::<2>().to_bits()), 123);
        // 1.235001 -> 1.24 (above half)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_001_i128).unwrap());
        assert_eq!(i128::from(micros.rescale::<2>().to_bits()), 124);
    }

    #[test]
    fn rescale_down_negative_half_to_even() {
        // -1.235000 -> -1.24 (tie, 4 is even — sign symmetric)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(-1_235_000_i128).unwrap());
        assert_eq!(i128::from(micros.rescale::<2>().to_bits()), -124);
    }

    // --- rescale_with mode coverage ------------------------------------

    #[test]
    fn rescale_with_each_mode_at_exact_half() {
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_000_i128).unwrap()); // 1.235000

        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits()),
            124
        );
        assert_eq!(
            i128::from(
                micros
                    .rescale_with::<2>(RoundingMode::HalfAwayFromZero)
                    .to_bits()
            ),
            124
        );
        assert_eq!(
            i128::from(
                micros
                    .rescale_with::<2>(RoundingMode::HalfTowardZero)
                    .to_bits()
            ),
            123
        );
        assert_eq!(i128::from(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits()), 123);
        assert_eq!(i128::from(micros.rescale_with::<2>(RoundingMode::Floor).to_bits()), 123);
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::Ceiling).to_bits()),
            124
        );
    }

    #[test]
    fn rescale_with_each_mode_at_exact_half_negative() {
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(-1_235_000_i128).unwrap()); // -1.235000

        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::HalfToEven).to_bits()),
            -124
        );
        assert_eq!(
            i128::from(
                micros
                    .rescale_with::<2>(RoundingMode::HalfAwayFromZero)
                    .to_bits()
            ),
            -124
        );
        assert_eq!(
            i128::from(
                micros
                    .rescale_with::<2>(RoundingMode::HalfTowardZero)
                    .to_bits()
            ),
            -123
        );
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits()),
            -123
        );
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::Floor).to_bits()),
            -124
        );
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::Ceiling).to_bits()),
            -123
        );
    }

    #[test]
    fn rescale_with_trunc_vs_floor_diverge_on_negative() {
        // -1.234500 (below the half-tie boundary in magnitude)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(-1_234_500_i128).unwrap());
        // Trunc rounds toward zero -> -1.23 (the half-tie isn't here; remainder is below half on this one)
        // Wait: divisor = 10^4 = 10000, abs_rem = 4500 which is < half (5000). So no rounding occurs.
        // Both Trunc and Floor return quotient = -123.
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::Trunc).to_bits()),
            -123
        );
        assert_eq!(
            i128::from(micros.rescale_with::<2>(RoundingMode::Floor).to_bits()),
            -124
        );
    }

    // --- equal scale identity ------------------------------------------

    #[test]
    fn rescale_same_scale_is_bit_identity() {
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(123_456_789_012_i128).unwrap());
        let same: D38s12 = v.rescale::<12>();
        assert_eq!(i128::from(same.to_bits()), 123_456_789_012);
    }

    #[test]
    fn rescale_with_same_scale_is_bit_identity_for_every_mode() {
        let v = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(123_456_789_012_i128).unwrap());
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            assert_eq!(i128::from(v.rescale_with::<12>(m).to_bits()), 123_456_789_012, "{m:?}");
        }
    }

    // --- rescale value correctness at runtime ---------------------------
    //
    // NB: with `Int<2>` storage, `rescale` is no longer a `const fn` (the
    // wide-integer divide path it uses is not const-evaluable), so this is
    // a runtime binding rather than the former `const` context. The value
    // contract is unchanged.

    #[test]
    fn rescale_value_matches_half_to_even() {
        let src: D38s6 = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_000_i128).unwrap());
        let dst: D38s2 = src.rescale::<2>();
        assert_eq!(i128::from(dst.to_bits()), 124);
    }
}

mod from_rescale_modes {
    //! Coverage for `macros/rescale.rs` — `rescale_with(mode)` on every
    //! decimal width × every rounding mode, plus the scale-up overflow panic
    //! path on D9 (the easiest tier to overflow at scale-up).

    use decimal_scaled::{D18, D38, RoundingMode};

    const ALL_MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    // `1.5050` rescaled 4→2 leaves a residual of exactly HALF (kept `1.50`, rest
    // `50`), so each mode's tie rule decides the last digit — the per-mode contract
    // these raw expectations pin (kept digits 150, the bump per mode):
    const TIE_POS: [i64; 6] = [150, 151, 150, 150, 150, 151];
    const TIE_NEG: [i64; 6] = [-150, -151, -150, -150, -151, -150];

    #[test]
    fn d18_rescale_with_all_modes() {

        let v = D18::<4>::from_bits(decimal_scaled::Int::<1>::from(15050_i64));
        let neg = D18::<4>::from_bits(decimal_scaled::Int::<1>::from(-15050_i64));
        for (i, m) in ALL_MODES.into_iter().enumerate() {
            let r: D18<2> = v.rescale_with::<2>(m);
            assert_eq!(r.to_bits(), decimal_scaled::Int::<1>::from(TIE_POS[i]), "{m:?} +tie");
            let r: D18<2> = neg.rescale_with::<2>(m);
            assert_eq!(r.to_bits(), decimal_scaled::Int::<1>::from(TIE_NEG[i]), "{m:?} -tie");
        }
        // Identity scale
        let r: D18<4> = v.rescale_with::<4>(RoundingMode::HalfToEven);
        assert_eq!(r.to_bits(), 15050);
    }

    #[test]
    fn d38_rescale_with_all_modes() {

        let v = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15050_i128).unwrap());
        let neg = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(-15050_i128).unwrap());
        for (i, m) in ALL_MODES.into_iter().enumerate() {
            let r: D38<2> = v.rescale_with::<2>(m);
            let want = decimal_scaled::Int::<2>::try_from(i128::from(TIE_POS[i])).unwrap();
            assert_eq!(r.to_bits(), want, "{m:?} +tie");
            let r: D38<2> = neg.rescale_with::<2>(m);
            let want = decimal_scaled::Int::<2>::try_from(i128::from(TIE_NEG[i])).unwrap();
            assert_eq!(r.to_bits(), want, "{m:?} -tie");
        }
        // Identity scale
        let r: D38<4> = v.rescale_with::<4>(RoundingMode::HalfToEven);
        assert_eq!(r.to_bits(), 15050);
    }

    // ─── Wide-tier rescale ─────────────────────────────────────────────────

    #[cfg(feature = "wide")]
    #[test]
    fn d76_rescale_with_all_modes() {
        use decimal_scaled::D76;

        let v: D76<4> = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15050_i128).unwrap()).into();
        let neg: D76<4> = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(-15050_i128).unwrap()).into();
        for (i, m) in ALL_MODES.into_iter().enumerate() {
            let r: D76<2> = v.rescale_with::<2>(m);
            let want: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(i128::from(TIE_POS[i])).unwrap()).into();
            assert_eq!(r, want, "{m:?} +tie");
            let r: D76<2> = neg.rescale_with::<2>(m);
            let want: D76<2> = D38::<2>::from_bits(decimal_scaled::Int::<2>::try_from(i128::from(TIE_NEG[i])).unwrap()).into();
            assert_eq!(r, want, "{m:?} -tie");
        }
        // Identity scale
        let r: D76<4> = v.rescale_with::<4>(RoundingMode::HalfToEven);
        assert_eq!(r, v);
        // with_scale path
        let _: D76<2> = v.with_scale::<2>();
    }

    #[cfg(feature = "wide")]
    #[test]
    #[should_panic(expected = "rescale: scale-up overflow")]
    fn d76_rescale_up_overflow_panics() {
        use decimal_scaled::D76;
        let v = D76::<0>::MAX;
        let _: D76<75> = v.rescale::<75>();
    }
}
