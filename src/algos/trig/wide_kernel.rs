//! Trigonometric wide-tier kernel — the single D57 `atan_strict_d57`
//! free function the D38 inverse trig family borrows.
//!
//! The forward (`sin` / `cos` / `tan`) and inverse (`atan`) wide-tier
//! kernels for every tier were superseded by the tier-generic
//! `*_series` kernels over `WideTrigCore` (the policies now dispatch to
//! those directly). The one survivor is `atan_strict_d57`: the D38
//! inverse family (`atan` / `asin` / `acos` / `atan2`) borrows D57 — the
//! D57 wide-tier atan is ~2× faster than D38's `fixed_d38`
//! adaptive-halvings path, and the inverse helpers compose atan — so
//! `crate::algos::trig::borrow_d57::atan_strict` widens D38 → D57 and
//! calls this kernel for the `SCALE` bands outside the 18..=22 lookup
//! window. See [`crate::algos::trig::borrow_d57`].

#[cfg(any(feature = "d57", feature = "wide"))]
use crate::int::types::Int;
#[cfg(any(feature = "d57", feature = "wide"))]
use crate::support::rounding::RoundingMode;

/// D57 `atan_strict` kernel — result in `(−π/2, π/2)`. Borrowed by the
/// D38 inverse trig family via [`crate::algos::trig::borrow_d57`].
#[cfg(any(feature = "d57", feature = "wide"))]
#[inline]
#[must_use]
pub(crate) fn atan_strict_d57(raw: Int<3>, mode: RoundingMode, scale: u32) -> Int<3> {
    use crate::types::widths::wide_trig_d57 as core;
    core::round_to_storage_directed(core::GUARD, scale, mode, |guard| {
        core::atan_fixed(core::to_work_w(raw, guard), scale + guard)
    })
}
