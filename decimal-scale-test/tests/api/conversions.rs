//! The public conversion surface: `From`/`TryFrom` constructors, the float
//! bridge, `to_int`, widen/narrow ergonomics, the composed cross-width +
//! cross-scale `convert_from` family, and the widen/narrow round-trip
//! invariants. The wide-gated ladder-hop coverage (`widen_narrow_default.rs`)
//! joins this target as its own `mod from_widen_narrow_default` block in the
//! feature-gated batch.

mod from_conversions {
    //! Integration tests for the conversion surface
    //! (From<integer> / TryFrom<i128|u128|f32|f64> / to_int /
    //! from_f64 / to_f64 / to_f32).
    //!
    //! Bodies live in src/macros/conversions.rs and float_bridge.rs;
    //! these tests exercise the resulting public API for D38 specifically.

    use decimal_scaled::{ConvertError, D38, D38s12};

    // --- widen / narrow ergonomic methods -------------------------------

    #[test]
    fn widen_narrow_one_tier_hop_narrow_arm() {
        use decimal_scaled::{D18s6, D38s6};
        let a = D18s6::from(123);
        let b: D38s6 = a.widen(); // D18 → D38
        assert_eq!(b.to_bits(), i128::from(a.to_bits()));
        let c: D18s6 = b.narrow().unwrap(); // D38 → D18
        assert_eq!(c.to_bits(), a.to_bits());
    }

    #[cfg(feature = "wide")]
    #[test]
    fn widen_narrow_into_wide_tier() {
        use decimal_scaled::{D38s12, D57};
        // After the 0.3 widen-chain rework, D38.widen() steps to D57
        // (the immediate next tier in the ladder) instead of jumping
        // straight to D76. The .narrow() symmetric is D57 -> D38.
        let a = D38s12::from(1_000_000);
        let b: D57<12> = a.widen();
        let back = b.narrow().unwrap();
        assert_eq!(back, a);
    }

    // Integer construction via the public From / TryFrom surface

    #[test]
    fn from_int_zero_is_zero() {
        assert_eq!(D38s12::from(0), D38s12::ZERO);
    }

    #[test]
    fn from_i32_zero_is_zero() {
        assert_eq!(D38s12::from(0), D38s12::ZERO);
    }

    #[test]
    fn from_int_one_is_one() {
        assert_eq!(D38s12::from(1), D38s12::ONE);
    }

    #[test]
    fn from_i32_one_is_one() {
        assert_eq!(D38s12::from(1), D38s12::ONE);
    }

    #[test]
    fn from_int_negative() {
        assert_eq!(D38s12::from(-1), -D38s12::ONE);
        assert_eq!(D38s12::from(-42).to_bits(), -42_000_000_000_000_i128);
    }

    // Lossless From<iN> / From<uN> -- bit-exact scaling

    #[test]
    fn from_i8_scales_correctly() {
        assert_eq!(D38s12::from(0_i8).to_bits(), 0);
        assert_eq!(D38s12::from(1_i8).to_bits(), 1_000_000_000_000);
        assert_eq!(D38s12::from(-1_i8).to_bits(), -1_000_000_000_000);
        assert_eq!(D38s12::from(i8::MAX).to_bits(), 127_000_000_000_000);
        assert_eq!(D38s12::from(i8::MIN).to_bits(), -128_000_000_000_000);
    }

    #[test]
    fn from_i64_scales_correctly() {
        assert_eq!(D38s12::from(0_i64).to_bits(), 0);
        assert_eq!(
            D38s12::from(i64::MAX).to_bits(),
            (i64::MAX as i128) * 1_000_000_000_000
        );
        assert_eq!(
            D38s12::from(i64::MIN).to_bits(),
            (i64::MIN as i128) * 1_000_000_000_000
        );
    }

    #[test]
    fn from_u64_at_boundary_is_lossless() {
        let v = D38s12::from(u64::MAX);
        assert_eq!(v.to_bits(), (u64::MAX as i128) * 1_000_000_000_000);
    }

    // to_int

    #[test]
    fn to_int_lossy_default_rounds_half_to_even() {
        // 2.5 with HalfToEven default -> 2 (even neighbour).
        assert_eq!(D38s12::from_bits(decimal_scaled::Int::<2>::try_from(2_500_000_000_000_i128).unwrap()).to_int(), 2);
        // 3.5 with HalfToEven -> 4 (even).
        assert_eq!(D38s12::from_bits(decimal_scaled::Int::<2>::try_from(3_500_000_000_000_i128).unwrap()).to_int(), 4);
    }

