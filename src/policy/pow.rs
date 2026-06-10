// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Floating-point power policy — the per-(N, SCALE) algorithm matcher.
//!
//! `D<Int<N>, SCALE>::powf_strict_with(exp, mode)` delegates directly to
//! the one shared [`dispatch`] generic function — the canonical
//! matcher-only policy shape (see `docs/ARCHITECTURE.md`), mirrored from
//! `sqrt`.
//!
//! # The one power algorithm — `ExpWithLn`
//!
//! `powf` is the hybrid `b^y = exp(y * ln b)`: a composition of the `exp`
//! and `ln` algorithms. `ExpWithLn` names that composition — not a single
//! kernel fn; realised per tier: narrow tiers on the 256-bit `Fixed`
//! intermediate via `pow::powf_series_2limb` (D18 widened to Int<2>), the
//! wide tiers via the C-generic `pow::pow_schoolbook` over their
//! `WideTrigCore` core. The integer-exponent square-and-multiply fast
//! path lives inside the kernels (`powf_series_2limb::powi_raw`). The
//! production wide-tier surface still composes the inherent
//! `powf_strict_with` shell directly; this dispatch is the canonical
//! matcher seam and is total over the key. `Schoolbook` is the unrouted
//! naive `exp(y*ln x)` reference.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    ExpWithLn,
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
    let _ = (N, SCALE);
    Select::ByAlgorithm(Algorithm::ExpWithLn)
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&base),
    };
    match algo {
        Algorithm::ExpWithLn => exp_with_ln_routed::<N, SCALE>(base, exponent, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(base, exponent, mode),
    }
}

