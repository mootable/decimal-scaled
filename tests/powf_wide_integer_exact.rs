//! Wide-tier `powf` exact integer-power directed-rounding gate.
//!
//! When the base and exponent are exact integers and the result terminates
//! (`base = 2^a·5^b`), `base^-k` lands exactly on a storage grid line, so
//! every rounding mode must return that exact value. The wide
//! `powf_strict_with` integer fast path used to compute the reciprocal as
//! `ONE.div_with(self.checked_pow(|k|), mode)`, and `checked_pow` is a
//! *decimal* power: at a near-max scale `base^|k| · 10^SCALE` overflows
//! storage, so it returned `None` and the shell DEFERRED to the to-nearest
//! `exp(k·ln base)` composition — which lands ~1 ULP low, mis-rounding Floor
//! and Trunc by one LSB. The exact integer pin divides `10^SCALE` by the
//! INTEGER `base^|k|`, so the reciprocal is exact even when the scaled power
//! overflows.
//!
//! Oracle: the exact raw value `10^SCALE / base^|k|` is computed by direct
//! integer division of the work integer (independent of the `powf`
//! composition); the division is exact because the divisor divides `10^SCALE`
//! for every terminating case here. Scales are `MAX_SCALE − 1` per tier — the
//! band edge where the scaled power overflows but the bases (≤ 40) and the
//! sub-1 results remain representable.

#![cfg(feature = "wide")]

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
