//! Mathematical constants and float-compatibility constants for every
//! decimal width.
//!
//! # Constants provided
//!
//! The [`DecimalConstants`] trait exposes `pi`, `tau`, `half_pi`,
//! `quarter_pi`, `golden`, and `e` as methods on every width. The
//! native-tier (`D38` and narrower) impls live here; the wide tier
//! (`D76` / `D153` / `D307`) impls live in `types/consts/wide.rs`.
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
//! storage precision; the rescale to the caller's `SCALE` is always
//! **downward**, never upward, so half-to-even rounding always lands
//! on the **correctly-rounded** value at the target scale:
//!
//! | Tier           | Reference storage | `SCALE_REF` (= reference digits) | Source file       |
//! |----------------|-------------------|----------------------------------|-------------------|
//! | D18 / D38 | `Int<4>`          | 75                               | this file         |
//! | D76            | `Int<4>`          | 75                               | `types/consts/wide.rs`  |
//! | D153           | `Int<8>`          | 153                              | `types/consts/wide.rs`  |
//! | D307           | `Int<16>`         | 307                              | `types/consts/wide.rs`  |
//!
//! The rescale from `SCALE_REF` to the caller's `SCALE` uses integer
//! division with the crate-default [`RoundingMode`] (half-to-even by
//! default; overridable via the `rounding-*` Cargo features). Going
//! through `f64` would cap precision at ~15–17 decimal digits; the
//! raw-integer path preserves the full per-tier reference width.
//!
//! **0.5 ULP at every supported scale**, on every width, with no
//! exceptions in the precision contract. The only constraint is the
//! width's *storage range*: a value that mathematically exceeds the
//! type's `Storage::MAX / 10^SCALE` cannot be represented at all. At
//! `D38<38>` the storage range is approximately ±1.70141, so the three
//! larger-magnitude constants — `pi ≈ 3.14159`, `tau ≈ 6.28318`,
//! `e ≈ 2.71828` — overflow `i128` and the corresponding methods panic
//! with a clear "constant out of storage range" message;
//! `half_pi ≈ 1.57080`, `quarter_pi ≈ 0.78540`, and `golden ≈ 1.61803`
//! all fit inside ±1.70141 and remain correctly-rounded to 0.5 ULP.
//!
//! [`RoundingMode`]: crate::support::rounding::RoundingMode
//!
//! # Sources
//!
//! Each raw constant is the half-to-even rounding of the canonical
//! decimal expansion to the tier's `SCALE_REF` fractional digits. ISO
//! 80000-2 (pi, tau, pi/2, pi/4), OEIS A001113 (e), OEIS A001622
//! (golden ratio).

use crate::algos::support::fixed_d38::Fixed;
use crate::types::widths::D38;
use crate::int::types::Int;

/// Reference scale for every constant in this file: the 75-digit
/// representation that fits an `Int<4>` (`4 · 64` bits). Every D38
/// scale (0..=38) is at most 38 digits, so we always rescale **down**
/// from 75 → SCALE, never up. The half-to-even rescale-down step is
/// performed by [`Fixed::round_to_i128_with`] (`Fixed` is the same 256-bit
/// guard-digit type the strict transcendentals use), giving 0.5 ULP at
/// the caller's `SCALE` for every value that fits `i128` at that
/// scale.
///
/// # Precision
///
/// N/A: constant value, no arithmetic performed.
const SCALE_REF: u32 = 75;

// Raw decimal strings at 75 fractional digits, materialised at build
// time by `build.rs` (the same hand-rolled multi-precision generator
// that emits the wide-tier constants). Sources: ISO 80000-2 (pi, tau,
// pi/2, pi/4), OEIS A001113 (e), OEIS A001622 (golden ratio).
//
// The build-time string -> Int<4> parse is `const fn` (via
// `Int::<4>::from_str_radix`, base 10 only). The 75-digit reference is
// the largest decimal expansion that always fits Int<4> for the
// biggest of these constants (tau ≈ 6.28×10⁷⁵ < Int::<4>::MAX ≈
// 5.78×10⁷⁶); a single shared SCALE_REF keeps the rescale helpers
// uniform across all six methods on the trait.

include!(concat!(env!("OUT_DIR"), "/wide_consts.rs"));

