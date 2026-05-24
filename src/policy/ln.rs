//! Natural-logarithm policy â€” the per-(N, SCALE) algorithm matcher
//! (plus the derived log2 / log10; arbitrary-base log lives in
//! `policy::log`).
//!
//! `D<Int<N>, SCALE>::ln_strict_with(mode)` delegates directly to the one
//! shared [`dispatch`] generic function â€” the canonical matcher-only
//! policy shape (see `docs/ARCHITECTURE.md`), mirrored from `sqrt`:
//!
//! 1. an [`Algorithm`] enum â€” Series / Tang / Schoolbook, no `Default`;
//! 2. a [`Select`] verdict;
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via `const { select::<N, SCALE>() }`, then an exhaustive
//!    `match algo` â€” no `_`, no panic.
//!
//! The narrow tiers run the 256-bit `Fixed` kernel (`ln_series_2limb`,
//! D18 widened to Int<2>); the wide tiers run the tier-generic `ln_series`
//! over `WideTrigCore`, or the per-tier `ln_tang` band kernel, reached by
//! a `match N` with `resize_to` bridges (identity at the matched `N`).
//!
//! log2 / log10 are derived (`ln(x)/ln2`, `ln(x)/ln10`) and route DOWN to
//! the narrow `ln_series_2limb::{log2,log10}_*` kernels or the wide
//! per-tier `wide_trig_<tier>::log{2,10}_*_with_kernel` free fns â€” never
//! back through a sibling decimal policy.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    Series,
    #[cfg(feature = "_wide-support")]
    Tang,
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
    match (N, SCALE) {
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 18..=22) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 50..=60) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 70..=82) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d230", feature = "wide"))]
        (12, 110..=120) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 140..=160) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 285..=295) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        (24, 225..=235) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        (32, 300..=315) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        (32, 585..=595) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        (48, 455..=465) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        (48, 895..=905) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        (64, 610..=620) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        (64, 1195..=1205) => Select::ByAlgorithm(Algorithm::Tang),
        _ => Select::ByAlgorithm(Algorithm::Series),
    }
}

#[inline]
fn resolve<const N: usize, const SCALE: u32>(raw: &Int<N>) -> Algorithm {
    match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(raw),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match resolve::<N, SCALE>(&raw) {
        Algorithm::Series => series_routed::<N, SCALE>(raw, mode),
        #[cfg(feature = "_wide-support")]
        Algorithm::Tang => tang_routed::<N, SCALE>(raw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn dispatch_with<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<N> {
    // The wide series/tang kernels are strict-guard; only the narrow tier
    // honours caller working_digits (matching the prior LnPolicy routing,
    // where wide ln_with_impl ignored working_digits).
    match N {
        1 | 2 => super::narrow_checked::<N>(
            crate::algos::ln::ln_series_2limb::ln_with(
                raw.resize_to::<Int<2>>(),
                SCALE,
                working_digits,
                mode,
            ),
            "ln_with",
            SCALE,
        ),
        _ => {
            let _ = working_digits;
            dispatch::<N, SCALE>(raw, mode)
        }
    }
}

#[inline]
fn series_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::ln_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "ln_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::ln_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "ln_strict", SCALE),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::ln::ln_schoolbook::ln_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "ln_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::ln::ln_schoolbook::ln_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "ln_strict", SCALE),
    }
}

#[cfg(feature = "_wide-support")]
#[inline]
fn tang_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => {
            let r = raw.resize_to::<Int<3>>();
            let out = match SCALE {
                18..=22 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 100, false>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => {
            let r = raw.resize_to::<Int<6>>();
            let out = match SCALE {
                50..=60 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 200, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => {
            let r = raw.resize_to::<Int<8>>();
            let out = match SCALE {
                70..=82 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 200, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => {
            let r = raw.resize_to::<Int<12>>();
            let out = match SCALE {
                110..=120 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d230::Core, SCALE, 10, 200, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d230::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => {
            let r = raw.resize_to::<Int<16>>();
            let out = match SCALE {
                140..=160 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 8, 400, true>(r, mode),
                285..=295 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 10, 400, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d307::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => {
            let r = raw.resize_to::<Int<24>>();
            let out = match SCALE {
                225..=235 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d462::Core, SCALE, 8, 400, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d462::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => {
            let r = raw.resize_to::<Int<32>>();
            let out = match SCALE {
                300..=315 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d616::Core, SCALE, 10, 400, true>(r, mode),
                585..=595 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d616::Core, SCALE, 10, 400, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d616::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => {
            let r = raw.resize_to::<Int<48>>();
            let out = match SCALE {
                455..=465 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d924::Core, SCALE, 8, 400, true>(r, mode),
                895..=905 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d924::Core, SCALE, 10, 400, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d924::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => {
            let r = raw.resize_to::<Int<64>>();
            let out = match SCALE {
                610..=620 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d1232::Core, SCALE, 8, 400, true>(r, mode),
                1195..=1205 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d1232::Core, SCALE, 10, 400, true>(r, mode),
                _ => crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d1232::Core, SCALE>(r, mode),
            };
            out.resize_to::<Int<N>>()
        }
        _ => series_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn log2_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "log2_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "log2_strict", SCALE),
    }
}

#[inline]
#[must_use]
pub(crate) fn log2_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, working_digits: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log2_with", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::log2_approx_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), working_digits, mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log2_with", SCALE),
    }
}

#[inline]
#[must_use]
pub(crate) fn log10_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log10_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "log10_strict", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log10_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode), "log10_strict", SCALE),
    }
}

#[inline]
#[must_use]
pub(crate) fn log10_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, working_digits: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log10_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log10_with", SCALE),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::types::widths::wide_trig_d57::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::types::widths::wide_trig_d76::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::types::widths::wide_trig_d115::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::types::widths::wide_trig_d153::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::types::widths::wide_trig_d230::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::types::widths::wide_trig_d307::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::types::widths::wide_trig_d462::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::types::widths::wide_trig_d616::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::types::widths::wide_trig_d924::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), working_digits, mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::types::widths::wide_trig_d1232::log10_approx_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), working_digits, mode).resize_to::<Int<N>>(),
        _ => super::narrow_checked::<N>(crate::algos::ln::ln_series_2limb::log10_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode), "log10_with", SCALE),
    }
}
