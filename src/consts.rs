//! Mathematical constants and float-compatibility constants for [`D128`].
//!
//! # Constants provided
//!
//! The [`DecimalConsts`] trait exposes `pi`, `tau`, `half_pi`,
//! `quarter_pi`, `golden`, and `e` as methods on `D128<SCALE>`.
//!
//! Two inherent associated constants, `EPSILON` and `MIN_POSITIVE`, are
//! provided as analogues to `f64::EPSILON` and `f64::MIN_POSITIVE` so
//! that generic code parameterised over numeric types continues to compile
//! when `T = D128<SCALE>`.
//!
//! # Precision strategy
//!
//! All constant values are derived from a 35-digit reference stored as a
//! raw `i128` at `SCALE_REF = 35`. They do not pass through `f64` at any
//! point. The rescale from `SCALE_REF` to the caller's `SCALE` uses
//! integer division with half-to-even rounding.
//!
//! Going through `f64` would cap precision at roughly 15-17 decimal digits
//! (f64 mantissa width). The raw-i128 path preserves up to 35 digits, which
//! exceeds every practical scale value.
//!
//! At `SCALE > SCALE_REF` (i.e. `SCALE > 35`) the constant is multiplied
//! up from the reference, so trailing digits are zero-extended and carry no
//! additional precision. At `SCALE = 38` the multiplication may overflow
//! `i128` for some constants; callers that need `SCALE > 35` should verify
//! that the result is in range.
//!
//! # Reference scale
//!
//! `SCALE_REF = 35` was chosen so that each constant fits in `i128` at
//! that scale (the largest value, `tau ~= 6.28e35`, is well under
//! `i128::MAX ~= 1.7e38`) while still providing more digits than any
//! practical caller `SCALE`. Each raw constant is the half-to-even
//! rounding of the canonical decimal expansion to 35 fractional digits.
//! Sources: ISO 80000-2 (pi, tau, pi/2, pi/4), OEIS A001113 (e),
//! OEIS A001622 (golden ratio).

use crate::core_type::D128;

/// Reference scale for the high-precision raw constants below.
///
/// Every constant fits in `i128` at this scale; the largest (tau ~= 6.28e35)
/// is well under `i128::MAX ~= 1.7e38`. Caller scales above this value are
/// handled by multiplying the reference by `10^(SCALE - SCALE_REF)`.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const SCALE_REF: u32 = 35;

// Raw i128 constants at SCALE_REF = 35.
//
// Each value is the half-to-even rounding of the canonical decimal
// expansion to 35 fractional digits. Sources: ISO 80000-2 (pi, tau, pi/2,
// pi/4), OEIS A001113 (e), OEIS A001622 (golden = (1 + sqrt(5)) / 2).

/// pi at SCALE_REF = 35.
/// Value: 3.14159265358979323846264338327950288
/// (36th digit was 4; no round-up.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const PI_RAW_S35: i128 = 314_159_265_358_979_323_846_264_338_327_950_288_i128;

/// tau = 2 * pi at SCALE_REF = 35.
/// Value: 6.28318530717958647692528676655900577
/// (36th digit was 8; rounded up from ...576 to ...577.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const TAU_RAW_S35: i128 = 628_318_530_717_958_647_692_528_676_655_900_577_i128;

/// pi / 2 at SCALE_REF = 35.
/// Value: 1.57079632679489661923132169163975144
/// (36th digit was 2; no round-up.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const HALF_PI_RAW_S35: i128 = 157_079_632_679_489_661_923_132_169_163_975_144_i128;

/// pi / 4 at SCALE_REF = 35.
/// Value: 0.78539816339744830961566084581987572
/// (36th digit was 1; no round-up.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const QUARTER_PI_RAW_S35: i128 = 78_539_816_339_744_830_961_566_084_581_987_572_i128;

/// e at SCALE_REF = 35.
/// Value: 2.71828182845904523536028747135266250
/// (36th digit was 7; rounded up from ...249 to ...250.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const E_RAW_S35: i128 = 271_828_182_845_904_523_536_028_747_135_266_250_i128;

/// Golden ratio = (1 + sqrt(5)) / 2 at SCALE_REF = 35.
/// Value: 1.61803398874989484820458683436563812
/// (36th digit was 7; rounded up from ...811 to ...812.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const GOLDEN_RAW_S35: i128 = 161_803_398_874_989_484_820_458_683_436_563_812_i128;

// Rescale helper (half-to-even).