    #[test]
    fn to_int_lossy_saturates() {
        assert_eq!(D38s12::MAX.to_int(), i64::MAX);
        assert_eq!(D38s12::MIN.to_int(), i64::MIN);
    }

    // from_f64 + to_f64

    #[test]
    fn from_f64_lossy_zero_is_zero() {
        assert_eq!(D38s12::from_f64(0.0), D38s12::ZERO);
    }

    #[test]
    fn zero_to_f64_lossy_is_zero() {
        assert_eq!(D38s12::ZERO.to_f64(), 0.0);
    }

    #[test]
    fn from_f64_lossy_one_is_one() {
        assert_eq!(D38s12::from_f64(1.0), D38s12::ONE);
    }

    #[test]
    fn from_f64_lossy_negative() {
        assert_eq!(D38s12::from_f64(-1.0), -D38s12::ONE);
    }

    #[test]
    fn from_f64_lossy_infinity_saturates_max() {
        assert_eq!(D38s12::from_f64(f64::INFINITY), D38s12::MAX);
    }

    #[test]
    fn from_f64_lossy_neg_infinity_saturates_min() {
        assert_eq!(D38s12::from_f64(f64::NEG_INFINITY), D38s12::MIN);
    }

    #[test]
    fn from_f64_lossy_nan_is_zero() {
        assert_eq!(D38s12::from_f64(f64::NAN), D38s12::ZERO);
    }

    #[test]
    fn from_f64_lossy_finite_out_of_range_saturates() {
        assert_eq!(D38s12::from_f64(1e30), D38s12::MAX);
        assert_eq!(D38s12::from_f64(-1e30), D38s12::MIN);
    }

    // TryFrom<i128> / TryFrom<u128>

    #[test]
    fn try_from_i128_in_range_succeeds() {
        let v: D38s12 = 1_000_000_i128.try_into().expect("in-range fits");
        assert_eq!(v.to_bits(), 1_000_000 * 1_000_000_000_000);
    }

    #[test]
    fn try_from_i128_overflow_returns_err() {
        let result: Result<D38s12, _> = i128::MAX.try_into();
        assert_eq!(result, Err(ConvertError::Overflow));
    }

    #[test]
    fn try_from_u128_max_returns_err() {
        let result: Result<D38s12, _> = u128::MAX.try_into();
        assert_eq!(result, Err(ConvertError::Overflow));
    }

    // TryFrom<f64> / TryFrom<f32>

    #[test]
    fn try_from_f64_one_succeeds() {
        let v: D38s12 = 1.0_f64.try_into().expect("one fits");
        assert_eq!(v, D38s12::ONE);
    }

    #[test]
    fn try_from_f64_nan_returns_err() {
        let result: Result<D38s12, _> = f64::NAN.try_into();
        assert_eq!(result, Err(ConvertError::NotFinite));
    }

    #[test]
    fn try_from_f64_out_of_range_returns_err() {
        let result: Result<D38s12, _> = 1e30_f64.try_into();
        assert_eq!(result, Err(ConvertError::Overflow));
    }

    #[test]
    fn try_from_f32_infinity_returns_err() {
        let result: Result<D38s12, _> = f32::INFINITY.try_into();
        assert_eq!(result, Err(ConvertError::NotFinite));
    }

    // Cross-scale sanity

    #[test]
    fn from_int_works_at_scale_6() {
        type D6 = D38<6>;
        let v: D6 = D6::from(1_000_i64);
        assert_eq!(v.to_bits(), 1_000_000_000); // 10^9
        assert_eq!(v.to_int(), 1_000);
    }

    #[test]
    fn from_int_works_at_scale_0() {
        type D0 = D38<0>;
        let v: D0 = D0::from(42_i64);
        assert_eq!(v.to_bits(), 42);
        assert_eq!(v.to_int(), 42);
    }

    // TryFrom<i64> / TryFrom<u64> for D18 (64-bit storage tier).
    //
    // D18 cannot offer an infallible `From<i64>` / `From<u64>` because
    // `value * 10^SCALE` may overflow the i64 storage (and `u64` above
    // `i64::MAX` overflows even at SCALE 0), so the standard surface is
    // `TryFrom`. Wider tiers (D38+) keep their infallible `From<i64>`.

