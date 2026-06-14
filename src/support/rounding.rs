// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Rounding-mode selector for scale-narrowing operations.
//!
//! Passed to every `*_with(mode)` sibling on every decimal width —
//! [`crate::D38::rescale_with`], `mul_with`, `div_with`, `to_int_with`,
//! `from_f64_with`, every `*_strict_with` on the wide tier, etc. — to
//! control how fractional digits are discarded when the result has
//! lower precision than the working intermediate. The six modes cover
//! IEEE-754's five rounding rules (`HalfToEven`, `HalfTowardZero`,
//! `Trunc`, `Floor`, `Ceiling`) plus the commercial `HalfAwayFromZero`
//! rule expected by users coming from `bigdecimal` / `rust_decimal`.
//!
//! The default mode is `HalfToEven` (IEEE-754 default; no systematic
//! bias). The `rounding-*` Cargo features let a downstream crate flip
//! the crate-wide default at compile time.

/// Selector for the rounding rule applied when a scale-narrowing
/// operation discards fractional digits.
///
/// See the module-level documentation for when each rule applies.
///
/// # Precision
///
/// N/A: this is a tag; no arithmetic is performed by constructing
/// or comparing variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    /// Round to nearest; on ties, round to the even neighbour.
    /// IEEE-754 `roundTiesToEven`; also called banker's rounding.
    /// Unbiased — repeated rounding does not drift sums. Crate default.
    ///
    /// Examples (truncate to integer): `0.5 -> 0`, `1.5 -> 2`,
    /// `2.5 -> 2`, `-0.5 -> 0`, `-1.5 -> -2`.
    HalfToEven,
    /// Round to nearest; on ties, round away from zero. Commercial
    /// rounding. Mildly biased in magnitude.
    ///
    /// Examples: `0.5 -> 1`, `1.5 -> 2`, `-0.5 -> -1`, `-1.5 -> -2`.
    HalfAwayFromZero,
    /// Round to nearest; on ties, round toward zero. Mildly biased
    /// toward zero. Rare in practice; included for completeness.
    ///
    /// Examples: `0.5 -> 0`, `1.5 -> 1`, `-0.5 -> 0`, `-1.5 -> -1`.
    HalfTowardZero,
    /// Truncate toward zero. Discards the fractional part. Cheapest
    /// in integer arithmetic; matches Rust's `as` cast for integer
    /// narrowing.
    ///
    /// Examples: `0.7 -> 0`, `-0.7 -> 0`, `1.9 -> 1`, `-1.9 -> -1`.
    Trunc,
    /// Round toward negative infinity (floor).
    ///
    /// Examples: `0.7 -> 0`, `-0.7 -> -1`, `1.9 -> 1`, `-1.9 -> -2`.
    Floor,
    /// Round toward positive infinity (ceiling).
    ///
    /// Examples: `0.7 -> 1`, `-0.7 -> 0`, `1.9 -> 2`, `-1.9 -> -1`.
    Ceiling,
}

/// Compile-time default `RoundingMode` for the no-arg `rescale` and
/// future default-rounding methods.
///
/// Selected by Cargo feature flags (priority order: first match wins):
/// 1. `rounding-half-away-from-zero` → `HalfAwayFromZero`
/// 2. `rounding-half-toward-zero` → `HalfTowardZero`
/// 3. `rounding-trunc` → `Trunc`
/// 4. `rounding-floor` → `Floor`
/// 5. `rounding-ceiling` → `Ceiling`
/// 6. (none) → `HalfToEven` (IEEE-754 default; banker's rounding)
#[cfg(feature = "rounding-half-away-from-zero")]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::HalfAwayFromZero;

#[cfg(all(
    not(feature = "rounding-half-away-from-zero"),
    feature = "rounding-half-toward-zero"
))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::HalfTowardZero;

#[cfg(all(
    not(feature = "rounding-half-away-from-zero"),
    not(feature = "rounding-half-toward-zero"),
    feature = "rounding-trunc"
))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::Trunc;

#[cfg(all(
    not(feature = "rounding-half-away-from-zero"),
    not(feature = "rounding-half-toward-zero"),
    not(feature = "rounding-trunc"),
    feature = "rounding-floor"
))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::Floor;

#[cfg(all(
    not(feature = "rounding-half-away-from-zero"),
    not(feature = "rounding-half-toward-zero"),
    not(feature = "rounding-trunc"),
    not(feature = "rounding-floor"),
    feature = "rounding-ceiling"
))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::Ceiling;