/// Rescales `raw` from `SCALE_REF` to `target_scale` using round-half-to-even
/// (IEEE-754 default; banker's rounding).
///
/// - Equal scales: returns `raw` unchanged.
/// - `target_scale < SCALE_REF`: integer division with round-half-to-even
///   rounding. Exact half-LSB values round to the even neighbour, eliminating
///   the systematic upward magnitude bias of the half-away-from-zero rule.
/// - `target_scale > SCALE_REF`: integer multiplication; panics on overflow in
///   debug builds (only reachable at `SCALE >= 38` for large constants).
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline]
fn rescale_from_ref(raw: i128, target_scale: u32) -> i128 {
    if target_scale == SCALE_REF {
        return raw;
    }
    if target_scale < SCALE_REF {
        let shift = SCALE_REF - target_scale;
        let divisor = 10i128.pow(shift);
        let quotient = raw / divisor;
        let remainder = raw % divisor;
        let abs_rem = remainder.unsigned_abs();
        let half = (divisor / 2) as u128;

        if abs_rem < half {
            quotient
        } else if abs_rem > half {
            if raw >= 0 { quotient + 1 } else { quotient - 1 }
        } else if quotient % 2 == 0 {
            quotient
        } else if raw >= 0 {
            quotient + 1
        } else {
            quotient - 1
        }
    } else {
        let shift = target_scale - SCALE_REF;
        raw * 10i128.pow(shift)
    }
}

/// Well-known mathematical constants available on any [`D128<SCALE>`].
///
/// Import this trait to call `D128s12::pi()`, `D128s12::e()`, etc.
///
/// All returned values are computed from a 35-digit raw-`i128` reference
/// without passing through `f64`. The result is bit-exact at the target
/// `SCALE` for every supported scale up to `SCALE = 35`.
pub trait DecimalConsts: Sized {
    /// Pi (~3.14159265...). One half-turn in radians.
    ///
    /// Source: ISO 80000-2 / OEIS A000796. 35-digit reference rescaled to
    /// `SCALE` with half-to-even rounding.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn pi() -> Self;

    /// Tau (~6.28318530...). One full turn in radians.
    ///
    /// Defined as `2 * pi`. 35-digit reference rescaled to `SCALE` with
    /// half-to-even rounding.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn tau() -> Self;

    /// Half-pi (~1.57079632...). One quarter-turn in radians.
    ///
    /// Defined as `pi / 2`. 35-digit reference rescaled to `SCALE` with
    /// half-to-even rounding.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn half_pi() -> Self;

    /// Quarter-pi (~0.78539816...). One eighth-turn in radians.
    ///
    /// Defined as `pi / 4`. 35-digit reference rescaled to `SCALE` with
    /// half-to-even rounding.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn quarter_pi() -> Self;

    /// The golden ratio (~1.61803398...). Dimensionless.
    ///
    /// Defined as `(1 + sqrt(5)) / 2`. Source: OEIS A001622. 35-digit
    /// reference rescaled to `SCALE` with half-to-even rounding.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn golden() -> Self;

    /// Euler's number (~2.71828182...). Dimensionless.
    ///
    /// Source: OEIS A001113. 35-digit reference rescaled to `SCALE` with
    /// half-to-even rounding.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn e() -> Self;
}

impl<const SCALE: u32> DecimalConsts for D128<SCALE> {
    #[inline]
    fn pi() -> Self {
        Self(rescale_from_ref(PI_RAW_S35, SCALE))
    }

    #[inline]
    fn tau() -> Self {
        Self(rescale_from_ref(TAU_RAW_S35, SCALE))
    }

    #[inline]
    fn half_pi() -> Self {
        Self(rescale_from_ref(HALF_PI_RAW_S35, SCALE))
    }

    #[inline]
    fn quarter_pi() -> Self {
        Self(rescale_from_ref(QUARTER_PI_RAW_S35, SCALE))
    }

    #[inline]
    fn golden() -> Self {
        Self(rescale_from_ref(GOLDEN_RAW_S35, SCALE))
    }

    #[inline]
    fn e() -> Self {
        Self(rescale_from_ref(E_RAW_S35, SCALE))
    }
}

// Inherent associated constants: EPSILON / MIN_POSITIVE.
//
// These mirror `f64::EPSILON` and `f64::MIN_POSITIVE` so that generic
// numeric code that calls `T::EPSILON` or `T::MIN_POSITIVE` compiles
// when `T = D128<SCALE>`. For D128 both equal `D128(1)` -- the smallest
// representable positive value (1 LSB = 10^-SCALE). There are no subnormals.

