// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Width-generic, value-preserving decimal magnitude conversion.
//!
//! The decimal `convert_from` / `convert_from_with` constructors emitted
//! on every concrete tier compose a *cross-width* and a *cross-scale*
//! step on the stored magnitude. Doing the scale change at too narrow a
//! width would let a legitimate value spuriously overflow, so the order
//! of the two steps depends on whether the target storage is wider or
//! narrower than the source storage.
//!
//! [`convert_magnitude`] performs both steps on the raw [`BigInt`]
//! magnitudes, branching on the *limb counts* of the two concrete
//! storages (a plain `const` comparison that folds away — no
//! `generic_const_exprs`, no computed `Int<max(N, M)>` intermediate):
//!
//! - **Target ≥ source (widen / equal):** widen the source magnitude
//!   into the target storage first (lossless), then rescale at the
//!   target width. A scale-up that overflows the target storage returns
//!   [`ConvertError::Overflow`].
//! - **Target < source (narrow):** rescale at the source width first
//!   (scale-down only shrinks the magnitude; scale-up that overflows the
//!   source storage returns [`ConvertError::Overflow`]), then narrow the
//!   magnitude into the target storage. A magnitude that no longer fits
//!   the target returns [`ConvertError::Overflow`].
//!
//! Scale-DOWN never errors: it rounds the discarded fractional digits
//! per the supplied [`RoundingMode`] and returns `Ok`.

use crate::int::types::BigInt;
use crate::support::error::ConvertError;
use crate::support::rounding::{should_bump, RoundingMode};

/// Rescales a [`BigInt`] magnitude from scale `source_scale` to scale
/// `target_scale` at its own width, applying `mode` to any scale-down
/// rounding.
///
/// - `target_scale == source_scale`: returns `value` unchanged.
/// - `target_scale > source_scale` (scale-up): multiplies by
///   `10^(target_scale - source_scale)`; returns `None` if that overflows
///   `T`'s range (the caller maps this to [`ConvertError::Overflow`]).
/// - `target_scale < source_scale` (scale-down): divides by
///   `10^(source_scale - target_scale)` and rounds per `mode`; always
///   `Some` (the magnitude only shrinks).
#[inline]
pub(crate) fn rescale_bigint<T: BigInt>(
    value: T,
    source_scale: u32,
    target_scale: u32,
    mode: RoundingMode
) -> Option<T> {
    if target_scale == source_scale {
        return Some(value);
    }
    if target_scale > source_scale {
        let shift = target_scale - source_scale;
        let multiplier = T::TEN.checked_pow(shift)?;
        return value.checked_mul(multiplier);
    }
    // Scale-down: divide by 10^shift with rounding.
    let shift = source_scale - target_scale;
    let divisor = match T::TEN.checked_pow(shift) {
        Some(power_of_ten) => power_of_ten,
        // 10^shift exceeds T's range: every in-range magnitude is
        // strictly smaller than the divisor, so the truncated quotient
        // is 0 and the remainder is the whole value. Round 0 per mode.
        None => return Some(round_when_quotient_zero(value, mode)),
    };
    let (quotient, remainder) = value.div_rem(divisor);
    if remainder == T::ZERO {
        return Some(quotient);
    }
    let abs_remainder = magnitude(remainder);
    let abs_divisor = magnitude(divisor);
    // `remainder_cmp`: |r| vs |divisor| - |r|, i.e. the round-up boundary
    // `2|r| vs |divisor|` without the doubling-overflow risk.
    let remainder_cmp = abs_remainder.cmp(&(abs_divisor - abs_remainder));
    // Last decimal digit of |quotient| (a wide `div_rem`, so O(limbs)).
    let q_mod_10 = quotient.div_rem(T::TEN).1.to_i128().unsigned_abs() as u8;
    let result_positive = (value < T::ZERO) == (divisor < T::ZERO);
    if should_bump(mode, remainder_cmp, q_mod_10, result_positive) {
        if result_positive {
            Some(quotient + T::ONE)
        } else {
            Some(quotient - T::ONE)
        }
    } else {
        Some(quotient)
    }
}

/// Magnitude (absolute value) of a signed [`BigInt`].
#[inline]
fn magnitude<T: BigInt>(value: T) -> T {
    if value < T::ZERO {
        T::ZERO - value
    } else {
        value
    }
}

/// Rounds the value `0.x` where the truncated quotient is `0` and the
/// remainder is the whole `value` (the divisor exceeds `T`'s range, so
/// `|value| < |divisor|`, meaning `|value| < |divisor| - |value|`, i.e.
/// strictly below the half boundary). Only the modes that step away from
/// zero on a bare discard can bump to `±1`: `Ceiling` for positive,
/// `Floor` for negative, `AwayFromZero` for either — and `ZeroFiveUp`,
/// whose last retained digit here is the `0` of the zero quotient.
#[inline]
fn round_when_quotient_zero<T: BigInt>(value: T, mode: RoundingMode) -> T {
    if value == T::ZERO {
        return T::ZERO;
    }
    let result_positive = value > T::ZERO;
    // `remainder_cmp == Less`: |r| is strictly below the half boundary
    // because the divisor strictly exceeds |value|.
    let remainder_cmp = core::cmp::Ordering::Less;
    // Truncated quotient is 0, so its last decimal digit is 0 (and even).
    if should_bump(mode, remainder_cmp, 0, result_positive) {
        if result_positive {
            T::ONE
        } else {
            T::ZERO - T::ONE
        }
    } else {
        T::ZERO
    }
}

/// Converts a source magnitude `source` (scale `source_scale`, width
/// `Src`) to the target magnitude type `Dst` at scale `target_scale`,
/// rounding any scale-down per `mode`.
///
/// The width-comparison branch uses only `Src::LIMBS` / `Dst::LIMBS`
/// (concrete `const usize` on each storage) — no computed-width
/// intermediate type, so this compiles on stable Rust.
///
/// # Errors
///
/// Returns [`ConvertError::Overflow`] when a scale-up overflows the
/// working storage, or when the rescaled magnitude does not fit `Dst`.
#[inline]
pub(crate) fn convert_magnitude<Src, Dst>(
    source: Src,
    source_scale: u32,
    target_scale: u32,
    mode: RoundingMode,
) -> Result<Dst, ConvertError>
where
    Src: BigInt,
    Dst: BigInt,
{
    if Dst::LIMBS >= Src::LIMBS {
        // Widen / equal: widen the magnitude into `Dst` first (lossless,
        // since `Dst`'s range covers `Src`'s), then rescale at the wider
        // (target) width so a legitimate scale-up cannot spuriously
        // overflow the narrower source storage.
        let widened: Dst = source.resize_to::<Dst>();
        rescale_bigint(widened, source_scale, target_scale, mode).ok_or(ConvertError::Overflow)
    } else {
        // Narrow: rescale at the source (wider) width first — scale-down
        // only shrinks, and a scale-up that overflows the source is a
        // genuine error — then narrow the magnitude into `Dst`.
        let rescaled: Src = rescale_bigint(source, source_scale, target_scale, mode)
            .ok_or(ConvertError::Overflow)?;
        // Fallible signed narrow via round-trip: resize down, then back
        // up, and require bit-equality. `resize_to` is the canonical
        // magnitude/sign-preserving width cast on the `BigInt` surface,
        // so a round-trip mismatch means the value did not fit `Dst`.
        let narrowed: Dst = rescaled.resize_to::<Dst>();
        let round_trip: Src = narrowed.resize_to::<Src>();
        if round_trip == rescaled {
            Ok(narrowed)
        } else {
            Err(ConvertError::Overflow)
        }
    }
}