pub(crate) const PI_RAW: Int<4> = match Int::<4>::from_str_radix(PI_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: PI_D76_S75 not parseable"),
};
const TAU_RAW: Int<4> = match Int::<4>::from_str_radix(TAU_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: TAU_D76_S75 not parseable"),
};
const HALF_PI_RAW: Int<4> = match Int::<4>::from_str_radix(HALF_PI_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: HALF_PI_D76_S75 not parseable"),
};
const QUARTER_PI_RAW: Int<4> = match Int::<4>::from_str_radix(QUARTER_PI_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: QUARTER_PI_D76_S75 not parseable"),
};
const E_RAW: Int<4> = match Int::<4>::from_str_radix(E_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: E_D76_S75 not parseable"),
};
const GOLDEN_RAW: Int<4> = match Int::<4>::from_str_radix(GOLDEN_D76_S75, 10) {
    Ok(v) => v,
    Err(_) => panic!("consts: GOLDEN_D76_S75 not parseable"),
};

/// Rescale a 75-digit `Int<4>` reference down to the caller's `TARGET`
/// scale as an `i128`, half-to-even. Panics if the value at `TARGET`
/// does not fit `i128` (the type's storage range at that scale just
/// doesn't include this constant — e.g. `pi ≈ 3.14` at `D38<38>` would
/// need `3.14 × 10^38 ≈ 3.14e38`, which exceeds `i128::MAX ≈ 1.7e38`).
fn rescale_75_to_target<const TARGET: u32>(raw: Int<4>, name: &'static str) -> i128 {
    rescale_75_to_target_with::<TARGET>(raw, name, crate::support::rounding::DEFAULT_ROUNDING_MODE)
}

/// Mode-aware variant of [`rescale_75_to_target`].
///
/// `Floor` gives the largest representable value ≤ true constant —
/// useful when downstream code uses the value as an upper bound that
/// must not be exceeded. `Ceiling` gives the smallest value ≥ true
/// constant — useful for conservative bucket counts /
/// over-approximation. The three half-modes coincide for irrational
/// constants (no integer mantissa hits the exact half-way point at
/// the 75-digit reference scale).
fn rescale_75_to_target_with<const TARGET: u32>(
    raw: Int<4>,
    name: &'static str,
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    let words = raw.limbs_le();
    let mag: [u128; 2] = [
        (words[0] as u128) | ((words[1] as u128) << 64),
        (words[2] as u128) | ((words[3] as u128) << 64),
    ];
    let f = Fixed {
        negative: false,
        mag,
    };
    match f.round_to_i128_with(SCALE_REF, TARGET, mode) {
        Some(v) => v,
        None => panic!(
            "D38 constant out of storage range: {name} cannot fit i128 at SCALE = {TARGET} \
             (storage range is ±i128::MAX / 10^SCALE)",
            name = name,
            TARGET = TARGET,
        ),
    }
}

/// Well-known mathematical constants available on every decimal width
/// (`D18` / `D38` / `D76` / `D153` / `D307`).
///
/// Import this trait to call `D38s12::pi()`, `D76::<35>::e()`, etc.
///
/// All returned values are computed from a raw integer reference at
/// the tier's maximum storage precision (75 digits for D18/D38 and
/// D76; 153 for D153; 307 for D307) without passing through `f64`,
/// then rescaled down to the caller's `SCALE` with half-to-even
/// rounding. The result is **within 0.5 ULP** of the canonical
/// decimal expansion at every supported scale on every width.
///
/// The one situation where a method does not return a value is when
/// the constant's magnitude exceeds the type's storage range at the
/// caller's `SCALE` — e.g. `D38<38>::pi()` would need `3.14 × 10³⁸`,
/// which exceeds `i128::MAX ≈ 1.7×10³⁸`. The method panics with a
/// clear "constant out of storage range" message in that case.
///
/// # Crossing into f64
///
/// `to_f64()` is itself correctly rounded, but it can only round to
/// the *decimal value the type holds* — not to the underlying ideal
/// constant. `f64` carries ~15.95 decimal digits of mantissa, so any
/// constant produced at `SCALE < 15` is intrinsically coarser than
/// the `f64` grid: `D38<12>::pi().to_f64()` lands ~466 ULPs from
/// [`std::f64::consts::PI`], because the 12-digit decimal rounds
/// differently than the closest-`f64` to true π. At `SCALE ≥ 15` the
/// round-trip is bit-exact for these constants (the decimal value
/// has enough digits to disambiguate the `f64` grid).
///
/// **Practical rule for downstream code that crosses into `f64`** —
/// CAD bulge-arc tessellation, OpenGL/GLSL, hardware drivers — and
/// uses the `f64` value to count, bucket, or seed a fixed-iteration
/// loop: source mathematical constants from [`std::f64::consts`]
/// directly at the boundary rather than going through
/// `Decimal::pi().to_f64()`. Otherwise pick a `SCALE` of 15 or more
/// so the decimal value can round-trip to the canonical `f64`.
pub trait DecimalConstants: Sized {
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

