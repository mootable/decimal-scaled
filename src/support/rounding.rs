// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Rounding-mode selector for scale-narrowing operations.
//!
//! Passed to every `*_with(mode)` sibling on every decimal width —
//! [`crate::D38::quantize_with`], `mul_with`, `div_with`, `to_int_with`,
//! `from_f64_with`, every `*_strict_with` on the wide tier, etc. — to
//! control how fractional digits are discarded when the result has
//! lower precision than the working intermediate. The eight modes are
//! the full General Decimal Arithmetic rounding set: IEEE-754's five
//! rounding rules (`HalfToEven`, `HalfTowardZero`, `Trunc`, `Floor`,
//! `Ceiling`), the commercial `HalfAwayFromZero` rule expected by users
//! coming from `bigdecimal` / `rust_decimal`, and the two remaining GDA
//! rules `AwayFromZero` (`round-up`) and `ZeroFiveUp` (`round-05up`).
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
    /// Round away from zero whenever anything was discarded — the exact
    /// mirror of [`RoundingMode::Trunc`]. An exact value is never moved.
    /// GDA name: `round-up`.
    ///
    /// Examples: `0.1 -> 1`, `0.7 -> 1`, `-0.1 -> -1`, `-1.9 -> -2`,
    /// `2.0 -> 2`.
    AwayFromZero,
    /// Round away from zero **iff** the last retained digit of the
    /// toward-zero result is `0` or `5`; otherwise truncate. An exact
    /// value is never moved.
    ///
    /// Reserves `0` and `5` as the only final digits that can absorb a
    /// discarded remainder, so a later round to one fewer digit never
    /// sees a half-way tie that the first rounding manufactured — the
    /// "round for reround" rule. GDA name: `round-05up`.
    ///
    /// Examples (truncate to integer): `0.7 -> 1` (last digit `0`),
    /// `5.7 -> 6` (last digit `5`), `1.7 -> 1`, `4.7 -> 4`, `6.7 -> 6`,
    /// `-0.7 -> -1`, `-5.7 -> -6`, `-4.7 -> -4`, `2.0 -> 2`.
    ZeroFiveUp,
}

/// Compile-time default `RoundingMode` for the no-arg `quantize` and
/// future default-rounding methods.
///
/// Selected by Cargo feature flags (priority order: first match wins):
/// 1. `rounding-half-away-from-zero` → `HalfAwayFromZero`
/// 2. `rounding-half-toward-zero` → `HalfTowardZero`
/// 3. `rounding-trunc` → `Trunc`
/// 4. `rounding-floor` → `Floor`
/// 5. `rounding-ceiling` → `Ceiling`
/// 6. `rounding-away-from-zero` → `AwayFromZero`
/// 7. `rounding-zero-five-up` → `ZeroFiveUp`
/// 8. (none) → `HalfToEven` (IEEE-754 default; banker's rounding)
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

#[cfg(all(
    not(feature = "rounding-half-away-from-zero"),
    not(feature = "rounding-half-toward-zero"),
    not(feature = "rounding-trunc"),
    not(feature = "rounding-floor"),
    not(feature = "rounding-ceiling"),
    feature = "rounding-away-from-zero"
))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::AwayFromZero;

#[cfg(all(
    not(feature = "rounding-half-away-from-zero"),
    not(feature = "rounding-half-toward-zero"),
    not(feature = "rounding-trunc"),
    not(feature = "rounding-floor"),
    not(feature = "rounding-ceiling"),
    not(feature = "rounding-away-from-zero"),
    feature = "rounding-zero-five-up"
))]
pub const DEFAULT_ROUNDING_MODE: RoundingMode = RoundingMode::ZeroFiveUp;

