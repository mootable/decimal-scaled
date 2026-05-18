//! D38 natural-logarithm kernel — `ln_fixed` on the 256-bit `Fixed`
//! intermediate with a configurable working-scale guard.
//!
//! Width-level specialisation for D38: the hand-tuned ln path that
//! has shipped since before the algorithm library existed. Captures
//! the four-variant matrix entry shape (`strict` vs `approx`, each
//! with an explicit-rounding-mode sibling) as a single kernel
//! parameterised by `working_digits`.
//!
//! Fast paths preserved verbatim from the typed surface:
//! - `self == 10^SCALE` (i.e. logical 1.0) returns `Self::ZERO`.
//! - `|self - 1| <= 10^(SCALE - ceil(SCALE/2))` returns `(self - 1)`
//!   directly — the linear `ln(1+x) ≈ x` band where the result is
//!   exact at storage precision.
//!
//! Panics on `raw <= 0` (the typed method's contract).

use crate::d_w128_kernels::Fixed;
use crate::log_exp_strict::{STRICT_GUARD, ln_fixed};
use crate::rounding::RoundingMode;

/// D38 natural log with explicit `working_digits` and rounding mode.
/// Called by both `ln_strict_with` (with `working_digits = STRICT_GUARD`)
/// and `ln_approx_with` (with the caller's value).
///
/// Returns the raw `i128` storage at the input's scale.
#[inline]
#[must_use]
pub(crate) fn ln_with(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    assert!(raw > 0, "ln kernel: argument must be positive");
    let one_bits: i128 = 10_i128.pow(scale);
    if raw == one_bits {
        return 0;
    }
    let delta = raw - one_bits;
    let ln1p_band: i128 = 10_i128.pow(scale.saturating_sub((scale + 1) / 2));
    if delta.abs() <= ln1p_band {
        return delta;
    }
    let w = scale + working_digits;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    ln_fixed(v_w, w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("ln kernel", scale))
}

/// Strict variant — fixed to `STRICT_GUARD` working digits. Equivalent
/// to `ln_with(raw, scale, STRICT_GUARD, mode)` but keeps the working
/// scale `w = SCALE + STRICT_GUARD` const-folded so LLVM specialises
/// one optimal kernel per `SCALE`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    assert!(raw > 0, "ln kernel: argument must be positive");
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    let delta = raw - one_bits;
    let ln1p_band: i128 = 10_i128.pow(SCALE.saturating_sub((SCALE + 1) / 2));
    if delta.abs() <= ln1p_band {
        return delta;
    }
    let w = SCALE + STRICT_GUARD;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
    ln_fixed(v_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("ln kernel", SCALE))
}
