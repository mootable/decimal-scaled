//! D38 exponential kernel — `exp_fixed` on the 256-bit `Fixed`
//! intermediate, parameterised by working-digit guard.
//!
//! Width-level specialisation for D38, capturing the hand-tuned path
//! that has shipped since before the algorithm library existed.
//! Returns the raw `i128` storage at the input's scale; the typed
//! method shell handles the panic-on-overflow message.

use crate::d_w128_kernels::Fixed;
use crate::log_exp_strict::{STRICT_GUARD, exp_fixed};
use crate::rounding::RoundingMode;

/// `e^x` with caller-chosen `working_digits` above the storage scale.
#[inline]
#[must_use]
pub(crate) fn exp_with(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale); // ONE for this scale
    }
    let w = scale + working_digits;
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false)
        .mul_u128(10u128.pow(working_digits));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    exp_fixed(v_w, w)
        .round_to_i128_with(w, scale, mode)
        .expect("exp kernel: result overflows the representable range")
}

/// Strict variant — const-folded `working_digits = STRICT_GUARD`.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false)
        .mul_u128(10u128.pow(STRICT_GUARD));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    exp_fixed(v_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("exp kernel: result overflows the representable range")
}
