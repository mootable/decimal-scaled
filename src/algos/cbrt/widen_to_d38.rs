//! Narrow-tier cube-root kernel — widen to D38, cbrt, narrow back.
//!
//! Same pattern as [`crate::algos::sqrt::widen_to_d38`]: the narrow
//! tiers don't carry their own cube-root path; they widen losslessly
//! to D38, call D38's cbrt, and narrow back.

use crate::support::rounding::RoundingMode;
use crate::types::widths::{D18, D38};

/// `D9` cube-root via widen → D38 → narrow.
#[inline]
#[must_use]
/// `D18` cube-root via widen → D38 → narrow.
#[inline]
#[must_use]
pub(crate) fn cbrt_d18<const SCALE: u32>(v: D18<SCALE>, mode: RoundingMode) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let cbrt_d38 = super::mg_divide_d38::cbrt(widened.0, SCALE, mode);
    D38::<SCALE>::from_bits(cbrt_d38)
        .try_into()
        .unwrap_or_else(|_| {
            crate::support::diagnostics::overflow_panic_with_scale("widen_to_d38::cbrt_d18", SCALE)
        })
}
