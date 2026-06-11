// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log_ln_divide` — arbitrary-base logarithm by the ratio of natural logs.
//!
//! `log(self, base) = ln(self) / ln(base)`. This module holds the narrow-tier
//! (D18, D38) composition kernels the [`crate::policy::log`] matcher delegates
//! *down* to:
//!
//! - [`log_ln_divide_d18`] / [`log_ln_divide_d18_approx`] — D18 (`Int<1>`)
//!   has no native log kernel, so it widens to the D38 (`Int<2>`) work width,
//!   delegates to D38's `log_strict_with` / `log_approx_with` surface (a
//!   cross-tier *down* call), then narrows the result back.
//! - [`log_ln_divide_d38`] / [`log_ln_divide_d38_approx`] — D38 (`Int<2>`)
//!   calls the `ln::ln_series_2limb` log kernel directly on raw storage.
//!
//! The wide tiers realise `LnDivide` through the per-tier
//! `log_strict_with_kernel` / `log_approx_with_kernel` free functions emitted
//! by `decl_wide_transcendental!` (in `crate::types::widths`); the policy
//! calls those down directly.

use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// D18 strict `log(self, base)`: widen to the D38 work width, run D38's
/// strict log, narrow back. Panics (with a scale-tagged overflow message)
/// if the result does not fit D18.
#[inline]
pub(crate) fn log_ln_divide_d18<const SCALE: u32>(
    raw: Int<1>,
    base_raw: Int<1>,
    mode: RoundingMode,
) -> Int<1> {
    let wide: crate::D<Int<2>, SCALE> = crate::D::<Int<1>, SCALE>(raw).into();
    let wbase: crate::D<Int<2>, SCALE> = crate::D::<Int<1>, SCALE>(base_raw).into();
    let result: crate::D<Int<1>, SCALE> =
        ::core::convert::TryInto::try_into(wide.log_strict_with(wbase, mode)).unwrap_or_else(|_| {
            crate::support::diagnostics::overflow_panic_with_scale("D18::log", SCALE)
        });
    result.0
}

/// D18 approx `log(self, base)` with caller-chosen guard digits: widen to
/// the D38 work width, run D38's approx log, narrow back.
#[inline]
pub(crate) fn log_ln_divide_d18_approx<const SCALE: u32>(
    raw: Int<1>,
    base_raw: Int<1>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<1> {
    let wide: crate::D<Int<2>, SCALE> = crate::D::<Int<1>, SCALE>(raw).into();
    let wbase: crate::D<Int<2>, SCALE> = crate::D::<Int<1>, SCALE>(base_raw).into();
    let result: crate::D<Int<1>, SCALE> =
        ::core::convert::TryInto::try_into(wide.log_approx_with(wbase, working_digits, mode))
            .unwrap_or_else(|_| {
                crate::support::diagnostics::overflow_panic_with_scale("D18::log", SCALE)
            });
    result.0
}

/// D38 strict `log(self, base)` via the `ln::ln_series_2limb` 256-bit log
/// kernel, on raw storage. `None` = result out of storage range.
#[inline]
pub(crate) fn log_ln_divide_d38<const SCALE: u32>(
    raw: Int<2>,
    base_raw: Int<2>,
    mode: RoundingMode,
) -> Option<Int<2>> {
    crate::algos::ln::ln_series_2limb::log_strict::<SCALE>(raw, base_raw, mode)
}

/// D38 approx `log(self, base)` with caller-chosen guard digits via the
/// `ln::ln_series_2limb` log kernel, on raw storage.
#[inline]
pub(crate) fn log_ln_divide_d38_approx<const SCALE: u32>(
    raw: Int<2>,
    base_raw: Int<2>,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<Int<2>> {
    crate::algos::ln::ln_series_2limb::log_with(raw, base_raw, SCALE, working_digits, mode)
}