/// The `checked` primitive for `checked_powf_strict[_with]`.
///
/// Narrow tiers (`N == 1 | 2`) run the seamed `powf_series_2limb`
/// kernel: its out-of-range `None` (and the `Int<2> -> Int<1>` narrow
/// fit) propagate exactly. The wide arms hop to the tier's inherent
/// `powf_strict_with` shell — the SAME path the default wide `powf`
/// surface takes (the wide production route does NOT go through
/// [`dispatch`]'s `pow_schoolbook` matcher seam, and bit-identity with
/// the default form is the contract) — so a wide out-of-range result
/// still panics inside that shell (see
/// `research/checked_wide_shell_patch.md`).
#[inline]
#[must_use]
pub(crate) fn checked_dispatch<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::pow::powf_series_2limb::powf_strict::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::D::<Int<3>, SCALE>(base.resize_to::<Int<3>>()).powf_strict_with(crate::D::<Int<3>, SCALE>(exponent.resize_to::<Int<3>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::D::<Int<4>, SCALE>(base.resize_to::<Int<4>>()).powf_strict_with(crate::D::<Int<4>, SCALE>(exponent.resize_to::<Int<4>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::D::<Int<6>, SCALE>(base.resize_to::<Int<6>>()).powf_strict_with(crate::D::<Int<6>, SCALE>(exponent.resize_to::<Int<6>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::D::<Int<8>, SCALE>(base.resize_to::<Int<8>>()).powf_strict_with(crate::D::<Int<8>, SCALE>(exponent.resize_to::<Int<8>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::D::<Int<12>, SCALE>(base.resize_to::<Int<12>>()).powf_strict_with(crate::D::<Int<12>, SCALE>(exponent.resize_to::<Int<12>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::D::<Int<16>, SCALE>(base.resize_to::<Int<16>>()).powf_strict_with(crate::D::<Int<16>, SCALE>(exponent.resize_to::<Int<16>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::D::<Int<24>, SCALE>(base.resize_to::<Int<24>>()).powf_strict_with(crate::D::<Int<24>, SCALE>(exponent.resize_to::<Int<24>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::D::<Int<32>, SCALE>(base.resize_to::<Int<32>>()).powf_strict_with(crate::D::<Int<32>, SCALE>(exponent.resize_to::<Int<32>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::D::<Int<48>, SCALE>(base.resize_to::<Int<48>>()).powf_strict_with(crate::D::<Int<48>, SCALE>(exponent.resize_to::<Int<48>>()), mode).0.resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::D::<Int<64>, SCALE>(base.resize_to::<Int<64>>()).powf_strict_with(crate::D::<Int<64>, SCALE>(exponent.resize_to::<Int<64>>()), mode).0.resize_to::<Int<N>>()),
        _ => crate::algos::pow::powf_series_2limb::powf_strict::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>),
    }
}

#[inline]
fn exp_with_ln_routed<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => crate::algos::pow::powf_series_2limb::powf_strict::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf_strict", SCALE)
        }),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d57::Core, SCALE,
        >(base.resize_to::<Int<3>>(), exponent.resize_to::<Int<3>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d76::Core, SCALE,
        >(base.resize_to::<Int<4>>(), exponent.resize_to::<Int<4>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d115::Core, SCALE,
        >(base.resize_to::<Int<6>>(), exponent.resize_to::<Int<6>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d153::Core, SCALE,
        >(base.resize_to::<Int<8>>(), exponent.resize_to::<Int<8>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d230::Core, SCALE,
        >(base.resize_to::<Int<12>>(), exponent.resize_to::<Int<12>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d307::Core, SCALE,
        >(base.resize_to::<Int<16>>(), exponent.resize_to::<Int<16>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d462::Core, SCALE,
        >(base.resize_to::<Int<24>>(), exponent.resize_to::<Int<24>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d616::Core, SCALE,
        >(base.resize_to::<Int<32>>(), exponent.resize_to::<Int<32>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d924::Core, SCALE,
        >(base.resize_to::<Int<48>>(), exponent.resize_to::<Int<48>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d1232::Core, SCALE,
        >(base.resize_to::<Int<64>>(), exponent.resize_to::<Int<64>>(), mode)
        .resize_to::<Int<N>>(),
        _ => crate::algos::pow::powf_series_2limb::powf_strict::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf_strict", SCALE)
        }),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(
            crate::algos::pow::pow_schoolbook::pow_schoolbook_strict::<SCALE>(
                base.resize_to::<Int<2>>(),
                exponent.resize_to::<Int<2>>(),
                mode,
            ),
            "powf_strict",
            SCALE,
        ),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d57::Core, SCALE,
        >(base.resize_to::<Int<3>>(), exponent.resize_to::<Int<3>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d76::Core, SCALE,
        >(base.resize_to::<Int<4>>(), exponent.resize_to::<Int<4>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d115::Core, SCALE,
        >(base.resize_to::<Int<6>>(), exponent.resize_to::<Int<6>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d153::Core, SCALE,
        >(base.resize_to::<Int<8>>(), exponent.resize_to::<Int<8>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d230::Core, SCALE,
        >(base.resize_to::<Int<12>>(), exponent.resize_to::<Int<12>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d307::Core, SCALE,
        >(base.resize_to::<Int<16>>(), exponent.resize_to::<Int<16>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d462::Core, SCALE,
        >(base.resize_to::<Int<24>>(), exponent.resize_to::<Int<24>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d616::Core, SCALE,
        >(base.resize_to::<Int<32>>(), exponent.resize_to::<Int<32>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d924::Core, SCALE,
        >(base.resize_to::<Int<48>>(), exponent.resize_to::<Int<48>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d1232::Core, SCALE,
        >(base.resize_to::<Int<64>>(), exponent.resize_to::<Int<64>>(), mode)
        .resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(
            crate::algos::pow::pow_schoolbook::pow_schoolbook_strict::<SCALE>(
                base.resize_to::<Int<2>>(),
                exponent.resize_to::<Int<2>>(),
                mode,
            ),
            "powf_strict",
            SCALE,
        ),
    }
}

/// `powf` with caller-chosen working digits — the `_approx_with` seam.
/// Narrow tiers run `powf_series_2limb::powf_with`; the wide `_` arm runs
/// the C-generic `pow_schoolbook` (the wide approx surface is not
/// policy-routed today, so the wide arms are reached only for totality).
#[inline]
#[must_use]
pub(crate) fn dispatch_with<const N: usize, const SCALE: u32>(
    base: Int<N>,
    exponent: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    match N {
        1 | 2 => crate::algos::pow::powf_series_2limb::powf_with::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            working_digits,
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf_with", SCALE)
        }),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d57::Core, SCALE,
        >(base.resize_to::<Int<3>>(), exponent.resize_to::<Int<3>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d76::Core, SCALE,
        >(base.resize_to::<Int<4>>(), exponent.resize_to::<Int<4>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d115::Core, SCALE,
        >(base.resize_to::<Int<6>>(), exponent.resize_to::<Int<6>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d153::Core, SCALE,
        >(base.resize_to::<Int<8>>(), exponent.resize_to::<Int<8>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d230::Core, SCALE,
        >(base.resize_to::<Int<12>>(), exponent.resize_to::<Int<12>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d307::Core, SCALE,
        >(base.resize_to::<Int<16>>(), exponent.resize_to::<Int<16>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d462::Core, SCALE,
        >(base.resize_to::<Int<24>>(), exponent.resize_to::<Int<24>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d616::Core, SCALE,
        >(base.resize_to::<Int<32>>(), exponent.resize_to::<Int<32>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d924::Core, SCALE,
        >(base.resize_to::<Int<48>>(), exponent.resize_to::<Int<48>>(), mode)
        .resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::pow::pow_schoolbook::pow_schoolbook::<
            crate::types::widths::wide_trig_d1232::Core, SCALE,
        >(base.resize_to::<Int<64>>(), exponent.resize_to::<Int<64>>(), mode)
        .resize_to::<Int<N>>(),
        _ => crate::algos::pow::powf_series_2limb::powf_with::<SCALE>(
            base.resize_to::<Int<2>>(),
            exponent.resize_to::<Int<2>>(),
            working_digits,
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf_with", SCALE)
        }),
    }
}
