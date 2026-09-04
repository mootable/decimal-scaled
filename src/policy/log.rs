// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Log-base policy — the per-(N, SCALE) algorithm matcher for the
//! arbitrary-base decimal logarithm `log(self, base)`.
//!
//! `D<Int<N>, SCALE>::log_with(base, mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`.
//!
//! # One algorithm — `LnDivide`
//!
//! `log(self, base) = ln(self) / ln(base)`. Every tier and scale uses the
//! same ratio. The narrow tiers route through the
//! `crate::algos::log::log_ln_divide` composition kernels (D18 widens to
//! Int<2>; D38 calls `ln::ln_series_2limb`); the wide tiers route through
//! the per-tier `wide_trig_<tier>::log_strict_with_kernel`
//! free functions (emitted by
//! `decl_wide_transcendental!`), reached by a `match N` with `resize_to`
//! bridges (identity at the matched `N`). `Schoolbook` is the unrouted
//! naive `ln(x)/ln(b)` reference.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    LnDivide,
    /// `ln(self)/ln(base)` with both logs taken by the narrow TANG core.
    /// Narrow tiers only — the wide tiers reach Tang inside their own
    /// `ln_fixed_routed` surface, so `LnDivide` already gives them it.
    LnDivideTang,
    #[allow(dead_code)]
    Schoolbook,
}

#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    let _ = SCALE;
    match N {
        // Narrow tiers take the ratio's two logs from the Tang core.
        // `log(x, b)` calls the working-scale ln TWICE, and on a base that
        // is not a power of two the whole cost is one full artanh series
        // (bbc 33874088471: D38 `log` 8,660 ns at s28 decomposed as
        // `ln_nd` 7,037 + `log10` 1,667 to within 1.005x — the series IS
        // the cost). Tang takes that series' argument from `|t| <= 1/3` to
        // `|t| <= 1/257`.
        //
        // Blanket over the whole scale range: the term-count argument is
        // scale-independent, and `LnDivide` remains the kept alternative
        // in the `_` arm.
        1 | 2 => Select::ByAlgorithm(Algorithm::LnDivideTang),
        _ => Select::ByAlgorithm(Algorithm::LnDivide),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    checked_dispatch::<N, SCALE>(raw, braw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("log", SCALE)
    })
}

/// The `checked` primitive under [`dispatch`]: exact out-of-range
/// `None` on the narrow tiers; the wide arms call the per-tier kernel
/// shells, whose internal out-of-range panic is not yet threaded
/// through. Domain errors
/// stay kernel panics — the `checked_` surface prechecks the domain.
#[inline]
#[must_use]
pub(crate) fn checked_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(&raw),
    };
    match algo {
        Algorithm::LnDivide => ln_divide_routed::<N, SCALE>(raw, braw, mode),
        Algorithm::LnDivideTang => ln_divide_tang_routed::<N, SCALE>(raw, braw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, braw, mode),
    }
}

/// The narrow Tang arm. Only `N = 1 | 2` select it; the `_` arm keeps the
/// widths whole by falling back to [`ln_divide_routed`], so the match stays
/// total over `N` even though `select` never sends a wide width here.
#[inline]
fn ln_divide_tang_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::log::log_ln_divide::log_ln_divide_tang_d38::<N, SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
        _ => ln_divide_routed::<N, SCALE>(raw, braw, mode),
    }
}

#[inline]
fn ln_divide_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        // N == 1 runs the same D38-width log kernel the widen-narrow
        // `log_ln_divide_d18` composition resolves to; the `narrow_fit`
        // reproduces its try-into fit check as `None`.
        1 | 2 => crate::algos::log::log_ln_divide::log_ln_divide_d38::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::types::widths::wide_trig_d57::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::types::widths::wide_trig_d76::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::types::widths::wide_trig_d115::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::types::widths::wide_trig_d153::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::types::widths::wide_trig_d230::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::types::widths::wide_trig_d307::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::types::widths::wide_trig_d462::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::types::widths::wide_trig_d616::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::types::widths::wide_trig_d924::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::types::widths::wide_trig_d1232::log_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => crate::algos::log::log_ln_divide::log_ln_divide_d38::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    braw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => super::narrow_fit::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode)),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), braw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), braw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), braw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), braw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), braw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), braw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), braw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), braw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), braw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::algos::log::log_schoolbook::log_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), braw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => super::narrow_fit::<N>(crate::algos::log::log_schoolbook::log_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), braw.resize_to::<Int<2>>(), mode)),
    }
}

