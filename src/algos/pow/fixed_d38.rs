//! D38 floating-point power kernel — `powf` computed as
//! `exp(y · ln(x))` on the 256-bit `Fixed` intermediate.
//!
//! Width-level specialisation for D38, capturing the hand-tuned path
//! that has shipped since before the algorithm library existed. The
//! `ln`, the multiplication, and the `exp` all share the wide guard-
//! digit working scale so no precision is dropped between stages.
//!
//! Fast path preserved verbatim from the typed surface:
//! - A non-positive base saturates to `0` (matches the f64-bridge
//!   NaN-to-ZERO policy for negative bases with arbitrary fractional
//!   exponents).
//!
//! Returns the raw `i128` storage at the input's scale.

use crate::d_w128_kernels::Fixed;
use crate::log_exp_strict::{STRICT_GUARD, exp_fixed, ln_fixed};
use crate::rounding::RoundingMode;

/// `base^exp` with caller-chosen `working_digits` above the storage scale.
///
/// Both `base` and `exp` are raw storage at `scale`. Non-positive `base`
/// saturates to `0`.
#[inline]
#[must_use]
pub(crate) fn powf_with<const SCALE: u32>(
    base: i128,
    exp: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if base <= 0 {
        return 0;
    }
    let w = SCALE + working_digits;
    let pow = 10u128.pow(working_digits);
    let ln_x = ln_fixed(
        Fixed::from_u128_mag(base as u128, false).mul_u128(pow),
        w,
    );
    let y_neg = exp < 0;
    let y_w = Fixed::from_u128_mag(exp.unsigned_abs(), false).mul_u128(pow);
    let y_w = if y_neg { y_w.neg() } else { y_w };
    exp_fixed(y_w.mul(ln_x, w), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("powf kernel", SCALE))
}

/// Strict variant — const-folded `working_digits = STRICT_GUARD`.
#[inline]
#[must_use]
pub(crate) fn powf_strict<const SCALE: u32>(
    base: i128,
    exp: i128,
    mode: RoundingMode,
) -> i128 {
    if base <= 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let pow = 10u128.pow(STRICT_GUARD);
    let ln_x = ln_fixed(
        Fixed::from_u128_mag(base as u128, false).mul_u128(pow),
        w,
    );
    let y_neg = exp < 0;
    let y_w = Fixed::from_u128_mag(exp.unsigned_abs(), false).mul_u128(pow);
    let y_w = if y_neg { y_w.neg() } else { y_w };
    exp_fixed(y_w.mul(ln_x, w), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("powf kernel", SCALE))
}
