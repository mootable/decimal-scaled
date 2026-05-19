//! Narrow-tier natural-log kernel — widen to D38, ln, narrow back.
//!
//! Width-level specialisation for D9 / D18. Same pattern as
//! [`crate::algos::sqrt::widen_to_d38`]: lossless widen, call D38's
//! ln kernel, narrow back.

use crate::types::widths::{D9, D18, D38};
use crate::support::rounding::RoundingMode;

/// `D9` natural log via widen → D38 → narrow. Strict working-scale.
#[inline]
#[must_use]
pub(crate) fn ln_strict_d9<const SCALE: u32>(v: D9<SCALE>, mode: RoundingMode) -> D9<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::ln_strict::<SCALE>(widened.0, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale("ln_strict", SCALE))
}

/// `D9` natural log with caller-chosen working digits.
#[inline]
#[must_use]
pub(crate) fn ln_with_d9<const SCALE: u32>(
    v: D9<SCALE>,
    working_digits: u32,
    mode: RoundingMode,
) -> D9<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::ln_with(widened.0, SCALE, working_digits, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale("ln_with", SCALE))
}

/// `D18` natural log via widen → D38 → narrow. Strict working-scale.
#[inline]
#[must_use]
pub(crate) fn ln_strict_d18<const SCALE: u32>(v: D18<SCALE>, mode: RoundingMode) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::ln_strict::<SCALE>(widened.0, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale("ln_strict", SCALE))
}

/// `D18` natural log with caller-chosen working digits.
#[inline]
#[must_use]
pub(crate) fn ln_with_d18<const SCALE: u32>(
    v: D18<SCALE>,
    working_digits: u32,
    mode: RoundingMode,
) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let raw = super::fixed_d38::ln_with(widened.0, SCALE, working_digits, mode);
    D38::<SCALE>::from_bits(raw)
        .try_into()
        .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale("ln_with", SCALE))
}
