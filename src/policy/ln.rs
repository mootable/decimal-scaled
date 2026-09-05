// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Natural-logarithm policy — the per-(N, SCALE) algorithm matcher
//! (plus the derived log2 / log10; arbitrary-base log lives in
//! `policy::log`).
//!
//! `D<Int<N>, SCALE>::ln_with(mode)` delegates directly to the one
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
    /// The table-driven Tang kernel. No longer `_wide-support`-gated: the
    /// kernel was lifted free of `WideTrigCore`, so the narrow tiers
    /// (D18/D38) can be routed to it too. While it WAS gated, narrow Tang
    /// was structurally unselectable — a computation the matcher could not
    /// reach (`architectural-review` Class N).
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
        // ── NARROW (D18 / D38) — closing a width INVERSION ──────────────
        //
        // These tiers ran `ln_series_2limb` (artanh at `|t| <= 1/3`, so
        // ~1.05·w terms, on the 256-bit `Fixed` type at `STRICT_GUARD = 30`)
        // while every WIDER tier ran Tang (`|t| < 1/256`, ~0.21·w terms, at
        // guard 8-10). Narrower storage was therefore doing MORE work than
        // wider storage: `ln_nd` (a non-degenerate argument — see the
        // benchmark-hazard note in `ln_tang::tang_ln_fixed_g`) measured D38 at
        // 2431-8205 ns against D57's 566-1697 and D76's 603-1750, a 4.0-5.8x
        // inversion. An inversion is a defect on the NARROW side by
        // definition: less storage must never cost more time.
        //
        // Tang was not merely unrouted here, it was UNROUTABLE: `Algorithm::
        // Tang`, `algos::ln::ln_tang` and the table were all
        // `_wide-support`-gated, and the kernel was bound `C: WideTrigCore`,
        // which no narrow tier implements. Lifting the kernel free of `C`
        // (storage as a plain `BigInt`, storage bounds as values, the table as
        // a type parameter) is what makes this arm expressible at all.
        //
        // ── THE RANGE IS THE TIER'S WHOLE RANGE, NOT A CARVE-OUT ─────────
        //
        // `0..=37` is FULL coverage, not a fitted band: the crate caps
        // `MAX_SCALE = N - 1` for `D{N}` (`lib.rs` — `SCALE = MAX_SCALE + 1`
        // is rejected at COMPILE time), so D38's legal scales are `0..=37` and
        // D18's are `0..=17`. Both tiers are covered end to end, exactly as
        // every wide arm below covers its own tier number minus one
        // (D57 -> 56, D76 -> 75, D1232 -> 1231). There is no top cell falling
        // through to Series, and `0..=38` would be a dead bound — `D38<38>`
        // does not compile.
        //
        // Validity across that whole range is a capacity argument, not a
        // measurement: the narrow work rung is `Int<12>`, whose Ziv-escalation
        // budget is `BITS/8 = 96` decimal digits, against a worst-cell need of
        // `SCALE + MARGIN = 37 + 51 = 88` under the same narrow-storage margin
        // `work_rung::ln_rung` already applies below 16 limbs. The Tang table
        // read needs 6 of its 8 stored limbs at `w = 96`. Both clear with
        // room, and identically at every scale in between — the bound moves
        // monotonically in `SCALE`, so there is no interior cell to bisect.
        (1 | 2, 0..=37) => Select::ByAlgorithm(Algorithm::Tang),
        // The table-driven Tang kernel eliminates the Series path's wide
        // argument-reduction sqrts and is bit-identical to Series (the
        // correctly-rounded oracle) across every wide tier's full valid
        // scale range. Tang wins or ties EVERY cell of the wide-tier
        // `ln_wide_series_tang_ab` map, so it owns the whole range at every
        // tier — narrow-wide AND wide — not point ranges snapped to
        // benchmarked cells. SCALE = 0 is included at every tier: there is no
        // validity wall at s0 and Tang wins there as at every neighbouring
        // scale. The golden gate covers the s0 band edge at these widths.
        //
        // ── THE MARGIN, MEASURED (series ÷ best Tang, 2026-09-03) ──
        //
        // An earlier version of this comment claimed 4.5×-57× at every cell.
        // **That figure was an artefact and must not be re-derived from the
        // old bench.** It came from the operand spread `{0.5, 2.0, 7.5}`,
        // every member of which short-circuits the kernel it was timing:
        // `0.5` and `2.0` are exact powers of two, so BOTH kernels take the
        // `m == 1` arm (`ln_tang::tang_ln_fixed_g`, `exp_generic::ln_fixed`)
        // and return `k·ln2` from a one-word product; and `7.5 = 2²·1.875`
        // with `1.875 = 1 + 112/128` sits EXACTLY on Tang table index 112, so
        // Tang's residual `t` is exactly zero and its artanh loop breaks on
        // the first iteration while Series still pays its full reduction.
        // The 57× was that asymmetry, not a kernel result. The same spread
        // voided the old validity wall, which never once ran the artanh
        // series. `benches/micro/ln_wide_series_tang_ab.rs` now states the
        // non-degeneracy contract (`raw` odd and `raw % 5 != 0` for
        // SCALE ≥ 1; odd and ≥ 257 at SCALE 0) and asserts it per operand.
        //
        // Re-raced on `1/3` and `7/3` (full-width repeating decimals, one
        // either side of 1), the true surface is **1.02×-4.22×**:
        //
        //   tier    s0     S/4    S/2    3S/4   S-1
        //   D76    2.62×  3.44×  3.17×  3.10×  2.49×
        //   D230   2.94×  4.22×  2.44×  2.03×  2.21×
        //   D307   2.62×  2.22×  2.16×  1.74×  1.73×
        //   D462   2.72×  2.70×  1.92×  1.57×  1.43×
        //   D616   3.23×  2.06×  1.56×  1.46×  1.24×
        //   D924   2.44×  1.47×  1.53×  1.20×  1.15×
        //   D1232  2.51×  1.87×  1.31×  1.22×  1.02×  ← a TIE
        //
        // The margin falls monotonically along BOTH axes — in scale within
        // every tier, and in width along the top-scale column (2.49 → 1.02).
        // Mechanism: Tang's artanh series is **O(w)** terms (`|t| ≤ 1/256`
        // needs `2j+1 > w/2.408`) while Series' Brent reduction is
        // **O(√w)** sqrt levels followed by a shorter series, so the gap
        // closes as the working scale grows.
        //
        // At D1232<1231> the two are TIED (1.02×, under the within-cell
        // replication floor), and that cell is rung-FAITHFUL — `ln_rung`
        // returns `C::W` when no ladder member clears the budget, so the
        // bench ran the same width `tang_at_rung` routes. The arms below are
        // unchanged because a tie warrants no move in either direction, and
        // Tang still wins outright everywhere else. A future re-tune that
        // wants to move this boundary needs the D1232 s924 → s1231 segment
        // bisected first; it is the one unbisected winner change in the grid.
        //
        // Caveat for anyone re-reading the raw bench numbers: the
        // `__bench_internals` Tang exports call `ln_tang` (`Wk = C::W`),
        // whereas `tang_routed` routes the narrowest `ln_rung`. That
        // handicaps Tang by up to 2.75× at LOW and MID scales, so those
        // margins are lower bounds; the top-scale cells are exact (D307<306>
        // excepted, 1.33×).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 0..=56) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d76", feature = "wide"))]
        (4, 0..=75) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d115", feature = "wide"))]
        (6, 0..=114) => Select::ByAlgorithm(Algorithm::Tang),
        #[cfg(any(feature = "d153", feature = "wide"))]
        (8, 0..=152) => Select::ByAlgorithm(Algorithm::Tang),
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
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(raw),
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
    matches!(select::<N, SCALE>(), Select::ByAlgorithm(Algorithm::Tang))
}

