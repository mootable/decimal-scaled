//! Regressions at SCALE = MAX_SCALE (and saturated-storage inputs): the
//! per-tier storage extremes are width-SPECIFIC, which the width-agnostic
//! golden lead format cannot express, so these stay as pinned reproducers.

/// Regression test for the `log10_strict` panic on `D38::<38>` at
/// scale-saturated inputs.
///
/// At SCALE=38 the storage `i128::MAX ≈ 1.7e38` only represents values
/// in roughly `[-1.7, 1.7]`. Calling `from_f64(2.0)` therefore
/// saturates to `Self::MAX`, and historically `log10_strict()` panicked
/// during the ln kernel's range reduction because the working-scale
/// intermediate `raw · 10^STRICT_GUARD = MAX · 10^30 ≈ 1.7e68` plus the
/// Mercator artanh series intermediate overflowed the 256-bit `Fixed`.
#[cfg(all(feature = "strict", not(feature = "fast")))]
mod from_d38_max_scale_log_panic {
    use decimal_scaled::D38;

    /// `D38::<38>::from_f64(2.0)` saturates to `MAX` then `log10_strict`
    /// must produce a finite result without panicking. `log10(MAX) ≈ 0.23`
    /// (because `MAX ≈ 1.7`), which fits the storage comfortably.
    #[test]
    fn log10_strict_on_saturated_d38_scale38() {
        let v = D38::<38>::from_f64(2.0);
        let r = v.log10_strict();
        // log10(1.7014…) ≈ 0.23099…  -> at scale 38, bits ≈ 2.31e37
        let bits = i128::from(r.to_bits());
        assert!(bits > 0, "log10(MAX) must be positive, got {bits}");
        assert!(
            bits < 3 * 10_i128.pow(37),
            "log10(MAX) ≈ 0.23, bits ≈ 2.3e37, got {bits}"
        );
    }

    /// And a direct "near-boundary" input via `from_bits` to pin the
    /// failure to the kernel rather than `from_f64`'s saturation policy.
    #[test]
    fn log10_strict_on_near_max_d38_scale38() {
        let v = D38::<38>::from_bits(decimal_scaled::Int::<2>::try_from(i128::MAX).unwrap());
        let r = v.log10_strict();
        // v = i128::MAX / 10^38 ∈ (1.70, 1.71), so log10(v) ∈ (log10(1.70),
        // log10(1.71)) = (0.2304, 0.2330) ⊂ (0.22, 0.24); at scale 38 the
        // raw bits land in (2.2e37, 2.4e37).
        let bits = i128::from(r.to_bits());
        assert!(
            bits > 22 * 10_i128.pow(36),
            "log10(near-MAX) ≈ 0.2308, bits ≈ 2.31e37, got {bits}"
        );
        assert!(
            bits < 24 * 10_i128.pow(36),
            "log10(near-MAX) ≈ 0.2308, bits ≈ 2.31e37, got {bits}"
        );
    }
}

/// Regression: `D57<MAX_SCALE = 57>::cbrt_strict` used to overflow the
/// work integer inside the generic-wide cbrt kernel.
///
/// At SCALE=57 the kernel needs `mag * 10^(2*SCALE) = mag * 10^114`. A
/// `mag` of any non-trivial magnitude together with `10^114` reaches
/// `~10^115`+, which does not fit `Int384`'s ~`10^115` capacity. The
/// work-integer width was bumped to `Int768` so the kernel is correct
/// at any SCALE up to `D57::MAX_SCALE`.
///
/// Note: At SCALE=57 the representable value range is bounded by
/// `Int192::MAX / 10^57 ≈ 3.14`, so all test inputs must be small.
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
mod from_d57_max_scale_cbrt_panic {
    use decimal_scaled::D57;

    /// Reproducer from the bug report. Previously panicked with
    /// `Int384: mul overflow` from inside the cbrt kernel; should now
    /// run to completion (cbrt of a value near MAX returns a value near
    /// `cbrt(MAX)`, which is the small cube-root of the saturated input).
    #[test]
    fn d57_max_scale_cbrt_does_not_panic() {
        let v = D57::<57>::from_f64(8.0);
        let r = v.cbrt_strict();
        // 8.0 saturates to MAX = Int192::MAX / 10^57 ≈ 3.1385; since
        // 1 < MAX < 8, the cube root must land strictly inside (1, 2)
        // (cbrt(MAX) ≈ 1.4641).
        assert!(
            r > D57::<57>::from(1),
            "cbrt(MAX ≈ 3.14) ≈ 1.464 must exceed 1, got {r:?}"
        );
        assert!(
            r < D57::<57>::from(2),
            "cbrt(MAX ≈ 3.14) ≈ 1.464 must be below 2, got {r:?}"
        );
    }

    /// cbrt(1.0) at D57<MAX_SCALE> — small input that fits the
    /// representable range exactly.
    #[test]
    fn d57_max_scale_cbrt_of_one() {
        let one = D57::<57>::from_bits(
            decimal_scaled::Int::<3>::from_str_radix(
                "1000000000000000000000000000000000000000000000000000000000",
                10,
            )
            .unwrap(),
        );
        let r = one.cbrt_strict();
        assert_eq!(r, one, "cbrt(1) at D57<57> should equal 1");
    }

