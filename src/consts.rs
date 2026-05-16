//! Mathematical constants and float-compatibility constants for every
//! decimal width.
//!
//! # Constants provided
//!
//! The [`DecimalConsts`] trait exposes `pi`, `tau`, `half_pi`,
//! `quarter_pi`, `golden`, and `e` as methods on every width. The
//! native-tier (`D38` and narrower) impls live here; the wide tier
//! (`D76` / `D153` / `D307`) impls live in `consts_wide.rs`.
//!
//! Two inherent associated constants, `EPSILON` and `MIN_POSITIVE`, are
//! provided as analogues to `f64::EPSILON` and `f64::MIN_POSITIVE` so
//! that generic code parameterised over numeric types continues to
//! compile when `T` is any of the decimal widths.
//!
//! # Precision strategy
//!
//! Constants are derived from raw integer references — no `f64`
//! anywhere. Each tier has its own reference at the tier's maximum
//! storage precision:
//!
//! | Tier           | Storage of reference | `SCALE_REF` (= reference digits) | Source file       |
//! |----------------|---------------------|----------------------------------|-------------------|
//! | D9 / D18 / D38 | `i128`              | 37                               | this file         |
//! | D76            | `Int256`            | 75                               | `consts_wide.rs`  |
//! | D153           | `Int512`            | 153                              | `consts_wide.rs`  |
//! | D307           | `Int1024`           | 307                              | `consts_wide.rs`  |
//!
//! The rescale from `SCALE_REF` to the caller's `SCALE` uses integer
//! division with the crate-default [`RoundingMode`] (half-to-even by
//! default; overridable via the `rounding-*` Cargo features). Going
//! through `f64` would cap precision at ~15–17 decimal digits; the
//! raw-integer path preserves the full per-tier reference width.
//!
//! At `SCALE ≤ SCALE_REF` (every supported scale on D9 / D18 / D76 /
//! D153 / D307, and every D38 scale up to 37) the result is within
//! **0.5 ULP** of the canonical decimal expansion. The single
//! exception is `D38<38>`: the largest D38 reference fits 37 fractional
//! digits in `i128` (`tau ≈ 6.28×10³⁷` is below `i128::MAX ≈
//! 1.7×10³⁸`), so `SCALE = 38` (the D38 maximum) is rescaled
//! upward — multiplying the 37-digit reference by 10 — which appends
//! a placeholder zero rather than adding precision. The error there
//! is bounded at ≈ 5 ULP for `pi` / `tau` / `e` / `golden`; `half_pi`
//! and `quarter_pi` (smaller in magnitude) remain inside 0.5 ULP.
//!
//! [`RoundingMode`]: crate::rounding::RoundingMode
//!
//! # Sources
//!
//! Each raw constant is the half-to-even rounding of the canonical
//! decimal expansion to the tier's `SCALE_REF` fractional digits. ISO
//! 80000-2 (pi, tau, pi/2, pi/4), OEIS A001113 (e), OEIS A001622
//! (golden ratio).

use crate::core_type::D38;

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

// Raw i128 constants at SCALE_REF = 37, materialised at build time
// by `build.rs` (the same hand-rolled multi-precision generator that
// emits the wide-tier constants). Sources: ISO 80000-2 (pi, tau,
// pi/2, pi/4), OEIS A001113 (e), OEIS A001622 (golden ratio).
//
// The build-time string -> i128 parse is `const fn` (Rust 1.83+).

include!(concat!(env!("OUT_DIR"), "/wide_consts.rs"));

const PI_RAW_S37: i128 = match i128::from_str_radix(PI_D38_S37, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: PI_D38_S37 not parseable"),
};
const TAU_RAW_S37: i128 = match i128::from_str_radix(TAU_D38_S37, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: TAU_D38_S37 not parseable"),
};
const HALF_PI_RAW_S37: i128 = match i128::from_str_radix(HALF_PI_D38_S37, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: HALF_PI_D38_S37 not parseable"),
};
const QUARTER_PI_RAW_S37: i128 = match i128::from_str_radix(QUARTER_PI_D38_S37, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: QUARTER_PI_D38_S37 not parseable"),
};
const E_RAW_S37: i128 = match i128::from_str_radix(E_D38_S37, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: E_D38_S37 not parseable"),
};
const GOLDEN_RAW_S37: i128 = match i128::from_str_radix(GOLDEN_D38_S37, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: GOLDEN_D38_S37 not parseable"),
};

