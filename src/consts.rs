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
//! All constant values are derived from a 37-digit reference stored as a
//! raw `i128` at `SCALE_REF = 37`. They do not pass through `f64` at any
//! point. The rescale from `SCALE_REF` to the caller's `SCALE` uses
//! integer division with the crate-default [`RoundingMode`] (half-to-even
//! by default; overridable via the `rounding-*` Cargo features).
//!
//! Going through `f64` would cap precision at roughly 15-17 decimal digits
//! (f64 mantissa width). The raw-i128 path preserves up to 37 digits, which
//! exceeds every practical scale value.
//!
//! At `SCALE > SCALE_REF` (i.e. `SCALE > 37`) the constant is multiplied
//! up from the reference, so trailing digits are zero-extended and carry no
//! additional precision. At `SCALE = 38` the multiplication may overflow
//! `i128` for some constants; callers that need `SCALE > 37` should verify
//! that the result is in range.
//!
//! # Why `SCALE_REF = 37`?
//!
//! The maximum `SCALE_REF` that keeps every constant in `i128`. The
//! largest constant, `tau ≈ 6.28...`, lands at ~6.28×10³⁷ — still
//! below `i128::MAX ≈ 1.7×10³⁸`. `SCALE_REF = 38` would overflow
//! `tau`/`pi`/`e`/`golden`. The smaller constants `half_pi` and
//! `quarter_pi` could individually hold 38 frac digits, but a single
//! shared `SCALE_REF` keeps the rescale helpers uniform; the
//! per-constant precision gain (1 digit, on two constants) is not
//! worth six independent rescale paths.
//!
//! At `SCALE_REF = 37` the constants are accurate to within 0.5 ULP
//! for every `SCALE ≤ 37`. At `SCALE = 38` (the D128 maximum) the
//! result is off by up to ≈ 5 ULP — three of the supported scales
//! would need a raw constant wider than `i128` can hold. Tightening
//! to 0.5 ULP at `SCALE = 38` (and for the wide tiers' deeper
//! scales) requires per-width raw constants stored at the storage
//! type's maximum precision; recorded as a follow-up.
//!
//! [`RoundingMode`]: crate::rounding::RoundingMode
//!
//! # Sources
//!
//! Each raw constant is the half-to-even rounding of the canonical
//! decimal expansion to 37 fractional digits. ISO 80000-2 (pi, tau,
//! pi/2, pi/4), OEIS A001113 (e), OEIS A001622 (golden ratio).

use crate::core_type::D128;

/// Reference scale for the high-precision raw constants below.
///
/// Every constant fits in `i128` at this scale; the largest
/// (tau ≈ 6.28×10³⁷) is below `i128::MAX ≈ 1.7×10³⁸`. Caller scales
/// above this value rescale up by `10^(SCALE - SCALE_REF)`, which
/// appends placeholder zeros without adding precision — `SCALE = 38`
/// loses up to ≈ 5 ULP this way. Caller scales at or below
/// `SCALE_REF` rescale down via the crate-default [`RoundingMode`],
/// preserving the 0.5 ULP contract.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
///
/// [`RoundingMode`]: crate::rounding::RoundingMode
const SCALE_REF: u32 = 37;

// Raw i128 constants at SCALE_REF = 37.
//
// Each value is the half-to-even rounding of the canonical decimal
// expansion to 37 fractional digits. Sources: ISO 80000-2 (pi, tau, pi/2,
// pi/4), OEIS A001113 (e), OEIS A001622 (golden = (1 + sqrt(5)) / 2).

/// pi at SCALE_REF = 37.
/// Value: 3.1415926535897932384626433832795028842
/// (38th digit was 9; rounded up from ...841 to ...842.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const PI_RAW_S37: i128 = 31_415_926_535_897_932_384_626_433_832_795_028_842_i128;

/// tau = 2 * pi at SCALE_REF = 37.
/// Value: 6.2831853071795864769252867665590057684
/// (38th digit was 9; rounded up from ...683 to ...684.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const TAU_RAW_S37: i128 = 62_831_853_071_795_864_769_252_867_665_590_057_684_i128;