#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    checked_dispatch::<N, SCALE>(raw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("ln", SCALE)
    })
}

/// The `checked` primitive under [`dispatch`]: same routing, but the
/// narrow kernels' out-of-range `None` propagates instead of panicking.
/// On the wide tiers the kernel-internal out-of-range panic is not yet
/// threaded through; those arms return `Some` of the kernel result and still panic on overflow.
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
        Algorithm::Tang => tang_routed::<N, SCALE>(raw, mode),
        Algorithm::Schoolbook => schoolbook_routed::<N, SCALE>(raw, mode),
    }
}

#[inline]
fn series_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::ln::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
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
        _ => crate::algos::ln::ln_series_2limb::ln::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
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

// The SCALE-derived work-rung machinery (the `AVAIL_RUNGS` ladder + the
// `ln_rung` selector) lives in the shared policy-support module
// `super::work_rung` — one ladder + walker for `ln` and the forward
// trig (`policy::trig`), no per-policy copies. A Tang-INTERNAL second
// axis — NOT in the `select` verdict, NOT on [`Algorithm`]: consulted
// only inside the Tang routing path below.
#[cfg(feature = "_wide-support")]
use super::work_rung::ln_rung;

/// The Tang arm (every wide tier): pick the work rung, then call the ONE generic
/// kernel [`ln_tang_g`] at that rung. `const { ln_rung::<C, SCALE>() }` is a
/// plain `usize` that folds per monomorphisation, so a concrete `D###<S>`
/// collapses to a single direct call at exactly one `Int<K>` — no runtime
/// branch, and the unchosen arms are pruned by the monomorphisation collector
/// (an integer-constant switch folds there; an enum verdict would not). The
/// `_` arm is `Int<176>`, reached only by 176 itself (see `work_rung`). The
/// rung never surfaces above this fn (no-leak: `dispatch`/`select`/`Algorithm`
/// unchanged). `(G, CAP, DIR, IE)` are the tier's existing Tang params,
/// threaded through.
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
) -> C::Storage
where
    <C::W as BigInt>::Scratch: crate::int::types::compute_limbs::ComputeLimbs,
{
    use crate::algos::ln::ln_tang::ln_tang_g;
    use crate::algos::support::ln_tang_slot::WideSlots;
    // The lifted kernel is `C`-free: storage type `C::Storage`, rung work
    // width `Int<K>`, fall-up width the tier's full `C::W`, the full-precision
    // `WideSlots` table, storage bounds threaded as values. `C` here only
    // sources those — the same routing, bit-identical to the pre-lift path.
    let (smax, smin) = (C::storage_max(), C::storage_min());
    match const { ln_rung::<C, SCALE>() } {
        3 => ln_tang_g::<C::Storage, Int<3>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        4 => ln_tang_g::<C::Storage, Int<4>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        6 => ln_tang_g::<C::Storage, Int<6>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        8 => ln_tang_g::<C::Storage, Int<8>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        12 => ln_tang_g::<C::Storage, Int<12>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        16 => ln_tang_g::<C::Storage, Int<16>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        24 => ln_tang_g::<C::Storage, Int<24>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        32 => ln_tang_g::<C::Storage, Int<32>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        48 => ln_tang_g::<C::Storage, Int<48>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        64 => ln_tang_g::<C::Storage, Int<64>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        96 => ln_tang_g::<C::Storage, Int<96>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        128 => ln_tang_g::<C::Storage, Int<128>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
        _ => ln_tang_g::<C::Storage, Int<176>, C::W, WideSlots, SCALE, G, CAP, DIR, IE>(raw, smax, smin, mode),
    }
}