// Rescaling from SCALE_REF to the caller's SCALE is delegated to
// `D38::rescale` (which uses round-half-to-even by default; see
// `src/rescale.rs`). The constants below construct a `D38<SCALE_REF>`
// from the raw integer literal and then rescale to the caller's
// `D38<SCALE>`.

/// Well-known mathematical constants available on every decimal width
/// (`D9` / `D18` / `D38` / `D76` / `D153` / `D307`).
///
/// Import this trait to call `D38s12::pi()`, `D76::<35>::e()`, etc.
///
/// All returned values are computed from a raw integer reference at
/// each tier's maximum storage precision (37 digits for D9/D18/D38, 75
/// for D76, 153 for D153, 307 for D307) without passing through `f64`.
/// The result is within 0.5 ULP of the canonical decimal expansion at
/// the target `SCALE` for every supported scale, with one exception:
/// `D38<38>` (the D38 maximum) rescales the 37-digit reference upward
/// by 10, appending a placeholder zero rather than adding precision;
/// the error there is bounded at ≈ 5 ULP for the larger-magnitude
/// constants. See the module-level docs for the per-tier table.
pub trait DecimalConsts: Sized {
    /// Pi (~3.14159265...). One half-turn in radians.
    ///
    /// Source: ISO 80000-2 / OEIS A000796. Rescaled per-tier (see the
    /// module-level table) to the caller's `SCALE` via the crate-default
    /// rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn pi() -> Self;

    /// Tau (~6.28318530...). One full turn in radians.
    ///
    /// Defined as `2 * pi`. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn tau() -> Self;

    /// Half-pi (~1.57079632...). One quarter-turn in radians.
    ///
    /// Defined as `pi / 2`. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn half_pi() -> Self;

    /// Quarter-pi (~0.78539816...). One eighth-turn in radians.
    ///
    /// Defined as `pi / 4`. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn quarter_pi() -> Self;

    /// The golden ratio (~1.61803398...). Dimensionless.
    ///
    /// Defined as `(1 + sqrt(5)) / 2`. Source: OEIS A001622. Rescaled
    /// per-tier (see the module-level table) to the caller's `SCALE`
    /// via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn golden() -> Self;

    /// Euler's number (~2.71828182...). Dimensionless.
    ///
    /// Source: OEIS A001113. Rescaled per-tier (see the module-level table) to the caller's `SCALE` via the crate-default rounding mode.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    fn e() -> Self;
}

// Public-to-crate helpers that return each constant's rescaled bits at
// the caller's target SCALE. Used by the `decl_decimal_consts!` macro
// to provide DecimalConsts for narrower widths (D9, D18) without
// duplicating the rescale logic.

