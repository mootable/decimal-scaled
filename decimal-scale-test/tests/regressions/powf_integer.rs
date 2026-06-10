//! `powf` integer-exponent regressions: the `powf(int) == powi(int)`
//! cross-surface parity contract, and the wide-tier exact integer-power
//! directed-rounding pin whose oracle is an independent integer division
//! (`10^SCALE / base^|k|`) — both shapes the width-agnostic golden leads
//! cannot target per tier.

/// Parity test: `powf_strict(D::try_from(n).unwrap())` must agree with `powi(n)`
/// to within storage ULP for every supported width.
///
/// Integer-valued exponents (with `|n|` within the fast-path threshold)
/// short-circuit `powf_strict` to the exact `powi` square-and-multiply
/// path. The two surfaces must therefore produce bit-identical results
/// on those inputs — `powi` is exact and `powf_strict` is the
/// 0.5-ULP-correctly-rounded form, but `powi(n)` is also the exact
/// integer-only result, so they must coincide bit-exactly.
///
/// For non-integer exponents (e.g. 0.5, 2.5), the fast path does not
/// fire; that case is exercised by the existing `powf_*` lib tests.
mod from_powf_integer_fastpath_parity {
    use decimal_scaled::{D18, D38};

    /// Exercises both `from_i32` and a manual `from_bits(n * multiplier)`
    /// shape, since the integer-detection helper checks
    /// `exp_raw % multiplier == 0` directly on the storage. Both shapes
    /// must hit the fast path.
    fn d38_check<const S: u32>(base_raw: i128, n: i32) {
        let base = D38::<S>::from_bits(decimal_scaled::Int::<2>::try_from(base_raw).unwrap());
        let exp_d = D38::<S>::from(n);
        let from_powf = base.powf_strict(exp_d);
        let from_powi = base.powi(n);
        assert_eq!(
            from_powf.to_bits(),
            from_powi.to_bits(),
            "D38<{S}>: powf_strict({base_raw}, {n}) != powi({n})",
        );
    }

    fn d18_check<const S: u32>(base_raw: i64, n: i32) {
        let base = D18::<S>::from_bits(decimal_scaled::Int::<1>::from(base_raw));
        let exp_d = D18::<S>::from(n);
        let from_powf = base.powf_strict(exp_d);
        let from_powi = base.powi(n);
        assert_eq!(
            from_powf.to_bits(),
            from_powi.to_bits(),
            "D18<{S}>: powf_strict({base_raw}, {n}) != powi({n})",
        );
    }

    #[test]
    fn d38_powf_integer_fastpath_parity_s12() {
        // 2.0, 3.0, 1.5, 0.7 at SCALE = 12.
        let bases: &[i128] = &[
            2_000_000_000_000,
            3_000_000_000_000,
            1_500_000_000_000,
            700_000_000_000,
            1_100_000_000_000,
        ];
        for &b in bases {
            for n in -5_i32..=10 {
                d38_check::<12>(b, n);
            }
        }
    }

    #[test]
    fn d18_powf_integer_fastpath_parity_s6() {
        // 2.0, 3.0, 1.5 at SCALE = 6.
        let bases: &[i64] = &[2_000_000, 3_000_000, 1_500_000, 700_000];
        for &b in bases {
            for n in -5_i32..=10 {
                d18_check::<6>(b, n);
            }
        }
    }

    /// Zero exponent: must return ONE regardless of base (matches the
    /// integer-exponent contract). Exercises the `n == 0` edge of the
    /// fast-path bounds check.
    #[test]
    fn d38_powf_zero_exp_returns_one() {
        let base =
            D38::<12>::from_bits(decimal_scaled::Int::<2>::try_from(2_000_000_000_000_i128).unwrap());
        let zero_exp = D38::<12>::from(0);
        assert_eq!(
            base.powf_strict(zero_exp).to_bits(),
            D38::<12>::ONE.to_bits()
        );
    }