/// The NARROW (`D18` / `D38`) Tang arm — the one that closes the width
/// inversion documented on [`select`].
///
/// ## One fixed work rung, `Int<12>` — and why NOT `Int<24>`
///
/// The wide tiers walk a rung ladder because their `$Work` is sized for a
/// max scale far above most cells. There are only two narrow storage
/// widths, so a ladder is machinery this case does not have: one fixed rung
/// serves both tiers at every scale.
///
/// `Int<12>` is that rung, and the choice is load-bearing.
/// `to_work_scaled_g` lifts through `BigInt::resize_to`, which stages the
/// SOURCE magnitude into `[u128; MAX_U128_LIMB]` — and
/// `MAX_U128_LIMB = 4 · MAX_WORK_N` is just **8** in a narrow-only build.
/// `resize_to` indexes that buffer at `ceil(N_source / 2)`, so a source
/// wider than 16 limbs panics on the slice range REGARDLESS OF VALUE.
/// `Int<24>` sources 12 > 8 and so fails on every conditioned call, at
/// every scale and every mode; `Int<12>` sources 6 and is safe in both
/// directions. Its `BITS/8 = 96`-digit escalation budget clears the worst
/// narrow cell's `37 + 51 = 88` (see [`select`]), and it holds the Tang
/// table product.
///
/// ## Storage arithmetic runs in `Int<4>`, to keep `checked_ln` exact
///
/// `ln` genuinely overflows narrow storage: at `D38<37>` the smallest
/// positive argument gives `ln(10^-37) ≈ -85.2`, needing ≈ `8.5·10^38`
/// against D38's ≈ `1.7·10^38` capacity. The narrow Series kernel reports
/// that as `None`, and `checked_ln`'s exactness on D18/D38 is a documented
/// contract (`docs/ARCHITECTURE.md` → "Overflow & domain behaviour").
///
/// The shared narrowing (`wide_trig_core::narrow_range_checked_g`) PANICS
/// on out-of-range instead, so handing it the true storage bounds would
/// convert that `None` into a panic — a silent regression of a shipped
/// contract. Running the kernel's storage arithmetic in `Int<4>` puts the
/// result far inside the bounds it checks, so it cannot trip; this fn then
/// applies the exact fit test itself, matching `narrow_fit`'s semantics for
/// both tiers in one step. `Int<4>` is a local widening of THIS arm only —
/// it imposes nothing on any other tier (Constitution rule 6), exactly as
/// the Series arm already widens D18 into `Int<2>`.
///
/// ## `(GUARD, CAP, DIRECTED, INTERNAL_EXTRA) = (10, 400, true, false)`
///
/// The configuration nine of the ten wide tiers use. `INTERNAL_EXTRA` is
/// `false` because the narrow cells have generous escalation headroom (a
/// 96-digit rung budget against a 47-digit base working scale), which is
/// exactly the condition that makes the pre-widening unnecessary — the wide
/// tiers need it only where the Ziv cap collapses onto the base guard at
/// max scale. `CAP = 400` covers working scales to
/// `(2·400 + 1) · 2.408 ≈ 1928`, against a narrow maximum of 96.
#[inline]
fn tang_narrow<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    use crate::algos::ln::ln_tang::ln_tang_g;
    use crate::algos::support::ln_tang_slot::NarrowSlots;
    let wide = ln_tang_g::<Int<4>, Int<12>, Int<12>, NarrowSlots, SCALE, 10, 400, true, false>(
        raw.resize_to::<Int<4>>(),
        Int::<4>::MAX,
        Int::<4>::MIN,
        mode,
    );
    // The `narrow_fit` round-trip idiom, taken at `Int<4>`: `None` iff the
    // result does not survive the trip back into storage.
    let out = wide.resize_to::<Int<N>>();
    if out.resize_to::<Int<4>>() != wide {
        return None;
    }
    Some(out)
}

