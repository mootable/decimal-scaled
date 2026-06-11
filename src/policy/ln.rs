// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Natural-logarithm policy — the per-(N, SCALE) algorithm matcher
//! (plus the derived log2 / log10; arbitrary-base log lives in
//! `policy::log`).
//!
//! `D<Int<N>, SCALE>::ln_strict_with(mode)` delegates directly to the one
//! shared [`dispatch`] generic function — the canonical matcher-only
//! policy shape (see `docs/ARCHITECTURE.md`), mirrored from `sqrt`:
//!
//! 1. an [`Algorithm`] enum — Series / Tang / Schoolbook, no `Default`;
//! 2. a [`Select`] verdict;
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via `const { select::<N, SCALE>() }`, then an exhaustive
//!    `match algo` — no `_`, no panic.
//!
//! The narrow tiers run the 256-bit `Fixed` kernel (`ln_series_2limb`,
//! D18 widened to Int<2>); the wide tiers run the tier-generic `ln_series`
//! over `WideTrigCore`, or the per-tier `ln_tang` band kernel, reached by
//! a `match N` with `resize_to` bridges (identity at the matched `N`).
//!
//! log2 / log10 are derived (`ln(x)/ln2`, `ln(x)/ln10`) and route DOWN to
//! the narrow `ln_series_2limb::{log2,log10}_*` kernels or the wide
//! per-tier `wide_trig_<tier>::log{2,10}_*_with_kernel` free fns — never
//! back through a sibling decimal policy.

use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;
#[cfg(feature = "_wide-support")]
use crate::algos::support::wide_trig_core::WideTrigCore;

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
        // snapped to benchmarked cells (the prior Class-I gate shape). The narrow
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
    checked_dispatch::<N, SCALE>(raw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("ln_strict", SCALE)
    })
}

/// The `checked` primitive under [`dispatch`]: same routing, but the
/// narrow kernels' out-of-range `None` propagates instead of panicking.
/// On the wide tiers the kernel-internal out-of-range panic is not yet
/// threaded through (see `research/checked_wide_shell_patch.md`); those
/// arms return `Some` of the kernel result and still panic on overflow.
/// Domain errors (`raw <= 0`) stay kernel panics — the `checked_`
/// surface prechecks the domain before calling here.
#[inline]
#[must_use]
pub(crate) fn checked_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
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
        1 | 2 => crate::algos::ln::ln_series_2limb::ln_with(
            raw.resize_to::<Int<2>>(),
            SCALE,
            working_digits,
            mode,
        )
        .and_then(super::narrow_fit::<N>)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("ln_with", SCALE)
        }),
        _ => {
            let _ = working_digits;
            dispatch::<N, SCALE>(raw, mode)
        }
    }
}

#[inline]
fn series_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::ln_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::algos::support::wide_trig_core::ln_series::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => crate::algos::ln::ln_series_2limb::ln_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[inline]
fn schoolbook_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Option<Int<N>> {
    match N {
        1 | 2 => super::narrow_fit::<N>(crate::algos::ln::ln_schoolbook::ln_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode)),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d57::Core, SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d76::Core, SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d115::Core, SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d153::Core, SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d230::Core, SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d307::Core, SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d462::Core, SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d616::Core, SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d924::Core, SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::algos::ln::ln_schoolbook::ln_schoolbook::<crate::types::widths::wide_trig_d1232::Core, SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => super::narrow_fit::<N>(crate::algos::ln::ln_schoolbook::ln_schoolbook_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode)),
    }
}

/// The SCALE-derived work-rung for the Tang `ln` kernel (the L7 work-width
/// campaign). A Tang-INTERNAL second axis — NOT in the `select` verdict, NOT on
/// [`Algorithm`]: consulted only inside the Tang routing path. The chosen rung
/// monomorphises the ONE generic kernel `ln_tang_g` at a narrower work integer
/// for sub-max-scale cells (where the tier's full `$Work` is over-wide),
/// const-folded away per `(N, SCALE)`. Variants are the ComputeLimbs widths the
/// ladder can span (min storage `Int<3>` .. max ln floor `Int<176>`).
#[cfg(feature = "_wide-support")]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Rung {
    W3,
    W4,
    W6,
    W8,
    W12,
    W16,
    W24,
    W32,
    W48,
    W64,
    W96,
    W128,
    W176,
}

/// The candidate rung ladder (ascending ComputeLimbs widths). Every wide tier's
/// storage width AND `$Work` floor is a member, so `pick_rung_limbs` can always
/// land on an enumerated width.
#[cfg(feature = "_wide-support")]
const AVAIL_RUNGS: [usize; 13] = [3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 176];

