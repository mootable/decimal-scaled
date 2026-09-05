//! `quantize` / `quantize_with` / `with_scale` semantics: direction, tie
//! handling per mode, identity, and the scale-up overflow panic.
//!
//! The deprecated `rescale` / `rescale_with` aliases (removed in 0.6.0)
//! are covered by the main crate's `tests/deprecated_aliases.rs`.

mod from_quantize {
    //! `D38::quantize` / `quantize_with` integration tests. Moved out of
    //! `src/types/quantize.rs` so that file carries only macro invocations.
    //!
    //! Several tests below use the plain `quantize::<N>()` form, whose
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

    use decimal_scaled::{D38s12, D38s2, D38s6, RoundingMode};

    // --- with_scale alias ----------------------------------------------

    #[test]
    fn with_scale_matches_rescale() {
        // Native tier.
        let a = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap());
        assert_eq!(
            i128::from(a.with_scale::<6>().to_bits()),
            i128::from(a.quantize::<6>().to_bits())
        );
        assert_eq!(
            i128::from(a.with_scale::<2>().to_bits()),
            i128::from(a.to_bits())
        );

        // The builder-style name is the only difference; semantics are
        // bit-identical to rescale.
        let b =
            D38s12::from_bits(decimal_scaled::Int::<2>::try_from(12_345_678_901_234_i128).unwrap());
        assert_eq!(
            i128::from(b.with_scale::<6>().to_bits()),
            i128::from(b.quantize::<6>().to_bits())
        );
    }

    // --- scale-up direction --------------------------------------------

    #[test]
    fn quantize_up_appends_zeros() {
        let cents = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(150_i128).unwrap());
        let micros = cents.quantize::<6>();
        assert_eq!(i128::from(micros.to_bits()), 1_500_000);
    }

    #[test]
    fn quantize_up_negative() {
        let cents = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(-150_i128).unwrap());
        let micros = cents.quantize::<6>();
        assert_eq!(i128::from(micros.to_bits()), -1_500_000);
    }

    #[test]
    fn quantize_up_zero() {
        let z = D38s2::from_bits(decimal_scaled::Int::<2>::try_from(0_i128).unwrap());
        let m = z.quantize::<12>();
        assert_eq!(i128::from(m.to_bits()), 0);
    }

    #[test]
    #[should_panic(expected = "scale-up overflow")]
    fn quantize_up_overflow_panics() {
        let big = D38s12::from_bits(decimal_scaled::Int::<2>::try_from(i128::MAX).unwrap());
        // Going from scale 12 to scale 38 multiplies by 10^26, which
        // overflows for any non-tiny source.
        let _ = big.quantize::<38>();
    }

    // --- scale-down direction (default = HalfToEven) -------------------

    #[test]
    fn quantize_down_truncates_zero_remainder() {
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_500_000_i128).unwrap());
        let cents = micros.quantize::<2>();
        assert_eq!(i128::from(cents.to_bits()), 150);
    }

    #[test]
    fn quantize_down_half_to_even_rounds_to_even() {
        use decimal_scaled::RoundingMode;
        // Pin the mode so this test verifies HalfToEven specifically,
        // regardless of which `rounding-*` feature happens to be set.
        // 1.235000 at cents: tie -> 1.24 (4 is even)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_000_i128).unwrap());
        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfToEven)
                    .to_bits()
            ),
            124
        );

        // 1.225000 at cents: tie -> 1.22 (2 is even)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_225_000_i128).unwrap());
        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfToEven)
                    .to_bits()
            ),
            122
        );
    }

    #[test]
    fn quantize_down_non_half_goes_nearest() {
        // 1.234999 -> 1.23 (below half)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_234_999_i128).unwrap());
        assert_eq!(i128::from(micros.quantize::<2>().to_bits()), 123);
        // 1.235001 -> 1.24 (above half)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_001_i128).unwrap());
        assert_eq!(i128::from(micros.quantize::<2>().to_bits()), 124);
    }

    #[test]
    fn quantize_down_negative_half_to_even() {
        // -1.235000 -> -1.24 (tie, 4 is even — sign symmetric)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(-1_235_000_i128).unwrap());
        assert_eq!(i128::from(micros.quantize::<2>().to_bits()), -124);
    }

    // --- quantize_with mode coverage -----------------------------------

    #[test]
    fn quantize_with_each_mode_at_exact_half() {
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_000_i128).unwrap()); // 1.235000

        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfToEven)
                    .to_bits()
            ),
            124
        );
        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfAwayFromZero)
                    .to_bits()
            ),
            124
        );
        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfTowardZero)
                    .to_bits()
            ),
            123
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Trunc).to_bits()),
            123
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Floor).to_bits()),
            123
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Ceiling).to_bits()),
            124
        );
    }

    #[test]
    fn quantize_with_each_mode_at_exact_half_negative() {
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(-1_235_000_i128).unwrap()); // -1.235000

        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfToEven)
                    .to_bits()
            ),
            -124
        );
        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfAwayFromZero)
                    .to_bits()
            ),
            -124
        );
        assert_eq!(
            i128::from(
                micros
                    .quantize_with::<2>(RoundingMode::HalfTowardZero)
                    .to_bits()
            ),
            -123
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Trunc).to_bits()),
            -123
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Floor).to_bits()),
            -124
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Ceiling).to_bits()),
            -123
        );
    }

    #[test]
    fn quantize_with_trunc_vs_floor_diverge_on_negative() {
        // -1.234500 (below the half-tie boundary in magnitude)
        let micros = D38s6::from_bits(decimal_scaled::Int::<2>::try_from(-1_234_500_i128).unwrap());
        // Trunc rounds toward zero -> -1.23 (the half-tie isn't here; remainder is below half on this one)
        // Wait: divisor = 10^4 = 10000, abs_rem = 4500 which is < half (5000). So no rounding occurs.
        // Both Trunc and Floor return quotient = -123.
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Trunc).to_bits()),
            -123
        );
        assert_eq!(
            i128::from(micros.quantize_with::<2>(RoundingMode::Floor).to_bits()),
            -124
        );
    }

    // --- equal scale identity ------------------------------------------

    #[test]
    fn quantize_same_scale_is_bit_identity() {
        let v =
            D38s12::from_bits(decimal_scaled::Int::<2>::try_from(123_456_789_012_i128).unwrap());
        let same: D38s12 = v.quantize::<12>();
        assert_eq!(i128::from(same.to_bits()), 123_456_789_012);
    }

    #[test]
    fn quantize_with_same_scale_is_bit_identity_for_every_mode() {
        let v =
            D38s12::from_bits(decimal_scaled::Int::<2>::try_from(123_456_789_012_i128).unwrap());
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
            RoundingMode::AwayFromZero,
            RoundingMode::ZeroFiveUp,
        ] {
            assert_eq!(
                i128::from(v.quantize_with::<12>(m).to_bits()),
                123_456_789_012,
                "{m:?}"
            );
        }
    }

    // --- rescale value correctness at runtime ---------------------------
    //
    // NB: with `Int<2>` storage, `rescale` is no longer a `const fn` (the
    // wide-integer divide path it uses is not const-evaluable), so this is
    // a runtime binding rather than the former `const` context. The value
    // contract is unchanged.

    #[test]
    fn quantize_value_matches_half_to_even() {
        let src: D38s6 =
            D38s6::from_bits(decimal_scaled::Int::<2>::try_from(1_235_000_i128).unwrap());
        let dst: D38s2 = src.quantize::<2>();
        assert_eq!(i128::from(dst.to_bits()), 124);
    }
}