    #[test]
    fn try_from_i64_d18_in_range() {
        use decimal_scaled::D18;
        let v: D18<2> = 100_i64.try_into().unwrap();
        assert_eq!(v.to_bits(), 10_000);
        // SCALE 0: identity-ish, the i64 stores directly.
        let v: D18<0> = (-7_i64).try_into().unwrap();
        assert_eq!(v.to_bits(), -7);
    }

    #[test]
    fn try_from_i64_d18_overflow_returns_err() {
        use decimal_scaled::D18;
        // i64::MAX scaled by 10^2 overflows the i64 storage.
        let result: Result<D18<2>, _> = i64::MAX.try_into();
        assert_eq!(result, Err(ConvertError::Overflow));
    }

    #[test]
    fn try_from_u64_d18_in_range() {
        use decimal_scaled::D18;
        let v: D18<2> = 100_u64.try_into().unwrap();
        assert_eq!(v.to_bits(), 10_000);
    }

    #[test]
    fn try_from_u64_d18_above_i64_max_returns_err() {
        use decimal_scaled::D18;
        // A u64 above i64::MAX cannot fit signed storage even at SCALE 0.
        let result: Result<D18<0>, _> = u64::MAX.try_into();
        assert_eq!(result, Err(ConvertError::Overflow));
    }

    // TryFrom<f64> rounds to scale via the crate-default RoundingMode
    // (HalfToEven unless a `rounding-*` feature overrides it). Under the
    // default build, a value whose scaled form lands on a .5 boundary
    // rounds to even.

    #[cfg(feature = "std")]
    #[cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]
    #[test]
    fn try_from_f64_rounds_half_to_even_default() {
        // 0.125 at SCALE 2 = 12.5 scaled units; HalfToEven -> 12.
        let v: D38<2> = 0.125_f64.try_into().unwrap();
        assert_eq!(v.to_bits(), 12);
        // 0.135 at SCALE 2 = 13.5 scaled units; HalfToEven -> 14.
        // (0.135 is not exactly representable in f64; it is slightly above
        // 0.135, so this also confirms the value is rounded, not truncated.)
        let v: D38<2> = 0.135_f64.try_into().unwrap();
        assert_eq!(v.to_bits(), 14);
        // A clearly-fractional value rounds rather than truncating: 1.6 at
        // SCALE 0 = 1.6 scaled units -> 2 (truncation would give 1).
        let v: D38<0> = 1.6_f64.try_into().unwrap();
        assert_eq!(v.to_bits(), 2);
    }
}

mod from_identity_invariants {
    //! Phase-1 capstone (story 1.6): invariant / property tests that encode
    //! the 1.1–1.5 rulings. Each test asserts; none silently no-ops.
    //!
    //! - 1.2  width round-trip: `narrow_n(widen_n(x)) == x`.
    //! - 1.3  cross-width AND cross-scale exact value-equality.
    //! - 1.1  conversion round-trips (`from_int` / `to_int`, `Int::widen` /
    //!   `Int::narrow`).
    //! - 1.4  overflow contract: debug-panic on the operator path, `None`
    //!   on the `checked_*` path.

    use decimal_scaled::{D18s9, D38, D38s12, Int, D};

    // ─── 1.2 — width round-trip: narrow_n(widen_n(x)) == x ────────────────

    #[test]
    fn widen_then_narrow_is_identity_positive() {
        let x: D18s9 = D18s9::from(7);
        // Widen the storage Int<1> -> Int<2>, then narrow back.
        let wide: D<Int<2>, 9> = x.widen_n::<2>();
        let back: D<Int<1>, 9> = wide.narrow_n::<1>().expect("value fits Int<1>");
        assert_eq!(back, x);
    }

    #[test]
    fn widen_then_narrow_is_identity_negative() {
        let x: D18s9 = D18s9::from(-12);
        let wide: D<Int<2>, 9> = x.widen_n::<2>();
        let back: D<Int<1>, 9> = wide.narrow_n::<1>().expect("value fits Int<1>");
        assert_eq!(back, x);
    }

    #[test]
    fn widen_then_narrow_is_identity_zero() {
        let x: D18s9 = D18s9::ZERO;
        let wide: D<Int<4>, 9> = x.widen_n::<4>();
        let back: D<Int<1>, 9> = wide.narrow_n::<1>().expect("zero fits Int<1>");
        assert_eq!(back, x);
    }

