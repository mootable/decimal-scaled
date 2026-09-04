// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `expm1` public-surface behaviour tests, moved from `src/algos/expm1/tests.rs`
//! (public surface only). Constructors rewritten from the crate-private
//! `Int::from_i128` / `D(..)` to the public `Int::try_from` + `from_bits`, and
//! `Int::as_i128` reached through the public `From<Int<N>> for i128`.

mod from_src_expm1 {
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

    /// Correctly-rounded `expm1` at D38<20>, half-to-even, from the published
    /// decimal expansions:
    ///
    /// - `e       = 2.71828182845904523536 028747…` → `e − 1` → `…536`
    /// - `e^0.5   = 1.64872127070012814684 865078…` → `…685`
    /// - `1/e     = 0.36787944117144232159 552377…` → `1/e − 1` → `…840`
    /// - `e^2     = 7.38905609893065022723 042746…` → `e² − 1` → `…723`
    ///
    /// The set straddles the matcher's region wall on purpose: `x = 1`, `0.5`
    /// and `-1` are inside the direct-series region `|x| <= 1` (with `±1`
    /// exactly on its edges), `x = 2` is outside it and lands on the `ViaExp`
    /// arm.
    #[test]
    fn expm1_matches_external_oracle_d38_s20() {
        const CASES: [(i128, i128); 4] = [
            // expm1(1) = e − 1 — series region, upper edge.
            (UNIT, 171_828_182_845_904_523_536),
            // expm1(0.5) = e^0.5 − 1 — series region.
            (UNIT / 2, 64_872_127_070_012_814_685),
            // expm1(-1) = 1/e − 1 — series region, lower edge.
            (-UNIT, -63_212_055_882_855_767_840),
            // expm1(2) = e² − 1 — ViaExp region.
            (2 * UNIT, 638_905_609_893_065_022_723),
        ];
        for (arg, want) in CASES {
            let got = d38s20(arg).expm1_strict();
            assert_eq!(
                got.to_bits().as_i128(),
                want,
                "expm1({arg}e-{S}) at D38<{S}>"
            );
        }
    }

    /// `expm1(0) = 0` exactly, in every mode — the one algebraically exact case
    /// (for algebraic `x != 0`, `e^x - 1` is transcendental, so it never lands
    /// on a storage grid line).
    #[test]
    fn expm1_zero_is_exact_in_every_mode() {
        for mode in MODES {
            assert_eq!(
                d38s20(0).expm1_strict_with(mode).to_bits().as_i128(),
                0,
                "expm1(0) mode {mode:?}"
            );
        }
    }

    /// EDGE 1 — the sub-resolution band near zero.
    ///
    /// `e^x > 1 + x` strictly (convexity), so `expm1(x) > x` for every `x != 0`,
    /// with the excess `≈ x²/2` far below the storage scale at `|x| = 1 ULP`
    /// (`10^-40` against a `10^-20` grid). The correctly-rounded answers are
    /// therefore fixed by the SIGN of that excess alone, and they are NOT
    /// symmetric between the two signs — which is exactly what
    /// `adjust_near_zero` encodes: `Ceiling` steps up for both signs, `Trunc`
    /// only for `x < 0` (toward zero is UP for a negative value, DOWN for a
    /// positive one).
    #[test]
    fn expm1_one_ulp_sub_resolution_band() {
        // x = +1 ULP: true value in (1e-20, 2e-20), hard against the lower end.
        for (mode, want) in [
            (RoundingMode::HalfToEven, 1),
            (RoundingMode::HalfAwayFromZero, 1),
            (RoundingMode::HalfTowardZero, 1),
            (RoundingMode::Floor, 1),
            (RoundingMode::Trunc, 1),
            (RoundingMode::Ceiling, 2),
        ] {
            assert_eq!(
                d38s20(1).expm1_strict_with(mode).to_bits().as_i128(),
                want,
                "expm1(+1 ULP) mode {mode:?}"
            );
        }
        // x = -1 ULP: true value in (-1e-20, 0), hard against the lower end.
        // Trunc rounds TOWARD ZERO, which for a negative value is UP.
        for (mode, want) in [
            (RoundingMode::HalfToEven, -1),
            (RoundingMode::HalfAwayFromZero, -1),
            (RoundingMode::HalfTowardZero, -1),
            (RoundingMode::Floor, -1),
            (RoundingMode::Trunc, 0),
            (RoundingMode::Ceiling, 0),
        ] {
            assert_eq!(
                d38s20(-1).expm1_strict_with(mode).to_bits().as_i128(),
                want,
                "expm1(-1 ULP) mode {mode:?}"
            );
        }
    }