/// Smallest available width (limbs) whose directed-Ziv cap (`limbs·8` decimal
/// digits, = `BITS/8`) clears `SCALE + MARGIN`, clamped to `[storage, floor]`.
/// If even `floor` cannot clear it (the tier's max-scale extreme), `floor` is
/// the answer — and there it reproduces the current per-tier `$Work`, so the
/// max-scale cells stay bit-identical.
///
/// `MARGIN` is the directed-Ziv escalation headroom above the working scale.
/// Wide tiers (storage >= 16 limbs) use `MARGIN = 24`: their near-grid-line
/// validity is monotone, so the tighter margin lands the narrowest valid rung.
/// Narrow tiers (storage < 16) keep `MARGIN = 51`: their validity is
/// non-monotone near the grid line, so no single tighter margin is safe — `51`
/// is never too aggressive, at the cost of some missed narrowing. Each tier
/// carries only its own width (rule 6); the golden gate is the correctness wall.
#[cfg(feature = "_wide-support")]
const fn pick_rung_limbs(scale: u32, storage: usize, floor: usize) -> usize {
    // Per-tier margin (measured map): wide tiers tighten to 24, narrow stay safe at 51.
    let margin: u32 = if storage >= 16 { 24 } else { 51 };
    let need = scale + margin;
    let mut i = 0;
    while i < AVAIL_RUNGS.len() {
        let w = AVAIL_RUNGS[i];
        if w >= storage && w <= floor && (w as u32) * 8 > need {
            return w;
        }
        i += 1;
    }
    floor
}

/// Resolve the work rung for tier `C` at `SCALE` — derives `[storage, floor]`
/// from `C`'s own associated types (`C::Storage`, `C::W` = the tier's `$Work`),
/// so ONE generic selector serves every wide tier (no per-tier ladder, no extra
/// const knob — the BigRule's "inspect your own types" allowance).
#[cfg(feature = "_wide-support")]
const fn work_rung<C: WideTrigCore, const SCALE: u32>() -> Rung {
    let storage = <C::Storage as BigInt>::LIMBS;
    let floor = <C::W as BigInt>::LIMBS;
    match pick_rung_limbs(SCALE, storage, floor) {
        3 => Rung::W3,
        4 => Rung::W4,
        6 => Rung::W6,
        8 => Rung::W8,
        12 => Rung::W12,
        16 => Rung::W16,
        24 => Rung::W24,
        32 => Rung::W32,
        48 => Rung::W48,
        64 => Rung::W64,
        96 => Rung::W96,
        128 => Rung::W128,
        _ => Rung::W176,
    }
}

/// The Tang arm (every wide tier): pick the work rung, then call the ONE generic
/// kernel [`ln_tang_g`] at that rung. `const { work_rung::<C, SCALE>() }` folds
/// per monomorphisation, so a concrete `D###<S>` collapses to a single direct
/// call at exactly one `Int<K>` — no runtime branch, no binary bloat (the
/// unchosen arms are dead-arm-eliminated). The rung never surfaces above this fn
/// (no-leak: `dispatch`/`select`/`Algorithm` unchanged). `(G, CAP, DIR, IE)` are
/// the tier's existing Tang params, threaded through.
#[cfg(feature = "_wide-support")]
#[inline]
fn tang_at_rung<
    C: WideTrigCore,
    const SCALE: u32,
    const G: u32,
    const CAP: u128,
    const DIR: bool,
    const IE: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    use crate::algos::ln::ln_tang::ln_tang_g;
    match const { work_rung::<C, SCALE>() } {
        Rung::W3 => ln_tang_g::<C, Int<3>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W4 => ln_tang_g::<C, Int<4>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W6 => ln_tang_g::<C, Int<6>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W8 => ln_tang_g::<C, Int<8>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W12 => ln_tang_g::<C, Int<12>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W16 => ln_tang_g::<C, Int<16>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W24 => ln_tang_g::<C, Int<24>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W32 => ln_tang_g::<C, Int<32>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W48 => ln_tang_g::<C, Int<48>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W64 => ln_tang_g::<C, Int<64>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W96 => ln_tang_g::<C, Int<96>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W128 => ln_tang_g::<C, Int<128>, SCALE, G, CAP, DIR, IE>(raw, mode),
        Rung::W176 => ln_tang_g::<C, Int<176>, SCALE, G, CAP, DIR, IE>(raw, mode),
    }
}