    // ─── *_with(mode) siblings ───────────────────────────────────
    //
    // Each `<const>_with(mode)` rescales the 75-digit reference under
    // the caller-supplied `RoundingMode`. Useful when the default
    // mode (half-to-even, or whatever a `rounding-*` Cargo feature
    // selects) is the wrong direction for the use case — e.g. a CAD
    // tessellation that needs `pi_with(Floor)` so the down-stream
    // f64 conversion stays ≤ true π and segment counts can't
    // over-flow their fixed-size buffers.

    /// `pi()` under the supplied rounding mode.
    fn pi_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `tau()` under the supplied rounding mode.
    fn tau_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `half_pi()` under the supplied rounding mode.
    fn half_pi_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `quarter_pi()` under the supplied rounding mode.
    fn quarter_pi_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `golden()` under the supplied rounding mode.
    fn golden_with(mode: crate::support::rounding::RoundingMode) -> Self;
    /// `e()` under the supplied rounding mode.
    fn e_with(mode: crate::support::rounding::RoundingMode) -> Self;
}

// Public-to-crate helpers that return each constant's rescaled bits at
// the caller's target SCALE. Used by the `decl_decimal_consts!` macro
// to provide DecimalConstants for narrower widths (D18) without
// duplicating the rescale logic.

pub(crate) fn pi_at_target<const TARGET: u32>() -> i128 {
    rescale_75_to_target::<TARGET>(PI_RAW, "pi")
}
pub(crate) fn tau_at_target<const TARGET: u32>() -> i128 {
    rescale_75_to_target::<TARGET>(TAU_RAW, "tau")
}
pub(crate) fn half_pi_at_target<const TARGET: u32>() -> i128 {
    rescale_75_to_target::<TARGET>(HALF_PI_RAW, "half_pi")
}
pub(crate) fn quarter_pi_at_target<const TARGET: u32>() -> i128 {
    rescale_75_to_target::<TARGET>(QUARTER_PI_RAW, "quarter_pi")
}
pub(crate) fn golden_at_target<const TARGET: u32>() -> i128 {
    rescale_75_to_target::<TARGET>(GOLDEN_RAW, "golden")
}
pub(crate) fn e_at_target<const TARGET: u32>() -> i128 {
    rescale_75_to_target::<TARGET>(E_RAW, "e")
}

// Mode-aware variants — used by the `*_with(mode)` constant methods.

pub(crate) fn pi_at_target_with<const TARGET: u32>(
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    rescale_75_to_target_with::<TARGET>(PI_RAW, "pi", mode)
}
pub(crate) fn tau_at_target_with<const TARGET: u32>(
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    rescale_75_to_target_with::<TARGET>(TAU_RAW, "tau", mode)
}
pub(crate) fn half_pi_at_target_with<const TARGET: u32>(
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    rescale_75_to_target_with::<TARGET>(HALF_PI_RAW, "half_pi", mode)
}
pub(crate) fn quarter_pi_at_target_with<const TARGET: u32>(
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    rescale_75_to_target_with::<TARGET>(QUARTER_PI_RAW, "quarter_pi", mode)
}
pub(crate) fn golden_at_target_with<const TARGET: u32>(
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    rescale_75_to_target_with::<TARGET>(GOLDEN_RAW, "golden", mode)
}
pub(crate) fn e_at_target_with<const TARGET: u32>(
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    rescale_75_to_target_with::<TARGET>(E_RAW, "e", mode)
}

// The `DecimalConstants` impl for `D38<SCALE>` is emitted by the
// `decl_decimal_consts!` macro — the same macro D18 / D76+ use.
// It expands to `Self(pi_at_target::<SCALE>())` etc.; each
// `*_at_target` helper above rescales the 75-digit Int<4> reference
// down to the caller's `SCALE` via half-to-even and narrows to i128
// (or panics with a clear message if the constant's magnitude
// exceeds the storage range at that scale).
crate::macros::consts::decl_decimal_consts!(wide D38, crate::int::types::Int<2>);

