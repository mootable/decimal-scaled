//! The f64-bridge `*_fast` transcendental surface.
//! Migrated from `tests/narrow_fast_transcendentals.rs`.

#[cfg(feature = "std")]
mod from_narrow_fast_transcendentals {
    //! Coverage suite for `macros/fast_transcendentals.rs` — the f64-bridge
    //! transcendental surface emitted for every non-D38 width (D18 and
    //! the wide tiers).
    //!
    //! These methods are now always-callable via the explicit `*_fast` form
    //! when `feature = "std"` is on, regardless of strict/fast mode. The
    //! plain `*` dispatcher resolves to them under
    //! `any(not(strict), fast)` — both call paths are exercised here so the
    //! routing fix stays covered.
    //!
    //! Accuracy contract: f64 round-trip introduces ~1 LSB of quantisation
    //! noise per `to_f64` / `from_f64`, and the transcendental itself can
    //! introduce another LSB or two. At SCALE=8 (D18) the LSB is 10⁻⁸, well
    //! above f64 noise, so the test tolerance is wide enough to never be a
    //! source of false failure. We just check the macro-emitted bodies are
    //! reachable and produce sensible results.

    use decimal_scaled::{D18};

    /// Loose tolerance for the f64-bridge: 4 LSB at S=4 is 4·10⁻⁴ ≈ 4e-4,
    /// orders of magnitude above the f64 quantisation noise.
    const D18_TOL: i64 = 64;

    #[track_caller]
    fn close_d18(label: &str, actual: D18<8>, expected_bits: i64) {
        let diff = (i128::from(actual.to_bits()) - i128::from(expected_bits)).abs();
        assert!(
            diff <= i128::from(D18_TOL),
            "{label}: bits {} vs expected {expected_bits} (diff {diff} > {D18_TOL})",
            i128::from(actual.to_bits())
        );
    }

    #[test]
    fn d18_logs_exps_fast() {
        assert_eq!(D18::<8>::ONE.ln_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ZERO.exp_fast().to_bits(), 100_000_000);
        close_d18("ln(2)", D18::<8>::try_from(2).unwrap().ln_fast(), 69_314_718);
        close_d18("exp(1)", D18::<8>::ONE.exp_fast(), 271_828_183);
        assert_eq!(D18::<8>::try_from(2).unwrap().log2_fast().to_bits(), 100_000_000);
        assert_eq!(D18::<8>::try_from(10).unwrap().log10_fast().to_bits(), 100_000_000);
        assert_eq!(D18::<8>::try_from(10).unwrap().exp2_fast().to_bits(), 102_400_000_000);
    }

    #[test]
    fn d18_roots_pow_fast() {
        assert_eq!(D18::<8>::try_from(4).unwrap().sqrt_fast().to_bits(), 200_000_000);
        assert_eq!(D18::<8>::try_from(27).unwrap().cbrt_fast().to_bits(), 300_000_000);
        close_d18(
            "2^10",
            D18::<8>::try_from(2).unwrap().powf_fast(D18::<8>::try_from(10).unwrap()),
            102_400_000_000,
        );
        close_d18(
            "hypot(3,4)",
            D18::<8>::try_from(3).unwrap().hypot_fast(D18::<8>::try_from(4).unwrap()),
            500_000_000,
        );
        close_d18(
            "log_2(8)",
            D18::<8>::try_from(8).unwrap().log_fast(D18::<8>::try_from(2).unwrap()),
            300_000_000,
        );
    }

    #[test]
    fn d18_trig_inverse_hyperbolic_fast() {
        assert_eq!(D18::<8>::ZERO.sin_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ZERO.cos_fast().to_bits(), 100_000_000);
        close_d18("sin(1)", D18::<8>::ONE.sin_fast(), 84_147_098);
        close_d18("atan(1)", D18::<8>::ONE.atan_fast(), 78_539_816);
        close_d18("atan2(1,1)", D18::<8>::ONE.atan2_fast(D18::<8>::ONE), 78_539_816);
        assert_eq!(D18::<8>::ZERO.tan_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ZERO.asin_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ONE.acos_fast().to_bits(), 0);
        close_d18("sinh(1)", D18::<8>::ONE.sinh_fast(), 117_520_119);
        close_d18("cosh(1)", D18::<8>::ONE.cosh_fast(), 154_308_063);
        close_d18("tanh(1)", D18::<8>::ONE.tanh_fast(), 76_159_416);
        assert_eq!(D18::<8>::ZERO.asinh_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ONE.acosh_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ZERO.atanh_fast().to_bits(), 0);
    }

    #[test]
    fn d18_angle_conversion_fast() {
        assert_eq!(D18::<8>::ZERO.to_degrees_fast().to_bits(), 0);
        assert_eq!(D18::<8>::ZERO.to_radians_fast().to_bits(), 0);
    }
}