    /// EDGE 2 — the deep-negative tail must land JUST ABOVE `-1`, never ON it.
    ///
    /// For `x` deep negative, `expm1(x) = -1 + e^x` with `e^x` below the working
    /// resolution: the true value is strictly ABOVE `-1`, i.e. its MAGNITUDE is
    /// strictly BELOW one. The kernels return the working-scale representative
    /// `1 - 10^w` (`expm1_support::just_above_minus_one`) rather than a bare
    /// `-10^w` precisely so this comes out right — a bare `-10^w` leaves a zero
    /// residual, and the sub-resolution rule would bump the MAGNITUDE, handing
    /// `Floor` a value one ULP BELOW `-1` (representable, hence silently wrong).
    #[test]
    fn expm1_deep_negative_lands_just_above_minus_one() {
        let below = -UNIT; // -1.0
        let above = -(UNIT - 1); // -0.99999999999999999999
        for (mode, want) in [
            (RoundingMode::HalfToEven, below),
            (RoundingMode::HalfAwayFromZero, below),
            (RoundingMode::HalfTowardZero, below),
            (RoundingMode::Floor, below),
            (RoundingMode::Trunc, above),
            (RoundingMode::Ceiling, above),
        ] {
            assert_eq!(
                d38s20(-1000 * UNIT)
                    .expm1_strict_with(mode)
                    .to_bits()
                    .as_i128(),
                want,
                "expm1(-1000) mode {mode:?}"
            );
        }
    }

    /// The identity that holds in fixed point — but ONLY for the rounding modes
    /// that commute with a grid translation.
    ///
    /// `1` is exactly `10^SCALE` raw units, so `v ↦ v - 1` maps grid to grid.
    /// `Floor` and `Ceiling` are defined relative to `±∞` and the nearest modes
    /// relative to the two bracketing grid points, so all of them commute with
    /// that translation and `exp_strict(x) - 1` is then bit-for-bit the
    /// correctly-rounded `expm1(x)`.
    ///
    /// `Trunc` does NOT commute — see
    /// [`expm1_trunc_differs_from_exp_minus_one_for_negative_x`], which pins the
    /// disagreement rather than papering over it. `AwayFromZero` and
    /// `ZeroFiveUp` are excluded for the same reason: both are defined
    /// relative to ZERO (the first steps a discard away from it, the second
    /// truncates toward it unless the retained digit is `0` or `5`), and for
    /// `x < 0` the translation moves the result across zero — `expm1(x)` is
    /// negative where `exp(x)` is a positive value below 1.
    ///
    /// (`HalfAwayFromZero` / `HalfTowardZero` are also defined relative to zero,
    /// but they differ from `HalfToEven` only ON an exact tie, and `e^x - 1` is
    /// transcendental for algebraic `x != 0`, so no tie can occur here.)
    #[test]
    fn expm1_agrees_with_exp_minus_one_under_translation_invariant_modes() {
        const COMMUTING: [RoundingMode; 5] = [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ];
        let one = D::<Int<2>, 20>::ONE;
        for arg in [
            -3 * UNIT,
            -2 * UNIT,
            -UNIT,
            -UNIT / 2,
            UNIT / 2,
            UNIT,
            2 * UNIT,
            3 * UNIT,
        ] {
            for mode in COMMUTING {
                let via_expm1 = d38s20(arg).expm1_strict_with(mode);
                let via_exp = d38s20(arg).exp_strict_with(mode) - one;
                assert_eq!(
                    via_expm1.to_bits().as_i128(),
                    via_exp.to_bits().as_i128(),
                    "expm1 vs exp-1 at {arg}e-{S} mode {mode:?}"
                );
            }
        }
    }

    /// `Trunc` is the one shipped mode defined relative to ZERO, so it does NOT
    /// commute with the `- 1` grid translation — and for `x < 0` the translation
    /// crosses zero: `e^x ∈ (0, 1)` is positive (Trunc rounds its magnitude
    /// DOWN) while `expm1(x) < 0` (Trunc rounds ITS magnitude down, i.e. the
    /// value UP). The two therefore land one ULP apart, and `exp_strict(x) - 1`
    /// is the WRONG one.
    ///
    /// This is a correctness reason for the function that survives fixed point,
    /// distinct from the domain-reach argument: on the negative half under
    /// `Trunc`, the two-step form is not merely slower, it is off by an ULP.
    ///
    /// Verified without any hand-computed expansion: for a NEGATIVE non-grid
    /// value, "toward zero" and "toward `+∞`" are the same direction, so a
    /// correctly rounded `Trunc` must equal `Ceiling`. `expm1_strict` satisfies
    /// that identity; `exp_strict(x) - 1` cannot, since it truncated a positive
    /// quantity.
    #[test]
    fn expm1_trunc_differs_from_exp_minus_one_for_negative_x() {
        let one = D::<Int<2>, 20>::ONE;
        for arg in [-3 * UNIT, -2 * UNIT, -UNIT, -UNIT / 2] {
            let trunc = d38s20(arg)
                .expm1_strict_with(RoundingMode::Trunc)
                .to_bits()
                .as_i128();
            let ceil = d38s20(arg)
                .expm1_strict_with(RoundingMode::Ceiling)
                .to_bits()
                .as_i128();
            assert_eq!(
                trunc, ceil,
                "for x < 0, Trunc and Ceiling are the same direction; arg {arg}"
            );
            let two_step = (d38s20(arg).exp_strict_with(RoundingMode::Trunc) - one)
                .to_bits()
                .as_i128();
            assert_eq!(
                two_step,
                trunc - 1,
                "exp_strict(x)-1 under Trunc must sit exactly one ULP below the \
                 correctly-rounded expm1 for x < 0; arg {arg}"
            );
        }
        // For x > 0 both values are positive, so Trunc IS Floor for both and the
        // identity is restored.
        for arg in [UNIT / 2, UNIT, 2 * UNIT] {
            let via_expm1 = d38s20(arg)
                .expm1_strict_with(RoundingMode::Trunc)
                .to_bits()
                .as_i128();
            let via_exp = (d38s20(arg).exp_strict_with(RoundingMode::Trunc) - one)
                .to_bits()
                .as_i128();
            assert_eq!(via_expm1, via_exp, "x > 0 Trunc must still agree; arg {arg}");
        }
    }

