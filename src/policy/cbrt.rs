// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Cube-root policy — the per-`(N, SCALE)` algorithm matcher.
//!
//! `D<Int<N>, SCALE>::cbrt_strict_with(mode)` delegates directly to the
//! one shared [`dispatch`] generic function. `dispatch` follows the
//! canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"), mirroring [`crate::policy::sqrt`]:
//!
//! 1. an [`Algorithm`] enum — the real cube-root algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (cbrt has no value split, so `ByValue` is never returned);
//! 3. a `const fn` [`select`] keyed on `(N, SCALE)`, total over the key;
//! 4. dispatch via an inline `const { select::<N, SCALE>() }` block,
//!    then an **exhaustive** `match algo` — no `_`, no panic.
//!
//! Because `select` is `const` and keyed only on the const generics, the
//! `const { … }` block folds per monomorphisation and every unchosen arm
//! is dead-arm-eliminated in release: each concrete `D<Int<N>, SCALE>`
//! compiles to a direct call to one kernel, no runtime branch.
//!
//! # Work width
//!
//! The `Newton` kernel forms the radicand `|raw| · 10^(2·SCALE)`, which
//! spans up to `4N` limbs. Rather than thread a work *type* `Int<4N>`
//! (unnameable from `N` on stable), `cbrt_newton` does that arithmetic
//! directly in limbs and calls the int layer's width-agnostic slice
//! `icbrt`. So the dispatch carries no work-width parameter and the policy
//! stays a pure `(N, SCALE)` matcher.

use crate::algos::cbrt;
use crate::int::types::traits::BigInt;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real cube-root algorithms — NAMED, no `Default` ────────────

/// The cube-root algorithms this policy chooses between. Variants are
/// the CamelCase of each kernel's name minus the `cbrt_` function prefix
/// (`cbrt_newton` → `Newton`, …) — strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`cbrt::cbrt_newton::cbrt_newton`] — Newton `icbrt` over a work
    /// width `W` covering `|raw| · 10^(2·SCALE)`. The generic default.
    Newton,
    /// [`cbrt::cbrt_mg_divide::cbrt_mg_divide`] — hand-tuned 384-bit
    /// cube root for the `Int<2>` storage (D38, and D18 widened to it).
    MgDivide,
    /// [`cbrt::cbrt_native_fast_d57::cbrt_native_fast_a`] — `f64`-seeded
    /// Newton run directly in a tight, concrete `Int<W>` (the work width `W`
    /// is chosen per `(N, SCALE)` cell in the dispatch arm to just cover
    /// `mag · 10^(2·SCALE)`), rather than through the width-agnostic int
    /// `icbrt` policy, whose build-max scratch buffer churn dominated the
    /// small mid-scale radicands. The seed is the 0.4.4 **full-radicand**
    /// f64 cbrt seed (commit routing the Native cells to `cbrt_native_fast_a`):
    /// a tight seed that cuts the Newton divide count, vs the earlier shipped
    /// top-64-bits seed which over-shot ∛n by ~2.5× and regressed cbrt@D57/D76.
    /// Routed by `N` for D57/D76 (N = 3/4) at every scale, and for the wider
    /// tiers D115..D1232 (N = 6..64) at high scale only (`SCALE >= 8·N` — the
    /// empirical crossover where the tight `Int<3N>` overtakes the slice; the
    /// slice's linear `2·scale`-length ×10 radicand build dominates above it).
    /// Microbench (`root_kernel_ab`): 1.1–1.4× at the mid-scale cells; at the
    /// max-scale (S-1) bench-branch-compare cells 1.4–2.9× faster than the
    /// generic slice [`Self::Newton`]. Bit-identical to [`Self::Newton`]
    /// across all six modes (the rounding tail is shared); the seed falls back
    /// to the top-bits path past the f64 range.
    ///
    /// Gated with the kernel: each routed `(N, SCALE)` cell only exists
    /// when its tier is compiled in, so the variant, its `select` arms,
    /// and its dispatch arms are gated together (the policy stays
    /// exhaustive in both configs — see `docs/ARCHITECTURE.md`
    /// "Feature-flagging a variation").
    #[cfg(any(feature = "d57", feature = "wide"))]
    Native,
    /// [`cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed`]
    /// — the prior `Int<6>` + int-`icbrt` arm for `(D57, 20)`. Superseded
    /// by [`Self::Native`] (the int-`icbrt` scratch churn made it ~2×
    /// slower); kept as an explicit benchmarkable reference seam, never
    /// selected by `select`.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[allow(dead_code)]
    NewtonWithTableSeed,
    /// Schoolbook reference tag -- delegates to
    /// [`cbrt::cbrt_schoolbook::cbrt_schoolbook`], which uses the same
    /// `W::icbrt`-based pipeline as `cbrt_newton`. Exists as an explicit
    /// benchmarkable seam; never selected by `select` in production.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". `ByValue` is part of the