mod from_rescale_modes {
    //! Coverage for `macros/quantize.rs` — `quantize_with(mode)` on every
    //! decimal width × every rounding mode, plus the scale-up overflow panic
    //! path on D9 (the easiest tier to overflow at scale-up).

    use decimal_scaled::{RoundingMode, D18, D38};

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    // `1.5050` rescaled 4→2 leaves a residual of exactly HALF (kept `1.50`, rest
    // `50`), so each mode's tie rule decides the last digit — the per-mode contract
    // these raw expectations pin (kept digits 150, the bump per mode).
    //
    // The two directed GDA modes read the residual, not the tie: `AwayFromZero`
    // bumps on any discard, and `ZeroFiveUp` bumps because the kept digit is `0`
    // — one of its two pivot digits. Both step the MAGNITUDE up, so both give
    // 151 on the positive tie and -151 on the negative one.
    const TIE_POS: [i64; 8] = [150, 151, 150, 150, 150, 151, 151, 151];
    const TIE_NEG: [i64; 8] = [-150, -151, -150, -150, -151, -150, -151, -151];

    #[test]
    fn d18_quantize_with_all_modes() {
        let v = D18::<4>::from_bits(decimal_scaled::Int::<1>::from(15050_i64));
        let neg = D18::<4>::from_bits(decimal_scaled::Int::<1>::from(-15050_i64));
        for (i, m) in ALL_MODES.into_iter().enumerate() {
            let r: D18<2> = v.quantize_with::<2>(m);
            assert_eq!(
                r.to_bits(),
                decimal_scaled::Int::<1>::from(TIE_POS[i]),
                "{m:?} +tie"
            );
            let r: D18<2> = neg.quantize_with::<2>(m);
            assert_eq!(
                r.to_bits(),
                decimal_scaled::Int::<1>::from(TIE_NEG[i]),
                "{m:?} -tie"
            );
        }
        // Identity scale
        let r: D18<4> = v.quantize_with::<4>(RoundingMode::HalfToEven);
        assert_eq!(r.to_bits(), 15050);
    }