#[cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
)))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::HalfToEven;

/// Strategy hook for the rounding-mode family.
///
/// Given a *truncated-toward-zero* quotient and the per-operation
/// numerator / divisor context, returns `true` if the quotient should
/// be bumped one step "away from zero" in the result's direction to
/// satisfy this mode. Caller is responsible for the actual bump (it
/// is `q + 1` when the result is positive, `q − 1` when negative).
///
/// The three inputs collapse the per-step numerics that every mode
/// cares about into mode-independent booleans / orderings:
///
/// - `cmp_r` — three-way comparison of `|r|` against `|m| − |r|`. This
///   is exactly the round-up condition (`|r| > |m| − |r|` ⇔ `2·|r| > |m|`)
///   without the doubling-overflow risk. `Equal` flags the half-way tie,
///   which only occurs when the divisor is even.
/// - `q_is_odd` — parity of the truncated quotient. Drives the
///   half-to-even tie break.
/// - `result_positive` — sign of the true result (`sign(n) == sign(m)`).
///   Drives `Floor` / `Ceiling`.
///
/// Caller pre-handles the `r == 0` case (no rounding needed).
///
/// `#[inline(always)]` because the entire body is one match on a
/// 6-variant enum. The hot operator path instantiates this with a
/// const `mode` (`DEFAULT_ROUNDING_MODE`), so const-propagation can
/// collapse the match away once inlined.
#[inline(always)]
pub(crate) fn should_bump(
    mode: RoundingMode,
    cmp_r: ::core::cmp::Ordering,
    q_is_odd: bool,
    result_positive: bool,
) -> bool {
    use ::core::cmp::Ordering;
    match mode {
        RoundingMode::HalfToEven => match cmp_r {
            Ordering::Less => false,
            Ordering::Greater => true,
            Ordering::Equal => q_is_odd,
        },
        RoundingMode::HalfAwayFromZero => !matches!(cmp_r, Ordering::Less),
        RoundingMode::HalfTowardZero => matches!(cmp_r, Ordering::Greater),
        RoundingMode::Trunc => false,
        RoundingMode::Floor => !result_positive,
        RoundingMode::Ceiling => result_positive,
    }
}

/// `true` for the three round-to-nearest modes (`HalfToEven`,
/// `HalfAwayFromZero`, `HalfTowardZero`), `false` for the directed
/// modes (`Trunc`, `Floor`, `Ceiling`).
///
/// Kernels with a sub-LSB linear-approximation fast path (e.g.
/// `ln(1 + δ)` near `δ`, `exp(δ)` near `1 + δ`) may short-circuit only
/// under nearest rounding: those approximations land within half an LSB
/// of the true value, which is exactly what nearest rounding needs but
/// not enough for a directed mode, whose answer depends on which side of
/// the boundary the true value falls. Directed modes must fall through
/// to the full working-scale evaluation so the residual sign is known.
#[inline(always)]
pub(crate) const fn is_nearest_mode(mode: RoundingMode) -> bool {
    matches!(
        mode,
        RoundingMode::HalfToEven | RoundingMode::HalfAwayFromZero | RoundingMode::HalfTowardZero
    )
}

/// Correctly-rounded result of an odd, strictly-compressing function
/// (`tanh`) at a tiny argument, for any rounding mode.
///
/// For `tanh` the Maclaurin series is `tanh(x) = x − x³/3 + …`, an
/// alternating series in odd powers of `x`. Within the small-argument
/// linear band the cubic correction `|x|³/3` is below one storage ULP
/// yet strictly positive, so the true value `t = tanh(x)·10^SCALE`
/// satisfies, for `raw = x·10^SCALE`:
///
/// ```text
///   raw > 0 :  raw − 1 < t < raw          (just below the grid line raw)
///   raw < 0 :  raw     < t < raw + 1      (just above the grid line raw)
/// ```
///
/// i.e. `|t|` lies strictly inside `(|raw| − 1, |raw|)`. The result is
/// therefore exactly determined by integer arithmetic on `raw` — no
/// finite-precision kernel can resolve the sub-ULP cubic, so the
/// directed modes must use this analytic decision rather than rounding
/// the (grid-exact) linear approximation. The three nearest modes round
/// to `raw` (the cubic is well under half a ULP in the band).
///
/// `one` is the storage value `1`; `zero` the storage value `0`. The
/// caller guarantees `0 < |raw| <= threshold`, the band where the cubic
/// stays under one ULP.
#[inline]
pub(crate) fn tiny_odd_compressing_directed<T>(raw: T, zero: T, one: T, mode: RoundingMode) -> T
where
    T: Copy + PartialOrd + ::core::ops::Add<Output = T> + ::core::ops::Sub<Output = T>,
{
    if is_nearest_mode(mode) {
        return raw;
    }
    let positive = raw > zero;
    match mode {
        // Toward zero: drop the sub-ULP magnitude, landing on |raw| − 1.
        RoundingMode::Trunc => {
            if positive {
                raw - one
            } else {
                raw + one
            }
        }
        // Toward −∞.
        RoundingMode::Floor => {
            if positive {
                raw - one
            } else {
                raw
            }
        }
        // Toward +∞.
        RoundingMode::Ceiling => {
            if positive {
                raw
            } else {
                raw + one
            }
        }
        // Nearest modes handled above.
        _ => raw,
    }
}

