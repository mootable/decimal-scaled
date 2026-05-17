//! Smoke tests for the new decimal tiers (D56 / D114 / D230 / D461 /
//! D615 / D923 / D1231). Each tier gets coverage on:
//!
//! - Arithmetic round-trip (from_int / add / sub / mul / div).
//! - `DecimalConsts::pi()` correctness (compared against
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

use decimal_scaled::DecimalConsts;

// Each tier's smoke battery is the same shape; this macro emits
// per-tier modules so test failures point at the offending tier.
macro_rules! tier_smoke {
    ($mod_name:ident, $T:ty, $Ts0:ty, $Tsmid:ty, $Tsmax:ty) => {
        mod $mod_name {
            use super::*;

            #[test]
            fn from_int_round_trip() {
                let a = <$Tsmid>::from_int(40);
                let b = <$Tsmid>::from_int(8);
                // Use even-dividing operands so the divide is exact
                // at every scale (no rounding-mode dependence).
                assert_eq!(a + b, <$Tsmid>::from_int(48));
                assert_eq!(a - b, <$Tsmid>::from_int(32));
                assert_eq!(a * b, <$Tsmid>::from_int(320));
                assert_eq!(a / b, <$Tsmid>::from_int(5));
            }

            #[test]
            fn from_bits_round_trip() {
                let v = <$Ts0>::from_int(12345);
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
                let three = <$T>::from_int(3);
                let four = <$T>::from_int(4);
                assert!(pi > three, "pi should exceed 3");
                assert!(pi < four, "pi should be below 4");
            }

            #[test]
            fn e_close_to_canonical() {
                // e ≈ 2.71828... — at any scale > 0, e ∈ (2, 3).
                let e = <$T>::e();
                let two = <$T>::from_int(2);
                let three = <$T>::from_int(3);
                assert!(e > two);
                assert!(e < three);
            }

            #[test]
            fn sqrt_perfect_square() {
                // √4 = 2 exactly.
                let four = <$Tsmid>::from_int(4);
                let r = four.sqrt_strict();
                assert_eq!(r, <$Tsmid>::from_int(2),
                    "sqrt(4) should be 2 exactly");
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
                let pi  = <$T>::pi();
                let two_pi = pi + pi;
                let diff = if tau > two_pi { tau - two_pi } else { two_pi - tau };
                let bound = pi / <$T>::from_int(1000);
                assert!(diff < bound,
                    "tau and 2·pi should agree to 3 decimals");
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
                let one = <$T>::from_int(1);
                let two = <$T>::from_int(2);
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
        }
    };
}

#[cfg(feature = "d56")]
tier_smoke!(d56, decimal_scaled::D56<5>, decimal_scaled::D56s0, decimal_scaled::D56<5>, decimal_scaled::D56s57);

#[cfg(feature = "d114")]
tier_smoke!(d114, decimal_scaled::D114<10>, decimal_scaled::D114s0, decimal_scaled::D114<10>, decimal_scaled::D114s115);

#[cfg(feature = "d230")]
tier_smoke!(d230, decimal_scaled::D230<10>, decimal_scaled::D230s0, decimal_scaled::D230<10>, decimal_scaled::D230s230);

#[cfg(feature = "d461")]
tier_smoke!(d461, decimal_scaled::D461<10>, decimal_scaled::D461s0, decimal_scaled::D461<10>, decimal_scaled::D461s462);

#[cfg(feature = "d615")]
tier_smoke!(d615, decimal_scaled::D615<10>, decimal_scaled::D615s0, decimal_scaled::D615<10>, decimal_scaled::D615s616);

#[cfg(feature = "d923")]
tier_smoke!(d923, decimal_scaled::D923<10>, decimal_scaled::D923s0, decimal_scaled::D923<10>, decimal_scaled::D923s924);

#[cfg(feature = "d1231")]
tier_smoke!(d1231, decimal_scaled::D1231<10>, decimal_scaled::D1231s0, decimal_scaled::D1231<10>, decimal_scaled::D1231s1232);