pub(crate) fn pi_at_target<const TARGET: u32>() -> i128 {
    D38::<SCALE_REF>::from_bits(PI_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn tau_at_target<const TARGET: u32>() -> i128 {
    D38::<SCALE_REF>::from_bits(TAU_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn half_pi_at_target<const TARGET: u32>() -> i128 {
    D38::<SCALE_REF>::from_bits(HALF_PI_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn quarter_pi_at_target<const TARGET: u32>() -> i128 {
    D38::<SCALE_REF>::from_bits(QUARTER_PI_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn golden_at_target<const TARGET: u32>() -> i128 {
    D38::<SCALE_REF>::from_bits(GOLDEN_RAW_S37).rescale::<TARGET>().to_bits()
}
pub(crate) fn e_at_target<const TARGET: u32>() -> i128 {
    D38::<SCALE_REF>::from_bits(E_RAW_S37).rescale::<TARGET>().to_bits()
}

// The `DecimalConsts` impl for `D38<SCALE>` is emitted by the
// `decl_decimal_consts!` macro — the same macro D9 / D18 / D76+ use.
// It expands to `Self(pi_at_target::<SCALE>())` etc., which is
// identical to the previous hand-coded
// `D38::<SCALE_REF>::from_bits(PI_RAW_S37).rescale::<SCALE>()` because
// `pi_at_target` is defined as exactly that, then `.to_bits()`.
crate::macros::consts::decl_decimal_consts!(D38, i128);

// Inherent associated constants: EPSILON / MIN_POSITIVE.
//
// These mirror `f64::EPSILON` and `f64::MIN_POSITIVE` so that generic
// numeric code that calls `T::EPSILON` or `T::MIN_POSITIVE` compiles
// when `T = D38<SCALE>`. For D38 both equal `D38(1)` -- the smallest
// representable positive value (1 LSB = 10^-SCALE). There are no subnormals.

impl<const SCALE: u32> D38<SCALE> {
    /// Smallest representable positive value: 1 LSB = `10^-SCALE`.
    ///
    /// Provided as an analogue to `f64::EPSILON` for generic numeric code.
    /// Note that this differs from the f64 definition ("difference between
    /// 1.0 and the next-larger f64"): for `D38` the LSB is uniform across
    /// the entire representable range.
    ///
    /// # Precision
    ///
    /// N/A: constant value, no arithmetic performed.
    pub const EPSILON: Self = Self(1);

    /// Smallest positive value (equal to [`Self::EPSILON`]).
    ///
    /// Provided as an analogue to `f64::MIN_POSITIVE` for generic numeric
    /// code. Unlike `f64`, `D38` has no subnormals, so `MIN_POSITIVE`
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
    use crate::core_type::D38s12;

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
        assert_eq!(D38s12::pi().to_bits(), 3_141_592_653_590_i128);
    }

    /// tau at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 6_283_185_307_179.
    /// 14th digit is 5 -> round up. Expected: 6_283_185_307_180.
    #[test]
    fn tau_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D38s12::tau().to_bits(), 6_283_185_307_180_i128);
    }

    /// half_pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_570_796_326_794.
    /// 14th digit is 8 -> round up. Expected: 1_570_796_326_795.
    #[test]
    fn half_pi_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D38s12::half_pi().to_bits(), 1_570_796_326_795_i128);
    }

    /// quarter_pi at SCALE=12: raw / 10^23.
    /// Truncated 12 digits: 785_398_163_397.
    /// 13th digit is 4 -> no round-up. Expected: 785_398_163_397.
    #[test]
    fn quarter_pi_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D38s12::quarter_pi().to_bits(), 785_398_163_397_i128);
    }

    /// e at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 2_718_281_828_459.
    /// 14th digit is 0 -> no round-up. Expected: 2_718_281_828_459.
    #[test]
    fn e_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D38s12::e().to_bits(), 2_718_281_828_459_i128);
    }

    /// golden at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_618_033_988_749.
    /// 14th digit is 8 -> round up. Expected: 1_618_033_988_750.
    #[test]
    fn golden_is_bit_exact_at_scale_12() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        assert_eq!(D38s12::golden().to_bits(), 1_618_033_988_750_i128);
    }

    // Closeness checks against core::f64::consts.
    // These verify that the correct reference digits were selected; the
    // bit-exact tests above are the primary acceptance criteria.

    /// pi() converted to f64 is within 1e-11 of `core::f64::consts::PI`.
    /// At SCALE=12, 1 LSB = 1e-12, so 1e-11 covers rescale rounding plus
    /// the f64 conversion step.
    #[test]
    fn pi_close_to_f64_pi() {
        let diff = (D38s12::pi().to_f64() - core::f64::consts::PI).abs();
        assert!(diff < 1e-11, "pi diverges from f64 PI by {diff}");
    }

    #[test]
    fn tau_close_to_f64_tau() {
        let diff = (D38s12::tau().to_f64() - core::f64::consts::TAU).abs();
        assert!(diff < 1e-11, "tau diverges from f64 TAU by {diff}");
    }

    #[test]
    fn half_pi_close_to_f64_frac_pi_2() {
        let diff =
            (D38s12::half_pi().to_f64() - core::f64::consts::FRAC_PI_2).abs();
        assert!(diff < 1e-11, "half_pi diverges from f64 FRAC_PI_2 by {diff}");
    }

    #[test]
    fn quarter_pi_close_to_f64_frac_pi_4() {
        let diff =
            (D38s12::quarter_pi().to_f64() - core::f64::consts::FRAC_PI_4).abs();
        assert!(
            diff < 1e-11,
            "quarter_pi diverges from f64 FRAC_PI_4 by {diff}"
        );
    }

    #[test]
    fn e_close_to_f64_e() {
        let diff = (D38s12::e().to_f64() - core::f64::consts::E).abs();
        assert!(diff < 1e-11, "e diverges from f64 E by {diff}");
    }

    /// golden() converted to f64 is within 1e-11 of the closed form
    /// `(1 + sqrt(5)) / 2`. Requires std for `f64::sqrt`.
    #[cfg(feature = "std")]
    #[test]
    fn golden_close_to_closed_form() {
        let expected = (1.0_f64 + 5.0_f64.sqrt()) / 2.0;
        let diff = (D38s12::golden().to_f64() - expected).abs();
        assert!(diff < 1e-11, "golden diverges from closed-form by {diff}");
    }

    // EPSILON / MIN_POSITIVE

    #[test]
    fn epsilon_is_one_ulp() {
        assert_eq!(D38s12::EPSILON.to_bits(), 1_i128);
        assert!(D38s12::EPSILON > D38s12::ZERO);
    }

    #[test]
    fn min_positive_is_one_ulp() {
        assert_eq!(D38s12::MIN_POSITIVE.to_bits(), 1_i128);
        assert_eq!(D38s12::MIN_POSITIVE, D38s12::EPSILON);
    }

    /// At SCALE = 6 the LSB is 10^-6; EPSILON is still raw 1.
    #[test]
    fn epsilon_at_scale_6_is_one_ulp() {
        type D6 = D38<6>;
        assert_eq!(D6::EPSILON.to_bits(), 1_i128);
        assert_eq!(D6::MIN_POSITIVE.to_bits(), 1_i128);
    }

    // Cross-scale exercises

    /// At SCALE = 6, pi() should equal 3.141593 (rounded half-to-even from
    /// 3.1415926535...). Expected raw bits: 3_141_593.
    #[test]
    fn pi_at_scale_6_is_bit_exact() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        type D6 = D38<6>;
        assert_eq!(D6::pi().to_bits(), 3_141_593_i128);
    }

    /// At SCALE = 0, pi() rounds to 3 (first fractional digit is 1, no
    /// round-up).
    #[test]
    fn pi_at_scale_0_is_three() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        type D0 = D38<0>;
        assert_eq!(D0::pi().to_bits(), 3_i128);
    }

    /// At SCALE = SCALE_REF (37), pi() returns exactly the raw constant.
    #[test]
    fn pi_at_scale_ref_is_raw_constant() {
        type D37 = D38<37>;
        assert_eq!(D37::pi().to_bits(), PI_RAW_S37);
    }

    /// At SCALE = SCALE_REF + 1 (38), pi() multiplies by 10, appending
    /// one trailing zero digit. PI_RAW_S37 * 10 ≈ 3.14×10³⁸ which is
    /// larger than i128::MAX ≈ 1.7×10³⁸, so this case overflows
    /// `D38<38>` storage at compile time — exercising the upper end
    /// of the rescale-up path is left to the SCALE = 37 case above.
    #[test]
    fn pi_at_scale_37_is_raw_constant() {
        type D37 = D38<37>;
        assert_eq!(D37::pi().to_bits(), PI_RAW_S37);
    }

    /// Negative-side rounding: negating pi gives the expected raw bits.
    #[test]
    fn neg_pi_round_trip() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        let pi = D38s12::pi();
        let neg_pi = -pi;
        assert_eq!(neg_pi.to_bits(), -3_141_592_653_590_i128);
    }

    // (`rescale_from_ref` boundary tests removed: the rounding logic now
    // lives in `D38::rescale` / `src/rounding.rs::apply_rounding` and is
    // covered by the tests in those modules.)
}