#[inline]
fn tang_routed<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Option<Int<N>> {
    // Per-tier `(GUARD, CAP)` tuning for the Tang kernel. The select gates
    // cover the FULL valid scale range for each tier (see [`select`]).
    //
    // ── CAP IS A VALIDITY WALL, NOT ONLY A SAFETY NET ──
    //
    // `CAP` bounds the artanh iteration index `j` in
    // `ln_tang::tang_ln_fixed_g`. The loop is *meant* to exit on a zero
    // contribution long before the cap, but if the cap bites first the series
    // is truncated and the result is WRONG. The coverage bound is derived in
    // full on the `M` const in `crate::algos::ln::ln_tang` (M bounds the
    // residual `t`, which fixes the term count); the usable form is
    //
    //     CAP = C  covers working scales  w ≤ (2C + 1) · 2.408
    //
    // and the requirement is that `w` at its LARGEST — the tier's max scale
    // plus GUARD, plus whatever the directed-Ziv escalation adds on top,
    // itself capped at the rung's `BITS/8` — stays inside that. Per-tier
    // check (2026-09-03), `j` = the terms actually needed at max scale:
    //
    //   tier    CAP   covers w ≤   max w    j needed   margin
    //   D57     100        482        64        13     ample
    //   D115    200        966       122        25     ample
    //   D153    200        966       162        34     ample
    //   D230+   400       1929       ≤1241     ≤258    ample
    //
    // The widest cell is D1232<1231>: `w = 1241`, needing `j ≈ 258`, against
    // `CAP = 400`. Every shipped tier clears its bound with room.
    //
    // This is not hypothetical. The `ln_wide_series_tang_ab` map runs a
    // `CAP = 200` CANDIDATE at every tier, and at D1232<1231> (`w = 1239`,
    // past `CAP=200`'s 966) it is caught by the validity wall:
    // `tang_g8_c200 != series (x_lo, HalfToEven)`. That candidate is not
    // wired anywhere; the wall rejected it exactly as intended. Adding a tier
    // or raising a max scale means re-running that arithmetic — a `CAP` that
    // silently bites produces wrong digits, not slow ones.
    //
    // Note the failure only became detectable once the map moved to
    // non-degenerate operands: under the old `{0.5, 2.0, 7.5}` spread the
    // artanh loop never ran a single iteration, so no `CAP` could ever bite.
    //
    // ── PERFORMANCE ──
    //
    // Choice of (G, CAP) *within* the validity bound is performance tuning.
    // The measured Tang-over-Series margin is **1.02×-4.22×**, LARGEST at low
    // scales and shrinking to a tie at D1232<1231> — see the surface table on
    // [`select`]. An earlier version of this comment said "4.5× (low scales)
    // to 57× (max scales)"; that figure came from operands that
    // short-circuited the kernel, and its direction was also inverted — the
    // margin shrinks with scale, it does not grow. The (G, CAP) winner moves
    // cell to cell with margins mostly ≤ 1.2×, i.e. under the replication
    // floor, so the values below are not claimed to be optimal — only valid.
    match N {
        1 | 2 => tang_narrow::<N, SCALE>(raw, mode),
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
        crate::support::diagnostics::overflow_panic_with_scale("log2", SCALE)
    })
}

