// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p` public-surface behaviour tests, moved from `src/algos/log1p/tests.rs`
//! (public surface only). Constructors rewritten from the crate-private
//! `Int::from_i128` / `D(..)` to the public `Int::try_from` + `from_bits`,
//! `Int::as_i128` reached through the public `From<Int<N>> for i128`, and
//! `consts::pow10::dispatch::<Int<N>>(k)` written as the public
//! `Int::<N>::try_from(10)?.pow(k)`.
//!
//! Two tests stayed behind, as inline `#[cfg(test)]` blocks in
//! `src/algos/log1p/mod.rs`, because each needs a crate-private item with no
//! public equivalent and copying it would weaken the test:
//! `both_kernels_agree_inside_the_overlap_band` drives the two `pub(crate)`
//! kernels directly, and `log1p_default_mode_siblings_agree` reads
//! `support::rounding::DEFAULT_ROUNDING_MODE`.

mod from_src_log1p {
    use decimal_scaled::{Int, RoundingMode, D};

    /// `Int::from_i128` is crate-private; same value via the public `TryFrom`.
    fn i1(v: i128) -> Int<1> {
        Int::<1>::try_from(v).unwrap()
    }
    fn i2(v: i128) -> Int<2> {
        Int::<2>::try_from(v).unwrap()
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    fn i3(v: i128) -> Int<3> {
        Int::<3>::try_from(v).unwrap()
    }

    /// `Int::as_i128` is crate-private; same accessor via the public
    /// `From<Int<N>> for i128` impls so the moved test bodies stay verbatim.
    trait AsI128 {
        fn as_i128(self) -> i128;
    }
    impl AsI128 for Int<1> {
        fn as_i128(self) -> i128 {
            i128::from(self)
        }
    }
    impl AsI128 for Int<2> {
        fn as_i128(self) -> i128 {
            i128::from(self)
        }
    }

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

    /// Scale the oracle anchors below are stated at.
    const S: u32 = 20;
    /// `1.0` at [`S`].
    const UNIT: i128 = 10_i128.pow(20);

    fn d38s20(raw: i128) -> D<Int<2>, 20> {
        D::<Int<2>, 20>::from_bits(i2(raw))
    }

    /// Correctly-rounded `log1p` at D38<20>, half-to-even, from the
    /// published decimal expansions of the underlying logarithms:
    ///
    /// - `ln 2   = 0.69314718055994530941 7232…` → `…942`
    /// - `ln 1.5 = 0.40546510810816438197 8013…` → `…198`
    /// - `ln 3   = 1.09861228866810969139 5245…` → `…140`
    /// - `ln 10  = 2.30258509299404568401 7991…` → `…402`
    ///
    /// The set straddles the matcher's region wall on purpose: `t = 0.5`
    /// and `t = 1` are inside the artanh region, `t = 2` and `t = 9` are
    /// outside it, `t = -0.5` sits exactly on the lower edge.
    #[test]
    fn log1p_matches_external_oracle_d38_s20() {
        const CASES: [(i128, i128); 5] = [
            // log1p(1) = ln 2 — artanh region, upper edge.
            (UNIT, 69_314_718_055_994_530_942),
            // log1p(0.5) = ln 1.5 — artanh region.
            (UNIT / 2, 40_546_510_810_816_438_198),
            // log1p(2) = ln 3 — WithLn region.
            (2 * UNIT, 109_861_228_866_810_969_140),
            // log1p(9) = ln 10 — WithLn region.
            (9 * UNIT, 230_258_509_299_404_568_402),
            // log1p(-0.5) = ln 0.5 = -ln 2 — artanh region, lower edge.
            (-UNIT / 2, -69_314_718_055_994_530_942),
        ];
        for &(t_raw, expected) in &CASES {
            assert_eq!(
                d38s20(t_raw)
                    .log1p_strict_with(RoundingMode::HalfToEven)
                    .to_bits()
                    .as_i128(),
                expected,
                "log1p D38<20> HalfToEven at raw={t_raw}"
            );
        }
    }

    /// `log1p(0) = 0` exactly, in every mode — the one rational value the
    /// function takes.
    #[test]
    fn log1p_of_zero_is_zero_in_every_mode() {
        for &mode in &MODES {
            assert_eq!(
                d38s20(0).log1p_strict_with(mode).to_bits().as_i128(),
                0,
                "log1p(0) mode={mode:?}"
            );
        }
    }

    /// The sub-ULP argument `t = 10^-20` at D38<20>: the true value
    /// `t - t²/2 + …` sits `5·10^-21` of an LSB BELOW one LSB, so the
    /// directed modes must split (`Floor`/`Trunc` down, `Ceiling` up) and
    /// the nearest modes must round to 1. This only resolves if the Ziv
    /// escalation actually walks past the base guard.
    #[test]
    fn log1p_sub_ulp_argument_splits_the_directed_modes() {
        const EXPECTED: [(RoundingMode, i128); 6] = [
            (RoundingMode::HalfToEven, 1),
            (RoundingMode::HalfAwayFromZero, 1),
            (RoundingMode::HalfTowardZero, 1),
            (RoundingMode::Trunc, 0),
            (RoundingMode::Floor, 0),
            (RoundingMode::Ceiling, 1),
        ];
        for &(mode, expected) in &EXPECTED {
            assert_eq!(
                d38s20(1).log1p_strict_with(mode).to_bits().as_i128(),
                expected,
                "log1p(1e-20) mode={mode:?}"
            );
        }
        // Mirror on the negative side: the magnitude exceeds one LSB by the
        // same `5·10^-21`, so `Floor` alone steps to -2.
        const EXPECTED_NEG: [(RoundingMode, i128); 6] = [
            (RoundingMode::HalfToEven, -1),
            (RoundingMode::HalfAwayFromZero, -1),
            (RoundingMode::HalfTowardZero, -1),
            (RoundingMode::Trunc, -1),
            (RoundingMode::Floor, -2),
            (RoundingMode::Ceiling, -1),
        ];
        for &(mode, expected) in &EXPECTED_NEG {
            assert_eq!(
                d38s20(-1).log1p_strict_with(mode).to_bits().as_i128(),
                expected,
                "log1p(-1e-20) mode={mode:?}"
            );
        }
    }

    /// The parity claim: `log1p(t)` and `ln(1 + t)` are the SAME correctly
    /// rounded value at a shared scale. Spans both matcher regions and both
    /// sides of each region edge, including the near-domain-edge arguments
    /// where the artanh series would not converge (so a mis-placed region
    /// wall fails here).
    #[test]
    fn log1p_equals_ln_of_one_plus_t_d38_s20() {
        const TS: [i128; 17] = [
            0,
            1,
            -1,
            UNIT / 1_000,
            -UNIT / 1_000,
            UNIT / 2,      // lower region edge (t = -1/2 mirrored)
            -UNIT / 2,     // lower region edge
            -UNIT / 2 - 1, // just outside the lower edge
            UNIT,          // upper region edge (t = 1)
            UNIT + 1,      // just outside the upper edge
            2 * UNIT,
            9 * UNIT,
            1_000 * UNIT,
            -(UNIT - 1), // t = -0.999…9, |u| → 1
            -(UNIT / 10),
            -(9 * UNIT / 10),
            -(99 * UNIT / 100),
        ];
        for &t in &TS {
            for &mode in &MODES {
                assert_eq!(
                    d38s20(t).log1p_strict_with(mode).to_bits().as_i128(),
                    d38s20(t + UNIT).ln_strict_with(mode).to_bits().as_i128(),
                    "log1p != ln(1+t) at t_raw={t} mode={mode:?}"
                );
            }
        }
    }

    /// `_approx` at a guard well past the strict one reproduces the strict
    /// result on well-conditioned inputs — the `*_approx` contract (same
    /// value, caller-chosen working width). Spans both regions.
    #[test]
    fn log1p_approx_at_a_deep_guard_matches_strict() {
        const TS: [i128; 5] = [UNIT / 2, UNIT, 2 * UNIT, 9 * UNIT, -UNIT / 2];
        for &t in &TS {
            for &mode in &MODES {
                assert_eq!(
                    d38s20(t).log1p_approx_with(60, mode).to_bits().as_i128(),
                    d38s20(t).log1p_strict_with(mode).to_bits().as_i128(),
                    "log1p_approx(60) != log1p_strict at t_raw={t} mode={mode:?}"
                );
            }
        }
    }

    /// Domain: `t = -1` is out of domain (`1 + t == 0`, exactly `ln`'s
    /// non-positive-argument case) and panics.
    #[test]
    #[should_panic(expected = "log1p: argument must be greater than -1")]
    fn log1p_at_minus_one_panics() {
        let _ = d38s20(-UNIT).log1p_strict();
    }

    /// Domain: `t < -1` panics.
    #[test]
    #[should_panic(expected = "log1p: argument must be greater than -1")]
    fn log1p_below_minus_one_panics() {
        let _ = d38s20(-2 * UNIT).log1p_strict();
    }

    /// The `_approx` surface carries the same domain guard.
    #[test]
    #[should_panic(expected = "log1p: argument must be greater than -1")]
    fn log1p_approx_below_minus_one_panics() {
        let _ = d38s20(-2 * UNIT).log1p_approx(45);
    }

    /// D18 (`Int<1>`) routes through the same policy at its own storage
    /// width and agrees with `ln(1 + t)` there too.
    #[test]
    fn log1p_equals_ln_of_one_plus_t_d18_s9() {
        const UNIT9: i128 = 1_000_000_000;
        for &t in &[
            0_i128,
            1,
            -1,
            UNIT9 / 2,
            -UNIT9 / 2,
            UNIT9,
            3 * UNIT9,
            -(UNIT9 - 1),
        ] {
            for &mode in &MODES {
                let x = D::<Int<1>, 9>::from_bits(i1(t));
                let y = D::<Int<1>, 9>::from_bits(i1(t + UNIT9));
                assert_eq!(
                    x.log1p_strict_with(mode).to_bits().as_i128(),
                    y.ln_strict_with(mode).to_bits().as_i128(),
                    "D18 log1p != ln(1+t) at t_raw={t} mode={mode:?}"
                );
            }
        }
    }

    /// The wide arm runs the SAME generic kernels at the tier's own work
    /// integer; it must agree with `ln(1 + t)` at that width as well.
    #[test]
    #[cfg(any(feature = "d57", feature = "wide"))]
    fn log1p_equals_ln_of_one_plus_t_d57_s20() {
        let unit = i3(UNIT);
        for &t in &[
            0_i128,
            1,
            -1,
            UNIT / 2,
            -UNIT / 2,
            UNIT,
            UNIT + 1,
            5 * UNIT,
            -(UNIT - 1),
        ] {
            for &mode in &MODES {
                let x = D::<Int<3>, 20>::from_bits(i3(t));
                let y = D::<Int<3>, 20>::from_bits(i3(t) + unit);
                assert_eq!(
                    x.log1p_strict_with(mode).to_bits(),
                    y.ln_strict_with(mode).to_bits(),
                    "D57 log1p != ln(1+t) at t_raw={t} mode={mode:?}"
                );
            }
        }
    }

    /// The wide oracle anchors, at a different width than the D38 set — the
    /// same external expansions, so the wide work integer is checked against
    /// the literature and not against the narrow arm.
    #[test]
    #[cfg(any(feature = "d57", feature = "wide"))]
    fn log1p_matches_external_oracle_d57_s20() {
        const CASES: [(i128, i128); 4] = [
            (UNIT, 69_314_718_055_994_530_942),
            (UNIT / 2, 40_546_510_810_816_438_198),
            (2 * UNIT, 109_861_228_866_810_969_140),
            (9 * UNIT, 230_258_509_299_404_568_402),
        ];
        for &(t_raw, expected) in &CASES {
            assert_eq!(
                D::<Int<3>, 20>::from_bits(i3(t_raw))
                    .log1p_strict_with(RoundingMode::HalfToEven)
                    .to_bits(),
                i3(expected),
                "log1p D57<20> HalfToEven at raw={t_raw}"
            );
        }
    }

    /// The region wall at a LARGE working scale — where the artanh series'
    /// term count (`≈2.1·w` inside the wall) and the 20 000-iteration
    /// series cap are furthest apart, and where a mis-placed wall would
    /// show as a truncated series rather than a slow one. D307<150> runs at
    /// `w ≈ 160`; the argument set straddles both region edges and reaches
    /// the domain edge `t → -1`.
    #[test]
    #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
    fn log1p_equals_ln_of_one_plus_t_d307_s150() {
        let unit = Int::<16>::try_from(10_i128).unwrap().pow(150);
        let lsb = Int::<16>::try_from(1_i128).unwrap();
        let zero = Int::<16>::try_from(0_i128).unwrap();
        let cases = [
            zero,
            lsb,
            -lsb,
            unit >> 1,
            -(unit >> 1),
            (unit >> 1) + lsb,
            unit,
            unit + lsb,
            unit + unit,
            -(unit - lsb),
        ];
        for &t in &cases {
            for &mode in &MODES {
                let x = D::<Int<16>, 150>::from_bits(t);
                let y = D::<Int<16>, 150>::from_bits(t + unit);
                assert_eq!(
                    x.log1p_strict_with(mode).to_bits(),
                    y.ln_strict_with(mode).to_bits(),
                    "D307<150> log1p != ln(1+t) at t={t:?} mode={mode:?}"
                );
            }
        }
    }

    /// The widest shipped tier at its top scale — D1232<1231>, `w ≈ 1241`,
    /// the largest working scale the crate reaches. Inside the region wall
    /// the artanh series needs `≈2.1·w ≈ 2 600` terms here, its worst case;
    /// this is the case the wall's series-cap headroom argument rests on,
    /// so both region edges are checked at the extreme. Kept to a small
    /// argument set because each call is a full-width evaluation.
    #[test]
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn log1p_equals_ln_of_one_plus_t_d1232_s1231() {
        let ten = Int::<64>::try_from(10_i128).unwrap();
        let unit = ten.pow(1231);
        let lsb = Int::<64>::try_from(1_i128).unwrap();
        // `t → -1` is bounded by the OUTPUT range, not the domain: at this
        // scale `|log1p(t)| < ~100` is all the storage holds, so the deepest
        // representable approach is `1 + t = 10^-20` (`log1p ≈ -46.05`).
        // Anything closer to -1 legitimately overflows and panics, exactly as
        // `ln` of the same argument does.
        let deepest_neg = -(unit - ten.pow(1211));
        let cases = [
            lsb,          // tiny t — artanh region, the sub-resolution case
            -lsb,         // its negative mirror
            unit,         // upper region edge
            unit + lsb,   // just outside it
            -(unit >> 1), // lower region edge
            deepest_neg,  // as close to the domain edge as the range allows
        ];
        for &t in &cases {
            for &mode in &[RoundingMode::HalfToEven, RoundingMode::Floor] {
                let x = D::<Int<64>, 1231>::from_bits(t);
                let y = D::<Int<64>, 1231>::from_bits(t + unit);
                assert_eq!(
                    x.log1p_strict_with(mode).to_bits(),
                    y.ln_strict_with(mode).to_bits(),
                    "D1232<1231> log1p != ln(1+t) at mode={mode:?}"
                );
            }
        }
    }

    /// The directed modes must SPLIT on the arguments whose deciding term the
    /// working scale cannot represent — an oracle-free contract.
    ///
    /// `log1p(t) = ln(1 + t)` is transcendental at every algebraic `t != 0`
    /// (Lindemann-Weierstrass), so it never lands on a storage grid line.
    /// `Floor` and `Ceiling` therefore ALWAYS straddle it by exactly one ULP,
    /// and `Trunc` is whichever of the two faces zero. Needing no oracle, this
    /// catches the precise failure these arguments used to show: the kernel read
    /// its residual as an exact zero, took the value for representable, and
    /// returned the SAME answer in all eight modes.
    ///
    /// The arguments are the family that forces it. At `D462<461>`,
    /// `t = c·10^-m` makes the leading terms of `t - t²/2 + t³/3 - …` exact
    /// whole ULP multiples, so the round is decided by the first term that is
    /// not — tens of digits below the guard, far past what the Ziv walker can
    /// reach at this scale. Only the kernel's own tail sign can settle it, and
    /// settling it here needs the sub-LSB imprecision measured rather than read
    /// off the rounding signs, since those genuinely oppose at the positive
    /// arguments.
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    #[test]
    fn directed_modes_straddle_when_the_deciding_term_is_out_of_reach() {
        const SC: u32 = 461;
        // `99·10^-m` and `10^-m`, at the exponents where the deciding term
        // falls past the walker's reach.
        const CASES: [(i128, u32); 6] = [
            (99, 88),
            (99, 92),
            (99, 127),
            (99, 147),
            (1, 75),
            (1, 76),
        ];
        let ten = Int::<24>::try_from(10_i128).unwrap();
        let one = Int::<24>::try_from(1_i128).unwrap();
        let zero = Int::<24>::try_from(0_i128).unwrap();
        for &(lead, expo) in &CASES {
            for sign in [1_i128, -1] {
                let raw = Int::<24>::try_from(sign * lead).unwrap() * ten.pow(SC - expo);
                let at = |m| D::<Int<24>, SC>::from_bits(raw).log1p_strict_with(m).to_bits();
                let (floor, ceiling, trunc) = (
                    at(RoundingMode::Floor),
                    at(RoundingMode::Ceiling),
                    at(RoundingMode::Trunc),
                );
                assert_eq!(
                    ceiling - floor,
                    one,
                    "log1p({sign}·{lead}e-{expo}) at D462<{SC}>: Ceiling and Floor \
                     must straddle a value that cannot be on the grid"
                );
                let toward_zero = if floor < zero { ceiling } else { floor };
                assert_eq!(
                    trunc, toward_zero,
                    "log1p({sign}·{lead}e-{expo}) at D462<{SC}>: Trunc must equal \
                     the neighbour facing zero"
                );
            }
        }
    }

    /// `S` is the scale every anchor above is stated at; assert `UNIT` is
    /// consistent with it so a future scale change cannot silently
    /// invalidate the baked expectations.
    #[test]
    fn oracle_unit_matches_the_stated_scale() {
        assert_eq!(UNIT, 10_i128.pow(S), "UNIT must be 10^S");
    }
}