/// canonical shape for uniformity across functions; cbrt never returns it
/// (the choice is fully determined by `(N, SCALE)`).
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the cube-root algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; the `_` arm is the generic `Newton`
/// default (a real algorithm — there is no synthetic default variant).
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // D18 (`Int<1>`) — widened to `Int<2>` storage in the dispatch
        // and run through the hand-tuned 384-bit cube root there.
        (1, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // D38 (`Int<2>`) — hand-tuned 384-bit cube root.
        (2, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // D57 / D76 (N = 3 / 4) — bespoke f64-seeded Newton in a tight,
        // concrete `Int<W>` with `W = 3N` (covering `mag · 10^(2·SCALE)`:
        // the magnitude is ≤ 64N bits and `10^(2·SCALE)` adds ≤ 128N more
        // for SCALE ≤ the tier's digit capacity, so 192N bits = 3N limbs
        // suffice at any valid scale). Routed by `N` (all scales) because
        // the build-max slice scratch the generic `Newton` kernel zeroes per
        // Newton iteration dominates these radicands even at the full-range
        // `W = 3N`. Microbench (`root_kernel_ab`, at the routed `W = 3N`):
        // native beats the generic slice 1.42× (D57<20>, via `fast_a`) and
        // 1.15× (D76<20>). Bit-identical to the oracle-gated `cbrt_native`
        // across all six modes and signs (kernel test gate).
        //
        // The wider mid tiers (D115/D153, N = 6/8) are NOT routed by `N`:
        // at the full-range `W = 3N` (18/24 limbs) the per-iteration Knuth
        // divide outweighs the slice's scratch churn at the benchmarked scales, so a
        // blanket `W = 3N` would regress them (rule 6 — size each width
        // exactly). They keep only their high-scale `Native` cells below.
        //
        // Gated with the kernel; each tier's cells are unreachable when the
        // tier isn't built (the policy stays exhaustive in both configs).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, _) // D57, W=9
        | (4, _) // D76, W=12
        => Select::ByAlgorithm(Algorithm::Native),
        // High-scale Native for the wider tiers (D115..D1232, N = 6..64),
        // each at the full-range work width `W = 3N` (covers `mag · 10^(2·
        // SCALE)` at any valid scale). As for sqrt the crossover is scale-
        // dependent: the tight `Int<3N>` Native pays full-`W` work on a small
        // radicand at low/mid scale (slice wins there), but at high scale the
        // slice's *linear* `2·scale`-length ×10 radicand build plus its
        // build-max `icbrt` scratch dominate and the tight `Int<3N>` Native
        // (radicand via a const-folded `pow`) wins. `root_kernel_ab` localizes
        // the cbrt crossover near `SCALE ≈ 7·N`; the conservative `SCALE >=
        // 8·N` gate sits just inside the win region (every routed cell wins,
        // none below is regressed). At the benchmarked max-scale (S-1) cells this
        // recovers 1.4–2.9× over the slice (bit-identical, all six modes).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (6, s) if s >= 48 => Select::ByAlgorithm(Algorithm::Native), // D115, W=18
        #[cfg(any(feature = "d57", feature = "wide"))]
        (8, s) if s >= 64 => Select::ByAlgorithm(Algorithm::Native), // D153, W=24
        #[cfg(any(feature = "d57", feature = "wide"))]
        (12, s) if s >= 96 => Select::ByAlgorithm(Algorithm::Native), // D230, W=36
        #[cfg(any(feature = "d57", feature = "wide"))]
        (16, s) if s >= 128 => Select::ByAlgorithm(Algorithm::Native), // D307, W=48
        #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
        (24, s) if s >= 192 => Select::ByAlgorithm(Algorithm::Native), // D462, W=72
        #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
        (32, s) if s >= 256 => Select::ByAlgorithm(Algorithm::Native), // D616, W=96
        #[cfg(feature = "xx-wide")]
        (48, s) if s >= 384 => Select::ByAlgorithm(Algorithm::Native), // D924, W=144
        #[cfg(feature = "xx-wide")]
        (64, s) if s >= 512 => Select::ByAlgorithm(Algorithm::Native), // D1232, W=192
        // Everything else (wider tiers at low/mid scales) — generic Newton
        // over the int layer's width-agnostic slice `icbrt`.
        _ => Select::ByAlgorithm(Algorithm::Newton),
    }
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Shared cube-root dispatch for storage `Int<N>`, decimal `SCALE`, and
/// Newton work width `W`. Zero input returns `Int::<N>::ZERO`; the sign
/// of a non-zero input is preserved.
///
/// `W` is the double-bumped work width for the `Newton` arm
/// (`Int<4N>`), supplied by the caller because `Int<4N>` is not
/// computable from `N` on stable. The `MgDivide` / `NewtonWithTableSeed`
/// arms run at their own fixed widths (`Int<2>` / `Int<3>`); the
/// `resize_to` bridges are no-ops at the `N` those arms are selected for
/// and dead-arm-eliminated at every other `N`.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if raw == Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
    };
    match algo {
        Algorithm::Newton => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
        // D18 / D38: run on `Int<2>` storage, resize back to `Int<N>`.
        // (`resize_to` is identity at N==2 and a lossless widen at N==1.)
        Algorithm::MgDivide => {
            cbrt::cbrt_mg_divide::cbrt_mg_divide(raw.resize_to::<Int<2>>(), SCALE, mode)
                .resize_to::<Int<N>>()
        }
        // Native tight-`Int<W>` arm: pick the literal work width `W` for
        // this `(N, SCALE)` cell, then run Newton directly in `Int<W>`.
        // The `(N, SCALE)` match is const-foldable (both are const
        // generics), so each monomorphisation keeps exactly one arm and
        // the rest are dead-arm-eliminated in release. The `_ => Newton`
        // fallback never fires for a cell `select` routed to `Native`.
        #[cfg(any(feature = "d57", feature = "wide"))]
        // Native cells use the 0.4.4-style full-radicand f64 cbrt seed
        // (`cbrt_native_fast_a`): a tight seed (vs the shipped top-64-bits
        // seed that over-shoots ∛n by ~2.5×) cuts the Newton divide count,
        // recovering the cbrt_D57/D76 regression. Bit-identical (the rounding
        // tail is shared); falls back to the top-bits seed past the f64 range.
        Algorithm::Native => match N {
            // All wide tiers run at the full-range work width `W = 3N`, which
            // covers `mag · 10^(2·SCALE)` for every valid SCALE of the tier
            // (the magnitude is ≤ 64N bits and `10^(2·SCALE)` adds ≤ 128N more
            // at the tier's max scale, so 192N bits = 3N limbs suffice).
            // `10^(2·SCALE)` folds at compile time. The `_ => Newton` fallback
            // is dead for any cell `select` routes to `Native`.
            3 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 9>(raw, const { Int::<9>::TEN.pow(2 * SCALE) }, mode),
            4 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 12>(raw, const { Int::<12>::TEN.pow(2 * SCALE) }, mode),
            6 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 18>(raw, const { Int::<18>::TEN.pow(2 * SCALE) }, mode),
            8 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 24>(raw, const { Int::<24>::TEN.pow(2 * SCALE) }, mode),
            12 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 36>(raw, const { Int::<36>::TEN.pow(2 * SCALE) }, mode),
            16 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 48>(raw, const { Int::<48>::TEN.pow(2 * SCALE) }, mode),
            #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
            24 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 72>(raw, const { Int::<72>::TEN.pow(2 * SCALE) }, mode),
            #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
            32 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 96>(raw, const { Int::<96>::TEN.pow(2 * SCALE) }, mode),
            #[cfg(feature = "xx-wide")]
            48 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 144>(raw, const { Int::<144>::TEN.pow(2 * SCALE) }, mode),
            #[cfg(feature = "xx-wide")]
            64 => cbrt::cbrt_native_fast_d57::cbrt_native_fast_a::<N, 192>(raw, const { Int::<192>::TEN.pow(2 * SCALE) }, mode),
            _ => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
        },
        #[cfg(any(feature = "d57", feature = "wide"))]
        Algorithm::NewtonWithTableSeed => {
            cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed(
                raw.resize_to::<Int<3>>(),
                mode,
            )
            .resize_to::<Int<N>>()
        }
        Algorithm::Schoolbook => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
    }
}
