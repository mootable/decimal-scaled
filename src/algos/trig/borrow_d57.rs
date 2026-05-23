//! D38 inverse trig (atan / asin / acos / atan2) via widen → D57 →
//! narrow back.
//!
//! See [`crate::algos::ln::borrow_d57`] for the broader rationale.
//!
//! - `atan` routes through [`crate::algos::trig::wide_kernel`]'s D57 free
//!   function.
//! - `asin` / `acos` / `atan2` route through D57's inherent
//!   `*_strict_with` methods (no separate algos kernel today; this
//!   mirrors how the wide-tier `TrigPolicy` impls invoke them).
//!
//! The D57 wide_kernel atan algorithm is qualitatively faster than D38's
//! `fixed_d38` adaptive-halvings path (~2× at SCALE 19), and asin / acos /
//! atan2 compose atan internally, so they inherit that gap — which is why
//! the D38 inverse family borrows D57 while the forward family stays on
//! the bespoke `fixed_d38` kernel.
//!
//! Correctness: all of these have outputs bounded within `[-π, π]`
//! (atan2) or smaller, so the narrowing `TryFrom` cannot fail on a
//! representable input.

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

// ── inverse (atan / asin / acos / atan2) ────────────────────────────

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
