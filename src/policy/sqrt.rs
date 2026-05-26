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
use crate::int::types::compute_int::ComputeInt;
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
    /// re-entering the int `isqrt` policy's build-max slice) and no longer
    /// selected by `select`.
    ///
    /// Gated with the kernel: it only exists when D57 is compiled in, so the
    /// variant and its dispatch arm are gated together (the policy stays
    /// exhaustive in both configs — see `docs/ARCHITECTURE.md`
    /// "Feature-flagging a variation").
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[allow(dead_code)]
    NewtonWithTableSeed,
    /// [`sqrt::sqrt_native::sqrt_native`] — `f64`-seeded Newton run directly
    /// in a tight, concrete `Int<W>` with `W = 2N` (chosen per tier in the
    /// dispatch arm to cover `mag · 10^SCALE` at any valid scale), rather
    /// than through the width-agnostic int `isqrt` slice, whose linear
    /// `scale`-length ×10 radicand-build loop and build-max scratch buffer
    /// dominate at high scale. Routed by `N` for the mid-wide tiers D57/D76
    /// (N = 3/4) at every scale, and for the wider tiers D115..D1232
    /// (N = 6..64) at high scale only (`SCALE >= 4·N` — the empirical
    /// crossover where the tight `Int<2N>` overtakes the slice). Microbench
    /// (`root_kernel_ab`): 1.1–1.97× faster at the mid-scale cells; at the
    /// max-scale (S-1) bench-branch-compare cells 1.8–5.8× faster than the
    /// generic slice [`Self::Newton`]. Bit-identical to [`Self::Newton`]
    /// across all six modes.
    ///
    /// Gated with the kernel: each tier only exists when compiled in, so the
    /// variant, its `select` arms, and its dispatch arms are gated together
    /// (the policy stays exhaustive in both configs).
    #[cfg(any(feature = "d57", feature = "wide"))]
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
        // D18 (`Int<1>`) — widened to `Int<2>` storage in the dispatch
        // and run through the hand-tuned 256-bit isqrt there.
        (1, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // D38 (`Int<2>`) — hand-tuned 256-bit isqrt.
        (2, _) => Select::ByAlgorithm(Algorithm::MgDivide),
        // D57 / D76 (N = 3 / 4) — bespoke f64-seeded Newton in a tight,
        // concrete `Int<W>` with `W = 2N` (covering `mag · 10^SCALE` at any
        // valid scale: the storage magnitude is ≤ 64N bits and `10^SCALE`
        // adds ≤ 64N more for SCALE ≤ the tier's digit capacity). Routed by
        // `N` (all scales) because the build-max slice scratch the generic
        // `Newton` kernel zeroes per Newton iteration dominates these
        // radicands even at the full-range `W = 2N`. Microbench
        // (`root_kernel_ab`, at the routed `W = 2N`): native beats the
        // generic slice 1.22× (D57<20>) and 1.13× (D76<20>). Bit-identical
        // to `Newton` across all six modes (kernel test gate).
        //
        // The wider mid tiers (D115/D153, N = 6/8) are NOT routed by `N`:
        // at the full-range `W = 2N` (12/16 limbs) the per-iteration Knuth
        // divide outweighs the slice's scratch churn at the bbc scales, so a
        // blanket `W = 2N` would regress them (rule 6 — size each width
        // exactly). They keep only their high-scale `Native` cells below,
        // where the radicand is genuinely wide.
        //
        // Gated with the kernel: each tier's cells only exist when the tier
        // is compiled in (the policy stays exhaustive in both configs).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, _) // D57, W=6
        | (4, _) // D76, W=8
        => Select::ByAlgorithm(Algorithm::Native),
        // High-scale Native for the wider tiers (D115..D1232, N = 6..64),
        // each at the full-range work width `W = 2N` (covers `mag · 10^SCALE`
        // at any valid scale). The crossover is scale-dependent: at low/mid
        // scale the radicand is small and the tight `Int<2N>` Native pays
        // full-`W` Knuth divides while the slice works on the actual
        // significant length, so the slice wins; at high scale the slice's
        // *linear* `scale`-length ×10 radicand-build loop plus its build-max
        // `isqrt` scratch dominate, and the tight `Int<2N>` Native (radicand
        // built via a const-folded `pow`, root in an exactly-sized integer)
        // wins decisively. `root_kernel_ab` localizes the crossover near
        // `SCALE ≈ 3.5·N`; the conservative `SCALE >= 4·N` gate sits at the
        // tie/win boundary, so every cell it routes to Native is a win and
        // none below it is regressed. At the bbc max-scale (S-1) cells this
        // recovers 1.8–5.8× over the slice (bit-identical, all six modes).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (6, s) if s >= 24 => Select::ByAlgorithm(Algorithm::Native), // D115, W=12
        #[cfg(any(feature = "d57", feature = "wide"))]
        (8, s) if s >= 32 => Select::ByAlgorithm(Algorithm::Native), // D153, W=16
        #[cfg(any(feature = "d57", feature = "wide"))]
        (12, s) if s >= 48 => Select::ByAlgorithm(Algorithm::Native), // D230, W=24
        #[cfg(any(feature = "d57", feature = "wide"))]
        (16, s) if s >= 64 => Select::ByAlgorithm(Algorithm::Native), // D307, W=32
        #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
        (24, s) if s >= 96 => Select::ByAlgorithm(Algorithm::Native), // D462, W=48
        #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
        (32, s) if s >= 128 => Select::ByAlgorithm(Algorithm::Native), // D616, W=64
        #[cfg(feature = "xx-wide")]
        (48, s) if s >= 192 => Select::ByAlgorithm(Algorithm::Native), // D924, W=96
        #[cfg(feature = "xx-wide")]
        (64, s) if s >= 256 => Select::ByAlgorithm(Algorithm::Native), // D1232, W=128
        // Everything else (wider tiers at low/mid scales) — generic Newton
        // over the int layer's width-agnostic slice `isqrt`.
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
    Int<N>: ComputeInt,
{
    if raw <= Int::<N>::ZERO {
        return Int::<N>::ZERO;
    }
    let algo = match const { select::<N, SCALE>() } {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f) => f(&raw),
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
        #[cfg(any(feature = "d57", feature = "wide"))]
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
        #[cfg(any(feature = "d57", feature = "wide"))]
        Algorithm::Native => match N {
            // All wide tiers run at the full-range work width `W = 2N`, which
            // covers `mag · 10^SCALE` for every valid SCALE of the tier (the
            // magnitude is ≤ 64N bits and `10^SCALE` adds ≤ 64N more at the
            // tier's max scale). `10^SCALE` folds at compile time via the
            // `const { … }` block. The `(N, SCALE)` cells routed here by
            // `select` all satisfy the per-N high-scale gate; the `_ => Newton`
            // fallback is dead for any cell `select` routes to `Native`.
            3 => sqrt::sqrt_native::sqrt_native::<N, 6>(raw, const { Int::<6>::TEN.pow(SCALE) }, mode),
            4 => sqrt::sqrt_native::sqrt_native::<N, 8>(raw, const { Int::<8>::TEN.pow(SCALE) }, mode),
            6 => sqrt::sqrt_native::sqrt_native::<N, 12>(raw, const { Int::<12>::TEN.pow(SCALE) }, mode),
            8 => sqrt::sqrt_native::sqrt_native::<N, 16>(raw, const { Int::<16>::TEN.pow(SCALE) }, mode),
            12 => sqrt::sqrt_native::sqrt_native::<N, 24>(raw, const { Int::<24>::TEN.pow(SCALE) }, mode),
            16 => sqrt::sqrt_native::sqrt_native::<N, 32>(raw, const { Int::<32>::TEN.pow(SCALE) }, mode),
            #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
            24 => sqrt::sqrt_native::sqrt_native::<N, 48>(raw, const { Int::<48>::TEN.pow(SCALE) }, mode),
            #[cfg(any(feature = "x-wide", feature = "xx-wide"))]
            32 => sqrt::sqrt_native::sqrt_native::<N, 64>(raw, const { Int::<64>::TEN.pow(SCALE) }, mode),
            #[cfg(feature = "xx-wide")]
            48 => sqrt::sqrt_native::sqrt_native::<N, 96>(raw, const { Int::<96>::TEN.pow(SCALE) }, mode),
            #[cfg(feature = "xx-wide")]
            64 => sqrt::sqrt_native::sqrt_native::<N, 128>(raw, const { Int::<128>::TEN.pow(SCALE) }, mode),
            _ => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
        },
        Algorithm::Schoolbook => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
    }
}
