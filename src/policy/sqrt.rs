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
    /// — `f64`-seeded narrow-work bespoke for the `(D57, 20)` cell.
    ///
    /// Gated with the kernel: the `(D57, 20)` cell only exists when D57
    /// is compiled in, so the variant, its `select` arm, and its
    /// dispatch arm are gated together (the policy stays exhaustive in
    /// both configs — see `docs/ARCHITECTURE.md` "Feature-flagging a
    /// variation").
    #[cfg(any(feature = "d57", feature = "wide"))]
    NewtonWithTableSeed,
    /// [`sqrt::sqrt_native::sqrt_native`] — top-bits-`f64`-seeded Newton
    /// run directly in a tight, concrete `Int<W>` (the work width `W` is
    /// chosen per `(N, SCALE)` cell in the dispatch arm to just cover
    /// `mag · 10^SCALE`), rather than through the width-agnostic int
    /// `isqrt` slice, whose build-max scratch buffer churn dominated the
    /// small mid-scale radicands of the wide tiers. Routed cells:
    /// `(D76,35)`, `(D115,57)`, `(D153,75)`, `(D230,115)`, `(D307,150)`.
    /// Microbench (`root_kernel_ab`): 1.2–1.6× faster than the generic
    /// slice [`Self::Newton`] at every routed cell. Bit-identical to
    /// [`Self::Newton`] across all six modes.
    ///
    /// Gated with the kernel: each routed `(N, SCALE)` cell only exists
    /// when its tier is compiled in, so the variant, its `select` arms,
    /// and its dispatch arms are gated together (the policy stays
    /// exhaustive in both configs).
    #[cfg(any(feature = "d57", feature = "wide"))]
    Native,
    /// Schoolbook reference tag -- delegates to
    /// [], which uses the same
    /// -based pipeline as . Exists as an explicit
    /// benchmarkable seam; never selected by  in production.
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
        // Mid-wide tiers (D57/D76/D115/D153, N = 3/4/6/8) — bespoke
        // f64-seeded Newton in a tight, concrete `Int<W>` with `W = 2N`
        // (covering `mag · 10^SCALE` at any valid scale, the storage
        // magnitude never exceeding 64N bits and `10^SCALE` adding
        // ≤ 64N more for SCALE ≤ the tier's digit capacity). Routed by `N`
        // (all scales) because the build-max slice scratch the generic
        // `Newton` kernel zeroes per Newton iteration dominates these small
        // mid-scale radicands. Microbench (`root_kernel_ab`, bbc scales):
        // native beats the generic slice 1.21× (D57<20>), 1.97× (D76<20>),
        // 1.19× (D115<25>), 1.11× (D153<25>). For N ≥ 12 (D230/D307…) the
        // slice path is at/ahead of native at the bbc scales, so those
        // tiers stay on the generic `Newton` `_` arm. Bit-identical to
        // `Newton` across all six modes (kernel test gate).
        //
        // Gated with the kernel: each tier's cells only exist when the tier
        // is compiled in, so the variant, these arms, and the dispatch arms
        // are gated together (the policy stays exhaustive in both configs).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, _) // D57,  W=6
        | (4, _) // D76,  W=8
        | (6, _) // D115, W=12
        | (8, _) // D153, W=16
        => Select::ByAlgorithm(Algorithm::Native),
        // Everything else (N ≥ 12 wide tiers, all scales) — generic Newton
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
            3 => sqrt::sqrt_native::sqrt_native::<N, 6>(raw, const { Int::<6>::TEN.pow(SCALE) }, mode),
            4 => sqrt::sqrt_native::sqrt_native::<N, 8>(raw, const { Int::<8>::TEN.pow(SCALE) }, mode),
            6 => sqrt::sqrt_native::sqrt_native::<N, 12>(raw, const { Int::<12>::TEN.pow(SCALE) }, mode),
            8 => sqrt::sqrt_native::sqrt_native::<N, 16>(raw, const { Int::<16>::TEN.pow(SCALE) }, mode),
            _ => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
        },
        Algorithm::Schoolbook => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
    }
}