#[cfg(feature = "_wide-support")]
#[inline]
fn tang_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Option<Int<N>> {
    // Per-tier `(GUARD, CAP)` tuning for the Tang kernel. The select gates
    // cover the FULL valid scale range for each tier (see [`select`]); the
    // `ln_wide_series_tang_ab` map confirmed every (G, CAP) candidate is
    // bit-identical to Series at every cell (zero validity failures across
    // 35 cells × 3 inputs × 6 modes), so the choice here is purely a
    // performance tuning. The Tang win over Series ranges from 4.5× (low
    // scales) to 57× (max scales) per the same map.
    match N {
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(tang_at_rung::<crate::types::widths::wide_trig_d57::Core, SCALE, 8, 100, true, false>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(tang_at_rung::<crate::types::widths::wide_trig_d76::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(tang_at_rung::<crate::types::widths::wide_trig_d115::Core, SCALE, 8, 200, true, false>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(tang_at_rung::<crate::types::widths::wide_trig_d153::Core, SCALE, 10, 200, true, false>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(tang_at_rung::<crate::types::widths::wide_trig_d230::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(tang_at_rung::<crate::types::widths::wide_trig_d307::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(tang_at_rung::<crate::types::widths::wide_trig_d462::Core, SCALE, 10, 400, true, true>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(tang_at_rung::<crate::types::widths::wide_trig_d616::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(tang_at_rung::<crate::types::widths::wide_trig_d924::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(tang_at_rung::<crate::types::widths::wide_trig_d1232::Core, SCALE, 10, 400, true, false>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => series_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
#[must_use]
pub(crate) fn log2_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    checked_log2_dispatch::<N, SCALE>(raw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("log2_strict", SCALE)
    })
}

/// The `checked` primitive under [`log2_dispatch`]: exact out-of-range
/// `None` on the narrow tiers; the wide arms call the per-tier kernel
/// shells, whose internal out-of-range panic is not yet threaded
/// through (see `research/checked_wide_shell_patch.md`).
#[inline]
#[must_use]
pub(crate) fn checked_log2_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::log2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::types::widths::wide_trig_d57::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::types::widths::wide_trig_d76::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::types::widths::wide_trig_d115::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::types::widths::wide_trig_d153::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::types::widths::wide_trig_d230::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::types::widths::wide_trig_d307::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::types::widths::wide_trig_d462::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::types::widths::wide_trig_d616::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::types::widths::wide_trig_d924::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::types::widths::wide_trig_d1232::log2_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => crate::algos::ln::ln_series_2limb::log2_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[inline]
#[must_use]
pub(crate) fn log2_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, working_digits: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::log2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode).and_then(super::narrow_fit::<N>).unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("log2_with", SCALE)),
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
        _ => crate::algos::ln::ln_series_2limb::log2_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode).and_then(super::narrow_fit::<N>).unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("log2_with", SCALE)),
    }
}

#[inline]
#[must_use]
pub(crate) fn log10_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    checked_log10_dispatch::<N, SCALE>(raw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("log10_strict", SCALE)
    })
}

/// The `checked` primitive under [`log10_dispatch`]; see
/// [`checked_log2_dispatch`] for the narrow/wide split.
#[inline]
#[must_use]
pub(crate) fn checked_log10_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::log10_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
        #[cfg(any(feature = "d57", feature = "wide"))]
        3 => Some(crate::types::widths::wide_trig_d57::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<3>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d76", feature = "wide"))]
        4 => Some(crate::types::widths::wide_trig_d76::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<4>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d115", feature = "wide"))]
        6 => Some(crate::types::widths::wide_trig_d115::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<6>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d153", feature = "wide"))]
        8 => Some(crate::types::widths::wide_trig_d153::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<8>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d230", feature = "wide"))]
        12 => Some(crate::types::widths::wide_trig_d230::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<12>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        16 => Some(crate::types::widths::wide_trig_d307::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<16>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        24 => Some(crate::types::widths::wide_trig_d462::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<24>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d616", feature = "x-wide"))]
        32 => Some(crate::types::widths::wide_trig_d616::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<32>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d924", feature = "xx-wide"))]
        48 => Some(crate::types::widths::wide_trig_d924::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<48>>(), mode).resize_to::<Int<N>>()),
        #[cfg(any(feature = "d1232", feature = "xx-wide"))]
        64 => Some(crate::types::widths::wide_trig_d1232::log10_strict_with_kernel::<SCALE>(raw.resize_to::<Int<64>>(), mode).resize_to::<Int<N>>()),
        _ => crate::algos::ln::ln_series_2limb::log10_strict::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[inline]
#[must_use]
pub(crate) fn log10_dispatch_with<const N: usize, const SCALE: u32>(raw: Int<N>, working_digits: u32, mode: RoundingMode) -> Int<N> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::log10_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode).and_then(super::narrow_fit::<N>).unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("log10_with", SCALE)),
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
        _ => crate::algos::ln::ln_series_2limb::log10_with(raw.resize_to::<Int<2>>(), SCALE, working_digits, mode).and_then(super::narrow_fit::<N>).unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("log10_with", SCALE)),
    }
}