    #[test]
    fn d38_quantize_with_all_modes() {
        let v = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15050_i128).unwrap());
        let neg = D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(-15050_i128).unwrap());
        for (i, m) in ALL_MODES.into_iter().enumerate() {
            let r: D38<2> = v.quantize_with::<2>(m);
            let want = decimal_scaled::Int::<2>::try_from(i128::from(TIE_POS[i])).unwrap();
            assert_eq!(r.to_bits(), want, "{m:?} +tie");
            let r: D38<2> = neg.quantize_with::<2>(m);
            let want = decimal_scaled::Int::<2>::try_from(i128::from(TIE_NEG[i])).unwrap();
            assert_eq!(r.to_bits(), want, "{m:?} -tie");
        }
        // Identity scale
        let r: D38<4> = v.quantize_with::<4>(RoundingMode::HalfToEven);
        assert_eq!(r.to_bits(), 15050);
    }

    // ─── Wide-tier rescale ─────────────────────────────────────────────────

    #[cfg(feature = "wide")]
    #[test]
    fn d76_quantize_with_all_modes() {
        use decimal_scaled::D76;

        let v: D76<4> =
            D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(15050_i128).unwrap()).into();
        let neg: D76<4> =
            D38::<4>::from_bits(decimal_scaled::Int::<2>::try_from(-15050_i128).unwrap()).into();
        for (i, m) in ALL_MODES.into_iter().enumerate() {
            let r: D76<2> = v.quantize_with::<2>(m);
            let want: D76<2> = D38::<2>::from_bits(
                decimal_scaled::Int::<2>::try_from(i128::from(TIE_POS[i])).unwrap(),
            )
            .into();
            assert_eq!(r, want, "{m:?} +tie");
            let r: D76<2> = neg.quantize_with::<2>(m);
            let want: D76<2> = D38::<2>::from_bits(
                decimal_scaled::Int::<2>::try_from(i128::from(TIE_NEG[i])).unwrap(),
            )
            .into();
            assert_eq!(r, want, "{m:?} -tie");
        }
        // Identity scale
        let r: D76<4> = v.quantize_with::<4>(RoundingMode::HalfToEven);
        assert_eq!(r, v);
        // with_scale path
        let _: D76<2> = v.with_scale::<2>();
    }

    #[cfg(feature = "wide")]
    #[test]
    #[should_panic(expected = "quantize: scale-up overflow")]
    fn d76_rescale_up_overflow_panics() {
        use decimal_scaled::D76;
        let v = D76::<0>::MAX;
        let _: D76<75> = v.quantize::<75>();
    }
}

