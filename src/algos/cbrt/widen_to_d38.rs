//! Narrow-tier cube-root kernel — widen to D38, cbrt, narrow back.
//!
//! Same pattern as [`crate::algos::sqrt::widen_to_d38`]: the narrow
//! tiers don't carry their own cube-root path; they widen losslessly
//! to D38, call D38's cbrt, and narrow back.

use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

/// `D9` cube-root via widen → D38 → narrow.
#[inline]
#[must_use]
pub(crate) fn cbrt_d9<const SCALE: u32>(v: D9<SCALE>, mode: RoundingMode) -> D9<SCALE> {
    let widened: D38<SCALE> = v.into();
    let cbrt_d38 = super::mg_divide_d38::cbrt(widened.0, SCALE, mode);
    D38::<SCALE>::from_bits(cbrt_d38)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("widen_to_d38::cbrt_d9", SCALE))
}

/// `D18` cube-root via widen → D38 → narrow.
#[inline]
#[must_use]
pub(crate) fn cbrt_d18<const SCALE: u32>(v: D18<SCALE>, mode: RoundingMode) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let cbrt_d38 = super::mg_divide_d38::cbrt(widened.0, SCALE, mode);
    D38::<SCALE>::from_bits(cbrt_d38)
        .try_into()
        .unwrap_or_else(|_| crate::diagnostics::overflow_panic_with_scale("widen_to_d38::cbrt_d18", SCALE))
}
