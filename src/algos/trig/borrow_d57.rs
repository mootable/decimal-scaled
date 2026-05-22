//! D38 trig (forward + inverse + atan2) via widen → D57 → narrow back.
//!
//! See [`crate::algos::ln::borrow_d57`] for the broader rationale.
//!
//! - `sin` / `cos` / `tan` / `atan` route through
//!   [`crate::algos::trig::wide_kernel`]'s D57 free functions.
//! - `asin` / `acos` / `atan2` route through D57's inherent
//!   `*_strict_with` methods (no separate algos kernel today; this
//!   mirrors how the wide-tier `TrigPolicy` impls invoke them).
//!
//! Correctness: all of these have outputs bounded within
//! `[-π, π]` (atan2) or smaller, so the narrowing `TryFrom` cannot fail
//! on a representable input. `tan(±π/2)` panics on the D57 side via
//! `wide_kernel::tan_strict_d57` (the same logical panic D38's
//! `fixed_d38::tan_strict` produces).

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
use crate::types::widths::{D38, D57};

#[inline]
fn narrow<const SCALE: u32>(raw_wide: Int<3>, op: &'static str) -> Int<2> {
    let wide = D57::<SCALE>::from_bits(raw_wide);
    let r: D38<SCALE> = wide.try_into().unwrap_or_else(|_| panic!(
        "{op}: result out of range — produced {wide}, D38<{SCALE}> represents only |x| < 1.7e{}",
        38_i32 - SCALE as i32,
    ));
    r.0
}

// ── forward (sin / cos / tan / atan) — via wide_kernel free fns ─────

#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_sincos::sin_strict::<SCALE>(widened.0, mode)
    } else {
        super::wide_kernel::sin_strict_d57(widened.0, mode, SCALE)
    };
    narrow::<SCALE>(raw_wide, "sin_strict")
}

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_sincos::cos_strict::<SCALE>(widened.0, mode)
    } else {
        super::wide_kernel::cos_strict_d57(widened.0, mode, SCALE)
    };
    narrow::<SCALE>(raw_wide, "cos_strict")
}

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_sincos::tan_strict::<SCALE>(widened.0, mode)
    } else {
        super::wide_kernel::tan_strict_d57(widened.0, mode, SCALE)
    };
    narrow::<SCALE>(raw_wide, "tan_strict")
}

#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_atan::atan_strict::<SCALE>(widened.0, mode)
    } else {
        super::wide_kernel::atan_strict_d57(widened.0, mode, SCALE)
    };
    narrow::<SCALE>(raw_wide, "atan_strict")
}

// ── inverse (asin / acos / atan2) — via D57 inherent methods ────────

#[inline]
#[must_use]
pub(crate) fn asin_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result_raw = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_inverse::asin_strict::<SCALE>(widened.0, mode)
    } else {
        widened.asin_strict_with(mode).0
    };
    narrow::<SCALE>(result_raw, "asin_strict")
}

#[inline]
#[must_use]
pub(crate) fn acos_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let widened: D57<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result_raw = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_inverse::acos_strict::<SCALE>(widened.0, mode)
    } else {
        widened.acos_strict_with(mode).0
    };
    narrow::<SCALE>(result_raw, "acos_strict")
}

#[inline]
#[must_use]
pub(crate) fn atan2_strict<const SCALE: u32>(y_raw: Int<2>, x_raw: Int<2>, mode: RoundingMode) -> Int<2> {
    let y_wide: D57<SCALE> = D38::<SCALE>::from_bits(y_raw).into();
    let x_wide: D57<SCALE> = D38::<SCALE>::from_bits(x_raw).into();
    let result_raw = if matches!(SCALE, 18..=22) {
        super::lookup_d57_s18_22_inverse::atan2_strict::<SCALE>(y_wide.0, x_wide.0, mode)
    } else {
        y_wide.atan2_strict_with(x_wide, mode).0
    };
    narrow::<SCALE>(result_raw, "atan2_strict")
}