    #[test]
    fn narrow_n_rejects_out_of_range() {
        // A value that only fits the wider tier must NOT narrow back.
        // from_int(10^17) at scale 2 stores 10^19 > i64::MAX, so it cannot
        // round-trip into the Int<1>-backed tier.
        let huge: D38<2> = D38::<2>::from(100_000_000_000_000_000_i64);
        assert!(huge.narrow_n::<1>().is_none());
    }

    // ─── 1.3 — cross-width / cross-scale exact value equality ─────────────

    #[test]
    fn cross_width_same_scale_value_equal() {
        let narrow: D18s9 = D18s9::from(5);
        let wide: D<Int<2>, 9> = D::<Int<2>, 9>::from(5);
        assert_eq!(narrow, wide);
        assert_eq!(wide, narrow);
    }

    #[test]
    fn cross_scale_value_equal() {
        // 5 at scale 9 and 5 at scale 12 are the same logical value.
        let a: D18s9 = D18s9::from(5);
        let b: D38s12 = D38s12::from(5);
        assert_eq!(a, b);
        assert_eq!(b, a);
    }

    #[test]
    fn cross_scale_unequal_when_fraction_differs() {
        // 5.000000001 (scale 9) vs 5.000000000000 (scale 12): not equal.
        let frac: D18s9 = D18s9::from_bits(Int::<1>::from(5_000_000_001_i64));
        let whole: D38s12 = D38s12::from(5);
        assert_ne!(frac, whole);
        assert!(frac > whole);
        assert!(whole < frac);
    }

    #[test]
    fn d_eq_primitive_int_exact() {
        assert!(D38s12::from(42) == 42_i32);
        assert!(42_i64 == D38s12::from(42));
        // A fractional decimal is never equal to an integer.
        let half: D38s12 = D38s12::from_bits(Int::<2>::from(5_500_000_000_000_i64));
        assert!(!(half == 5_i32));
        assert!(!(half == 6_i32));
    }

    // ─── 1.1 — conversion round-trips ─────────────────────────────────────

    #[test]
    fn from_int_to_int_round_trip() {
        for n in [-9_i64, -1, 0, 1, 7, 1234, 9_999_999] {
            let d: D38s12 = D38s12::from(n);
            assert_eq!(d.to_int(), n);
        }
    }

    #[test]
    fn int_widen_narrow_round_trip() {
        for n in [-123_i64, -1, 0, 5, 987_654_321] {
            let small = Int::<1>::from(n);
            let wide = small.widen::<4>();
            let back: Int<1> = wide.narrow::<1>().expect("value fits");
            assert_eq!(back, small);
        }
    }

    // ─── 1.4 — overflow contract ──────────────────────────────────────────

    #[test]
    fn checked_add_returns_none_on_overflow() {
        // The checked path never panics; it reports the overflow.
        assert!(D38s12::MAX.checked_add(D38s12::ONE).is_none());
        // A non-overflowing checked add succeeds.
        assert!(D38s12::ZERO.checked_add(D38s12::ONE).is_some());
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn operator_add_panics_on_overflow() {
        // The `+` operator panics on overflow in BOTH debug and release — a
        // fixed-width decimal never silently wraps a wrong number. (The
        // explicit `wrapping_add` carries the modular behaviour.)
        let _ = D38s12::MAX + D38s12::ONE;
    }
}

mod from_convert_cross_width_scale {
    //! Integration tests for the composed cross-width + cross-scale decimal
    //! `convert_from` / `convert_from_with` constructors.
    //!
    //! These exercise the value-preserving width/scale ordering: widen then
    //! rescale when the target is at least as wide as the source, rescale
    //! then narrow when the target is narrower. Behaviour is asserted on
    //! exact stored values.
    //!
    //! The default-mode (`convert_from`) cases assume `HalfToEven`, so the
    //! whole file is compile-gated to a default-rounding build; that way no
    //! test silently no-ops under a `rounding-*` feature.