/// pi / 2 at SCALE_REF = 37.
/// Value: 1.5707963267948966192313216916397514421
/// (38th digit was 9; rounded up from ...420 to ...421.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const HALF_PI_RAW_S37: i128 = 15_707_963_267_948_966_192_313_216_916_397_514_421_i128;

/// pi / 4 at SCALE_REF = 37.
/// Value: 0.7853981633974483096156608458198757210
/// (38th digit was 4; no round-up.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const QUARTER_PI_RAW_S37: i128 = 7_853_981_633_974_483_096_156_608_458_198_757_210_i128;

/// e at SCALE_REF = 37.
/// Value: 2.7182818284590452353602874713526624978
/// (38th digit was 5, 37th was 7 odd → round to even up: ...977 → ...978.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const E_RAW_S37: i128 = 27_182_818_284_590_452_353_602_874_713_526_624_978_i128;

/// Golden ratio = (1 + sqrt(5)) / 2 at SCALE_REF = 37.
/// Value: 1.6180339887498948482045868343656381177
/// (38th digit was 2; no round-up.)
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const GOLDEN_RAW_S37: i128 = 16_180_339_887_498_948_482_045_868_343_656_381_177_i128;

// Rescaling from SCALE_REF to the caller's SCALE is delegated to
// `D128::rescale` (which uses round-half-to-even by default; see
// `src/rescale.rs`). The constants below construct a `D128<SCALE_REF>`
// from the raw integer literal and then rescale to the caller's
// `D128<SCALE>`.

/// Well-known mathematical constants available on any [`D128<SCALE>`].
///
/// Import this trait to call `D128s12::pi()`, `D128s12::e()`, etc.
///
/// All returned values are computed from a 37-digit raw-`i128` reference
/// without passing through `f64`. The result is bit-exact at the target
/// `SCALE` for every supported scale up to `SCALE = 37`.
pub trait DecimalConsts: Sized {
    /// Pi (~3.14159265...). One half-turn in radians.
    ///
    /// Source: ISO 80000-2 / OEIS A000796. 37-digit reference rescaled to
    /// `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn pi() -> Self;

    /// Tau (~6.28318530...). One full turn in radians.
    ///
    /// Defined as `2 * pi`. 37-digit reference rescaled to `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn tau() -> Self;

    /// Half-pi (~1.57079632...). One quarter-turn in radians.
    ///
    /// Defined as `pi / 2`. 37-digit reference rescaled to `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn half_pi() -> Self;

    /// Quarter-pi (~0.78539816...). One eighth-turn in radians.
    ///
    /// Defined as `pi / 4`. 37-digit reference rescaled to `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn quarter_pi() -> Self;

    /// The golden ratio (~1.61803398...). Dimensionless.
    ///
    /// Defined as `(1 + sqrt(5)) / 2`. Source: OEIS A001622. 35-digit
    /// reference rescaled to `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn golden() -> Self;

    /// Euler's number (~2.71828182...). Dimensionless.
    ///
    /// Source: OEIS A001113. 37-digit reference rescaled to `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn e() -> Self;
}

// Public-to-crate helpers that return each constant's rescaled bits at
// the caller's target SCALE. Used by the `decl_decimal_consts!` macro
// to provide DecimalConsts for narrower widths (D32, D64) without
// duplicating the rescale logic.