#[cfg(not(any(
    feature = "rounding-half-away-from-zero",
    feature = "rounding-half-toward-zero",
    feature = "rounding-trunc",
    feature = "rounding-floor",
    feature = "rounding-ceiling",
    feature = "rounding-away-from-zero",
    feature = "rounding-zero-five-up",
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
/// - `remainder_cmp` — three-way comparison of `|r|` against `|m| − |r|`.
///   This
///   is exactly the round-up condition (`|r| > |m| − |r|` ⇔ `2·|r| > |m|`)
///   without the doubling-overflow risk. `Equal` flags the half-way tie,
///   which only occurs when the divisor is even.
/// - `q_mod_10` — the last decimal digit of the truncated quotient's
///   *magnitude* (`|q| % 10`, so always `0..=9`). Its parity
///   (`q_mod_10 & 1`) drives the half-to-even tie break — `10` is even,
///   so the last digit and the whole number share a parity — and its
///   value drives `ZeroFiveUp`, which is why the digit rather than the
///   parity is the parameter.
/// - `result_positive` — sign of the true result (`sign(n) == sign(m)`).
///   Drives `Floor` / `Ceiling`.
///
/// Caller pre-handles the `r == 0` case (no rounding needed). That
/// contract is what lets `AwayFromZero` be an unconditional `true` and
/// `ZeroFiveUp` test only the digit: both are defined to leave an exact
/// value alone, and an exact value never reaches here.
///
/// `#[inline(always)]` because the entire body is one match on an
/// 8-variant enum. The hot operator path instantiates this with a
/// const `mode` (`DEFAULT_ROUNDING_MODE`), so const-propagation can
/// collapse the match away once inlined.
#[inline(always)]
pub(crate) fn should_bump(
    mode: RoundingMode,
    remainder_cmp: ::core::cmp::Ordering,
    q_mod_10: u8,
    result_positive: bool,
) -> bool {
    use ::core::cmp::Ordering;
    match mode {
        RoundingMode::HalfToEven => match remainder_cmp {
            Ordering::Less => false,
            Ordering::Greater => true,
            Ordering::Equal => q_mod_10 & 1 == 1,
        },
        RoundingMode::HalfAwayFromZero => !matches!(remainder_cmp, Ordering::Less),
        RoundingMode::HalfTowardZero => matches!(remainder_cmp, Ordering::Greater),
        RoundingMode::Trunc => false,
        RoundingMode::Floor => !result_positive,
        RoundingMode::Ceiling => result_positive,
        RoundingMode::AwayFromZero => true,
        RoundingMode::ZeroFiveUp => q_mod_10 == 0 || q_mod_10 == 5,
    }
}

/// Last decimal digit of a little-endian `u64`-limb MAGNITUDE — the
/// `q_mod_10` input to [`should_bump`] for the kernels whose truncated
/// quotient is a limb slice rather than a typed integer (`div_widen_scale`,
/// `mul_schoolbook`, `barrett_reciprocal`, `mg_divide`, `fixed`). Those
/// sites read `quot[0] & 1` for the parity the tie break used to need;
/// the last DECIMAL digit is not carried by one limb, so it needs the
/// whole magnitude.
///
/// `2^64 ≡ 6 (mod 10)`, and `6^k ≡ 6 (mod 10)` for every `k >= 1`, so
///
/// ```text
///   Σ limb_i · 2^(64·i)  ≡  limb_0 + 6 · Σ_{i>=1} limb_i   (mod 10)
/// ```
///
/// which replaces an O(limbs) long division with one `u64 % 10` per limb
/// — a constant divisor, so the compiler lowers each to a multiply-shift.
/// The accumulator cannot overflow: each term is at most `6·9 = 54`.
///
/// `mag` must be a magnitude (the sign lives outside the limbs, as it
/// does at every call site); an empty slice is the value `0`.
///
/// `const` because `consts::table`'s compile-time-baked constant
/// accessors (`pi_const_n` and siblings) are `const fn` and need the
/// pivot digit of their floor entry; hence the `while` loop rather than
/// an iterator.
#[inline]
pub(crate) const fn limbs_mod_10(mag: &[u64]) -> u8 {
    let mut acc: u32 = 0;
    let mut i = 0;
    while i < mag.len() {
        let digit = (mag[i] % 10) as u32;
        acc += if i == 0 { digit } else { 6 * digit };
        i += 1;
    }
    (acc % 10) as u8
}