    #![cfg(not(any(
        feature = "rounding-half-away-from-zero",
        feature = "rounding-half-toward-zero",
        feature = "rounding-trunc",
        feature = "rounding-floor",
        feature = "rounding-ceiling",
    )))]

    use decimal_scaled::{ConvertError, D18, D38, Int, RoundingMode};

    /// Raw `D38<S>` constructor: `raw` is the stored integer (logical value
    /// `raw / 10^S`).
    fn d38_raw<const S: u32>(raw: i128) -> D38<S> {
        D38::<S>::from_bits(Int::<2>::try_from(raw).unwrap())
    }

    /// Raw `D18<S>` constructor.
    fn d18_raw<const S: u32>(raw: i64) -> D18<S> {
        D18::<S>::from_bits(Int::<1>::try_from(raw as i128).unwrap())
    }

    // ── widen + scale-up (exact) ─────────────────────────────────────────

    /// Widening to a wider tier and scaling UP appends zeros exactly; no
    /// rounding occurs and the result is `Ok`.
    #[test]
    fn widen_scale_up_is_exact() {
        // 1.50 @ D18<2> -> D38<6> == 1.500000 (raw 1_500_000).
        let src: D18<2> = d18_raw::<2>(150);
        let out: D38<6> = D38::<6>::convert_from(src).unwrap();
        assert_eq!(i128::from(out.to_bits()), 1_500_000);
    }

    /// Same width, scale-up (equal-width branch).
    #[test]
    fn same_width_scale_up_is_exact() {
        let src: D38<2> = d38_raw::<2>(150);
        let out: D38<6> = D38::<6>::convert_from(src).unwrap();
        assert_eq!(i128::from(out.to_bits()), 1_500_000);
    }

    // ── widen + scale-down (rounding) ────────────────────────────────────

    /// Widening with a scale-DOWN rounds the discarded digits per the mode.
    /// 1.2345 @ D18<4> -> D38<2>: 1.2345 rounds half-to-even to 1.23 (the
    /// dropped `45` is below the half boundary `50`), i.e. raw 123.
    #[test]
    fn widen_scale_down_rounds_half_to_even() {
        let src: D18<4> = d18_raw::<4>(12_345);
        let out: D38<2> = D38::<2>::convert_from(src).unwrap();
        assert_eq!(i128::from(out.to_bits()), 123);
    }

    /// Explicit-mode scale-down: an exact half rounds differently per mode.
    /// 2.5 @ S=1 -> S=0. Half-to-even -> 2; half-away -> 3; ceiling -> 3;
    /// floor/trunc -> 2.
    #[test]
    fn widen_scale_down_respects_explicit_mode() {
        let src: D18<1> = d18_raw::<1>(25); // 2.5
        let even: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::HalfToEven).unwrap();
        assert_eq!(i128::from(even.to_bits()), 2);
        let away: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::HalfAwayFromZero).unwrap();
        assert_eq!(i128::from(away.to_bits()), 3);
        let ceil: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::Ceiling).unwrap();
        assert_eq!(i128::from(ceil.to_bits()), 3);
        let floor: D38<0> = D38::<0>::convert_from_with(src, RoundingMode::Floor).unwrap();
        assert_eq!(i128::from(floor.to_bits()), 2);
    }

    // ── narrow that fits (Ok, value preserved) ───────────────────────────

    /// Narrowing a value that fits the target storage at the same scale is
    /// exact and `Ok`.
    #[test]
    fn narrow_that_fits_preserves_value() {
        // 7.50 @ D38<2> -> D18<2>: comfortably fits i64.
        let src: D38<2> = d38_raw::<2>(750);
        let out: D18<2> = D18::<2>::convert_from(src).unwrap();
        assert_eq!(i128::from(out.to_bits()), 750);
    }

    // ── narrow that overflows (Err) ──────────────────────────────────────

    /// Narrowing a magnitude that does not fit the target storage at the
    /// requested scale returns `Err(Overflow)`.
    #[test]
    fn narrow_overflow_is_err() {
        // ~1e30 stored in D38<0> cannot fit i64 (max ~9.2e18) at scale 0.
        let big: i128 = 1_000_000_000_000_000_000_000_000_000_000; // 1e30
        let src: D38<0> = d38_raw::<0>(big);
        let out: Result<D18<0>, ConvertError> = D18::<0>::convert_from(src);
        assert_eq!(out, Err(ConvertError::Overflow));
    }

    // ── precision-preserving narrow with scale-down ──────────────────────

    /// The motivating case for rescaling at the SOURCE (wider) width before
    /// narrowing: a value that does NOT fit the target at the source scale,
    /// but DOES fit after the scale-down, must convert to `Ok` (not `Err`).
    ///
    /// Source: 5e19 @ D38<2> (logical 5e17). i64::MAX is ~9.2e18, so the
    /// raw 5e19 magnitude does not fit i64. Converting to D18<0> scales
    /// down by 10^2 -> raw 5e17, which DOES fit i64. If the narrow were
    /// done first this would spuriously overflow; the source-width-first
    /// ordering makes it succeed.
    #[test]
    fn narrow_with_scale_down_fits_after_rescale() {
        let raw_src: i128 = 50_000_000_000_000_000_000; // 5e19, > i64::MAX
        assert!(raw_src > i128::from(i64::MAX));
        let src: D38<2> = d38_raw::<2>(raw_src); // logical 5e17
        let out: D18<0> = D18::<0>::convert_from(src).unwrap();
        // 5e17 fits i64 (< 9.2e18).
        assert_eq!(i128::from(out.to_bits()), 500_000_000_000_000_000);
    }

    // ── same width, same scale (identity) ────────────────────────────────

    /// Same width and same scale is the identity (bit-for-bit).
    #[test]
    fn same_width_same_scale_is_identity() {
        let src: D38<6> = d38_raw::<6>(1_234_567);
        let out: D38<6> = D38::<6>::convert_from(src).unwrap();
        assert_eq!(i128::from(out.to_bits()), 1_234_567);

        // Cross-width but same scale, identity value.
        let narrow: D18<6> = d18_raw::<6>(9_999);
        let widened: D38<6> = D38::<6>::convert_from(narrow).unwrap();
        assert_eq!(i128::from(widened.to_bits()), 9_999);
    }

    // ── round-trip (lossless) ────────────────────────────────────────────

    /// A lossless widen followed by the inverse narrow recovers the original
    /// value. Widening up in scale + width then narrowing back is exact when
    /// no precision is dropped.
    #[test]
    fn round_trip_widen_then_narrow_is_lossless() {
        let original: D18<2> = d18_raw::<2>(4_242);
        // Widen to D38<6> (scale-up, exact), then narrow back to D18<2>.
        let wide: D38<6> = D38::<6>::convert_from(original).unwrap();
        let back: D18<2> = D18::<2>::convert_from(wide).unwrap();
        assert_eq!(i128::from(back.to_bits()), i128::from(original.to_bits()));
    }

    /// Negative values preserve sign through both branches.
    #[test]
    fn negative_values_round_correctly() {
        // -1.2345 @ D18<4> -> D38<2>: half-to-even, dropped 45 < 50 -> -1.23.
        let src: D18<4> = d18_raw::<4>(-12_345);
        let out: D38<2> = D38::<2>::convert_from(src).unwrap();
        assert_eq!(i128::from(out.to_bits()), -123);

        // Negative narrow that fits.
        let neg: D38<2> = d38_raw::<2>(-750);
        let narrowed: D18<2> = D18::<2>::convert_from(neg).unwrap();
        assert_eq!(i128::from(narrowed.to_bits()), -750);
    }
}

