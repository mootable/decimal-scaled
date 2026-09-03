// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Square-root policy — the per-`(N, SCALE)` algorithm matcher.
//!
//! `D<Int<N>, SCALE>::sqrt_strict_with(mode)` delegates directly to the
//! one shared [`dispatch`] generic function. `dispatch` follows the
//! canonical policy shape (see `docs/ARCHITECTURE.md` → "Policy file
//! structure"):
//!
//! 1. an [`Algorithm`] enum — the real square-root algorithms, no
//!    `Default` variant;
//! 2. a [`Select`] verdict — a settled algorithm or "the value decides"
//!    (sqrt has no value split, so `ByValue` is never returned);
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
//! The `Newton` kernel forms the radicand `raw · 10^SCALE`, which spans up
//! to `2N` limbs. Rather than thread a work *type* `Int<2N>` (unnameable
//! from `N` on stable), `sqrt_newton` does that arithmetic directly in
//! limbs and calls the int layer's width-agnostic slice `isqrt`. So the
//! dispatch carries no work-width parameter and the policy stays a pure
//! `(N, SCALE)` matcher.

use crate::algos::sqrt;
use crate::int::types::traits::BigInt;
use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── 1. the real square-root algorithms — NAMED, no `Default` ──────────

/// The square-root algorithms this policy chooses between. Variants are
/// the CamelCase of each kernel's name minus the `sqrt_` function prefix
/// (`sqrt_newton` → `Newton`, …) — strict 1:1 with the kernel fns.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    /// [`sqrt::sqrt_newton::sqrt_newton`] — Newton `isqrt` over a work
    /// width `W` covering `raw · 10^SCALE`. The generic default.
    Newton,
    /// [`sqrt::sqrt_mg_divide::sqrt_mg_divide`] — hand-tuned 256-bit
    /// isqrt for the `Int<2>` storage (D38, and D18 widened to it).
    MgDivide,
    /// [`sqrt::sqrt_newton_with_table_seed::sqrt_newton_with_table_seed`]
    /// — `f64`-seeded narrow-work bespoke for the `(D57, 20)` cell, kept as
    /// an explicit benchmarkable reference seam. Superseded by
    /// [`Self::Native`] (which seeds Newton in a tight `Int<W>` instead of
    /// re-entering the int `isqrt` policy's build-max slice) and not
    /// selected by `select`.
    #[allow(dead_code)]
    NewtonWithTableSeed,
    /// [`sqrt::sqrt_native::sqrt_native`] — `f64`-seeded Newton run directly
    /// in a tight, concrete `Int<W>` with `W = 2N` (chosen per tier in the
    /// dispatch arm to cover `mag · 10^SCALE` at any valid scale), rather
    /// than through the width-agnostic int `isqrt` slice.
    ///
    /// **KEPT but UNROUTED.** This arm was routed by `N` at D57/D76 and at
    /// high scale for D115..D1232, on the strength of two slice costs that
    /// have since been deleted: the build-max `isqrt` scratch (now exact
    /// per-`N` `ComputeLimbs` buffers) and the linear `scale`-length ×10
    /// radicand-build loop (now ONE multiply against the baked `pow10_limbs`
    /// const-table entry — `algos/sqrt/sqrt_newton.rs:89`). Re-measured over
    /// the full surface (2026-09-03), the generic slice [`Self::Newton`] is
    /// faster at EVERY sampled cell of every tier `N >= 3` — 50 cells,
    /// margins 1.36x-3.22x, each clearing its own cell's measured noise
    /// figure — so `select` routes nothing here.
    ///
    /// The kernel is retained, not deleted, as a benchmarkable reference
    /// seam that stays golden-covered, so a future remap cannot ship a break
    /// through it. The dispatch arm's per-`N` work-width table is retained
    /// with it so the kernel stays reachable and compiled at every width.
    #[allow(dead_code)]
    Native,
    /// Schoolbook reference tag -- delegates to
    /// [`sqrt::sqrt_newton::sqrt_newton`], which uses the same
    /// slice-`isqrt`-based pipeline as `sqrt_newton`. Exists as an explicit
    /// benchmarkable seam; never selected by `select` in production.
    #[allow(dead_code)]
    Schoolbook,
}

// ── 2. the const verdict ──────────────────────────────────────────────

/// A settled algorithm, or "the value decides". `ByValue` is part of the
/// canonical shape for uniformity across functions; sqrt never returns it
/// (the choice is fully determined by `(N, SCALE)`).
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),
    #[allow(dead_code)]
    ByValue(fn(&Int<N>) -> Algorithm),
}

// ── 3. the matcher: const, keyed on `(N, SCALE)`, total over the key ──