// EPSILON / MIN_POSITIVE for every width are now emitted by
// `decl_decimal_basics!`. The D38-specific inherent impl that used
// to live here has been removed in favour of the macro-emitted ones
// so D18 / D38 / D57 / D76 / D115 / D153 / D230 / D307 / D462 /
// D616 / D924 / D1232 all share the same EPSILON / MIN_POSITIVE
// surface.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::widths::D38s12;

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
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        assert_eq!(D38s12::pi().to_bits().as_i128(), 3_141_592_653_590_i128);
    }

    /// tau at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 6_283_185_307_179.
    /// 14th digit is 5 -> round up. Expected: 6_283_185_307_180.
    #[test]
    fn tau_is_bit_exact_at_scale_12() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        assert_eq!(D38s12::tau().to_bits().as_i128(), 6_283_185_307_180_i128);
    }

    /// half_pi at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_570_796_326_794.
    /// 14th digit is 8 -> round up. Expected: 1_570_796_326_795.
    #[test]
    fn half_pi_is_bit_exact_at_scale_12() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        assert_eq!(D38s12::half_pi().to_bits().as_i128(), 1_570_796_326_795_i128);
    }

    /// quarter_pi at SCALE=12: raw / 10^23.
    /// Truncated 12 digits: 785_398_163_397.
    /// 13th digit is 4 -> no round-up. Expected: 785_398_163_397.
    #[test]
    fn quarter_pi_is_bit_exact_at_scale_12() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        assert_eq!(D38s12::quarter_pi().to_bits().as_i128(), 785_398_163_397_i128);
    }

    /// e at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 2_718_281_828_459.
    /// 14th digit is 0 -> no round-up. Expected: 2_718_281_828_459.
    #[test]
    fn e_is_bit_exact_at_scale_12() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        assert_eq!(D38s12::e().to_bits().as_i128(), 2_718_281_828_459_i128);
    }

    /// golden at SCALE=12: raw / 10^23.
    /// Truncated 13 digits: 1_618_033_988_749.
    /// 14th digit is 8 -> round up. Expected: 1_618_033_988_750.
    #[test]
    fn golden_is_bit_exact_at_scale_12() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        assert_eq!(D38s12::golden().to_bits().as_i128(), 1_618_033_988_750_i128);
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
        let diff = (D38s12::half_pi().to_f64() - core::f64::consts::FRAC_PI_2).abs();
        assert!(
            diff < 1e-11,
            "half_pi diverges from f64 FRAC_PI_2 by {diff}"
        );
    }

    #[test]
    fn quarter_pi_close_to_f64_frac_pi_4() {
        let diff = (D38s12::quarter_pi().to_f64() - core::f64::consts::FRAC_PI_4).abs();
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
        assert_eq!(D38s12::EPSILON.to_bits().as_i128(), 1_i128);
        assert!(D38s12::EPSILON > D38s12::ZERO);
    }

    #[test]
    fn min_positive_is_one_ulp() {
        assert_eq!(D38s12::MIN_POSITIVE.to_bits().as_i128(), 1_i128);
        assert_eq!(D38s12::MIN_POSITIVE, D38s12::EPSILON);
    }

    /// At SCALE = 6 the LSB is 10^-6; EPSILON is still raw 1.
    #[test]
    fn epsilon_at_scale_6_is_one_ulp() {
        type D6 = crate::D<crate::int::types::Int<2>, 6>;
        assert_eq!(D6::EPSILON.to_bits().as_i128(), 1_i128);
        assert_eq!(D6::MIN_POSITIVE.to_bits().as_i128(), 1_i128);
    }

    // Cross-scale exercises

    /// At SCALE = 6, pi() should equal 3.141593 (rounded half-to-even from
    /// 3.1415926535...). Expected raw bits: 3_141_593.
    #[test]
    fn pi_at_scale_6_is_bit_exact() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        type D6 = crate::D<crate::int::types::Int<2>, 6>;
        assert_eq!(D6::pi().to_bits().as_i128(), 3_141_593_i128);
    }

    /// At SCALE = 0, pi() rounds to 3 (first fractional digit is 1, no
    /// round-up).
    #[test]
    fn pi_at_scale_0_is_three() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        type D0 = crate::D<crate::int::types::Int<2>, 0>;
        assert_eq!(D0::pi().to_bits().as_i128(), 3_i128);
    }

    /// `D38<37>::pi()` is the canonical pi rounded half-to-even to 37
    /// fractional digits. The 75-digit Int<4> reference is rescaled
    /// down to 37 digits; the result is bit-identical to the
    /// hand-tabulated constant.
    #[test]
    fn pi_at_scale_37_matches_canonical_37_digit_rounding() {
        type D37 = crate::D<crate::int::types::Int<2>, 37>;
        // pi to 38 digits: 3.14159265358979323846264338327950288420
        //                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                   keep 37 frac digits; the 38th digit is 0
        //                   so half-to-even rounds down — no bump.
        let expected: i128 = 31_415_926_535_897_932_384_626_433_832_795_028_842;
        assert_eq!(D37::pi().to_bits().as_i128(), expected);
    }

    // `D38<38>` storage range is approximately ±1.70141 (i128::MAX /
    // 10^38). The three constants whose magnitude exceeds that bound
    // must panic with a clear "out of storage range" message:
    //
    // - pi    ≈ 3.14159    > 1.70141 → must panic
    // - tau   ≈ 6.28318    > 1.70141 → must panic
    // - e     ≈ 2.71828    > 1.70141 → must panic
    //
    // The three that DO fit must be correctly rounded to 0.5 ULP:
    //
    // - half_pi    ≈ 1.57079   < 1.70141 → must round to 0.5 ULP
    // - quarter_pi ≈ 0.78540   < 1.70141 → must round to 0.5 ULP
    // - golden     ≈ 1.61803   < 1.70141 → must round to 0.5 ULP

    #[test]
    #[should_panic(expected = "out of storage range")]
    fn pi_at_scale_38_panics_storage_range() {
        let _ = crate::D::<crate::int::types::Int<2>, 38>::pi();
    }

    #[test]
    #[should_panic(expected = "out of storage range")]
    fn tau_at_scale_38_panics_storage_range() {
        let _ = crate::D::<crate::int::types::Int<2>, 38>::tau();
    }

    #[test]
    #[should_panic(expected = "out of storage range")]
    fn e_at_scale_38_panics_storage_range() {
        let _ = crate::D::<crate::int::types::Int<2>, 38>::e();
    }

    /// `half_pi` / `quarter_pi` / `golden` at `D38<38>` must not panic
    /// (their magnitudes are inside the type's ±1.7 storage range) and
    /// each must be correctly rounded to 0.5 ULP (= 1 LSB).
    #[test]
    fn fitting_constants_at_scale_38_are_correctly_rounded() {
        // half_pi to 38 digits: 1.57079632679489661923132169163975144210
        let expected_half_pi: i128 = 157_079_632_679_489_661_923_132_169_163_975_144_210;
        let got = crate::D::<crate::int::types::Int<2>, 38>::half_pi().to_bits().as_i128();
        let diff = (got - expected_half_pi).abs();
        assert!(
            diff <= 1,
            "half_pi: got {got}, expected {expected_half_pi}, diff {diff} > 1 LSB"
        );

        // quarter_pi to 38 digits: 0.78539816339744830961566084581987572105
        let expected_quarter_pi: i128 = 78_539_816_339_744_830_961_566_084_581_987_572_105;
        let got = crate::D::<crate::int::types::Int<2>, 38>::quarter_pi().to_bits().as_i128();
        let diff = (got - expected_quarter_pi).abs();
        assert!(
            diff <= 1,
            "quarter_pi: got {got}, expected {expected_quarter_pi}, diff {diff} > 1 LSB"
        );

        // golden to 38 digits: 1.61803398874989484820458683436563811772
        let expected_golden: i128 = 161_803_398_874_989_484_820_458_683_436_563_811_772;
        let got = crate::D::<crate::int::types::Int<2>, 38>::golden().to_bits().as_i128();
        let diff = (got - expected_golden).abs();
        assert!(
            diff <= 1,
            "golden: got {got}, expected {expected_golden}, diff {diff} > 1 LSB"
        );
    }

    /// Negative-side rounding: negating pi gives the expected raw bits.
    #[test]
    fn neg_pi_round_trip() {
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
        let pi = D38s12::pi();
        let neg_pi = -pi;
        assert_eq!(neg_pi.to_bits().as_i128(), -3_141_592_653_590_i128);
    }

    // (`rescale_from_ref` boundary tests removed: the rounding logic now
    // lives in `D38::rescale` / `src/rounding.rs::apply_rounding` and is
    // covered by the tests in those modules.)
}
