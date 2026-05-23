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

/// Rescales a [`BigInt`] magnitude from scale `s_from` to scale `s_to`
/// at its own width, applying `mode` to any scale-down rounding.
///
/// - `s_to == s_from`: returns `value` unchanged.
/// - `s_to > s_from` (scale-up): multiplies by `10^(s_to - s_from)`;
///   returns `None` if that overflows `T`'s range (the caller maps this
///   to [`ConvertError::Overflow`]).
/// - `s_to < s_from` (scale-down): divides by `10^(s_from - s_to)` and
///   rounds per `mode`; always `Some` (the magnitude only shrinks).
#[inline]
fn rescale_bigint<T: BigInt>(value: T, s_from: u32, s_to: u32, mode: RoundingMode) -> Option<T> {
    if s_to == s_from {
        return Some(value);
    }
    if s_to > s_from {
        let shift = s_to - s_from;
        let multiplier = T::TEN.checked_pow(shift)?;
        return value.checked_mul(multiplier);
    }
    // Scale-down: divide by 10^shift with rounding.
    let shift = s_from - s_to;
    let divisor = match T::TEN.checked_pow(shift) {
        Some(d) => d,
        // 10^shift exceeds T's range: every in-range magnitude is
        // strictly smaller than the divisor, so the truncated quotient
        // is 0 and the remainder is the whole value. Round 0 per mode.
        None => return Some(round_when_quotient_zero(value, mode)),
    };
    let (quotient, remainder) = value.div_rem(divisor);
    if remainder == T::ZERO {
        return Some(quotient);
    }
    let abs_rem = magnitude(remainder);
    let abs_div = magnitude(divisor);
    // `cmp_r`: |r| vs |divisor| - |r|, i.e. the round-up boundary
    // `2|r| vs |divisor|` without the doubling-overflow risk.
    let cmp_r = abs_rem.cmp(&(abs_div - abs_rem));
    let q_is_odd = quotient.bit(0);
    let result_positive = (value < T::ZERO) == (divisor < T::ZERO);
    if should_bump(mode, cmp_r, q_is_odd, result_positive) {
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
/// strictly below the half boundary). Only the directed-away modes
/// (`Ceiling` for positive, `Floor` for negative) can bump to `±1`.
#[inline]
fn round_when_quotient_zero<T: BigInt>(value: T, mode: RoundingMode) -> T {
    if value == T::ZERO {
        return T::ZERO;
    }
    let result_positive = value > T::ZERO;
    // `cmp_r == Less`: |r| is strictly below the half boundary because
    // the divisor strictly exceeds |value|.
    let cmp_r = core::cmp::Ordering::Less;
    // Truncated quotient is 0 (even).
    if should_bump(mode, cmp_r, false, result_positive) {
        if result_positive {
            T::ONE
        } else {
            T::ZERO - T::ONE
        }
    } else {
        T::ZERO
    }
}

/// Converts a source magnitude `src` (scale `s_from`, width `Src`) to
/// the target magnitude type `Dst` at scale `s_to`, rounding any
/// scale-down per `mode`.
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
    src: Src,
    s_from: u32,
    s_to: u32,
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
        let widened: Dst = src.resize_to::<Dst>();
        rescale_bigint(widened, s_from, s_to, mode).ok_or(ConvertError::Overflow)
    } else {
        // Narrow: rescale at the source (wider) width first — scale-down
        // only shrinks, and a scale-up that overflows the source is a
        // genuine error — then narrow the magnitude into `Dst`.
        let rescaled: Src = rescale_bigint(src, s_from, s_to, mode).ok_or(ConvertError::Overflow)?;
        // Fallible signed narrow via round-trip: resize down, then back
        // up, and require bit-equality. `resize_to` is the canonical
        // magnitude/sign-preserving width cast on the `BigInt` surface,
        // so a round-trip mismatch means the value did not fit `Dst`.
        let narrowed: Dst = rescaled.resize_to::<Dst>();
        let back: Src = narrowed.resize_to::<Src>();
        if back == rescaled {
            Ok(narrowed)
        } else {
            Err(ConvertError::Overflow)
        }
    }
}