/// Directed rounding for an odd transcendental whose true value at a
/// tiny argument sits just *above* the grid line `raw` in magnitude —
/// e.g. `sinh(x) = x + x³/6 + …`, where the cubic is strictly positive
/// but below one ULP. The mirror of [`tiny_odd_compressing_directed`]
/// (which handles the just-*below* case like `tanh`).
///
/// `raw` is the stored argument (= the leading term `x · 10^SCALE`),
/// `zero`/`one` the type's storage `0` / `1`. The true value lies in
/// `(|raw|, |raw| + 1)` in magnitude, so:
///
/// - nearest modes round to `raw` (the excess is < 0.5 ULP);
/// - toward-zero (`Trunc`) drops the excess → `raw`;
/// - `Floor` (toward −∞): `raw` if positive, `raw − 1` if negative;
/// - `Ceiling` (toward +∞): `raw + 1` if positive, `raw` if negative.
#[inline]
pub(crate) fn tiny_odd_expanding_directed<T>(raw: T, zero: T, one: T, mode: RoundingMode) -> T
where
    T: Copy + PartialOrd + ::core::ops::Add<Output = T> + ::core::ops::Sub<Output = T>,
{
    if is_nearest_mode(mode) {
        return raw;
    }
    let positive = raw > zero;
    match mode {
        // Toward zero: the excess is sub-ULP, so the magnitude stays at
        // `|raw|` — i.e. `raw` unchanged.
        RoundingMode::Trunc => raw,
        // Toward −∞.
        RoundingMode::Floor => {
            if positive {
                raw
            } else {
                raw - one
            }
        }
        // Toward +∞.
        RoundingMode::Ceiling => {
            if positive {
                raw + one
            } else {
                raw
            }
        }
        // Nearest modes handled above.
        _ => raw,
    }
}

/// Applies `mode` to integer division `raw / divisor`, returning the
/// rounded quotient.
///
/// Used by `D38::rescale_with` and by the multiplier-and-divide
/// fast paths in `mg_divide`. The whole mode-specific logic is
/// delegated to [`should_bump`]; this function is just the i128
/// arithmetic wrapper that builds its inputs and applies the bump.
#[inline(always)]
pub(crate) fn apply_rounding(raw: i128, divisor: i128, mode: RoundingMode) -> i128 {
    let quotient = raw / divisor;
    let remainder = raw % divisor;

    if remainder == 0 {
        return quotient;
    }

    let abs_rem = remainder.unsigned_abs();
    let abs_div = divisor.unsigned_abs();
    let comp = abs_div - abs_rem;
    let cmp_r = abs_rem.cmp(&comp);
    let q_is_odd = (quotient & 1) != 0;
    let result_positive = (raw < 0) == (divisor < 0);

    if should_bump(mode, cmp_r, q_is_odd, result_positive) {
        if result_positive {
            quotient + 1
        } else {
            quotient - 1
        }
    } else {
        quotient
    }
}

/// `2^52` — the threshold at or above which every finite `f64` is
/// already an exact integer (the mantissa can no longer represent a
/// fractional bit). Used by the libm-free `f64` rounding helpers to
/// short-circuit large magnitudes, which also keeps the `as i128`
/// truncation inside `i128` range (`2^52 < i128::MAX`).
const F64_INTEGER_THRESHOLD: f64 = 9_007_199_254_740_992.0_f64;