mod from_macros_surface {
    //! The conversion-surface blocks of the retired `tests/macros_surface.rs`
    //! (`from_*` constructors, the float bridge, and `TryFrom` narrowing). Its
    //! equality / overflow / num-traits / bitwise / foundation / trait blocks
    //! live in their own targets.

    use decimal_scaled::{D18, D38};

    // ─── macros/int_methods.rs: from_int / from_intN ───────────────────────

    #[test]
    fn from_int_narrow_signed() {
        assert_eq!(D18::<2>::from(100).to_bits(), 10_000);
        assert_eq!(D38::<2>::from(42).to_bits(), 4_200);
        // negative
        assert_eq!(D38::<2>::from(-5).to_bits(), -500);
    }

    #[test]
    fn from_primitive_paths_d38() {
        // D38 has impls for every primitive int type via decl_from_primitive.
        let _ = D38::<2>::from(7_i8);
        let _ = D38::<2>::from(7_i16);
        let _ = D38::<2>::from(7_i32);
        let _ = D38::<2>::from(7_i64);
        let _ = D38::<2>::from(7_u8);
        let _ = D38::<2>::from(7_u16);
        let _ = D38::<2>::from(7_u32);
        let _ = D38::<2>::from(7_u64);
        // i64 via from_int (D38's IntSrc is i64; i128 conversion is via TryFrom).
        assert_eq!(D38::<2>::from(0i64).to_bits(), 0);
        let _: D38<2> = i128::from(0i32).try_into().unwrap_or(D38::<2>::ZERO);
    }

