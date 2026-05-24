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
use crate::int::types::work_scratch::WorkScratch;
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
        // (D57, SCALE == 20) — bespoke narrow-work + f64 seed. Gated
        // with the kernel; falls to `Newton` when D57 is not compiled in
        // (in which case the `(3, 20)` cell is unreachable anyway).
        #[cfg(any(feature = "d57", feature = "wide"))]
        (3, 20) => Select::ByAlgorithm(Algorithm::NewtonWithTableSeed),
        // Everything else (all wide tiers, all other scales) — generic
        // Newton over the tier's work width.
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
    Int<N>: WorkScratch,
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
        Algorithm::Schoolbook => sqrt::sqrt_newton::sqrt_newton::<N>(raw, SCALE, mode),
    }
}
