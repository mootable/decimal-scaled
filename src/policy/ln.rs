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
        // The table-driven Tang kernel eliminates the Series path's wide
        // argument-reduction sqrts and is bit-identical to Series (the
        // correctly-rounded oracle) across every wide tier's full valid
        // scale range. The wide-tier `ln_wide_series_tang_ab` map (the
        // N-way width × scale × (G, CAP) sweep, 35 cells, 3-input × 6-mode
        // validity wall) shows Tang beats Series by 4.5×-57× at EVERY
        // (N, SCALE) cell across {0, S/4, S/2, 3S/4, S-1} for every wide
        // tier, with zero validity failures. So Tang owns the whole range
        // at every tier — narrow-wide AND wide — not just point ranges
        // snapped to bbc cells (the prior Class-I gate shape). The narrow
        // tiers exclude SCALE=0 because the narrow-wide validation
        // (`ln_series_tang_ab`) was only run for SCALE >= 1.
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 1..=56) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d76", feature = "wide"))]
        (4, 0..=75) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 1..=114) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 1..=152) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d230", feature = "wide"))]
        (12, 0..=229) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        (16, 0..=306) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        (24, 0..=461) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        (32, 0..=615) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        (48, 0..=923) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        (64, 0..=1231) => Select::ByAlgorithm(Algorithm::Tang),
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

/// Returns `true` iff the policy routes Tang at this `(N, SCALE)` cell.
///
/// Used by the working-scale `ln_fixed_routed<SCALE>` surface emitted per
/// tier by `decl_wide_transcendental!` to keep its scale gates in sync
/// with the canonical [`select`] above — the SAME wide-tier Tang gates,
/// just read at the working-scale call sites that compose ln (log, log2,
/// log10, powf, asinh, acosh, atanh) instead of at the strict storage
/// dispatcher [`dispatch`]. If [`select`] widens further, the routed
/// surface tracks it automatically through this query.
#[cfg(feature = "_wide-support")]
#[inline]
#[must_use]
pub(crate) const fn is_tang<const N: usize, const SCALE: u32>() -> bool {
    match select::<N, SCALE>() {
        Select::ByAlgorithm(Algorithm::Tang) => true,
        _ => false,
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
    // Per-tier `(GUARD, CAP)` tuning for the Tang kernel. The select gates
    // cover the FULL valid scale range for each tier (see [`select`]); the
    // `ln_wide_series_tang_ab` map confirmed every (G, CAP) candidate is
    // bit-identical to Series at every cell (zero validity failures across
    // 35 cells × 3 inputs × 6 modes), so the choice here is purely a
    // performance tuning. The Tang win over Series ranges from 4.5× (low
    // scales) to 57× (max scales) per the same map.
    match N {
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 100, true, false>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d76::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 200, true, false>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 200, true, false>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d230::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d307::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d462::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d616::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d924::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>(),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => crate::algos::ln::ln_tang::ln_tang::<crate::types::widths::wide_trig_d1232::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>(),
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