    #[test]
    fn from_primitive_paths_d18() {
        let _ = D18::<2>::from(7_i8);
        let _ = D18::<2>::from(7_i16);
        let _ = D18::<2>::from(7_i32);
        let _ = D18::<2>::from(7_u8);
        let _ = D18::<2>::from(7_u16);
        let _ = D18::<2>::from(7_u32);
    }
    // ─── macros/float_bridge.rs: from_f32 / to_f32 / from_f64 / to_f64 ─────

    #[cfg(feature = "std")]
    #[test]
    fn float_bridge_narrow() {
        // f64
        assert_eq!(D38::<2>::from_f64(1.5).to_bits(), 150);
        assert_eq!(D38::<2>::ZERO.to_f64(), 0.0);
        // NaN saturates to ZERO
        assert_eq!(D38::<2>::from_f64(f64::NAN), D38::<2>::ZERO);
        // +inf saturates to MAX
        assert_eq!(D38::<2>::from_f64(f64::INFINITY), D38::<2>::MAX);
        // -inf saturates to MIN
        assert_eq!(D38::<2>::from_f64(f64::NEG_INFINITY), D38::<2>::MIN);

        // f64 bridge for narrow width
        assert_eq!(D18::<2>::from_f64(1.5).to_bits(), 150);
        assert_eq!(D18::<2>::from_f64(2.5).to_bits(), 250);

        // Out-of-range saturation
        assert_eq!(D18::<2>::from_f64(f64::INFINITY), D18::<2>::MAX);
        assert_eq!(D18::<2>::from_f64(f64::NEG_INFINITY), D18::<2>::MIN);
        assert_eq!(D18::<2>::from_f64(f64::NAN), D18::<2>::ZERO);

        // to_f32 / to_f64
        assert_eq!(D18::<2>::from(1).to_f32(), 1.0_f32);
        assert_eq!(D18::<2>::from(1).to_f64(), 1.0);

        // from_f64_with: rounding-mode-aware variant
        use decimal_scaled::RoundingMode;
        let v = D38::<2>::from_f64_with(1.5, RoundingMode::HalfToEven);
        assert_eq!(v.to_bits(), 150);
        let v = D38::<2>::from_f64_with(1.5, RoundingMode::Trunc);
        assert_eq!(v.to_bits(), 150);
    }
    // ─── macros/conversions.rs: TryFrom narrowing ──────────────────────────

    #[test]
    fn try_from_d38_to_d18_in_range() {
        let v = D38::<2>::from(5);
        let r: D18<2> = v.try_into().unwrap();
        assert_eq!(r.to_bits(), 500);
    }
}

mod from_macros_bitwise_and_overflow {
    //! The float-bridge rounding-mode block of the retired
    //! `tests/macros_bitwise_and_overflow.rs`; its bitwise and overflow blocks
    //! live in `api/bitwise.rs` and `contracts/overflow.rs`.

    use decimal_scaled::{D18, D38};

    // ─── float_bridge: from_f64 boundary cases and rounding modes ──────────

    #[cfg(feature = "std")]
    #[test]
    fn float_bridge_rounding_modes() {
        use decimal_scaled::RoundingMode;

        // Each mode at the half-LSB boundary.
        // At SCALE=2, 1.005 sits between 100 and 101 (half-LSB).
        for mode in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            // exercise the dispatch branch in from_f64_with for each mode.
            let _ = D38::<2>::from_f64_with(1.005, mode);
            let _ = D18::<2>::from_f64_with(1.005, mode);
            let _ = D18::<2>::from_f64_with(1.005, mode);
        }

        // Negative side
        let _ = D38::<2>::from_f64_with(-1.005, RoundingMode::Floor);
        let _ = D18::<2>::from_f64_with(-1.005, RoundingMode::Ceiling);

        // Exact zero
        assert_eq!(
            D38::<2>::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
            0
        );
        assert_eq!(
            D18::<2>::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
            0
        );
        assert_eq!(
            D18::<2>::from_f64_with(0.0, RoundingMode::HalfToEven).to_bits(),
            0
        );

        #[cfg(feature = "wide")]
        {
            use decimal_scaled::D76;

            for mode in [
                RoundingMode::HalfToEven,
                RoundingMode::HalfAwayFromZero,
                RoundingMode::HalfTowardZero,
                RoundingMode::Trunc,
                RoundingMode::Floor,
                RoundingMode::Ceiling,
            ] {
                let _ = D76::<2>::from_f64_with(1.005, mode);
            }
        }
    }
}