/// Truncate an `f64` toward zero, libm-free.
///
/// Equivalent to [`f64::trunc`] but built from arithmetic and `as`
/// casts only, so it is available in `no_std` without `libm`. For
/// magnitudes at or above `2^52` (already integral) and for non-finite
/// inputs the value is returned unchanged; otherwise the integral part
/// is recovered via an `i128` round-trip, which is exact in that range.
/// The negative-zero sign is preserved to match [`f64::trunc`] bit-for-bit.
#[inline]
pub(crate) fn trunc_f64(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    let magnitude = if x < 0.0 { -x } else { x };
    if magnitude >= F64_INTEGER_THRESHOLD {
        // NaN is already returned above, so `>=` is the exact complement of
        // `< THRESHOLD` here: already-integral / too-large magnitudes pass
        // through unchanged.
        return x;
    }
    let truncated = x as i128 as f64;
    if truncated == 0.0 && x.is_sign_negative() {
        -0.0
    } else {
        truncated
    }
}

/// Round an `f64` toward negative infinity, libm-free. Equivalent to
/// [`f64::floor`]: drop to the truncated value, then step down by one
/// when truncation rounded a negative value up toward zero.
#[inline]
pub(crate) fn floor_f64(x: f64) -> f64 {
    let truncated = trunc_f64(x);
    if truncated > x {
        truncated - 1.0
    } else {
        truncated
    }
}

/// Round an `f64` toward positive infinity, libm-free. Equivalent to
/// [`f64::ceil`]: the mirror of [`floor_f64`].
#[inline]
pub(crate) fn ceil_f64(x: f64) -> f64 {
    let truncated = trunc_f64(x);
    if truncated < x {
        truncated + 1.0
    } else {
        truncated
    }
}

/// Round an `f64` to the nearest integer, ties away from zero, libm-free.
/// Equivalent to [`f64::round`]: a fractional part with magnitude `>= 0.5`
/// steps the truncated value one away from zero.
#[inline]
pub(crate) fn round_half_away_f64(x: f64) -> f64 {
    let truncated = trunc_f64(x);
    let fraction = x - truncated;
    if fraction >= 0.5 {
        truncated + 1.0
    } else if fraction <= -0.5 {
        truncated - 1.0
    } else {
        truncated
    }
}

/// Round an `f64` to the nearest integer, ties to even, libm-free.
/// Equivalent to [`f64::round_ties_even`]: a fractional part strictly
/// past `0.5` in magnitude steps one away from zero; an exact half steps
/// only when the truncated value is odd, landing on the even neighbour.
#[inline]
pub(crate) fn round_half_even_f64(x: f64) -> f64 {
    let truncated = trunc_f64(x);
    let fraction = x - truncated;
    if fraction > 0.5 {
        truncated + 1.0
    } else if fraction < -0.5 {
        truncated - 1.0
    } else if fraction == 0.5 {
        if (truncated as i128) & 1 == 0 {
            truncated
        } else {
            truncated + 1.0
        }
    } else if fraction == -0.5 {
        if (truncated as i128) & 1 == 0 {
            truncated
        } else {
            truncated - 1.0
        }
    } else {
        truncated
    }
}

/// Round an `f64` to the nearest integer, ties toward zero, libm-free.
/// Reproduces the previous `std` formulation
/// (`(x - 0.5).ceil()` for `x >= 0`, `(x + 0.5).floor()` otherwise)
/// using the libm-free [`ceil_f64`] / [`floor_f64`].
#[inline]
pub(crate) fn round_half_toward_zero_f64(x: f64) -> f64 {
    if x >= 0.0 {
        ceil_f64(x - 0.5)
    } else {
        floor_f64(x + 0.5)
    }
}

/// `true` when the crate is built with [`DEFAULT_ROUNDING_MODE`] set to
/// [`RoundingMode::HalfToEven`] — i.e. none of the `rounding-*` feature
/// flags is selected. Used by tests whose expected values assume the
/// default IEEE-754 rounding to short-circuit themselves under a
/// non-default rounding feature build.
#[cfg(test)]
pub(crate) const DEFAULT_IS_HALF_TO_EVEN: bool =
    matches!(DEFAULT_ROUNDING_MODE, RoundingMode::HalfToEven);

#[cfg(test)]
mod tests {
    use super::*;

    fn modes() -> [RoundingMode; 6] {
        [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ]
    }

    /// Zero remainder is exact for every mode.
    #[test]
    fn zero_remainder_is_quotient_for_all_modes() {
        for m in modes() {
            assert_eq!(apply_rounding(20, 10, m), 2, "{m:?}");
            assert_eq!(apply_rounding(-20, 10, m), -2, "{m:?}");
            assert_eq!(apply_rounding(0, 10, m), 0, "{m:?}");
        }
    }

