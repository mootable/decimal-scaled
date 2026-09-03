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
    ///
    /// Routed at `N == 1 | 2`, every scale. This is the only cube-root
    /// candidate that bypasses the int layer, and it now earns that bypass.
    /// It previously did not, but the kernel it was judged on no longer
    /// exists: its `div_384_by_256` was an unconditional 384-iteration
    /// bit-serial shift-subtract with no fast path at any scale or value,
    /// and it now routes through the int layer's divisor-shape matcher
    /// (`div_rem_via_int_layer`, `algos/support/mg_divide.rs:1018`) —
    /// word-serial via Möller-Granlund or Knuth. The "60-179x slower than
    /// both int-layer paths" figure that unrouted this arm was measured
    /// against that deleted loop. See [`select`] for the re-measured
    /// per-cell margins.
    MgDivide,
    /// [`cbrt::cbrt_native_fast::cbrt_native_fast_a`] — `f64`-seeded
    /// Newton run directly in a tight, concrete `Int<W>` (the work width `W`
    /// is chosen per `(N, SCALE)` cell in the dispatch arm to just cover
    /// `mag · 10^(2·SCALE)`), rather than through the width-agnostic int
    /// `icbrt` policy, whose build-max scratch buffer churn dominated the
    /// small mid-scale radicands. The seed is the **full-radicand**
    /// f64 cbrt seed:
    /// a tight seed that cuts the Newton divide count, vs the
    /// top-64-bits seed which over-shoots ∛n by ~2.5×.
    /// Routed at exactly ONE region of the surface: D57 (N = 3) above the
    /// scale-22..25 crossover — see [`select`], which also records why the
    /// band above it is held as a measured TIE rather than a win. Every
    /// other cell the arm used to hold (D76 at all scales, and D115..D1232
    /// at high scale under a `SCALE >= 8·N` gate) re-measured as a slice
    /// win on the full-surface map and is routed to [`Self::Newton`]; the
    /// per-`N` work-width table in `dispatch` is kept whole regardless, so
    /// the kernel stays reachable and compiled at every width.
    ///
    /// Bit-identical to [`Self::Newton`] across every mode (the rounding
    /// tail is shared); the seed falls back to the top-bits path past the
    /// f64 range.
    ///
    /// NOT feature-gated: the variant and its `select`/`dispatch` arms are
    /// always present so routing is feature-INDEPENDENT (a build with a
    /// single wide tier routes that tier exactly as a full build does). The
    /// arms are reached only at the `N` their tier instantiates, so they are
    /// dead-arm-eliminated in any build without that tier; `#[allow(dead_code)]`
    /// covers the narrow-only build where the variant is never constructed.
    #[allow(dead_code)]
    Native,
    /// [`cbrt::cbrt_newton_with_table_seed::cbrt_newton_with_table_seed`]
    /// — the prior `Int<6>` + int-`icbrt` arm for `(D57, 20)`. Superseded
    /// by [`Self::Native`] (the int-`icbrt` scratch churn made it ~2×
    /// slower); kept as an explicit benchmarkable reference seam, never
    /// selected by `select`.
    #[allow(dead_code)]
    NewtonWithTableSeed,
    /// Schoolbook reference tag -- delegates to
    /// [`cbrt::cbrt_newton::cbrt_newton`], the same `W::icbrt`-based
    /// pipeline [`Self::Newton`] uses. (There is no `cbrt_schoolbook`
    /// kernel; an earlier doc named one that has never existed in
    /// `src/algos/cbrt/`.) Exists as an explicit benchmarkable seam; never
    /// selected by `select` in production.
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
        // ── D18 / D38 (N = 1 / 2): the int-layer bypass, re-earned ─────
        // `MgDivide` is the only cube-root candidate that BYPASSES the int
        // layer, so it has to EARN that bypass with a win across the tier.
        // These cells were routed away from it because it did not: its
        // `div_384_by_256` was an unconditional 384-iteration bit-serial
        // shift-subtract with no fast path at any scale or value, and it
        // measured 60-179x slower than both int-layer paths.
        //
        // That loop is GONE. `algos/support/mg_divide.rs:1018` now routes
        // the divide through the int layer's divisor-shape matcher
        // (`div_rem_via_int_layer`): a single-limb divisor takes the
        // hardware Möller-Granlund engine, a multi-limb one takes Knuth —
        // word-serial either way. The figure that unrouted this arm was
        // measured against a kernel that no longer exists.
        //
        // Re-measured over the full surface (2026-09-03; interleaved
        // min-of-rounds, with a same-kernel control measuring each cell's
        // own noise), `MgDivide` is the fastest ELIGIBLE candidate at EVERY
        // sampled scale of both tiers — D18 s0/s4/s9/s13/s17 by
        // 1.13x/1.05x/1.08x/1.02x/1.11x against per-cell noise of
        // 1.057x/1.039x/1.045x/1.026x/1.031x, and D38 s0/s9/s19/s28/s37 by
        // 1.30x/1.46x/1.67x/1.44x/1.69x against noise
        // 1.001x/1.062x/1.002x/1.005x/1.024x.
        //
        // The D38 margins clear their noise comfortably; the D18 margins are
        // genuinely small, several sitting at or near their cell's figure.
        // So D18 was not left on the 5-point grid: it was re-run at 11
        // scales x 9 rounds, `MgDivide` won all 11, and the ordering held
        // across every scale and both passes. (The arm it beats at D18 is
        // `Native`, never the slice.) The winner is constant in scale across
        // both tiers, so each arm covers its tier whole — no scale-fitted
        // constant is left here to go stale, as the removed `SCALE >= 9`
        // gate once did.
        //
        // Validity: the map's wall requires bit-identity with the generic
        // slice across the input spread x all eight rounding modes;
        // `MgDivide` is eligible at N = 1 and N = 2 at every scale.
        (1, _) | (2, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // ── D57 (N = 3): the ONE genuine crossover on the whole surface ─
        // This is the only width, for either root, where the winning kernel
        // changes with scale. The map bisected it at 12 scales x 9 rounds.
        //
        // Below the crossover the generic slice wins DECISIVELY, decaying
        // monotonically: s0 2.02x, s7 1.26x, s14 1.13x, s21 1.09x, against
        // per-cell noise of 1.025x/1.001x/1.077x/1.087x. The bisection
        // localises the crossover to the interval scale 22-25 — s21 is the
        // last measured decisive slice win, s25 the first `fast_a` win that
        // clears its own cell's noise (1.06x vs 1.007x).
        //
        // The boundary sits at the LOW edge of that bisected interval, so
        // every cell this arm moves onto the slice is one the map actually
        // measured as a slice win. Scales 22-24 lie inside the interval and
        // were not sampled; they stay with the incumbent below rather than
        // being switched on no measurement.
        (3, 0..=21) => Select::ByAlgorithm(Algorithm::Newton),
        // Above the crossover the map records a TIE, not a win, so this band
        // is deliberately UNCHANGED from what the tier routed before the
        // remap. `fast_a` (the kernel `Native` dispatches to) is nominally
        // ahead at 7 of the 8 points from s25 up, but at s28, s31, s35, s42
        // and s49 the margin is AT or BELOW that cell's own noise figure;
        // only s25, s53 and s56 clear it, by 1.06x-1.11x. The per-input
        // detail shows why the band is soft: `fast_a` leads on the small and
        // mid magnitudes (`1.0`, `2.0`) by up to 1.23x but LOSES to the
        // slice on the tier's large ones (`9·10^(S-1)`). A tie justifies no
        // routing change in either direction, so the map reports this
        // boundary as deliberately un-localised and the band keeps `Native`.
        (3, _) => Select::ByAlgorithm(Algorithm::Native),
        // ── D76 … D1232 (N >= 4): the generic slice, at every scale ────
        // The generic `Newton` kernel (the int layer's width-agnostic slice
        // `icbrt`) is the fastest eligible candidate at EVERY sampled cell
        // of all nine remaining wide tiers — 45 cells, margins 1.06x-4.88x
        // over the best of the three native-family kernels, against per-cell
        // noise figures of 1.003x-1.128x. The winning kernel is constant in
        // scale at every one of those widths, so no crossover exists to gate
        // on.
        //
        // This replaces the blanket `(4, _) => Native` arm and eight
        // per-tier high-scale gates — (6,>=48) (8,>=64) (12,>=96)
        // (16,>=128) (24,>=192) (32,>=256) (48,>=384) (64,>=512). Each was
        // probed at gate-1 / gate / gate+1 and the slice wins on BOTH sides
        // of all eight by 1.72x-2.85x: there was no win region for them to
        // gate into. They had been calibrated at max scale (S-1) only and
        // extrapolated down over the rest of the tier.
        //
        // The cost that justified them is gone. The slice's build-max
        // `icbrt` scratch — six buffers sized from `MAX_WORK_N`, 13,824
        // bytes zero-initialised per call in a full build, paid in full even
        // by a one-limb radicand — was replaced by exact per-`N`
        // `ComputeLimbs` buffers threaded through the `icbrt_newton_into`
        // door (`algos/cbrt/cbrt_newton.rs:150-170`), and the linear
        // `2·scale`-length ×10 radicand build by ONE multiply against the
        // baked const-table entry. Accordingly the `D76<20>` figure the old
        // comment cited re-measures INVERTED: native was claimed ahead
        // 1.15x; the slice leads 1.31x.
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
        Select::ByAlgorithm(algorithm) => algorithm,
        Select::ByValue(choose) => choose(&raw),
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
        // Native cells use the full-radicand f64 cbrt seed
        // (`cbrt_native_fast_a`): a tight seed (vs the top-64-bits
        // seed that over-shoots ∛n by ~2.5×) cuts the Newton divide count.
        // Bit-identical (the rounding
        // tail is shared); falls back to the top-bits seed past the f64 range.
        Algorithm::Native => match N {
            // All wide tiers run at the full-range work width `W = 3N`, which
            // covers `mag · 10^(2·SCALE)` for every valid SCALE of the tier
            // (the magnitude is ≤ 64N bits and `10^(2·SCALE)` adds ≤ 128N more
            // at the tier's max scale, so 192N bits = 3N limbs suffice).
            // `10^(2·SCALE)` folds at compile time. The `_ => Newton` fallback
            // is dead for any cell `select` routes to `Native`.
            // Narrow tiers at the same full-range `W = 3N`: `mag · 10^(2·SCALE)`
            // needs `(64N-1) + ceil(2·SCALE·log2 10) <= 192N - 1` bits, i.e.
            // `SCALE <= 19.266·N`. With `MAX_SCALE = tier - 1` that is 17 <=
            // 19.27 at N=1 (15 bits spare) and 37 <= 38.53 at N=2 (10 spare) —
            // more headroom than the already-routed N=3/4 cells have.
            1 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 3>(raw, const { Int::<3>::TEN.pow(2 * SCALE) }, mode),
            2 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 6>(raw, const { Int::<6>::TEN.pow(2 * SCALE) }, mode),
            3 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 9>(raw, const { Int::<9>::TEN.pow(2 * SCALE) }, mode),
            4 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 12>(raw, const { Int::<12>::TEN.pow(2 * SCALE) }, mode),
            6 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 18>(raw, const { Int::<18>::TEN.pow(2 * SCALE) }, mode),
            8 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 24>(raw, const { Int::<24>::TEN.pow(2 * SCALE) }, mode),
            12 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 36>(raw, const { Int::<36>::TEN.pow(2 * SCALE) }, mode),
            16 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 48>(raw, const { Int::<48>::TEN.pow(2 * SCALE) }, mode),
            24 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 72>(raw, const { Int::<72>::TEN.pow(2 * SCALE) }, mode),
            32 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 96>(raw, const { Int::<96>::TEN.pow(2 * SCALE) }, mode),
            48 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 144>(raw, const { Int::<144>::TEN.pow(2 * SCALE) }, mode),
            64 => cbrt::cbrt_native_fast::cbrt_native_fast_a::<N, 192>(raw, const { Int::<192>::TEN.pow(2 * SCALE) }, mode),
            _ => cbrt::cbrt_newton::cbrt_newton::<N>(raw, SCALE, mode),
        },
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
