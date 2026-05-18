//! Narrow-tier square-root kernel — widen to D38, sqrt, narrow back.
//!
//! Captures the **width-level specialisation** that has lived on D9 /
//! D18 since the strict transcendentals shipped: the narrow tiers
//! don't have their own integer-sqrt path — they widen losslessly to
//! D38 (same SCALE), call D38's `sqrt_strict`, and narrow back. The
//! narrowing can't lose precision for sqrt because the result is
//! bounded by `sqrt(self) <= self` for `self >= 1` (within bounds
//! of the narrower storage).
//!
//! Two free functions are exposed — one per narrow width — so the
//! policy file's per-width impls are straight-line.

use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

/// `D9` square-root via widen → D38 → narrow.
#[inline]
#[must_use]
pub(crate) fn sqrt_d9<const SCALE: u32>(v: D9<SCALE>, mode: RoundingMode) -> D9<SCALE> {
    let widened: D38<SCALE> = v.into();
    let sqrt_d38 = super::mg_divide_d38::sqrt(widened.0, SCALE, mode);
    let narrowed: D9<SCALE> = D38::<SCALE>(sqrt_d38)
        .try_into()
        .expect("widen_to_d38::sqrt_d9: result out of range");
    narrowed
}

/// `D18` square-root via widen → D38 → narrow.
#[inline]
#[must_use]
pub(crate) fn sqrt_d18<const SCALE: u32>(v: D18<SCALE>, mode: RoundingMode) -> D18<SCALE> {
    let widened: D38<SCALE> = v.into();
    let sqrt_d38 = super::mg_divide_d38::sqrt(widened.0, SCALE, mode);
    let narrowed: D18<SCALE> = D38::<SCALE>(sqrt_d38)
        .try_into()
        .expect("widen_to_d38::sqrt_d18: result out of range");
    narrowed
}