    /// Half-to-even: ties go to even neighbour.
    #[test]
    fn half_to_even_ties() {
        let m = RoundingMode::HalfToEven;
        assert_eq!(apply_rounding(5, 10, m), 0); // 0.5 -> 0 (even)
        assert_eq!(apply_rounding(15, 10, m), 2); // 1.5 -> 2
        assert_eq!(apply_rounding(25, 10, m), 2); // 2.5 -> 2 (even)
        assert_eq!(apply_rounding(35, 10, m), 4); // 3.5 -> 4
        assert_eq!(apply_rounding(-5, 10, m), 0); // -0.5 -> 0
        assert_eq!(apply_rounding(-15, 10, m), -2); // -1.5 -> -2
        assert_eq!(apply_rounding(-25, 10, m), -2); // -2.5 -> -2
        assert_eq!(apply_rounding(-35, 10, m), -4); // -3.5 -> -4
    }

    /// Half-away-from-zero: ties go away from zero.
    #[test]
    fn half_away_from_zero_ties() {
        let m = RoundingMode::HalfAwayFromZero;
        assert_eq!(apply_rounding(5, 10, m), 1);
        assert_eq!(apply_rounding(15, 10, m), 2);
        assert_eq!(apply_rounding(25, 10, m), 3);
        assert_eq!(apply_rounding(-5, 10, m), -1);
        assert_eq!(apply_rounding(-15, 10, m), -2);
        assert_eq!(apply_rounding(-25, 10, m), -3);
    }

    /// Half-toward-zero: ties go toward zero.
    #[test]
    fn half_toward_zero_ties() {
        let m = RoundingMode::HalfTowardZero;
        assert_eq!(apply_rounding(5, 10, m), 0);
        assert_eq!(apply_rounding(15, 10, m), 1);
        assert_eq!(apply_rounding(25, 10, m), 2);
        assert_eq!(apply_rounding(-5, 10, m), 0);
        assert_eq!(apply_rounding(-15, 10, m), -1);
        assert_eq!(apply_rounding(-25, 10, m), -2);
    }

    /// Trunc: always toward zero, regardless of magnitude.
    #[test]
    fn trunc_always_toward_zero() {
        let m = RoundingMode::Trunc;
        assert_eq!(apply_rounding(7, 10, m), 0);
        assert_eq!(apply_rounding(9, 10, m), 0);
        assert_eq!(apply_rounding(19, 10, m), 1);
        assert_eq!(apply_rounding(-7, 10, m), 0);
        assert_eq!(apply_rounding(-19, 10, m), -1);
    }

    /// Floor: always toward negative infinity.
    #[test]
    fn floor_toward_negative_infinity() {
        let m = RoundingMode::Floor;
        assert_eq!(apply_rounding(1, 10, m), 0);
        assert_eq!(apply_rounding(7, 10, m), 0);
        assert_eq!(apply_rounding(9, 10, m), 0);
        assert_eq!(apply_rounding(-1, 10, m), -1);
        assert_eq!(apply_rounding(-7, 10, m), -1);
        assert_eq!(apply_rounding(-19, 10, m), -2);
    }

    /// Ceiling: always toward positive infinity.
    #[test]
    fn ceiling_toward_positive_infinity() {
        let m = RoundingMode::Ceiling;
        assert_eq!(apply_rounding(1, 10, m), 1);
        assert_eq!(apply_rounding(7, 10, m), 1);
        assert_eq!(apply_rounding(19, 10, m), 2);
        assert_eq!(apply_rounding(-1, 10, m), 0);
        assert_eq!(apply_rounding(-7, 10, m), 0);
        assert_eq!(apply_rounding(-19, 10, m), -1);
    }

    /// Non-half values go to the nearest neighbour for every "half"
    /// mode and ignore the half-tie rule.
    #[test]
    fn non_half_goes_to_nearest() {
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
        ] {
            assert_eq!(apply_rounding(4, 10, m), 0, "{m:?} 0.4");
            assert_eq!(apply_rounding(6, 10, m), 1, "{m:?} 0.6");
            assert_eq!(apply_rounding(-4, 10, m), 0, "{m:?} -0.4");
            assert_eq!(apply_rounding(-6, 10, m), -1, "{m:?} -0.6");
        }
    }
}
