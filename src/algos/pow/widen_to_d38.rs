//! Narrow-tier floating-point power kernel — widen both base and
//! exponent to D38, `powf`, narrow back.
//!
//! Width-level specialisation for D9 / D18. Same pattern as
//! [`crate::algos::ln::widen_to_d38`], but the wrapper takes two values
//! (base and exponent) which are widened independently before the
//! kernel call.
//!
//! Integer-exponent fast path: if `exp` is an exact integer with
//! `|n| <= INT_FAST_PATH_THRESHOLD`, route directly to D9/D18's
//! macro-emitted `powi(n)` (square-and-multiply on the native
//! storage) and skip the widen-to-D38 + exp(y·ln(x)) chain.

use super::fixed_d38::INT_FAST_PATH_THRESHOLD;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

/// `Some(n)` if `exp_raw` (at `SCALE`) represents an exact integer
/// `n` with `|n| <= INT_FAST_PATH_THRESHOLD`. Identical contract to
/// [`super::fixed_d38::exp_as_small_int`], duplicated for the narrow
/// storage types where the divisor is `i64` / `i128`.
#[inline]
fn exp_as_small_int_i128<const SCALE: u32>(exp_raw: i128) -> Option<i32> {
    let mult = 10_i128.pow(SCALE);
    if exp_raw % mult != 0 {
        return None;
    }
    let q = exp_raw / mult;
    if !(i32::MIN as i128..=i32::MAX as i128).contains(&q) {
        return None;
    }
    let n = q as i32;
    if n.unsigned_abs() <= INT_FAST_PATH_THRESHOLD as u32 {
        Some(n)
    } else {
        None
    }
}

/// `D9` powf via widen → D38 → narrow. Strict working-scale.
#[inline]
#[must_use]
/// `D9` powf with caller-chosen working digits.
#[inline]
#[must_use]
/// `D18` powf via widen → D38 → narrow. Strict working-scale.
#[inline]
#[must_use]
pub(crate) fn powf_strict_d18<const SCALE: u32>(
    base: D18<SCALE>,
    exp: D18<SCALE>,
    mode: RoundingMode,
) -> D18<SCALE> {
    if base.to_bits() > 0 {
        if let Some(n) = exp_as_small_int_i128::<SCALE>(exp.to_bits() as i128) {
            return base.powi(n);
        }
    }
    let base_w: D38<SCALE> = base.into();
    let exp_w: D38<SCALE> = exp.into();
    let raw = super::fixed_d38::powf_strict::<SCALE>(base_w.0, exp_w.0, mode);
    D38::<SCALE>::from_bits(raw).try_into().unwrap_or_else(|_| {
        crate::support::diagnostics::overflow_panic_with_scale("powf_strict", SCALE)
    })
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
    if base.to_bits() > 0 {
        if let Some(n) = exp_as_small_int_i128::<SCALE>(exp.to_bits() as i128) {
            return base.powi(n);
        }
    }
    let base_w: D38<SCALE> = base.into();
    let exp_w: D38<SCALE> = exp.into();
    let raw = super::fixed_d38::powf_with::<SCALE>(base_w.0, exp_w.0, working_digits, mode);
    D38::<SCALE>::from_bits(raw).try_into().unwrap_or_else(|_| {
        crate::support::diagnostics::overflow_panic_with_scale("powf_with", SCALE)
    })
}