/// Pick the square-root algorithm for storage limb count `N` and decimal
/// `SCALE`. Total over the key; the `_` arm is the generic `Newton`
/// default (a real algorithm — there is no synthetic default variant).
const fn select<const N: usize, const SCALE: u32>() -> Select<N> {
    match (N, SCALE) {
        // ── D18 / D38 (N = 1 / 2): the int-layer bypass EARNS its place,
        // ── at every scale AND every magnitude ─────────────────────────
        // `MgDivide` is the one square-root candidate that bypasses the int
        // layer (hand-written 256-bit `u128` arithmetic in
        // `algos::support::mg_divide`), so it belongs here only if it beats
        // the int-layer paths ACROSS the tier — never on the strength of a
        // single cell.
        //
        // Full-surface map (2026-09-03; interleaved min-of-rounds, with a
        // same-kernel control measuring each cell's own noise): `MgDivide`
        // is the fastest ELIGIBLE candidate at EVERY sampled scale of both
        // tiers — D18 s0/s4/s9/s13/s17 by 1.45x/1.49x/1.20x/1.23x/1.32x
        // against per-cell noise of 1.055x/1.020x/1.000x/1.037x/1.014x, and
        // D38 s0/s9/s19/s28/s37 by 1.96x/1.66x/2.03x/1.41x/1.59x against
        // noise 1.086x/1.051x/1.009x/1.041x/1.048x. Every margin clears its
        // own cell's noise figure and the winner is constant in scale, so
        // each arm covers its tier whole — there is no scale-fitted constant
        // here to go stale.
        //
        // Why no value gate at D38. The radicand `raw · 10^SCALE` fits
        // `u128` only while `raw <= u128::MAX / 10^SCALE`, and outside that
        // region `MgDivide` genuinely loses its hardware-`u128::isqrt` fast
        // path — the map's magnitude ladder confirms the step is REAL (at
        // s19: 110-128 ns inside the region, 344-531 ns outside). The step
        // does not justify a branch. Measured across that ladder
        // (`raw = 10^k`, k in {0,5,10,15,19,22,25,30,37}, at s0/s19/s37),
        // `MgDivide` is still the fastest arm at 26 of 27 points —
        // INCLUDING outside the region. The lone exception is s19 raw=1e22,
        // where the slice leads 1.14x; the ladder carries no noise control,
        // so that single point is not an established win region. A
        // `ByValue` split would buy a runtime branch on every D38 call, and
        // forfeit dead-arm elimination in every monomorphisation, to chase
        // it.
        //
        // This replaces a `ByValue` arm that sent the outside-region values
        // to `Native`. That target measured WRONG: `Native` is the SLOWEST
        // of the three arms at every outside-region ladder point (813-961 ns
        // at s19). The 8963 ns outside-region `MgDivide` reading the old
        // comment cited did not reproduce at any ladder point — the worst
        // taken anywhere in the sweep was 530.7 ns.
        //
        // Validity: the map's wall requires bit-identity with the generic
        // slice across the input spread — which includes `9·10^(S-1)`, far
        // outside the u128 region at D38 — times all eight rounding modes.
        // `MgDivide` is eligible at N = 1 and N = 2 at every scale.
        (1, _) | (2, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // ── D57 … D1232 (N >= 3): the generic slice, at every scale ────
        // The generic `Newton` kernel (the int layer's width-agnostic slice
        // `isqrt`) is the fastest eligible candidate at EVERY sampled cell
        // of all ten wide tiers — 50 cells, margins 1.36x-3.22x over
        // `Native`, the next DISTINCT kernel, against per-cell noise figures
        // of 1.002x-1.208x. The winning kernel is constant in scale at every
        // one of those widths, so no crossover exists to gate on and the arm
        // covers the whole region rather than a band.
        //
        // This replaces the blanket `(3, _) | (4, _) => Native` arms and
        // eight per-tier high-scale gates — (6,>=24) (8,>=32) (12,>=70)
        // (16,>=64) (24,>=96) (32,>=160) (48,>=260) (64,>=256). All eight
        // measured MISPLACED: the map probed each at gate-1 / gate / gate+1
        // and the slice wins on BOTH sides of every one by 1.88x-2.76x, so
        // there was no win region for them to gate into.
        //
        // Those arms rested on two slice costs that no longer exist. The
        // build-max `isqrt` scratch is gone — `sqrt_newton` threads exact
        // per-`N` `ComputeLimbs` buffers. The "linear `scale`-length ×10
        // radicand build" is gone — `algos/sqrt/sqrt_newton.rs:89` forms the
        // radicand with ONE multiply against the baked `pow10_limbs`
        // const-table entry. Accordingly the two figures the old comments
        // cited re-measure INVERTED at those same cells: native was claimed
        // ahead 1.22x at D57<20> and 1.13x at D76<20>; the slice leads
        // 1.68x and 1.67x.
        _ => Select::ByAlgorithm(Algorithm::Newton),
    }
}

// ── 4. the shared dispatch: resolve the verdict, then dispatch ────────

/// Shared square-root dispatch for storage `Int<N>`, decimal `SCALE`,
/// and Newton work width `W`. Negative / zero inputs saturate to
/// `Int::<N>::ZERO`.
///
/// `W` is the next-up work width for the `Newton` arm (`Int<2N>`),
/// supplied by the caller because `Int<2N>` is not computable from `N`
/// on stable. The `MgDivide` / `NewtonWithTableSeed` arms run at their
/// own fixed widths (`Int<2>` / `Int<4>`); the `resize_to` bridges are
/// no-ops at the `N` those arms are selected for and dead-arm-eliminated
/// at every other `N`.
#[inline]
#[must_use]
pub(crate) fn dispatch<const N: usize, const SCALE: u32>(raw: Int<N>, mode: RoundingMode) -> Int<N>
where
    Limbs<N>: ComputeLimbs,
{
    if raw <= Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(&raw),
    };
    match algo {
        Algorithm::Newton => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
        // D18 / D38: run on `Int<2>` storage, resize back to `Int<N>`.
        // (`resize_to` is identity at N==2 and a lossless widen at N==1.)
        Algorithm::MgDivide => {
            sqrt::sqrt_mg_divide::sqrt_mg_divide(raw.resize_to::<Int<2>>(), SCALE, mode)
                .resize_to::<Int<N>>()
        }
        // (D57, 20): the bespoke kernel works on `Int<3>` storage.
        Algorithm::NewtonWithTableSeed => {
            sqrt::sqrt_newton_with_table_seed::sqrt_newton_with_table_seed(
                raw.resize_to::<Int<3>>(),
                mode,
            )
            .resize_to::<Int<N>>()
        }
        // Native tight-`Int<W>` arm: pick the literal work width `W` for
        // this `(N, SCALE)` cell, then run Newton directly in `Int<W>`.
        // The `(N, SCALE)` match is const-foldable (both const generics),
        // so each monomorphisation keeps exactly one arm and the rest are
        // dead-arm-eliminated in release. The `_ => Newton` fallback never
        // fires for a cell `select` routed to `Native`.
        Algorithm::Native => match N {
            // All wide tiers run at the full-range work width `W = 2N`, which
            // covers `mag · 10^SCALE` for every valid SCALE of the tier (the
            // magnitude is ≤ 64N bits and `10^SCALE` adds ≤ 64N more at the
            // tier's max scale). `10^SCALE` folds at compile time via the
            // `const { … }` block. The `(N, SCALE)` cells routed here by
            // `select` all satisfy the per-N high-scale gate; the `_ => Newton`
            // fallback is dead for any cell `select` routes to `Native`.
            // Narrow tiers at the same full-range `W = 2N`: `mag · 10^SCALE`
            // needs `(64N-1) + ceil(SCALE·log2 10) <= 128N - 1` bits, i.e.
            // `SCALE <= 19.266·N`. With `MAX_SCALE = tier - 1` that is 17 <=
            // 19.27 at N=1 (7 bits spare) and 37 <= 38.53 at N=2 (5 spare) —
            // at least as much headroom as the already-routed N=3/4 cells.
            // Only N=2 is reachable (N=1 routes to `MgDivide` unconditionally);
            // the N=1 arm is present so the width table stays total.
            1 => sqrt::sqrt_native::sqrt_native::<N, 2>(raw, const { Int::<2>::TEN.pow(SCALE) }, mode),
            2 => sqrt::sqrt_native::sqrt_native::<N, 4>(raw, const { Int::<4>::TEN.pow(SCALE) }, mode),
            3 => sqrt::sqrt_native::sqrt_native::<N, 6>(raw, const { Int::<6>::TEN.pow(SCALE) }, mode),
            4 => sqrt::sqrt_native::sqrt_native::<N, 8>(raw, const { Int::<8>::TEN.pow(SCALE) }, mode),
            6 => sqrt::sqrt_native::sqrt_native::<N, 12>(raw, const { Int::<12>::TEN.pow(SCALE) }, mode),
            8 => sqrt::sqrt_native::sqrt_native::<N, 16>(raw, const { Int::<16>::TEN.pow(SCALE) }, mode),
            12 => sqrt::sqrt_native::sqrt_native::<N, 24>(raw, const { Int::<24>::TEN.pow(SCALE) }, mode),
            16 => sqrt::sqrt_native::sqrt_native::<N, 32>(raw, const { Int::<32>::TEN.pow(SCALE) }, mode),
            24 => sqrt::sqrt_native::sqrt_native::<N, 48>(raw, const { Int::<48>::TEN.pow(SCALE) }, mode),
            32 => sqrt::sqrt_native::sqrt_native::<N, 64>(raw, const { Int::<64>::TEN.pow(SCALE) }, mode),
            48 => sqrt::sqrt_native::sqrt_native::<N, 96>(raw, const { Int::<96>::TEN.pow(SCALE) }, mode),
            64 => sqrt::sqrt_native::sqrt_native::<N, 128>(raw, const { Int::<128>::TEN.pow(SCALE) }, mode),
            _ => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
        },
        Algorithm::Schoolbook => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
    }
}
