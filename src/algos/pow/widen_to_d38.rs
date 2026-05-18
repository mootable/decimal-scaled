//! Narrow-tier floating-point power kernel — widen both base and
//! exponent to D38, `powf`, narrow back.
//!
//! Width-level specialisation for D9 / D18. Same pattern as
//! [`crate::algos::ln::widen_to_d38`], but the wrapper takes two values
//! (base and exponent) which are widened independently before the
//! kernel call.

use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

/// `D9` powf via widen → D38 → narrow. Strict working-scale.
#[inline]
#[must_use]
pub(crate) fn powf_strict_d9<const SCALE: u32>(
    base: D9<SCALE>,
    exp: D9<SCALE>,
    mode: RoundingMode,
) -> D9<SCALE> {
    let base_w: D38<SCALE> = base.into();
    let exp_w: D38<SCALE> = exp.into();
    let raw = super::fixed_d38::powf_strict::<SCALE>(base_w.0, exp_w.0, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("powf_strict", SCALE))
}

/// `D9` powf with caller-chosen working digits.
#[inline]
#[must_use]
pub(crate) fn powf_with_d9<const SCALE: u32>(
    base: D9<SCALE>,
    exp: D9<SCALE>,
    working_digits: u32,
    mode: RoundingMode,
) -> D9<SCALE> {
    let base_w: D38<SCALE> = base.into();
    let exp_w: D38<SCALE> = exp.into();
    let raw = super::fixed_d38::powf_with::<SCALE>(base_w.0, exp_w.0, working_digits, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("powf_with", SCALE))
}

/// `D18` powf via widen → D38 → narrow. Strict working-scale.
#[inline]
#[must_use]
pub(crate) fn powf_strict_d18<const SCALE: u32>(
    base: D18<SCALE>,
    exp: D18<SCALE>,
    mode: RoundingMode,
) -> D18<SCALE> {
    let base_w: D38<SCALE> = base.into();
    let exp_w: D38<SCALE> = exp.into();
    let raw = super::fixed_d38::powf_strict::<SCALE>(base_w.0, exp_w.0, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("powf_strict", SCALE))
}

/// `D18` powf with caller-chosen working digits.
#[inline]
#[must_use]
pub(crate) fn powf_with_d18<const SCALE: u32>(
    base: D18<SCALE>,
    exp: D18<SCALE>,
    working_digits: u32,
    mode: RoundingMode,
) -> D18<SCALE> {
    let base_w: D38<SCALE> = base.into();
    let exp_w: D38<SCALE> = exp.into();
    let raw = super::fixed_d38::powf_with::<SCALE>(base_w.0, exp_w.0, working_digits, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("powf_with", SCALE))
}