    /// The capability `exp` cannot provide: because the `- 1` is applied at the
    /// WORKING scale, ahead of the storage range check, `expm1` is defined on
    /// `x <= ln(1 + MAX)` where `exp` stops at `ln(MAX)`.
    ///
    /// At D38<37>, `MAX = Int<2>::MAX / 10^37 = 17.0141…`, so the extra band is
    /// `x ∈ (ln 17.0141, ln 18.0141] = (2.8342…, 2.8913…]`. At `x = 2.85`,
    /// `e^x = 17.2877…` exceeds `MAX` (so `exp_strict` is out of range) while
    /// `expm1(x) = 16.2877…` sits comfortably inside it.
    ///
    /// The band is narrow — `ln(1 + 1/MAX) ≈ 0.057` here, and narrower at lower
    /// scales — but it is real, and this pins that it is reachable.
    #[test]
    fn expm1_reaches_arguments_exp_cannot_represent() {
        // 2.85 at SCALE 37.
        let x = D::<Int<2>, 37>::from_bits(i2(285) * i2(10).pow(35));
        let got = x.expm1_strict().to_bits();
        // 16.2877 < expm1(2.85) < 16.2879, from e^2.85 = 17.28778…
        // At SCALE 37, `16.2877` is `162877 * 10^33` (6 significant digits + 33).
        let lo = i2(162_877) * i2(10).pow(33);
        let hi = i2(162_879) * i2(10).pow(33);
        assert!(
            got > lo && got < hi,
            "expm1(2.85) at D38<37> outside its analytic window"
        );
    }

    /// The other half of the previous test: the same argument through
    /// `exp_strict` is out of range, which is what makes the `expm1` answer a
    /// capability rather than a convenience.
    #[test]
    #[should_panic(expected = "out of range")]
    fn exp_cannot_represent_what_expm1_reaches() {
        let x = D::<Int<2>, 37>::from_bits(i2(285) * i2(10).pow(35));
        let _ = x.exp_strict();
    }

    /// Narrow-width parity: D18 routes through the same policy at its own
    /// storage width, so it must agree with D38 on a value both can hold.
    #[test]
    fn expm1_d18_agrees_with_d38_on_shared_values() {
        for raw in [1_i128, -1, 10_i128.pow(17) / 2, -(10_i128.pow(17) / 2)] {
            let narrow = D::<Int<1>, 17>::from_bits(i1(raw))
                .expm1_strict()
                .to_bits()
                .as_i128();
            let wide = D::<Int<2>, 17>::from_bits(i2(raw))
                .expm1_strict()
                .to_bits()
                .as_i128();
            assert_eq!(narrow, wide, "expm1 D18 vs D38 at raw {raw}");
        }
    }

    /// Wide-tier routing: the D57 arm reaches the same generic kernels through
    /// `Core::W` / `Core::Wexp`, so it must reproduce the D38 answers on values
    /// both widths hold. Covers both matcher regions.
    ///
    /// `Int<3>` has no public `i128` conversion, so the two widths are compared
    /// by widening the D38 answer rather than narrowing the D57 one — the same
    /// equality, read in the direction the public surface allows.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[test]
    fn expm1_d57_agrees_with_d38_across_both_regions() {
        for raw in [UNIT / 2, -UNIT / 2, UNIT, -UNIT, 2 * UNIT, -2 * UNIT] {
            for mode in MODES {
                let wide = D::<Int<3>, 20>::from_bits(i3(raw))
                    .expm1_strict_with(mode)
                    .to_bits();
                let narrow = d38s20(raw).expm1_strict_with(mode).to_bits().as_i128();
                assert_eq!(
                    wide,
                    i3(narrow),
                    "expm1 D57 vs D38 at raw {raw} mode {mode:?}"
                );
            }
        }
    }
}