    /// Negative integer exponent: must agree with `powi(-n)`, which routes
    /// through `ONE / pow(|n|)`. Exercises the sign branch.
    #[test]
    fn d38_powf_negative_integer_exp_parity() {
        let base =
            D38::<12>::from_bits(decimal_scaled::Int::<2>::try_from(2_000_000_000_000_i128).unwrap()); // 2.0
        for n in [-1_i32, -2, -3, -5, -10] {
            let exp_d = D38::<12>::from(n);
            assert_eq!(
                base.powf_strict(exp_d).to_bits(),
                base.powi(n).to_bits(),
                "D38<12>: powf_strict(2.0, {n}) != powi({n})",
            );
        }
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d57_powf_integer_fastpath_parity() {
        use decimal_scaled::D57;
        // 2.0 at SCALE = 20 (a primary target scale for D57).
        let two = D57::<20>::from(2);
        for n in -5_i32..=10 {
            let exp_d = D57::<20>::from(n);
            let from_powf = two.powf_strict(exp_d);
            let from_powi = two.powi(n);
            assert_eq!(
                from_powf.to_bits(),
                from_powi.to_bits(),
                "D57<20>: powf_strict(2.0, {n}) != powi({n})",
            );
        }
    }

    #[cfg(feature = "wide")]
    #[test]
    fn d76_powf_integer_fastpath_parity() {
        use decimal_scaled::D76;
        // 2.0 at SCALE = 35.
        let two = D76::<35>::from(2);
        for n in -3_i32..=8 {
            let exp_d = D76::<35>::from(n);
            let from_powf = two.powf_strict(exp_d);
            let from_powi = two.powi(n);
            assert_eq!(
                from_powf.to_bits(),
                from_powi.to_bits(),
                "D76<35>: powf_strict(2.0, {n}) != powi({n})",
            );
        }
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d307_powf_integer_fastpath_parity() {
        use decimal_scaled::D307;
        // 2.0 at SCALE = 150 (matches the bench's xx-wide cell).
        let two = D307::<150>::from(2);
        for n in -3_i32..=8 {
            let exp_d = D307::<150>::from(n);
            let from_powf = two.powf_strict(exp_d);
            let from_powi = two.powi(n);
            assert_eq!(
                from_powf.to_bits(),
                from_powi.to_bits(),
                "D307<150>: powf_strict(2.0, {n}) != powi({n})",
            );
        }
    }
}

/// Wide-tier `powf` exact integer-power directed-rounding gate.
///
/// When the base and exponent are exact integers and the result terminates
/// (`base = 2^a·5^b`), `base^-k` lands exactly on a storage grid line, so
/// every rounding mode must return that exact value. The wide
/// `powf_strict_with` integer fast path used to compute the reciprocal as
/// `ONE.div_with(self.checked_pow(|k|), mode)`, and `checked_pow` is a
/// *decimal* power: at a near-max scale `base^|k| · 10^SCALE` overflows
/// storage, so it returned `None` and the shell DEFERRED to the to-nearest
/// `exp(k·ln base)` composition — which lands ~1 ULP low, mis-rounding Floor
/// and Trunc by one LSB. The exact integer pin divides `10^SCALE` by the
/// INTEGER `base^|k|`, so the reciprocal is exact even when the scaled power
/// overflows.
///
/// Oracle: the exact raw value `10^SCALE / base^|k|` is computed by direct
/// integer division of the work integer (independent of the `powf`
/// composition); the division is exact because the divisor divides `10^SCALE`
/// for every terminating case here. Scales are `MAX_SCALE − 1` per tier — the
/// band edge where the scaled power overflows but the bases (≤ 40) and the
/// sub-1 results remain representable.
#[cfg(feature = "wide")]
mod from_powf_wide_integer_exact {
    use decimal_scaled::{Int, RoundingMode};

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    /// `(base, exponent, divisor = base^|exponent|)` — exact (terminating)
    /// integer reciprocals drawn from the failing golden inputs.
    const CASES: &[(i32, i32, i64)] = &[
        (10, -2, 100),
        (10, -3, 1000),
        (16, -3, 4096),
        (20, -2, 400),
        (25, -2, 625),
        (25, -3, 15_625),
        (32, -2, 1024),
        (32, -3, 32_768),
        (4, -3, 64),
        (5, -2, 25),
        (5, -3, 125),
        (40, -2, 1600),
    ];

    macro_rules! tier_test {
        ($name:ident, $Ty:ident, $N:literal, $S:literal) => {
            #[test]
            fn $name() {
                use decimal_scaled::$Ty;
                // 10^S in the tier's work integer (exact; fits with ≥ 1 digit
                // headroom at MAX_SCALE − 1).
                let p10: Int<$N> = Int::<$N>::from(10i64).pow($S);
                for &(b, e, div) in CASES {
                    let base = $Ty::<$S>::from(b);
                    let exp = $Ty::<$S>::from(e);
                    // Exact reciprocal raw = 10^S / base^|k| (terminating ⇒ exact).
                    let want: Int<$N> = p10 / Int::<$N>::from(div);
                    for &mode in &MODES {
                        let got = base.powf_strict_with(exp, mode).to_bits();
                        assert_eq!(
                            got, want,
                            "{} S={} {}^{} mode={:?}",
                            stringify!($Ty), $S, b, e, mode
                        );
                    }
                }
            }
        };
    }

    tier_test!(d57_integer_powers_directed_exact, D57, 3, 55);
    tier_test!(d76_integer_powers_directed_exact, D76, 4, 74);
    tier_test!(d115_integer_powers_directed_exact, D115, 6, 113);
    tier_test!(d153_integer_powers_directed_exact, D153, 8, 151);
    tier_test!(d230_integer_powers_directed_exact, D230, 12, 228);
    tier_test!(d307_integer_powers_directed_exact, D307, 16, 305);

    #[cfg(feature = "x-wide")]
    tier_test!(d462_integer_powers_directed_exact, D462, 24, 460);
    #[cfg(feature = "x-wide")]
    tier_test!(d616_integer_powers_directed_exact, D616, 32, 614);

    #[cfg(feature = "xx-wide")]
    tier_test!(d924_integer_powers_directed_exact, D924, 48, 922);
    #[cfg(feature = "xx-wide")]
    tier_test!(d1232_integer_powers_directed_exact, D1232, 64, 1230);

    /// Fractional-base integer exponents: a terminating-decimal base to a small
    /// integer power is computed by the exact chain (`2.5^2 = 6.25`) and a
    /// negative exponent by one correctly-rounded division — including the
    /// non-terminating reciprocal `1.5^-1 = 0.666...`, whose last digit each mode
    /// must place per its rule (residual `.66... > half`: nearest modes round up,
    /// Trunc/Floor keep, Ceiling bumps).
    mod fractional_base {
        use super::MODES;
        use decimal_scaled::{RoundingMode, D1232, D57};

        fn sixes_with_last(scale: usize, last: char) -> String {
            let mut s = String::from("0.");
            for _ in 1..scale {
                s.push('6');
            }
            s.push(last);
            s
        }

        macro_rules! check_tier {
            ($D:ty, $S:expr) => {{
                type D = $D;
                let parse = |s: &str| s.parse::<D>().unwrap();
                for m in MODES {
                    assert_eq!(parse("2.5").powf_strict_with(parse("2"), m), parse("6.25"), "{m:?} 2.5^2");
                    assert_eq!(parse("0.5").powf_strict_with(parse("-2"), m), parse("4"), "{m:?} 0.5^-2");
                    assert_eq!(parse("1.5").powf_strict_with(parse("3"), m), parse("3.375"), "{m:?} 1.5^3");
                    assert_eq!(parse("0.1").powf_strict_with(parse("5"), m), parse("0.00001"), "{m:?} 0.1^5");
                    let last = match m {
                        RoundingMode::Trunc | RoundingMode::Floor => '6',
                        _ => '7', // nearest (residual above half) and Ceiling round up
                    };
                    assert_eq!(
                        parse("1.5").powf_strict_with(parse("-1"), m),
                        parse(&sixes_with_last($S, last)),
                        "{m:?} 1.5^-1"
                    );
                }
            }};
        }

        #[test]
        fn d57_fractional_exact_and_reciprocal() {
            check_tier!(D57<19>, 19);
        }

        #[cfg(feature = "xx-wide")]
        #[test]
        fn d1232_fractional_exact_and_reciprocal() {
            check_tier!(D1232<30>, 30);
        }
    }
}
