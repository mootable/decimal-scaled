//! D38 trig (forward + inverse + atan2) via widen → D56 → narrow back.
//!
//! See [`crate::algos::ln::borrow_d56`] for the broader rationale.
//!
//! - `sin` / `cos` / `tan` / `atan` route through
//!   [`crate::algos::trig::wide_kernel`]'s D56 free functions.
//! - `asin` / `acos` / `atan2` route through D56's inherent
//!   `*_strict_with` methods (no separate algos kernel today; this
//!   mirrors how the wide-tier `TrigPolicy` impls invoke them).
//!
//! Correctness: all of these have outputs bounded within
//! `[-π, π]` (atan2) or smaller, so the narrowing `TryFrom` cannot fail
//! on a representable input. `tan(±π/2)` panics on the D56 side via
//! `wide_kernel::tan_strict_d56` (the same logical panic D38's
//! `fixed_d38::tan_strict` produces).

use crate::core_type::{D38, D56};
use crate::rounding::RoundingMode;
use crate::wide_int::I192;

#[inline]
fn narrow<const SCALE: u32>(raw_wide: I192, msg: &'static str) -> i128 {
    let r: D38<SCALE> = D56::<SCALE>::from_bits(raw_wide)
        .try_into()
        .expect(msg);
    r.0
}

// ── forward (sin / cos / tan / atan) — via wide_kernel free fns ─────

#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = super::wide_kernel::sin_strict_d56(widened.0, mode, SCALE);
    narrow::<SCALE>(raw_wide, "sin: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = super::wide_kernel::cos_strict_d56(widened.0, mode, SCALE);
    narrow::<SCALE>(raw_wide, "cos: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = super::wide_kernel::tan_strict_d56(widened.0, mode, SCALE);
    narrow::<SCALE>(raw_wide, "tan: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = super::wide_kernel::atan_strict_d56(widened.0, mode, SCALE);
    narrow::<SCALE>(raw_wide, "atan: result out of range")
}

// ── inverse (asin / acos / atan2) — via D56 inherent methods ────────

#[inline]
#[must_use]
pub(crate) fn asin_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result = widened.asin_strict_with(mode);
    narrow::<SCALE>(result.0, "asin: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn acos_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result = widened.acos_strict_with(mode);
    narrow::<SCALE>(result.0, "acos: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn atan2_strict<const SCALE: u32>(y_raw: i128, x_raw: i128, mode: RoundingMode) -> i128 {
    let y_wide: D56<SCALE> = D38::<SCALE>::from_bits(y_raw).into();
    let x_wide: D56<SCALE> = D38::<SCALE>::from_bits(x_raw).into();
    let result = y_wide.atan2_strict_with(x_wide, mode);
    narrow::<SCALE>(result.0, "atan2: result out of range")
}