mod wide_matcher_differential {
    //! Wide-tier `quantize` scale-down, differenced against an independent
    //! implementation of the same operation.
    //!
    //! # Why this module exists
    //!
    //! `quantize_with`'s scale-down routes its `/ 10^shift` through the
    //! crate's rescale matcher, which picks between three kernels on
    //! `(shift, significant width)`. Some of those cells were unreachable
    //! from `quantize` at any width the rest of this file covers: the
    //! widest tier tested elsewhere here is D76, whose storage is four u64
    //! limbs, and the matcher's baked-reciprocal arm engages only from
    //! twenty-four limbs upward with `shift` in `200..=1850`. Without these
    //! tests a routing change at D462 and wider is invisible.
    //!
    //! # The oracle
    //!
    //! Not a hand-computed expected value — at these widths that would be a
    //! second implementation with no reviewer. Each result is instead
    //! differenced against `convert_from_with` at the *same* width, which
    //! reaches the same operation by a wholly separate route: a typed
    //! `div_rem` against `10^shift` in the integer layer, never touching
    //! the rescale matcher. The two share only the mode decider, so they
    //! disagree if the quotient, the remainder comparison, the last decimal
    //! digit, or the sign handling differs on either side.
    //!
    //! # Both liveness guards matter
    //!
    //! A differential that only ever hits the zero-remainder early return
    //! agrees trivially and proves nothing, so each test asserts that the
    //! rounding actually ran:
    //!
    //! - the dense case asserts `Trunc` and `AwayFromZero` disagree, which
    //!   holds exactly when the discarded digits are non-zero;
    //! - the tie case asserts `HalfTowardZero` and `HalfAwayFromZero`
    //!   disagree, which holds exactly when the remainder is the half —
    //!   i.e. that the tie the test was built to create really is one.
    //!
    //! Both inputs are built near the storage maximum on purpose: the
    //! matcher keys on the significant limb length after leading-zero
    //! trimming, so a small value at a wide tier routes as a narrow one and
    //! would silently miss the band. Each test asserts its input stayed
    //! above `MAX >> 3` so that property cannot rot.

