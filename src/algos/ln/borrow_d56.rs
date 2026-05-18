//! D38 natural-log via widen → D56 wide_kernel → narrow back.
//!
//! Per-scale survey v2 (`research/per_scale_perf_2026-05-18_v2.md`)
//! showed D56's wide-tier ln kernel is 2-4× faster than D38's bespoke
//! `Fixed` 256-bit path at matched precision. This wrapper preserves
//! D38's external contract while routing the math through the faster
//! kernel.
//!
//! Correctness: `ln(D38<S>::MAX) < D38<S>::MAX` for any `S`, so the
//! narrowing TryFrom can only fail on caller-induced overflow — never
//! on a representable input.

use crate::core_type::{D38, D56};
use crate::rounding::RoundingMode;

/// D38 natural log via widen → D56 wide_kernel → narrow back.
/// Strict working scale (`SCALE + GUARD` const-folded inside D56).
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let raw_wide = super::wide_kernel::ln_strict_d56(widened.0, mode, SCALE);
    let narrowed: D38<SCALE> = D56::<SCALE>::from_bits(raw_wide)
        .try_into()
        .expect("ln_strict: result out of range");
    narrowed.0
}