/// The `checked` primitive under [`log2_dispatch`]: exact out-of-range
/// `None` on the narrow tiers; the wide arms call the per-tier kernel
/// shells, whose internal out-of-range panic is not yet threaded
/// through.
#[inline]
#[must_use]
pub(crate) fn checked_log2_dispatch<const N: usize, const SCALE: u32>(
    raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>> {
    match N {
        1 | 2 => crate::algos::ln::ln_series_2limb::log2::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
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
        _ => crate::algos::ln::ln_series_2limb::log2::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[inline]
#[must_use]
pub(crate) fn log10_dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N> {
    checked_log10_dispatch::<N, SCALE>(raw, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("log10", SCALE)
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
        1 | 2 => crate::algos::ln::ln_series_2limb::log10::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
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
        _ => crate::algos::ln::ln_series_2limb::log10::<SCALE>(raw.resize_to::<Int<2>>(), mode).and_then(super::narrow_fit::<N>),
    }
}

#[cfg(test)]
mod narrow_tang_tests {
    //! The narrow (D18 / D38) Tang arm must be bit-identical to the Series
    //! kernel it replaces. Both are correctly-rounded, so where both are
    //! valid they agree exactly — this is the correctness wall behind the
    //! routing change, and it runs in EVERY build (the narrow tiers are
    //! always compiled).
    //!
    //! Comparing the two `Option`s rather than two values checks the
    //! overflow contract at the same time: `ln` genuinely overflows narrow
    //! storage at high scales, and `checked_ln`'s exact `None` on D18/D38 is
    //! a documented guarantee. A Tang arm that panicked where Series
    //! returned `None` would fail here.

    use crate::int::types::Int;
    use crate::support::rounding::RoundingMode;

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// Assert the routed narrow Tang arm equals the Series arm on `raws`,
    /// under every rounding mode.
    fn agrees<const N: usize, const SCALE: u32>(raws: &[i128]) {
        for &r in raws {
            let raw = Int::<N>::from_i128(r);
            for mode in ALL_MODES {
                assert_eq!(
                    super::tang_narrow::<N, SCALE>(raw, mode),
                    super::series_routed::<N, SCALE>(raw, mode),
                    "narrow ln disagreed: raw={r} N={N} SCALE={SCALE} mode={mode:?}"
                );
            }
        }
    }

    /// Operands are chosen NON-DEGENERATE (odd, and not a multiple of 5;
    /// at SCALE 0 also >= 257) so neither kernel short-circuits: an exact
    /// power of two takes the `m == 1` early-out in BOTH kernels, and a
    /// mantissa landing exactly on a Tang table index makes Tang's residual
    /// `t` zero so its artanh loop breaks immediately. See the
    /// benchmark-hazard note in `ln_tang::tang_ln_fixed_g`. A degenerate
    /// operand would assert almost nothing.
    #[test]
    fn d38_narrow_tang_matches_series() {
        agrees::<2, 0>(&[257, 999, 100_003, 999_999_937]);
        agrees::<2, 1>(&[3, 7, 13, 12_345_678_901_234_567]);
        agrees::<2, 18>(&[
            1_000_000_000_000_000_001,
            999_999_999_999_999_999,
            3_000_000_000_000_000_007,
            7_000_000_000_000_000_003,
        ]);
        // D38's MAX_SCALE — the tier's top cell, and the largest inversion.
        agrees::<2, 37>(&[
            10i128.pow(37) + 1,
            10i128.pow(37) - 1,
            3 * 10i128.pow(37) + 7,
            7 * 10i128.pow(37) + 3,
        ]);
    }

    #[test]
    fn d18_narrow_tang_matches_series() {
        agrees::<1, 0>(&[257, 999, 100_003]);
        agrees::<1, 9>(&[1_000_000_001, 999_999_999, 3_000_000_007]);
        // D18's MAX_SCALE.
        agrees::<1, 17>(&[
            10i128.pow(17) + 1,
            10i128.pow(17) - 1,
            3 * 10i128.pow(17) + 7,
        ]);
    }

    /// The out-of-range band: at `D38<37>` a small argument drives the
    /// result past storage. `ln(10^-37) ~ -85.2` needs ~8.5e38 against
    /// D38's ~1.7e38, so this MUST report `None` from both kernels rather
    /// than panicking from either.
    #[test]
    fn d38_s37_out_of_range_agrees_and_does_not_panic() {
        agrees::<2, 37>(&[1, 3, 7, 999_999_937]);
        // And it really is the out-of-range band, not a quiet success:
        // if this stops being `None` the case above has lost its point.
        assert_eq!(
            super::tang_narrow::<2, 37>(Int::<2>::from_i128(1), RoundingMode::HalfToEven),
            None,
            "expected ln(10^-37) at D38<37> to overflow storage"
        );
    }
}

#[cfg(test)]
mod tang_rung_tests {
    //! Light anchor for the ln work-rung fall-up (the `policy::trig`
    //! tiny-argument test shape): the rung-routed Tang path must equal
    //! the tier-width `ln_tang` bit-for-bit on the near-1 family
    //! `ln(1 + δ) = δ − δ²/2 + …` whose deciding quadratic term sits
    //! beyond the rung's escalation cap (an unresolved-at-rung walk must
    //! fall up to the tier width), plus ordinary anchors.

    #[cfg(feature = "d307")]
    const ALL_MODES: [crate::support::rounding::RoundingMode; 8] = [
        crate::support::rounding::RoundingMode::HalfToEven,
        crate::support::rounding::RoundingMode::HalfAwayFromZero,
        crate::support::rounding::RoundingMode::HalfTowardZero,
        crate::support::rounding::RoundingMode::Trunc,
        crate::support::rounding::RoundingMode::Floor,
        crate::support::rounding::RoundingMode::Ceiling,
        crate::support::rounding::RoundingMode::AwayFromZero,
        crate::support::rounding::RoundingMode::ZeroFiveUp,
    ];

    #[test]
    #[cfg(feature = "d307")]
    fn d307_s153_tang_rung_matches_tier() {
        type Core = crate::types::widths::wide_trig_d307::Core;
        let one = crate::int::types::Int::<16>::from_i128(10i128).pow(153);
        let tiny = crate::int::types::Int::<16>::from_i128(3 * 10i128.pow(35));
        // 1 ± 3·10^-118 (the near-1 quadratic-term band), plus ordinary 2.0
        // and 0.5 anchors.
        let half = one / crate::int::types::Int::<16>::from_i128(2); // 0.5 (= 5·10^152)
        let raws = [one + tiny, one - tiny, one + one, half];
        for raw in raws {
            for mode in ALL_MODES {
                assert_eq!(
                    super::tang_at_rung::<Core, 153, 10, 400, true, false>(raw, mode),
                    crate::algos::ln::ln_tang::ln_tang::<Core, 153, 10, 400, true, false>(raw, mode),
                    "ln raw={raw:?} mode {mode:?}"
                );
            }
        }
    }
}
