//! Wide-tier `DecimalConstants` coverage.
//! Migrated from `tests/wide_constants_all_six.rs` and
//! `tests/wide_constants_high_scale.rs`.

#[cfg(feature = "wide")]
mod from_wide_constants_all_six {
    //! Coverage suite for `consts_wide.rs` — all six wide-tier constants
    //! (`pi`, `tau`, `half_pi`, `quarter_pi`, `golden`, `e`) on D76 / D153
    //! / D307 at multiple scales.
    //!
    //! The existing `wide_constants_high_scale.rs` covers π and one high
    //! scale per tier. This file exercises every constant on every wide
    //! tier at the canonical reference scale and at a small storage scale.
    //! Every test asserts the additive identities between the constants
    //! (`τ = π + π`, `π = π/2 + π/2`, `π/2 = π/4 + π/4`, within 1 LSB to
    //! absorb the independent per-constant rounding) plus coarse magnitude
    //! bounds for `e` and `golden` — digit-exact values are the golden
    //! gate's job, identities and reachability are this file's.

    use decimal_scaled::DecimalConstants;

    /// Assert the six constants' additive identities + magnitude bounds at one
    /// `(tier, scale)` cell. `$one_bits` builds the 1-LSB witness from raw storage.
    macro_rules! check_constants {
        ($D:ty, $Int:ty) => {{
            type D = $D;
            let one_lsb = <D>::from_bits(<$Int>::try_from(1_i128).unwrap());
            let within_one_lsb = |a: D, b: D, what: &str| {
                let diff = if a > b { a - b } else { b - a };
                assert!(diff <= one_lsb, "{what}: diff = {diff:?}");
            };
            within_one_lsb(D::tau(), D::pi() + D::pi(), "tau vs pi + pi");
            within_one_lsb(D::pi(), D::half_pi() + D::half_pi(), "pi vs 2 half_pi");
            within_one_lsb(D::half_pi(), D::quarter_pi() + D::quarter_pi(), "half_pi vs 2 quarter_pi");
            let two: D = "2".parse().unwrap();
            let three: D = "3".parse().unwrap();
            let one: D = "1".parse().unwrap();
            assert!(D::e() > two && D::e() < three, "e in (2, 3)");
            assert!(D::golden() > one && D::golden() < two, "golden in (1, 2)");
        }};
    }

    #[test]
    fn d76_all_six_constants_at_scale_12() {
        check_constants!(decimal_scaled::D76<12>, decimal_scaled::Int<4>);
    }

    #[test]
    fn d76_all_six_constants_at_scale_37() {
        check_constants!(decimal_scaled::D76<37>, decimal_scaled::Int<4>);
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d153_all_six_constants() {
        check_constants!(decimal_scaled::D153<37>, decimal_scaled::Int<8>);
        // The canonical reference scale (S = 152).
        check_constants!(decimal_scaled::D153<152>, decimal_scaled::Int<8>);
    }

    #[cfg(feature = "x-wide")]
    #[test]
    fn d307_all_six_constants() {
        check_constants!(decimal_scaled::D307<37>, decimal_scaled::Int<16>);
        check_constants!(decimal_scaled::D307<306>, decimal_scaled::Int<16>);
    }
}

#[cfg(all(feature = "wide", not(any(feature = "rounding-half-away-from-zero", feature = "rounding-half-toward-zero", feature = "rounding-trunc", feature = "rounding-floor", feature = "rounding-ceiling"))))]
mod from_wide_constants_high_scale {
    //! Verifies the per-width raw constants in `consts_wide.rs` produce
    //! correct values at the wide tiers' deeper scales — the case that
    //! previously panicked on the rescale-up `i128` overflow.

    // Truth strings below are the half-to-even-rounded pi reference; gate
    // the module to the default rounding mode so every test always asserts.
    use decimal_scaled::{D76, D153, D307, DecimalConstants};

    /// D76<76>::pi() used to panic at the i128 rescale-up. After wiring
    /// the build-time-generated 75-digit Int256 constants, it returns a
    /// well-defined value.
    #[test]
    fn d76_pi_at_max_scale_does_not_panic() {
        // SCALE=50: deeper than D38 but inside D76's max of 76.
        let pi50 = D76::<50>::pi();
        // Sanity: roughly 3 in integer part.
        assert!(pi50.to_bits().to_string().starts_with("314"));
    }

    #[test]
    fn d76_pi_at_scale_75_is_exact() {
        // At SCALE = SCALE_REF (75), pi() returns the raw constant
        // exactly — no rescaling.
        let pi75 = D76::<75>::pi();
        let s = pi75.to_bits().to_string();
        // First few significant digits of pi (no decimal point in raw).
        assert!(
            s.starts_with("3141592653589793238462643383279502884"),
            "got {s}"
        );
    }

    #[test]
    fn d153_pi_at_scale_152_works() {
        // v0.4.0 cap: MAX_SCALE for D153 is 152.
        let pi = D153::<152>::pi();
        let s = pi.to_bits().to_string();
        assert!(
            s.starts_with("3141592653589793238462643383279502884"),
            "got {s}"
        );
    }

    #[test]
    fn d307_pi_at_scale_300_works() {
        let pi = D307::<300>::pi();
        let s = pi.to_bits().to_string();
        assert!(
            s.starts_with("3141592653589793238462643383279502884"),
            "got {s}"
        );
    }

    /// Cross-tier check: D76<37> and D38<37>::pi() should produce the
    /// same logical value (the storage layouts differ but the rescaled
    /// integer agrees). Uses the public `Decimal` trait to bridge.
    #[test]
    fn d76_pi_at_scale_37_matches_d38() {
        use decimal_scaled::D38;
        let n = i128::from(D38::<37>::pi().to_bits());
        let w = D76::<37>::pi().to_bits();
        let w_as_i128 = w.to_i128_checked().expect("fits");
        let diff = (w_as_i128 - n).abs();
        assert!(diff <= 1, "D76<37>::pi {w_as_i128} vs D38<37>::pi {n}");
    }
}
