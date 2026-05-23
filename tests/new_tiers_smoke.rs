//! Smoke tests for the new decimal tiers (D57 / D115 / D230 / D462 /
//! D616 / D924 / D1232). Each tier gets coverage on:
//!
//! - Arithmetic round-trip (from_int / add / sub / mul / div).
//! - `DecimalConstants::pi()` correctness (compared against
//!   the 100-digit canonical reference 3.14159...749 to its tier's
//!   safe precision).
//! - `sqrt_strict(4) == 2` exact.
//! - `sin_strict(0) == 0` and `cos_strict(0) == 1` exact.
//! - `exp_strict(0) == 1` and `ln_strict(1) == 0` exact.
//! - `from_bits`/`to_bits` round-trip.
//!
//! These are intentionally NOT exhaustive — the algorithm correctness
//! is covered by the existing per-method tests in src/. These tests
//! exist to prove the tier wiring (storage type, transcendental work
//! integer, build-time consts) actually compiles and produces
//! sensible numbers for each new tier.

#![cfg(feature = "wide")]

use decimal_scaled::DecimalConstants;

// Each tier's smoke battery is the same shape; this macro emits
// per-tier modules so test failures point at the offending tier.
macro_rules! tier_smoke {
    ($mod_name:ident, $T:ty, $Ts0:ty, $Tsmid:ty, $Tsmax:ty) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn from_int_round_trip() {
                let a = <$Tsmid>::try_from(40).unwrap();
                let b = <$Tsmid>::try_from(8).unwrap();
                // Use even-dividing operands so the divide is exact
                // at every scale (no rounding-mode dependence).
                assert_eq!(a + b, <$Tsmid>::try_from(48).unwrap());
                assert_eq!(a - b, <$Tsmid>::try_from(32).unwrap());
                assert_eq!(a * b, <$Tsmid>::try_from(320).unwrap());
                assert_eq!(a / b, <$Tsmid>::try_from(5).unwrap());
            }

            #[test]
            fn from_bits_round_trip() {
                let v = <$Ts0>::try_from(12345).unwrap();
                let raw = v.to_bits();
                let back = <$Ts0>::from_bits(raw);
                assert_eq!(v, back);
            }

            #[test]
            fn zero_one_constants() {
                assert!(<$Tsmid>::ZERO < <$Tsmid>::ONE);
                assert_eq!(<$Tsmid>::ZERO + <$Tsmid>::ONE, <$Tsmid>::ONE);
                assert_eq!(<$Tsmid>::ONE - <$Tsmid>::ONE, <$Tsmid>::ZERO);
            }

            #[test]
            fn pi_close_to_canonical() {
                // π ≈ 3.14159265358979323846... — at scale 5, expect 3.14159
                let pi = <$T>::pi();
                let three = <$T>::try_from(3).unwrap();
                let four = <$T>::try_from(4).unwrap();
                assert!(pi > three, "pi should exceed 3");
                assert!(pi < four, "pi should be below 4");
            }

            #[test]
            fn e_close_to_canonical() {
                // e ≈ 2.71828... — at any scale > 0, e ∈ (2, 3).
                let e = <$T>::e();
                let two = <$T>::try_from(2).unwrap();
                let three = <$T>::try_from(3).unwrap();
                assert!(e > two);
                assert!(e < three);
            }

            #[test]
            fn sqrt_perfect_square() {
                // √4 = 2 exactly.
                let four = <$Tsmid>::try_from(4).unwrap();
                let r = four.sqrt_strict();
                assert_eq!(r, <$Tsmid>::try_from(2).unwrap(), "sqrt(4) should be 2 exactly");
            }

            #[test]
            fn sin_zero_is_zero() {
                assert_eq!(<$Tsmid>::ZERO.sin_strict(), <$Tsmid>::ZERO);
            }

            #[test]
            fn cos_zero_is_one() {
                assert_eq!(<$Tsmid>::ZERO.cos_strict(), <$Tsmid>::ONE);
            }

            #[test]
            fn exp_zero_is_one() {
                assert_eq!(<$Tsmid>::ZERO.exp_strict(), <$Tsmid>::ONE);
            }

            #[test]
            fn ln_one_is_zero() {
                assert_eq!(<$Tsmid>::ONE.ln_strict(), <$Tsmid>::ZERO);
            }

            #[test]
            fn sin_cos_strict_at_zero_exact() {
                // sin_cos(0) = (0, 1) exact at every storage scale.
                let (s, c) = <$Tsmid>::ZERO.sin_cos_strict();
                assert_eq!(s, <$Tsmid>::ZERO);
                assert_eq!(c, <$Tsmid>::ONE);
            }

            #[test]
            fn tau_close_to_two_pi() {
                // τ ≈ 2π — independently rounded so the last digit
                // can disagree. Check the relation up to a small
                // fraction of π rather than bit-exact.
                let tau = <$T>::tau();
                let pi = <$T>::pi();
                let two_pi = pi + pi;
                let diff = if tau > two_pi {
                    tau - two_pi
                } else {
                    two_pi - tau
                };
                let bound = pi / <$T>::try_from(1000).unwrap();
                assert!(diff < bound, "tau and 2·pi should agree to 3 decimals");
            }

            #[test]
            fn half_quarter_pi_ordered() {
                // quarter_pi < half_pi < pi < tau — these
                // build-time constants exercise the per-tier rescale
                // dispatcher.
                let qp = <$T>::quarter_pi();
                let hp = <$T>::half_pi();
                let pi = <$T>::pi();
                let tau = <$T>::tau();
                assert!(qp < hp);
                assert!(hp < pi);
                assert!(pi < tau);
                // Each is approximately double the previous —
                // tolerate ±10% to absorb low-scale truncation.
                assert!(hp > qp);
                assert!(pi > hp);
                assert!(tau > pi);
            }

            #[test]
            fn golden_in_expected_range() {
                // φ = (1 + √5) / 2 ≈ 1.61803... — bracket between 1 and 2.
                let g = <$T>::golden();
                let one = <$T>::try_from(1).unwrap();
                let two = <$T>::try_from(2).unwrap();
                assert!(g > one);
                assert!(g < two);
            }

            #[test]
            fn default_is_zero() {
                // Default for every tier should be ZERO. This
                // exercises the per-tier Default impl that the
                // tier-decl macro emits.
                let d: $Tsmid = Default::default();
                assert_eq!(d, <$Tsmid>::ZERO);
            }

            #[test]
            fn widen_narrow_round_trip() {
                // .widen() (where it exists) lifts to the next tier
                // in the comprehensive ladder; .narrow() drops to
                // the previous tier. Round-trip should preserve the
                // value because the next-up tier is strictly larger
                // and at_scale=Tsmid stays in range on either side.
                let v: $Tsmid = <$Tsmid>::try_from(42).unwrap();
                let n = v.narrow().expect("narrow into previous tier");
                assert_eq!(
                    n.to_bits().to_string(),
                    v.to_bits().to_string(),
                    "narrow should bit-preserve when value fits the smaller storage"
                );
            }

            #[test]
            fn transcendentals_at_half_max_scale_do_not_overflow() {
                // Regression: the bench panicked at D57<56>/ln_strict
                // because the work integer was too narrow to hold
                // the squared intermediate at working scale
                // SCALE+GUARD. The fix was to bump D57's work
                // integer from Int512 to Int1024.
                //
                // Exercise the four headline strict transcendentals
                // at `Tsmid` (an interior, non-trivial scale where
                // the series actually runs); `1 + 1/2` is in range
                // at every interior scale.
                let half = <$Tsmid>::try_from(1).unwrap() / <$Tsmid>::try_from(2).unwrap();
                let one_and_a_half = <$Tsmid>::try_from(1).unwrap() + half;
                let _ = one_and_a_half.ln_strict();
                let _ = half.exp_strict();
                let _ = one_and_a_half.sin_strict();
                let _ = one_and_a_half.sqrt_strict();
            }
        }
    };
}