    /// cbrt(0.125) = 0.5 at D57<MAX_SCALE>.
    #[test]
    fn d57_max_scale_cbrt_of_one_eighth() {
        // 0.125 = 1/8 at SCALE=57 → raw = 125 * 10^54.
        let raw = decimal_scaled::Int::<3>::from_str_radix(
            "125000000000000000000000000000000000000000000000000000000",
            10,
        )
        .unwrap();
        let half_raw = decimal_scaled::Int::<3>::from_str_radix(
            "500000000000000000000000000000000000000000000000000000000",
            10,
        )
        .unwrap();
        let v = D57::<57>::from_bits(raw);
        let expected = D57::<57>::from_bits(half_raw);
        let r = v.cbrt_strict();
        assert_eq!(r, expected, "cbrt(0.125) at D57<57> should equal 0.5");
    }

    /// cbrt(0.0) = 0.0 at D57<MAX_SCALE> — guard that the zero short-cut
    /// path is unaffected.
    #[test]
    fn d57_max_scale_cbrt_of_zero() {
        let zero =
            D57::<57>::from_bits(decimal_scaled::Int::<3>::from_str_radix("0", 10).unwrap());
        let r = zero.cbrt_strict();
        assert_eq!(r, zero, "cbrt(0) at D57<57> should equal 0");
    }
}

/// Regression: every wide tier's `cbrt_strict` used to overflow the
/// work integer inside the generic-wide cbrt kernel when SCALE
/// approached `MAX_SCALE`.
///
/// At SCALE = MAX_SCALE the kernel needs `mag * 10^(2*SCALE)`, which
/// peaks near `10^(3*SCALE)` bits. The original `$CbrtWide` for each
/// tier was sized at 2× the storage width — large enough for most
/// SCALEs but not for MAX_SCALE. The work-integer widths were bumped
/// one step further per tier so the kernel is correct at any SCALE up
/// to that tier's `MAX_SCALE`.
///
/// Companion to the D57 module above, which covers the D57 reproducer
/// in detail.
///
/// Note: at MAX_SCALE the representable range is bounded by
/// `Storage::MAX / 10^MAX_SCALE`, so test inputs must be small. Each
/// tier asserts `cbrt(1) == 1` — a value that always fits since
/// `Storage::MAX >= 10^MAX_SCALE` by construction of MAX_SCALE.
#[cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
)))]
mod from_wide_max_scale_cbrt_panic {
    /// Build `D::<MAX_SCALE>` representing `1.0` via `from_bits(10^MAX_SCALE)`
    /// then assert `cbrt(1) == 1`. The raw `10^MAX_SCALE` is generated by
    /// `pow` on the storage type so the literal can't drift away from the
    /// SCALE.
    ///
    /// Only used by the wide-tier invocations below; gated so a default
    /// (narrow-only) build, where every invocation is `cfg`-stripped, does
    /// not see an unused macro.
    #[cfg(any(feature = "wide", feature = "x-wide", feature = "xx-wide"))]
    macro_rules! cbrt_of_one_test {
        ($fn_name:ident, $tier:ident, $scale:literal, $Storage:ty) => {
            #[test]
            fn $fn_name() {
                let ten = <$Storage>::from_str_radix("10", 10).expect("10 literal");
                let raw = ten.pow($scale as u32);
                let one = decimal_scaled::$tier::<$scale>::from_bits(raw);
                let r = one.cbrt_strict();
                assert_eq!(
                    r, one,
                    concat!(
                        "cbrt(1) at ",
                        stringify!($tier),
                        "<",
                        stringify!($scale),
                        "> should equal 1",
                    ),
                );
            }
        };
    }

    #[cfg(feature = "wide")]
    cbrt_of_one_test!(d76_max_scale_cbrt_of_one, D76, 76, decimal_scaled::Int<4>);

    #[cfg(feature = "wide")]
    cbrt_of_one_test!(d115_max_scale_cbrt_of_one, D115, 115, decimal_scaled::Int<6>);

    #[cfg(feature = "wide")]
    cbrt_of_one_test!(d153_max_scale_cbrt_of_one, D153, 153, decimal_scaled::Int<8>);

    #[cfg(feature = "wide")]
    cbrt_of_one_test!(d230_max_scale_cbrt_of_one, D230, 230, decimal_scaled::Int<12>);

    #[cfg(any(feature = "wide", feature = "x-wide"))]
    cbrt_of_one_test!(d307_max_scale_cbrt_of_one, D307, 307, decimal_scaled::Int<16>);

    #[cfg(feature = "x-wide")]
    cbrt_of_one_test!(d462_max_scale_cbrt_of_one, D462, 461, decimal_scaled::Int<24>);

    #[cfg(feature = "x-wide")]
    cbrt_of_one_test!(d616_max_scale_cbrt_of_one, D616, 615, decimal_scaled::Int<32>);

    #[cfg(feature = "xx-wide")]
    cbrt_of_one_test!(d924_max_scale_cbrt_of_one, D924, 923, decimal_scaled::Int<48>);

    #[cfg(feature = "xx-wide")]
    cbrt_of_one_test!(d1232_max_scale_cbrt_of_one, D1232, 1231, decimal_scaled::Int<64>);
}