impl<const SCALE: u32> D128<SCALE> {
    /// Smallest representable positive value: 1 LSB = `10^-SCALE`.
    ///
    /// Provided as an analogue to `f64::EPSILON` for generic numeric code.
    /// Note that this differs from the f64 definition ("difference between
    /// 1.0 and the next-larger f64"): for `D128` the LSB is uniform across
    /// the entire representable range.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const EPSILON: Self = Self(1);

    /// Smallest positive value (equal to [`Self::EPSILON`]).
    ///
    /// Provided as an analogue to `f64::MIN_POSITIVE` for generic numeric
    /// code. Unlike `f64`, `D128` has no subnormals, so `MIN_POSITIVE`
    /// and `EPSILON` are the same value.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const MIN_POSITIVE: Self = Self(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::D128s12;

    // Bit-exact assertions at SCALE = 12.
    //
    // At SCALE = 12 each constant is the 35-digit raw integer divided by
    // 10^23, rounded half-to-even.

    /// pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 3_141_592_653_589.
    /// 14th digit is 7 (from position 14 of the raw) -> round up.
    /// Expected: 3_141_592_653_590.
    #[test]
    fn pi_is_bit_exact_at_scale_12() {
        assert_eq!(D128s12::pi().to_bits(), 3_141_592_653_590_i128);
    }

    /// tau at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 6_283_185_307_179.
    /// 14th digit is 5 -> round up. Expected: 6_283_185_307_180.
    #[test]
    fn tau_is_bit_exact_at_scale_12() {
        assert_eq!(D128s12::tau().to_bits(), 6_283_185_307_180_i128);
    }

    /// half_pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_570_796_326_794.
    /// 14th digit is 8 -> round up. Expected: 1_570_796_326_795.
    #[test]
    fn half_pi_is_bit_exact_at_scale_12() {
        assert_eq!(D128s12::half_pi().to_bits(), 1_570_796_326_795_i128);
    }

    /// quarter_pi at SCALE=12: raw / 10^23.
    /// Truncated 12 digits: 785_398_163_397.
    /// 13th digit is 4 -> no round-up. Expected: 785_398_163_397.
    #[test]
    fn quarter_pi_is_bit_exact_at_scale_12() {
        assert_eq!(D128s12::quarter_pi().to_bits(), 785_398_163_397_i128);
    }

    /// e at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 2_718_281_828_459.
    /// 14th digit is 0 -> no round-up. Expected: 2_718_281_828_459.
    #[test]
    fn e_is_bit_exact_at_scale_12() {
        assert_eq!(D128s12::e().to_bits(), 2_718_281_828_459_i128);
    }

    /// golden at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_618_033_988_749.
    /// 14th digit is 8 -> round up. Expected: 1_618_033_988_750.
    #[test]
    fn golden_is_bit_exact_at_scale_12() {
        assert_eq!(D128s12::golden().to_bits(), 1_618_033_988_750_i128);
    }

    // Closeness checks against core::f64::consts.
    // These verify that the correct reference digits were selected; the
    // bit-exact tests above are the primary acceptance criteria.

    /// pi() converted to f64 is within 1e-11 of `core::f64::consts::PI`.
    /// At SCALE=12, 1 LSB = 1e-12, so 1e-11 covers rescale rounding plus
    /// the f64 conversion step.
    #[test]
    fn pi_close_to_f64_pi() {
        let diff = (D128s12::pi().to_f64_lossy() - core::f64::consts::PI).abs();
        assert!(diff < 1e-11, "pi diverges from f64 PI by {diff}");
    }

    #[test]
    fn tau_close_to_f64_tau() {
        let diff = (D128s12::tau().to_f64_lossy() - core::f64::consts::TAU).abs();
        assert!(diff < 1e-11, "tau diverges from f64 TAU by {diff}");
    }

    #[test]
    fn half_pi_close_to_f64_frac_pi_2() {
        let diff =
            (D128s12::half_pi().to_f64_lossy() - core::f64::consts::FRAC_PI_2).abs();
        assert!(diff < 1e-11, "half_pi diverges from f64 FRAC_PI_2 by {diff}");
    }

    #[test]
    fn quarter_pi_close_to_f64_frac_pi_4() {
        let diff =
            (D128s12::quarter_pi().to_f64_lossy() - core::f64::consts::FRAC_PI_4).abs();
        assert!(
            diff < 1e-11,
            "quarter_pi diverges from f64 FRAC_PI_4 by {diff}"
        );
    }

    #[test]
    fn e_close_to_f64_e() {
        let diff = (D128s12::e().to_f64_lossy() - core::f64::consts::E).abs();
        assert!(diff < 1e-11, "e diverges from f64 E by {diff}");
    }