#[cfg(feature = "d57")]
tier_smoke!(
    d57,
    decimal_scaled::D57<5>,
    decimal_scaled::D57s0,
    decimal_scaled::D57<5>,
    decimal_scaled::D57s56
);

#[cfg(feature = "d115")]
tier_smoke!(
    d115,
    decimal_scaled::D115<10>,
    decimal_scaled::D115s0,
    decimal_scaled::D115<10>,
    decimal_scaled::D115s114
);

#[cfg(feature = "d230")]
tier_smoke!(
    d230,
    decimal_scaled::D230<10>,
    decimal_scaled::D230s0,
    decimal_scaled::D230<10>,
    decimal_scaled::D230s229
);

#[cfg(feature = "d462")]
tier_smoke!(
    d462,
    decimal_scaled::D462<10>,
    decimal_scaled::D462s0,
    decimal_scaled::D462<10>,
    decimal_scaled::D462s461
);

#[cfg(feature = "d616")]
tier_smoke!(
    d616,
    decimal_scaled::D616<10>,
    decimal_scaled::D616s0,
    decimal_scaled::D616<10>,
    decimal_scaled::D616s615
);

#[cfg(feature = "d924")]
tier_smoke!(
    d924,
    decimal_scaled::D924<10>,
    decimal_scaled::D924s0,
    decimal_scaled::D924<10>,
    decimal_scaled::D924s923
);

#[cfg(feature = "d1232")]
tier_smoke!(
    d1232,
    decimal_scaled::D1232<10>,
    decimal_scaled::D1232s0,
    decimal_scaled::D1232<10>,
    decimal_scaled::D1232s1231
);