    use decimal_scaled::{Int, RoundingMode};

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// Emits the dense pair and the exact-tie pair for one
    /// `(tier, limbs, source scale, target scale)` cell.
    macro_rules! wide_quantize_differential {
        ($dense:ident, $tie:ident, $ty:ident, $n:literal, $src:literal, $tgt:literal) => {
            /// Generic (non-tie) remainders across all eight modes, both
            /// signs, at three near-maximal magnitudes.
            #[test]
            fn $dense() {
                type Src = decimal_scaled::$ty<$src>;
                type Dst = decimal_scaled::$ty<$tgt>;

                let one = Int::<$n>::ONE;
                let three = one + one + one;
                let seven = three + three + one;
                let width_floor = Int::<$n>::MAX >> 3u32;

                for mag in [Int::<$n>::MAX, Int::<$n>::MAX / three, Int::<$n>::MAX / seven] {
                    assert!(
                        mag >= width_floor,
                        "{}: input dropped below the width floor, so it no longer \
                         routes as a full-width value",
                        stringify!($dense)
                    );
                    for raw in [mag, Int::<$n>::ZERO - mag] {
                        let x = Src::from_bits(raw);

                        let trunc = x.quantize_with::<$tgt>(RoundingMode::Trunc);
                        let away = x.quantize_with::<$tgt>(RoundingMode::AwayFromZero);
                        assert_ne!(
                            trunc.to_bits(),
                            away.to_bits(),
                            "{}: discarded digits were zero, so the rounding path \
                             never ran and this proves nothing",
                            stringify!($dense)
                        );

                        for &mode in &ALL_MODES {
                            let got = x.quantize_with::<$tgt>(mode);
                            let want = Dst::convert_from_with(x, mode)
                                .expect("a same-width scale-down cannot overflow");
                            assert_eq!(
                                got.to_bits(),
                                want.to_bits(),
                                "{}: quantize disagrees with convert_from_with under {:?}",
                                stringify!($dense),
                                mode
                            );
                        }
                    }
                }
            }

            /// The exact half-way remainder — the one input that separates
            /// the three nearest modes — built by construction rather than
            /// searched for.
            #[test]
            fn $tie() {
                type Src = decimal_scaled::$ty<$src>;
                type Dst = decimal_scaled::$ty<$tgt>;

                let one = Int::<$n>::ONE;
                let pow = Int::<$n>::TEN.pow(($src - $tgt) as u32);
                // `10^k` is even for every `k >= 1`, so the half is exact.
                let half = pow >> 1u32;
                // The largest quotient leaving room for the half digit and
                // for a bump away from zero.
                let quotient = Int::<$n>::MAX / pow - one;
                let raw = quotient * pow + half;

                assert!(
                    raw >= Int::<$n>::MAX >> 3u32,
                    "{}: input dropped below the width floor, so it no longer \
                     routes as a full-width value",
                    stringify!($tie)
                );

                for signed in [raw, Int::<$n>::ZERO - raw] {
                    let x = Src::from_bits(signed);

                    let toward = x.quantize_with::<$tgt>(RoundingMode::HalfTowardZero);
                    let away = x.quantize_with::<$tgt>(RoundingMode::HalfAwayFromZero);
                    assert_ne!(
                        toward.to_bits(),
                        away.to_bits(),
                        "{}: the remainder was not the exact half, so the tie-break \
                         branch never ran",
                        stringify!($tie)
                    );

                    for &mode in &ALL_MODES {
                        let got = x.quantize_with::<$tgt>(mode);
                        let want = Dst::convert_from_with(x, mode)
                            .expect("a same-width scale-down cannot overflow");
                        assert_eq!(
                            got.to_bits(),
                            want.to_bits(),
                            "{}: quantize disagrees with convert_from_with under {:?}",
                            stringify!($tie),
                            mode
                        );
                    }
                }
            }
        };
    }

    // D462 — twenty-four u64 limbs, the matcher's lower width bound. The
    // three cells walk the shift axis across all three kernels: 30 is the
    // single-chunk band, 100 the chained band below the baked-reciprocal
    // threshold, 361 inside it.
    #[cfg(feature = "d462")]
    wide_quantize_differential!(d462_shift30_dense, d462_shift30_tie, D462, 24, 130, 100);
    #[cfg(feature = "d462")]
    wide_quantize_differential!(d462_shift100_dense, d462_shift100_tie, D462, 24, 200, 100);
    #[cfg(feature = "d462")]
    wide_quantize_differential!(d462_shift361_dense, d462_shift361_tie, D462, 24, 461, 100);

    // The wider tiers inside the baked-reciprocal band.
    #[cfg(feature = "d616")]
    wide_quantize_differential!(d616_shift500_dense, d616_shift500_tie, D616, 32, 600, 100);
    #[cfg(feature = "d924")]
    wide_quantize_differential!(d924_shift800_dense, d924_shift800_tie, D924, 48, 900, 100);
    #[cfg(feature = "d1232")]
    wide_quantize_differential!(
        d1232_shift1100_dense,
        d1232_shift1100_tie,
        D1232,
        64,
        1200,
        100
    );
}
