//! Narrow-tier exponential kernel — widen to D38, exp, narrow back.
//!
//! Width-level specialisation for D9 / D18. Same pattern as
//! [`crate::algos::ln::widen_to_d38`].

use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

#[inline]
#[must_use]
pub(crate) fn exp_strict_d9<const SCALE: u32>(v: D9<SCALE>, mode: RoundingMode) -> D9<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::exp_strict::<SCALE>(widened.0, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("exp_strict", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn exp_with_d9<const SCALE: u32>(
    v: D9<SCALE>,
    working_digits: u32,
    mode: RoundingMode,
) -> D9<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::exp_with(widened.0, SCALE, working_digits, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("exp_with", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn exp_strict_d18<const SCALE: u32>(v: D18<SCALE>, mode: RoundingMode) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::exp_strict::<SCALE>(widened.0, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("exp_strict", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn exp_with_d18<const SCALE: u32>(
    v: D18<SCALE>,
    working_digits: u32,
    mode: RoundingMode,
) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::exp_with(widened.0, SCALE, working_digits, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("exp_with", SCALE))
}
