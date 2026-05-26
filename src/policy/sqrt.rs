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
    /// than through the width-agnostic int `isqrt` slice, whose build-max
    /// scratch buffer churn dominated the mid-scale radicands of the wide
    /// tiers. Routed by `N` for the mid-wide tiers D57/D76/D115/D153
    /// (N = 3/4/6/8), every scale. Microbench (`root_kernel_ab`, bbc
    /// scales): 1.11–1.97× faster than the generic slice [`Self::Newton`].
    /// Bit-identical to [`Self::Newton`] across all six modes.
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
        // High-scale Native cells for the wider tiers, each with its own
        // exact `W` sized to the cell (no build-max blanket). These were the
        // historically benched-win golden-table scales.
        #[cfg(any(feature = "d57", feature = "wide"))]
        (6, 57) // D115<57>, W=9
        | (8, 75) // D153<75>, W=12
        | (8, 76) // D153<76>, W=12
        | (12, 115) // D230<115>, W=19
        | (16, 150) // D307<150>, W=24
        => Select::ByAlgorithm(Algorithm::Native),
        // Everything else (wider tiers at other scales) — generic Newton
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
        Algorithm::Native => match (N, SCALE) {
            // D57 / D76 routed by N (all scales), W = 2N.
            (3, _) => sqrt::sqrt_native::sqrt_native::<N, 6>(raw, const { Int::<6>::TEN.pow(SCALE) }, mode),
            (4, _) => sqrt::sqrt_native::sqrt_native::<N, 8>(raw, const { Int::<8>::TEN.pow(SCALE) }, mode),
            // High-scale cells for the wider tiers, each with its exact W.
            (6, 57) => sqrt::sqrt_native::sqrt_native::<N, 9>(raw, const { Int::<9>::TEN.pow(SCALE) }, mode),
            (8, 75) | (8, 76) => sqrt::sqrt_native::sqrt_native::<N, 12>(raw, const { Int::<12>::TEN.pow(SCALE) }, mode),
            (12, 115) => sqrt::sqrt_native::sqrt_native::<N, 19>(raw, const { Int::<19>::TEN.pow(SCALE) }, mode),
            (16, 150) => sqrt::sqrt_native::sqrt_native::<N, 24>(raw, const { Int::<24>::TEN.pow(SCALE) }, mode),
            _ => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
        },
        Algorithm::Schoolbook => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
    }
}
