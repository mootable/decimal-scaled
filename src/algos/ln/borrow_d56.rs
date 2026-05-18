//! D38 natural-log / base-2 log / base-10 log / arbitrary-base log
//! via widen → D56 wide_kernel → narrow back.
//!
//! Per-scale survey v2 (`research/per_scale_perf_2026-05-18_v2.md`)
//! showed D56's wide-tier ln kernel is 2-4× faster than D38's bespoke
//! `Fixed` 256-bit path at matched precision. These wrappers preserve
//! D38's external contract while routing the math through the faster
//! kernel.
//!
//! Correctness: `ln(D38<S>::MAX) < D38<S>::MAX` and the same bound
//! holds for log2 and log10 (both smaller than ln by a constant
//! factor), so the narrowing TryFrom can only fail on caller-induced
//! overflow — never on a representable input. For `log(self, base)`
//! the bound depends on `base`: bases close to 1 amplify the result;
//! the same overflow panic the bespoke kernel produced is preserved
//! by the narrowing `TryFrom` returning `Err` and the wrapping
//! `expect` re-raising it.

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

/// D38 base-2 log via widen → D56 inherent `log2_strict_with` →
/// narrow back. D56's `log2_strict_with` uses the same
/// `wide_trig_d56::ln_fixed` core the borrow `ln_strict` does, so
/// this picks up the same 2-4× speedup the v2 survey measured.
#[inline]
#[must_use]
pub(crate) fn log2_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result = widened.log2_strict_with(mode);
    let narrowed: D38<SCALE> = result.try_into().expect("log2_strict: result out of range");
    narrowed.0
}

/// D38 base-10 log via widen → D56 inherent `log10_strict_with` →
/// narrow back. See [`log2_strict`] for the rationale.
#[inline]
#[must_use]
pub(crate) fn log10_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let result = widened.log10_strict_with(mode);
    let narrowed: D38<SCALE> = result.try_into().expect("log10_strict: result out of range");
    narrowed.0
}

/// D38 arbitrary-base log via widen → D56 inherent `log_strict_with`
/// → narrow back. See [`log2_strict`] for the rationale. The
/// "base must not equal 1" panic is preserved on the D56 side
/// (`base.ln == 0` is detected before division).
#[inline]
#[must_use]
pub(crate) fn log_strict<const SCALE: u32>(
    raw: i128,
    base_raw: i128,
    mode: RoundingMode,
) -> i128 {
    let widened: D56<SCALE> = D38::<SCALE>::from_bits(raw).into();
    let base_wide: D56<SCALE> = D38::<SCALE>::from_bits(base_raw).into();
    let result = widened.log_strict_with(base_wide, mode);
    let narrowed: D38<SCALE> = result.try_into().expect("log_strict: result out of range");
    narrowed.0
}