/// Last decimal digit of a little-endian `u128`-limb MAGNITUDE — the
/// u128-packed sibling of [`limbs_mod_10`], for the kernels that carry
/// their quotient in u128 limbs (`barrett_reciprocal`'s u128 path).
///
/// The same fold applies unchanged: `2^128 = (2^64)^2 ≡ 6^2 = 36 ≡ 6
/// (mod 10)`, so the base's residue is `6` here too.
#[inline]
pub(crate) fn limbs_u128_mod_10(mag: &[u128]) -> u8 {
    let mut acc: u32 = 0;
    for (i, &limb) in mag.iter().enumerate() {
        let digit = (limb % 10) as u32;
        acc += if i == 0 { digit } else { 6 * digit };
    }
    (acc % 10) as u8
}

/// `true` for the three round-to-nearest modes (`HalfToEven`,
/// `HalfAwayFromZero`, `HalfTowardZero`), `false` for the directed
/// modes (`Trunc`, `Floor`, `Ceiling`, `AwayFromZero`, `ZeroFiveUp`).
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
///
/// `raw_mod_10` is `|raw| % 10`, supplied by the caller because `T` here
/// carries no division. Only `ZeroFiveUp` reads it: that mode needs the
/// last digit of the *toward-zero* result, whose magnitude is `|raw| − 1`
/// — hence the `+ 9` borrow step below, exact because `|raw| >= 1`.
#[inline]
pub(crate) fn tiny_odd_compressing_directed<T>(
    raw: T,
    zero: T,
    one: T,
    raw_mod_10: u8,
    mode: RoundingMode,
) -> T
where
    T: Copy + PartialOrd + ::core::ops::Add<Output = T> + ::core::ops::Sub<Output = T>,
{
    if is_nearest_mode(mode) {
        return raw;
    }
    let is_positive = raw > zero;
    // The toward-zero result: magnitude |raw| − 1.
    let toward_zero = if is_positive { raw - one } else { raw + one };
    match mode {
        // Toward zero: drop the sub-ULP magnitude, landing on |raw| − 1.
        RoundingMode::Trunc => toward_zero,
        // Toward −∞.
        RoundingMode::Floor => {
            if is_positive {
                raw - one
            } else {
                raw
            }
        }
        // Toward +∞.
        RoundingMode::Ceiling => {
            if is_positive {
                raw
            } else {
                raw + one
            }
        }
        // The discarded part is non-zero throughout the band, so away
        // from zero is always the full magnitude |raw|.
        RoundingMode::AwayFromZero => raw,
        // Last digit of |raw| − 1; bump to |raw| only on 0 or 5.
        RoundingMode::ZeroFiveUp => {
            let d = (raw_mod_10 + 9) % 10;
            if d == 0 || d == 5 {
                raw
            } else {
                toward_zero
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
/// - `Ceiling` (toward +∞): `raw + 1` if positive, `raw` if negative;
/// - `AwayFromZero`: `raw + 1` if positive, `raw − 1` if negative;
/// - `ZeroFiveUp`: as `AwayFromZero` when `|raw| % 10` is `0` or `5`,
///   otherwise `raw`.
///
/// `raw_mod_10` is `|raw| % 10`, supplied by the caller because `T` here
/// carries no division. Only `ZeroFiveUp` reads it, and here the
/// toward-zero result *is* `raw`, so the digit needs no adjustment.
#[inline]
pub(crate) fn tiny_odd_expanding_directed<T>(
    raw: T,
    zero: T,
    one: T,
    raw_mod_10: u8,
    mode: RoundingMode,
) -> T
where
    T: Copy + PartialOrd + ::core::ops::Add<Output = T> + ::core::ops::Sub<Output = T>,
{
    if is_nearest_mode(mode) {
        return raw;
    }
    let is_positive = raw > zero;
    // One step away from zero from the toward-zero result `raw`.
    let away = if is_positive { raw + one } else { raw - one };
    match mode {
        // Toward zero: the excess is sub-ULP, so the magnitude stays at
        // `|raw|` — i.e. `raw` unchanged.
        RoundingMode::Trunc => raw,
        // Toward −∞.
        RoundingMode::Floor => {
            if is_positive {
                raw
            } else {
                raw - one
            }
        }
        // Toward +∞.
        RoundingMode::Ceiling => {
            if is_positive {
                raw + one
            } else {
                raw
            }
        }
        // The discarded excess is non-zero throughout the band.
        RoundingMode::AwayFromZero => away,
        // The toward-zero result is `raw` itself, so its last digit is
        // `raw_mod_10` directly.
        RoundingMode::ZeroFiveUp => {
            if raw_mod_10 == 0 || raw_mod_10 == 5 {
                away
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
/// Used by the multiplier-and-divide fast paths in `mg_divide`. The
/// magnitude-slice kernels there call [`should_bump`] directly instead,
/// having already produced the quotient and remainder in one pass. The
/// whole mode-specific logic is
/// delegated to [`should_bump`]; this function is just the i128
/// arithmetic wrapper that builds its inputs and applies the bump.
#[inline(always)]
pub(crate) fn apply_rounding(raw: i128, divisor: i128, mode: RoundingMode) -> i128 {
    let quotient = raw / divisor;
    let remainder = raw % divisor;

    if remainder == 0 {
        return quotient;
    }

    let abs_remainder = remainder.unsigned_abs();
    let abs_divisor = divisor.unsigned_abs();
    let complement = abs_divisor - abs_remainder;
    let remainder_cmp = abs_remainder.cmp(&complement);
    // Last decimal digit of |quotient|: `%` on a negative `i128` keeps the
    // sign, so take the magnitude of the one-digit remainder.
    let q_mod_10 = (quotient % 10).unsigned_abs() as u8;
    let result_positive = (raw < 0) == (divisor < 0);

    if should_bump(mode, remainder_cmp, q_mod_10, result_positive) {
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
pub(crate) fn trunc_f64(value: f64) -> f64 {
    if value.is_nan() {
        return value;
    }
    let magnitude = if value < 0.0 { -value } else { value };
    if magnitude >= F64_INTEGER_THRESHOLD {
        // NaN is already returned above, so `>=` is the exact complement of
        // `< THRESHOLD` here: already-integral / too-large magnitudes pass
        // through unchanged.
        return value;
    }
    let truncated = value as i128 as f64;
    if truncated == 0.0 && value.is_sign_negative() {
        -0.0
    } else {
        truncated
    }
}

/// Round an `f64` toward negative infinity, libm-free. Equivalent to
/// [`f64::floor`]: drop to the truncated value, then step down by one
/// when truncation rounded a negative value up toward zero.
#[inline]
pub(crate) fn floor_f64(value: f64) -> f64 {
    let truncated = trunc_f64(value);
    if truncated > value {
        truncated - 1.0
    } else {
        truncated
    }
}

/// Round an `f64` toward positive infinity, libm-free. Equivalent to
/// [`f64::ceil`]: the mirror of [`floor_f64`].
#[inline]
pub(crate) fn ceil_f64(value: f64) -> f64 {
    let truncated = trunc_f64(value);
    if truncated < value {
        truncated + 1.0
    } else {
        truncated
    }
}

/// Round an `f64` to the nearest integer, ties away from zero, libm-free.
/// Equivalent to [`f64::round`]: a fractional part with magnitude `>= 0.5`
/// steps the truncated value one away from zero.
#[inline]
pub(crate) fn round_half_away_f64(value: f64) -> f64 {
    let truncated = trunc_f64(value);
    let fraction = value - truncated;
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
pub(crate) fn round_half_even_f64(value: f64) -> f64 {
    let truncated = trunc_f64(value);
    let fraction = value - truncated;
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
pub(crate) fn round_half_toward_zero_f64(value: f64) -> f64 {
    if value >= 0.0 {
        ceil_f64(value - 0.5)
    } else {
        floor_f64(value + 0.5)
    }
}

/// Round an `f64` away from zero whenever it has a fractional part,
/// libm-free — the `f64` form of [`RoundingMode::AwayFromZero`] and the
/// exact mirror of [`trunc_f64`]. An already-integral value (which
/// includes every magnitude at or above `2^52`, and the non-finite
/// values) passes through unchanged.
#[inline]
pub(crate) fn away_from_zero_f64(x: f64) -> f64 {
    let truncated = trunc_f64(x);
    if truncated == x {
        return truncated;
    }
    if x > 0.0 {
        truncated + 1.0
    } else {
        truncated - 1.0
    }
}

/// Round an `f64` away from zero iff the last decimal digit of the
/// truncated value is `0` or `5`, libm-free — the `f64` form of
/// [`RoundingMode::ZeroFiveUp`]. An already-integral value passes
/// through unchanged, so the `as i128` digit read only runs where
/// [`trunc_f64`] took its exact `i128` round-trip (magnitude below
/// `2^52`).
#[inline]
pub(crate) fn zero_five_up_f64(x: f64) -> f64 {
    let truncated = trunc_f64(x);
    if truncated == x {
        return truncated;
    }
    let last_digit = (truncated as i128 % 10).unsigned_abs();
    if last_digit == 0 || last_digit == 5 {
        if x > 0.0 {
            truncated + 1.0
        } else {
            truncated - 1.0
        }
    } else {
        truncated
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

    fn modes() -> [RoundingMode; 8] {
        [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Trunc,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
            RoundingMode::AwayFromZero,
            RoundingMode::ZeroFiveUp,
        ]
    }

    /// Zero remainder is exact for every mode.
    #[test]
    fn zero_remainder_is_quotient_for_all_modes() {
        for mode in modes() {
            assert_eq!(apply_rounding(20, 10, mode), 2, "{mode:?}");
            assert_eq!(apply_rounding(-20, 10, mode), -2, "{mode:?}");
            assert_eq!(apply_rounding(0, 10, mode), 0, "{mode:?}");
        }
    }

    /// Half-to-even: ties go to even neighbour.
    #[test]
    fn half_to_even_ties() {
        let mode = RoundingMode::HalfToEven;
        assert_eq!(apply_rounding(5, 10, mode), 0); // 0.5 -> 0 (even)
        assert_eq!(apply_rounding(15, 10, mode), 2); // 1.5 -> 2
        assert_eq!(apply_rounding(25, 10, mode), 2); // 2.5 -> 2 (even)
        assert_eq!(apply_rounding(35, 10, mode), 4); // 3.5 -> 4
        assert_eq!(apply_rounding(-5, 10, mode), 0); // -0.5 -> 0
        assert_eq!(apply_rounding(-15, 10, mode), -2); // -1.5 -> -2
        assert_eq!(apply_rounding(-25, 10, mode), -2); // -2.5 -> -2
        assert_eq!(apply_rounding(-35, 10, mode), -4); // -3.5 -> -4
    }

    /// Half-away-from-zero: ties go away from zero.
    #[test]
    fn half_away_from_zero_ties() {
        let mode = RoundingMode::HalfAwayFromZero;
        assert_eq!(apply_rounding(5, 10, mode), 1);
        assert_eq!(apply_rounding(15, 10, mode), 2);
        assert_eq!(apply_rounding(25, 10, mode), 3);
        assert_eq!(apply_rounding(-5, 10, mode), -1);
        assert_eq!(apply_rounding(-15, 10, mode), -2);
        assert_eq!(apply_rounding(-25, 10, mode), -3);
    }

    /// Half-toward-zero: ties go toward zero.
    #[test]
    fn half_toward_zero_ties() {
        let mode = RoundingMode::HalfTowardZero;
        assert_eq!(apply_rounding(5, 10, mode), 0);
        assert_eq!(apply_rounding(15, 10, mode), 1);
        assert_eq!(apply_rounding(25, 10, mode), 2);
        assert_eq!(apply_rounding(-5, 10, mode), 0);
        assert_eq!(apply_rounding(-15, 10, mode), -1);
        assert_eq!(apply_rounding(-25, 10, mode), -2);
    }

    /// Trunc: always toward zero, regardless of magnitude.
    #[test]
    fn trunc_always_toward_zero() {
        let mode = RoundingMode::Trunc;
        assert_eq!(apply_rounding(7, 10, mode), 0);
        assert_eq!(apply_rounding(9, 10, mode), 0);
        assert_eq!(apply_rounding(19, 10, mode), 1);
        assert_eq!(apply_rounding(-7, 10, mode), 0);
        assert_eq!(apply_rounding(-19, 10, mode), -1);
    }

    /// Floor: always toward negative infinity.
    #[test]
    fn floor_toward_negative_infinity() {
        let mode = RoundingMode::Floor;
        assert_eq!(apply_rounding(1, 10, mode), 0);
        assert_eq!(apply_rounding(7, 10, mode), 0);
        assert_eq!(apply_rounding(9, 10, mode), 0);
        assert_eq!(apply_rounding(-1, 10, mode), -1);
        assert_eq!(apply_rounding(-7, 10, mode), -1);
        assert_eq!(apply_rounding(-19, 10, mode), -2);
    }

    /// Ceiling: always toward positive infinity.
    #[test]
    fn ceiling_toward_positive_infinity() {
        let mode = RoundingMode::Ceiling;
        assert_eq!(apply_rounding(1, 10, mode), 1);
        assert_eq!(apply_rounding(7, 10, mode), 1);
        assert_eq!(apply_rounding(19, 10, mode), 2);
        assert_eq!(apply_rounding(-1, 10, mode), 0);
        assert_eq!(apply_rounding(-7, 10, mode), 0);
        assert_eq!(apply_rounding(-19, 10, mode), -1);
    }

    /// Non-half values go to the nearest neighbour for every "half"
    /// mode and ignore the half-tie rule.
    #[test]
    fn non_half_goes_to_nearest() {
        for mode in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
        ] {
            assert_eq!(apply_rounding(4, 10, mode), 0, "{mode:?} 0.4");
            assert_eq!(apply_rounding(6, 10, mode), 1, "{mode:?} 0.6");
            assert_eq!(apply_rounding(-4, 10, mode), 0, "{mode:?} -0.4");
            assert_eq!(apply_rounding(-6, 10, mode), -1, "{mode:?} -0.6");
        }
    }

    /// Away-from-zero (GDA `round-up`): any non-zero discard steps the
    /// magnitude up; an exact value is left alone. The mirror of `Trunc`.
    #[test]
    fn away_from_zero_lifts_every_discard() {
        let m = RoundingMode::AwayFromZero;
        assert_eq!(apply_rounding(1, 10, m), 1); // 0.1 -> 1
        assert_eq!(apply_rounding(5, 10, m), 1); // 0.5 -> 1
        assert_eq!(apply_rounding(9, 10, m), 1); // 0.9 -> 1
        assert_eq!(apply_rounding(11, 10, m), 2); // 1.1 -> 2
        assert_eq!(apply_rounding(19, 10, m), 2); // 1.9 -> 2
        assert_eq!(apply_rounding(-1, 10, m), -1);
        assert_eq!(apply_rounding(-5, 10, m), -1);
        assert_eq!(apply_rounding(-11, 10, m), -2);
        assert_eq!(apply_rounding(-19, 10, m), -2);
        // Exact: nothing discarded, so nothing moves.
        assert_eq!(apply_rounding(20, 10, m), 2);
        assert_eq!(apply_rounding(-20, 10, m), -2);
    }

    /// Zero-five-up (GDA `round-05up`): the discard lifts the magnitude
    /// only when the last retained digit is `0` or `5`.
    #[test]
    fn zero_five_up_pivots_on_zero_and_five() {
        let m = RoundingMode::ZeroFiveUp;
        // Last retained digit 0 -> bump.
        assert_eq!(apply_rounding(7, 10, m), 1); // q = 0
        assert_eq!(apply_rounding(107, 10, m), 11); // q = 10
        // Last retained digit 5 -> bump.
        assert_eq!(apply_rounding(57, 10, m), 6); // q = 5
        assert_eq!(apply_rounding(157, 10, m), 16); // q = 15
        // Every other last digit truncates — 4 and 6 straddle the 5.
        assert_eq!(apply_rounding(47, 10, m), 4);
        assert_eq!(apply_rounding(67, 10, m), 6);
        assert_eq!(apply_rounding(17, 10, m), 1);
        assert_eq!(apply_rounding(97, 10, m), 9);
        // Nothing discarded: no bump even on a 0 or 5 last digit.
        assert_eq!(apply_rounding(50, 10, m), 5);
        assert_eq!(apply_rounding(40, 10, m), 4);
        assert_eq!(apply_rounding(0, 10, m), 0);
    }

    /// Zero-five-up on the negative side: the pivot is the last digit of
    /// the MAGNITUDE, and the bump is away from zero.
    #[test]
    fn zero_five_up_is_sign_symmetric() {
        let m = RoundingMode::ZeroFiveUp;
        assert_eq!(apply_rounding(-7, 10, m), -1); // q = 0
        assert_eq!(apply_rounding(-107, 10, m), -11); // q = -10
        assert_eq!(apply_rounding(-57, 10, m), -6); // q = -5
        assert_eq!(apply_rounding(-157, 10, m), -16); // q = -15
        assert_eq!(apply_rounding(-47, 10, m), -4);
        assert_eq!(apply_rounding(-67, 10, m), -6);
        assert_eq!(apply_rounding(-17, 10, m), -1);
        assert_eq!(apply_rounding(-50, 10, m), -5);
        assert_eq!(apply_rounding(-40, 10, m), -4);
    }

    /// The half-to-even tie break reads `q_mod_10 & 1`, which must agree
    /// with the parity of the whole quotient — `10` is even, so the last
    /// digit carries it. Guards the `q_is_odd` -> `q_mod_10` migration at
    /// multi-digit quotients, which the single-digit ties above miss.
    #[test]
    fn half_to_even_parity_holds_past_one_digit() {
        let m = RoundingMode::HalfToEven;
        assert_eq!(apply_rounding(105, 10, m), 10); // 10.5 -> 10 (even)
        assert_eq!(apply_rounding(125, 10, m), 12); // 12.5 -> 12 (even)
        assert_eq!(apply_rounding(135, 10, m), 14); // 13.5 -> 14
        assert_eq!(apply_rounding(195, 10, m), 20); // 19.5 -> 20
        assert_eq!(apply_rounding(-105, 10, m), -10);
        assert_eq!(apply_rounding(-135, 10, m), -14);
    }

    /// The `f64` siblings of the two new modes must agree with the
    /// integer path on the same values.
    #[test]
    fn f64_helpers_match_the_new_modes() {
        assert_eq!(away_from_zero_f64(0.1), 1.0);
        assert_eq!(away_from_zero_f64(1.9), 2.0);
        assert_eq!(away_from_zero_f64(-0.1), -1.0);
        assert_eq!(away_from_zero_f64(-1.9), -2.0);
        assert_eq!(away_from_zero_f64(2.0), 2.0);
        assert_eq!(away_from_zero_f64(-2.0), -2.0);

        assert_eq!(zero_five_up_f64(0.7), 1.0); // last digit 0
        assert_eq!(zero_five_up_f64(5.7), 6.0); // last digit 5
        assert_eq!(zero_five_up_f64(4.7), 4.0);
        assert_eq!(zero_five_up_f64(6.7), 6.0);
        assert_eq!(zero_five_up_f64(10.7), 11.0);
        assert_eq!(zero_five_up_f64(-0.7), -1.0);
        assert_eq!(zero_five_up_f64(-5.7), -6.0);
        assert_eq!(zero_five_up_f64(-4.7), -4.0);
        assert_eq!(zero_five_up_f64(5.0), 5.0); // exact: no bump
    }

    /// The tiny-argument analytic helpers under the two new modes. The
    /// compressing band's true value sits in `(|raw| - 1, |raw|)`, the
    /// expanding band's in `(|raw|, |raw| + 1)`.
    #[test]
    fn tiny_odd_helpers_carry_the_new_modes() {
        let away = RoundingMode::AwayFromZero;
        let zfu = RoundingMode::ZeroFiveUp;

        // Compressing: toward-zero result is |raw| - 1.
        // raw = 7 -> toward-zero 6, last digit 6 -> no bump.
        assert_eq!(tiny_odd_compressing_directed(7_i128, 0, 1, 7, zfu), 6);
        // raw = 6 -> toward-zero 5, last digit 5 -> bump to 6.
        assert_eq!(tiny_odd_compressing_directed(6_i128, 0, 1, 6, zfu), 6);
        // raw = 1 -> toward-zero 0, last digit 0 -> bump to 1.
        assert_eq!(tiny_odd_compressing_directed(1_i128, 0, 1, 1, zfu), 1);
        assert_eq!(tiny_odd_compressing_directed(7_i128, 0, 1, 7, away), 7);
        assert_eq!(tiny_odd_compressing_directed(-7_i128, 0, 1, 7, away), -7);
        assert_eq!(tiny_odd_compressing_directed(-7_i128, 0, 1, 7, zfu), -6);
        assert_eq!(tiny_odd_compressing_directed(-6_i128, 0, 1, 6, zfu), -6);

        // Expanding: toward-zero result is |raw| itself.
        assert_eq!(tiny_odd_expanding_directed(7_i128, 0, 1, 7, away), 8);
        assert_eq!(tiny_odd_expanding_directed(-7_i128, 0, 1, 7, away), -8);
        assert_eq!(tiny_odd_expanding_directed(7_i128, 0, 1, 7, zfu), 7);
        assert_eq!(tiny_odd_expanding_directed(5_i128, 0, 1, 5, zfu), 6);
        assert_eq!(tiny_odd_expanding_directed(10_i128, 0, 1, 0, zfu), 11);
        assert_eq!(tiny_odd_expanding_directed(-5_i128, 0, 1, 5, zfu), -6);

        // Nearest modes are untouched by the new arms.
        for m in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
        ] {
            assert_eq!(tiny_odd_compressing_directed(7_i128, 0, 1, 7, m), 7, "{m:?}");
            assert_eq!(tiny_odd_expanding_directed(7_i128, 0, 1, 7, m), 7, "{m:?}");
        }
    }

    /// `limbs_mod_10` must agree with `u128 % 10` across the limb
    /// boundary, which is where the `2^64 ≡ 6 (mod 10)` fold earns its
    /// keep — a single-limb read would answer for the low limb only.
    #[test]
    fn limbs_mod_10_matches_u128_remainder() {
        for v in [
            0_u128,
            1,
            5,
            9,
            10,
            1234567890,
            u64::MAX as u128,
            (u64::MAX as u128) + 1,
            (u64::MAX as u128) + 5,
            1_u128 << 64,
            (1_u128 << 64) + 7,
            (3_u128 << 64) + 4,
            u128::MAX,
            u128::MAX - 3,
        ] {
            let limbs = [v as u64, (v >> 64) as u64];
            assert_eq!(
                u32::from(limbs_mod_10(&limbs)),
                (v % 10) as u32,
                "value {v}"
            );
        }
        // A one-limb slice and the empty (zero) slice.
        assert_eq!(limbs_mod_10(&[47]), 7);
        assert_eq!(limbs_mod_10(&[]), 0);
        // Three limbs: 2^128 mod 10 = 6, so the top limb folds in too.
        assert_eq!(limbs_mod_10(&[0, 0, 1]), 6);
        assert_eq!(limbs_mod_10(&[0, 1, 0]), 6);
        assert_eq!(limbs_mod_10(&[0, 1, 1]), 2);
    }

    /// Both new modes are DIRECTED: they must not be granted the
    /// sub-LSB linear-approximation short circuit.
    #[test]
    fn new_modes_are_not_nearest() {
        assert!(!is_nearest_mode(RoundingMode::AwayFromZero));
        assert!(!is_nearest_mode(RoundingMode::ZeroFiveUp));
    }
}