    /// golden() converted to f64 is within 1e-11 of the closed form
    /// `(1 + sqrt(5)) / 2`. Requires std for `f64::sqrt`.
    #[cfg(feature = "std")]
    #[test]
    fn golden_close_to_closed_form() {
        let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
        let diff = (D128s12::golden().to_f64_lossy() - expected).abs();
        assert!(diff < 1e-11, "golden diverges from closed-form by {diff}");
    }

    // EPSILON / MIN_POSITIVE

    #[test]
    fn epsilon_is_one_ulp() {
        assert_eq!(D128s12::EPSILON.to_bits(), 1_i128);
        assert!(D128s12::EPSILON > D128s12::ZERO);
    }

    #[test]
    fn min_positive_is_one_ulp() {
        assert_eq!(D128s12::MIN_POSITIVE.to_bits(), 1_i128);
        assert_eq!(D128s12::MIN_POSITIVE, D128s12::EPSILON);
    }

    /// At SCALE = 6 the LSB is 10^-6; EPSILON is still raw 1.
    #[test]
    fn epsilon_at_scale_6_is_one_ulp() {
        type D6 = D128<6>;
        assert_eq!(D6::EPSILON.to_bits(), 1_i128);
        assert_eq!(D6::MIN_POSITIVE.to_bits(), 1_i128);
    }

    // Cross-scale exercises

    /// At SCALE = 6, pi() should equal 3.141593 (rounded half-to-even from
    /// 3.1415926535...). Expected raw bits: 3_141_593.
    #[test]
    fn pi_at_scale_6_is_bit_exact() {
        type D6 = D128<6>;
        assert_eq!(D6::pi().to_bits(), 3_141_593_i128);
    }

    /// At SCALE = 0, pi() rounds to 3 (first fractional digit is 1, no
    /// round-up).
    #[test]
    fn pi_at_scale_0_is_three() {
        type D0 = D128<0>;
        assert_eq!(D0::pi().to_bits(), 3_i128);
    }

    /// At SCALE = SCALE_REF (35), pi() returns exactly the raw constant.
    #[test]
    fn pi_at_scale_ref_is_raw_constant() {
        type D35 = D128<35>;
        assert_eq!(D35::pi().to_bits(), PI_RAW_S35);
    }

    /// At SCALE = SCALE_REF + 1 (36), pi() multiplies by 10, appending
    /// one trailing zero digit.
    #[test]
    fn pi_at_scale_36_multiplies_raw_by_ten() {
        type D36 = D128<36>;
        assert_eq!(D36::pi().to_bits(), PI_RAW_S35 * 10);
    }

    /// Negative-side rounding: negating pi gives the expected raw bits.
    #[test]
    fn neg_pi_round_trip() {
        let pi = D128s12::pi();
        let neg_pi = -pi;
        assert_eq!(neg_pi.to_bits(), -3_141_592_653_590_i128);
    }

    /// Round-half-to-even: at the exact half boundary, ties round to the
    /// even neighbour. 0.5 -> 0 (even), 1.5 -> 2, 2.5 -> 2 (even),
    /// 3.5 -> 4, -0.5 -> 0, -1.5 -> -2.
    #[test]
    fn rescale_half_to_even_at_boundary() {
        // SCALE_REF = 35; choosing target_scale = 34 shifts by one digit so
        // the divisor is 10 and `5` is exactly half.
        assert_eq!(rescale_from_ref(5, 34), 0);
        assert_eq!(rescale_from_ref(15, 34), 2);
        assert_eq!(rescale_from_ref(25, 34), 2);
        assert_eq!(rescale_from_ref(35, 34), 4);
        assert_eq!(rescale_from_ref(45, 34), 4);
        assert_eq!(rescale_from_ref(-5, 34), 0);
        assert_eq!(rescale_from_ref(-15, 34), -2);
        assert_eq!(rescale_from_ref(-25, 34), -2);
        assert_eq!(rescale_from_ref(-35, 34), -4);
    }

    /// Round-half-to-even: non-half remainders go to the nearest neighbour
    /// regardless of parity.
    #[test]
    fn rescale_non_half_rounds_nearest() {
        assert_eq!(rescale_from_ref(4, 34), 0);
        assert_eq!(rescale_from_ref(6, 34), 1);
        assert_eq!(rescale_from_ref(14, 34), 1);
        assert_eq!(rescale_from_ref(16, 34), 2);
        assert_eq!(rescale_from_ref(-4, 34), 0);
        assert_eq!(rescale_from_ref(-6, 34), -1);
        assert_eq!(rescale_from_ref(-14, 34), -1);
        assert_eq!(rescale_from_ref(-16, 34), -2);
    }
}