pub(crate) fn pi_at_target<const TARGET: u32>() -> i128 {
    D128::<SCALE_REF>::from_bits(PI_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn tau_at_target<const TARGET: u32>() -> i128 {
    D128::<SCALE_REF>::from_bits(TAU_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn half_pi_at_target<const TARGET: u32>() -> i128 {
    D128::<SCALE_REF>::from_bits(HALF_PI_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn quarter_pi_at_target<const TARGET: u32>() -> i128 {
    D128::<SCALE_REF>::from_bits(QUARTER_PI_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn golden_at_target<const TARGET: u32>() -> i128 {
    D128::<SCALE_REF>::from_bits(GOLDEN_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn e_at_target<const TARGET: u32>() -> i128 {
    D128::<SCALE_REF>::from_bits(E_RAW_S37).rescale::<TARGET>().to_bits()
}

// The `DecimalConsts` impl for `D128<SCALE>` is emitted by the
// `decl_decimal_consts!` macro — the same macro D32 / D64 / D256+ use.
// It expands to `Self(pi_at_target::<SCALE>())` etc., which is
// identical to the previous hand-coded
// `D128::<SCALE_REF>::from_bits(PI_RAW_S37).rescale::<SCALE>()` because
// `pi_at_target` is defined as exactly that, then `.to_bits()`.
crate::macros::consts::decl_decimal_consts!(D128, i128);

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
    // At SCALE = 12 each constant is the 37-digit raw integer divided by
    // 10^23, rounded half-to-even.

    /// pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 3_141_592_653_589.
    /// 14th digit is 7 (from position 14 of the raw) -> round up.
    /// Expected: 3_141_592_653_590.
    #[test]
    fn pi_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D128s12::pi().to_bits(), 3_141_592_653_590_i128);
    }

    /// tau at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 6_283_185_307_179.
    /// 14th digit is 5 -> round up. Expected: 6_283_185_307_180.
    #[test]
    fn tau_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D128s12::tau().to_bits(), 6_283_185_307_180_i128);
    }

    /// half_pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_570_796_326_794.
    /// 14th digit is 8 -> round up. Expected: 1_570_796_326_795.
    #[test]
    fn half_pi_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D128s12::half_pi().to_bits(), 1_570_796_326_795_i128);
    }

    /// quarter_pi at SCALE=12: raw / 10^23.
    /// Truncated 12 digits: 785_398_163_397.
    /// 13th digit is 4 -> no round-up. Expected: 785_398_163_397.
    #[test]
    fn quarter_pi_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D128s12::quarter_pi().to_bits(), 785_398_163_397_i128);
    }

    /// e at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 2_718_281_828_459.
    /// 14th digit is 0 -> no round-up. Expected: 2_718_281_828_459.
    #[test]
    fn e_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D128s12::e().to_bits(), 2_718_281_828_459_i128);
    }

    /// golden at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_618_033_988_749.
    /// 14th digit is 8 -> round up. Expected: 1_618_033_988_750.
    #[test]
    fn golden_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
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
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        type D6 = D128<6>;
        assert_eq!(D6::pi().to_bits(), 3_141_593_i128);
    }

    /// At SCALE = 0, pi() rounds to 3 (first fractional digit is 1, no
    /// round-up).
    #[test]
    fn pi_at_scale_0_is_three() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        type D0 = D128<0>;
        assert_eq!(D0::pi().to_bits(), 3_i128);
    }

    /// At SCALE = SCALE_REF (37), pi() returns exactly the raw constant.
    #[test]
    fn pi_at_scale_ref_is_raw_constant() {
        type D37 = D128<37>;
        assert_eq!(D37::pi().to_bits(), PI_RAW_S37);
    }

    /// At SCALE = SCALE_REF + 1 (38), pi() multiplies by 10, appending
    /// one trailing zero digit. PI_RAW_S37 * 10 ≈ 3.14×10³⁸ which is
    /// larger than i128::MAX ≈ 1.7×10³⁸, so this case overflows
    /// `D128<38>` storage at compile time — exercising the upper end
    /// of the rescale-up path is left to the SCALE = 37 case above.
    #[test]
    fn pi_at_scale_37_is_raw_constant() {
        type D37 = D128<37>;
        assert_eq!(D37::pi().to_bits(), PI_RAW_S37);
    }

    /// Negative-side rounding: negating pi gives the expected raw bits.
    #[test]
    fn neg_pi_round_trip() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        let pi = D128s12::pi();
        let neg_pi = -pi;
        assert_eq!(neg_pi.to_bits(), -3_141_592_653_590_i128);
    }

    // (`rescale_from_ref` boundary tests removed: the rounding logic now
    // lives in `D128::rescale` / `src/rounding.rs::apply_rounding` and is
    // covered by the tests in those modules.)
}
